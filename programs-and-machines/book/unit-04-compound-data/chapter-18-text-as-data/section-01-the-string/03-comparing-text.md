# Comparing Text

Deciding whether two pieces of text are the same is harder than it looks, and it
gets harder the more of the world you admit.

## Never use ==

Established in Section 18.1.1 and repeated because it is the most common string
bug there is:

```java
String a = "hello";
String b = "hello";
String c = new String("hello");

a == b        // true  — same pooled literal
a == c        // false — different objects
a.equals(c)   // true  — same contents
```

`==` compares references. It happens to work for literals because the compiler
pools them, and fails for anything built at run time — from user input, from a
file, from concatenation.

Which means the bug **passes every test written with literals** and fails on real
data. If you write one defensive habit from this chapter into your fingers, make it
`equals`.

## The comparison family

```java
a.equals(b)                 // exact, case-sensitive
a.equalsIgnoreCase(b)       // case-insensitive
a.compareTo(b)              // ordering: negative, zero, positive
a.compareToIgnoreCase(b)
```

`compareTo` returns a number rather than a boolean, which is what sorting needs:

```java
"apple".compareTo("banana")     // -1: apple comes first
```

The value's *sign* is the meaning; the magnitude is unspecified and should not be
relied upon. Test `< 0`, `== 0`, `> 0`.

## What compareTo actually compares

Code point order — Chapter 4's numbers — not alphabetical order in any human
sense.

Consequences:

```java
"Zebra".compareTo("apple")      // negative: uppercase Z (90) before lowercase a (97)
```

All uppercase letters sort before all lowercase ones, because ASCII put them in
that order. Sorting a list of names with mixed capitalization gives `Adams`,
`Brown`, `adams`, `brown` — which is correct by the specification and wrong by any
user's expectation.

And beyond ASCII it degrades further. `"ä"` sorts after `"z"` because its code
point is higher, which is right for Swedish and wrong for German.

## Collation

The proper answer, from Chapter 4's warning that sorting is cultural:

```java
Collator c = Collator.getInstance(Locale.GERMAN);
list.sort(c);
```

A `Collator` sorts according to a locale's rules. Use one whenever the order will
be shown to a person. `compareTo` is fine for internal purposes — deduplication,
map keys, consistent-but-arbitrary ordering — and is not fine for a displayed list
of names.

Most programs get this wrong, and most of the time it does not matter enough for
anyone to complain. It is worth knowing which situation you are in.

## Normalization

Chapter 4's other warning, with teeth:

```java
"café"      // as one code point: e-acute
"café"      // as two: e + combining accent
```

These display identically and are **not equal**. `equals` compares code point
sequences, and the sequences differ.

If you compare text that came from outside — usernames, filenames, search terms —
normalize first:

```java
String a = Normalizer.normalize(input1, Normalizer.Form.NFC);
String b = Normalizer.normalize(input2, Normalizer.Form.NFC);
a.equals(b);
```

NFC prefers the single combined code point. Doing this consistently at the boundary
— when text enters your program — is much easier than remembering it at every
comparison.

## Case is cultural too

`toUpperCase()` and `toLowerCase()` take a locale, and the default is the
platform's.

The famous case is Turkish. Turkish has a dotless `ı` and a dotted `i`, and
uppercasing `i` gives `İ` rather than `I`. So:

```java
"title".toUpperCase()      // "TITLE" in English
                           // "TİTLE" in Turkish
```

A program comparing `"TITLE"` against an uppercased user input will fail on a
Turkish machine, and the failure depends on a setting nobody thought about. This
is a real class of bug with a name — the Turkish-I problem — and the defense is to
pass a locale explicitly when the comparison is internal:

```java
s.toUpperCase(Locale.ROOT)
```

`Locale.ROOT` means locale-independent, which is what you want for protocol
strings, keys, and identifiers. Use the user's locale only for text they will read.

## Searching within

```java
s.contains("ell")
s.startsWith("he")
s.endsWith("lo")
s.indexOf("l")           // 2, or -1 if absent
s.lastIndexOf("l")       // 3
s.isEmpty()              // length is zero
s.isBlank()              // empty or only whitespace
```

`indexOf` returning −1 for absent is Chapter 11's sentinel, and an acceptable one:
an index cannot be negative, so the value cannot be mistaken for a real answer.

`isBlank` is worth knowing about. Input validation usually wants it rather than
`isEmpty`, since a field containing three spaces is empty in every sense the user
cares about.

## Comparing, summarized

| you want | use |
|---|---|
| same text | `equals` |
| same ignoring case | `equalsIgnoreCase` |
| an ordering, internal | `compareTo` |
| an ordering, shown to a person | `Collator` with a locale |
| text from outside | normalize, then `equals` |
| case conversion for keys | `toUpperCase(Locale.ROOT)` |
| case conversion for display | `toUpperCase(userLocale)` |

Next: turning text into data and back.
