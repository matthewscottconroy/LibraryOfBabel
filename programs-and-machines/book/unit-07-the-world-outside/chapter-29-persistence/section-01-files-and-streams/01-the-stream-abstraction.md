# The Stream Abstraction

A file might be four bytes. It might be four hundred gigabytes. And a network
connection has no length at all — data turns up until it stops turning up, and
nobody can tell you in advance how much there will be.

So think about what abstraction could possibly cover all of those. It cannot be
"a file is an array of bytes", because an array has a length and you can hold it
all at once, and neither of those is available here.

What survives is considerably weaker, and its weakness is the point:

> A **stream** is a sequence of items, available one at a time, in order, possibly
> without a known end.

Two directions: an **input stream** you read from, an **output stream** you write
to. Both are one-way and both are sequential. That is deliberately less than a
file offers — a file can be seeked — because the smaller abstraction also
describes a socket, a pipe, a compressor, and standard input, and code written
against it works on all of them.

You have seen the word before. Chapter 26's `Stream<T>` is the same idea applied
to collections rather than to I/O: a sequence pulled one element at a time, lazily,
possibly infinite. The names are not a coincidence; the shared property is that
you consume it once and cannot go back.

## The four families

Java has four base types, and the two-by-two structure is the thing to remember:

|  | bytes | characters |
|---|---|---|
| **read** | `InputStream` | `Reader` |
| **write** | `OutputStream` | `Writer` |

`InputStream.read()` returns an `int` in the range 0–255, or $-1$ at the end.
`Reader.read()` returns a `char` as an `int`, or $-1$.

The split matters and it is Chapter 4's. Bytes are what a file contains;
characters are what a byte sequence means under an encoding. Confusing them is the
single most common I/O bug and Section 29.1.2 shows it happening.

The rule: **bytes for anything that is not text; characters for anything that is,
with the encoding stated.**

## Decorators

The families are small because behavior is added by wrapping:

```java
InputStream raw     = Files.newInputStream(path);
InputStream fast    = new BufferedInputStream(raw);
InputStream gunzip  = new GZIPInputStream(fast);
Reader      chars   = new InputStreamReader(gunzip, StandardCharsets.UTF_8);
BufferedReader text = new BufferedReader(chars);
```

Each wrapper is itself a stream, takes a stream, and adds one thing: buffering,
decompression, decoding, line handling. They compose in any sensible order, and
the same `BufferedReader` code works whether the source is a file, a socket, or a
string.

This is the **decorator** pattern, and Java's I/O library is the textbook example
of it — in both directions. It is also the textbook example of what the pattern
costs.

Be honest about that five-line incantation. It is off-putting. The correct order is
not something you could derive, and if your reaction on first meeting it was to
wonder why reading a file needs five objects, your reaction was entirely
reasonable.

The answer is that the alternative is a method for every combination —
`readGzippedUtf8FileWithBuffering` — and there are more combinations than methods
anyone would write. The library chose composability over convenience, and then
spent twenty years adding convenience on top.

And look at what the library is quietly demonstrating here. `BufferedInputStream`
**holds** an `InputStream`; it does not extend one. Had it extended
`FileInputStream` instead, it could never have buffered a socket — which is the
whole composition-over-inheritance argument, sitting in the JDK, decided correctly
in 1996.

## The convenience layer

Most of the time you should use the short forms, added in Java 7 and 11:

```java
String   text  = Files.readString(path);              // whole file, UTF-8
List<String> ls = Files.readAllLines(path);
byte[]   bytes = Files.readAllBytes(path);
Files.writeString(path, "hello\n");

try (Stream<String> lines = Files.lines(path)) { ... }         // lazy
try (BufferedReader r = Files.newBufferedReader(path)) { ... }
```

`Files.readString` reads the whole thing into memory, which is right for a
configuration file and wrong for a ten-gigabyte log. `Files.lines` is lazy — it
reads as you consume — which is right for the log and needs closing, hence the
try-with-resources.

And that is essentially the entire decision, every time, for the rest of your
career: **does it fit in memory?**

If yes, read the whole thing and stop thinking about streams. If no — or, and this
is the case people get wrong, if you do not actually know — stream it.

## Path

```java
Path p = Path.of("data", "books.csv");
p.getFileName()      // books.csv
p.getParent()        // data
p.resolve("x.txt")   // data/books.csv/x.txt
p.toAbsolutePath()
```

Verified on this machine, the separator is `/`; on Windows it is `\`. `Path.of`
with several arguments joins them correctly on either, which is why hard-coding a
separator into a string is a portability bug you can avoid for free.

`Path` replaced the older `File` class in Java 7, and `File` should be treated as
legacy — its methods return `boolean` for operations that fail, which is Section
28.1.1's sentinel problem, and `Files` throws instead.

## What the abstraction hides

Worth naming, because the hiding is why the abstraction is useful and also why it
occasionally surprises.

**Blocking.** `read()` does not return until data is available. On a file that is
microseconds; on a socket it can be forever. Chapter 31 returns to this.

**Physical structure.** Disks read in blocks of some thousands of bytes, and the
operating system caches aggressively. Your one-byte `read()` usually does not
touch the disk at all, which is why Section 29.1.3's measurement is smaller than
you might guess and still large.

**Failure.** Every operation can fail, at any point, with `IOException`. Chapter
28's material is not optional here: a stream is precisely where the world stops
cooperating.

Next: text, and the encoding question arriving for real.
