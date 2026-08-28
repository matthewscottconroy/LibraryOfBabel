# Reading and Writing Text

Chapter 4 argued that an encoding is an agreement, and you may have filed that
under philosophy.

Here it is as a bug report. A file written on one machine displays on another as
`GÃ¶del` instead of `Gödel`. Nothing threw. Nothing was corrupted. Both programs
did exactly what they were told, and the file is fine.

Chapter 4 argued that an encoding is an agreement and that the same bytes read
under two agreements mean two different things. Here is that, on a real file.

## Writing

```java
try (BufferedWriter w = Files.newBufferedWriter(f, StandardCharsets.UTF_8)) {
    w.write("title,author,year");
    w.newLine();
    for (Book b : books) { w.write(b.toLine()); w.newLine(); }
}
```

Verified: `wrote 207 bytes`.

Two things done deliberately.

**The charset is stated.** `StandardCharsets.UTF_8`, explicitly, on every read and
write in this chapter.

Before Java 18, omitting it meant the platform default — which is UTF-8 on most
Linux systems, was `windows-1252` on many Windows installations, and could differ
between the machine that wrote the file and the machine that read it. That is a
bug that does not reproduce, appears only for non-ASCII data, and is discovered by
a user in another country.

Java 18 made UTF-8 the default everywhere, which fixed it for new code. State it
anyway: it costs one argument, it documents the intent, and it works on older
runtimes.

**`newLine()` rather than `"\n"`.** It writes the platform's line separator —
`\n` on Unix, `\r\n` on Windows. Whether you want that is a real question, taken
up below.

## Reading

```java
try (Stream<String> lines = Files.lines(f, StandardCharsets.UTF_8)) {
    lines.skip(1).map(Book::fromLine).forEach(...);
}
```

Verified:

```
Book[title=Structure and Interpretation of Computer Programs, author=Abelson and Sussman, year=1985]
Book[title=The Mythical Man-Month, author=Brooks, year=1975]
Book[title=Gödel, Escher, Bach, author=Hofstadter, year=1979]
Book[title=Programming, with a comma, author=Nobody, year=2026]
```

`skip(1)` drops the header. Line splitting handles `\n`, `\r\n` and a bare `\r`,
which is one fewer thing to get wrong.

## Getting the encoding wrong

The same 207 bytes, decoded two ways:

```java
byte[] utf8 = Files.readAllBytes(f);
String correct = new String(utf8, StandardCharsets.UTF_8);
String wrong   = new String(utf8, StandardCharsets.ISO_8859_1);
```

Verified:

```
bytes on disk: 207
as UTF-8 chars: 206
as Latin-1 chars: 207

UTF-8   : Gödel
Latin-1 : GÃ¶del
```

The file contains "Gödel". Read as UTF-8 it is "Gödel". Read as Latin-1 it is
"GÃ¶del".

The reason is exactly Chapter 4's. `ö` is one character, encoded in UTF-8 as two
bytes, `0xC3 0xB6`. Latin-1 is a one-byte encoding, so it decodes those as two
characters: `Ã` and `¶`. Hence 207 characters from 207 bytes rather than 206.

Two things worth noticing about this failure.

**Nothing threw.** Latin-1 assigns a character to all 256 byte values, so no byte
sequence is invalid and no error is possible. The program read the file, got
nonsense, and reported success. This is Section 28.2.3's "plausible wrong answer"
in its purest form.

**It is invisible in ASCII.** Every character in "The Mythical Man-Month" decodes
identically under both. A test suite with English test data passes, and the bug
ships and is found by someone called Müller.

That pattern — `Ã©`, `Ã¶`, `â€™` — is called **mojibake**, and once you know it
means "UTF-8 read as Latin-1" you will see it everywhere: in badly configured web
pages, in exported spreadsheets, in database columns.

## Detecting the encoding

You cannot, reliably. A byte sequence does not carry its encoding, so a file's
encoding is metadata that lives outside the file — in a convention, a header, or
a specification.

The partial answers:

**Convention.** "This program's files are UTF-8." The best answer, and it works
until someone hands you a file from elsewhere.

**Declaration.** HTTP sends a `Content-Type` header, HTML has a `<meta charset>`,
XML has a declaration. All of them are the file saying what it is.

**A byte-order mark.** A three-byte prefix, `EF BB BF`, marking UTF-8. Legal,
occasionally useful, and a nuisance — it appears as an invisible character at the
start of a string and breaks parsers that expect the first byte to be data. Do not
write one.

**Guessing.** Statistical detection, as browsers do. It is right most of the time,
which is not a property you want in a data pipeline.

The practical position: **decide, write it down, and state it in code.**

## Line endings

The other portability trap, and it is smaller than the encoding one but it bites
more often.

Unix ends a line with `\n`. Windows ends it with `\r\n`. Classic Mac used `\r`.
The difference is a historical accident of teletype mechanics — carriage return
and line feed were two physical motions — and it has outlived teletypes by fifty
years.

For **reading**, do nothing: `readLine`, `Files.lines` and `readAllLines` all
handle every variant.

For **writing**, decide:

- If the file is for a person on this machine, `newLine()` is right.
- If it is a data format read by other programs, **write `\n` unconditionally**.
  Every reader handles it, and a file whose bytes depend on which machine produced
  it is a file that will differ under version control for no reason.

The second case is more common than the first, so `\n` is the better default and
`newLine()` is the special case.

If you have ever seen `^M` at the end of every line in a text editor, or a git
diff claiming every line changed when nothing did, that is this.

Next: what happens underneath, and why buffering exists.
