//! Frozen, separate held-out engineering pilot.
//! Task labels are independent human judgments, not model-quality ground truth.
//! Agent execution is deliberately outside this harness.
mod workspace;

use crate::context::RepoContext;
use crate::router::{self, Tier};
use crate::slm;
use std::path::PathBuf;
use std::time::Instant;

const MANIFEST: &str = include_str!("../evals/heldout/v1.tsv");

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    List {
        json: bool,
    },
    Route {
        json: bool,
        model: Option<String>,
    },
    Prepare {
        id: String,
        source: PathBuf,
        output: PathBuf,
    },
    Grade {
        id: String,
        candidate: PathBuf,
    },
}

#[derive(Clone, Copy)]
pub struct Case {
    pub id: &'static str,
    pub expected: Tier,
    pub repo: &'static str,
    pub sha: &'static str,
    pub license: &'static str,
    pub patch_path: &'static str,
    pub test_path: &'static str,
    pub prompt: &'static str,
}

pub fn parse(mut args: impl Iterator<Item = String>) -> Result<Command, String> {
    let mode = args
        .next()
        .ok_or("heldout requires list, route, prepare or grade")?;
    match mode.as_str() {
        "list" => {
            let flags: Vec<String> = args.collect();
            match flags.as_slice() {
                [] => Ok(Command::List { json: false }),
                [flag] if flag == "--json" => Ok(Command::List { json: true }),
                _ => Err("usage: preagent heldout list [--json]".into()),
            }
        }
        "route" => {
            let mut json = false;
            let mut model = None;
            while let Some(flag) = args.next() {
                match flag.as_str() {
                    "--json" if !json => json = true,
                    "--slm" if model.is_none() => {
                        model = Some(args.next().ok_or("--slm needs an installed model name")?);
                    }
                    _ => return Err("usage: preagent heldout route [--json] [--slm MODEL]".into()),
                }
            }
            Ok(Command::Route { json, model })
        }
        "prepare" => {
            let fields: Vec<String> = args.collect();
            if fields.len() != 3 {
                return Err(
                    "usage: preagent heldout prepare TASK LOCAL_SOURCE NEW_OUTPUT_DIRECTORY".into(),
                );
            }
            Ok(Command::Prepare {
                id: fields[0].clone(),
                source: PathBuf::from(&fields[1]),
                output: PathBuf::from(&fields[2]),
            })
        }
        "grade" => {
            let fields: Vec<String> = args.collect();
            if fields.len() != 2 {
                return Err("usage: preagent heldout grade TASK CANDIDATE_DIRECTORY".into());
            }
            Ok(Command::Grade {
                id: fields[0].clone(),
                candidate: PathBuf::from(&fields[1]),
            })
        }
        _ => Err("heldout requires list, route, prepare or grade".into()),
    }
}

pub fn run(command: Command) -> Result<String, String> {
    match command {
        Command::List { json } => list(json),
        Command::Route { json, model } => route(json, model.as_deref()),
        Command::Prepare { id, source, output } => {
            workspace::prepare(find_case(&id)?, &source, &output)
        }
        Command::Grade { id, candidate } => workspace::grade(find_case(&id)?, &candidate),
    }
}

pub fn fixtures(id: &str) -> Option<(&'static str, &'static str)> {
    match id {
        "itoa-decimal-pairs" => Some((
            include_str!("../evals/heldout/seeds/itoa-decimal-pairs.patch"),
            include_str!("../evals/heldout/acceptance/itoa-decimal-pairs.rs"),
        )),
        "ryu-negative-infinity" => Some((
            include_str!("../evals/heldout/seeds/ryu-negative-infinity.patch"),
            include_str!("../evals/heldout/acceptance/ryu-negative-infinity.rs"),
        )),
        "byteorder-endian-read" => Some((
            include_str!("../evals/heldout/seeds/byteorder-endian-read.patch"),
            include_str!("../evals/heldout/acceptance/byteorder-endian-read.rs"),
        )),
        "byteorder-signed-width" => Some((
            include_str!("../evals/heldout/seeds/byteorder-signed-width.patch"),
            include_str!("../evals/heldout/acceptance/byteorder-signed-width.rs"),
        )),
        _ => None,
    }
}

pub fn cases() -> Result<Vec<Case>, String> {
    let mut result = Vec::new();
    for (line_no, line) in MANIFEST.lines().enumerate() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() != 8 {
            return Err(format!("manifest line {} must have 8 columns", line_no + 1));
        }
        let [id, label, repo, sha, license, patch_path, test_path, prompt] =
            <[&str; 8]>::try_from(parts.as_slice()).map_err(|_| "manifest must have 8 columns")?;
        if result.iter().any(|case: &Case| case.id == id) {
            return Err(format!("duplicate heldout task {id}"));
        }
        let expected = match label {
            "simple" => Tier::Simple,
            "standard" => Tier::Standard,
            "complex" => Tier::Complex,
            _ => return Err(format!("unrecognized frozen task label: {label}")),
        };
        if sha.len() != 40 || !sha.bytes().all(|b| b.is_ascii_hexdigit()) {
            return Err(format!("task {id} lacks a pinned 40-digit git SHA"));
        }
        if prompt.trim().is_empty() || fixtures(id).is_none() {
            return Err(format!("task {id} has no description or fixtures"));
        }
        if patch_path != format!("seeds/{id}.patch") || test_path != format!("acceptance/{id}.rs") {
            return Err(format!("task {id} fixture paths do not match its ID"));
        }
        if !matches!(
            repo,
            "dtolnay/itoa" | "dtolnay/ryu" | "BurntSushi/byteorder"
        ) {
            return Err(format!("task {id} references an unexpected repository"));
        }
        if !matches!(
            license,
            "MIT OR Apache-2.0" | "Apache-2.0 OR BSL-1.0" | "Unlicense OR MIT"
        ) {
            return Err(format!("task {id} references an unexpected license"));
        }
        result.push(Case {
            id,
            expected,
            repo,
            sha,
            license,
            patch_path,
            test_path,
            prompt,
        });
    }
    if result.len() != 4 {
        return Err("heldout pilot must contain the four frozen v1 cases".into());
    }
    Ok(result)
}

fn find_case(id: &str) -> Result<Case, String> {
    cases()?
        .into_iter()
        .find(|case| case.id == id)
        .ok_or_else(|| format!("unknown heldout task '{id}'"))
}

fn list(json: bool) -> Result<String, String> {
    let cases = cases()?;
    if json {
        let values = cases
            .iter()
            .map(|case| {
                format!(
                    "{{\"id\":{},\"repository\":{},\"commit\":{},\"license\":{},\"human_tier\":{},\"prompt\":{}}}",
                    slm::json_quote(case.id),
                    slm::json_quote(case.repo),
                    slm::json_quote(case.sha),
                    slm::json_quote(case.license),
                    slm::json_quote(case.expected.as_str()),
                    slm::json_quote(case.prompt)
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        Ok(format!("{{\"suite\":\"heldout-v1\",\"tasks\":[{values}]}}"))
    } else {
        let mut result =
            "Frozen heldout-v1 pilot: 4 independently labeled seeded regressions\n".to_string();
        for case in cases {
            result.push_str(&format!(
                "{} [{}] {} @ {}\n  {}\n",
                case.id,
                case.expected.as_str(),
                case.repo,
                case.sha,
                case.prompt
            ));
        }
        result.push_str("No agent is started; the frozen tier label is not a correctness test.");
        Ok(result)
    }
}

struct Row<'a> {
    case: &'a Case,
    rules: Tier,
    rules_us: u128,
    slm: Option<Tier>,
    slm_ms: Option<f64>,
    status: &'static str,
    hybrid: Tier,
}

fn route(json: bool, model: Option<&str>) -> Result<String, String> {
    let cases = cases()?;
    let mut rows = Vec::new();
    let mut rules_correct = 0usize;
    let mut slm_correct = 0usize;
    let mut slm_count = 0usize;
    let mut hybrid_correct = 0usize;
    let mut unavailable = 0usize;
    for case in &cases {
        let started = Instant::now();
        let rules = router::classify(case.prompt, &RepoContext::default()).tier;
        let rules_us = started.elapsed().as_micros();
        let mut slm_tier = None;
        let mut slm_ms = None;
        let mut status = "not_requested";
        if let Some(model_id) = model {
            let started = Instant::now();
            let suggestion = slm::classify(case.prompt, model_id);
            slm_ms = Some(started.elapsed().as_secs_f64() * 1000.0);
            match suggestion {
                Ok(value) => {
                    slm_tier = Some(value);
                    status = "ok";
                    slm_count += 1;
                    if value == case.expected {
                        slm_correct += 1;
                    }
                }
                Err(_) => {
                    status = "unavailable";
                    unavailable += 1;
                }
            }
        }
        let hybrid = slm_tier.map_or(rules, |tier| slm::safer_tier(rules, tier));
        if rules == case.expected {
            rules_correct += 1;
        }
        if hybrid == case.expected {
            hybrid_correct += 1;
        }
        rows.push(Row {
            case,
            rules,
            rules_us,
            slm: slm_tier,
            slm_ms,
            status,
            hybrid,
        });
    }
    if !json {
        let mut out = format!(
            "Frozen heldout-v1: {rules_correct}/{} rule-tier labels agreed\n",
            cases.len()
        );
        if model.is_some() {
            out.push_str(&format!(
                "SLM: {slm_correct}/{slm_count} valid replies agreed, {unavailable} unavailable\n"
            ));
            out.push_str(&format!(
                "Hybrid: {hybrid_correct}/{} labels agreed (fallback included)\n",
                cases.len()
            ));
        }
        for row in rows {
            out.push_str(&format!(
                "{}: expected={} rules={} rules_us={} slm={} slm_ms={} hybrid={}\n",
                row.case.id,
                row.case.expected.as_str(),
                row.rules.as_str(),
                row.rules_us,
                row.slm.map(Tier::as_str).unwrap_or(row.status),
                row.slm_ms
                    .map(|ms| format!("{ms:.3}"))
                    .unwrap_or_else(|| "n/a".into()),
                row.hybrid.as_str()
            ));
        }
        out.push_str("Coding-task outcomes: NOT MEASURED. No coding agents were started.");
        return Ok(out);
    }
    let cases_json = rows
        .iter()
        .map(|row| format!(
            "{{\"id\":{},\"human_tier\":{},\"rule_tier\":{},\"rule_us\":{},\"slm_tier\":{},\"slm_ms\":{},\"slm_status\":{},\"hybrid_tier\":{},\"task_correctness\":null}}",
            slm::json_quote(row.case.id),
            slm::json_quote(row.case.expected.as_str()),
            slm::json_quote(row.rules.as_str()),
            row.rules_us,
            row.slm.map(|tier| slm::json_quote(tier.as_str())).unwrap_or_else(|| "null".into()),
            row.slm_ms.map(|ms| format!("{ms:.3}")).unwrap_or_else(|| "null".into()),
            slm::json_quote(row.status),
            slm::json_quote(row.hybrid.as_str())
        ))
        .collect::<Vec<_>>()
        .join(",");
    let slm_model = model.map(slm::json_quote).unwrap_or_else(|| "null".into());
    let peak = peak_rss_kib()
        .map(|v| v.to_string())
        .unwrap_or_else(|| "null".into());
    Ok(format!(
        "{{\"suite\":\"heldout-v1\",\"platform\":{},\"arch\":{},\"slm_model\":{},\"preagent_peak_rss_kib\":{},\"rule_agreement\":{{\"correct\":{},\"total\":{}}},\"slm_agreement\":{{\"correct\":{},\"valid\":{},\"unavailable\":{}}},\"hybrid_agreement\":{{\"correct\":{},\"total\":{}}},\"task_correctness\":null,\"rows\":[{}]}}",
        slm::json_quote(std::env::consts::OS),
        slm::json_quote(std::env::consts::ARCH),
        slm_model,
        peak,
        rules_correct,
        cases.len(),
        slm_correct,
        slm_count,
        unavailable,
        hybrid_correct,
        cases.len(),
        cases_json
    ))
}

// Linux-only process peak RSS. Ollama and GPU memory are not included.
// Null is preferable to a fabricated estimate on other operating systems.
fn peak_rss_kib() -> Option<u64> {
    let proc_status = std::fs::read_to_string("/proc/self/status").ok()?;
    let value = proc_status
        .lines()
        .find(|line| line.starts_with("VmHWM:"))?;
    value.split_whitespace().nth(1)?.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frozen_set_does_not_reuse_the_smoke_corpus() {
        let cases = cases().expect("manifest must be valid");
        let smoke = include_str!("../evals/baseline.tsv");
        assert_eq!(cases.len(), 4);
        for case in &cases {
            assert!(
                !smoke.contains(case.prompt),
                "heldout task copied from smoke corpus"
            );
            let (patch, test) = fixtures(case.id).unwrap();
            assert!(patch.contains("diff --git"));
            assert!(test.contains("#[test]"));
        }
    }

    #[test]
    fn lists_cases_without_network_or_agent() {
        let output = run(Command::List { json: true }).unwrap();
        assert!(output.contains("\"suite\":\"heldout-v1\""));
        assert!(output.contains("byteorder-signed-width"));
    }

    #[test]
    fn rejects_unknown_case_before_touching_files() {
        let error = run(Command::Grade {
            id: "no-such-task".into(),
            candidate: PathBuf::from("."),
        })
        .unwrap_err();
        assert!(error.contains("unknown heldout task"));
    }

    #[test]
    fn route_only_is_advisory_and_keeps_correctness_null() {
        let output = run(Command::Route {
            json: true,
            model: None,
        })
        .unwrap();
        assert!(output.contains("\"task_correctness\":null"));
        assert!(output.contains("\"slm_model\":null"));
        assert!(output.contains("\"slm_status\":\"not_requested\""));
    }
}
