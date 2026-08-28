# One Job per Method

The principle is easy to state and harder to apply than it looks, because "one
job" is not self-defining.

## The problem with the phrase

Consider:

```java
static double mean(int[] a) {
    int total = 0;
    for (int x : a) total += x;
    return (double) total / a.length;
}
```

Does this do one thing? It sums, and it divides. That is two operations. Yet
nobody would say `mean` does two jobs.

Now:

```java
static void saveAndEmail(Order o) {
    database.save(o);
    mailer.send(o.customer(), "Order received");
}
```

Also two operations, and this one clearly does two jobs.

So the count of operations is not the criterion. What is?

## A better formulation

**A method does one job when it operates at a single level of abstraction, and
when the caller would think of it as one action.**

`mean` passes: summing and dividing are both parts of *computing a mean*, which is
what a caller thinks of as one thing. The operations are at the same level — both
are arithmetic in service of one result.

`saveAndEmail` fails: persisting and notifying are separate concerns that happen
to be done together at one call site. A caller might want one without the other,
and the fact that the name needs "and" is the signal.

The **level of abstraction** part is the piece usually missing from the advice, and
it catches a different error:

```java
static void processOrder(Order o) {
    validate(o);
    calculateTotals(o);
    for (int i = 0; i < o.items.size(); i++) {          // wrong level
        Item item = o.items.get(i);
        item.price = item.price * (1 - item.discount);
    }
    save(o);
}
```

Three high-level steps and one loop over item prices. The loop is at a lower level
than everything around it, and reading the method means shifting gears in the
middle. Extract it as `applyDiscounts(o)` and the method reads as four steps at
one level.

The rule of thumb: **a method's body should read as a sequence of steps at
comparable altitude.** When one step is markedly more detailed than its
neighbors, it wants to be a method.

## Command and query

A specific and useful case of the principle, worth having as a rule.

**A method should either do something or answer something, not both.**

```java
// answers — a query
static boolean isValid(Order o)

// does — a command
static void save(Order o)

// both — trouble
static boolean saveAndReport(Order o)
```

This is Chapter 11's pure-versus-effectful distinction, sharpened into a design
rule and usually called **command–query separation**.

Why it matters: a query can be called freely — twice, or not at all, or in a
different order — because it changes nothing. A command cannot. When a method is
both, every call site has to think about both aspects, and you lose the ability to
reason about the query part independently.

The classic violation is a method like `getNextItem()` that returns an item *and*
advances a position. Calling it twice gives different answers, so it is not really
a getter, and the name misleads. Iterators are built this way for good historical
reasons, and they are a recognized source of confusion.

## How long should a method be?

You will see numbers offered — five lines, ten, twenty, one screen. I am not going
to give you one, because length is a symptom rather than the thing itself.

What actually matters:

**Can you name it?** If yes, and the name has no "and", the length is probably
fine.

**Can you hold it in your head?** If understanding line 40 requires remembering
what happened at line 5, it is too long regardless of the count.

**Is it at one level?** Covered above.

A forty-line method that is a single flat sequence of clearly-named calls can be
easier to read than five eight-line methods that must be visited in turn. And a
six-line method doing three unrelated things is too long.

That said: if you are writing something over about fifty lines, it is worth
stopping to ask. The correlation between length and trouble is not perfect but it
is real.

## The number of parameters, again

Five parameters was a signal, back in Chapter 11, and I left it at that. Here is
what it is signalling.

Frequently it means the method does too much — it needs many inputs because it has
many jobs, and splitting it reduces both.

Sometimes it means several parameters belong together. Four parameters describing
a rectangle want to be one `Rectangle`, and Unit V will let you do that. When you
find a group of parameters that always travel together, that group is an object
waiting to be named.

## Cohesion

The word for the property this lesson is describing.

A method is **cohesive** when its parts all contribute to one purpose. It lacks
cohesion when it does several unrelated things — and the tell is that you can
describe the parts separately without loss.

The same idea applies to classes in Unit V, where it becomes more important
because a class holds state as well as behavior. A class whose fields are used by
disjoint sets of methods is two classes that have not been separated yet.

Cohesion has a partner, **coupling** — how much one unit depends on another — and
the general aim is stated as *high cohesion, low coupling*. Chapter 23 takes both
seriously. Introducing the vocabulary now gives you something to notice with while
you write the next few chapters' code.

Next: how to check that any of this works.
