# Enums as Closed Sets

Some sets do not grow. There are seven days, four suits, three states a traffic
light can be in, and no amount of future requirements will produce an eighth day.

Represent those with numbers — 0 for Monday, 1 for Tuesday — and nothing stops a
variable holding 9, nothing stops you passing a day where a suit was expected since
both are `int`, and printing gives you `1` rather than `TUESDAY`. Worst of all, the
compiler cannot tell you that a `switch` forgot a case, because as far as it knows
there are four billion of them.

Every one of those problems comes from the same source: you knew the set was
closed and had no way to say so.

Here is how everybody did it before Java 1.5, and how a surprising amount of code
still does:

```java
static final int SUNNY = 0;
static final int RAINY = 1;
static final int SNOWY = 2;
```

Three names, three numbers, and every failure listed above now available to you in
your own code. `weather = 7` compiles. So does handing a weather to something
expecting a day. Printing gives you `1`.

It is Chapter 1's argument about encodings in its most miserable form: a pattern
that means whatever anybody decides it means, with nothing anywhere to hold the
agreement in place.

## The construct

```java
enum Weather { SUNNY, RAINY, SNOWY }
```

This declares a type with exactly three values. Not "at least"; exactly. There is
no way to make a fourth.

What comes with it:

```
SUNNY ordinal 0 -> Sunglasses
RAINY ordinal 1 -> Umbrella
SNOWY ordinal 2 -> Coat
```

- `values()` returns the constants in declaration order
- `ordinal()` gives the position, `name()` gives the identifier
- `valueOf("RAINY")` parses one, throwing if the text does not match
- `toString()` prints the name, so debugging output is readable
- the values are `Comparable`, ordered by declaration

## Why == is safe

Enum constants are singletons. `Weather.RAINY` is one object, created once when
the class loads, and there is no way to make another — the constructor is
implicitly private and `new Weather(...)` does not compile.

So reference equality is value equality, and the verified check confirms it:

```
identity: true
```

`Weather.valueOf("RAINY") == w` is true. Parsing did not create anything; it
found the existing constant.

This is Chapter 20's exception, promised there: `==` is right for enums, and it is
also faster, null-safe, and checked by the compiler — comparing two different enum
types with `==` is an error, where `equals` would silently return false.

## Exhaustive switch

Chapter 8's promise:

```java
String advice = switch (weather) {
    case SUNNY -> "Sunglasses";
    case RAINY -> "Umbrella";
    case SNOWY -> "Coat";
};
```

Verified: `switch gives Umbrella`.

No `default` clause, and none is needed — the compiler knows the set is closed and
that all three are covered. That is not a convenience. It means that **adding a
fourth constant turns every such switch into a compile error**, and you are handed
the list of places to update.

Compare with the `int` version, where adding a fourth weather compiles fine and
misbehaves at run time in whichever branch you forgot. This is the closed set
paying for itself.

Write a `default` clause and you throw that away — the compiler stops checking,
because you told it you have covered everything. Prefer no `default` on an enum
switch, and let the error find you.

## Enums with state and behavior

A Java enum is a class, and the constants are instances. So they can carry data:

```java
enum Weather {
    SUNNY("Sunglasses"), RAINY("Umbrella"), SNOWY("Coat");

    private final String advice;

    Weather(String advice) { this.advice = advice; }

    String advice() { return advice; }
}
```

Each constant calls the constructor with its own argument. `RAINY.advice()`
returns `"Umbrella"`, and the data lives with the constant rather than in a
lookup table somebody has to keep aligned.

Constants may also override methods individually, giving each its own behavior —
which is dynamic dispatch over a closed set, and a clean way to express an
operation that differs per case without a switch anywhere.

## EnumSet and EnumMap

Because the set is closed and the ordinals are small, collections of enums can be
represented as bit fields:

```java
EnumSet.of(Weather.SUNNY, Weather.SNOWY)     // [SUNNY, SNOWY]
```

An `EnumSet` over a type with 64 or fewer constants is a single `long`. Membership
is a bit test, union is an OR, and the whole set fits in a register. `EnumMap` is
likewise an array indexed by ordinal.

Both are dramatically faster than `HashSet` and `HashMap` for this case, and they
iterate in declaration order rather than hash order. Use them whenever the
elements are enum constants; it costs nothing.

## Where enums are wrong

When the set is not actually closed. Currencies, countries, product categories —
these look enumerable and are not, and an enum makes each addition a
recompilation. If the values come from a database or a configuration file, they
are data, not a type.

The test: could this list change without the program's logic changing? If yes, it
is data.

Next: the same restriction move, applied to data instead of to a set of values.
