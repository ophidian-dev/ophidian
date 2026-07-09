use crate::diagnostics::{Diagnostic, Severity};
use crate::span::Span;
use owo_colors::OwoColorize;

pub struct DiagnosticEmitter<'a> {
    source: &'a [u8],
}

impl<'a> DiagnosticEmitter<'a> {
    pub fn new(source: &'a [u8]) -> Self {
        Self { source }
    }
    pub fn emit(&self, diagnostic: &Diagnostic) -> String {
        let mut fmt = String::new();

        match diagnostic.severity {
            Severity::Error => {
                fmt.push_str(&format!("{} ", "error:".bright_red().bold()));
            }
        }

        fmt.push_str(&format!("{} \n", &diagnostic.message.bold()));

        let line = self.get_line_num(diagnostic.span);
        let column = self.get_col_num(diagnostic.span);

        fmt.push_str(&format!(
            "  {} placeholderpath.op:",
            "-->".bright_blue().bold()
        ));
        fmt.push_str(&format!("{}:{}\n", line, column));

        for _ in 0..line.to_string().len() + 1 {
            fmt.push(' ');
        }

        fmt.push_str("|\n");
        fmt.push_str(&format!(
            "{} | {}\n",
            line,
            String::from_utf8_lossy(self.get_line(diagnostic.span))
        ));

        for _ in 0..line.to_string().len() + 1 {
            fmt.push(' ');
        }
        fmt.push_str("| ");

        fmt.push_str(&" ".repeat(column - 1));
        fmt.push_str(&format!(
            "{}",
            "^".repeat(diagnostic.span.len().max(1)).green().bold()
        ));

        fmt
    }

    fn get_line(&self, span: Span) -> &[u8] {
        let mut start = span.offset();
        let mut finish = span.end();

        while start > 0 && self.source[start - 1] != b'\n' {
            start -= 1;
        }

        while finish < self.source.len() && self.source[finish] != b'\n' {
            finish += 1;
        }

        &self.source[start..finish]
    }

    fn get_line_num(&self, span: Span) -> usize {
        self.source[..span.offset()]
            .iter()
            .filter(|&c| *c == b'\n')
            .count()
            + 1
    }

    fn get_col_num(&self, span: Span) -> usize {
        let line_start: usize = self.source[..span.offset()]
            .iter()
            .rposition(|&c| c == b'\n')
            .map(|i| i + 1)
            .unwrap_or(0);

        span.offset() - line_start + 1
    }
}
