# Bytes and Buffers

Two programs read the same 200,000-byte file one byte at a time. One takes 43
milliseconds and the other takes 3.

They differ by a single wrapper object. What that wrapper does is worth
understanding beyond the fourteen-fold speedup, because the same shape — a fixed
cost per operation, much larger than the work itself — is behind cache lines,
`ArrayList` growth, database round trips, and network packets.

Under the text lies a byte stream, and under the byte stream lies a system call.
That last layer is why buffering exists.

## The measurement

A 200,000-byte file, read one byte at a time, three ways:

```
200,000 single reads, unbuffered : 43 ms
200,000 single reads, buffered   :  3 ms
readAllBytes                     :  0 ms
```

Fourteen times, for one wrapper.

## Why

`InputStream.read()` on a file stream is a **system call** — a transition from
your program into the operating system kernel and back. The transition costs a few
hundred nanoseconds regardless of how much data crosses it, because the processor
must switch privilege level, save state, and switch back.

Unbuffered, that is 200,000 transitions to move 200,000 bytes. About 215
nanoseconds each, and essentially all of it is overhead — the data was in the
operating system's cache the whole time.

`BufferedInputStream` keeps an internal array, by default 8192 bytes. Your `read()`
takes the next byte from the array, and only when the array is empty does it make
one system call to refill it. So 200,000 reads become about 25 system calls, and
the other 199,975 are array accesses.

The general form is worth extracting, because it recurs everywhere:

> **Batch the expensive operation.** When each unit of work carries a fixed
> overhead much larger than the work itself, do many units per operation.

Chapter 15's cache lines were this. Chapter 17's `ArrayList` growth was this.
Database round trips, network packets, and screen redraws are all this.

## What to actually do

**Wrap in a buffer whenever you read or write small pieces.** `BufferedReader`,
`BufferedInputStream`, `BufferedWriter`, `BufferedOutputStream`. `Files.
newBufferedReader` and `Files.lines` do it for you, which is a good reason to
prefer them.

**Do not wrap when you already read in bulk.** `Files.readAllBytes` makes one large
request; a buffer in front of it adds a copy and no benefit. The third row above is
that case.

**Do not buffer a buffer.** `new BufferedReader(Files.newBufferedReader(p))` is
two buffers and one wasted copy. Harmless and pointless.

## Flushing

The mirror image, and the one that produces confusing bugs.

A buffered writer holds your data in memory until the buffer fills. If the program
exits without closing the writer, the last partial buffer is never written — the
file exists, is shorter than expected, and is missing the end.

```java
try (BufferedWriter w = Files.newBufferedWriter(f)) {
    w.write(everything);
}                                  // close flushes
```

try-with-resources closes it, and closing flushes. Which sharpens the last
chapter's point considerably: the thing you are failing to release here is not a
handle, it is
*unwritten data*, and failing to release it loses information rather than leaking
a descriptor.

`flush()` exists for when you need the data out now without closing — a log you
want to survive a crash, a network stream where the other side is waiting for a
response before it will send more. Note that flushing gets the data to the
operating system, not to the disk; surviving a power failure needs
`FileChannel.force`, and that is a different and much slower guarantee.

## Bytes as data

For anything that is not text, you work in bytes:

```java
try (OutputStream out = Files.newOutputStream(path)) {
    for (int k = 0; k < 200_000; k++) out.write(k & 0xFF);
}
```

`write(int)` writes the low eight bits and ignores the rest, which is why the mask
is there for clarity rather than necessity.

`read()` returns an `int`, not a `byte`, and this is a real trap. The range is 0
to 255 for data and $-1$ for end-of-stream — three hundred and fifty-seven
distinct values in total, which does not fit in a `byte`. Writing

```java
byte b = (byte) in.read();
while (b != -1) { ... }
```

is wrong twice over: the byte `0xFF` becomes $-1$ and ends the loop early, and
Chapter 2's two's complement is why. The correct form keeps the `int`:

```java
int b;
while ((b = in.read()) != -1) { ... }
```

And when you do have a `byte` and want its numeric value, `b & 0xFF` is the
conversion, for the same reason.

## Binary formats

Reading and writing structured binary means `DataInputStream` and
`DataOutputStream`:

```java
out.writeInt(42);
out.writeUTF("hello");
```

`writeInt` writes four bytes, big-endian, which is Chapter 1's endianness decided
for you and stated in the specification — one of the places Java's
platform-independence is doing visible work.

Binary is compact and fast and you should still default to text for anything you
might have to look at. A malformed text file can be opened and read; a malformed
binary file requires a program that may itself be the thing that is broken.

The reasonable rule: **text unless there is a measured reason**, and the reason is
usually size or throughput at a scale you have confirmed.

Next: giving those bytes a structure.
