# Encodings on the Wire

A code point is a number. Files hold bytes. Something has to get us from one to
the other, and that something is an encoding.

## The obvious approach, and its cost

Since code points go up to U+10FFFF, they fit in 21 bits. Round up to 32 and
store every character as four bytes. That is **UTF-32**, and it has one genuine
virtue: every character is the same size, so the *n*-th character is at byte
4*n*, and indexing is arithmetic.

It also quadruples the size of every English document, since ASCII text becomes
three zero bytes per character plus one meaningful one. Nobody was willing to pay
that, so UTF-32 is used inside programs occasionally and almost never in files.

**UTF-16** uses two bytes for most characters and four for the rest, via the
surrogate pairs from the last lesson. It doubles the size of English text and
still has variable width, which means it pays the cost of both approaches. It is
widely used inside programs — Java and JavaScript both use it — mostly for
historical reasons rather than because anyone would choose it now.

## UTF-8

**UTF-8** is the one that won, and it is worth understanding because the design is
genuinely elegant.

It uses one to four bytes per character, with the length signaled by the leading
bits of the first byte:

```
code point range        bytes    pattern
U+0000 – U+007F         1        0xxxxxxx
U+0080 – U+07FF         2        110xxxxx 10xxxxxx
U+0800 – U+FFFF         3        1110xxxx 10xxxxxx 10xxxxxx
U+10000 – U+10FFFF      4        11110xxx 10xxxxxx 10xxxxxx 10xxxxxx
```

The `x`s are where the code point's bits go. Read the scheme carefully and you
will see four properties fall out, none of them accidental.

**ASCII is unchanged.** Code points below 128 use the one-byte form `0xxxxxxx`,
which is exactly the ASCII byte. An ASCII file *is* a UTF-8 file, unmodified. That
compatibility is why UTF-8 could be adopted incrementally rather than by a flag
day.

**The byte tells you its own role.** A byte starting `0` is a complete character.
One starting `11` begins a multi-byte sequence, and the number of leading 1s says
how many bytes follow. One starting `10` is a continuation. So you can look at any
byte in isolation and know whether it starts a character.

**You can resynchronize.** If you land in the middle of a file — or a stream drops
bytes — scan forward until you find a byte that does not start with `10`, and you
are at a character boundary. A corrupted section damages a few characters rather
than everything after it.

**ASCII bytes never appear inside multi-byte characters.** Every byte of a
multi-byte sequence has its high bit set, so an ASCII value like `/` or `\0` can
never occur as part of some other character. Software that searches for those
bytes keeps working unmodified — which mattered enormously for filesystems and C
libraries.

## Worked examples

`é` is U+00E9, which is 233, in the two-byte range:

```
U+00E9 = 0000 0000 1110 1001
take the low 11 bits:  000 1110 1001
split 5 + 6:           00011  101001
place into pattern:    110 00011  10 101001
                     = 11000011  10101001
                     = C3        A9
```

And indeed `é` in UTF-8 is the bytes `C3 A9` — which we met in Chapter 1 as the
two bytes that become `Ã©` when read under a single-byte encoding. Now you can
see exactly why: `C3` and `A9` are both legitimate single-byte characters in
ISO-8859-1, and a program using that encoding has no reason to suspect they
belong together.

GRINNING FACE, U+1F600, is in the four-byte range, and encodes as
`F0 9F 98 80`.

The word `café` is therefore 4 characters but 5 bytes: `c`, `a`, `f` are one byte
each, and `é` is two. If you have ever seen a database column reject a name that
"fits", this is why — the limit was in bytes and the count was in characters.

## The remaining problem

UTF-8 solves the encoding problem completely, and does not solve the *agreement*
problem at all.

A file of bytes still does not say what encoding it uses. If a program reads
UTF-8 bytes as ISO-8859-1, it gets mojibake, and nothing raises an error — the
Chapter 1 argument, unchanged. Encoding is metadata, and metadata has to travel
separately or by convention.

Different systems solve this differently. HTTP sends a `Content-Type` header. HTML
carries a `<meta charset>` declaration, which is a slight paradox since you must
guess an encoding to read the declaration that tells you the encoding — resolved
because the declaration is ASCII, and nearly all encodings agree on ASCII.

Some files begin with a **byte order mark**, U+FEFF, whose UTF-8 form `EF BB BF`
serves as a signature. It is not required in UTF-8 and causes as much trouble as
it prevents, because software that does not expect it sees three junk bytes at
the start of the file.

And a great deal of software falls back on the platform default encoding, which
is why a program can work on your machine and fail on a colleague's. **Always
specify the encoding explicitly when reading or writing text.** Java lets you
omit it and will use the platform default; do not accept the offer. Since Java
18, the default for file I/O is UTF-8 regardless of platform, which removed a
long-standing source of this bug — but code targeting earlier versions, and code
that reads the default from elsewhere, still carries the hazard.

## The rule

If you remember one operational thing from this chapter, remember this: **there
is no such thing as plain text.** There is text plus an encoding, and if you do
not know the encoding you do not have text — you have bytes and a hope.

Next, the same question asked of things that are not writing at all.
