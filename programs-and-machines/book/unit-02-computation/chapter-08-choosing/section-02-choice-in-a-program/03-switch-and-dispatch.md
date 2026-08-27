# switch and Dispatch

When the choice is among many values of one thing, a chain of `else if` gets
repetitive:

```java
if (day == 1)      name = "Monday";
else if (day == 2) name = "Tuesday";
else if (day == 3) name = "Wednesday";
// ...
```

Every branch compares the same variable against a different constant. `switch`
says that once.

## The modern form

```java
String name = switch (day) {
    case 1 -> "Monday";
    case 2 -> "Tuesday";
    case 3 -> "Wednesday";
    case 6, 7 -> "Weekend";
    default -> "Unknown";
};
```

Points worth noting.

**It is an expression.** It produces a value, assigned to `name`. Older Java only
had a `switch` *statement*; the expression form arrived in Java 14 and is better
in almost every way.

**Several labels can share an arm** — `case 6, 7 ->`.

**`default` catches everything else.** For a `switch` expression it is required
unless the compiler can prove the cases are exhaustive, because an expression
must produce a value in every case. That requirement is the compiler protecting
you from a gap you did not notice.

**No fall-through.** Each arm is independent.

## The old form, and why it bites

You will meet this in existing code:

```java
switch (day) {
    case 1:
        name = "Monday";
        break;
    case 2:
        name = "Tuesday";
        break;
    default:
        name = "Unknown";
}
```

The colons and the `break` statements are the older syntax, and the `break` is
not decoration. Without it, execution **falls through** into the next case and
keeps going:

```java
switch (day) {
    case 1:
        name = "Monday";     // no break
    case 2:
        name = "Tuesday";
        break;
}
```

With `day` equal to 1, this sets `name` to `"Monday"` and then to `"Tuesday"`.
The final value is `"Tuesday"`.

Forgetting `break` is one of the classic bugs in C-family languages, and it is
almost always a mistake rather than an intention. Occasionally fall-through is
what you want — grouping several cases that share code — but the modern
comma-separated form expresses that better and without the hazard.

The lesson is a general one: **a default behavior that is usually wrong is a
design error.** Fall-through is the default and is rarely wanted, so the language
made the common case require extra work and the rare case require none. The
arrow form fixes it, which is why you should prefer it.

## What can be switched on

`byte`, `short`, `char`, `int`, their wrapper types, `String`, and `enum` types.
Not `long`, not `double`, not `boolean` — for `boolean` an `if` is clearly better,
and for the others the reason is historical and to do with how `switch` is
implemented.

Switching on an `enum` is where it is at its best, and Chapter 22 returns to it:

```java
String advice = switch (weather) {
    case SUNNY  -> "Sunglasses";
    case RAINY  -> "Umbrella";
    case SNOWY  -> "Coat";
};
```

No `default` is needed, because the compiler knows every value of the enum and
can check you covered them. And if someone later adds `WINDY` to the enum, **this
code stops compiling** until the new case is handled — which is exactly the
failure you want, at exactly the time you want it.

That is a genuinely excellent property, and it is the argument for enums over
integer constants in a sentence.

## Underneath

Briefly, because it explains a performance claim you may hear.

A chain of `else if` does comparisons in order — testing *n* values costs up to
*n* comparisons. `switch` on integers can compile to a **jump table**: the value
is used as an index into a table of addresses, and the machine jumps directly to
the right arm in constant time regardless of how many cases there are.

The JVM has two instructions for this: `tableswitch` for dense consecutive values,
and `lookupswitch` for sparse ones.

This is Chapter 6's "consult the state to choose a continuation" implemented as
literally as possible — the state *is* the index, and choosing is arithmetic on
the program counter.

Do not choose `switch` for speed. Choose it because "one value, many cases" is
what you mean, and the compiler's exhaustiveness checking is worth more than the
jump table.

## Closing the chapter

We began with Boole in 1854 and Shannon in 1937, and the observation that logic
and switching are the same algebra — which is why circuits can be designed rather
than discovered.

We built AND, OR, NOT and XOR as tables, found that NAND alone generates all of
them, and then derived a binary adder from two gates by noticing that the sum
column is XOR and the carry column is AND. Chapter 2's arithmetic, constructed.

Then the algebra that lets conditions be rewritten safely, De Morgan's laws chief
among them, and Java's own conditionals — including the two ANDs, and why the
short-circuiting one is really control flow wearing a logical operator's clothes.

Chapter 6 said a program is states and transitions. We now have transitions that
change state and transitions that consult it. One thing is missing before the
model is complete: a transition that goes *backwards*, returning the machine to
somewhere it has been, with something changed.

That is a loop, and it is the next chapter — and the place where this book asks
you to prove something for the first time.
