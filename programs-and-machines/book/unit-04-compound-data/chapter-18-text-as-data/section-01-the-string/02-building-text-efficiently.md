# Building Text Efficiently

```java
String result = "";
for (int i = 0; i < 40_000; i++) {
    result += "a";
}
```

That took **79 ms** on the machine used for this book. The equivalent using
`StringBuilder` took **1 ms** — about eighty times faster.

The reason is immutability, and the arithmetic is Chapter 17's in reverse.

## Why concatenation in a loop is quadratic

`result += "a"` cannot modify `result`, so it must:

1. allocate a new string of length *n* + 1
2. copy all *n* existing characters into it
3. copy the new character
4. make `result` refer to the new string
5. leave the old one as garbage

Iteration *i* copies *i* characters. Over *n* iterations that is
1 + 2 + 3 + … + *n*, which is about $n^{2}/2$.

For 40,000 characters that is 800 million character copies, plus 40,000
allocations, plus 40,000 strings for the garbage collector.

This is the same shape as Chapter 17's grow-by-one analysis. The remedy is the same
too.

## StringBuilder

```java
StringBuilder sb = new StringBuilder();
for (int i = 0; i < 40_000; i++) {
    sb.append("a");
}
String result = sb.toString();
```

A `StringBuilder` is a **mutable** sequence of characters, backed by an array that
grows by doubling — Chapter 17's `ArrayList`, specialized for characters.

So `append` is amortized constant time, the whole loop is linear, and one string is
created at the end when you call `toString`.

Eighty times faster here, and the ratio grows with the length: the quadratic
version degrades and the linear one does not.

## The operations

```java
StringBuilder sb = new StringBuilder();
sb.append("hello");
sb.append(' ');
sb.append(42);              // any type
sb.insert(0, ">> ");
sb.reverse();
sb.setLength(0);            // reuse without reallocating
sb.length();
String s = sb.toString();
```

`append` returns the builder, so calls chain:

```java
String s = new StringBuilder()
        .append("x = ").append(x)
        .append(", y = ").append(y)
        .toString();
```

## When not to bother

The compiler is cleverer than the rule suggests, and using `StringBuilder`
everywhere is its own mistake.

**A single concatenation is fine.** `"Hello, " + name` compiles to a
`StringBuilder` automatically — `javac` does the transformation for you.

**A fixed number of concatenations is fine.** `a + b + c + d` becomes one builder
and one `toString`.

**Only loops matter.** The compiler cannot hoist the builder out of a loop, because
each iteration's `+=` is a separate expression. That is exactly why the loop case
is the one that hurts.

So the rule is narrow and worth stating precisely: **use `StringBuilder` when you
build a string across multiple statements, especially in a loop. Otherwise use
`+`, which is clearer and costs nothing.**

## The alternatives

For joining a collection, neither is needed:

```java
String.join("-", List.of("a", "b", "c"));      // "a-b-c"
```

And for anything with structure, formatting is clearer than concatenation:

```java
String.format("%s scored %d (%.1f%%)", name, score, percent);
```

Section 18.2.3 covers formatting. The general preference: **say what the result
should look like, rather than assembling it piece by piece.** The formatted version
survives a change of layout; the concatenated one has to be rebuilt.

## StringBuffer

You will meet `StringBuffer`, which is `StringBuilder` with every method
synchronized for thread safety — Chapter 31's subject.

It is older, it is slower, and it is almost never what you want, because a builder
is nearly always local to one method and therefore touched by one thread.
`StringBuilder` was added in Java 5 precisely to provide the unsynchronized
version.

If you see `StringBuffer` in code, it is usually habit rather than a decision.

Next: comparing.
