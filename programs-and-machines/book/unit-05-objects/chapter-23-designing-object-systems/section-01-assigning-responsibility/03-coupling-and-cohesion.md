# Coupling and Cohesion

Chapter 14 handed you two words and asked you to notice things with them. Now they
have to do some work.

The slogan is *high cohesion, low coupling*, and as slogans go it is unusually
empty — nobody has ever set out to write incoherent, tightly coupled code. What
makes the pair useful is that both are measurable enough to argue about, and that
they are proxies for a question you can answer with a number.

**Cohesion** is how strongly the parts of one unit belong together.
**Coupling** is how much one unit depends on another.

The aim is stated the same way everywhere: **high cohesion, low coupling.** Both
halves need unpacking, because stated that way it is a slogan.

## Cohesion

A cohesive class does one thing. Its fields are all used by most of its methods,
its name is a noun you could define in a sentence, and adding a feature to it
feels natural rather than intrusive.

The classic failure is the class named for a layer rather than a concept —
`Manager`, `Handler`, `Processor`, `Utils`. Such a name means *I could not say
what this is*, and it is a reliable predictor that the class holds unrelated
things.

Two practical tests.

**The sentence test.** Describe the class in one sentence with no "and". If you
cannot, it is doing two jobs. "A `Playlist` holds an ordered collection of items
and reports facts about them" passes — the second clause is about the first. "A
`UserManager` validates passwords and sends emails and writes to the database"
does not.

**The field test.** For each field, count the methods that use it. If a class has
six fields and two clusters of methods, each using three fields and ignoring the
others, you have two classes sharing a file. That is Chapter 14's observation and
it works on real code.

Cohesion also has a lower bound. A class with one field and one method that
returns it is not cohesive so much as pointless; splitting until everything is
trivially cohesive produces a program with sixty types and no structure. The
useful direction is *toward* cohesion from a large class, not toward it from a
small one.

## Coupling

Every dependency is a coupling: a field of another type, a parameter, a
constructor call, an inherited superclass, a static call. They differ in strength,
and that is the useful part.

From weakest to strongest:

**Data coupling.** A method takes a parameter and returns a result. The weakest
and the goal.

**Interface coupling.** A class depends on `List`, not `ArrayList`. It knows a
contract and not an implementation, which is Chapter 22's rule and the reason for
it.

**Concrete coupling.** A class depends on a specific class. Fine when you own both
and the class is stable.

**Construction coupling.** A class calls `new` on another. Now it depends on the
type *and* on how to build one, and a test cannot substitute anything. Moving the
`new` to the caller — passing the object in — removes half of it. That is
dependency injection, and the whole idea is that sentence.

**Inheritance coupling.** The strongest, for the reasons Section 23.1.2 gave. You
depend on the superclass's internal calls, which are not documented and may
change.

**Global coupling.** Shared mutable static state. Everything that touches it is
coupled to everything else that touches it, invisibly, and Chapter 31 shows what
this does when threads arrive.

The rule follows: **use the weakest coupling that does the job.** Prefer a
parameter to a field, a field to a `new`, an interface to a class, composition to
inheritance, and nearly anything to global state.

## Why they go together

The two are not independent, which is why they are always named as a pair.

Splitting an incoherent class raises cohesion, and it also lowers coupling for its
callers — a caller that needed one of the two jobs now depends on one small class
rather than one large one.

And they trade against each other if pushed too far. Perfect cohesion — one method
per class — maximizes the number of connections between classes, which is coupling
by another route. The aim is a balance, and the balance is judged by the question
this chapter opened with: when a likely change arrives, how many files does it
touch?

## The measure that actually matters

Coupling and cohesion are proxies. The thing they are proxies for is:

**How far does a change propagate?**

A good design localizes likely changes. Changing how a playlist stores its items
should touch `Playlist` and nothing else. Adding a new item type should touch the
new type and the place that creates it. Changing an output format should touch the
formatter.

When you are unsure whether a design is good, do not audit it against principles.
Name the three changes most likely to arrive within a year, and trace each one
through the design counting files. That number is the answer, and it is a
measurement rather than an opinion.

The three principles this section gave — behavior with its data, composition over
inheritance, weakest coupling that works — are all just accumulated experience
about what keeps that number small.

Next: getting the design out of your head before you commit to it.
