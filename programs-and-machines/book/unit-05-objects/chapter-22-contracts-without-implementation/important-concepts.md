# Important Concepts

**Interface** — a named set of method signatures with no implementation. A class
declares `implements` and the compiler enforces that every method is supplied.

**Multiple interfaces, one superclass** — a class may implement any number of
interfaces because interfaces carry no state, so there is nothing to conflict.
Multiple inheritance of contract is safe where multiple inheritance of
implementation is not.

**An interface is for callers** — it is a type invented so that code can be
written against a promise. `String` and `LocalDate` share `Comparable` and nothing
else, and `Collections.sort` works on both.

**Implicit modifiers** — interface methods are public and abstract by default;
interface fields are public static final, which is a reason not to declare them.

**Default methods** — interface methods with a body, added in Java 8 so that
`List` could gain `sort` and `forEach` without breaking every existing
implementation. They may call the abstract methods, so they can supply real
behavior derived from the contract.

**Static methods on interfaces** — useful for factories, often returning an
anonymous class.

**Anonymous class** — a class declared and instantiated in one expression, with no
name. The pre-Java-8 way to supply a one-off implementation; Chapter 26 shows the
short form.

**invokeinterface** — the call instruction for interface methods. A class may
implement any combination of interfaces, so no single method-table layout works
and the JVM must search rather than index. Call-site caching makes the measured
cost negligible.

**Abstract class** — a class that cannot be instantiated, with some methods
implemented and some left abstract. It has state and a constructor, which is the
real difference from an interface.

**Template method** — a parent implementing an algorithm and calling abstract
steps the subclass supplies. The order lives in one place and the subclass cannot
get it wrong.

**Interface or abstract class** — interface when unrelated things share a
capability, abstract class when implementations share state and are genuinely
kinds of one thing. When unsure, the interface constrains less.

**Programming to an interface** — declaring variables, parameters and return types
by the weakest type that serves the caller, so the implementation can change and
callers cannot depend on what was not promised. Chapter 17's `List<String> names
= new ArrayList<>()`, explained.

**Enum** — a type with a fixed, closed set of instances, created once at class
load. Carries `values()`, `ordinal()`, `name()`, `valueOf`, a readable
`toString`, and an ordering.

**Enums and ==** — constants are singletons and cannot be duplicated, so reference
equality is value equality. Faster, null-safe, and type-checked, where `equals`
across two enum types silently returns false.

**Exhaustive switch** — a `switch` over an enum needs no `default` when all
constants are covered, so adding a constant becomes a compile error listing every
place to update. Writing `default` throws that check away.

**Enums with state** — constants may take constructor arguments and override
methods individually, which puts per-case data and behavior with the case rather
than in a parallel table.

**EnumSet and EnumMap** — a set of enum constants as a bit field in a single
`long`, and a map as an array indexed by ordinal. Substantially faster than the
hash-based versions and iterating in declaration order.

**When an enum is wrong** — when the set is not really closed. If the values could
change without the logic changing, they are data.

**Record** — a class that is exactly its components. The compiler generates the
private final fields, canonical constructor, accessors, `equals`, `hashCode`, and
`toString`.

**Records are immutable and final** — no setters can be added, the class cannot be
extended, and it may implement interfaces.

**Compact constructor** — a record constructor with no parameter list, for
validation and normalization; the assignments are added after your body runs. It
means a record can enforce an invariant, and immutability makes that invariant
permanent.

**When to use a record** — when two instances with equal contents would be
interchangeable. When they would not, the type has identity and needs a class.

**Restriction as a feature** — enums and records give up openness and get compiler
guarantees in exchange. Interfaces do the opposite, promising less to stay open.
Both are ways of moving work to the compiler.

**Sealed interfaces and algebraic data types** — a closed set of implementations,
each a record, switched over with exhaustiveness checked. Chapter 24 returns to
the idea.
