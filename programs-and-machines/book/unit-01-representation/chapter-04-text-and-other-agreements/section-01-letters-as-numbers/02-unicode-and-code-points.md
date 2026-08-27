# Unicode and Code Points

The obvious fix for "128 characters is not enough" is a bigger table. Unicode does
provide one — but its important contribution is a *distinction*, and if you take
only one thing from this chapter, take that.

## The distinction

Unicode separates two questions that ASCII had merged:

1. **Which character is this?** Answered by a number called a **code point**.
2. **How is that number stored as bytes?** Answered by an **encoding**.

In ASCII these were the same question. The letter `A` *is* 65 *is* the byte
`01000001`, and no one had reason to distinguish the three.

Unicode prises them apart. The character `A` has code point U+0041 — that is its
identity, permanent and independent of any file. How U+0041 turns into bytes is a
separate decision, and there are several valid answers, and *that* is the choice
a file makes.

Once you have this distinction, a great many confusions dissolve, because most of
them come from conflating "what character is this" with "what bytes are these".

## Code points

A code point is written `U+` followed by hexadecimal digits — Chapter 2's
notation, earning its keep.

```
U+0041   A            LATIN CAPITAL LETTER A
U+0061   a            LATIN SMALL LETTER A
U+00E9   é            LATIN SMALL LETTER E WITH ACUTE
U+03A9   Ω            GREEK CAPITAL LETTER OMEGA
U+4E2D   (CJK)        CJK UNIFIED IDEOGRAPH-4E2D
U+1F600  (emoji)      GRINNING FACE
```

The range runs from U+0000 to U+10FFFF — about 1.1 million possible code points,
of which around 150,000 are currently assigned. There is room.

Two design decisions are worth noting.

**The first 128 code points are exactly ASCII.** U+0041 is 65, the same number
ASCII used. This was not forced; it was chosen, to make the transition
survivable.

**Code points name characters, not glyphs.** U+0041 is "LATIN CAPITAL LETTER A"
— the abstract character — not any particular drawing of it. The shapes in a
serif font and a handwriting font are wildly different pictures of the same code
point. A font maps code points to glyphs; Unicode stops one level above that.

This is why the ohm sign and Greek omega have separate code points despite
looking identical: they are different characters that happen to share a glyph.
And it is why `A` and `a` are separate code points despite being "the same
letter" in some senses — Unicode is encoding characters, and case is a property
characters have.

## The parts that stay hard

Unicode is a repair, not a solution, and some difficulties are intrinsic.

**The same text can have two representations.** `é` can be the single code point
U+00E9, or it can be `e` (U+0065) followed by COMBINING ACUTE ACCENT (U+0301).
Both display identically. Both are correct. They are not equal as sequences of
code points, so a naive comparison says two identical-looking strings differ.

Unicode's answer is **normalization**: procedures that convert text to a
canonical choice, so comparison becomes meaningful. NFC prefers the single
combined code point; NFD prefers the decomposed form. If you compare
user-supplied text — usernames, filenames, search terms — you should normalize
first, and most people learn this by being bitten.

**Some characters have no single obvious length.** The family emoji — a man, a
woman and a girl shown as one picture — is three separate human-figure code points
joined by invisible ZERO WIDTH JOINER characters (U+200D), and depending on what
you are counting it is one thing, three things, or five code points. Skin
tone modifiers compound this. The question "how many characters is this string"
turns out not to have one answer, and asking it precisely means saying whether
you mean bytes, code points, or user-perceived characters — which Unicode calls
**grapheme clusters**.

**Sorting is cultural.** Alphabetical order differs by language: in Swedish `ä`
sorts after `z`, while in German it sorts with `a`. There is no universal correct
order, so Unicode supplies a collation algorithm that takes a locale, and the
answer depends on who is asking.

None of this is over-engineering. Each is a real property of human writing that
ASCII avoided by ignoring every language but one.

## Java's inheritance

One detail that will matter in Unit IV, and which you can now understand.

Java's `char` is 16 bits. That was a reasonable decision in the early 1990s, when
Unicode was expected to need at most 65,536 code points and 16 bits looked
sufficient forever. It was not sufficient. Unicode grew past U+FFFF, and Java was
left with a `char` type too small to hold a code point.

The workaround is that characters above U+FFFF are stored as *two* `char` values,
called a surrogate pair. Which means:

```
"\uD83D\uDE00".length()  →  2      // the same emoji, written as its surrogate pair
```

The string contains one character, and `length()` returns 2, because `length()`
counts 16-bit units rather than characters. It is not a bug. It is a 16-bit
decision from 1993 showing through, and it is why `codePointCount` exists as a
separate method that returns 1.

Watch how directly this descends from Chapter 1. A fixed width was chosen in
advance; the world did not fit; the format now carries a workaround forever. The
only thing that has changed since Chapter 1 is the size of the box.

Next: how code points become bytes.
