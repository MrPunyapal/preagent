use crate::context::RepoContext;

/// Require token boundaries so "author" does not trigger "auth", or
/// "contest" trigger "test". Phrases such as "clean up" remain supported.
/// This is a conservative deterministic signal, not language understanding.
fn contains_phrase(text: &str, phrase: &str) -> bool {
    text.match_indices(phrase).any(|(index, _)| {
        let before = text[..index]
            .chars()
            .next_back()
            .map_or(true, |c| !c.is_alphanumeric() && c != '_');
        let after = text[index + phrase.len()..]
            .chars()
            .next()
            .map_or(true, |c| !c.is_alphanumeric() && c != '_');
        before && after
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Intent {
    Documentation,
    Tests,
    Security,
    Architecture,
    Debugging,
    Refactoring,
    Feature,
    General,
}

impl Intent {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Documentation => "documentation",
            Self::Tests => "tests",
            Self::Security => "security",
            Self::Architecture => "architecture",
            Self::Debugging => "debugging",
            Self::Refactoring => "refactoring",
            Self::Feature => "feature",
            Self::General => "general",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tier {
    Simple,
    Standard,
    Complex,
}

impl Tier {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Simple => "simple",
            Self::Standard => "standard",
            Self::Complex => "complex",
        }
    }

    pub fn suggested_effort(self) -> &'static str {
        match self {
            Self::Simple => "low",
            Self::Standard => "medium",
            Self::Complex => "high",
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Decision {
    pub intent: Intent,
    pub tier: Tier,
    pub heuristic_score: u8,
    pub is_repo: bool,
    pub changed_files: usize,
    pub reasons: Vec<&'static str>,
    pub uncertain: bool,
}

pub fn classify(prompt: &str, context: &RepoContext) -> Decision {
    let p = prompt.to_lowercase();
    let has = |words: &[&str]| words.iter().any(|word| contains_phrase(&p, word));

    let security = has(&[
        "security",
        "vulnerability",
        "vulnerabilities",
        "cve",
        "exploit",
        "auth",
        "authentication",
        "authorization",
        "permission",
        "permissions",
        "secret",
        "secrets",
        "credential",
        "credentials",
        "payment",
        "payments",
    ]);
    let architecture = has(&[
        "architecture",
        "architectural",
        "redesign",
        "distributed",
        "microservice",
        "microservices",
        "migration",
        "migrations",
    ]);
    let debugging = has(&[
        "fix",
        "bug",
        "bugs",
        "debug",
        "debugging",
        "error",
        "errors",
        "crash",
        "regression",
        "repair",
        "resolve",
    ]);
    let refactoring = has(&[
        "refactor",
        "refactoring",
        "restructure",
        "cleanup",
        "clean up",
    ]);
    let feature = has(&["implement", "add", "build", "create", "feature"]);
    let docs = has(&[
        "readme",
        "documentation",
        "docs",
        "typo",
        "comment",
        "markdown",
        "spelling",
        "whitespace",
    ]);
    let tests = has(&["test", "tests", "testing", "coverage", "spec", "specs"]);
    let trivial = docs
        && has(&["typo", "spelling", "whitespace", "readme"])
        && !security
        && !architecture
        && !feature
        && !refactoring;

    let intent = if security {
        Intent::Security
    } else if architecture {
        Intent::Architecture
    } else if trivial {
        Intent::Documentation
    } else if debugging {
        Intent::Debugging
    } else if refactoring {
        Intent::Refactoring
    } else if feature {
        Intent::Feature
    } else if tests {
        Intent::Tests
    } else if docs {
        Intent::Documentation
    } else {
        Intent::General
    };

    let mut score: u8 = 0;
    let mut reasons = Vec::new();

    if (debugging || refactoring || feature || tests) && !trivial {
        score += 2;
        reasons.push("implementation or verification requested");
    }
    if has(&[
        "entire",
        "whole codebase",
        "across the project",
        "many files",
        "multiple modules",
        "end-to-end",
    ]) {
        score += 3;
        reasons.push("broad scope");
    }
    if has(&[
        "race condition",
        "deadlock",
        "concurrency",
        "distributed",
        "intermittent",
        "performance regression",
    ]) {
        score += 3;
        reasons.push("difficult technical constraints");
    }
    if architecture {
        score += 3;
        reasons.push("architecture or migration");
    }
    if security {
        score += 3;
        reasons.push("security-sensitive task");
    }
    if has(&["production", "critical", "data loss"]) {
        score += 2;
        reasons.push("production or high-impact change");
    }

    // Working-tree files can be unrelated to the prompt. Only route on them
    // when the user explicitly mentions these particular changes.
    let refers_to_changes = has(&[
        "these changes",
        "current diff",
        "working tree",
        "staged changes",
    ]);
    if refers_to_changes && context.changed_paths.len() >= 5 {
        score += 2;
        reasons.push("many changed files referenced");
    }
    let sensitive_changed = refers_to_changes
        && context.changed_paths.iter().any(|path| {
            let path = path.to_ascii_lowercase();
            ["/auth/", "/security/", "/migrations/", "/payments/", ".env"]
                .iter()
                .any(|segment| path.contains(segment))
        });
    if sensitive_changed {
        score += 3;
        reasons.push("sensitive changed paths referenced");
    }

    score = score.min(10);
    let tier = if security || architecture || sensitive_changed || score >= 5 {
        Tier::Complex
    } else if score >= 2 {
        Tier::Standard
    } else {
        Tier::Simple
    };
    let uncertain = p.split_whitespace().count() < 4 || intent == Intent::General;

    if reasons.is_empty() {
        reasons.push("no complexity signals detected");
    }
    Decision {
        intent,
        tier,
        heuristic_score: score,
        is_repo: context.is_repo,
        changed_files: context.changed_paths.len(),
        reasons,
        uncertain,
    }
}

impl Decision {
    pub fn to_text(&self) -> String {
        format!(
            "Intent: {}\nTier: {} (advisory)\nSuggested effort: {} (not applied)\nHeuristic score: {}/10\nGit repo detected: {}\nTracked changed files: {}\nNeeds review: {}\nReasons: {}\nModel: agent default (not automatically overridden)",
            self.intent.as_str(),
            self.tier.as_str(),
            self.tier.suggested_effort(),
            self.heuristic_score,
            self.is_repo,
            self.changed_files,
            if self.uncertain { "yes" } else { "no" },
            self.reasons.join("; ")
        )
    }

    pub fn to_json(&self) -> String {
        let reasons = self
            .reasons
            .iter()
            .map(|reason| format!("\"{reason}\""))
            .collect::<Vec<_>>()
            .join(",");
        format!(
            "{{\"intent\":\"{}\",\"tier\":\"{}\",\"suggested_effort\":\"{}\",\"heuristic_score\":{},\"is_repo\":{},\"changed_files\":{},\"uncertain\":{},\"reasons\":[{}],\"model_override\":null}}",
            self.intent.as_str(),
            self.tier.as_str(),
            self.tier.suggested_effort(),
            self.heuristic_score,
            self.is_repo,
            self.changed_files,
            self.uncertain,
            reasons
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn route(prompt: &str) -> Decision {
        classify(prompt, &RepoContext::default())
    }

    #[test]
    fn words_inside_other_words_do_not_trigger_security() {
        assert_eq!(
            route("Update author attribution in README").tier,
            Tier::Simple
        );
        assert_eq!(
            route("Contest winners are listed in README").tier,
            Tier::Simple
        );
    }

    #[test]
    fn full_authentication_word_is_security_sensitive() {
        assert_eq!(
            route("Fix authentication login failure").tier,
            Tier::Complex
        );
    }

    #[test]
    fn standalone_add_is_a_feature_signal() {
        assert_eq!(route("Add profile page").tier, Tier::Standard);
    }

    #[test]
    fn phrase_matching_observes_word_boundaries() {
        assert!(contains_phrase("clean up this module", "clean up"));
        assert!(!contains_phrase("authors", "auth"));
        assert!(!contains_phrase("contest", "test"));
        assert!(contains_phrase("Fix auth bug", "auth"));
    }

    #[test]
    fn small_documentation_change_stays_simple() {
        assert_eq!(route("Fix typo in README").tier, Tier::Simple);
    }

    #[test]
    fn ordinary_feature_is_standard() {
        assert_eq!(route("Add a profile feature").tier, Tier::Standard);
    }

    #[test]
    fn sensitive_auth_fix_is_complex() {
        assert_eq!(route("Fix authentication redirect bug").tier, Tier::Complex);
    }

    #[test]
    fn architecture_work_is_complex() {
        assert_eq!(route("Redesign the architecture").tier, Tier::Complex);
    }

    #[test]
    fn concurrency_is_complex() {
        assert_eq!(
            route("Fix intermittent concurrency crash").tier,
            Tier::Complex
        );
    }

    #[test]
    fn unrelated_git_changes_do_not_affect_routing() {
        let context = RepoContext {
            is_repo: true,
            changed_paths: vec!["src/auth/login.rs".to_string(); 9],
        };
        assert_eq!(classify("Fix typo in README", &context).tier, Tier::Simple);
    }

    #[test]
    fn referenced_security_paths_raise_tier() {
        let context = RepoContext {
            is_repo: true,
            changed_paths: vec!["src/security/rules.rs".to_string()],
        };
        assert_eq!(
            classify("Review these changes", &context).tier,
            Tier::Complex
        );
    }

    #[test]
    fn short_prompts_are_uncertain() {
        assert!(route("fix this").uncertain);
    }

    #[test]
    fn json_output_has_expected_fields() {
        let json = route("Fix the regression").to_json();
        assert!(json.contains("\"intent\":\"debugging\""));
        assert!(json.contains("\"model_override\":null"));
    }
}
