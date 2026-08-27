# Objects, State, and Identity

Unit IV ended owing you a mechanism.

Chapter 16 argued that a collection of values means whatever we have agreed it
means, that the agreement is a **representation invariant**, and that the value of
putting a boundary around one is that the code which could break it becomes small
enough to check. Then it said the mechanism was `private`, and deferred it.

This unit is that deferral coming due, and it is worth saying at the outset that
the mechanism is the least interesting part. `private` is one keyword. What takes
five chapters is the judgment: what belongs together, what should be visible, when
two things count as the same thing, and when a family of types is a good idea
rather than a trap.

Unit I said a machine holds patterns and meaning is an agreement layered on top.
Here is that claim in its fifth costume.

An **object** is a pattern in memory. That it is an `Account` rather than a
`Point` is an agreement. That its `cents` field means a balance in pence, that
the balance is never negative, that two accounts with the same owner are not
thereby the same account — every one of those is an agreement, and none of them
is in the bits.

The difference this unit makes is that Java can now **enforce** some of the
agreement instead of merely documenting it. That is a genuine change in kind, and
it is what Liskov's abstract data types contributed: a convention anyone can
reach around is not a guarantee, and a guarantee is what lets you reason about
which code can break an invariant.

Five chapters.

**Chapter 19 — Bundling State and Behavior.** The class: fields, constructors,
encapsulation, and `static`. This is also where the debts from Chapter 5 come
due — `public`, `static`, `void`, and `String[] args` all get their explanation,
and you are invited to compare them against what you guessed.

**Chapter 20 — Identity and Equality.** The hard one. When are two objects the
same object, and when are they merely equal? Aliasing, `==` against `.equals`,
the `hashCode` contract, and immutability as the strategy that makes most of the
difficulty disappear.

**Chapter 21 — Families of Types.** Inheritance and polymorphism, treated as a
design decision with a principle underneath it rather than as a syntax to learn.

**Chapter 22 — Contracts Without Implementation.** Interfaces, abstract classes,
enums, and records — four ways of saying what something must do without saying
how.

**Chapter 23 — Designing Object Systems.** Responsibility, composition against
inheritance, coupling and cohesion, and a worked design from a problem statement
to a set of classes.

A warning about how this is usually taught. Object orientation is often introduced as a set of four words — encapsulation,
inheritance, polymorphism, abstraction — followed by a `Dog extends Animal`
example, and the impression left is that inheritance is the center of the subject
and that you should use it wherever a hierarchy suggests itself.

That is close to backwards. Inheritance is the most easily misused construct in
the language, and Chapter 23 will argue that composition is right far more often.
What is actually central is the thing Unit IV set up: **a boundary around an
invariant**, so that a reader knows which code could possibly have broken it.

If you have met object orientation before and it felt like ceremony — getters and
setters written mechanically for every field, deep hierarchies that had to be
navigated to understand anything — that impression was earned by a lot of real
code. This unit tries to give you the reasons, so that you write the ceremony
where it buys something and omit it where it does not.
