# ASCII and Its Assumptions

To store text you need an agreement mapping characters to numbers.

Here is the thing worth noticing about that requirement: *any* agreement would do.
There is nothing about the letter `A` that makes 65 a better number for it than 12
or 200. The only property that matters is that everybody uses the same one.

Which was precisely the problem, because in the 1950s nobody did.

Different manufacturers used different codes. Moving a tape from an IBM machine
to a Univac meant translating every byte. The American Standards Association set
out to fix this, and in 1963 published **ASCII**, the American Standard Code for
Information Interchange.

## What it decided

ASCII is a 7-bit code: 128 characters, numbered 0 to 127.

```
  0–31    control characters (newline, tab, bell, ...)
  32      space
  33–47   ! " # $ % & ' ( ) * + , - . /
  48–57   0 1 2 3 4 5 6 7 8 9
  58–64   : ; < = > ? @
  65–90   A B C ... Z
  91–96   [ \ ] ^ _ `
  97–122  a b c ... z
  123–126 { | } ~
  127     delete
```

Look down that table for a minute before reading on. It appears to be an
arbitrary list — somebody had to put the characters somewhere, and they did.

It is not arbitrary. At least four of those placements are doing real work, and
you can find them yourself if you look at the *numbers* rather than the
characters.

**Digits start at 48.** So the numeric value of a digit character is the
character minus 48. Since 48 is `0110000` in binary, the low four bits of any
digit character are exactly its value: `'7'` is `0110111`, and the low nibble is
`0111`, which is 7. Converting a digit character to a number is a masking
operation.

**Letters are contiguous.** `A` through `Z` run from 65 to 90 with no gaps, so
comparisons and ranges work arithmetically. This is why `if (c >= 'A' && c <= 'Z')`
is a valid test for an uppercase letter, and it is a property of the *encoding*,
not a fact about the alphabet.

**Upper and lower case differ by exactly 32.** `A` is 65 and `a` is 97. And 32 is
$2^{5}$, a single bit:

```
A  1000001
a  1100001
   ^
   this bit alone
```

Changing case is flipping one bit. On 1963 hardware that mattered enormously; you
could uppercase a string with a bitwise AND and no lookup table at all.

**`delete` is 127 — all seven bits set.** On paper tape you deleted a character by
punching out every hole in its column, which physically cannot be undone and
produces the all-ones pattern. The code was designed around the medium.

None of that was inevitable. Every one of those is a decision, taken by particular
people in a particular room in 1963, and you are still living inside all four of
them.

## What it assumed

ASCII worked well, and it is still the foundation of nearly everything. But it
built in four assumptions, and every one of them has since failed.

**That text is English.** There is no `é`, no `ñ`, no `ü` — let alone Greek,
Cyrillic, Arabic, Hebrew, Devanagari, or Chinese. The name says it: *American*
Standard Code. For most of the world's writing, ASCII has nothing to offer.

**That 128 characters is enough.** It is enough for English. Chinese alone needs
tens of thousands.

**That one character is one byte.** With 7 bits in an 8-bit byte, the mapping is
trivially one-to-one, and a great deal of software was written assuming that the
number of bytes equals the number of characters. That assumption is baked into
older code in ways that are still being found.

**That the eighth bit is spare.** ASCII uses 7 of 8 bits. Everyone noticed the
free bit, and everyone used it differently.

## The years of chaos

That last point caused the damage, so it is worth being specific.

With 128 unused values available, manufacturers and national bodies each defined
their own extension. IBM's code page 437 put box-drawing characters and some
Greek there. Code page 850 put Western European accents there instead. ISO
defined a family: ISO-8859-1 for Western Europe, ISO-8859-5 for Cyrillic,
ISO-8859-7 for Greek, and so on. Russia had several mutually incompatible schemes
at once.

All of them agreed on the bottom 128 values, because all of them were ASCII down
there. All of them disagreed above 127.

Follow the consequence through. A document of pure English text moved between any
two systems perfectly, every time. A document containing one accented character
became a lottery.

Byte 233 is `é` in ISO-8859-1, `щ` in ISO-8859-5, and `ι` in ISO-8859-7. Three
different letters from three different alphabets, one byte, and **nothing in the
file said which one was meant.** The receiving program guessed — normally by
assuming its own local default — and when it guessed wrong, the text was quietly
wrong with no error anywhere.

This is the world Unicode was created to end, and understanding how bad it was
makes Unicode's design decisions look less like over-engineering.

## What survived

ASCII is not a historical curiosity. It is still the base layer of the modern
system: the first 128 Unicode code points are exactly ASCII, with the same
numbers, and UTF-8 encodes every one of them as a single byte with the same value
ASCII would have used.

That was a deliberate compatibility decision, and it is why a plain English text
file written in 1975 opens correctly today, and why so much software that
predates Unicode kept working. We will see in two lessons how much design effort
went into buying that property.

Next: what Unicode actually is, which is less a bigger table than a different
idea about what a table is for.
