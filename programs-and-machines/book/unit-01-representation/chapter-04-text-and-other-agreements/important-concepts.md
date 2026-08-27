# Key Concepts

**ASCII.** A 7-bit code standardized in 1963: 128 characters covering English
letters, digits, punctuation, and control codes. Still the base layer of
everything that followed.

**ASCII's structural choices.** Digits begin at 48, so a digit's value is its low
four bits. Letters are contiguous, so range comparisons work. Upper and lower
case differ by 32 — a single bit — so case conversion is a bitwise operation.

**ASCII's failed assumptions.** That text is English; that 128 characters
suffice; that one character is one byte; that the eighth bit is spare. Every one
of these has since broken.

**Code page chaos.** With 128 values above ASCII unclaimed, every vendor and
nation defined its own extension. All agreed below 128 and disagreed above, so
English moved safely between systems and anything accented did not.

**The Unicode distinction.** Unicode's central contribution is separating *which
character this is* (a **code point**) from *how it is stored* (an **encoding**).
ASCII merged the two; nearly every text confusion comes from still merging them.

**Code point.** A number identifying an abstract character, written `U+` plus
hex, ranging U+0000 to U+10FFFF. The first 128 are deliberately identical to
ASCII.

**Characters, not glyphs.** A code point names an abstract character, not a
drawing of it. Fonts map code points to glyphs. This is why the ohm sign and
Greek omega are separate code points despite looking the same.

**Normalization.** The same text may have several code-point sequences — `é` as
one code point or as `e` plus a combining accent. NFC and NFD convert to a
canonical form so comparison is meaningful. Normalize before comparing
user-supplied text.

**Grapheme clusters.** What a reader calls "one character" may be several code
points joined together, as in emoji built from zero-width joiners. "How long is
this string" has three defensible answers: bytes, code points, or grapheme
clusters.

**UTF-32, UTF-16, UTF-8.** Fixed four bytes; two-or-four bytes via surrogate
pairs; one-to-four bytes with the length signaled by the leading bits. UTF-8 won.

**Why UTF-8 won.** ASCII files are already valid UTF-8; each byte announces
whether it starts a character; a reader can resynchronize after damage; and no
byte of a multi-byte character can be mistaken for an ASCII byte, so older
byte-oriented software kept working.

**Java's `char` is 16 bits.** A 1990s decision made when Unicode was expected to
fit in 65,536 code points. Characters above U+FFFF need two `char` values, so
`"\uD83D\uDE00".length()` is 2 and `codePointCount` is needed to get 1.

**Encoding is metadata.** Bytes never say how to read themselves. The encoding
must travel separately — an HTTP header, a declaration, a convention — or be
assumed. Always state the encoding explicitly rather than accepting a platform
default.

**There is no plain text.** There is text plus an encoding. Without the encoding
you have bytes and a hope.

**Metamers and three-channel color.** Human retinas have three cone types, so
three numbers suffice to reproduce a color for a human observer. Eight bits per
channel gives 16,777,216 combinations; the quantization shows up as banding on
smooth gradients.

**Sampling.** Continuous signals are recorded by measuring at intervals. The
Nyquist–Shannon theorem: a signal with no frequencies above *f* can be
reconstructed exactly from samples taken faster than 2*f*. Sample too slowly and
high frequencies alias into low ones.

**Bit depth and quantization noise.** Each sample is rounded to a level; the error
is noise. CD audio uses 16 bits at 44.1 kHz, giving 1,411,200 bits per second.

**The four questions.** Every representation answers: what am I keeping, how
finely, what are my limits, and what happens to what does not fit? The last has a
follow-up that matters most — is the policy loud or silent? In this unit it was
silent every time.
