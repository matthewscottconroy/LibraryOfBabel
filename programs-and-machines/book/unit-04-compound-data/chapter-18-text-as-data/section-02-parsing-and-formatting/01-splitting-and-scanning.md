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

Three things about `split` that catch people.

**The argument is a regular expression**, not a literal string. For a comma this
makes no difference. For a dot it makes all the difference:

```java
"a.b.c".split(".")      // an empty array
```

In a regular expression `.` means *any character*, so everything matched and
everything was consumed. To split on a literal dot, escape it: `split("\\.")`.

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

It has one trap, and everyone meets it:

```java
int n = in.nextInt();
String name = in.nextLine();      // returns "" — not what you wanted
```

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

The lesson generalizes and it is worth taking seriously: **for any format with a
specification, use a library.** CSV, JSON, XML, dates, URLs. Every one of them has
escaping rules, edge cases, and a decade of accumulated bug fixes in the library,
and every hand-rolled parser rediscovers them one production incident at a time.

Hand-splitting is fine for data you control and whose shape you know. It is not
fine for anything that came from a user or another system.

Next: describing patterns rather than positions.
