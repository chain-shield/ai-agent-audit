/// Contract inheritance analysis and parsing.
///
/// This module processes Slither's inheritance printer output to extract
/// parent-child relationships between smart contracts, supporting both
/// immediate and transitive inheritance hierarchies.
use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;

/// Step 2: pull every DOT file’s `content` string
pub fn parse_inheritance_json(json: &str) -> Result<Vec<(String, String)>> {
    /// Represents a list of immediate and non-immediate inheritance relationships.
    #[derive(Debug, Deserialize)]
    pub struct InheritanceRelation {
        pub immediate: Vec<String>,
        pub not_immediate: Vec<String>,
    }

    /// Outer container for additional fields (child_to_base and base_to_child maps).
    #[derive(Debug, Deserialize)]
    pub struct AdditionalFields {
        pub child_to_base: HashMap<String, InheritanceRelation>,
        pub base_to_child: HashMap<String, InheritanceRelation>,
    }

    #[derive(Deserialize)]
    struct Printer {
        pub printer: String,
        pub additional_fields: AdditionalFields,
    }
    #[derive(Deserialize)]
    struct Root {
        results: Results,
    }
    #[derive(Deserialize)]
    struct Results {
        #[serde(default)]
        printers: Vec<Printer>,
    }

    let root: Root = serde_json::from_str(json)?;
    let mut child_parent_map = HashMap::new();
    let mut edges = Vec::new();
    for printer in root.results.printers {
        if printer.printer == "inheritance" {
            child_parent_map = printer.additional_fields.child_to_base;
        }
    }

    for (child, parents) in child_parent_map.into_iter() {
        for parent in parents.immediate.iter() {
            edges.push((child.clone(), parent.to_string()));
        }
    }
    Ok(edges)
}
