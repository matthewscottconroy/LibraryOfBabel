# Short-Circuit Evaluation

Java has two AND operators and two OR operators:

```
&&   ||     short-circuit
&    |      full evaluation
```

They compute the same truth tables. They differ in whether the right-hand side is
evaluated at all.

## The behavior

`&&` evaluates its left operand. **If that is false, it stops** — the result must
be false regardless of the right side, so the right side is never evaluated.

`||` evaluates its left operand. **If that is true, it stops** — the result must
be true regardless.

`&` and `|` always evaluate both.

Here is the difference made visible. Suppose `loud` prints its name and returns
what it was given:

```java
boolean r = loud("A", false) && loud("B", true);
```

```
  evaluated A
  result false
```

`B` never ran. Now with `&`:

```java
boolean r = loud("A", false) | loud("B", true);
```

```
  evaluated A
  evaluated B
  result true
```

Both ran, even though the first `|` operand could not have settled it.

## Why it matters

Two reasons, and the second is the important one.

**Speed**, occasionally. If the right side is expensive and the left often
settles the question, you avoid the work. Real but usually minor.

**Guarding**, constantly. Short-circuiting lets the left operand *protect* the
right:

```java
if (s != null && s.length() > 0) {
    // ...
}
```

If `s` is null, `&&` stops, and `s.length()` is never called. Had it been called
it would have failed — you cannot ask a null reference for its length, and Java
would throw a `NullPointerException`.

Write the same thing with `&` and it breaks:

```java
if (s != null & s.length() > 0) {     // throws when s is null
```

Both sides evaluate, the guard does not guard, and the program crashes on exactly
the input the check was written to handle.

The same pattern with `||`:

```java
if (s == null || s.isEmpty()) {
    // treat as absent
}
```

If `s` is null, `||` stops with true, and `isEmpty()` is never reached.

You will write these constantly once objects arrive in Unit V, and the ordering
is not stylistic. **The check that establishes safety must come first**, because
the operator's guarantee runs left to right.

## When you want full evaluation

Rarely, and deliberately: when the right operand has a side effect you need.

```java
if (checkA() & checkB()) { }     // both run, both record their results
```

If each records a validation failure, and you want all failures reported rather
than only the first, `&` is right. This is legitimate, and it is worth a comment
saying so — a reader who knows the difference will assume `&` was a typo for `&&`
otherwise.

`&` and `|` also serve as **bitwise** operators on integers, which is a different
job entirely: `5 & 3` operates on bit patterns and gives 1. Chapter 2's material,
and the reuse of the symbol is unfortunate.

## Order matters now

Short-circuiting means `&&` is not commutative in effect, even though it is in
logic.

```java
s != null && s.length() > 0      // safe
s.length() > 0 && s != null      // throws when s is null
```

Boolean algebra says `A && B` equals `B && A`. That law holds for the *values*
and not for the *evaluation*, because in Java one of these orders can crash
before producing a value at all.

This is worth flagging because the last lesson gave you laws for rewriting
conditions, and here is a case where a legal rewrite changes behavior. The laws
apply to expressions that terminate normally. Once evaluation can fail partway,
order becomes significant.

A useful way to hold it: `&&` and `||` are not really logical operators. They are
**control flow** — closer to `if` than to AND — and they happen to produce a
boolean.

Next: choosing among many.
