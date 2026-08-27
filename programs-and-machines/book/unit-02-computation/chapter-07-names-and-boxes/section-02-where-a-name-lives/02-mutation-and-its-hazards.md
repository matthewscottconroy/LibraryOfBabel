# Mutation and Its Hazards

A variable can change. That is what the *vari*- is doing in the word, and it is
the reason the construct exists — Chapter 6's transitions have to change
something, and variables are what they change.

It is also, in a sense this lesson will try to make precise, the source of most
of the difficulty in programming.

## What changes cost you

Consider reading an unfamiliar program and reaching this line:

```java
System.out.println(total);
```

What is `total`? To answer, you must find its declaration, and then find every
assignment to it, and then work out which of those assignments last executed
before this line — which depends on the conditions and loops in between.

Now suppose `total` had been `final`. It was assigned once, at its declaration,
and cannot have changed. Finding the declaration answers the question completely.

That difference is the cost of mutation, and it scales badly. A variable assigned
in five places has five possible histories at any point. Two such variables have
twenty-five combinations. This is the state-space explosion of Chapter 6,
arriving in the specific form of "how hard is this code to read".

## The rule that follows

**Prefer variables that do not change, and when they must change, keep the
changes close together.**

Concretely:

```java
// harder to follow
int result = 0;
result = compute(a);
doSomethingElse();
result = result + adjust(b);
moreCode();
result = result * 2;
return result;
```

```java
// easier
final int base = compute(a);
final int adjusted = base + adjust(b);
return adjusted * 2;
```

The second version has more names and no mutation. Each name means one thing
forever, so a reader can understand any line without tracing execution.

This is not a rule against mutation — you will write loops that accumulate, and
they are correct and clear. It is a rule about *scattered* mutation, where a
variable's value depends on which of several distant lines ran last.

## The accumulator pattern

The legitimate case, which you will use constantly:

```java
int total = 0;
for (int i = 1; i <= 5; i++) {
    total += i;
}
System.out.println(total);      // 15
```

`total` changes five times, and this is fine, because all the changes are in one
place and they follow one rule. You can describe the variable's meaning in a
single sentence — *the sum of the numbers considered so far* — and that sentence
is true at every iteration.

That sentence is a **loop invariant**, and it is the subject of Chapter 9. Note
what it does: it replaces "trace the execution" with "check that one claim
survives each step". The mutation is still there, and it has stopped being a
reasoning problem.

## Aliasing, previewed

The hazard that this chapter cannot yet show you, and which I want you to be
waiting for.

For primitives, `y = x` copies a value, and afterwards the two are unrelated. We
checked this earlier: `x` changed and `y` did not.

For objects, `y = x` copies a *reference*, and both names then refer to the same
object. Changing the object through `x` changes what you see through `y`, because
there is only one object.

This means two pieces of code with no apparent connection can affect each other,
and it is genuinely hard to reason about. It is the single most common source of
serious confusion for people moving from primitives to objects, and Chapter 20 is
devoted to it.

Everything in this chapter is true. Some of it is true *only of primitives*, and
I have tried to mark which. When you reach Unit V, come back to this chapter and
re-read it with references in mind; a surprising amount of it changes.

## Where this leaves us

A variable is a named cell of state. Assignment is a transition that overwrites
it. Scope limits where the name is visible, which limits how much a reader must
track. Mutation is what makes computation possible and what makes programs hard
to follow, and the tension between those is managed rather than resolved — by
narrow scopes, by `final`, and from Chapter 9 onwards by invariants.

The next chapter takes up the other half of Chapter 6's model. We have transitions
that change state; now we need transitions that *consult* it, and choose what to
do next.
