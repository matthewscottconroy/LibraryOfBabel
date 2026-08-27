# Copying, Shallow and Deep

Copying an object raises a question the language cannot answer for you: **how far
down?**

```java
class Order {
    String customer;
    List<Item> items;
}
```

A copy of an `Order` clearly needs its own `customer` and `items` fields. Does
it need its own *list*? Its own *items*?

Three answers, all defensible.

## The three depths

**A reference copy** is not a copy at all — `b = a` gives a second name for one
object. Included because people frequently believe this is a copy, and it is the
default when you do nothing.

**A shallow copy** makes a new object whose fields hold the same values — which,
for reference fields, means the same references.

```java
List<List<Integer>> outer = new ArrayList<>();
outer.add(new ArrayList<>(List.of(1, 2)));

List<List<Integer>> shallow = new ArrayList<>(outer);
shallow.get(0).add(3);

System.out.println(outer.get(0));      // [1, 2, 3]
```

Two outer lists, one inner list. The copy is genuinely a separate object — adding
to `shallow` itself would not affect `outer` — but everything it points at is
shared.

**A deep copy** copies recursively, so nothing is shared. You must write it.

## Why shallow is the default everywhere

Every copying facility Java gives you is shallow: `clone()`, `Arrays.copyOf`,
`new ArrayList<>(other)`, the copy constructors in the library.

The reason is that deep copying is not well defined in general. How deep is deep?
If an `Order` refers to a `Customer` who refers to their other orders, a deep copy
of one order duplicates the customer and every order they ever placed. Cycles make
it worse — a naive recursive copy of a structure that refers back to itself does
not terminate.

So the language does the one thing that is always well defined and leaves the rest
to you, which is the right decision and catches everyone once.

Chapter 15 met this with `int[][]`: `clone()` copies the outer array's references,
so both grids share their rows.

## Writing a deep copy

By hand, in a copy constructor:

```java
public Order(Order other) {
    this.customer = other.customer;                    // String: immutable, share it
    this.items = new ArrayList<>();
    for (Item item : other.items) {
        this.items.add(new Item(item));                // each item copied too
    }
}
```

Note the first line. `customer` is a `String`, which is immutable, so sharing it is
safe and copying it would be waste. **Immutable things never need copying**, and
noticing which of your fields are immutable is most of the work of writing a
correct copy.

## clone and why to avoid it

Java has a `clone()` mechanism involving a `Cloneable` interface that declares no
methods, a `clone` method that is `protected` on `Object`, and a convention that
implementations should call `super.clone()`.

It is widely regarded as a mistake. `Cloneable` does not declare `clone`, so
implementing it does not make `clone` available; the default is shallow, which is
rarely what people want; and it interacts badly with `final` fields, since it
creates an object without running a constructor.

**Use a copy constructor or a static factory instead.** They are ordinary methods,
you write what they do, and there is no protocol to get wrong.

```java
Order copy = new Order(original);          // copy constructor
Order copy = Order.copyOf(original);       // static factory
```

## The question to ask

Before writing any copy, ask: **why?**

Frequently the honest answer is "so the caller cannot modify mine", which is
Section 20.1.2's defensive copying — and the better fix is usually to make the
thing immutable.

Sometimes it is "I need to modify one without affecting the other", which is a
genuine reason and a copy is right.

And sometimes it is "everyone writes copy constructors", which is not a reason.
An object you never modify never needs copying, and most objects in a
well-designed program are never modified.

That is where the chapter is heading, and Section 20.2.3 makes the argument
properly.

Next: the two questions English confuses.
