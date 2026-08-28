# Delimited Data

Everyone knows how to parse CSV. Split each line on commas.

Everyone is wrong, and the counterexample takes one line to produce: a book
called `Programming, with a comma`. The moment your data can contain your
delimiter — and it always can — the obvious approach stops working, and the
standard fix is more interesting than it looks.

The simplest structure you can put on a text file: one record per line, fields
separated by a character.

```
title,author,year
The Mythical Man-Month,Brooks,1975
```

That is CSV, it is thirty years old, every spreadsheet reads it, and it is the
format you should reach for first for tabular data.

It also has a problem, and the problem is instructive because it is the same
problem every delimited format has.

## The comma problem

```
Programming, with a comma,Nobody,2026
```

Four fields, or three with a comma in the first? Splitting on a comma gives four,
and the record is wrong.

Any delimiter can appear in the data. Choosing a rarer one — a tab, a pipe, an
ASCII unit separator — makes it less likely and not impossible, and "less likely"
means the bug arrives later, in production, on someone's real data.

## The two fixes

**Escaping.** Precede the delimiter with a marker: `Programming\, with a comma`.
Then the marker needs escaping too, and you are counting backslashes.

**Quoting.** Wrap the field, and double the quote inside it:

```java
static String escape(String s) {
    return s.contains(",") || s.contains("\"")
        ? "\"" + s.replace("\"", "\"\"") + "\"" : s;
}
```

Verified:

```
written as : "Programming, with a comma",Nobody,2026
read back  : Book[title=Programming, with a comma, author=Nobody, year=2026]
round trip : true
```

This is what CSV actually specifies, and it means **you cannot parse CSV with
`split(",")`.** The parser needs a small state machine:

```java
boolean quoted = false;
for (int i = 0; i < line.length(); i++) {
    char c = line.charAt(i);
    if (quoted) {
        if (c == '"') {
            if (i + 1 < line.length() && line.charAt(i + 1) == '"') { cur.append('"'); i++; }
            else quoted = false;
        } else cur.append(c);
    } else if (c == '"')  quoted = true;
    else if (c == ',')    { out.add(cur.toString()); cur.setLength(0); }
    else cur.append(c);
}
```

Twelve lines, one boolean, and it is Chapter 24's tokenizer at its smallest — a
position moving forward, a mode, and a decision per character. It is a regular
language, so no stack is needed, which is Section 24.1.1's classification doing
practical work.

This version still does not handle everything real CSV does. A quoted field may
contain a **newline**, which means CSV is not a line-oriented format at all and a
correct reader cannot use `readLine`. RFC 4180 says so; most hand-written parsers
do not know.

**Use a library for CSV you did not write.** Apache Commons CSV, OpenCSV,
univocity. Write your own only for data you also produce, where you control what
goes in.

## Reading a record

```java
static Book fromLine(String line) {
    List<String> f = splitCsv(line);
    if (f.size() != 3) throw new IllegalArgumentException("expected 3 fields, got " + f.size());
    return new Book(f.get(0), f.get(1), Integer.parseInt(f.get(2).trim()));
}
```

Verified:

```
bad line: expected 3 fields, got 2
```

The field-count check is the whole of Chapter 28's fail-fast in this context.
Without it, `f.get(2)` throws `IndexOutOfBoundsException`, which names an index
rather than a malformed record.

What a production version adds: the line number and the file name, via Section
28.2.1's catch-and-add-context. `expected 3 fields, got 2` is a start; `line
4,127 of customers.csv: expected 3 fields, got 2` is an answer.

## The header

```
title,author,year
```

One line at the top naming the columns. Worth having, and for a reason beyond
readability: a reader that consults the header can tolerate reordered or added
columns, where a reader that assumes position cannot.

The cost is that the header is one more thing that can disagree with the data, and
that `skip(1)` is easy to forget — a forgotten header becomes a record whose year
is the text `year`, and `Integer.parseInt` says so.

## The alternatives

CSV is not the only choice and it is often not the right one.

**TSV** — tabs instead of commas. Tabs are rarer in text, and the format is
otherwise identical. Marginally better and much less widely supported by
spreadsheet software.

**JSON** — nested structures, typed values, and a real specification. The right
answer for anything with structure beyond a table: configuration, API payloads,
documents. Not line-oriented, so a large JSON file must be read whole or streamed
with a specialized parser.

**JSON Lines** — one JSON object per line. Combines JSON's structure with CSV's
streamability, and it is what most log pipelines use. A very good default for
records you will append to.

**Properties files** — key and value per line, built into Java. Fine for
configuration, and its escaping rules are quietly awful.

**Binary** — compact and fast, unreadable when broken, and requiring a program to
inspect. Section 29.1.3's rule applies.

The decision, roughly: **tabular data, CSV. Structured data, JSON. Many structured
records, JSON Lines. Configuration, whatever your ecosystem already uses.**

And one thing that holds for all of them: **do not invent a format.** Every custom
format needs a parser, a writer, a specification, and error handling, and every
one of those is a place to be subtly wrong. The formats above have all four
already.

Next: what a format has to survive.
