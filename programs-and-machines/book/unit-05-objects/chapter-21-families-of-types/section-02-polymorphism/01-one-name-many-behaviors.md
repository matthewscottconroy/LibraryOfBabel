# One Name, Many Behaviors

*Polymorphism* is Greek for "many forms". In Java it means: a single call site
runs different code depending on the actual type of the receiver.

```java
Shape[] shapes = { new Circle(1), new Square(2) };
for (Shape s : shapes) {
    System.out.println(s.area());
}
```

One call, `s.area()`. Two different methods run. The loop does not know or ask
which — and that is the point, because it means the loop keeps working when a
third shape is added.

Verified output:

```
Circle         area 3.1416
Square         area 4.0000
```

## What it replaces

Without polymorphism you would branch:

```java
double area(Shape s) {
    if (s instanceof Circle c)      return Math.PI * c.r * c.r;
    else if (s instanceof Square q) return q.side * q.side;
    else if (s instanceof Triangle t) ...
    throw new IllegalArgumentException("unknown shape");
}
```

Compare the two. The polymorphic version puts each shape's area next to that
shape's data; the branching version collects all the areas in one place, away from
the data they use.

The difference shows when you add a type. Polymorphic: write the new class, done —
the loop is untouched. Branching: find every such chain in the program and add a
case, and hope you found them all. The compiler will not tell you that you missed
one; you will find out at run time, from the `throw` at the bottom.

This is why long `instanceof` chains and `switch`-on-type are treated as a design
smell. Not always — sometimes the set of types genuinely is closed, and Java 21's
sealed types plus pattern matching handle that case well, with the compiler
checking exhaustiveness. But the default reading of such a chain is: this should
have been a method.

## The general shape of the idea

Polymorphic code is written against **what a thing can do**, not what it is.

```java
static double totalArea(List<Shape> shapes) {
    double sum = 0;
    for (Shape s : shapes) sum += s.area();
    return sum;
}
```

`totalArea` needs one guarantee — that each element responds to `area()` — and it
is indifferent to everything else. It will work on shapes that were written after
it, by people who never read it. That is the abstraction Chapter 19 was
maintaining a boundary for, now paying out.

## Three kinds

The word covers three distinct mechanisms, and it is worth separating them.

**Subtype polymorphism** — this chapter. A `Circle` may be used where a `Shape` is
expected, and the dispatch chooses.

**Parametric polymorphism** — Chapter 17's generics. `List<T>` works for every `T`
by not caring what `T` is.

**Ad-hoc polymorphism** — overloading. Several methods share a name and the
compiler picks. Only a resemblance, really; nothing is deferred to run time.

The first two are the substantial ones. Both let you write code once and use it for
many types, and they compose: `List<Shape>` is both at once.

## The cost of the arrangement

Two things you give up, and it is fair to name them.

**Indirection.** Reading `s.area()`, you cannot tell what runs. You must know the
possible types. In a hierarchy of two that is nothing; in a hierarchy of twenty
spread over four levels it is a real cost, and it is the argument against deep
inheritance from the reader's side.

**A commitment.** Once callers depend on `Shape`, the method set is effectively
fixed. Adding a method to `Shape` breaks every subclass that does not implement it
— including ones you do not control. Chapter 22 shows the escape hatch Java 8 added
for exactly this problem.

Next: how the machine actually finds the method.
