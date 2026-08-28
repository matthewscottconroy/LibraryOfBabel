# Programming to an Interface

There is a line you have been writing since Chapter 17 without being told why:

```java
List<String> names = new ArrayList<>();
```

Two different type names for one object, which looks like an inconsistency a
careful person would want to remove. The argument for it is now available, and it
generalises well past collections.

Chapter 17 gave a rule without an argument:

```java
List<String> names = new ArrayList<>();       // yes
ArrayList<String> names = new ArrayList<>();  // no
```

The argument is now available.

The declared type is what the rest of your code sees; the constructor is a
decision about representation. Writing `List` on the left says: this variable
holds something that behaves like a list, and nothing downstream may assume more.

## What it buys

**You can change the implementation.** If profiling later says a `LinkedList`
suits the access pattern — Chapter 17 measured a case where it did not, but such
cases exist — you change one line. With `ArrayList` on the left, you change one
line and then discover every place that used an `ArrayList`-only method.

**Callers cannot depend on what you did not promise.** This is Chapter 19's
encapsulation applied to types rather than fields. `ArrayList` has methods `List`
does not — `ensureCapacity`, `trimToSize`. Declaring the interface makes them
unreachable, so no caller comes to rely on them, so you stay free.

**It states an intent.** `List` says *ordered, indexed, duplicates allowed*.
`ArrayList` says that plus *stored in an array*, which is a fact about the
implementation that the reader now has to decide whether to care about.

The same reasoning applies with more force to parameters and return types:

```java
void process(List<String> items)             // yes
void process(ArrayList<String> items)        // no — refuses List.of, Arrays.asList
```

A method that demands `ArrayList` rejects perfectly good lists for no reason. On
a parameter, take the weakest type that will do the job.

## Where the rule stops

Three honest exceptions.

**When the implementation is the point.** If you need `ArrayDeque`'s stack and
queue operations both, and `Deque` does not name what you want, say what you mean.

**Local variables in short methods.** A three-line method that builds a list and
returns it is not going to surprise anyone. `var` is often the right answer there
and it sidesteps the question.

**Return types deserve thought, not reflex.** Returning `List` is right. Returning
`Collection` when every caller wants indexing has weakened the contract for no
gain — you did not make yourself free, you made your callers cast. The rule is the
*weakest type that serves the caller*, which is not always the weakest type.

## The general principle

"Program to an interface, not an implementation" comes from the 1994 design
patterns book, and it is broader than variable declarations. It is the same idea
this book has been circling since Chapter 1: define what a thing does, keep how it
does it separate, and let the second change without disturbing the first.

Chapter 1 called an encoding an agreement. Chapter 11 said a method's name is a
promise and its body is nobody's business. Chapter 19 put a boundary around a
class. This is that idea applied to the type system, and it is the last form of
it before Chapter 23 asks how to design a system out of these parts.

The counterweight, since this book has tried to give both sides: interfaces are
not free. Every one is a type to name, understand and keep current, and an
interface with exactly one implementation that nobody else will ever write is
ceremony. The test is whether the abstraction is real — whether there could
plausibly be another implementation, or a test double, or a caller you want to
keep at arm's length. If there could not, a class is a fine thing to be.

Next: two constructs that get their power from restriction rather than
abstraction.
