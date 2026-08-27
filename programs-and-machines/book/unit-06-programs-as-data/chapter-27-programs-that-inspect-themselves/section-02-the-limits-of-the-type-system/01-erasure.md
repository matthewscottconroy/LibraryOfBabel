# Erasure

Chapter 17 said a `List<String>` is a list of strings and the compiler enforces
it. That was true and it was not the whole truth.

```java
List<String> strings = new ArrayList<>();
List<Integer> ints = new ArrayList<>();
strings.getClass() == ints.getClass()
```

Verified: `true`. Both are `java.util.ArrayList`.

There is no class `ArrayList<String>` at run time. The type argument was checked
during compilation and then **erased** — removed from the bytecode entirely. What
runs is an `ArrayList` holding `Object` references, exactly as Java 1.4 had.

## What that means

The compiler does two things and then forgets:

**It checks.** `strings.add(42)` does not compile.

**It inserts casts.** `String s = strings.get(0)` compiles to a `get` returning
`Object` followed by a cast to `String`.

So the generics of Chapter 17 are a compile-time discipline over an untyped
runtime, and the guarantee is: *if all your code compiled without unchecked
warnings, the casts will never fail.*

The condition matters, and here is what happens when it is violated:

```java
List raw = strings;      // raw type — a warning, not an error
raw.add(42);
```

Verified:

```
smuggled in: [42]
read fails: class java.lang.Integer cannot be cast to class java.lang.String
```

An `Integer` is now inside a `List<String>`. The list was happy to take it,
because at run time it is a list of `Object`. The failure arrives later, at the
*read*, in the compiler-inserted cast — which is why the exception mentions a
class cast that appears nowhere in the source.

This is called **heap pollution**, and it is why unchecked warnings are worth
taking seriously. They mark exactly the places where the compile-time guarantee
has a hole.

## Why erasure

The reason is migration compatibility, and it is worth stating because it explains
a great deal.

Generics arrived in Java 5, in 2004, into an ecosystem with nine years of existing
code and compiled libraries. The requirement was that old code keep working, that
new generic code call old non-generic code, and — the hard one — that old compiled
code call new generic code.

Erasure achieves all three. `List<String>` and `List` compile to the same thing, so
a class compiled in 1999 can pass its `List` to a method written in 2024 expecting
a `List<String>`, and the bytecode is identical.

The cost is everything below.

C# made the other choice, adding **reified** generics in 2005 with runtime type
information preserved — and could do so because the CLR was five years old and the
ecosystem was small enough to break. Both decisions were right for their
circumstances, which is a more interesting conclusion than either being better.

## What erasure forbids

The restrictions follow mechanically, and knowing the cause makes them stop being
arbitrary.

**No `instanceof` with a type argument.**
```java
if (x instanceof List<String>)     // does not compile
if (x instanceof List<?>)          // fine
```
There is nothing at run time to test.

**No `new T[]` and no `new T()`.** The type argument is unknown when the code
runs, so there is nothing to allocate. Library code works around this with
`(T[]) new Object[n]` and an unchecked warning, which is why `ArrayList`'s
internals contain exactly that.

**No overloading on type arguments.**
```java
void f(List<String> xs)
void f(List<Integer> xs)     // does not compile: same erasure
```
Both erase to `f(List)`.

**No generic exceptions.** `catch` needs a runtime type test.

**No primitives as type arguments.** `List<int>` is illegal, because erasure
requires the element to be an `Object` reference. This is the single largest
consequence — it is why autoboxing exists, why `Stream<Integer>` was ten times
slower than `IntStream` in Chapter 26, and why `java.util.function` needs forty
interfaces instead of six.

Chapter 16 introduced the primitive/object split as a decision made in 1995. This
is where the bill arrives.

## What survives

Not everything is erased, which surprises people who have been told it all is.

```java
static void takesList(List<String> xs) { }
```

Verified:

```
parameter type: java.util.List<java.lang.String>
erased type   : interface java.util.List
```

`getGenericParameterTypes()` recovered `List<String>`. The information is retained
in the class file's signature attribute — as **metadata**, not as a runtime type —
for the benefit of the compiler when it reads a library, and of tools.

The rule: **generic information is retained where it is declared, and erased where
it is used.** A field's declared type, a method's signature, a class's supertype
are all recoverable. The type argument of a particular *object* is not, because
the object never had one.

Frameworks exploit the retained part heavily. Jackson knows a field is a
`List<Person>` by reading the signature attribute, which is how it deserializes
into the right element type. There is also a well-known trick — the "super type
token", subclassing an abstract generic class so the argument becomes part of a
class declaration and is therefore retained — which is why serialization APIs
sometimes ask you to write `new TypeReference<List<Person>>() {}` with the empty
braces that create a subclass.

That empty pair of braces is doing real work, and now you know what.

Next: the mechanism for attaching information the type system cannot express.
