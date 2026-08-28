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

Read it again and notice what it does not tell you. How many cards. What a grade
looks like. Whether two students share a deck. Whether "policy" means two options
or twenty.

That is what a real specification is like — incomplete, faintly ambiguous, and
entirely enough to start with.

## First pass: underline the nouns

It is the oldest trick in the subject and it still works. Write down every noun in
the paragraph and then argue with the list.

*study program, flashcard, front, back, student, grade, scheduling policy.*

**Card** — clearly a class. Front and back are its data.

**Grade** — a small, fixed set of possibilities that nobody is going to extend at
run time. That is an enum, and it is the one decision here you can make without
thinking.

**Scheduling policy** — "different policies exist and the student chooses" is an
interface, almost word for word. Several implementations, no shared state, chosen
at run time.

**Student** — probably not, and this is the interesting one. The word is in the
paragraph, so the instinct is to make a class for it. But read the requirements
again: nothing asks the program to *know* anything about a student. A class with
no state and no behaviour is a class waiting to be deleted, so leave it out. If
something later needs it, it will say so.

**Study program** — the whole application, not a class. But something has to hold
the collection of cards, and that will need a name. Call it **Deck**.

Four types: `Card`, `Grade`, `Scheduler`, `Deck`.

## The first wrong turn

Here is what I actually wrote first, and I would encourage you to look at it and
decide whether you would have caught it:

```java
class Card {
    String front, back;
    int box;
    int dueOnDay;

    void review(Grade g, int today) { ... }        // ← wrong
}
```

And it looks right. The card knows its own state, and the last section spent
several pages arguing that behaviour belongs with the data it needs.

It is wrong, and the reason is worth naming carefully, because it is subtle enough
that I did not see it until I tried to write the next method: **`review` needs the
scheduling policy, and the policy is none of the card's business.**

Follow it through. Either every card carries a reference to a scheduler — one field
duplicated across ten thousand cards, all pointing at the same object — or `review`
takes a scheduler as a parameter, which means the caller has to supply it, which
means the caller has to know about scheduling, which was precisely the thing we
were trying to keep out of sight.

The signal was there before I noticed it, and it is a useful one: **the class
needed a collaborator to do its own job.** That is exactly what the index-card
exercise catches — a responsibility written on the wrong card.

## The second wrong turn

The fix I reached for next:

```java
class LeitnerCard extends Card { ... }             // ← also wrong
class CramCard extends Card { ... }                // ←
```

A card that schedules itself one way, a card that schedules itself another. Two
problems with it, and Chapter 21 gave you both.

Start by saying it out loud: *a Leitner card is a kind of card.* Is it? A
scheduling policy is not a kind of card. A card is a front and a back; how
often it comes back is a fact about the study session, not about the card. The
`is-a` test fails on reading it out loud.

And it does not do what the specification asked. "The student should be able to
choose a policy" means choosing at run time, for the whole deck. Inheritance
decides at compile time, per object. To change policy you would have to rebuild
every card.

Which is the closing point of the last section, turning up in practice rather than
in the abstract: inheritance fixes a choice at the moment the class is written, and
this particular choice has to be made much later than that.

## What the wrong turns were pointing at

Two failed attempts, and both failed at the same place — something about a card was
refusing to sit inside the card. Look at what that something was, and the design
falls out:

**a card and its review history are two different things.**

A card is content — front and back — and never changes. History is state, and it
is state about *this student's* relationship with the card, which is exactly why
it does not belong inside content that could be shared.

```java
record Card(String front, String back) { }

record Progress(int box, int dueOnDay) {
    Progress { if (box < 0) throw new IllegalArgumentException("box"); }
}
```

Both records, for the reason Chapter 22 gave: each of them *is* its data, two with
equal contents are interchangeable, and `Card` in particular is about to become a
map key — where immutability and an honest `hashCode` stop being preferences.

And that compact constructor is not decoration. A negative box number is
meaningless, so it is refused once, at the only moment a `Progress` can come into
existence. Because the record is immutable, that check can never be undone. One
line, and a whole category of bad state is now unrepresentable.

## The policy

```java
interface Scheduler {
    Progress next(Progress current, Grade grade, int today);
    String name();
}
```

One method that matters. Given where a card stands, how it was graded, and what
day it is, produce the new standing.

Read the signature for what is missing from it.

There is no `Card`. Scheduling has nothing to do with what is written on the front
of the card, and accepting one would tie the policy to a type it has no use for.

And there is no mutation. It returns a *new* `Progress` rather than editing the old
one — so a scheduler holds no state at all, which means two threads can share one
without any of Chapter 31's difficulties arising.

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

That `intervals.clone()` is a defensive copy on the way in. Leave it out and the
caller still holds a reference to the array they handed you, and can quietly
rewrite your schedule from outside at any point afterwards.

What you have just built has a name — **Strategy** — and of the twenty-three
patterns in that book it is the one worth learning first: an algorithm as an
object, chosen by whoever constructs the thing that uses it.

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

Five decisions in twenty lines, and every one of them traces back to something
earlier in the book.

**A `Map<Card, Progress>`** rather than a card holding its own progress — the
separation, made concrete. And Chapter 17's warning about mutable map keys does
not apply here, because `Card` is a record and cannot change under the map.

**`LinkedHashMap`** so iteration order is insertion order rather than hash order.
The specification did not ask, but unpredictable ordering is the kind of thing
that produces a bug report about "random" behavior, and this costs nothing.

**The scheduler arrives in the constructor.** `Deck` never calls `new` on one,
which is the construction coupling from earlier in the chapter, avoided — so a test
can hand it a fake, and the class is tied to no policy at all.

**`due` returns `List.copyOf`.** Try calling `add` on what comes back and it
throws, which is the boundary holding.

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

Now look at the `cram` line, because that is the one the whole design was for.

Same cards. Same review. Different answer. And the only thing that differs
anywhere in the program is one argument to `Deck`'s constructor.

That sentence in the requirements — *the student should be able to choose one* —
turned into that. Both wrong turns were the price of getting there.

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
