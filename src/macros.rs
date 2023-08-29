/// Returns a EOF token at the specified location.
#[macro_export]
macro_rules! eof {
    ($idx: expr) => {
        crate::lexer::Token {
            kind: crate::lexer::TokenKind::Eof,
            span: crate::lexer::Span {
                start_index: $idx,
                end_index: $idx,
            }
        }
    };
}