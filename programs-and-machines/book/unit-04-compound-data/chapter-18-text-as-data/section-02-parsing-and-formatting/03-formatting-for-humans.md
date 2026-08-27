# Formatting for Humans

Data becomes text for two audiences, and they want different things.

**Machines** want exactness and stability: a format that round-trips, that sorts
correctly, that another program can parse. ISO dates, decimal points, no thousands
separators.

**People** want readability in their own conventions: local date order, local
decimal separator, currency symbols, aligned columns.

Confusing the two produces a recurring class of bug, and the first rule is to know
which you are serving.

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

The value of this over concatenation is that **the layout is visible in one
place**. Changing a column width means editing the format string; with
concatenation it means restructuring the expression. And alignment is achievable at
all, which it is not with `+`.

## Rounding, and a warning

`%.2f` rounds for display, and Chapter 3 applies: the underlying `double` is
unchanged, and its value was never exactly what you wrote.

```java
String.format("%.2f", 0.125)      // "0.13" or "0.12" depending on the stored value
```

Do not use display formatting to fix an arithmetic problem. If the number must be
exact to the cent, it should have been a `BigDecimal` or an integer count of cents
— Section 3.2.3's argument, and this is where people discover they ignored it.

## Locale

`format` uses the default locale unless told otherwise, and the default comes from
the machine:

```java
String.format("%,.2f", 1234.5)                  // "1,234.50" in the US
                                                // "1.234,50" in Germany
String.format(Locale.ROOT, "%,.2f", 1234.5)     // "1,234.50" always
```

Both are right for their audience and disastrous when confused. Writing a data file
with the platform default means the file's format depends on where it was written,
and a German-configured machine produces `1.234,50` which a comma-splitting parser
reads as two fields.

**Use `Locale.ROOT` for machine-readable output. Use the user's locale for
display.** The same distinction as `toUpperCase` in Section 18.1.3, and the same
class of bug.

## Dates

Worth a note, because dates are where formatting goes wrong most often.

```java
LocalDate d = LocalDate.of(2024, 1, 15);
d.toString();                                        // "2024-01-15"
d.format(DateTimeFormatter.ISO_DATE);                // "2024-01-15"
d.format(DateTimeFormatter.ofLocalizedDate(FormatStyle.MEDIUM));  // locale-dependent
```

`java.time`, added in Java 8, replaced an earlier API that was genuinely bad. Use
it.

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
