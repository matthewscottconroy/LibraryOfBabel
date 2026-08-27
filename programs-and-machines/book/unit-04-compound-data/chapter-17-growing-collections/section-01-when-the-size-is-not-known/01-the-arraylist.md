# The ArrayList

```java
import java.util.ArrayList;
import java.util.List;

List<String> names = new ArrayList<>();
names.add("Ada");
names.add("Grace");
System.out.println(names);          // [Ada, Grace]
System.out.println(names.size());   // 2
```

No size was declared. Elements were added and it grew.

## Reading the declaration

```java
List<String> names = new ArrayList<>();
```

Four things are happening, and separating them is worth doing once.

**`List<String>`** is the declared type of the variable: *a list of strings*. It
is an **interface** — Chapter 16's abstract data type — naming the operations
without saying how they are done. Chapter 22 covers interfaces properly.

**`new ArrayList<>()`** creates the actual object: a particular implementation,
which stores elements in an array.

**The declared type is the interface, not the implementation.** This is deliberate
and it is the single most useful convention in the chapter. Declaring
`List<String>` rather than `ArrayList<String>` means you can change the
implementation on one line and nothing else needs touching — Section 16.1.1's
argument, made available in one keystroke.

**`<>`** is the diamond, and it means *the same type parameter as on the left*.
Writing `new ArrayList<String>()` is equivalent and longer.

## The operations

```java
List<String> names = new ArrayList<>();

names.add("Ada");                // append
names.add(0, "Grace");           // insert at position
names.get(0);                    // read by position
names.set(0, "Alan");            // replace at position
names.remove(0);                 // remove by position
names.remove("Ada");             // remove by value
names.size();                    // how many
names.isEmpty();
names.contains("Ada");
names.indexOf("Ada");            // position, or -1
names.clear();
```

Two notes for people arriving from arrays.

**`size()` is a method**, where an array has `length` as a field and a `String` has
`length()` as a method. Chapter 15 flagged this; here is the third spelling.

**Indices are still zero-based and half-open**, so valid positions run from 0 to
`size() - 1` and everything from Chapter 9 applies unchanged.

## Walking it

```java
for (String name : names) {
    System.out.println(name);
}
```

The enhanced `for` works on any collection, and for the reason Chapter 9 gave —
no index, no index error — it should be your default.

When you need positions:

```java
for (int i = 0; i < names.size(); i++) {
    System.out.println(i + ": " + names.get(i));
}
```

## Collections hold objects

The consequence of Chapter 16's split:

```java
List<Integer> numbers = new ArrayList<>();   // Integer, not int
numbers.add(5);                              // autoboxed
int first = numbers.get(0);                  // unboxed
```

`List<int>` does not compile. Everything Section 16.2.2 warned about is now
routine — in particular:

```java
List<Integer> list = new ArrayList<>(List.of(10, 20, 30));
list.remove(1);                     // removes position 1 → [10, 30]
list.remove(Integer.valueOf(10));   // removes the value 10 → [20, 30]
```

and

```java
Integer x = list.get(5);      // may be null if you are not careful with maps
int y = x;                    // NullPointerException if it is
```

For large collections of numbers the boxing cost is real, and specialized
libraries exist for it. For ordinary work it does not matter and clarity wins.

## Creating with contents

```java
List<String> fixed = List.of("Ada", "Grace", "Alan");
```

Compact, and note what it makes: an **immutable** list. Calling `add` on it throws
`UnsupportedOperationException`. That is frequently what you want — Chapter 20
argues that unchangeable things are easier to reason about — and it is a trap when
you wanted a working list.

To get a mutable one from a fixed set of values:

```java
List<String> names = new ArrayList<>(List.of("Ada", "Grace"));
```

## Arrays or ArrayList?

**`ArrayList` unless you have a reason.** It grows, it has useful methods, it
prints readably, and it works with the rest of the library.

**An array when** you know the size and it will not change; when you are holding
primitives and the boxing cost matters; or when an interface you must satisfy
demands one.

The default is worth stating plainly because beginners frequently reach for arrays
out of familiarity and then hand-write growth logic, which is the next lesson's
subject and which is already written.

Next: how it grows.
