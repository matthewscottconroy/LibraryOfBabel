# A Worked Design

Every design in this chapter so far has arrived finished, which is a dishonest way
to present the subject. Finished designs make the decisions look inevitable, and
they were not — somebody tried something else first.

So here is one worked from a paragraph of requirements to running code, with the
wrong turns left in. The wrong turns are the part worth reading; watching somebody
change their mind is more instructive than watching them be right.

A problem, in one paragraph:

> A study program holds flashcards. Each card has a front and a back. After
> reviewing a card the student grades how well they knew it, and the program
> decides when to show it again. Different scheduling policies exist and the
> student should be able to choose one.

That is a real specification: incomplete, slightly ambiguous, and enough to start.

What follows is the design as it actually goes, wrong turns included.

## First pass: name the nouns

The oldest technique there is. Underline the nouns and see which survive.

*study program, flashcard, front, back, student, grade, scheduling policy.*

**Card** — clearly a class. Front and back are its data.

**Grade** — a fixed, small set of possibilities. This is Chapter 22's enum, and
it is the easiest decision in the design.

**Scheduling policy** — "different policies exist and the student chooses" is an
interface, almost word for word. Several implementations, no shared state, chosen
at run time.

**Student** — probably not. Nothing in the specification asks the program to know
anything about a student, and a class with no state and no behavior is a class
waiting to be deleted. Leave it out; add it when something needs it.

**Study program** — the whole application, not a class. But something has to hold
the collection of cards, and that will need a name. Call it **Deck**.

Four types: `Card`, `Grade`, `Scheduler`, `Deck`.

## The first wrong turn

Here is what I wrote first:

```java
class Card {
    String front, back;
    int box;
    int dueOnDay;

    void review(Grade g, int today) { ... }        // ← wrong
}
```

It seems right. The card knows its own state, and Section 23.1.1 said behavior
belongs with the data.

It is wrong for a reason worth naming: **`review` needs the scheduling policy, and
the policy is not the card's business.** Either every card carries a reference to
a scheduler — a field duplicated across ten thousand cards, all pointing at the
same object — or `review` takes one as a parameter, which means the caller must
supply it, which means the caller must know about scheduling, which is the thing
we were trying to encapsulate.

The signal was there before I noticed it: the class needed a collaborator to do
its own job. That is what Section 23.1.1's card exercise catches — a
responsibility written on the wrong card.

## The second wrong turn

The fix I reached for next:

```java
class LeitnerCard extends Card { ... }             // ← also wrong
class CramCard extends Card { ... }                // ←
```

A card that schedules itself one way, a card that schedules itself another.

Two problems, and both are Chapter 21's.

The scheduling policy is not a *kind of card*. A card is a front and a back; how
often it comes back is a fact about the study session, not about the card. The
`is-a` test fails on reading it out loud.

And it does not do what the specification asked. "The student should be able to
choose a policy" means choosing at run time, for the whole deck. Inheritance
decides at compile time, per object. To change policy you would have to rebuild
every card.

That is Section 23.1.2's closing point arriving in practice: inheritance fixes a
choice when the class is written, and this choice needs to be made later.

## The separation

The insight, once the wrong turns had made it obvious: **a card and its review
history are two different things.**

A card is content — front and back — and never changes. History is state, and it
is state about *this student's* relationship with the card, which is exactly why
it does not belong inside content that could be shared.

```java
record Card(String front, String back) { }

record Progress(int box, int dueOnDay) {
    Progress { if (box < 0) throw new IllegalArgumentException("box"); }
}
```

Both records, and both for the reason Chapter 22 gave: they are their data, two of
each with equal contents are interchangeable, and `Card` in particular is going to
be a map key, where immutability and a correct `hashCode` are not optional.

The compact constructor is doing real work. A negative box is meaningless, it is
rejected once, and because the record is immutable it can never become negative
later.

## The policy

```java
interface Scheduler {
    Progress next(Progress current, Grade grade, int today);
    String name();
}
```

One method that matters. Given where a card stands, how it was graded, and what
day it is, produce the new standing.

Notice what the signature does not have. No `Card` — scheduling does not depend on
the content, and taking one would couple the policy to a type it has no use for.
No mutation — it returns a new `Progress` rather than modifying one, so a
scheduler has no state and two threads may share it safely.

Two implementations:

```java
final class Leitner implements Scheduler {
    private final int[] intervals;
    Leitner(int... intervals) { this.intervals = intervals.clone(); }

    public Progress next(Progress p, Grade g, int today) {
        int box = g == Grade.AGAIN ? 0 : Math.min(p.box() + 1, intervals.length - 1);
        return new Progress(box, today + intervals[box]);
    }
    public String name() { return "Leitner"; }
}

final class Cram implements Scheduler {
    public Progress next(Progress p, Grade g, int today) {
        return new Progress(p.box(), g == Grade.EASY ? today + 1 : today);
    }
    public String name() { return "Cram"; }
}
```

`intervals.clone()` is Chapter 20's defensive copy on the way in — without it, the
caller keeps a reference to the array and can change the schedule afterwards.

The pattern has a name, **Strategy**, and it is the one design pattern that is
worth learning early: an algorithm as an object, chosen by whoever constructs the
thing that uses it.

## The deck

```java
final class Deck {
    private final Map<Card, Progress> progress = new LinkedHashMap<>();
    private final Scheduler scheduler;

    Deck(Scheduler scheduler) { this.scheduler = scheduler; }

    void add(Card c) { progress.putIfAbsent(c, new Progress(0, 0)); }

    List<Card> due(int today) {
        List<Card> out = new ArrayList<>();
        for (var e : progress.entrySet())
            if (e.getValue().dueOnDay() <= today) out.add(e.getKey());
        return List.copyOf(out);
    }

    void review(Card c, Grade g, int today) {
        Progress p = progress.get(c);
        if (p == null) throw new NoSuchElementException(c.front());
        progress.put(c, scheduler.next(p, g, today));
    }
}
```

Several decisions in there, each traceable to something earlier.

**A `Map<Card, Progress>`** rather than a card that holds its progress. The
separation, made concrete. Chapter 17's warning about mutable keys does not apply,
because `Card` is a record and immutable.

**`LinkedHashMap`** so iteration order is insertion order rather than hash order.
The specification did not ask, but unpredictable ordering is the kind of thing
that produces a bug report about "random" behavior, and this costs nothing.

**The scheduler arrives in the constructor.** Section 23.1.3's construction
coupling, avoided: `Deck` never calls `new` on a scheduler, so a test can pass a
fake one and the class is not tied to any policy.

**`due` returns `List.copyOf`.** Section 23.1.1's boundary. Verified — calling
`add` on the returned list throws.

**`review` takes the day as a parameter** rather than calling `LocalDate.now()`.
This is one of the highest-value habits in the chapter: a class that reads the
clock cannot be tested without waiting, and time is the single most common source
of untestable code.

## Running it

```
scheduler Leitner, 2 cards
due day 0 : 2
after day 0: a=Progress[box=1, dueOnDay=2] b=Progress[box=0, dueOnDay=1]
due day 1 : 1
due day 2 : 2
cram      : a=Progress[box=0, dueOnDay=0]
grades    : [AGAIN, HARD, GOOD, EASY]
due() list is immutable
```

Both cards are due on day 0. Card `a` graded GOOD moves to box 1 and returns on
day 2; card `b` graded AGAIN stays in box 0 and returns on day 1. On day 1 exactly
one is due; on day 2 both are.

The `cram` line is the one to look at. Same cards, same review, different result —
and the only difference in the program is the argument to `Deck`'s constructor.
That is what "the student can choose a policy" turned into, and it is the payoff
for both wrong turns.

## What was left out, deliberately

**Persistence.** Nothing here saves to a file. Saving is about a file format, not
about flashcards, and Section 23.1.1's test — would this method still make sense
if the format changed? — puts it outside. A separate `DeckStore` reads and writes
a `Deck`, and neither knows much about the other.

**The student.** Still not needed. When multiple students appear, each gets their
own `Map<Card, Progress>`, and the separation done above is what makes that a
small change rather than a rewrite.

**A user interface.** Also outside. The deck exposes `due` and `review`, which is
everything a console loop, a web page, or a test needs.

Notice that all three exclusions are the same decision: **keep out what changes
for reasons unrelated to the domain.** Storage formats, students, and interfaces
change on their own schedules, and each one that gets inside the `Deck` is a
reason the `Deck` will have to change.

## Testing the design

Section 23.1.3 said to stop auditing against principles and count files instead.
Three plausible changes:

**A new scheduling policy.** One new class. Nothing else is touched — not `Deck`,
not `Card`, not `Progress`. One file.

**Cards gain a hint field.** `Card` changes; whatever constructs cards changes.
`Deck` and the schedulers do not, because neither ever looks inside a card. Two
files.

**Scheduling starts depending on how long the student took to answer.** This one
hurts: `Scheduler.next` needs another parameter, so the interface changes and both
implementations change and `Deck.review` changes. Four files.

That third number is the design's honest weak point, and it is worth sitting with
rather than hiding. Could it have been avoided? Yes — `next` could take a
`ReviewEvent` record instead of three loose parameters, and a new field would then
be additive.

Should it have been? Probably not, at the point the design was made. Wrapping
three parameters in a type against a change nobody has asked for is speculative
generality, and it has its own cost: an extra type to name, construct, and read
past. The specification said nothing about timing.

The judgment being exercised here is the one this chapter is really about. You
cannot make every change cheap. You choose which changes to make cheap, based on
which you actually expect, and you accept that the others will cost more.

Getting that judgment right is what design is, and it comes from having been wrong
about it before — which is why the two wrong turns above were left in.

That closes Unit V. Unit VI takes the whole apparatus — classes, interfaces,
records, recursion, the stack — and turns it on programs themselves.
