# The Empty Set

The empty set — the set with no members, usually written {} or ∅ — is one of the most philosophically provocative entities in mathematics. Its existence seems to be both trivially obvious (of course there is a set with no members) and deeply puzzling (what exactly is it? What makes it a "set" if it has no members to collect?).

The standard axiom of ZF set theory that posits the empty set asserts that there is at least one set that contains nothing. From this and the other axioms, the rest of the cumulative hierarchy can be built up. The empty set is, in a sense, the ultimate foundation of all set-theoretic mathematics — it is the one given from which everything else is constructed.

Philosophically, the empty set raises several questions. First, what individuates it? Since sets are usually individuated by their members (the axiom of extensionality: two sets are identical iff they have the same members), and the empty set has no members, it seems like a limiting case. But it works: any two empty sets must have the same members (vacuously), so they are identical. There is exactly one empty set.

Second, why does it exist? Platonists say the empty set exists because it is a genuine mathematical object — the collection of nothing — and mathematical objects exist necessarily. Nominalists who want to avoid abstract objects find the empty set particularly challenging: what concrete thing could the empty set be?

Third, the empty set enables a famous construction of the natural numbers. Von Neumann defined 0 as the empty set, 1 as the set containing the empty set, 2 as the set containing both, and so on. Each natural number is a set of all smaller natural numbers. The elegance of this construction is remarkable — the entire structure of arithmetic grows from nothing.

## The Ontological Puzzle

The empty set is philosophically puzzling in a way that ordinary sets are not. When we think about the set {the moon, Mars}, we can understand it as a collection — two physical objects gathered together into a single abstract unit. The set has something to "collect."

But the empty set collects nothing. It is the collection of no objects. What is left when you remove everything that might be in a collection? Is there anything at all? Frege worried about this: if sets are extensions of concepts, the empty set should be the extension of a concept with no instances. The concept "is both round and square" has no instances, so ∅ = {x : x is round and square}. This shows that the empty set is not nothing — it is a genuine set, the extension of an unsatisfied concept.

**The nothing problem**: Can there be a collection of nothing? Our intuitive concept of a set comes from gathering things together. If there are no things to gather, the gathering operation produces... nothing? Or something (the empty set)?

The Axiom of Extensionality saves us from confusion: two sets are identical iff they have the same members. Since there can be at most one set with no members (any two empty sets are identical by extensionality), the empty set is unique. Its existence follows from the set-theoretic axioms. Its uniqueness is secured by extensionality. Whether it exists *metaphysically* is a further question that the axioms alone cannot settle.

## The Empty Set and Ontological Dependence

A recurring issue in the metaphysics of sets is whether sets are *ontologically dependent* on their members — whether the existence of a set requires the existence of its members. For ordinary sets, this seems plausible: the set {Mars, Jupiter} depends for its existence on Mars and Jupiter. If those planets did not exist, neither would the set.

But if sets are ontologically dependent on their members, the empty set creates a special puzzle: the empty set has no members. On what does it depend? Three options:

1. **The empty set is self-subsistent**: It exists necessarily, without depending on any other entity. It is the foundational abstract object from which all others are built. This is the Platonist view: the empty set is a necessary existent.

2. **The empty set depends on logic or pure possibility**: The empty set exists because the concept of a collection of nothing is logically coherent. Its existence is guaranteed by logical necessity rather than by any particular object or collection of objects.

3. **The empty set does not exist**: Strict nominalists might deny the empty set along with all other mathematical objects. Set theory, including the axiom asserting the empty set, is on this view a useful but literally false framework.

## The Constructive Role of the Empty Set

Despite its philosophical puzzles, the empty set plays an indispensable constructive role in mathematics. Von Neumann's construction of the natural numbers begins with the empty set and builds everything else:

- 0 = ∅
- 1 = {0} = {∅}
- 2 = {0, 1} = {∅, {∅}}
- 3 = {0, 1, 2} = {∅, {∅}, {∅, {∅}}}
- n+1 = n ∪ {n}

Each natural number is identified with the set of all smaller natural numbers. The empty set is 0 — the number from which counting begins. The entire structure of arithmetic — addition, multiplication, ordering, induction — is then derived from purely set-theoretic operations applied to these constructions.

This construction demonstrates something remarkable: from the single assumption that the empty set exists (and from the standard set-theoretic operations), we can derive the entire structure of arithmetic. The empty set is not merely a curiosity but a mathematical foundation — the one primitive entity from which all of number theory grows.

**The philosophical lesson**: If this construction is taken seriously as an ontological reduction (numbers *are* sets), then the existence of arithmetic depends only on the existence of the empty set. But Benacerraf's argument applies here too: the Zermelo construction (which identifies 2 with {{∅}} rather than {∅, {∅}}) is equally valid. The empty set is the unique foundation for both constructions, but which construction gives the "real" natural numbers is undetermined.
