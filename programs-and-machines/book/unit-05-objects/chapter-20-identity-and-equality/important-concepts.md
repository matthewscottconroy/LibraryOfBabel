# Key Concepts

**Two questions English confuses.** *Identity* — are these the same object, one
thing or two? *Equality* — do these count as the same for my purposes? Java
answers the first with `==` and the second with `.equals`.

**The right answer depends on the domain.** Two banknotes with one serial number
are a forgery; two £5 notes are interchangeable. The language cannot know which
you mean.

**Assignment copies the contents of the variable** — the value for a primitive,
the reference for an object. One rule, two very different-looking outcomes.

**Why objects are not copied on assignment.** Copying is expensive, usually
unwanted, and ambiguous — there is no answer the language could give about how
deep to go.

**Aliasing.** Two names for one object. Usually what you want: an account passed
to three subsystems should be the same account. The trouble is the *unintended*
case, where a caller believes they hold their own copy.

**Where aliases come from.** Assignment; argument passing; returning an internal;
storing in a collection.

**Defensive copying** at both boundaries — copy what comes in, copy what goes out
— or make the type immutable, which removes the problem rather than defending
against it.

**Three depths.** A reference copy is not a copy. A shallow copy makes a new
object whose reference fields point at the same things. A deep copy recurses, and
you must write it.

**Every copying facility in Java is shallow** — `clone`, `Arrays.copyOf`,
`new ArrayList<>(other)` — because deep copying is not well defined: how deep, and
what about cycles?

**Immutable fields need no copying.** Noticing which of your fields are immutable
is most of the work of writing a correct copy.

**Avoid `clone`.** `Cloneable` declares no methods, the default is shallow, and it
creates objects without running a constructor. Use a copy constructor or a static
factory.

**`Object.equals` compares references**, so by default equality *is* identity. If
you want anything else you must say so.

**Writing `equals`:** identity check, `instanceof` with pattern matching, then
compare the fields that matter. The parameter must be `Object` — `equals(Point o)`
is an *overload*, not an override, and the collections will call the inherited
version. `@Override` turns that silent bug into a compile error.

**Which fields count** is a domain decision. The test: would replacing one with
the other be acceptable everywhere?

**Not every class should define it.** Value objects should. Entities with identity
usually should not, or should compare only an identifier. Threads, windows and
connections should not — two connections are never the same connection.

**`equals` contract:** reflexive, symmetric, transitive, consistent, and false for
`null`.

**`hashCode` contract:** equal objects **must** have equal hash codes; unequal
objects need not differ but should. Equal implies same hash; same hash does not
imply equal, and a collision is handled by comparing with `equals`.

**Define `equals` without `hashCode` and hash collections silently fail** — a
`HashSet` containing two equal elements, a `HashMap` that cannot find a key it
holds.

**Use the same fields in both**, and write them adjacent so adding a field updates
each. `Objects.hash(...)` does the combining. `return 0;` is legal and turns
constant-time lookup into a linear scan.

**Never mutate a hash key.** The entry was filed by its hash at insertion; change
the hash and the lookup goes to a different bucket. The entry remains, permanently
unreachable — it cannot be retrieved or removed.

**Records** generate `equals`, `hashCode` and `toString` from the components. For
a value object that is the right answer, and writing them by hand should be a
choice you have a reason for.

**Immutability removes the causes rather than mitigating them.** Aliasing becomes
harmless, defensive copying unnecessary, hash keys permanently safe, and thread
safety free.

**Five rules:** no mutators; all fields `private final`; the class `final`;
mutable components defensively copied both ways; no reference to a mutable
internal escapes. The fourth is the one people miss — a `final` field holding an
`ArrayList` is not immutable.

**The cost is allocation**, which is why the immutable-type-with-mutable-builder
pattern exists. `String` and `StringBuilder` are exactly that.

**The default should be immutable**, with mutability justified. Roughly: entities
have identity and change over time; values are defined by what they hold. Values
should be immutable, and most classes you write are values.
