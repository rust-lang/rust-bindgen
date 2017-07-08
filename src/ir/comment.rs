//! Utilities for manipulating C/C++ comments.

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
pub fn preprocess(comment: String) -> String {
    match self::kind(&comment) {
        Some(Kind::SingleLines) => preprocess_single_lines(&comment),
        Some(Kind::MultiLine) => preprocess_multi_line(&comment),
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

/// Preprocesses mulitple single line comments.
///
/// Handles lines starting with both `//` and `///`.
fn preprocess_single_lines(comment: &str) -> String {
    assert!(comment.starts_with("//"), "comment is not single line");

    let lines: Vec<_> = comment.lines()
                               .map(|l| l.trim_left_matches('/').trim())
                               .map(|l| format!("/// {}", l).trim().to_owned())
                               .collect();
    lines.join("\n")
}

fn preprocess_multi_line(comment: &str) -> String {
    let comment = comment.trim_left_matches('/')
                         .trim_left_matches("*")
                         .trim_left_matches("!")
                         .trim_right_matches('/')
                         .trim_right_matches('*')
                         .trim();

    // Strip any potential `*` characters preceding each line.
    let mut lines: Vec<_> = comment.lines()
        .map(|line| line.trim().trim_left_matches('*').trim())
        .skip_while(|line| line.is_empty()) // Skip the first empty lines.
        .map(|line| format!("/// {}", line).trim().to_owned())
        .collect();

    // Remove the trailing `*/`.
    let last_idx = lines.len() - 1;
    if lines[last_idx].is_empty() {
        lines.remove(last_idx);
    }

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
        assert_eq!(preprocess("/// hello".to_owned()), "/// hello");
        assert_eq!(preprocess("// hello".to_owned()), "/// hello");
    }

    #[test]
    fn processes_multi_lines_correctly() {
        assert_eq!(preprocess("/** hello \n * world \n * foo \n */".to_owned()),
                   "/// hello\n/// world\n/// foo");

        assert_eq!(preprocess("/**\nhello\n*world\n*foo\n*/".to_owned()),
                   "/// hello\n/// world\n/// foo");
    }
}
