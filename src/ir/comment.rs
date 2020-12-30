//! Utilities for manipulating C/C++ comments.

use std::iter;

/// The type of a comment.
#[derive(Debug, PartialEq, Eq)]
enum Kind {
    /// A `///` comment, or something of the like.
    /// All lines in a comment should start with the same symbol.
    SingleLines,
    /// A `/**` comment, where each other line can start with `*` and the
    /// entire block ends with `*/`.
    MultiLine,
}

/// Preprocesses a C/C++ comment so that it is a valid Rust comment.
pub fn preprocess(comment: &str, indent: usize) -> String {
    match self::kind(&comment) {
        Some(Kind::SingleLines) => preprocess_single_lines(comment, indent),
        Some(Kind::MultiLine) => preprocess_multi_line(comment, indent),
        None => comment.to_owned(),
    }
}

/// Gets the kind of the doc comment, if it is one.
fn kind(comment: &str) -> Option<Kind> {
    if comment.starts_with("/*") {
        Some(Kind::MultiLine)
    } else if comment.starts_with("//") {
        Some(Kind::SingleLines)
    } else {
        None
    }
}

fn make_indent(indent: usize) -> String {
    const RUST_INDENTATION: usize = 4;
    iter::repeat(' ').take(indent * RUST_INDENTATION).collect()
}

/// Preprocesses multiple single line comments.
///
/// Handles lines starting with both `//` and `///`.
fn preprocess_single_lines(comment: &str, indent: usize) -> String {
    debug_assert!(comment.starts_with("//"), "comment is not single line");

    let indent = make_indent(indent);
    let mut is_first = true;
    let mut lines = vec![format!("{}///```text", indent)];
    let comments: Vec<_> = comment
        .replace("`", "\\`")
        .lines()
        .map(|line| line.trim().trim_start_matches('/'))
        .map(|line| {
            let indent = if is_first { "" } else { &*indent };
            is_first = false;
            format!("{}///{}", indent, line)
        })
        .collect();
    lines.extend(comments);
    lines.push(format!("{}///```", indent));
    lines.join("\n")
}

fn preprocess_multi_line(comment: &str, indent: usize) -> String {
    let comment = comment
        .trim_start_matches('/')
        .trim_end_matches('/')
        .trim_end_matches('*')
        .replace("`", "\\`");

    let indent = make_indent(indent);
    // Strip any potential `*` characters preceding each line.
    let mut lines = vec![format!("{}///```text", indent)];
    let mut is_first = true;
    let mut comments: Vec<_> = comment
        .lines()
        .map(|line| line.trim().trim_start_matches('*').trim_start_matches('!'))
        .skip_while(|line| line.trim().is_empty()) // Skip the first empty lines.
        .map(|line| {
            let indent = if is_first { "" } else { &*indent };
            is_first = false;
            format!("{}///{}", indent, line)
        })
        .collect();

    // Remove the trailing line corresponding to the `*/`.
    if comments
        .last()
        .map_or(false, |l| l.trim().is_empty() || l.trim() == "///")
    {
        comments.pop();
    }

    lines.extend(comments);
    lines.push(format!("{}///```", indent));
    lines.join("\n")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn picks_up_single_and_multi_line_doc_comments() {
        assert_eq!(kind("/// hello"), Some(Kind::SingleLines));
        assert_eq!(kind("/** world */"), Some(Kind::MultiLine));
    }

    #[test]
    fn processes_single_lines_correctly() {
        assert_eq!(preprocess("/// hello", 0), "///```text\n/// hello\n///```");
        assert_eq!(preprocess("// hello", 0), "///```text\n/// hello\n///```");
        assert_eq!(
            preprocess("//    hello", 0),
            "///```text\n///    hello\n///```"
        );
        assert_eq!(
            preprocess("//    hello[i]", 0),
            "///```text\n///    hello[i]\n///```"
        );
    }

    #[test]
    fn processes_multi_lines_correctly() {
        assert_eq!(
            preprocess("/** hello \n * world \n * foo \n */", 0),
            "///```text\n/// hello\n/// world\n/// foo\n///```"
        );

        assert_eq!(
            preprocess("/**\nhello\n*world\n*foo\n*/", 0),
            "///```text\n///hello\n///world\n///foo\n///```"
        );

        assert_eq!(
            preprocess("/**\nhello\n*world[i]\n*foo\n*/", 0),
            "///```text\n///hello\n///world[i]\n///foo\n///```"
        );
    }
}
