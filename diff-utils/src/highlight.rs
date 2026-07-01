//! Syntax highlighting for the diff panels, built on `syntect`.
//!
//! Uses the pure-Rust `regex-fancy` engine (no oniguruma C dependency) so the
//! build stays friendly to conda/pixi packaging. A small custom `.log` syntax
//! is registered on top of syntect's default syntax set so log files get
//! colored timestamps and log levels.

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Span;
use std::path::Path;
use syntect::easy::HighlightLines;
use syntect::highlighting::{FontStyle, Style as SynStyle, Theme, ThemeSet};
use syntect::parsing::{SyntaxDefinition, SyntaxReference, SyntaxSet};

/// A minimal Sublime-style syntax for generic log files: timestamps and the
/// common log levels (ERROR/WARN/INFO/DEBUG) are colored via standard scopes.
const LOG_SYNTAX: &str = r#"%YAML 1.2
---
name: Log
file_extensions: [log, syslog, out]
scope: text.log
contexts:
  main:
    - match: '\b\d{4}-\d{2}-\d{2}[T ]\d{2}:\d{2}:\d{2}(?:\.\d+)?(?:Z|[+-]\d{2}:?\d{2})?\b'
      scope: comment.other.timestamp.log
    - match: '\b\d{2}:\d{2}:\d{2}(?:\.\d+)?\b'
      scope: comment.other.timestamp.log
    - match: '\b(?i:ERROR|ERR|FATAL|CRITICAL|SEVERE|PANIC)\b'
      scope: keyword.control.error.log
    - match: '\b(?i:WARN(?:ING)?)\b'
      scope: keyword.control.warn.log
    - match: '\b(?i:INFO|NOTICE)\b'
      scope: keyword.control.info.log
    - match: '\b(?i:DEBUG|TRACE|FINE(?:ST|R)?)\b'
      scope: keyword.control.debug.log
    - match: '\b([A-Z][A-Z0-9_]{2,})\b'
      scope: support.constant.log
"#;

/// Owns the syntect state needed to highlight any supported file.
pub struct HighlightEngine {
    syntax_set: SyntaxSet,
    theme: Theme,
}

impl HighlightEngine {
    pub fn new() -> Self {
        let mut builder = SyntaxSet::load_defaults_newlines().into_builder();
        if let Ok(log_def) = SyntaxDefinition::load_from_str(LOG_SYNTAX, false, Some("log")) {
            builder.add(log_def);
        }
        let syntax_set = builder.build();

        let theme_set = ThemeSet::load_defaults();
        let theme = theme_set
            .themes
            .get("base16-ocean.dark")
            .or_else(|| theme_set.themes.get("base16-eighties.dark"))
            .or_else(|| theme_set.themes.values().next())
            .cloned()
            .expect("syntect default theme set is non-empty");

        HighlightEngine { syntax_set, theme }
    }

    /// Pick a syntax reference for `path`, by extension then first-line hint.
    /// Returns `None` for unrecognized files (caller falls back to plain text).
    pub fn syntax_for_path(&self, path: &Path) -> Option<SyntaxReference> {
        match self.syntax_set.find_syntax_for_file(path) {
            Ok(Some(syntax)) => Some(syntax.clone()),
            _ => self
                .syntax_set
                .find_syntax_by_extension(path.extension()?.to_str()?)
                .cloned(),
        }
    }

    /// Highlight every line of `text`, returning per-line styled `Span`s
    /// indexed in source order (line 1 → index 0). Multi-line constructs
    /// (e.g. Python triple-quoted strings) stay correct because the
    /// `HighlightLines` state is advanced sequentially over the whole file.
    pub fn highlight_text(&self, syntax: &SyntaxReference, text: &str) -> Vec<Vec<Span<'static>>> {
        let mut highlighter = HighlightLines::new(syntax, &self.theme);
        let mut out = Vec::with_capacity(text.lines().count());
        for (i, line) in text.lines().enumerate() {
            if i == 0 && line.starts_with("#!") {
                // syntect's Python grammar mishandles shebangs and poisons parser
                // state for the rest of the file. Render it as a comment without
                // advancing the highlighter.
                out.push(vec![Span::styled(
                    line.to_string(),
                    Style::default().fg(Color::Indexed(SHEBANG_COLOR)),
                )]);
                continue;
            }

            let regions = highlighter.highlight_line(line, &self.syntax_set);
            let spans = match regions {
                Ok(regions) => regions_to_spans(&regions, line),
                Err(_) => vec![Span::raw(line.to_string())],
            };
            out.push(spans);
        }
        out
    }
}

/// Muted gray from the base16-ocean palette, used for shebang lines.
const SHEBANG_COLOR: u8 = 66;

/// Build display spans from syntect regions, always using text from `display_line`.
fn regions_to_spans(regions: &[(SynStyle, &str)], display_line: &str) -> Vec<Span<'static>> {
    let mut spans = Vec::with_capacity(regions.len());
    let mut col = 0usize;
    for (style, region_text) in regions {
        let len = region_text.len();
        let end = col.saturating_add(len).min(display_line.len());
        let slice = &display_line[col..end];
        col = end;
        if !slice.is_empty() {
            spans.push(Span::styled(slice.to_string(), to_tui_style(*style)));
        }
    }
    if col < display_line.len() {
        spans.push(Span::raw(display_line[col..].to_string()));
    }
    spans
}

/// Convert a syntect `Style` to a ratatui `Style`. Only foreground color and
/// font modifiers are carried over; background is left unset so the diff row's
/// background highlight (applied in `ui.rs`) shows through.
fn to_tui_style(style: SynStyle) -> Style {
    let mut modifier = Modifier::empty();
    if style.font_style.contains(FontStyle::BOLD) {
        modifier |= Modifier::BOLD;
    }
    if style.font_style.contains(FontStyle::ITALIC) {
        modifier |= Modifier::ITALIC;
    }
    if style.font_style.contains(FontStyle::UNDERLINE) {
        modifier |= Modifier::UNDERLINED;
    }
    Style::default()
        .fg(color_to_tui(style.foreground))
        .add_modifier(modifier)
}

fn color_to_tui(c: syntect::highlighting::Color) -> Color {
    // Use the 256-color palette for syntax foregrounds. Truecolor (`38;2`) is
    // often dropped by terminals and screen recorders (including VHS) while
    // indexed colors (`38;5`) render reliably alongside diff backgrounds.
    Color::Indexed(rgb_to_256(c.r, c.g, c.b))
}

/// Map an sRGB triplet to the xterm 256-color palette (16 + 6×6×6 cube + gray).
fn rgb_to_256(r: u8, g: u8, b: u8) -> u8 {
    if r == g && g == b {
        if r < 8 {
            return 0;
        }
        if r > 248 {
            return 15;
        }
        return ((r as u16 - 8) / 10 + 232) as u8;
    }
    16 + 36 * (r as u16 / 51) as u8 + 6 * (g as u16 / 51) as u8 + (b as u16 / 51) as u8
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::path::Path;

    #[test]
    fn python_syntax_produces_distinct_colors() {
        let engine = HighlightEngine::new();
        let syntax = engine.syntax_for_path(Path::new("test.py")).unwrap();
        let text = "def add(a: float, b: float) -> float:\nfrom typing import List\nx = \"hello\"\n";
        let lines = engine.highlight_text(&syntax, text);
        let mut colors = HashSet::new();
        for spans in &lines {
            for s in spans {
                if let Some(Color::Indexed(idx)) = s.style.fg {
                    colors.insert(idx);
                }
            }
        }
        assert!(colors.len() > 1, "expected multiple syntax colors, got: {colors:?}");
    }

    #[test]
    fn shebang_does_not_break_python_highlighting() {
        let engine = HighlightEngine::new();
        let syntax = engine.syntax_for_path(Path::new("calculator.py")).unwrap();
        let text = std::fs::read_to_string("/workspace/demo/calculator_old.py").unwrap();
        let lines = engine.highlight_text(&syntax, &text);
        let def_line = &lines[6]; // line 7: def add(...)
        let colors: HashSet<_> = def_line.iter().filter_map(|s| s.style.fg).collect();
        assert!(
            def_line.len() > 1 && colors.len() > 1,
            "expected tokenized def line, got {} spans / {colors:?}",
            def_line.len()
        );
    }
}
