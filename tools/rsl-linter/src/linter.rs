use crate::rules::*;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub enum Severity {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub severity: Severity,
    pub message: String,
    pub file: PathBuf,
    pub line: usize,
    pub column: usize,
    pub rule: String,
    pub fix: Option<String>,
}

pub struct Linter {
    ignored_rules: Vec<String>,
}

impl Linter {
    pub fn new(ignored_rules: Vec<String>) -> Self {
        Self { ignored_rules }
    }

    pub fn lint_file(&self, file: &PathBuf, content: &str, fix: bool) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        if !self.is_ignored("missing-end") {
            diagnostics.extend(check_missing_end(file, &lines));
        }
        if !self.is_ignored("unclosed-string") {
            diagnostics.extend(check_unclosed_strings(file, &lines));
        }
        if !self.is_ignored("missing-semicolon") {
            diagnostics.extend(check_missing_semicolon(file, &lines));
        }
        if !self.is_ignored("single-equals-in-condition") {
            diagnostics.extend(check_single_equals_in_condition(file, &lines));
        }

        let check_record = !self.is_ignored("deprecated-record");
        let check_array = !self.is_ignored("deprecated-array");

        if check_record || check_array {
            diagnostics.extend(check_deprecated_usage(file, &lines, fix));
        }
        if !self.is_ignored("unused-variable") {
            diagnostics.extend(check_unused_variables(file, &lines));
        }
        if !self.is_ignored("empty-block") {
            diagnostics.extend(check_empty_blocks(file, &lines));
        }
        if !self.is_ignored("trailing-whitespace") {
            diagnostics.extend(check_trailing_whitespace(file, &lines, fix));
        }
        if !self.is_ignored("empty-whitespace-line") {
            diagnostics.extend(check_empty_whitespace_line(file, &lines, fix));
        }

        diagnostics
    }

    fn is_ignored(&self, rule: &str) -> bool {
        self.ignored_rules.contains(&rule.to_string())
    }
}