# Autoboxing and Its Traps

Writing conversions by hand is tedious:

```java
Integer boxed = Integer.valueOf(5);
int unboxed = boxed.intValue();
```

Since Java 5 the compiler inserts them:

```java
Integer boxed = 5;         // autoboxing
int unboxed = boxed;       // unboxing
```

Convenient, and the convenience is the problem: **the conversion is invisible, and
its consequences are not.**

## Trap one: identity

```java
Integer a = 127, b = 127;
Integer c = 128, d = 128;

a == b      // true
c == d      // false
```

Same code, different answers, depending on the value.

The explanation is the cache from the last lesson. `Integer.valueOf(127)` returns
the cached instance both times, so `a` and `b` refer to **one object** and `==` —
which compares references, per Chapter 12 — is true. 128 is outside the cache, so
two objects are created, and `==` is false.

The real lesson is not the boundary at 127. It is that **`==` on wrappers compares
identity, not value**, and the cache makes it accidentally work for small numbers,
which is worse than if it never worked at all — a bug that passes every test using
small values and fails in production on a large one.

```java
c.equals(d)      // true — compares value
```

**Never use `==` on wrapper types.** Use `equals`, or unbox one side explicitly.
Chapter 20 treats identity versus equality properly; this is your first
encounter with the distinction costing you something.

## Trap two: unboxing null

```java
Integer n = null;
int x = n;         // NullPointerException
```

The assignment compiles — the compiler inserts `n.intValue()` — and throws at run
time, because you cannot call a method on `null`.

What makes this nasty is that the line looks like an assignment between numbers.
There is no visible method call, no visible dereference. A `NullPointerException`
from a line containing no `.` is baffling until you know that unboxing happened
there.

It arrives most often from a map lookup:

```java
Map<String, Integer> counts = ...;
int n = counts.get("missing");     // get returns null; unboxing throws
```

## Trap three: overload resolution

Chapter 12 previewed this and here it is properly:

```java
List<Integer> list = new ArrayList<>(List.of(10, 20, 30));
list.remove(1);                       // [10, 30]
list.remove(Integer.valueOf(10));     // [20, 30]
```

`List` has `remove(int index)` and `remove(Object o)`. The literal `1` matches
`remove(int)` exactly, so it removes *by position*. To remove by value you must
force the other overload.

Chapter 12's rule — exact match before boxing — decides it, and the result is that
two calls which look like they differ only in spelling do entirely different
things.

## Trap four: silent cost

```java
Long sum = 0L;
for (int i = 0; i < 3_000_000; i++) {
    sum += i;
}
```

That loop allocates three million `Long` objects, because `sum += i` unboxes,
adds, and boxes the result. Measured, it is about twenty-seven times slower than
the same loop with a primitive `long`.

Nothing in the source says so. One capital letter is the entire difference, and it
is easy to write by accident — particularly when a variable's type came from a
collection and you did not think about it.

## Trap five: arithmetic on mixed types

```java
Integer a = 1;
Long b = 1L;
a.equals(b)      // false
```

`equals` on `Integer` requires the argument to be an `Integer`. A `Long` holding
the same numeric value is not equal to it, and this is correct behavior that
looks wrong.

The general lesson: **wrappers are objects with types, not numbers.** Numeric
promotion — which would make `1` and `1L` compare equal as primitives — does not
apply to method calls.

## The rules

Five traps, four rules.

**Use primitives unless you cannot.** Local variables, parameters, fields, and
loop counters should be `int`, not `Integer`. Reach for a wrapper when a
collection or generic requires one.

**Never compare wrappers with `==`.** `equals`, always.

**Be careful unboxing anything that might be null**, especially map lookups. Check
first, or use a default:

```java
Integer v = counts.get(key);
int n = (v == null) ? 0 : v;
```

**Watch types in hot loops.** A `Long` accumulator in a loop over millions is a
real and invisible cost.

## The wider point

It is worth stepping back, because this lesson is a case study in something more
general.

Autoboxing was added to remove tedium, and it succeeded. It also **hid a
distinction that still matters** — between a value and an object holding a value —
and every trap above is that hidden distinction becoming visible at an
inconvenient moment.

That is the failure mode of a leaky abstraction, in Chapter 11's sense. An
abstraction that hides something you never need to think about is a gain. One that
hides something you occasionally must think about is worse than none, because it
removes the reminder while leaving the consequences.

Whether autoboxing was worth it is genuinely debated. What is not debatable is
that using it well requires knowing exactly what it does — which is the opposite
of what a convenience feature is supposed to demand.

Next: the value that is not a value.
