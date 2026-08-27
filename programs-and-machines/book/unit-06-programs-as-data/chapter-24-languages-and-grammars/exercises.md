# Exercises

**24.1** Write down three strings that are in the language of legal Java programs
and three that are not. For each rejected one, say whether it fails on syntax or
on meaning, and which component of a compiler would catch it.

**24.2** Using the ambiguous grammar of Section 24.1.2, draw both derivation trees
for `1 + 2 * 3 + 4`. There are more than two — find them all and say how many
there are.

**24.3** Extend the layered grammar to include a comparison operator `<` that
binds more loosely than `+`. Write the three rules that change or are added, then
say what `1 + 2 < 3 * 4` parses to.

**24.4** Implement the tokenizer of Section 24.2.1. Confirm that
`tokenize("2 + 3 * width")` produces five tokens and an `END`. Then feed it `1x`
and explain the two tokens you get.

**24.5** Add support for a `%` operator at the same precedence as `*` and `/`.
Count the places you had to change. Then add `**` for exponentiation, which is
right-associative and binds tighter than `*` — count again, and say why the second
change was larger.

**24.6** Add unary minus, so that `-3 + 4` and `2 * -3` both parse. The grammar
rule is `factor := '-' factor | NUMBER | NAME | '(' expression ')'`. Explain why
that rule is recursive and what `--3` therefore does.

**24.7** Write a `Expr` walker that returns the set of variable names appearing in
a tree. Confirm `width * 2 + height * height` gives two names. Your method should
have the same shape as `depth` from Section 24.2.2.

**24.8** Write a walker that prints the expression back in ordinary infix form
with the fewest necessary parentheses. Test it on `(2 + 3) * 4` and on
`2 + 3 * 4`. The second should print without any brackets, and getting that right
is the exercise.

**24.9** Remove the `expect(END)` from `parse()` and find an input that now
succeeds when it should fail. Then remove the `expect(RPAREN)` from `factor` and
find another. Report both.

**24.10** Feed the parser a string of five thousand open parentheses. Report what
happens and connect it to Section 24.1.1's claim that context-free languages need
a stack, and to Chapter 12.

**24.11** *Longer.* Write a grammar for a small configuration format: a file is a
sequence of sections, a section has a bracketed name and a sequence of key-value
lines, and values may be quoted strings or bare words. Then write the tokenizer
and recursive-descent parser for it. This is a genuinely useful thing to be able
to do and it is the same skill as the arithmetic parser at a slightly larger
scale.

**24.12** [carries forward] Keep your parser. Chapter 25 extends this exact code
into an interpreter, and the exercises there assume you have `tokenize`, the `Expr`
types, and `parse` working. If you skipped the implementation, write it now.
