use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use anyhow::Result;
use rig::{
    completion::{CompletionModel, CompletionRequest, CompletionRequestBuilder, Prompt},
    providers::openai::{
        self,
        completion::{CompletionModel as OpenaiCompletionModel, GPT_4O_MINI}, // model handle & constants
        Client,
    },
    vector_store::VectorStoreIndex,
};

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct ContentScore {
    score: f32,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct AnswerWithCitations {
    pub answer: String,
    pub sources_used: Vec<u8>, // Source numbers that were actually used in the answer
    /// Confidence level in the answer (1-10)
    pub confidence: Option<f32>,
}

/// Concrete index type (same one you pass to `VectorSearchTool`)
pub type SharedIndex<I> = Arc<I>;

pub struct AdvancedRAGSystem<I>
where
    I: VectorStoreIndex + Send + Sync + 'static,
{
    pub vector_index: SharedIndex<I>,
    pub llm: OpenaiCompletionModel, // `openai_client.completion_model(GPT_4O_MINI)`
    pub relevance_threshold: f32,   // e.g. 0.75
}

impl<I> AdvancedRAGSystem<I>
where
    I: VectorStoreIndex + Send + Sync + 'static,
{
    /// 1️⃣ semantic search → 2️⃣ threshold filter → 3️⃣ (optional) LLM re-rank
    /// → 4️⃣ answer with citations.
    pub async fn enhanced_query(&self, question: &str) -> Result<String> {
        // The payload we indexed is plain text (`String`)
        let mut docs: Vec<(f64, String, String)> =
            self.vector_index.top_n::<String>(question, 10).await?;

        // keep only highly-relevant
        docs.retain(|(score, _, _)| *score >= self.relevance_threshold as f64);
        docs.truncate(5);

        let reranked = self.rerank_documents(question, docs).await?;
        self.generate_response(question, &reranked).await
    }

    /* ───────── helpers ─────────────────────────────────────────── */

    async fn rerank_documents(
        &self,
        query: &str,
        docs: Vec<(f64, String, String)>, // (score, id, content)
    ) -> Result<Vec<(f64, String, String)>> {
        let mut scored = Vec::<(f64, String, String, f32)>::new();

        let openai = Client::new(&std::env::var("OPENAI_API_KEY")?);

        // Improved preamble with clearer instructions and examples
        let preamble = format!(
            r#"You are an expert document relevance scorer. Your task is to rate how relevant a document is to answering a specific query.

            SCORING CRITERIA:
            - 10: Perfect match - document directly answers the query
            - 8-9: Highly relevant - contains most information needed
            - 6-7: Moderately relevant - contains some useful information
            - 4-5: Somewhat relevant - tangentially related
            - 1-3: Not relevant - unrelated or unhelpful

            QUERY: {query}

            Rate the relevance of the following document on a scale of 1-10. Consider:
            1. How directly the document addresses the query
            2. The completeness of information provided
            3. The accuracy and reliability of the content

            Provide your score as a number between 1 and 10."#
        );

        let content_scorer = openai
            .extractor::<ContentScore>(GPT_4O_MINI)
            .preamble(&preamble)
            .build();

        for (orig_score, id, content) in docs {
            let result = content_scorer.extract(&content).await?;

            if result.score >= 6.0 {
                scored.push((orig_score, id, content, result.score));
            }
        }
        scored.sort_by(|a, b| b.3.partial_cmp(&a.3).unwrap()); // high → low
        Ok(scored.into_iter().map(|(s, id, c, _)| (s, id, c)).collect())
    }

    async fn generate_response(
        &self,
        question: &str,
        docs: &[(f64, String, String)],
    ) -> Result<String> {
        if docs.is_empty() {
            return Ok("I don’t have enough relevant information to answer that.".into());
        }

        let context = docs
            .iter()
            .enumerate()
            .map(|(i, (_, _, content))| format!("Source {}:\n{}", i + 1, content))
            .collect::<Vec<_>>()
            .join("\n\n");
        let openai = Client::new(&std::env::var("OPENAI_API_KEY")?);

        let preamble = format!(
            r#"
            You are a helpful AI assistant that answers questions based solely on provided source materials.

            INSTRUCTIONS:
            1. Answer the question using ONLY information from the provided sources
            2. Include inline citations using square brackets [1], [2], etc.
            3. If information is insufficient, clearly state what's missing
            4. Do not make assumptions or add information not in the sources
            5. Maintain a professional, informative tone

            CITATION EXAMPLES:
            - "According to the contract terms [1], the payment is due within 30 days."
            - "The study shows [2] that performance improved by 15%."

            IMPORTANT: In your response JSON, the sources_used field should contain the actual numbers you cited (e.g., if you cite [1] and [3], then sources_used should be [1, 3]).

            QUESTION: {question}

            AVAILABLE SOURCES:
            {context}

            Please provide a comprehensive answer with proper citations.
            "#
        );

        // More specific extraction prompt
        let extraction_prompt = "Based on the question and sources provided above, generate a well-structured answer with proper citations. Include the source numbers you referenced in your response.";
        let answer_extractor = openai
            .extractor::<AnswerWithCitations>(GPT_4O_MINI)
            .preamble(&preamble)
            .build();

        let result = answer_extractor.extract(extraction_prompt).await?;

        // Validate that citations were actually used
        if result.sources_used.is_empty() && !docs.is_empty() {
            eprintln!("Warning: No sources were cited in the response");
        }

        Ok(result.answer)
    }
}
