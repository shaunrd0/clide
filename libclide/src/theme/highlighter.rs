use std::fs;
use std::path::Path;
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::html::{IncludeBackground, append_highlighted_html_for_styled_line};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

pub struct Highlighter {
    path: String,
    ss: SyntaxSet,
    ts: ThemeSet,
}

impl Highlighter {
    pub fn new<P: AsRef<Path>>(p: P) -> anyhow::Result<Highlighter> {
        let path = p.as_ref();
        let meta =
            fs::metadata(path).unwrap_or_else(|_| panic!("Failed to get file metadata {path:?}"));
        let ss = SyntaxSet::load_defaults_nonewlines();
        let ts = ThemeSet::load_defaults();
        if !meta.is_file() {
            crate::error!(target:"FileSystem", "Attempted to open file {path:?} that is not a valid file");
            Err(anyhow::anyhow!(
                "Attempted to open file {path:?} that is not a valid file"
            ))?;
        }
        Ok(Highlighter {
            path: path.to_string_lossy().to_string(),
            ss,
            ts,
        })
    }

    pub fn syntax_highlight_text<P: AsRef<str>>(&self, p: P) -> String {
        let text = p.as_ref();
        let theme = &self.ts.themes["base16-ocean.dark"];
        let lang = self
            .ss
            .find_syntax_by_extension(
                Path::new(self.path.as_str())
                    .extension()
                    .map(|s| s.to_str())
                    .unwrap_or_else(|| Some("md"))
                    .expect("Failed to get file extension"),
            )
            .unwrap_or_else(|| self.ss.find_syntax_plain_text());
        let mut highlighter = HighlightLines::new(lang, theme);
        // If you care about the background, see `start_highlighted_html_snippet(theme);`.
        let mut output = String::from("<pre>\n");
        for line in LinesWithEndings::from(text) {
            let regions = highlighter
                .highlight_line(line, &self.ss)
                .expect("Failed to highlight");

            append_highlighted_html_for_styled_line(
                &regions[..],
                IncludeBackground::No,
                &mut output,
            )
            .expect("Failed to insert highlighted html");
        }
        output.push_str("</pre>\n");
        output
    }
}
