# Parsing rules for `cal`

The parser for `cal` uses a recursive descent parsing technique.

## Why Recursive Descent Parsing?

*tl;dr: Ease of implementation*

`cal` is a non-Turing-complete programming language with an overly simple
structure - the whole *language* consisting of a single mathematical expression.
Therefore, it would be a shame if we have to use some lexer-parser generators
such as Yacc/Lex, ANTLR, etc. These are simply too redundant for `cal`.

I finally choose Recursive Descent Parsing as it nicely goes with the simplicity
of the language. It can easily be handwritten, which eliminates the needs for
external crates, and is theoretically faster than other parsing techniques,
since it does not require backtracking.

## Grammar design

As a disclaimer, I have taken *zero* class on programming language design, so
the grammar below is as bad as it can be. Though at the very least, it seems
intuitive enough to me, so I will stick to it in the mean time.

Also, grammar revisions are Semantically Versioned. I aim to keep the version 1
as far as possible. 

### Version 1.0

**Date**: August 31, 2023

This version was adopted as soon as I started this project.

```
start       :=  expr

# expr = term [+|- term]*
expr        :=  term_ifx
term_ifx    :=  term + term_ifx
            :=  term - term_ifx
            :=  term

# term = factor [*|/ factor]*
term        :=  factor_ifx
factor_ifx  :=  factor * factor_ifx
            :=  factor / factor_ifx
            :=  factor

# factor = unary_op? atomic
factor      :=  atomic
            :=  + atomic
            :=  - atomic

# atomic = number | ( expr )
atomic      :=  number
            :=  LeftParen expr RightParen

# number = Flt | Int
number      :=  Flt
            :=  Int
```

This grammar implements 4 basic mathematical operators as well as the use of
parentheses.