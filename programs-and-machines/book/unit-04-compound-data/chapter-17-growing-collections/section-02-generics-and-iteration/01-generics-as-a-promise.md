# Generics as a Promise

```java
List<String> names = new ArrayList<>();
```

The `<String>` is a **type parameter**, and it says: this list holds strings.

## What it buys

The compiler enforces it, which gives two things.

**Wrong things cannot go in:**

```java
names.add(42);      // error: incompatible types
```

**Right things come out without a cast:**

```java
String first = names.get(0);      // no cast needed
```

Before generics arrived in Java 5, collections held `Object` and every read
required a cast:

```java
List names = new ArrayList();
names.add("Ada");
String first = (String) names.get(0);      // the old way
```

That cast is an assertion — *trust me, this is a String* — checked at run time. Get
it wrong and you get `ClassCastException`, at the point of the cast, which may be
far from where the wrong thing was added.

Generics move the check to compile time. Chapter 5's principle: an error caught by
`javac` costs seconds, and the same error at run time costs more.

## The declaration

```java
Map<String, Integer> counts = new HashMap<>();
```

Two parameters: key type and value type. Read as *a map from String to Integer*.

Nesting is legal and gets ugly quickly:

```java
Map<String, List<Integer>> scoresByStudent = new HashMap<>();
```

*A map from String to lists of Integer.* When declarations get worse than this, it
is usually a sign that a named type would help — a `Student` class holding its own
scores, which is Unit V.

## Reference types only

```java
List<int> numbers;         // error
List<Integer> numbers;     // fine
```

Chapter 16's split again. A type parameter must be a reference type, so primitives
must be wrapped, and everything about boxing applies.

The reason is the next section's subject and it is worth knowing that this is a
Java-specific limitation rather than a fact about generics — C# and Rust do not
have it.

## Erasure

Java's generics are checked at compile time and then **discarded**. At run time,
`List<String>` and `List<Integer>` are the same class: `List`. This is called
**type erasure**.

Chapter 27 covers it properly. Three consequences are worth having now, because
they cause confusing errors.

**You cannot ask what a generic type is at run time:**

```java
if (list instanceof List<String>)      // error: cannot check
```

The information is gone. `list instanceof List` is allowed and tells you less.

**You cannot create an array of a generic type:**

```java
T[] items = new T[10];      // error
```

which is why generic collections store `Object[]` internally and cast on the way
out.

**Overloads cannot differ only by type parameter:**

```java
void f(List<String> x)
void f(List<Integer> x)      // error: same erasure
```

Both erase to `f(List)`.

Why did Java do this? Compatibility. Generics arrived in Java 5, and code written
before had to keep working, so generic types had to be the same classes at run
time as the raw ones. It was a large decision with permanent consequences, and
the alternative — used by C# — required breaking compatibility, which Java was
unwilling to do.

## Raw types

You can still write the pre-generics form:

```java
List names = new ArrayList();      // raw type
```

It compiles, with a warning. **Do not.** You lose every check, and a raw list can
have anything put in it, which then fails as a `ClassCastException` somewhere else
entirely.

You will meet raw types in old code. Adding the type parameter is nearly always
the right fix.

## Writing your own

You can parameterize your own types, and Unit V will:

```java
public class Box<T> {
    private T item;
    public void put(T item) { this.item = item; }
    public T get() { return item; }
}

Box<String> b = new Box<>();
b.put("hello");
String s = b.get();      // no cast
```

`T` is a placeholder for a type supplied by the user of the class. By convention
`T` is a type, `E` an element, `K` a key, `V` a value.

Generic methods are also possible:

```java
static <T> T firstOrNull(List<T> list) {
    return list.isEmpty() ? null : list.get(0);
}
```

The `<T>` before the return type declares the parameter. This method works for any
element type and returns the right one — which without generics would return
`Object` and require a cast at every call.

## The promise, restated

Chapter 7 said a type is a promise the compiler enforces. A generic type is the
same promise about *contents*:

> This collection contains only Strings, and I want you to hold me to it.

And, like every promise in this book, its value is that you can then stop checking.
A `List<String>` needs no defensive `instanceof`, no cast, no wondering. That is
what removing a category of doubt is worth.

Next: walking one safely.
