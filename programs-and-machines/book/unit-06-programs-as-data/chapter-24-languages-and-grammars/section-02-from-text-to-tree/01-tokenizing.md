# Tokenizing

`2 + 3 * width` is thirteen characters, three of which are spaces that mean
nothing.

A parser that worked directly on those characters would have to skip whitespace at
every position, reassemble multi-digit numbers in the middle of its expression
rules, and mix two entirely different concerns in every method. It would work and
it would be miserable to read.

So nobody does it. Every language implementation ever built runs a smaller pass
first, and there is a reason for that beyond convenience.

What the parser wants to see is *number, plus, number, star, name*. Five things,
already sorted into kinds, with the spaces gone.

**Tokenizing** is the pass that gives it that. It is also called lexing or
scanning, and the
component is a lexer or scanner. All the same thing.

## Tokens

A token is a classified piece of text:

```java
enum Kind { NUMBER, NAME, PLUS, MINUS, STAR, SLASH, LPAREN, RPAREN, END }

record Token(Kind kind, String text) { }
```

An enum and a record, and both choices are the ones Chapter 22 argued for. There
are exactly nine kinds of token and there will never be a tenth without somebody
editing this line — a closed set, so every `switch` over it can be checked for
exhaustiveness. And a token *is* its kind and its text and nothing else, which is
the definition of a record.

`text` matters only for `NUMBER` and `NAME` — the others are determined by their
kind — but carrying it uniformly is simpler than not, and it is what an error
message wants to print. A real compiler also stores the line and column here, for
the same reason.

`END` is a sentinel appended after the last real token. It saves the parser from
checking for the end of the list at every step; instead it can always ask for the
next token and get something back.

Steal that trick. It shows up everywhere once you have a name for it: **a sentinel
turns a special case into an ordinary one**, and special cases are where bugs
live.

## The loop

```java
static List<Token> tokenize(String src) {
    List<Token> out = new ArrayList<>();
    int i = 0;
    while (i < src.length()) {
        char c = src.charAt(i);
        if (Character.isWhitespace(c)) { i++; continue; }
        ...
    }
    out.add(new Token(Kind.END, ""));
    return out;
}
```

One position, moving forward, never backward. At each step: look at the current
character, decide what kind of token starts here, consume as much as belongs to
it, and emit.

Whitespace is consumed and discarded, which is the line that makes the language
free-format — indentation carries no meaning, and `2+3` and `2 + 3` tokenize
identically. Python's tokenizer does not do this, which is why indentation is
significant there; the difference between the two language families is a few lines
in this loop.

## Numbers and names

```java
if (Character.isDigit(c)) {
    int j = i;
    while (j < src.length() && Character.isDigit(src.charAt(j))) j++;
    out.add(new Token(Kind.NUMBER, src.substring(i, j)));
    i = j; continue;
}
```

Scan forward while the characters keep qualifying, then emit the whole span at
once.

That greedy inner loop has a name — **maximal munch** — and it is the rule that
takes the longest thing that could be a token. Without it you would get three
numbers out of `123` and five names out of `width`, which is not a subtle bug so
much as a complete failure to have started.

Names are the same with a wrinkle:

```java
if (Character.isLetter(c)) {
    int j = i;
    while (j < src.length() && Character.isLetterOrDigit(src.charAt(j))) j++;
    ...
}
```

The first character must be a letter; the rest may be letters *or* digits. That
asymmetry is why `x1` is a name and `1x` is not — and `1x` does not produce an
error here, it produces `NUMBER(1)` followed by `NAME(x)`, which the parser will
reject. Tokenizers are usually permissive; catching that is the next stage's job.

This is where Chapter 4's Unicode work quietly pays off. `Character.isLetter`
returns true for Greek, Cyrillic, and Han characters, so `λ` and `ναι` are legal
Java identifiers. Whether that is a good idea is debatable; that it follows from
using the Unicode-aware predicate rather than `c >= 'a' && c <= 'z'` is not.

## Single characters

```java
Kind k = switch (c) {
    case '+' -> Kind.PLUS;   case '-' -> Kind.MINUS;
    case '*' -> Kind.STAR;   case '/' -> Kind.SLASH;
    case '(' -> Kind.LPAREN; case ')' -> Kind.RPAREN;
    default -> throw new IllegalArgumentException(
        "unexpected character '" + c + "' at position " + i);
};
```

The `default` is the only place a tokenizer can fail, and it produces the first
of the three error messages this chapter can generate:

```
2 $ 3  ==>  unexpected character '$' at position 2
```

Look at the end of that message, because it is doing more work than the rest of
it. An error that does not tell you *where* is close to useless — and the
tokenizer is the last component that still knows, since once the parser has tokens
the character offsets are gone unless somebody thought to keep them.

Real compilers keep them. That is how `javac` manages to draw a caret under the
exact column of your mistake.

## The result

```java
tokenize("2 + 3 * width")
```

Verified:

```
[NUMBER(2), PLUS, NUMBER(3), STAR, NAME(width), END]
```

Thirteen characters became five tokens and a sentinel. The whitespace is gone, the
digits are grouped, and the parser now has a clean sequence.

## Why this is a separate pass at all

You could, in principle, do all of this inside the parser. Two reasons nobody
does, and the first is not a matter of taste.

**Tokens are regular; structure is not.** Every token here can be recognized by a
machine with finite memory — a run of digits, a run of letters, a single symbol.
Nesting cannot. Splitting the problem puts the easy half in a simple loop and
leaves the parser to do the part that genuinely needs a stack.

**It simplifies the parser enormously.** A parser working on characters would
handle whitespace at every position, reassemble multi-digit numbers inside the
expression rules, and mix two levels of concern in every method. With a token
list, the parser's rules read like the grammar.

The industrial version of this loop is generated from regular expressions by a
tool — `lex`, `flex`, ANTLR — and for a language with string literals, escape
sequences, comments, and floating-point formats you should use one. For a small
language, sixty lines of hand-written loop is clearer than the tool's output and
easier to give good error messages from.

Next: the shape the parser is going to build.
