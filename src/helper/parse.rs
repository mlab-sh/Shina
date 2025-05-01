use crate::helper::dsl::Shina;
use std::collections::HashMap;

#[derive(Debug, serde::Deserialize)]
struct RawRule {
    name: String,
    description: String,
    severity: String,
    tags: Vec<String>,
    #[serde(default)]
    metadata: HashMap<String, String>,
    rules: String,
}
pub fn parse_shina_from_yaml(yaml: &str) -> Result<Vec<Shina>, Box<dyn std::error::Error>> {
    let raw_rules: Vec<RawRule> = serde_yaml::from_str(yaml)?;
    let shinas = raw_rules
        .into_iter()
        .map(|r| Shina::new(&r.name, &r.description, r.metadata, r.rules, r.severity.as_str(), r.tags))
        .collect();
    Ok(shinas)
}