# Patterns and Regular Expressions

Splitting works when the structure is positional — field 2 is the surname. Often
the structure is a *shape* instead: a date, an email address, a number embedded in
a sentence.

**Regular expressions** are a small language for describing shapes of text, and
they are worth meeting even though this section cannot teach them properly.

## The idea

```java
"2024-01-15".matches("\\d{4}-\\d{2}-\\d{2}")      // true
```

The pattern says: four digits, a hyphen, two digits, a hyphen, two digits.

That is a description of a shape rather than a procedure for checking one. You
state what the text should look like; the library works out how to test it. It is a
small declarative language embedded in a procedural one, which is a genuinely
different way of expressing a computation and a preview of Chapter 24, where we
build a parser for a language of our own.

## Enough syntax to read one

| pattern | matches |
|---|---|
| `abc` | exactly that |
| `.` | any single character |
| `\d` | a digit; `\w` a word character; `\s` whitespace |
| `[abc]` | any one of a, b, c |
| `[a-z]` | any lowercase letter |
| `[^abc]` | any character *except* those |
| `*` | zero or more of the preceding |
| `+` | one or more |
| `?` | zero or one |
| `{4}` | exactly four |
| `{2,5}` | between two and five |
| `^` / `$` | start / end of the text |
| `(...)` | a group, which can be captured |
| `|` | either side |

In Java these live in string literals, so every backslash is doubled: the pattern
`\d` is written `"\\d"`. That doubling is a constant source of confusion and it is
Chapter 5's escape rule, not a regex rule.

## The three uses

**Testing:**

```java
if (input.matches("\\d+")) { ... }      // is it entirely digits?
```

`matches` requires the *whole* string to match, which differs from most other
languages' equivalents and catches people.

**Extracting:**

```java
Pattern p = Pattern.compile("(\\d{4})-(\\d{2})-(\\d{2})");
Matcher m = p.matcher("due 2024-01-15 ok");
if (m.find()) {
    String year  = m.group(1);      // "2024"
    String month = m.group(2);      // "01"
    String day   = m.group(3);      // "15"
}
```

Parenthesized groups are captured and numbered from 1. `find` searches anywhere in
the text; `matches` requires the whole thing.

**Replacing:**

```java
"a1b22c".replaceAll("\\d+", "#")      // "a#b#c"
```

And splitting, which we have already used — `split("\\s+")` divides on runs of
whitespace, which is what you usually want rather than splitting on single spaces.

## Compile once

```java
private static final Pattern DATE = Pattern.compile("(\\d{4})-(\\d{2})-(\\d{2})");
```

`Pattern.compile` does real work, analyzing the pattern into a matching machine.
Doing it inside a loop repeats that work every iteration. Compile once and reuse —
patterns are immutable and safe to share.

`String.matches` and `String.replaceAll` compile the pattern every call, which is
fine occasionally and wasteful in a loop.

## When not to use them

Regular expressions are seductive and frequently the wrong tool.

**They cannot parse nested structures.** This is not a limitation of a particular
implementation; it is a theorem. Regular expressions describe exactly the languages
a *finite state machine* can recognize — Chapter 6's machine, with a fixed number
of states — and matching nested brackets requires unbounded counting, which a
finite machine cannot do.

Chapter 6 showed this: no finite machine accepts *n* zeros followed by *n* ones.
The same argument means **you cannot parse HTML, JSON, or arithmetic expressions
with a regular expression**, and attempts to do so are a recurring internet joke
with a real theorem behind it. Chapter 24 builds the machinery that can.

**They become unreadable fast.** A pattern for a valid email address, written
properly, is over four hundred characters and nobody can verify it. If your pattern
needs a comment explaining it, consider whether ordinary code would be clearer.

**They can be catastrophically slow.** Certain patterns on certain inputs take
exponential time — a vulnerability with a name, ReDoS. Nested quantifiers like
`(a+)+` are the classic shape.

The rule I would offer: **use them for simple, well-understood shapes — a date, a
number, a fixed format. Use a parser for anything nested. Use a library for
anything standard.**

Next: the other direction.
