# Functional Interfaces

Here is a small frustration you have probably already felt.

You write a method that squares every element of an array. Then you want one that
adds ten to every element, so you copy it and change one expression. Then doubling.
Then negating. Four methods, six lines each, and five of the six lines are
identical every time.

Chapter 11 taught you to extract a repeated *value* into a parameter. The obvious
wish is to do the same with the repeated *expression* — but you cannot pass an
expression, because parameters hold values.

Unless the expression can be made into one.

Start with the problem. Here is a method that squares every element of an array:

```java
static int[] squareAll(int[] a) {
    int[] out = new int[a.length];
    for (int i = 0; i < a.length; i++) out[i] = a[i] * a[i];
    return out;
}
```

And here is one that adds ten to every element. And one that doubles. And one that
negates. Each is six lines, five of which are identical.

The varying part is `a[i] * a[i]` — one expression. Chapter 11 taught you to
extract a repeated *value* into a parameter, and the natural wish is to do the
same with a repeated *expression*. But you cannot pass an expression; parameters
hold values.

Unless the expression is wrapped in something that is a value.

## An interface with one method

```java
@FunctionalInterface
interface IntOp {
    int apply(int x);
}
```

An ordinary interface, in the sense of Chapter 22. One abstract method. It has a
name for that fact — a **functional interface** — and the annotation asserts it,
so the compiler rejects the interface if a second abstract method is added. Like
`@Override`, it costs a line and prevents a class of accident.

Now the method can take one:

```java
static int[] mapArray(int[] a, IntOp f) {
    int[] out = new int[a.length];
    for (int i = 0; i < a.length; i++) out[i] = f.apply(a[i]);
    return out;
}
```

One method, and all four of the earlier ones are calls to it. Verified:

```
mapArray({1,2,3,4}, square)  ->  [1, 4, 9, 16]
mapArray({1,2,3,4}, plus10)  ->  [11, 12, 13, 14]
```

The loop is written once. What differs per call arrives as an argument.

## Supplying one

Before Java 8, the only way was an anonymous class:

```java
IntOp square = new IntOp() {
    public int apply(int x) { return x * x; }
};
```

Five lines to say `x * x`. Chapter 22 introduced this form and promised a shorter
one. Here it is:

```java
IntOp square = x -> x * x;
```

Verified: both produce 49 for input 7. They are equivalent in behavior, and the
second is what people actually write.

That is the entire relationship between lambdas and interfaces. **A lambda is an
implementation of a functional interface, written in a shorter notation.** It is
not a new kind of value, not a function type, not a closure object in the sense
some languages have. It is an `IntOp`.

You can check this. `square.getClass()` returns something, `square` can be passed
where an `IntOp` is expected, and `square.apply(7)` is an ordinary interface
method call — `invokeinterface`, from Section 22.1.1.

## Why Java did it this way

The alternative was to add function types to the language — a type like
`(int) -> int` standing on its own, as Kotlin and Scala have.

Java chose not to, and the reason is compatibility. Every existing library that
took a callback took an interface: `Runnable`, `Comparator`, `ActionListener`,
`Callable`. If lambdas had produced a new kind of value, none of those would have
accepted one, and the entire standard library would have needed a parallel set of
methods.

Instead, a lambda is *whatever functional interface the context expects*:

```java
Runnable   r = () -> System.out.println("hi");
Comparator<String> c = (a, b) -> a.length() - b.length();
IntOp      f = x -> x * x;
```

Three lambdas, three different types, decided by the variable's declared type.
Which means `Comparator` — written in 1998, long before lambdas existed — works
with the new syntax without a line changing. Twenty years of libraries became
lambda-friendly at once.

The cost is that a lambda has no type of its own. You cannot write `var f = x ->
x * x;` — the compiler has no context to infer from, and reports so. That is a
real irritation and it is the price of the compatibility.

## The standard interfaces

You will rarely declare your own. `java.util.function` supplies about forty, and
six cover nearly everything:

| interface | method | means |
|---|---|---|
| `Function<T,R>` | `R apply(T)` | takes a T, gives an R |
| `Predicate<T>` | `boolean test(T)` | a yes-or-no question about a T |
| `Consumer<T>` | `void accept(T)` | does something with a T, returns nothing |
| `Supplier<T>` | `T get()` | produces a T from nothing |
| `UnaryOperator<T>` | `T apply(T)` | a `Function` from T to T |
| `BinaryOperator<T>` | `T apply(T,T)` | combines two Ts into one |

Verified:

```java
Predicate<String> isLong = s -> s.length() > 4;
isLong.test("hi")     ->  false
isLong.test("hello")  ->  true
```

Learning these six names is most of reading modern Java. The other thirty-odd are
primitive specializations — `IntPredicate`, `ToIntFunction`, `IntBinaryOperator` —
which exist because `Predicate<Integer>` would box every value, and Section 26.2.3
measures what that costs.

That the library needs forty interfaces to avoid boxing is a genuine wart. It is
Chapter 16's primitive/object split, showing up thirty years later as an API design
problem, and it is the strongest single argument for the value-types work that has
been in progress for a decade.

Next: the syntax, and what a lambda can see.
