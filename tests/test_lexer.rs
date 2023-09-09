use tilted::{Function::*, Lexer, Operator::*, TokenKind::*};

macro_rules! make_lexer_test {
    ($name: ident, $source: literal, [$($token_kind: expr,)*]) => {
        #[test]
        fn $name() {
            let mut lexer = Lexer::from_source_code($source);
            $(
                assert_eq!(lexer.lex().unwrap().kind, $token_kind);
            )*
        }
    };
    (E: $name: ident, $source: literal) => {
        #[test]
        fn $name() {
            let mut lexer = Lexer::from_source_code($source);
            assert!(lexer.lex().is_err());
        }
    };
}

make_lexer_test!(test_lexer_empty, "", [Eof,]);

make_lexer_test!(test_lexer_one_int, "8", [Int(8), Eof,]);

make_lexer_test!(test_lexer_one_flt, "9.0", [Flt(9.0), Eof,]);

make_lexer_test!(
    test_lexer_one_int_and_flt,
    "8 9.0",
    [Int(8), Flt(9.0), Eof,]
);

make_lexer_test!(
    test_lexer_ops,
    "+ - * / ^",
    [Op(Plus), Op(Minus), Op(Star), Op(Slash), Op(Caret), Eof,]
);

make_lexer_test!(test_lexer_parens, "( )", [LeftParen, RightParen, Eof,]);

make_lexer_test!(
    test_lexer_simple_expr,
    "8 + 9.0 * 2",
    [Int(8), Op(Plus), Flt(9.0), Op(Star), Int(2), Eof,]
);

make_lexer_test!(
    test_lexer_expr_with_parens,
    "(8 + 9.0) * 2",
    [
        LeftParen,
        Int(8),
        Op(Plus),
        Flt(9.0),
        RightParen,
        Op(Star),
        Int(2),
        Eof,
    ]
);

make_lexer_test!(
    test_lexer_expr_with_parens_and_unary,
    "-(8 + 9.0) * 2",
    [
        Op(Minus),
        LeftParen,
        Int(8),
        Op(Plus),
        Flt(9.0),
        RightParen,
        Op(Star),
        Int(2),
        Eof,
    ]
);

make_lexer_test!(
    test_lexer_trigo,
    "sin cos tan",
    [Func(Sin), Func(Cos), Func(Tan), Eof,]
);

make_lexer_test!(
    test_lexer_trigo_with_expr,
    "sin(3.14)",
    [Func(Sin), LeftParen, Flt(3.14), RightParen, Eof,]
);

make_lexer_test!(E: test_lexer_too_many_decimals, "9.0.0");
make_lexer_test!(E: test_lexer_invalid_char, "a");
