# A Class as an Object

Chapter 21 used `getClass()` in passing to find out what an object actually was.
That was a hint, and it is worth taking seriously now.

If `getClass()` *returns* something, then a class is not only a compile-time notion.
There is an object, at run time, standing for `Account` — and if you can hold it,
you can ask it questions.

Every class you write becomes an object at run time. Not an instance of it — an
object standing *for* the class, which you can hold in a variable and ask
questions of.

```java
Class<?> c = Account.class;
```

`Account.class` is a value. It can be stored, compared, passed to a method, and
asked questions. Chapter 21 met it already as the return of `getClass()`, which
is the other way to obtain one:

```java
Account a = new Account(1, 2500, "Ada");
a.getClass() == Account.class        // true, verified
```

The same object, because the JVM creates exactly one `Class` per loaded class.

## What it knows

```
name        Mirror$Account
simple      Account
superclass  java.lang.Object
```

The `$` in the name is the JVM's convention for a nested class, which is a small
piece of Chapter 5's compilation model showing through.

Fields:

```
id       long   private final
cents    long   private
owner    String private
```

Methods:

```
toString -> String
balance -> long
secret -> void
```

`getDeclaredFields` and `getDeclaredMethods` return everything the class declares,
including private members. `getFields` and `getMethods` return only public ones,
including inherited. The pair of names is worth learning because choosing the
wrong one is the commonest reflection bug.

Note that the full signature is available — parameter types, return type,
modifiers, exceptions, annotations. Everything `javac` knew is still there, with
one large exception that Section 27.2.1 is about.

## Reading and writing by name

```java
Field cents = c.getDeclaredField("cents");
cents.setAccessible(true);
cents.setLong(a, 999999);
```

Verified:

```
before Account[1, 2500]
after  Account[1, 999999]
```

A private field, modified from outside the class, in three lines.

The same for methods:

```java
Method secret = c.getDeclaredMethod("secret");
secret.setAccessible(true);
secret.invoke(a);
```

Verified: `(private method invoked)`.

And for `final`:

```java
Field id = c.getDeclaredField("id");
id.setAccessible(true);
id.setLong(a, 42);
```

Verified: `final field now Account[42, 999999]`.

The field was declared `private final`, assigned once in the constructor, and
reflection changed it. Chapter 20's argument that immutability makes a whole
category of reasoning disappear has just been shown to depend on nobody doing
this.

## setAccessible

That one call is what makes the above possible. Without it, each attempt throws
`IllegalAccessException` — the checks are real and they run.

They are *suppressible*, which is a deliberate design decision rather than an
oversight. The Java designers concluded that frameworks legitimately need to reach
into classes they do not own — a serializer must read private fields, a
dependency injector must call a private constructor — and that the alternative was
every framework demanding that your classes be public.

Since Java 9's module system, `setAccessible` can be refused. A module may declare
which packages are `open` for reflection, and code outside cannot break into the
rest. The JDK's own internals are closed this way, which broke a good deal of
software in 2017 and was correct.

The summary: **encapsulation is enforced against ordinary code and negotiable
against reflection.** Chapter 19's boundary is real for every reader of your class
who is playing by the rules, and reflection is how you announce you are not.

## Constructing by name

```java
Class<?> k = Class.forName("java.util.ArrayList");
Object list = k.getDeclaredConstructor().newInstance();
k.getMethod("add", Object.class).invoke(list, "hello");
```

Verified: `[hello] of ArrayList`.

Nothing in that code mentions `ArrayList` as a type. The class was found from a
string, instantiated, and had a method called on it, all decided at run time.

That is the capability frameworks are built on, and it is worth being precise
about what it enables: **a program can use a class that did not exist when the
program was compiled.** A plugin system, a database driver loaded from a
configuration file, a test runner finding your tests — all of them are
`Class.forName` and `invoke`.

It is also where the type safety goes. `getMethod("add", Object.class)` is a
string and a class object; misspell the string and you get
`NoSuchMethodException` at run time, where a normal call would have failed to
compile. Every guarantee Chapter 17 got from generics and Chapter 22 from
interfaces is suspended inside a reflective call.

## The shape of reflective code

Reflection code has a recognizable and unpleasant shape:

```java
try {
    Method m = obj.getClass().getMethod("process", String.class);
    Object result = m.invoke(obj, "input");
    return (String) result;
} catch (NoSuchMethodException | IllegalAccessException | InvocationTargetException e) {
    throw new RuntimeException(e);
}
```

Three checked exceptions, a cast on the result, and a string where a method name
belongs. Compare with `obj.process("input")`.

One detail worth knowing: `InvocationTargetException` wraps whatever the invoked
method threw. The real exception is in `getCause()`, and forgetting this produces
stack traces that say nothing useful — which is why the test runner in Section
27.2.2 unwraps it.

Next: what all of this costs.
