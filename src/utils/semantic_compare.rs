use regex::Regex;
use std::collections::{HashMap, HashSet};

// Cargo.toml:
// [dependencies]
// regex = "1"
// strsim = "0.10"

fn normalize(s: &str) -> String {
    // 1) Remove leading tags like [H-5], [M-01]
    let re_tag = Regex::new(r#"^\s*\[[^\]]+\]\.?\s*"#).unwrap();
    let s = re_tag.replace(s, "").to_string();

    // 2) Split CamelCase and letter-digit boundaries before lowercasing
    let re_camel = Regex::new(r#"([a-z])([A-Z])"#).unwrap();
    let s = re_camel.replace_all(&s, "$1 $2").to_string();
    let re_a1 = Regex::new(r#"([A-Za-z])([0-9])"#).unwrap();
    let s = re_a1.replace_all(&s, "$1 $2").to_string();
    let re_1a = Regex::new(r#"([0-9])([A-Za-z])"#).unwrap();
    let s = re_1a.replace_all(&s, "$1 $2").to_string();

    // 3) Lowercase
    let s = s.to_lowercase();

    // 4) Keep letters, numbers, and spaces; drop punctuation
    let re_punct = Regex::new(r#"[^a-z0-9\s]"#).unwrap();
    let s = re_punct.replace_all(&s, " ");

    // 5) Collapse whitespace
    let re_space = Regex::new(r#"\s+"#).unwrap();
    re_space.replace_all(&s, " ").trim().to_string()
}

fn simple_stem(token: &str) -> String {
    // very naive stemming to improve overlap: remove common suffixes
    let mut t = token.to_string();
    // don't stem very short tokens
    if t.len() <= 4 {
        return t;
    }
    for suf in ["ing", "ed", "es", "s"].iter() {
        if t.ends_with(suf) && t.len() > suf.len() + 2 {
            t.truncate(t.len() - suf.len());
            break;
        }
    }
    // normalize frequent domain variants
    match t.as_str() {
        "dosable" | "dos" | "dosing" => return "dos".to_string(),
        "bricking" | "bricked" | "bricks" => return "brick".to_string(),
        "locking" | "locked" | "locks" => return "lock".to_string(),
        "accumulator" | "accumulation" => return "accumulate".to_string(),
        "withdrawfees" | "withdraw" | "withdrawal" | "withdrawals" => {
            return "withdraw".to_string();
        }
        "permanently" | "permanent" => return "permanent".to_string(),
        "pattern" | "patterns" => return "pattern".to_string(),
        "predictable" | "predict" => return "predict".to_string(),
        "winner" | "winning" => return "winner".to_string(),
        "caller" | "calling" | "call" => return "call".to_string(),
        "attacker" | "attacking" | "attack" => return "attack".to_string(),
        "steal" | "stealing" | "steals" => return "steal".to_string(),
        "drain" | "draining" | "drains" => return "drain".to_string(),
        "overflow" | "overflows" | "overflowing" => return "overflow".to_string(),
        "forced" | "force" | "forcing" => return "force".to_string(),
        "skew" | "skews" | "skewing" => return "skew".to_string(),
        "break" | "breaks" | "breaking" => return "break".to_string(),
        "balance" | "balances" => return "balance".to_string(),
        "fee" | "fees" => return "fee".to_string(),
        "protocol" | "protocols" => return "protocol".to_string(),
        "invariant" | "invariants" => return "invariant".to_string(),
        "check" | "checks" | "checking" => return "check".to_string(),
        "entry" | "entries" | "entrant" => return "entry".to_string(),
        "prize" | "prizes" => return "prize".to_string(),
        "pool" | "pools" => return "pool".to_string(),
        "pot" | "pots" => return "pot".to_string(),
        "rng" | "random" | "randomness" => return "random".to_string(),
        "timestamp" | "time" => return "time".to_string(),
        "difficulty" | "difficult" => return "difficulty".to_string(),
        "sender" | "send" | "sending" => return "send".to_string(),
        "msg" | "message" => return "message".to_string(),
        "via" | "through" | "using" => return "via".to_string(),
        _ => {}
    }
    t
}

fn tokenize(s: &str) -> Vec<String> {
    // very small stopword set; expand as needed
    const STOP: &[&str] = &[
        "the", "a", "an", "and", "or", "of", "to", "in", "on", "for", "with", "via", "by", "is",
        "are", "this", "that", "it", "as", "be", "can", "will", "at", "from", "into", "over",
    ];
    let stop: HashSet<&str> = STOP.iter().copied().collect();

    s.split_whitespace()
        .filter(|t| !stop.contains(*t))
        .map(|t| simple_stem(t))
        .collect()
}

fn jaccard(tokens_a: &[String], tokens_b: &[String]) -> f64 {
    let set_a: HashSet<&str> = tokens_a.iter().map(|s| s.as_str()).collect();
    let set_b: HashSet<&str> = tokens_b.iter().map(|s| s.as_str()).collect();
    let inter = set_a.intersection(&set_b).count() as f64;
    let union = set_a.union(&set_b).count() as f64;
    if union == 0.0 {
        0.0
    } else {
        inter / union
    }
}

fn tf(tokens: &[String]) -> HashMap<String, f64> {
    let mut m = HashMap::new();
    for t in tokens {
        *m.entry(t.clone()).or_insert(0.0) += 1.0;
    }
    // normalize by length (so it’s TF / |tokens|)
    let n = tokens.len() as f64;
    if n > 0.0 {
        for v in m.values_mut() {
            *v /= n;
        }
    }
    m
}

fn cosine(tf_a: &HashMap<String, f64>, tf_b: &HashMap<String, f64>) -> f64 {
    let mut dot = 0.0;
    let mut na = 0.0;
    let mut nb = 0.0;

    for (k, va) in tf_a {
        na += va * va;
        if let Some(vb) = tf_b.get(k) {
            dot += va * vb;
        }
    }
    for vb in tf_b.values() {
        nb += vb * vb;
    }
    if na == 0.0 || nb == 0.0 {
        return 0.0;
    }
    dot / (na.sqrt() * nb.sqrt())
}

fn jaro_winkler(a: &str, b: &str) -> f64 {
    strsim::jaro_winkler(a, b)
}

pub fn similarity_score(title_a: &str, title_b: &str) -> f64 {
    let a = normalize(title_a);
    let b = normalize(title_b);

    let toks_a = tokenize(&a);
    let toks_b = tokenize(&b);

    let jac = jaccard(&toks_a, &toks_b);
    let cos = cosine(&tf(&toks_a), &tf(&toks_b));
    let jw = jaro_winkler(&a, &b);

    // blend: emphasize token overlap more for technical titles
    0.50 * jac + 0.35 * cos + 0.15 * jw
}

pub fn likely_duplicate(title_a: &str, title_b: &str, threshold: f64) -> bool {
    similarity_score(title_a, title_b) >= threshold
}

#[cfg(test)]
mod tests {
    use super::*;
    // titles from your dataset
    const H1: &str = "Forced ETH breaks balance==totalFees invariant in withdrawFees, permanently locking protocol fees";
    const H2: &str =
        "Predictable RNG lets any caller steer winner index to own entry and steal the pot";
    const H3: &str = "Predictable winner via msg.sender + block.timestamp/difficulty lets attacker steal entire prize pool";
    const H4: &str = "Reentrancy via refund lets a single entrant drain multiple entrance fees before their slot is cleared";
    const H5: &str = "Forced-ETH skew of balance check bricks PuppyRaffle.withdrawFees and permanently locks all accumulated fees";
    const H6: &str = "uint64 fee accumulator overflows in PuppyRaffle.selectWinner, desyncing fees vs balance and permanently bricking withdrawFees";
    const H7: &str = "Rounding dust in 80/20 split leaves 1 wei each round, permanently bricking withdrawFees via strict balance==totalFees check";
    const H8: &str = "uint64 fee accumulation overflows in PuppyRaffle.selectWinner, permanently bricking fee withdrawals";
    const M1: &str = "selectWinner is permanently DoS-able via winner fallback or onERC721Received hook reverting";
    const M2: &str = "DOS via quadratic duplicate scan in PuppyRaffle.enterRaffle lets anyone bloat players and block future entries within block gas limit";
    const M3: &str = "selectWinner computes prize from players.length ignoring refunded slots, causing prize > balance and bricking the raffle";

    #[test]
    fn print_similarity_scores_for_similar_pairs() {
        let pairs = vec![
            ("Forced-ETH dupes (H1 vs H5)", H1, H5, true), // strong dupes
            ("Overflow dupes (H6 vs H8)", H6, H8, true),   // strong dupes
            ("Predictable RNG (H2 vs H3)", H2, H3, true),  // strong dupes
            // “maybe” related; print only, no assert to keep test robust
            ("Forced-ETH vs rounding-dust (H1 vs H7)", H1, H7, false),
            ("Overflow vs rounding-dust (H6 vs H7)", H6, H7, false),
            // negative controls
            ("Different topics (H1 vs M2)", H1, M2, false),
            ("Different topics (H4 vs M1)", H4, M1, false),
            ("Different topics (H2 vs M3)", H2, M3, false),
        ];

        println!("\n=== Similarity Scores ===");
        for (label, a, b, _should_dupe) in pairs {
            let score = similarity_score(a, b);
            println!("{label}: {score:.3}");
            println!("  A: \"{a}\"");
            println!("  B: \"{b}\"");
            println!();
        }
    }
    #[test]
    fn test_dupe_titles() {
        let t1 = "Forced ETH via selfdestruct manipulates balance check and bricks PuppyRaffle.withdrawFees";
        let t2 = "Forced ETH breaks balance==totalFees invariant, permanently DoSing PuppyRaffle.withdrawFees and locking protocol fees";

        let score = similarity_score(t1, t2);
        println!("similarity = {score:.3}");
        assert!(score >= 0.50, "score too low: {score}");
    }

    #[test]
    fn test_similarity_thresholds() {
        // Test that obvious duplicates score above 0.55
        let obvious_dupes = vec![(
            "Overflow variants",
            "uint64 fee accumulator overflows in PuppyRaffle.selectWinner, desyncing fees vs balance and permanently bricking withdrawFees",
            "uint64 fee accumulation overflows in PuppyRaffle.selectWinner, permanently bricking fee withdrawals",
        )];

        for (label, a, b) in obvious_dupes {
            let score = similarity_score(a, b);
            println!("{label}: {score:.3}");
            assert!(
                score >= 0.55,
                "Expected obvious duplicate >= 0.55, got {score} for: {label}"
            );
        }

        // Test that clearly different topics score below 0.30
        let different_topics = vec![(
            "Forced ETH vs DoS",
            "Forced ETH breaks balance==totalFees invariant in withdrawFees, permanently locking protocol fees",
            "DOS via quadratic duplicate scan in PuppyRaffle.enterRaffle lets anyone bloat players and block future entries within block gas limit",
        )];

        for (label, a, b) in different_topics {
            let score = similarity_score(a, b);
            println!("{label}: {score:.3}");
            assert!(
                score < 0.30,
                "Expected different topics < 0.30, got {score} for: {label}"
            );
        }
    }
}
