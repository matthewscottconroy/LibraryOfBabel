# Exercises

Exercises marked **[carries forward]** introduce something a later chapter
assumes.

## Warm-up

**4.1.** Using the fact that `'A'` is 65 and `'a'` is 97, give the numeric values
of `'C'`, `'z'`, and `'5'`. What is `'5'` minus `'0'`, and why is that useful?

**4.2.** ASCII sets the case bit at value 32. Show, in binary, that `'M'` and
`'m'` differ in exactly one bit position, and say which.

**4.3.** How many code points can Unicode define in total? Roughly what fraction
are currently assigned?

**4.4.** How many bytes does `hello` take in UTF-8? In UTF-16? In UTF-32?

## Working through

**4.5. [carries forward]** Encode U+00FC (`ü`) into UTF-8 by hand, using the
two-byte pattern from Section 4.1.3. Show your working, then check the result.

**4.6.** Decode the UTF-8 byte sequence `E2 82 AC` by hand. Which code point is
it, and what character?

**4.7.** The word `naïve` is five characters. How many bytes is it in UTF-8?
Explain the discrepancy to someone who has just had a database reject it as "too
long" for a five-character field.

**4.8. [carries forward]** In Java, `"\uD83D\uDE00".length()` returns 2 —
the literal is one emoji written as a surrogate pair. Explain why the answer is 2, in
terms of Java's `char` type and the history in Section 4.1.2. What would you use
instead to get 1?

**4.9.** Byte 233 is `é` in ISO-8859-1 and `щ` in ISO-8859-5. A file contains that
byte and nothing indicating an encoding. Is the file wrong? Is either reading
wrong? What exactly is missing?

## Reasoning

**4.10.** UTF-8 guarantees that no byte of a multi-byte character has a value
below 128. Name two concrete benefits, and say what would break if the guarantee
did not hold.

**4.11.** An HTML file declares its encoding with a `<meta charset>` tag inside
the file. To read the tag you must already know the encoding. Explain how this
apparent circularity is resolved in practice.

**4.12.** `é` can be one code point or two. Give a situation where treating them
as different is correct, and one where treating them as the same is correct.

**4.13.** The sampling theorem says a 20 kHz signal needs a rate above 40 kHz. CD
audio uses 44.1 kHz rather than exactly 40 kHz. Give a reason for the margin.

**4.14.** Explain aliasing to someone who has noticed that wagon wheels sometimes
appear to spin backwards in films. What is being sampled, and at what rate?

## Going further

**4.15.** Section 4.2.2 lists four questions answered by any representation.
Apply all four to a calendar date. Then apply them to a *timestamp* including
time of day, and say which additional thing gets discarded and what bug that
causes.

**4.16.** Human eyes have three cone types, so three numbers suffice for color.
Design a color representation for a species with five. What changes, and what
does not?

**4.17.** UTF-8 was designed so that ASCII files are already valid UTF-8. Estimate
what adoption would have looked like without that property, and say which of
UTF-8's other three virtues you would have given up to keep it.

**4.18.** Pick any file format you use — a photograph, a song, a document — and
answer the four questions of Section 4.2.2 about it. What does it keep, how
finely, what are its limits, and what is its policy for things that do not fit?
You may need to look up the format, which is the point.
