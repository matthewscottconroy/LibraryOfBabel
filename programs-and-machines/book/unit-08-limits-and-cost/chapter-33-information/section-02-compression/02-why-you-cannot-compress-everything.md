# Why You Cannot Compress Everything

Here is a plausible business plan. `gzip` shrinks text by eighty percent, so run it
twice. Then again. Repeat until everything is a few bytes.

People have patented versions of this and raised money for it. The argument that
kills it is three lines long, requires no knowledge of the method being proposed,
and is the first outright impossibility proof in this book.

So let us try it. Compress something, compress the result, compress that:

```
original 9,000 -> gzip 108 -> gzip again 109 -> again 130
```

The second pass made it one byte larger. The third made it larger still.

That is not a defect of `gzip`. **No compressor can shrink every input**, and the
proof is three lines.

## The counting argument

Suppose a lossless compressor shrinks every input by at least one bit.

Consider all inputs of exactly $n$ bits. There are $2^n$ of them.

Each compresses to at most $n - 1$ bits. The number of distinct bit strings of
length $n - 1$ or less is

$$2^{n-1} + 2^{n-2} + \cdots + 2 + 1 = 2^n - 1$$

which is Chapter 17's sum of powers of two, in its third appearance.

So $2^n$ inputs must map into $2^n - 1$ outputs. By the pigeonhole principle, two
distinct inputs share an output — and the decompressor, given that output, cannot
know which was meant.

**So the compressor is not lossless.** Contradiction.

The conclusion follows immediately and it is stronger than "some inputs do not
shrink":

> For every lossless compressor, some inputs must **grow**.

## Verified

Two hundred random 64-byte inputs through `gzip`:

```
grew 200, shrank 0, unchanged 0
```

Every single one grew. Random data is incompressible by definition — it has
maximum entropy, there is no structure to exploit — and `gzip` adds a header, so
the output is larger.

Which is why every real compressor has a fallback: if compression made the block
bigger, store it uncompressed with a one-bit flag saying so. That flag is the
unavoidable growth, reduced to its minimum. `gzip`'s 44-byte output for a
zero-entropy input is mostly this header, and its 109-byte output for
already-compressed input is that header applied to something with nothing left to
find.

## What this means

**Compressors are specialists.** `gzip` is good at text because text repeats.
It is bad at already-compressed data, encrypted data, and random data — all of
which look the same to it, which is not a coincidence: encryption is meant to make
output indistinguishable from random.

If your compression ratio suddenly gets worse, suspect that something upstream
started compressing or encrypting.

**Compressing twice is worse than once.** The first pass removed the structure; the
second finds none and adds a header. The measured 108 to 109 to 130 is exactly
this.

**Any claim of universal compression is false.** This has not stopped people from
claiming it — there is a small history of patents and investment schemes promising
compression of arbitrary data — and the argument above refutes all of them in three
lines, without examining the method. Comp.compression's FAQ maintained a standing
challenge on this for years and nobody collected.

That is a useful thing to have: **an impossibility result lets you dismiss a class
of claim without inspecting it.** You do not need to find the flaw in the scheme;
you know one is there.

## The shape of the argument

Worth extracting, because Chapter 34 uses the same shape twice.

1. Count the things that exist: $2^n$ inputs.
2. Count the things the mechanism can produce or distinguish: $2^n - 1$ outputs.
3. The second is smaller, so the mechanism cannot be one-to-one.

Section 32.2.1's sorting bound is this exact argument: $n!$ orderings against $2^k$
comparison outcomes. Chapter 34's uncomputability results are relatives of it —
countably many programs against uncountably many functions, which is the same
observation with infinite sets and Cantor's diagonal in place of pigeonhole.

Three impossibility results, one technique. It is the most reliable tool in the
subject for proving that something cannot be done, and it works precisely because
it says nothing about *how* the thing would be done.

## Most strings are incompressible

A sharper form of the same counting.

How many $n$-bit strings can be compressed to $n - 10$ bits or fewer? At most
$2^{n-9} - 1$, by the same sum. Against $2^n$ strings in total, that is a fraction
under $1/512$.

**So fewer than one string in five hundred can be shrunk by even ten bits.**
Fewer than one in a million by twenty.

The overwhelming majority of bit strings are incompressible. Compression works at
all only because the files we actually care about — text, images, audio,
programs — are a vanishingly small and highly structured corner of the space of
possible files.

That observation is the bridge to Chapter 34. A string that cannot be described
more briefly than by writing it out is, in Kolmogorov's sense, **random** — and
the counting above says almost every string is. Chapter 34 then shows that
although almost all strings are random, no program can identify a single one of
them.

Chapter 34 is the last technical chapter, and it is the one where the impossibility
results stop being about compressors and start being about programs.
