# Patterns and Regular Expressions

Splitting works when the structure is positional — field 2 is the surname. Often
the structure is a *shape* instead: a date, an email address, a number embedded in
a sentence.

**Regular expressions** are a small language for describing shapes of text. I
should say at the outset that this section cannot teach them properly — people
write whole books — and that what it can do is show you what they are, what they
are for, and the one thing about them that is a theorem rather than an opinion.

## The idea

```java
"2024-01-15".matches("\\d{4}-\\d{2}-\\d{2}")      // true
```

The pattern says: four digits, a hyphen, two digits, a hyphen, two digits.

Stop and notice what kind of thing you just wrote, because it is not like the rest
of your program. There is no loop in it. There is no index, no comparison, no
control flow of any kind. You did not write a *procedure for checking* a date — you
wrote a *description of what a date looks like*, and handed the checking to
somebody else.

That is a small declarative language living inside a procedural one, and it is a
genuinely different way to express a computation. It is also a preview of Chapter
24, where you build a parser for a language of your own and find yourself on the
other side of this arrangement.

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

**They cannot parse nested structures.** And this is the one worth having
properly, because it is not a complaint about somebody's implementation and it is
not a matter of the pattern being awkward to write. It is a theorem, and nothing
will ever fix it.

Regular expressions describe exactly the languages a *finite state machine* can
recognize — Chapter 6's machine, with its fixed number of states. Matching nested
brackets requires counting how deep you are, with no bound on the depth. A machine
with finitely many states cannot count without a bound, because it would need a
state for every depth.

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
