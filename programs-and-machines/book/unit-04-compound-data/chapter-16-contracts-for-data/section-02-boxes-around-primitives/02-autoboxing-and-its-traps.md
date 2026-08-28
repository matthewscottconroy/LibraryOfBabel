# Autoboxing and Its Traps

Writing the conversions out by hand is tedious, and nobody enjoyed it:

```java
Integer boxed = Integer.valueOf(5);
int unboxed = boxed.intValue();
```

So since Java 5 the compiler writes them for you:

```java
Integer boxed = 5;         // autoboxing
int unboxed = boxed;       // unboxing
```

Which is a real improvement, and also the source of everything that follows.
**The conversion became invisible. Its consequences did not.**

There are five places where this bites. They are worth meeting one at a time,
because each of them looks like a different bug and all of them are the same bug.

## The first one: two numbers that are not the same number

Read this and decide what each comparison gives you. Both lines look identical
apart from the value.

```java
Integer a = 127, b = 127;
Integer c = 128, d = 128;

a == b      // ?
c == d      // ?
```

`true`, then `false`.

Same code. Same shape. Different answer, and the only thing that changed is that
the number got bigger.

The mechanism is the cache from the last lesson. `Integer.valueOf(127)` hands back
a cached instance both times, so `a` and `b` are two names for **one object**, and
`==` compares references, so it says true. But 128 falls outside the cached range,
so two separate objects get made, and `==` looks at two different references and
says false.

Now, the temptation here is to remember the number 127 and feel that you have
learned something. Resist it. The boundary is not the lesson.

The lesson is that **`==` on wrapper types has been comparing identity all along**,
and the cache is what stopped you noticing. That is worse than if it had never
worked, because a mechanism that fails only above a certain size will pass every
test you write with small numbers and then fail in production on a real one.

```java
c.equals(d)      // true — this one compares value
```

**So: never use `==` on a wrapper type.** Use `equals`, or unbox one side yourself
and compare primitives. Chapter 20 gives identity and equality the full treatment
they deserve. This is the first time the distinction costs you anything.

## The second one: a NullPointerException with nothing to dereference

```java
Integer n = null;
int x = n;         // NullPointerException
```

Look at that second line and try to find the method call.

There isn't one. There is no dot, no arrow, nothing you could point at and say
*that is where it went wrong*. It is an assignment between two things that both
look like numbers, and it throws a `NullPointerException` at run time.

It compiles because the compiler quietly inserts `n.intValue()`. It throws because
you cannot call a method on `null`. Both halves are reasonable and the combination
is baffling the first time you meet it in a stack trace.

It reaches you most often through a map:

```java
Map<String, Integer> counts = ...;
int n = counts.get("missing");     // get returns null; unboxing throws
```

## The third one: two calls that differ by spelling and do different things

```java
List<Integer> list = new ArrayList<>(List.of(10, 20, 30));
list.remove(1);                       // ?
list.remove(Integer.valueOf(10));     // ?
```

Cover the answers and commit to a guess for each.

`[10, 30]` and `[20, 30]`.

The first call removed the element *at position 1*. The second removed *the element
equal to 10*, which happened to be at position 0.

`List` offers both `remove(int index)` and `remove(Object o)`. The literal `1` is
an exact match for `remove(int)`, and Chapter 12's rule is that an exact match wins
before any boxing is considered. So the first call is a positional removal, and the
only way to get a removal by value is to hand it something that is already an
object.

Two calls, one visible difference in spelling, entirely different meanings.

## The fourth one: a cost with no symptom

```java
Long sum = 0L;
for (int i = 0; i < 3_000_000; i++) {
    sum += i;
}
```

Nothing in that loop looks expensive. Before reading on, guess how much slower it
runs than the identical loop with a primitive `long`.

About twenty-seven times.

Every single iteration of `sum += i` unboxes `sum`, does the addition, and boxes
the result back up. Three million `Long` objects get allocated, used once, and
thrown away.

The entire difference between the fast version and the slow one is a single capital
letter. There is nothing else — no extra call, no visible allocation, no hint of
any kind in the source text. And it is genuinely easy to write by accident,
especially when the variable's type arrived from a collection somewhere and you
never consciously chose it.

## The fifth one: correct behavior that looks like a bug

```java
Integer a = 1;
Long b = 1L;
a.equals(b)      // false
```

One equals one, and the answer is false.

`Integer.equals` requires its argument to be an `Integer`. A `Long` holding the
same numeric value is a different type, so it is not equal, and this is the
specification working exactly as designed.

The general principle underneath it: **wrappers are objects that have types, not
numbers.** The numeric promotion that would happily compare `1` and `1L` as equal
primitives is a rule about arithmetic, and it has no jurisdiction over a method
call.

## Four rules that cover all five

**Use primitives unless something stops you.** Local variables, parameters, fields,
loop counters — `int`, not `Integer`. Reach for a wrapper when a collection or a
generic type leaves you no choice.

**Never compare wrappers with `==`.** `equals`, every time, without thinking about
it.

**Treat anything that might be null with suspicion before unboxing it**, and treat
map lookups as the prime suspect:

```java
Integer v = counts.get(key);
int n = (v == null) ? 0 : v;
```

**Watch the types in a hot loop.** A `Long` accumulator running over millions of
iterations is a real cost, and an invisible one.

## What this was really about

Step back from the five traps, because they are a case study in something with a
much wider reach.

Autoboxing was added to remove tedium, and it removed the tedium. That part
worked. What it also did was **hide a distinction that had not stopped mattering**
— the difference between a value and an object that holds a value. Every trap above
is that hidden distinction surfacing at an awkward moment.

Which is the exact failure mode Chapter 11 warned about. An abstraction that hides
something you genuinely never need to think about is a gain, free of charge. An
abstraction that hides something you must *occasionally* think about is worse than
no abstraction at all, because it takes away the reminder and leaves the
consequences sitting there.

Whether autoboxing was worth the trade is still argued about by reasonable people.
What nobody argues about is that using it safely requires knowing precisely what it
does — which is the opposite of what a convenience is supposed to ask of you.

Next: the value that is not a value.
