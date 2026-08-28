# Formatting for Humans

Every time you turn data into text, somebody is going to read it. The trouble is
that there are two somebodies and they want opposite things.

**Machines** want exactness and stability: a format that round-trips, that sorts
correctly, that another program can parse. ISO dates, decimal points, no thousands
separators.

**People** want readability in their own conventions: local date order, local
decimal separator, currency symbols, aligned columns.

Serve the wrong one and you get a bug that is invisible on your machine and
obvious on somebody else's. So before you format anything, answer the question:
who is reading this?

## printf and format

Chapter 5 introduced `printf`. `String.format` is the same thing returning a string
rather than printing it:

```java
String s = String.format("%s scored %d", name, points);
System.out.printf("%s scored %d%n", name, points);
```

The specifiers, with the parts that matter:

```
%-10s     a string, left-aligned in 10 columns
%5d       an integer, right-aligned in 5
%8.2f     a float, 8 wide, 2 decimal places
%,d       an integer with thousands separators
%05.1f    zero-padded, 5 wide, 1 decimal place
%%        a literal percent sign
%n        a platform-appropriate newline
```

```java
System.out.printf("%-10s|%5d|%8.2f|%n", "left", 42, 3.14159);
// left      |   42|    3.14|

String.format("%,d", 1234567);      // "1,234,567"
String.format("%05.1f", 3.14159);   // "003.1"
```

What you get over gluing strings together with `+` is that **the layout lives in
one place, where you can see it.** Widen a column and you edit one number in the
format string. Do the same with concatenation and you are restructuring an
expression.

And alignment is possible at all, which it flatly is not with `+`.

## A warning about rounding

`%.2f` rounds the *display*. It does not round the number. The `double` underneath
is untouched, and — as Chapter 3 spent some time establishing — its value was never
exactly what you typed in the first place.

```java
String.format("%.2f", 0.125)      // "0.13" or "0.12" depending on the stored value
```

So do not reach for display formatting to paper over an arithmetic problem. If a
number has to be exact to the penny, it needed to be an integer count of pennies
or a `BigDecimal` from the beginning.

Chapter 3 said so. This is the lesson where people discover they were not
listening.

## Locale

`format` uses the default locale unless told otherwise, and the default comes from
the machine:

```java
String.format("%,.2f", 1234.5)                  // "1,234.50" in the US
                                                // "1.234,50" in Germany
String.format(Locale.ROOT, "%,.2f", 1234.5)     // "1,234.50" always
```

Both of those are correct. Each is a disaster in the other one's context.

Write a data file using the platform default and you have made the file's format
depend on which machine happened to produce it — so a colleague in Berlin runs your
exporter, out comes `1.234,50`, and your comma-splitting parser reads one number as
two fields and never says a word about it.

**Use `Locale.ROOT` for machine-readable output. Use the user's locale for
display.** The same distinction as `toUpperCase` in Section 18.1.3, and the same
class of bug.

## Dates, where this goes wrong most often

```java
LocalDate d = LocalDate.of(2024, 1, 15);
d.toString();                                        // "2024-01-15"
d.format(DateTimeFormatter.ISO_DATE);                // "2024-01-15"
d.format(DateTimeFormatter.ofLocalizedDate(FormatStyle.MEDIUM));  // locale-dependent
```

`java.time` arrived in Java 8 to replace an earlier date API that was, by common
consent including its authors', bad. Use the new one. If you find yourself holding
a `java.util.Date`, you are in old code.

ISO 8601 — `2024-01-15` — is the format for storage and interchange. It is
unambiguous, it sorts correctly as text, and it does not depend on whether the
reader is American. `01/02/2024` is the second of January or the first of February
depending on the country, and choosing it for a data file guarantees that someone
will eventually read it wrongly.

## Text blocks

For multi-line text, Java 15 added a form that avoids escape clutter:

```java
String report = """
    line one
    line two""";
```

Indentation common to all lines is stripped, so the string is what it appears to
be. Useful for SQL, JSON, HTML, and anything with embedded quotes.

## Closing the chapter and the unit

A `String` is an immutable sequence of characters, and the immutability is a
decision with four payoffs: sharing is safe, so literals can be pooled; passing is
safe, so a method cannot alter your string; hashing is safe, so strings are ideal
map keys; and threads need no coordination. The cost is that modification means
allocation, which is nothing occasionally and quadratic in a loop — hence
`StringBuilder`, which is Chapter 17's doubling array specialized for characters,
and which measured eighty times faster over 40,000 appends.

Comparison is where the unit's traps concentrate. `==` compares references and
works for literals, so it passes tests and fails on real input. `compareTo` uses
code point order, which puts all uppercase before all lowercase and is not
alphabetical in any human language. Text from outside needs normalizing before it
can be compared, and case conversion needs a locale — or Turkish will find you.

And the boundary between text and data is where assumptions surface. `split` takes
a regular expression, discards trailing empties, and cannot handle quoted fields.
Regular expressions describe shapes elegantly and cannot describe nesting at all,
for the reason Chapter 6 gave. Formatting should say what the output looks like,
and should know whether its audience is a machine or a person.

Unit IV set out to turn a pile of values into a structure. Arrays gave us many
values with constant-time access by position. The abstract data type gave us a way
to say what a collection *means* and — with the representation invariant — an
obligation each operation must preserve. The collections library gave us growth,
and three shapes answering three questions. And strings turned out to exercise all
of it.

What we still lack is the mechanism. Chapter 16 said `private` is what puts a
boundary around an invariant, and then deferred it. Unit V is that deferral coming
due.
