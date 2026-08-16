mod diagnostic;

pub use diagnostic::Diagnostic;
pub use diagnostic::Severity;

use crate::span::Span;
use owo_colors::OwoColorize;

pub struct DiagnosticFormatter<'a> {
    source: &'a [u8],
}

impl<'a> DiagnosticFormatter<'a> {
    pub fn new(source: &'a [u8]) -> Self {
        Self { source }
    }

    pub fn format(&self, diagnostic: &Diagnostic) -> String {
        let mut fmt = String::new();

        match diagnostic.severity {
            Severity::Error => {
                fmt.push_str(&format!("{} ", "error: ".bright_red().bold()));
            }
        }

        fmt.push_str(&format!("{} \n", &diagnostic.message.bold()));

        let (line, column) = self.get_line_col(&diagnostic.span);

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
            String::from_utf8_lossy(self.get_line(&diagnostic.span))
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

    fn get_line(&self, span: &Span) -> &[u8] {
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

    fn get_line_col(&self, span: &Span) -> (usize, usize) {
        let line_num = self.source[..span.offset()]
            .iter()
            .filter(|&c| *c == b'\n')
            .count()
            + 1;

        let line_start: usize = self.source[..span.offset()]
            .iter()
            .rposition(|&c| c == b'\n')
            .map(|i| i + 1)
            .unwrap_or(0);

        let col_num = span.offset() - line_start + 1;

        (line_num, col_num)
    }
}
