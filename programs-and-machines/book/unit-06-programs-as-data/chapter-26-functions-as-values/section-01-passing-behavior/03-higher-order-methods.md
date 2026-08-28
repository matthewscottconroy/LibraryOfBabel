# Higher-Order Methods

`Collections.sort` has taken a comparison as an argument since 1998. You supply how
to compare; it supplies a good sorting algorithm somebody wrote once and got right.

That division is sixteen years older than lambdas, which is the tell: the *idea*
never needed new syntax. What the syntax changed is how small a piece of behavior
it is worth passing — and once that threshold drops, the technique shows up in
places it never used to.

A **higher-order** method is one that takes a function as an argument, returns one
as a result, or both. `mapArray` from Section 26.1.1 was the first kind.

## Taking behavior

The pattern generalizes past `mapArray`. Anywhere you have written nearly the
same method twice, with one expression differing, a functional parameter collapses
them.

You have already been using this without noticing. `Collections.sort` takes a
`Comparator`, which is a functional interface, and has since 1998:

```java
list.sort((a, b) -> a.length() - b.length());
list.sort(Comparator.comparing(Person::name));
```

The sorting algorithm is fixed — someone implemented a good one once. The
comparison is yours. That division is the whole idea, and the reason it predates
lambdas by sixteen years is that the *concept* never needed them; only the
notation did.

The same shape appears in `removeIf`, `forEach`, `computeIfAbsent`, and `merge` —
which is where Chapter 17's `Integer::sum` came from.

## The templates it replaces

Section 22.1.2's template method solved a related problem: a parent implementing
an algorithm and calling abstract steps the subclass fills. Compare:

```java
abstract class Processor {           // template method
    void run(int[] a) { ...; step(a[i]); ... }
    abstract int step(int x);
}

static void run(int[] a, IntOp step) { ...; step.apply(a[i]); ... }
```

Same structure, and the second requires no class, no inheritance, and no
commitment at compile time. This is Section 23.1.2's argument — composition over
inheritance, run-time choice over compile-time — with the composed part shrunk
from an object to a function.

The template method still earns its place when the parent needs *several* holes
filled by one coherent implementer, or when there is shared state. For one hole,
a functional parameter is smaller in every way.

## Returning behavior

The other direction, and the one that makes closures pay:

```java
static IntOp adder(int n) {
    return x -> x + n;
}
```

`adder` is a method that returns a function. The returned lambda captures `n`.

Verified:

```java
IntOp add5   = adder(5);
IntOp add100 = adder(100);
add5.apply(1)     ->  6
add100.apply(1)   ->  101
```

Two functions, from one method, differing in a value captured at creation.

Look at what `adder(5)` produced: an object holding the number 5 and some code
that uses it. That is a `Procedure` plus an environment — Chapter 25's closure,
built by Java. `n` lived in `adder`'s stack frame, which was destroyed when
`adder` returned, and the lambda still has it, because the value was copied in.

This is also, exactly, an object with one field and one method. A closure and a
one-method object are the same thing under two notations, which is the Lambda
Papers' claim from Chapter 25's reading list, visible in six lines.

## Composing behavior

Functions can be combined into functions:

```java
static IntOp compose(IntOp f, IntOp g) {
    return x -> f.apply(g.apply(x));
}
```

Takes two, returns a third. Verified: composing *double* with *add 3*, applied to
10, gives 26 — the inner one runs first.

The standard interfaces have this built in:

```java
Predicate<String> shortAndUpper = isShort.and(isUpper);
Function<String,Integer> f = String::trim.andThen(String::length);
Predicate<String> notEmpty = isEmpty.negate();
```

`and`, `or`, `negate`, `andThen`, `compose` — all `default` methods on the
interfaces, which is Section 22.1.1's feature doing real work. They exist because
composition is the natural operation on functions, and having it available turns
a collection of small predicates into a vocabulary.

## Where this leads

Two ideas worth naming, because they explain what the style is for.

**A pure function** — one that reads only its arguments and does nothing but
return a value — can be moved, reused, cached, tested in isolation, and run on
several threads without a lock. Chapter 11 said a method should either do
something or compute something, not both. This is what the second half buys, and
Unit VII will show it mattering.

**Referential transparency** is the property that a call can be replaced by its
result without changing the program's meaning. A pure function has it; a function
that prints, or reads a field, or increments a counter does not. It is the
property that makes an optimizer's job possible, and it is why the parallel
counter of Section 26.1.2 broke — the lambda was not pure, and the stream assumed
it was.

Neither idea requires lambdas. You could have written pure methods since Chapter
11, and should have been. What lambdas add is that pure functions become small
enough to pass around, at which point the discipline starts paying for itself
rather than being a rule you follow.

Next: three operations that between them replace most loops.
