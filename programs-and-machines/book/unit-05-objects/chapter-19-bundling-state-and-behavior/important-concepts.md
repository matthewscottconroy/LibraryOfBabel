# Key Concepts

**What a class is for.** Putting state and its operations in one unit and making
the state unreachable from outside, so the code that could break an invariant is
small enough to check. Everything else a class does is secondary.

**Class versus object.** The class is a description, written once. An object is a
particular thing built to it, living on the heap with its own copy of the fields.
One class, many objects.

**An object is a small machine.** Chapter 6's state plus transitions, at a scale
you choose. `new` allocates, initializes, runs the constructor, and returns a
reference.

**`this`.** The object the method was called on. `this.x = x` in a constructor
distinguishes field from parameter; without it, `x = x` assigns the parameter to
itself and leaves the field null.

**Fields default; that is the problem.** A field is 0, `false`, or `null` before a
constructor runs, so an object can exist satisfying nothing — and a
half-initialized object is indistinguishable from a real one until it misbehaves.

**A constructor's job is to establish the invariant**, so that a badly-formed
object cannot exist anywhere in the program. Not "we try not to create one" —
there is no way to obtain one. Chapter 11's fail-fast argument at the boundary
where objects come into being.

**Constructor rules.** The class's name, no return type — giving it `void` makes
it a method that `new` never calls. Writing any constructor removes the default
no-argument one. `this(...)` delegates to another constructor and must be first,
which keeps validation in one place.

**`final` fields** are assigned exactly once, in the constructor. `final` on a
reference stops reassignment, not modification of what it points at.

**Every method preserves the invariant.** Establishment is the constructor's job;
preservation is every method's. This is Chapter 9 and Chapter 16's obligation with
somewhere to live.

**Throw versus return.** Throw for what should never happen — a negative deposit
is a programming error. Return for what might reasonably happen — an overdraft is
an answer, not an error.

**Getters and setters written mechanically undo encapsulation.** `setCents(-500)`
does exactly what direct field access would. Ask whether the accessor corresponds
to something a caller actually wants to do: `deposit` does, `setBalance` does not.

**Tell, don't ask.** Ask the object to do the thing rather than asking for its
state and deciding elsewhere. Keeps the rules inside the boundary that owns the
data, so a change to the rules is one edit.

**`private` is a reasoning tool, not a security one.** Reflection reaches around
it and anyone can edit the source. What it does is stop an *ordinary reader* from
having to consider the possibility — it makes Chapter 16's preservation check
finite.

**Four access levels.** `private`; package-private (no modifier); `protected`,
which also grants package access and exposes internals to every future subclass;
and `public`. Make everything as private as it can be — widening later is
harmless, narrowing breaks everyone.

**The public surface is a set of promises**, and larger than the list of methods:
whatever people can observe, they come to depend on.

**Design the surface from the caller.** Ask what a caller should be able to *do*,
name operations in the domain, and expose the smallest set that does the job.

**Never return your internals.** Returning the actual list lets a caller clear it.
Return `List.copyOf`, an unmodifiable view, or a copy — the first is usually
right.

**`toString`** is for humans reading diagnostics. Without it, printing gives a
class name and a hash code. `@Override` is optional and worth always writing,
because it catches a misspelling.

**`static` belongs to the class, not to an object.** One field shared by all
instances, existing whether any instance does or not.

**A static method cannot see instance fields** because it is not called on any
object, so there is no object whose field it could mean.

**When static is right:** pure utility methods, constants (`static final`), and
factory methods that can have a name and can return a cached instance.

**When static is wrong:** mutable static state is a global variable — every part
of the program can change it, nothing records who did, and concurrency makes it a
minefield. Static methods also cannot be overridden or substituted for testing.

**`public static void main(String[] args)`, explained.** `public` because the JVM
is outside your class; **`static` because no objects exist yet when the JVM
starts, so something must be callable before anything has been created**; `void`
because no caller in your program wants a value; `main` because the specification
says so; `String[] args` because the person starting the program may have
something to say to it.
