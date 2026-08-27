# Ambiguity

The grammar from the last lesson has a problem.

```
expression := NUMBER
            | expression '+' expression
            | expression '*' expression
            | '(' expression ')'
```

Derive `2 + 3 * 4`. Two derivations exist, and they disagree.

**First**, applying `+` at the top:

```
        +
       / \
      2   *
         / \
        3   4
```

**Second**, applying `*` at the top:

```
        *
       / \
      +   4
     / \
    2   3
```

Both are legal derivations from this grammar. The first tree means 14; the second
means 20.

A grammar that permits two distinct trees for one string is **ambiguous**, and an
ambiguous grammar does not determine the meaning of its own language. That is
fatal, because a language whose meaning depends on which derivation a parser
happened to find is not a specification at all.

## Fixing it with layers

The fix is not to add a rule saying "`*` binds tighter". Grammars have no such
mechanism. Instead you **restructure the grammar so that only the intended tree is
derivable**:

```
expression := term (('+' | '-') term)*
term       := factor (('*' | '/') factor)*
factor     := NUMBER | NAME | '(' expression ')'
```

Three levels, and the structure encodes the precedence.

An `expression` is a sequence of `term`s joined by `+` or `-`. A `term` is a
sequence of `factor`s joined by `*` or `/`. So `2 + 3 * 4` must parse as one
`term` (`2`), a `+`, and another `term` (`3 * 4`) — there is no derivation in
which `2 + 3` is a `term`, because `term` cannot contain a `+`.

The ambiguity is gone, and precedence came out of the layering rather than from
an annotation.

Verified, using the parser of Section 24.2.3:

```
2 + 3 * 4      -> (+ 2 (* 3 4))
(2 + 3) * 4    -> (* (+ 2 3) 4)
```

The first is the 14 tree. The second shows the parentheses doing their job — the
`factor` rule lets an entire `expression` back in, which is how a lower level
escapes upward.

The general recipe: **one level per precedence tier, lowest binding at the top.**
A language with comparison, addition and multiplication has four levels. Java's
grammar has about fifteen, for the same reason and in the same shape.

## Associativity

Precedence is only half. Consider `2 - 3 - 4`.

Both readings respect precedence, and they differ:

```
    ((2 - 3) - 4) = -5          left-associative
    (2 - (3 - 4)) = 3           right-associative
```

Arithmetic is left-associative, so the first is correct.

The EBNF rule `term (('+' | '-') term)*` says nothing about this by itself — it
gives a flat sequence. Associativity comes from how the parser builds the tree
from that sequence, and Section 24.2.3's loop builds leftward:

```java
Expr left = term();
while (match(PLUS)) left = new Bin('+', left, term());
```

Each iteration puts the accumulated tree on the *left* of the new node. Verified:

```
2 - 3 - 4      -> (- (- 2 3) 4)
1 + 2 + 3 + 4  -> (+ (+ (+ 1 2) 3) 4)
```

Left-leaning, as it should be. Assignment and exponentiation are right-associative
in most languages, and are built with recursion instead of a loop.

## The dangling else

The most famous ambiguity, and it is in almost every language you will use:

```
statement := 'if' '(' expr ')' statement
           | 'if' '(' expr ')' statement 'else' statement
           | ...
```

Now read:

```java
if (a) if (b) x(); else y();
```

Which `if` owns the `else`? Both readings derive.

Every mainstream language resolves it the same way — **the `else` binds to the
nearest unmatched `if`** — and most resolve it by rule rather than by fixing the
grammar, because the unambiguous version is genuinely unpleasant to read.

Java does this. So does C, C++, C#, and JavaScript. The rule is stated in the
specification as an exception, which is a small admission that the grammar alone
does not settle it.

This is also the strongest argument for the brace style Chapter 8 recommended:
write the braces and the question cannot arise. The ambiguity is in the grammar,
but it only bites code that omits them.

## Ambiguity outside programming

Natural language is ambiguous everywhere, and the same tree-drawing shows why.

*"I saw the man with the telescope."* Does `with the telescope` attach to `saw` or
to `the man`? Two trees, two meanings, and nothing in the sentence decides.

Human readers resolve this from context, plausibility, and tone. A compiler has
none of those, which is why programming languages are designed to be
unambiguous — every string that parses at all parses exactly one way — and why the
grammar layering above is not fussiness but a requirement.

It is also why programming languages feel rigid compared to English. That rigidity
is purchased deliberately, and what it buys is that a program means one thing.

Next: turning text into tokens.
