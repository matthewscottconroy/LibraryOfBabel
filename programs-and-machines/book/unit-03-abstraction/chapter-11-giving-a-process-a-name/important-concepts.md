# Key Concepts

**Abstraction.** Hiding detail behind a name so that the detail no longer needs to
be considered. The last clause is what distinguishes abstraction from
concealment: if you still have to think about the hidden part, nothing was
gained.

**What a method buys.** Reuse (least important); a unit of thought, so a reader
reads a word instead of executing four lines mentally; a single place to be
correct; and a reasoning boundary, since the internals become unreachable rather
than merely inadvisable to touch.

**The name is the abstraction.** `process()` yields none of the benefits except
reuse, because the reader must still go and look. Naming is not labelling
something you built; it is the point of building it.

**The one-sentence test.** If you cannot say what a method does in a sentence
without "and", it probably does two things.

**The costs.** Indirection on every lookup; a *wrong* abstraction is worse than
none, because the reader must carry the name and its exceptions; and too many
layers can be harder to follow than the flat version.

**Parameter versus argument.** The parameter is the name in the definition; the
argument is the value at the call site.

**A parameter is a local variable** initialized with a copy of the argument. It
lives for the duration of the call. Reassigning it affects nothing outside — Java
passes by value.

**Positional matching.** Arguments bind to parameters by position, and Java has no
named arguments. Defenses: fewer parameters, distinct types, consistent ordering.
Zero to two is comfortable; five or more is a signal that the method should be
split or that some parameters want to become one object.

**Varargs.** `int... xs` accepts any number of arguments and delivers them as an
array. For genuinely open-ended counts, not for avoiding a decision.

**Return type and `return`.** The declared type is a promise about what comes out.
`return` supplies the value *and ends the method immediately*, which is what makes
guard clauses work and how you escape a nested loop cleanly.

**`void`.** No value produced. `return;` may still be used to leave early.

**Compute versus cause.** A **pure** method has no effect beyond its return value:
it can be called any number of times, moved, removed, or reordered freely. Methods
with effects can do none of these. Prefer pure; when a method must have an effect,
do not also make it compute something interesting.

**Returning one thing.** Java returns a single value. For two results: an object or
`record` (usually best), an array (positional problems again), output parameters
(worst — the effect is invisible at the call site), or two methods.

**Naming conventions.** `boolean` methods read as `isValid`, `hasNext`. Methods
returning a value are named for the value; methods causing an effect are named for
the action. A `getBalance` that opens a network connection betrays every reader.

**Signature.** The first line is a summary of the contract: what goes in, what
comes out. What it omits — which inputs are acceptable, what is guaranteed,
whether arguments are modified — is the rest of the contract.

**Precondition and postcondition.** What the method requires of its caller; what it
guarantees in return. Both sides have obligations and may rely on the other's.

**A precondition is a way of not handling a case.** Declaring that the array must
be non-empty excuses the method from deciding what "the largest of nothing" means.

**Strong versus weak preconditions.** Strong demands more, keeps the method simple
and fast, and burdens every caller. Weak demands little and moves the work inside,
once. Decide by the number of callers, the cost of checking, and how bad a silent
wrong answer would be.

**Three scales of invariant.** Loop invariants (Chapter 9), method contracts (here),
and representation invariants (Unit IV). One technique: state what stays true, and
check that each step preserves it.

**The compiler checks none of it.** Contracts are claims in comments, and a wrong
comment is worse than none because it is believed. Relies on care, explicit
checks, and tests.

**Responding to a violated precondition.** Undefined behavior (cheap, confusing);
check and throw (clear, costs a little); a sentinel (dangerous when it could pass
as a real answer); or widen the contract and document it.

**Fail fast.** The distance between a mistake and its symptom is the cost of the
bug. A bad value caught at entry is a two-minute fix; the same value found three
subsystems later is an afternoon.

**Assertions.** `assert` is for your own beliefs about your own code, and is
disabled by default — so it cannot validate input from outside. Use `if` and
`throw` for external input; `assert` for internal invariants.

**Trust is the point.** A method is worth having when you can call it without
reading it. That needs an honest name, an informative signature, and a stated
contract. When any of the three fails, the abstraction leaks and you are carrying
more than before.
