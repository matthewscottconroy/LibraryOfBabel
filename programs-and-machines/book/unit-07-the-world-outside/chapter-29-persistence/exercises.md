# Exercises

**29.1** Write a list of records to a CSV file with a header, then read it back
and confirm the round trip. State the charset explicitly on both sides.

**29.2** Write a file containing a non-ASCII character. Read it back twice, once
as UTF-8 and once as ISO-8859-1, and print both the character count and the text.
Explain the difference in byte terms.

**29.3** Take a file whose content is entirely ASCII and repeat Exercise 29.2.
Explain why the bug is invisible, and say what that implies about test data.

**29.4** *Measurement.* Read a 200,000-byte file one byte at a time, unbuffered
and then wrapped in a `BufferedInputStream`. Report both times and the ratio.
Then explain where the cost went, using the phrase "system call".

**29.5** Write to a `BufferedWriter` without closing it — no try-with-resources,
no `flush` — and let the program exit. Report the file's size against what you
wrote. Explain in terms of Section 28.2.2.

**29.6** Write a byte-reading loop as `byte b = (byte) in.read();` and run it on a
file containing the byte `0xFF`. Report what happens and explain it using Chapter
2.

**29.7** Implement the quoted-field CSV parser from Section 29.2.1. Test it on a
field containing a comma, a field containing a quote, and a field containing both.
Then find an input it gets wrong, and check your answer against RFC 4180.

**29.8** Implement the atomic write: temporary file plus rename. Then simulate an
interruption by throwing partway through the write, and confirm the original file
is intact and the temporary file is left behind.

**29.9** Design a file format for a to-do list: tasks with a description, a due
date, a done flag, and optional tags. Write down the format including its version,
its encoding, its comment convention, and how an absent due date is represented.
Then write the reader and the writer.

**29.10** Take your format from 29.9 and add a field. Confirm that your reader
still reads version 1 files. If it does not, say what you would change about the
design.

**29.11** Write a program that reads a CSV file and reports, for each malformed
line, the line number and what was wrong. Give it a file with three different
kinds of error and confirm all three are reported rather than the first.

**29.12** *Design, no code.* You have a program storing ten thousand records in a
CSV file. Name the specific change in requirements that would make you move to
SQLite, and the further change that would make you move to a database server.
Justify each against Section 29.2.3's list.

**29.13** *Longer.* [carries forward] Give Chapter 25's interpreter the ability to
read a program from a file named on the command line. Handle the file not existing,
the file being empty, and the file being in the wrong encoding, each with a message
naming what happened. Chapter 31 asks you to make it serve programs over a socket.
