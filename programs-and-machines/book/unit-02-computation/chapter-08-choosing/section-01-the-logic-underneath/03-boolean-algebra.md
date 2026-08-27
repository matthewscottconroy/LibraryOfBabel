# Boolean Algebra

Boole's contribution was noticing that logic obeys algebraic laws — that you can
transform a logical expression the way you transform an arithmetic one, and be
certain the meaning is unchanged.

This is practical. Conditions in real code get complicated, and a complicated
condition is where bugs hide. The laws let you rewrite one into a provably
equivalent form that a human can read.

## The laws

Writing AND as `&&`, OR as `||`, NOT as `!`:

**Commutative** — order does not matter.
```
A && B   ==   B && A
A || B   ==   B || A
```

**Associative** — grouping does not matter.
```
(A && B) && C   ==   A && (B && C)
```

**Distributive** — like multiplication over addition, and, unlike arithmetic, it
works both ways round.
```
A && (B || C)   ==   (A && B) || (A && C)
A || (B && C)   ==   (A || B) && (A || C)
```

**Identity and annihilation.**
```
A && true  == A          A && false == false
A || false == A          A || true  == true
```

**Double negation.**
```
!!A  ==  A
```

**Idempotence** — repetition adds nothing.
```
A && A  ==  A
A || A  ==  A
```

**Complement.**
```
A && !A  ==  false
A || !A  ==  true
```

Every one of these can be verified by writing out the truth table for both sides
and checking the columns match. That is a complete proof, because there are
finitely many rows. Do one or two by hand; the finiteness is the thing to
appreciate.

## De Morgan's laws

The two most useful, named for Augustus De Morgan, a contemporary of Boole:

```
!(A && B)   ==   !A || !B
!(A || B)   ==   !A && !B
```

In words: **negating an AND gives an OR of the negations, and vice versa.** The
negation moves inward and flips the operator.

They are worth internalizing because negated compound conditions are common and
hard to read. Consider:

```java
if (!(age >= 18 && hasLicence)) {
    refuse();
}
```

Working out when that fires requires holding a negation over a conjunction in
your head. De Morgan turns it into:

```java
if (age < 18 || !hasLicence) {
    refuse();
}
```

"Too young, or no licence." Which is what a person would say, and now the code
says it too.

Note that the inner comparison flipped as well: `!(age >= 18)` became
`age < 18`. Negating a comparison inverts it, and the boundary is where mistakes
happen — the opposite of `>=` is `<`, not `<=`. Get that wrong and you have an
off-by-one that fires on exactly one input value.

## Using this in practice

Three habits that follow.

**Push negations inward.** A `!` on a compound expression is harder to read than
negations on the pieces. De Morgan lets you move it without risk.

**Name intermediate conditions.** Boolean algebra tells you a rewrite is safe;
naming tells the reader what it means.

```java
// hard
if (!(user != null && user.isActive() && !user.isSuspended())) { ... }

// better
boolean canProceed = user != null && user.isActive() && !user.isSuspended();
if (!canProceed) { ... }
```

Nothing was simplified logically. It became readable, which was the actual
problem.

**Beware the English "and".** Specifications say things like "reject users under
18 and users without a licence". That "and" describes two rules; the condition
that implements it is an OR. Translating requirements into conditions is where
this chapter's algebra earns its keep, and where a truth table on paper is often
the fastest route to certainty.

## Simplification

The laws let you shrink expressions:

```
(A && B) || (A && !B)
  = A && (B || !B)      distributive
  = A && true           complement
  = A                   identity
```

So `(A && B) || (A && !B)` is just `A`. If you found that condition in real code
you could delete two thirds of it, and you would know — not suspect — that the
behavior is unchanged.

This is the same activity as simplifying an algebraic expression, and it has the
same value: fewer terms, fewer places to be wrong.

In hardware it matters directly, since each removed operator is removed gates,
and there are systematic methods — Karnaugh maps, the Quine–McCluskey algorithm —
for finding minimal forms. For software the benefit is comprehension rather than
cost, which is worth as much.

Next: what Java actually gives you to write conditions with.
