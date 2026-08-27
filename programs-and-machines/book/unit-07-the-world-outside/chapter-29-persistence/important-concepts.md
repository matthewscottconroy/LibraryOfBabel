# Important Concepts

**A file is a sequence of bytes** — every structure above that is an interpretation
you impose. There is no text file, only a byte file and an agreement about how to
read it.

**Storage outlives assumptions** — data will be read by a program you have not
written, so a file format is a contract with your future self.

**Stream** — a sequence of items available one at a time, in order, possibly with
no known end. Deliberately less than a file offers, so that the same code works on
sockets and pipes.

**The four families** — `InputStream` and `OutputStream` for bytes, `Reader` and
`Writer` for characters. Confusing bytes with characters is the commonest I/O bug.

**Decorators** — buffering, decompression and decoding are added by wrapping
rather than by subclassing, which is why `BufferedInputStream` can buffer a
socket. Composability was chosen over convenience, and convenience was added later.

**The convenience layer** — `Files.readString`, `readAllLines`, `readAllBytes`,
`writeString` for things that fit in memory; `Files.lines` and
`newBufferedReader` for things that do not.

**Path over File** — `File`'s operations return `boolean` on failure, which is
Section 28.1.1's sentinel problem; `Files` throws.

**State the charset** — before Java 18 the default was the platform's, so a file
could be written and read under different agreements on different machines. Java
18 made it UTF-8; state it anyway.

**Mojibake** — `Gödel` read as Latin-1 becomes `GÃ¶del`, because the two bytes of
`ö` are decoded as two characters. Nothing throws: Latin-1 assigns a character to
every byte, so no input is invalid.

**Encoding cannot be detected reliably** — a byte sequence does not carry its
encoding. The answers are convention, declaration, a byte-order mark, or guessing,
and only the first two are dependable.

**Line endings** — read anything, write `\n` for data formats and `newLine()` only
for files a local person will open. A file whose bytes depend on the machine that
wrote it will churn in version control.

**Buffering** — measured at 43 ms against 3 ms for 200,000 single-byte reads. Each
unbuffered read is a system call; a buffer turns 200,000 of them into about 25.

**Batch the expensive operation** — the general form, which also explains cache
lines, `ArrayList` growth, database round trips and network packets.

**Flushing** — a buffered writer holds data until closed. Exiting without closing
loses the tail; the resource being released here is unwritten data, not a handle.

**read() returns an int** — 0 to 255 plus $-1$, which does not fit in a `byte`.
Casting to `byte` makes `0xFF` into $-1$ and ends the loop early.

**Text unless there is a measured reason** — a malformed text file can be opened
and read; a malformed binary file needs a program that may itself be broken.

**The delimiter problem** — any delimiter can appear in the data. A rarer one makes
the bug arrive later, not never.

**CSV quoting** — wrap the field and double the quote inside. This means
`split(",")` cannot parse CSV; a twelve-line state machine can, and a quoted field
may contain a newline, so CSV is not even line-oriented.

**Use a library for CSV you did not write** — hand-rolled parsers are for data you
also produce.

**A header** lets a reader find a column rather than count to it, which is what
makes adding a field survivable.

**Format choice** — CSV for tabular, JSON for structured, JSON Lines for many
structured records, and whatever your ecosystem uses for configuration. Do not
invent a format.

**The interrupted write** — a plain overwrite has a window in which the old data is
gone and the new data is partial. Write to a temporary file and rename over the
target; a rename within a filesystem is atomic.

**Versioning** — put a version in the file. Adding a field should be safe,
removing or reordering needs a new version, and changing a field's *meaning* is
the violation no check can detect.

**Absent values** — every candidate is ambiguous against real data. Decide, write
it down, and prefer the empty field. It is the sentinel problem in a format.

**Timestamps in UTC, ISO 8601, with the offset. Money in integer minor units.**
Both are Chapter 1's argument made permanent by being written to disk.

**A file format is an interface** — you can refactor a class and you cannot
refactor a file somebody already has.

**Where files run out** — concurrent writers, partial updates, queries, atomicity
across records, and readers during a write. That list is what a database provides.

**ACID** — atomicity, consistency, isolation, durability. Each is something you
could implement over files, and the field took two decades to do it well.

**SQL is declarative** — you state the result and the planner chooses the
algorithm, using statistics about the actual data. The only widely used
declarative language most programmers meet.

**SQL injection** — building a query by concatenation puts user data into the
grammar. A `PreparedStatement` placeholder keeps it a value. Chapter 24's
distinction, with consequences.

**The ladder** — a file, then SQLite, then a database server. A program with a CSV
file, an in-memory index, a locking scheme and a backup routine has reimplemented
SQLite badly.
