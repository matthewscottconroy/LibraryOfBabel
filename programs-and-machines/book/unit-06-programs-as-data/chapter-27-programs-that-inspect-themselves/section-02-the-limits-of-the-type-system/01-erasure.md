# Erasure

Try this. It takes two lines and it will unsettle something.

```java
List<String> strings = new ArrayList<>();
List<Integer> ints = new ArrayList<>();

System.out.println(strings.getClass() == ints.getClass());
```

`true`.

Not "similar". Not "compatible". The same object. Ask either list what it is and
both answer `java.util.ArrayList`, with no mention of strings or integers
anywhere.

So whatever those angle brackets have been doing for the last ten chapters, they
did not produce two types. At run time there is no such thing as an
`ArrayList<String>`. There never was.

Chapter 17 told you that a `List<String>` is a list of strings and the compiler
enforces it. Every word of that is true. It is also not the whole story, and the
rest of the story explains a set of rules that have probably struck you as
arbitrary, plus the tenfold performance gap you measured in Chapter 26.

## What the compiler actually does

It checks. Then it forgets.

`strings.add(42)` will not compile — the check is real and it happens. But once it
has passed, the compiler strips the type argument out of the bytecode entirely.
That is the word: **erased.** What survives into the running program is a plain
`ArrayList` full of `Object` references, exactly what Java 1.4 had before generics
existed.

Where you wrote `String s = strings.get(0)`, the compiler quietly wrote `get`
returning `Object`, followed by a cast to `String`. It has been inserting those
casts on your behalf the whole time. You just never had to look at them.

So generics are a compile-time discipline laid over an untyped runtime, and the
guarantee they offer has a condition attached:

> If all your code compiled without unchecked warnings, the casts the compiler
> inserted will never fail.

Now watch what happens when somebody violates the condition.

```java
List raw = strings;      // a raw type: warning, not error
raw.add(42);
```

That compiles. And:

```
smuggled in: [42]
read fails: class java.lang.Integer cannot be cast to class java.lang.String
```

There is an `Integer` living inside your `List<String>`. The list took it without
complaint, because at run time it is a list of `Object` and an `Integer` is an
`Object`.

Look at *where* it failed. Not at the `add` — the list was perfectly happy. It
failed at the *read*, in a cast you never wrote, mentioned in an exception that
names a line of code you cannot find because it does not appear in your source.

The name for this is **heap pollution**, and it is the reason to treat unchecked
warnings as real. Each one marks a spot where the compile-time guarantee has a
hole in it, and where the failure — if it comes — will surface somewhere else
entirely.

## Why anyone would design it this way

The answer is a date.

Generics arrived in Java 5, in 2004, into an ecosystem that had been accumulating
code since 1995. Nine years of libraries, most of them shipped as compiled class
files by people who had moved on. And the requirement was not merely that old code
keep working. It was that new generic code call old non-generic code, *and* — the
brutal one — that old compiled code call new generic code, without recompilation,
because nobody had the source.

Erasure gets all three. `List<String>` and `List` compile to the same bytecode, so
a class compiled in 1999 can hand its `List` to a method written next year that
expects a `List<String>`, and neither side can tell the difference, because at the
level where they meet there *is* no difference.

Everything in the next section is the bill for that.

C# faced the same question the following year and answered it the other way,
keeping type arguments alive at run time. It could afford to: the CLR was five
years old and the ecosystem was small enough to break. Both teams made the right
call for the situation they were standing in, which is a more interesting
conclusion than one of them being cleverer.

## The bill

Every restriction below is the same fact, wearing a different hat. Once you know
the cause you stop having to memorize them.

**You cannot ask `instanceof` about a type argument.**
```java
if (x instanceof List<String>)     // does not compile
if (x instanceof List<?>)          // fine
```
There is nothing at run time to ask about.

**You cannot write `new T[]` or `new T()`.** At the moment that code runs, nobody
knows what `T` was. Library code works around it with `(T[]) new Object[n]` and an
unchecked warning — open `ArrayList` and you will find exactly that line, with a
comment apologizing for it.

**You cannot overload on type arguments.**
```java
void f(List<String> xs)
void f(List<Integer> xs)     // does not compile
```
Both erase to `f(List)`, and the compiler will not define the same method twice.

**You cannot have generic exceptions**, because `catch` has to do a runtime type
test.

**And you cannot write `List<int>.**

That last one is the expensive one. Erasure needs every element to be an `Object`
reference, and a primitive is not one. So numbers in collections have to be
wrapped — which is why autoboxing exists, why `Stream<Integer>` took ten times as
long as `IntStream` when you measured it, and why `java.util.function` contains
forty near-identical interfaces instead of six.

Chapter 16 introduced the split between primitives and objects as a decision made
in 1995 and left it there. This is where the invoice arrives.

## What survives

Now the part that surprises people who have been told everything is erased,
because it is not.

```java
static void takesList(List<String> xs) { }
```

Ask reflection about that parameter and you get this:

```
parameter type: java.util.List<java.lang.String>
erased type   : interface java.util.List
```

The full generic type came back. It is sitting in the class file's signature
attribute — as *metadata*, not as a runtime type — kept there for the compiler to
read when it compiles something against your library, and for tools to read when
they want to know what you meant.

Which gives a rule worth carrying:

> Generic information is kept where it is **declared**, and erased where it is
> **used**.

A method's signature, a field's declared type, a class's supertype — all
recoverable. The type argument of a particular object in your hand — gone, because
that object never had one to begin with.

Frameworks live off the surviving half. Jackson knows a field is a `List<Person>`
by reading the signature attribute, which is how it manages to deserialize into
the right element type instead of handing you a list of maps.

And there is a trick built on the same fact that you have almost certainly typed
without knowing why. Subclass a generic type and the argument becomes part of a
*class declaration* — declared, therefore kept. Which is why serialization
libraries ask you for this:

```java
new TypeReference<List<Person>>() {}
```

Those empty braces are not punctuation. They create an anonymous subclass, purely
so that `List<Person>` ends up somewhere erasure cannot reach.

You have probably copied that line off the internet at some point and wondered
what the braces were for.

Next: the mechanism for attaching information the type system was never able to
express.
