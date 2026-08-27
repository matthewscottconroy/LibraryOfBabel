# Grammars

A **grammar** is a finite set of rules that generates a language.

Here is one, for a language of arithmetic:

```
expression := NUMBER
            | expression '+' expression
            | expression '*' expression
            | '(' expression ')'
```

Four rules. Read `:=` as "can be", and `|` as "or". So an expression can be a
number, or two expressions with a `+` between them, or two with a `*`, or a
parenthesized expression.

That finite text describes infinitely many strings, because the second rule can be
applied to its own output as often as you like.

## Terminals and non-terminals

Two kinds of symbol, and keeping them apart is most of reading a grammar.

**Terminals** are the actual symbols of the language — `+`, `*`, `(`, `)`, and
`NUMBER`. They appear in the strings the grammar produces, and nothing expands
them further. They are the leaves.

**Non-terminals** are names for grammatical categories — `expression` here. They
never appear in a final string; they exist to be replaced. They are the internal
nodes.

One non-terminal is the **start symbol**, and a string is in the language exactly
when you can begin from the start symbol, replace non-terminals by their rules
repeatedly, and arrive at that string.

## A derivation

Deriving `2 + 3 * 4`, one replacement at a time:

```
expression
expression '+' expression                  rule 2
NUMBER '+' expression                      rule 1
NUMBER '+' expression '*' expression       rule 3
NUMBER '+' NUMBER '*' expression           rule 1
NUMBER '+' NUMBER '*' NUMBER               rule 1
```

with the numbers 2, 3 and 4. Every step replaces one non-terminal using one rule,
and the process ends when no non-terminals remain.

That is what "generates" means, and it is why grammars are called generative.
Recognizing — going the other way, from a string to a derivation — is what a
parser does, and it is the harder direction.

## Recursion is the whole trick

Look at rule 2:

```
expression := expression '+' expression
```

The name being defined appears in its own definition. That is what makes the
grammar describe an infinite set from four lines, and it is Chapter 13's
recursion arriving in a new place — a definition in terms of itself, with the
number rule as the base case that stops it.

If the recursion had no base case, the grammar would generate nothing at all:
every derivation would go on forever and never reach a string. A grammar needs a
non-recursive alternative for the same reason a recursive method needs one.

## Notation

Several notations exist and they differ only cosmetically.

**BNF** — Backus–Naur Form, from the ALGOL 60 report — writes non-terminals in
angle brackets and uses `::=`:

```
<expression> ::= <number> | <expression> "+" <expression>
```

**EBNF** adds three convenient operators, and they save a great deal of writing:

```
expression := term ('+' term)*        (* zero or more *)
option     := 'else' block ?          (* optional *)
list       := item (',' item)+        (* one or more *)
```

Each is shorthand for something BNF could express with an extra recursive rule.
`('+' term)*` in particular becomes a loop in the parser rather than a recursive
call, which is why the code in Section 24.2.3 has `while` in it.

**Railroad diagrams** draw the same information as tracks with branches, and are
easier to read at a glance and harder to write. Java's own specification uses a
BNF variant.

This book uses the EBNF style above: `:=`, plain names, quoted terminals,
`*` `+` `?` where they help.

## What grammars are for

Three things, and the second and third are why this is a general skill rather
than a compiler-writing skill.

**Specifying a language precisely.** The Java Language Specification's grammar is
the definition. When a compiler disagrees with it, the compiler is wrong. Before
this notation existed, language definitions were prose and every implementation
differed.

**Generating a parser.** Tools — yacc, ANTLR, bison — take a grammar and produce
parser code. Section 24.2.3 writes one by hand, which is worth doing once so that
you know what the tool is doing.

**Deciding what to accept.** Any time you read structured text — a configuration
file, a query string, a log format, a data interchange format — you are making
decisions a grammar would state exactly. Writing four lines of grammar before
writing the reader is often the difference between a parser that works and one
that mostly works.

## The limit

A grammar of this kind describes **syntax** and nothing else.

The grammar above happily generates `1 / 0`, and it would generate `x + 1` with
`x` undeclared if we added variables. Those are not grammatical errors; they are
errors of *meaning*, and no context-free grammar can catch them.

This is why real compilers have a phase after parsing. The parser establishes the
shape; a separate pass checks the things shape cannot express — that names are
declared, that types agree, that a method returns on every path. Chapter 25's
evaluator meets this directly when it looks a variable up and does not find it.

The line between the two is a real design boundary, and it is the same boundary
Section 24.1.1 drew between a language and its meaning.

Next: what happens when a grammar allows two readings of the same string.
