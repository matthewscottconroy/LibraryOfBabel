# Persistence

Everything your programs have computed so far vanished when they stopped.

Storage does not vanish. That is the whole of it, and the consequences run further
than the sentence suggests, because something that outlives the process outlives
the assumptions the process was written under.

A file you write today will be read by a program you have not written, possibly
on a machine with a different operating system, possibly after the format has
changed twice. A file you read was written by something you do not control, and
it may be truncated, in the wrong encoding, or produced by a version that
predates the field you are looking for.

So a file format is a **contract**, in the same sense Chapter 16 meant it, and the
other party to it is your future self.

Section 29.1 is the mechanism. The **stream** abstraction, which is the same idea
across every language and is worth understanding once. Reading and writing text,
where Chapter 4's encodings return and this time get things wrong visibly. Then
bytes and buffers, where a measurement explains why `BufferedReader` exists.

Section 29.2 is about structure. Delimited data — CSV, which everyone thinks is
easy and is not, and Section 29.2.1 shows exactly where it stops being easy. Then
the design of a format that will still be readable in five years. Then a look at
databases, which is what you reach for when files stop being enough, and the
chapter explains what "enough" means.

Two ideas to carry through.

**A file is a sequence of bytes and nothing else.** Every structure — lines,
fields, records, objects — is an interpretation you impose, exactly as Chapter 1
said of everything else. There is no such thing as a text file; there is a byte
file and an agreement about how to read it.

**Writing can be interrupted.** The process can be killed, the machine can lose
power, the disk can fill, between your first byte and your last. A program that
has not thought about this has a state it can be interrupted into, and it is
usually one where the old data is gone and the new data is incomplete. Section
29.2.2 shows the standard fix, which is two lines.
