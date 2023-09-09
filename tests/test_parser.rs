use tilted::{Lexer, Number, Parser};

macro_rules! make_parser_test {
    ($name: ident, $source: literal, $expected: literal) => {
        #[test]
        fn $name() {
            let lexer = Lexer::from_source_code($source);
            let mut parser = Parser::from_lexer(lexer);
            let node = parser.parse();

            assert!(node.is_ok());

            let actual = node.unwrap().evaluate();
            let expected = Number::from($expected);

            assert_eq!(actual, expected);
        }
    };
}

make_parser_test!(test_parser_one_int, "5", 5);
make_parser_test!(test_parser_one_flt, "5.0", 5.0);
make_parser_test!(test_parser_int_expr, "5 + 5", 10);
make_parser_test!(test_parser_flt_expr, "5.0 + 5.0", 10.0);
make_parser_test!(test_parser_int_flt_expr, "5 + 5.0", 10.0);
make_parser_test!(test_parser_order_of_expr, "7 + 6 * 2 - 4 * (8 + 3)", -25);
make_parser_test!(test_parser_unary_op, "-5", -5);
make_parser_test!(test_parser_multi_unary_op, "+--+5*+-+-5", 25);
make_parser_test!(test_parser_unary_op_expr, "-5 + 5", 0);
make_parser_test!(test_parser_impl_mul, "5(5)", 25);
make_parser_test!(test_parser_impl_mul_expr, "5(5 + 5)", 50);
make_parser_test!(test_parser_complex_expr, "2*-(3*(1+-(2)))^2", -18);
make_parser_test!(test_parser_impl_mul_func, "5sin(0)", 0.0);
