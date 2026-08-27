# The Object Class

Every class in Java extends something. If you do not say what, it extends
`Object`.

```java
class Shape { }
class Shape extends Object { }      // the same thing
```

So every object is an `Object`, every class inherits its methods, and the root of
the hierarchy is a single class. That is why `Object[]` can hold anything, why
`equals` takes an `Object` parameter, and why Chapter 20 had to *override*
`equals` rather than invent it.

## What you inherit

Eleven methods, of which five matter now:

```java
public String toString()
public boolean equals(Object o)
public int hashCode()
public final Class<?> getClass()
protected Object clone()
```

The defaults are deliberately minimal, and each one is a decision the language had
to make about a class it knows nothing about.

**`toString`** returns the class name and a hash code — `Account@1b6d3586`. Not
useful, and Chapter 19 said to override it on nearly everything.

**`equals`** compares references, so by default equality is identity. Chapter 20's
whole subject.

**`hashCode`** returns something derived from the object's address, so distinct
objects almost always differ. Consistent with the default `equals` and
inconsistent with any `equals` you write — hence the contract.

**`getClass`** reports the actual runtime class, which is how the polymorphism
demonstrations in this chapter printed `Circle` and `Square`. It is `final`, so it
cannot lie.

**`clone`** is the mechanism Chapter 20 advised avoiding.

Notice the pattern: three of the five are things you are expected to replace. The
inherited versions exist so that every object supports the operation, not because
the defaults are right.

## Why a common root

Two reasons, and the second has consequences.

**Universal operations.** Every object can be printed, compared, hashed and put in
a collection, because every object has these methods. Without a common root, a
collection could not accept arbitrary things.

**A universal type.** Before generics — Chapter 17 — collections held `Object`, and
every read needed a cast. Generics moved the check to compile time, and erasure
means that at run time a `List<String>` still holds `Object` references. So the
common root is still doing the work; it is merely hidden.

## Primitives are not Objects

Chapter 16's split, and this is where its cost shows.

`int` is not an `Object`. It has no methods, cannot be null, and cannot go in a
collection. That is why the wrapper classes exist and why autoboxing was added,
with the five traps Section 16.2.2 catalogued.

Languages designed later frequently avoid this. Kotlin and Scala present a single
hierarchy in which everything is an object, and optimize primitives underneath.
Java could not, because the split was fixed in 1995 and the libraries depend on it.

## The wildcard type

You will meet `Object` as a parameter type when a method genuinely accepts
anything:

```java
public boolean equals(Object o)
System.out.println(Object x)
```

And you should be suspicious of it elsewhere. A method taking `Object` has given
up all type checking — anything compiles, and what happens next is a cast and a
hope. Since generics, most such methods should be generic instead:

```java
static void print(Object x)          // accepts anything, knows nothing
static <T> void print(T x)           // accepts anything, keeps the type
```

## getClass and instanceof

Two ways to ask what something actually is:

```java
c instanceof Circle              // true for Circle and its subclasses
c.getClass() == Circle.class     // true only for exactly Circle
```

`instanceof` is the usual one, and since Java 16 it binds a variable in the same
step:

```java
if (o instanceof Circle circle) {
    use(circle.r);
}
```

which is the form Chapter 20's `equals` used.

Both are worth a moment's suspicion when you write them. Code that asks what type
something is, and then branches, is frequently code that should have called an
overridden method and let dispatch do the branching. That is the next section's
argument, and `instanceof` chains are the classic symptom of not having made it.

The legitimate uses are narrow: implementing `equals`, and handling a value that
genuinely arrived from outside your type system.

Next: what all of this was for.
