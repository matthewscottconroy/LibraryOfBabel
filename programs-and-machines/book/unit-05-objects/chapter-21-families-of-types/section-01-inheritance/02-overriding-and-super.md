# Overriding and super

**Overriding** is replacing an inherited method with your own.

```java
class Shape {
    double area() { return 0; }
}

class Circle extends Shape {
    @Override
    double area() { return Math.PI * r * r; }
}
```

Same name, same parameters, same return type. When `area()` is called on a
`Circle`, the `Circle` version runs — even if the calling code thinks it has a
`Shape`. That is dynamic dispatch, and Section 21.2.2 explains the mechanism.

## Overriding is not overloading

The distinction Chapter 12 promised, and it is worth stating side by side:

| | overloading | overriding |
|---|---|---|
| what varies | the parameters | the class |
| resolved | at compile time | at run time |
| decided by | the *declared* type | the *actual* type |
| relationship | two methods, one name | one method, replaced |

```java
static String describe(Shape x) { return "a shape"; }
static String describe(Circle x) { return "a circle"; }

Shape c = new Circle(1);

c.area()        // 3.1416 — the Circle version. Overriding: dynamic.
describe(c)     // "a shape" — Overloading: static, uses the declared type.
```

That pair is the whole idea. `c` is declared `Shape` and is actually a `Circle`.
The **override** follows what it actually is; the **overload** follows what it was
declared as, because the compiler chose before the program ran and had only the
declaration to go on.

Forcing the other overload requires a cast, which is you telling the compiler
something it could not know:

```java
describe((Circle) c)      // "a circle"
```

If overload resolution has ever surprised you, this is why — and it is a good
argument for Chapter 12's advice not to overload methods that do different things.

## @Override

```java
@Override
double area() { ... }
```

An annotation asserting that this method replaces one from a supertype. If it does
not — a misspelling, a wrong parameter type — the compiler reports an error.

**Always write it.** Chapter 20 showed the failure it prevents: `equals(Point o)`
takes the wrong parameter type, so it is an overload rather than an override, the
collections call the inherited version, and nothing tells you. `@Override` turns
that silent wrong behavior into a compile error.

It costs one line and it is the cheapest correctness check in the language.

## super

Sometimes you want to extend a behavior rather than replace it:

```java
class Savings extends Account {
    @Override
    public void deposit(long amount) {
        super.deposit(amount);        // do what an Account does
        recordInterestEvent(amount);  // and then this
    }
}
```

`super.deposit(...)` calls the superclass's version. Without it, writing
`deposit(...)` would call *this* method again — infinite recursion, and Chapter
12's `StackOverflowError` with a repeating trace.

`super` is also how a constructor initializes the inherited part, as the last
lesson showed.

## What overriding must not do

Overriding is where inheritance goes wrong, and the rules are about not breaking
promises.

**Do not weaken the contract.** If `Account.withdraw` promises to refuse
overdrafts, a subclass that permits them has broken every caller written against
`Account`. This is the substitution principle, and Section 21.2.3 gives it
properly.

**Do not strengthen the preconditions.** If the superclass accepts any positive
amount, a subclass that demands amounts under 100 will fail on input the caller
was entitled to pass.

**Do not narrow the access.** Java enforces this one — you cannot make a `public`
method `protected` in a subclass, because callers holding the supertype would be
calling something they can no longer reach.

**Do not call an overridable method from a constructor.** This one is subtle and
worth knowing. The superclass constructor runs first, and if it calls a method the
subclass overrode, the subclass's version runs **before the subclass's fields have
been initialized** — so it sees them as null or zero. The result is a
`NullPointerException` in a method that looks perfectly correct.

Constructors should call only `private`, `static`, or `final` methods, all of
which cannot be overridden.

## Fields are not overridden

A trap that looks like overriding and is not:

```java
class A { String name = "A"; }
class B extends A { String name = "B"; }
```

This does not replace the field; it adds a second one. The object now has two
fields called `name`, and which you get depends on the *declared* type of the
reference — like overloading, resolved statically.

This is called **shadowing** and it is essentially always a mistake. Fields should
be `private`, in which case the question does not arise, which is one more reason
for Chapter 19's rule.

Next: the class everything inherits from.
