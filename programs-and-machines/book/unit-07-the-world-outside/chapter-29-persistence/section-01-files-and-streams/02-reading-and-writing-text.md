# Reading and Writing Text

Back in Chapter 4 you were told that an encoding is an agreement. It is possible
you filed that under philosophy and moved on. Most people do.

Here it is again as a bug report.

A file is written on one machine. It is read on another. The name of the
mathematician Kurt Gödel comes out the other end as `GÃ¶del`.

Now, the interesting part. Nothing threw an exception. Nothing was corrupted in
transit. The disk is fine, the network is fine, and both programs did exactly what
they were told to do. If you open the file with the right tool it is perfect. The
bytes on disk are not damaged in any way.

Somebody has been let down by an agreement, and this lesson is about who.

## Writing

```java
try (BufferedWriter w = Files.newBufferedWriter(f, StandardCharsets.UTF_8)) {
    w.write("title,author,year");
    w.newLine();
    for (Book b : books) { w.write(b.toLine()); w.newLine(); }
}
```

Verified: `wrote 207 bytes`.

Two choices in there were made on purpose, and both are worth a minute.

**The charset is stated out loud.** `StandardCharsets.UTF_8`, explicitly, on every
read and every write in this chapter.

You are allowed to leave it out. Before Java 18, leaving it out meant *the platform
default*, and the platform default was UTF-8 on most Linux systems and
`windows-1252` on a great many Windows installations. Read that sentence again with
a suspicious eye: the encoding depended on which machine the program happened to be
running on.

Which is where the `GÃ¶del` at the top of this lesson comes from. The bug never
reproduces on the developer's laptop, appears only when the data contains a
non-ASCII character, and is found several months later by a user in another
country.

Java 18 made UTF-8 the default everywhere and closed that hole for new code. State
it anyway. It costs you one argument, it tells the next reader what you intended,
and it keeps working on older runtimes.

**`newLine()` rather than `"\n"`.** This writes the platform's line separator —
`\n` on Unix, `\r\n` on Windows. Whether that is what you want turns out to be a
real question with a real answer, and we come back to it at the end.

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

`skip(1)` throws away the header row. The line splitting copes with `\n`, `\r\n`
and even a lonely `\r`, which is one fewer thing available for you to get wrong.

## Now break it on purpose

Here are the same 207 bytes, unchanged, decoded two different ways.

```java
byte[] utf8 = Files.readAllBytes(f);
String correct = new String(utf8, StandardCharsets.UTF_8);
String wrong   = new String(utf8, StandardCharsets.ISO_8859_1);
```

Before you look at the output, predict two things. First, which of those two lines
throws. Second, how many characters come out of 207 bytes in each case.

Verified:

```
bytes on disk: 207
as UTF-8 chars: 206
as Latin-1 chars: 207

UTF-8   : Gödel
Latin-1 : GÃ¶del
```

Neither line threw. And the character counts differ by exactly one.

That difference of one is the whole story, and it is Chapter 4's argument arriving
to collect. The character `ö` is one character. UTF-8 encodes it as two bytes,
`0xC3` and `0xB6`. Latin-1 is a one-byte-per-character encoding, so when it meets
those two bytes it does the only thing it knows how to do: it decodes them as two
characters, `Ã` and `¶`. Two characters where there should have been one. Hence 207
out of 207, instead of 206.

Two features of this failure deserve your attention more than the mechanism does.

**Nothing threw, and nothing could have.** Latin-1 assigns a character to all 256
possible byte values. There is no such thing as a byte sequence that is invalid
Latin-1. So the decoder cannot detect a problem, because from where it is standing
there is no problem — it was handed bytes, it turned them into characters, it did
its job. The program read the file, got nonsense, and reported complete success.
That is the "plausible wrong answer" of Section 28.2.3 in its purest available
form.

**It is completely invisible in ASCII.** Go back and look at the four book titles.
Every single character in "The Mythical Man-Month" decodes identically under both
encodings. So a test suite written in English passes. Every test. The bug ships,
and it is found by somebody called Müller.

That pattern — `Ã©` where an `é` should be, `Ã¶` for `ö`, `â€™` for a curly
apostrophe — has a name. It is **mojibake**, from the Japanese for "character
transformation". Once you know that it means *UTF-8 read as Latin-1*, you will
start seeing it everywhere: on badly configured web pages, in spreadsheets exported
from the wrong tool, in database columns that were set up in a hurry. It is one of
the small pleasures of knowing this material that a category of everyday ugliness
turns into a diagnosis.

## So can you detect the encoding?

No. Not reliably, and it is worth understanding why not, because the reason is
structural rather than a gap somebody will fill in later.

A sequence of bytes does not carry its encoding. There is nowhere for it to live.
The encoding of a file is metadata that necessarily sits *outside* the file — in a
convention, a header, or a specification — and if that outside information is lost
then the meaning is lost with it, no matter how intact the bytes are.

Which leaves four partial answers, in rough order of how much you should like them.

**Convention.** "The files this program writes are UTF-8." The best answer
available, and it works beautifully right up until somebody hands you a file from
somewhere else.

**Declaration.** HTTP sends a `Content-Type` header. HTML has a `<meta charset>`.
XML has a declaration on the first line. All of these are the file, or its wrapper,
announcing what it is.

**A byte-order mark.** Three bytes, `EF BB BF`, glued to the front to mark a file
as UTF-8. It is legal, it is occasionally useful, and it is mostly a nuisance,
because it shows up as an invisible character at the start of your first string and
quietly breaks any parser expecting the first byte to be data. Do not write one.

**Guessing.** Statistical detection, of the sort browsers do. It is right most of
the time. Ask yourself whether "right most of the time" is a property you want in
something that moves your data around.

The position to take: **decide, write it down, and say it in the code.**

## Line endings

One more portability trap. Smaller than the encoding one, and it bites more often.

Unix ends a line with `\n`. Windows ends it with `\r\n`. Classic Mac OS used a
bare `\r`. The disagreement is a fossil of teletype mechanics, back when a carriage
return and a line feed were two separate physical movements of an actual machine —
one to slide the carriage back, one to roll the paper up. Those machines have been
gone for fifty years. The two characters are still here.

For **reading**, do nothing at all. `readLine`, `Files.lines` and `readAllLines`
handle every variant without being asked.

For **writing**, you have to decide, and the decision is genuinely a decision:

- If the file is for a person sitting at this machine, `newLine()` is right.
- If it is a data format that other programs will read, **write `\n` and nothing
  else.** Every reader on earth copes with it, and a file whose actual bytes depend
  on which machine produced it is a file that will show up as changed in version
  control when nobody changed it.

The second case is far more common than the first. So `\n` is the better default,
and `newLine()` is the exception you reach for deliberately.

If you have ever seen `^M` sitting at the end of every line in an editor, or opened
a diff that claims all four hundred lines changed when you edited one — you have
already met this, without knowing its name.

Next: what is happening underneath all of this, and why buffering exists at all.
