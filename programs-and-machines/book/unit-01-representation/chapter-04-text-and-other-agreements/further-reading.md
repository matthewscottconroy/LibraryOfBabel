# Further Reading

## Start here

Spolsky, J. (2003). "The Absolute Minimum Every Software Developer Absolutely,
Positively Must Know About Unicode and Character Sets (No Excuses!)."
*Joel on Software*, 8 October 2003.

The best short introduction to this material, and the title is not an
exaggeration. It covers the same ground as Section 4.1 in about twenty minutes,
in a voice you will either enjoy or not. Slightly dated in its details; entirely
sound in its argument.

## Unicode

*The Unicode Standard*, current version. The Unicode Consortium.

The authority, freely available online, and much more readable than a standard
has any right to be. Chapter 2, "General Structure", is the part that repays a
direct reading; the rest is reference.

Becker, J. D. (1988). *Unicode 88*. Xerox Corporation.

The original proposal, short and available online. Worth reading for the argument
it makes about why code pages could not be patched — and for the 16-bit
assumption, visible in its original context.

Pike, R., & Thompson, K. (1993). "Hello World, or Καλημέρα κόσμε, or Konnichiwa
Sekai." *Proceedings of the Winter 1993 USENIX Conference*.

(The title is printed in the original with the Greek in Greek script and the
Japanese in Japanese script — the point being that the paper could be typeset at
all. It is transliterated here because the font this book is set in does not
carry Japanese, which is itself a small demonstration of the chapter's argument:
the code points are fine, and the *rendering* is a separate agreement that can
fail on its own.)

The paper introducing UTF-8 to the wider world, describing how Plan 9 was
converted. Short, and the design rationale is laid out plainly by the people who
did it.

## Text processing in practice

*The Java Language Specification*, Java SE 17 edition. Oracle. Section 3.1,
"Unicode".

The statement that Java source is written in Unicode and that `char` is a UTF-16
code unit, with the surrogate-pair consequences spelled out.

The `java.text.Normalizer` class documentation, and `String.codePoints()`.

Worth reading once now, so that when you need normalization in Unit IV you
remember it exists.

## Sampling and media

Nyquist, H. (1928). "Certain Topics in Telegraph Transmission Theory."
*Transactions of the AIEE*, 47(2), 617–644.

Shannon, C. E. (1949). "Communication in the Presence of Noise." *Proceedings of
the IRE*, 37(1), 10–21.

The sampling theorem in its original statements. The Shannon paper is the more
readable of the two.

Smith, S. W. (1997). *The Scientist and Engineer's Guide to Digital Signal
Processing*. California Technical Publishing.

Free online, and unusually patient with readers who are not already engineers.
Chapters 3 and 4 cover sampling and quantization at exactly the level Section 4.2
gestures at.

## For when text breaks

The `file` command on any Unix system, and `iconv` for converting between
encodings.

Not reading, but the tools you will actually reach for. When a file arrives
garbled, `file` will guess its encoding and `iconv` will convert it, and knowing
both exist will save you an afternoon at some point.
