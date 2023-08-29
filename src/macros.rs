/// Returns a EOF token at the specified location.
#[macro_export]
macro_rules! eof {
    ($idx: expr) => {
        crate::lexer::Token {
            kind: crate::lexer::TokenKind::Eof,
            span: crate::lexer::Span {
                start_index: $idx,
                end_index: $idx,
            },
        }
    };
}

/// Quickly crafts a token of a specified kind of a known length and location.
#[macro_export]
macro_rules! token {
    ($kind: expr, $start: expr, $length: expr) => {
        crate::lexer::Token {
            kind: $kind,
            span: crate::lexer::Span {
                start_index: $start,
                end_index: $start + $length - 1,
            },
        }
    };
}