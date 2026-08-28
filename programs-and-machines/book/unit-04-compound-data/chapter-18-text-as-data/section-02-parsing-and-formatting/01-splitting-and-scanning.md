# Splitting and Scanning

The commonest text task: a line arrives, and it contains fields.

```
Ada,Lovelace,1815
```

## split

```java
String[] parts = line.split(",");
// ["Ada", "Lovelace", "1815"]
```

Three things about `split` catch people, and the first one is worth meeting as a
puzzle rather than a warning. What does this give you?

```java
"a.b.c".split(".")      // ?
```

An empty array. Not three pieces, not one piece — nothing at all.

**The argument is a regular expression**, and never a literal string. For a comma
that distinction costs you nothing, which is exactly why it goes unnoticed until
the day you split on a dot.

In a regular expression, `.` means *any character*. So every character was a
separator, everything matched, everything was consumed, and what came back was the
nothing that remained. To split on an actual dot, escape it: `split("\\.")`.

**Trailing empty strings are discarded:**

```java
"a,b,,".split(",")          // length 2: ["a", "b"]
"a,b,,".split(",", -1)      // length 4: ["a", "b", "", ""]
```

The second argument is a limit; −1 keeps everything. If your fields are positional
— field 4 is the postcode — the default will silently give you a shorter array and
an index error later.

**Empty fields in the middle are kept:**

```java
"a,b,,c".split(",")      // ["a", "b", "", "c"]
```

Which is what you want, and it means an empty field arrives as an empty string
rather than as `null`.

## Then convert

Split gives strings. Numbers need parsing:

```java
int year = Integer.parseInt(parts[2]);
double d  = Double.parseDouble(s);
boolean b = Boolean.parseBoolean(s);
```

`parseInt` throws `NumberFormatException` for anything that is not an integer,
including an empty string, a string with spaces, and `"12.5"`. That is correct
behavior and it means **input parsing must expect failure**:

```java
try {
    int year = Integer.parseInt(parts[2].trim());
    ...
} catch (NumberFormatException e) {
    System.err.println("Bad year on line " + lineNumber + ": " + parts[2]);
}
```

Chapter 28 covers exceptions properly. The point now is that a boundary between
text and data is a place where things go wrong, and code that does not handle it
will fail on the first malformed line — which real data always contains.

Note the `.trim()`. Fields from real files have spaces around them more often than
not.

## Scanner

For reading rather than splitting:

```java
Scanner in = new Scanner(System.in);
int n = in.nextInt();
String word = in.next();
String line = in.nextLine();
```

`Scanner` reads tokens of a requested type from a source — the console, a file, or
a string.

It has one trap, and every single person meets it. Predict what lands in `name`
when the user types `42` and presses return and then types their name:

```java
int n = in.nextInt();
String name = in.nextLine();      // ?
```

An empty string.

`nextInt` consumes the number and **leaves the newline**. `nextLine` then reads to
that newline and returns an empty string.

The fix is an extra `nextLine()` to consume the rest of the line:

```java
int n = in.nextInt();
in.nextLine();                    // discard the rest of the line
String name = in.nextLine();
```

Or, more reliably, read everything with `nextLine` and parse the pieces yourself.
Mixing token-reading and line-reading is where the confusion lives.

## Why CSV is harder than it looks

Splitting on commas works until a field contains a comma:

```
Ada,"Lovelace, Countess",1815
```

Splitting on `,` gives four fields, and the second and third are halves of one.
The convention is that a quoted field may contain commas, and that a quote inside a
quoted field is doubled — and implementing that correctly is a small state machine,
not a `split`.

Take the general lesson seriously, because it will save you more time than
anything else in this chapter: **for any format that has a specification, use a
library.**

CSV, JSON, XML, dates, URLs. Every one of them has escaping rules and edge cases,
and every library for them carries a decade of accumulated bug fixes contributed by
people who found those edge cases the hard way. A hand-rolled parser rediscovers
them too — one production incident at a time.

Hand-splitting is fine for data you control and whose shape you know. It is not
fine for anything that came from a user or another system.

Next: describing patterns rather than positions.
