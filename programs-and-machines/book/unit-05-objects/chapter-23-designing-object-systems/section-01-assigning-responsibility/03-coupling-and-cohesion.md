# Coupling and Cohesion

*High cohesion, low coupling.*

You will hear that slogan for the rest of your working life, and as slogans go it
is unusually empty. Nobody has ever sat down at a keyboard intending to write
incoherent, tightly coupled code. Advice that only tells you to avoid the thing
nobody wants is not advice.

What rescues the pair is that both halves are concrete enough to argue about with
evidence, and that both are standing in for a single question you can answer with
an actual number. We will get to the number at the end. First the two words have to
do some work, because Chapter 14 handed them to you and asked only that you
notice things with them.

**Cohesion** is how strongly the parts of one unit belong together.
**Coupling** is how much one unit depends on another.

Both halves need unpacking before they mean anything at all. Take them in turn.

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

**The field test.** Go through the class field by field and count which methods
touch each one. Then look at the pattern.

If a class has six fields and the methods fall into two clusters — one cluster
using three fields and ignoring the other three, the second cluster doing the
reverse — you are looking at two classes that happen to share a file. This is
Chapter 14's observation, and unlike most design advice it works on real code, in
five minutes, without any judgment being required of you.

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

So when you are unsure whether a design is any good, do not sit down and audit it
against a list of principles. Do this instead.

Name the three changes most likely to arrive within the next year. Take each one
and trace it through the design, counting the files it touches. Write down the
three numbers.

That is your answer, and the reason it is worth more than any principle in this
chapter is that it is a measurement rather than an opinion — yours or anybody
else's.

The three principles this section gave — behavior with its data, composition over
inheritance, weakest coupling that works — are all just accumulated experience
about what keeps that number small.

Next: getting the design out of your head before you commit to it.
