# Important Concepts

**Inheritance** — declaring that one class is a specialized kind of another with
`extends`, so that the subclass has the superclass's members and may add its own.

**Superclass and subclass** — the general type and the specific one. Java permits a
single superclass; Chapter 22 shows how interfaces relax that.

**Subclasses and private members** — a subclass's code cannot name a superclass's
private fields, though the object contains them. Encapsulation holds against
subclasses too.

**Constructor chaining** — every constructor invokes a superclass constructor,
explicitly with `super(...)` or implicitly with `super()`, before its own body
runs. The inherited part is established first.

**final class, final method** — forbidding extension or overriding. The rule is to
design for extension deliberately or forbid it; `String` is final so that no
subclass can lie about its length.

**Overriding** — replacing an inherited method with one of the same signature.
Resolved by the object's actual type at run time.

**Overloading versus overriding** — overloading varies the parameters and is
resolved by the compiler from the declared type; overriding varies the class and is
resolved by the JVM from the actual type. `describe(c)` printing `a shape` while
`c.area()` runs `Circle.area` is the two rules in one line.

**@Override** — an annotation asserting that a method replaces an inherited one.
The compiler rejects it if it does not, which turns the `equals(Point)` mistake
from Chapter 20 into an error.

**super** — calling the superclass's version of a method, so behavior can be
extended rather than replaced.

**Shadowing** — declaring a field with the same name as an inherited one. It adds a
second field rather than replacing the first, and which you see depends on the
declared type. Private fields make it impossible.

**The constructor trap** — a superclass constructor that calls an overridable
method runs the subclass's version before the subclass's fields exist. Constructors
should call only private, static, or final methods.

**Object** — the root of the class hierarchy. Every class inherits `toString`,
`equals`, `hashCode`, and `getClass`; three of those are meant to be replaced.

**getClass versus instanceof** — `getClass()` gives the exact runtime class,
`instanceof` is true for subclasses too. Both are worth suspecting: branching on
type is usually a method that was not written.

**Polymorphism** — one call site running different code depending on the receiver's
actual type. It replaces `instanceof` chains, and it lets code work on types that
did not exist when it was written.

**Subtype, parametric, and ad-hoc polymorphism** — inheritance, generics, and
overloading respectively. The first two compose: `List<Shape>`.

**Dynamic dispatch** — the runtime mechanism. Each class holds a method table laid
out so that inherited slots keep their positions, so the compiler can fix a slot
number that stays valid for every future subclass.

**invokestatic, invokespecial, invokevirtual, invokeinterface** — the JVM's four
call instructions. The first two are direct, which is why private and static
methods are safe to call from a constructor.

**Monomorphic and megamorphic call sites** — a site that sees one implementing
class can be devirtualized and inlined by the JIT; one that sees many falls back to
the table. Measured here at 1.4 against 2.1 nanoseconds per call.

**Virtual by default** — Java dispatches every instance method dynamically unless
it is private, static, or final. C++ chose the opposite default.

**The Liskov substitution principle** — anything a caller could prove about the
supertype must remain true of every subtype. The test for whether `extends` is
honest, and the thing the compiler cannot check.

**The four obligations** — a subclass may not strengthen preconditions, weaken
postconditions, break invariants, or permit state changes the supertype forbade.

**The square-rectangle problem** — `Sq extends Rect` compiles, keeps its own
invariant, and still makes a correct caller compute 25 where 20 was guaranteed. The
violation is mutability, not geometry.

**Stack extends Vector** — a substitution violation in the standard library,
retained for compatibility. `ArrayDeque` is the composition-based replacement.
