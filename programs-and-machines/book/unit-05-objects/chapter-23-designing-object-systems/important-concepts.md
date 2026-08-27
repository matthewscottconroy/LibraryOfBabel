# Important Concepts

**Design is judged by change** — a badly designed program runs as fast, passes the
same tests, and produces the same output. The difference appears only when a
requirement changes, so the question is always *what happens then*.

**Responsibility** — deciding which class should know a given fact. The bulk of
design is a long sequence of small decisions of this form.

**Behavior belongs with its data** — if a computation reads only one object's
state, it is that object's method. Encapsulation as a design rule rather than an
access modifier.

**A getter for every field is not encapsulation** — a class of fields and getters
with the logic elsewhere has exposed its representation as thoroughly as public
fields would.

**When the heuristic runs out** — a computation needing two objects usually wants a
third class; an operation about a format rather than the domain belongs outside.
Test: would this method still make sense if the output format changed?

**Handing out a collection** — a getter returning the internal collection gives
away the ability to break the invariant. `List.copyOf` returns an immutable
snapshot; better still, expose the operations instead of the collection.

**CRC cards** — one index card per class, with responsibilities on the left and
collaborators on the right. A class whose responsibilities do not fit is doing too
much, and moving one is physically moving a line.

**Composition over inheritance** — prefer holding an object in a field to
extending its class. The central design argument of the unit.

**The counting-set failure** — `CountingHashSet extends HashSet` reports 6 for
three elements, because `HashSet.addAll` is implemented by calling `add`. Nobody
made a mistake; inheritance exposed an undocumented internal call.

**Internal calls become your contract** — extending a class couples you not to what
it promises but to how it is built, and how it is built may change.

**The composition pattern in full** — inherit the type from an interface, and get
the implementation from a field. It is what the standard library's wrappers do.

**What composition costs** — you must forward methods by hand, and Java has no
delegation keyword. That is the real price and it is worth paying.

**Run-time versus compile-time choice** — inheritance fixes a relationship when the
class is written; composition lets it be chosen, swapped, or injected later.

**Design and document for inheritance, or prohibit it** — Bloch's rule. A class
that is neither has subclasses that break for reasons nobody chose.

**Cohesion** — how strongly one unit's parts belong together. Tested by whether
you can describe the class in one sentence without "and", and by whether the
fields cluster.

**A class named for a layer** — `Manager`, `Handler`, `Processor`, `Utils` — means
the author could not say what it is, and reliably predicts unrelated contents.

**Coupling, weakest to strongest** — data, interface, concrete, construction,
inheritance, global. Use the weakest that does the job.

**Construction coupling and dependency injection** — a class calling `new` on a
collaborator depends on how to build it and cannot be given a substitute. Passing
the object in is the whole idea.

**How far does a change propagate** — the measurement the two principles are
proxies for. Name three likely changes, trace each, count files.

**UML as sketch** — hand-drawn, before the code, only for the part you are unsure
of, thrown away afterwards. Diagrams that must be kept current are a second copy
of the code.

**Class diagram** — a box per class with fields and methods; lines for uses,
aggregation, composition, and inheritance. The multiplicities carry more of the
thinking than the arrowheads.

**Sequence diagram** — objects across the top, time down, arrows for calls. For
behavior spanning several objects.

**Naming the nouns** — the first pass of a design. Some nouns become classes, some
become enums, some become fields, and some turn out not to exist yet.

**Strategy** — an algorithm as an object, chosen by whoever constructs the class
that uses it. The `Scheduler` interface, and the pattern most worth learning
early.

**Separating content from state about content** — a `Card` is immutable content; a
`Progress` is one student's history with it. Keeping them apart is what makes
multiple students a small change.

**Do not read the clock inside a class** — take the time as a parameter. A class
that calls `now()` cannot be tested without waiting, and time is the commonest
source of untestable code.

**Speculative generality** — adding structure against a change nobody has asked
for. It has a real cost, and choosing which changes to make cheap is the judgment
that design consists of.
