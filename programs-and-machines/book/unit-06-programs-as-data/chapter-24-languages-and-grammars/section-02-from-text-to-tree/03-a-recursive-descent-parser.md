# A Recursive-Descent Parser

Here is the grammar again, unambiguous, from Section 24.1.3:

```
expression := term (('+' | '-') term)*
term       := factor (('*' | '/') factor)*
factor     := NUMBER | NAME | '(' expression ')'
```

And here is the parser. Read them side by side, because the point of this lesson
is that they are the same thing.

## The machinery

```java
static final class Parser {
    private final List<Token> tokens;
    private int pos = 0;

    Token peek()    { return tokens.get(pos); }
    Token advance() { return tokens.get(pos++); }

    boolean match(Kind k) {
        if (peek().kind() == k) { pos++; return true; }
        return false;
    }

    void expect(Kind k) {
        if (!match(k)) throw new IllegalArgumentException(
            "expected " + k + " but found " + peek());
    }
}
```

One field of interest: `pos`, the position in the token list. Everything else is
four small operations on it.

`peek` looks without consuming — this is the **lookahead**, and one token of it is
all this grammar needs. A grammar decidable with one token of lookahead is called
LL(1), and most hand-written parsers target it because more lookahead means more
bookkeeping.

`match` consumes conditionally: if the next token is what you hoped, take it and
say so. `expect` consumes or fails, and it is where a missing bracket is caught.

## One method per rule

```java
// expression := term (('+' | '-') term)*
Expr expression() {
    Expr left = term();
    while (true) {
        if (match(PLUS))       left = new Bin('+', left, term());
        else if (match(MINUS)) left = new Bin('-', left, term());
        else return left;
    }
}
```

Compare with the rule. `term` first, then zero or more operator-and-term pairs.
The `*` in the grammar is the `while`. The two alternatives inside are the two
branches.

The tree-building is the one thing the grammar does not state: `left = new Bin(op,
left, term())` puts the accumulated tree on the left, which produces the
left-associativity of Section 24.1.3. Building it as `new Bin(op, term(), left)`
would be right-associative and wrong for subtraction. That single line is where
`2 - 3 - 4` becomes $-5$ rather than 3.

```java
// term := factor (('*' | '/') factor)*
Expr term() {
    Expr left = factor();
    while (true) {
        if (match(STAR))       left = new Bin('*', left, factor());
        else if (match(SLASH)) left = new Bin('/', left, factor());
        else return left;
    }
}
```

The same method with different operators, one level down. This is where precedence
lives: `expression` calls `term` for its operands, so a `term` is always assembled
completely before an addition can use it. `3 * 4` becomes a node before `2 +` ever
sees it.

```java
// factor := NUMBER | NAME | '(' expression ')'
Expr factor() {
    Token t = advance();
    return switch (t.kind()) {
        case NUMBER -> new Num(Integer.parseInt(t.text()));
        case NAME   -> new Var(t.text());
        case LPAREN -> { Expr e = expression(); expect(RPAREN); yield e; }
        default -> throw new IllegalArgumentException("unexpected " + t);
    };
}
```

Three alternatives, three cases, and the third is the interesting one.

`factor` calls `expression` — the top level — from the bottom level. That is the
recursion, and it is what parentheses mean: inside brackets, start over with no
precedence context. `(2 + 3)` is a legal `factor` because `2 + 3` is a legal
`expression`.

The `expect(RPAREN)` immediately after is what makes brackets balance. It is the
only thing in the parser that does, and it works because the recursive call
returns exactly when the inner expression is finished.

## Why this is called recursive descent

**Descent**: the methods go down through the grammar's levels, `expression` to
`term` to `factor`, most loosely binding to most tightly.

**Recursive**: `factor` calls back up to `expression`, so the descent can start
over at any depth.

And where is the stack that Section 24.1.1 said context-free languages require?
It is Chapter 12's call stack. Each nested parenthesis is a live `expression`
frame, waiting for its `expect(RPAREN)`. The parser never declares a stack because
it inherited one.

That is worth pausing on. The theory said this problem needs unbounded memory
arranged as a stack; the implementation gets it for free by being recursive. Deep
enough nesting will exhaust it and produce Chapter 12's `StackOverflowError` — try
five thousand open brackets — which is the theory's requirement showing up as a
practical limit.

## Running it

```java
static Expr parse(String src) {
    return new Parser(tokenize(src)).parse();
}
```

where `parse()` is `expression()` followed by `expect(END)`. That last check
matters: without it, `2 + 3 )` would parse `2 + 3`, stop happily, and ignore the
rest. Demanding the sentinel is how a parser insists it consumed everything.

Verified:

```
2 + 3 * 4      -> (+ 2 (* 3 4))                depth 3
(2 + 3) * 4    -> (* (+ 2 3) 4)                depth 3
2 - 3 - 4      -> (- (- 2 3) 4)                depth 3
1 + 2 + 3 + 4  -> (+ (+ (+ 1 2) 3) 4)          depth 4
width * 2 + 1  -> (+ (* width 2) 1)            depth 3
```

Sixty lines of parser, and it handles precedence, associativity, parentheses to
any depth, and variables.

## The errors

```
2 +     ==>  unexpected END
2 $ 3   ==>  unexpected character '$' at position 2
(2 + 3  ==>  expected RPAREN but found END
```

Three failures, three different components. The middle one is the tokenizer's, and
it never reached the parser. The first is `factor` reaching for an operand and
finding the sentinel. The third is `expect` not getting its bracket.

These messages are adequate and not good. A real parser reports a line and column,
shows the source line with a caret, and recovers — skipping to the next statement
so that one missing semicolon does not produce forty errors. Error recovery is a
substantial subject and it is most of what separates a teaching parser from a
usable one.

But notice that the diagnosis is already correct in each case. Getting the message
right is presentation; knowing what went wrong is structure, and the structure is
here.

## What this technique costs

Recursive descent is the parsing method most hand-written parsers use — `javac`,
Clang, TypeScript, and Python's CPython parser all descend recursively — and the
reasons are the ones you have just seen: it is readable, it debugs with a stack
trace, and error messages can be written wherever they belong.

Two limitations are worth naming.

**Left recursion loops forever.** A rule like `expression := expression '+' term`
would compile to a method whose first act is to call itself with the position
unchanged. Instant `StackOverflowError`. That is why the grammar was written with
`*` instead, and rewriting left recursion into iteration is a standard
transformation you now know by example.

**The grammar must be decidable with limited lookahead.** These rules choose a
branch by examining one token. Grammars needing more — or needing to try a branch
and back out — require a more capable technique, and the standard answers are
parser generators producing table-driven LR parsers, or backtracking approaches
like PEG.

For everything you are likely to write by hand, recursive descent is the right
tool, and the sixty lines above are the whole of it.

Next chapter: the tree stops being something we print and becomes something we
run.
