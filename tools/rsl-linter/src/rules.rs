// src/rules.rs
use crate::linter::{Diagnostic, Severity};
use regex::Regex;
use std::path::PathBuf;

struct Scrubber {
    in_block_comment: bool,
}

impl Scrubber {
    fn new() -> Self {
        Self { in_block_comment: false }
    }
}

struct CodeCleaner {
    in_block_comment: bool,
}

impl CodeCleaner {
    fn new() -> Self {
        Self { in_block_comment: false }
    }

    fn clean(&mut self, line: &str) -> String {
        let mut result = String::with_capacity(line.len());
        let mut in_string = false;
        let mut chars = line.chars().peekable();

        while let Some(c) = chars.next() {
            if self.in_block_comment {
                if c == '*' && chars.peek() == Some(&'/') {
                    self.in_block_comment = false;
                    chars.next();
                }
                result.push(' ');
                continue;
            }

            if !in_string {
                if c == '/' && chars.peek() == Some(&'*') {
                    self.in_block_comment = true;
                    chars.next();
                    result.push(' '); result.push(' ');
                    continue;
                }
                if c == '/' && chars.peek() == Some(&'/') {
                    break; 
                }
            }

            if c == '"' {
                in_string = !in_string;
                result.push('"');
                continue;
            }

            if in_string {
                result.push(' ');
            } else {
                result.push(c);
            }
        }

        while result.len() < line.len() {
            result.push(' ');
        }
        
        result
    }
}


pub fn check_unclosed_strings(file: &PathBuf, lines: &[&str]) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let mut scrubber = Scrubber::new();

    for (i, line) in lines.iter().enumerate() {
        let mut in_string = false;
        let mut chars = line.chars().peekable();

        while let Some(c) = chars.next() {
            if scrubber.in_block_comment {
                if c == '*' && chars.peek() == Some(&'/') { scrubber.in_block_comment = false; chars.next(); }
                continue;
            }
            if c == '/' && chars.peek() == Some(&'*') { scrubber.in_block_comment = true; chars.next(); continue; }
            if c == '/' && chars.peek() == Some(&'/') { break; }
            
            if c == '"' { in_string = !in_string; }
        }

        if in_string {
            diagnostics.push(Diagnostic {
                severity: Severity::Error,
                message: "Незакрытая строка.".to_string(),
                file: file.clone(),
                line: i + 1,
                column: line.rfind('"').unwrap_or(0) + 1,
                rule: "unclosed-string".to_string(),
                fix: None,
            });
        }
    }
    diagnostics
}

pub fn check_missing_semicolon(file: &PathBuf, lines: &[&str]) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let mut cleaner = CodeCleaner::new();
    let mut open_parens = 0;

    let skip_re = Regex::new(r"(?i)\b(Macro|Class|If|Else|Elif|For|While|With|Return|Import|Const|Var|Private|Local)\b").unwrap();

    for (i, line) in lines.iter().enumerate() {
        let clean = cleaner.clean(line);
        let trimmed = clean.trim();
        
        if trimmed.is_empty() { continue; }

        let opened_in_line = clean.matches('(').count() as i32;
        let closed_in_line = clean.matches(')').count() as i32;
        
        let prev_open_parens = open_parens;
        open_parens += opened_in_line - closed_in_line;

        let ends_with_special = trimmed.ends_with(';') || 
                                trimmed.ends_with('{') || 
                                trimmed.ends_with('}') || 
                                trimmed.ends_with(',');
        
        let is_keyword_start = skip_re.is_match(trimmed);

        let in_multiline_condition = open_parens > 0 || (prev_open_parens > 0 && trimmed.ends_with(')'));

        if !ends_with_special && !is_keyword_start && !in_multiline_condition {
            diagnostics.push(Diagnostic {
                severity: Severity::Warning,
                message: "Отсутствует точка с запятой.".to_string(),
                file: file.clone(),
                line: i + 1,
                column: line.len() + 1,
                rule: "missing-semicolon".to_string(),
                fix: Some(format!("{};", line)),
            });
        }
    }
    diagnostics
}


pub fn check_single_equals_in_condition(file: &PathBuf, lines: &[&str]) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let mut cleaner = CodeCleaner::new();
    let mut buffer = String::new();
    let mut active = false;
    
    let trigger = Regex::new(r"(?i)\b(if|elif|while)\b").unwrap();
    let bad_eq = Regex::new(r"[^<>=!]=\s*[^=]").unwrap();

    for (i, line) in lines.iter().enumerate() {
        let clean = cleaner.clean(line);
        if trigger.is_match(&clean) || active {
            active = true;
            buffer.push_str(&clean);
            if buffer.matches('(').count() == buffer.matches(')').count() && buffer.contains('(') {
                if bad_eq.is_match(&buffer) {
                    diagnostics.push(create_diag(file, i + 1, "Используйте '==' для сравнения.", "single-equals"));
                }
                buffer.clear();
                active = false;
            }
        }
    }
    diagnostics
}

pub fn check_trailing_whitespace(file: &PathBuf, lines: &[&str], fix: bool) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    
    for (i, line) in lines.iter().enumerate() {
        let trimmed_end = line.trim_end();
        let trailing_count = line.len() - trimmed_end.len();
        
        if trailing_count > 0 {
            diagnostics.push(Diagnostic {
                severity: Severity::Warning,
                message: format!("Лишние пробелы в конце строки ({}).", trailing_count),
                file: file.clone(),
                line: i + 1,
                column: trimmed_end.len() + 1,
                rule: "trailing-whitespace".to_string(),
                fix: if fix { Some(trimmed_end.to_string()) } else { None },
            });
        }
    }
    diagnostics
}

pub fn check_empty_whitespace_line(file: &PathBuf, lines: &[&str], fix: bool) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    
    for (i, line) in lines.iter().enumerate() {
        if !line.is_empty() && line.chars().all(|c| c.is_whitespace()) {
            diagnostics.push(Diagnostic {
                severity: Severity::Info,
                message: "Строка содержит только пробелы.".to_string(),
                file: file.clone(),
                line: i + 1,
                column: 1,
                rule: "empty-whitespace-line".to_string(),
                fix: if fix { Some(String::new()) } else { None },
            });
        }
    }
    diagnostics
}

pub fn check_deprecated_usage(file: &PathBuf, lines: &[&str], fix: bool) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let mut cleaner = CodeCleaner::new();
    
    let re_record = Regex::new(r"\brecord\b").unwrap();
    let re_array = Regex::new(r"\barray\b").unwrap();
    
    for (i, line) in lines.iter().enumerate() {
        let clean = cleaner.clean(line);

        if let Some(mat) = re_record.find(&clean) {
            diagnostics.push(create_deprecated_diag(file, i+1, mat.start()+1, "record", "TRecHandler", fix, line));
        }

        if let Some(mat) = re_array.find(&clean) {
            diagnostics.push(create_deprecated_diag(file, i+1, mat.start()+1, "array", "TArray", fix, line));
        }
    }
    diagnostics
}

fn create_deprecated_diag(file: &PathBuf, line: usize, col: usize, old: &str, new: &str, fix: bool, original: &str) -> Diagnostic {
    Diagnostic {
        severity: Severity::Warning,
        message: format!("'{}' устарел. Используйте '{}'.", old, new),
        file: file.clone(),
        line,
        column: col,
        rule: format!("deprecated-{}", old),
        fix: if fix { Some(original.replace(old, new)) } else { None },
    }
}

pub fn check_unused_variables(file: &PathBuf, lines: &[&str]) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let mut cleaner = CodeCleaner::new();
    let mut var_declarations = Vec::new();
    let mut cleaned_lines = Vec::new();

    let var_re = Regex::new(r"(?i)\bVar\s+([a-zA-Z_][a-zA-Z0-9_]*)").unwrap();

    for (i, line) in lines.iter().enumerate() {
        let clean = cleaner.clean(line);
        if let Some(cap) = var_re.captures(&clean) {
            var_declarations.push((cap[1].to_string(), i + 1, cap.get(1).unwrap().start() + 1));
        }
        cleaned_lines.push(clean);
    }

    for (name, line_num, col) in var_declarations {
        let usage_re = Regex::new(&format!(r"\b{}\b", regex::escape(&name))).unwrap();
        let count = cleaned_lines.iter().map(|l| usage_re.find_iter(l).count()).sum::<usize>();

        if count <= 1 {
            diagnostics.push(Diagnostic {
                severity: Severity::Warning,
                message: format!("Переменная '{}' не используется.", name),
                file: file.clone(),
                line: line_num,
                column: col,
                rule: "unused-variable".to_string(),
                fix: None,
            });
        }
    }
    diagnostics
}

pub fn check_empty_blocks(file: &PathBuf, lines: &[&str]) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let mut cleaner = CodeCleaner::new();
    let mut last_block_start = None;

    let re_start = Regex::new(r"(?i)\b(Macro|Class|If|For|While|With)\b").unwrap();
    let re_end = Regex::new(r"(?i)\bEnd\b").unwrap();

    for (i, line) in lines.iter().enumerate() {
        let clean = cleaner.clean(line);
        let line_num = i + 1;

        if re_start.is_match(&clean) {
            last_block_start = Some(line_num);
        } else if re_end.is_match(&clean) {
            if let Some(start_line) = last_block_start {
                if start_line == line_num - 1 {
                    diagnostics.push(Diagnostic {
                        severity: Severity::Warning,
                        message: "Пустой блок кода.".to_string(),
                        file: file.clone(),
                        line: start_line,
                        column: 1,
                        rule: "empty-block".to_string(),
                        fix: None,
                    });
                }
            }
            last_block_start = None;
        } else if !clean.trim().is_empty() {
            last_block_start = None;
        }
    }
    diagnostics
}

pub fn check_missing_end(file: &PathBuf, lines: &[&str]) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let mut stack = Vec::new();
    let mut cleaner = CodeCleaner::new();
    
    let re_start = Regex::new(r"(?i)\b(Macro|Class|If|For|While|With)\b").unwrap();
    let re_end = Regex::new(r"(?i)\bEnd\b").unwrap();

    for (i, line) in lines.iter().enumerate() {
        let clean = cleaner.clean(line);
        let line_num = i + 1;

        if re_start.is_match(&clean) && re_end.is_match(&clean) { continue; }

        if let Some(mat) = re_start.find(&clean) {
            stack.push((mat.as_str().to_string(), line_num));
        } else if re_end.is_match(&clean) {
            if stack.pop().is_none() {
                diagnostics.push(create_diag(file, line_num, "Лишний 'End'.", "orphaned-end"));
            }
        }
    }

    for (name, line) in stack {
        diagnostics.push(create_diag(file, line, &format!("Незакрытый блок '{}'.", name), "missing-end"));
    }
    diagnostics
}

fn create_diag(file: &PathBuf, line: usize, msg: &str, rule: &str) -> Diagnostic {
    Diagnostic {
        severity: Severity::Error,
        message: msg.to_string(),
        file: file.clone(),
        line,
        column: 1,
        rule: rule.to_string(),
        fix: None,
    }
}