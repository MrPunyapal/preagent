//! Transparent, offline evaluation of the deterministic routing baseline.
//! The checked-in cases are human-labeled routing expectations, NOT proof of
//! downstream coding-agent success or an unbiased model-quality benchmark.
use crate::context::RepoContext;
use crate::router::{self, Tier};

const CORPUS: &str = include_str!("../evals/baseline.tsv");

#[derive(Debug, Clone)]
pub struct ResultRow {
    id: String,
    expected: Tier,
    actual: Tier,
}

pub struct Report {
    rows: Vec<ResultRow>,
}

pub fn evaluate_builtin() -> Result<Report, String> {
    let mut rows = Vec::new();
    for (index, line) in CORPUS.lines().enumerate() {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let fields: Vec<&str> = line.split('\t').collect();
        if fields.len() != 3 {
            return Err(format!("line {}: expected 3 tab-separated columns", index + 1));
        }
        let (id, expected, prompt) = (fields[0], fields[1], fields[2]);
        if id.is_empty()
            || !id.bytes().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'-')
        {
            return Err(format!("line {}: invalid case identifier", index + 1));
        }
        if rows.iter().any(|row: &ResultRow| row.id == id) {
            return Err(format!("line {}: duplicate case identifier", index + 1));
        }
        if prompt.trim().is_empty() {
            return Err(format!("line {}: empty prompt", index + 1));
        }
        let expected = match expected {
            "simple" => Tier::Simple,
            "standard" => Tier::Standard,
            "complex" => Tier::Complex,
            _ => return Err(format!("line {}: unknown tier '{expected}'", index + 1)),
        };
        let actual = router::classify(prompt, &RepoContext::default()).tier;
        rows.push(ResultRow {
            id: id.to_string(),
            expected,
            actual,
        });
    }
    if rows.is_empty() {
        return Err("corpus contains no cases".to_string());
    }
    Ok(Report { rows })
}

impl Report {
    pub fn accuracy(&self) -> (usize, usize) {
        let correct = self.rows.iter().filter(|r| r.expected == r.actual).count();
        (correct, self.rows.len())
    }

    pub fn to_text(&self) -> String {
        let (correct, total) = self.accuracy();
        let mut output = format!(
            "Routing smoke corpus: {correct}/{total} labels matched ({:.1}%).\n",
            100.0 * correct as f64 / total as f64
        );
        for tier in [Tier::Simple, Tier::Standard, Tier::Complex] {
            let matched = self.rows.iter().filter(|r| r.expected == tier).count();
            let accurate = self
                .rows
                .iter()
                .filter(|r| r.expected == tier && r.actual == tier)
                .count();
            output.push_str(&format!("  {}: {accurate}/{matched}\n", tier.as_str()));
        }
        for row in &self.rows {
            if row.expected != row.actual {
                output.push_str(&format!(
                    "  mismatch {}: expected {}, got {}\n",
                    row.id,
                    row.expected.as_str(),
                    row.actual.as_str()
                ));
            }
        }
        output.push_str("These subjective routing labels are not coding-success benchmarks.");
        output
    }

    pub fn to_json(&self) -> String {
        let (correct, total) = self.accuracy();
        let mismatches = self.rows.iter().filter(|r| r.expected != r.actual)
            .map(|row| format!(
                "{{\"id\":\"{}\",\"expected\":\"{}\",\"actual\":\"{}\"}}",
                row.id,
                row.expected.as_str(),
                row.actual.as_str(),
            ))
            .collect::<Vec<_>>()
            .join(",");
        format!(
            "{{\"corpus\":\"baseline\",\"total\":{total},\"correct\":{correct},\"accuracy\":{},\"mismatches\":[{mismatches}]}}",
            correct as f64 / total as f64
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn corpus_has_three_tiers_and_unique_ids() {
        let report = evaluate_builtin().expect("corpus should parse");
        assert!(report.rows.len() >= 30, "minimum smoke coverage");
        for tier in [Tier::Simple, Tier::Standard, Tier::Complex] {
            assert!(report.rows.iter().any(|r| r.expected == tier));
        }
    }

    #[test]
    fn report_is_machine_readable() {
        let report = evaluate_builtin().unwrap();
        let json = report.to_json();
        assert!(json.starts_with("{\"corpus\":\"baseline\""));
        assert!(json.contains("\"mismatches\":["));
    }
}
