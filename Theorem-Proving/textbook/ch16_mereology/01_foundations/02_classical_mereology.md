# Classical Mereology

Classical Mereology (CM) — also called General Extensional Mereology (GEM) — extends ground mereology with the axiom of *unrestricted fusion* (composition).

## Unrestricted Fusion

**Axiom (Fusion)**: For any non-empty collection of objects (specified by a condition φ), there exists a *fusion* — an object that has exactly those objects as parts, and is part of anything that has all of them as parts.

```
∃x φ(x) → ∃y [∀x(φ(x) → P(x, y)) ∧ ∀z(∀x(φ(x) → P(x, z)) → P(y, z))]
```

The fusion of all φ-things is their *mereological sum* — the "smallest" thing containing all of them.

## Consequences

1. **Closure under arbitrary composition**: Fuse any objects — a scattered object is perfectly legitimate. The fusion of Napoleon's left hand and the Eiffel Tower exists as a mereological entity.

2. **Unique fusions**: Each condition determines at most one fusion (by extensionality).

3. **Lattice structure**: GEM gives objects the structure of a *complete Boolean algebra* (minus the bottom element, since empty fusions are typically excluded): every non-empty set of objects has a unique fusion (join) and every pair with a common part has an intersection (meet).

4. **Atoms and atomlessness**:
   - *Atoms* are objects with no proper parts.
   - *Atomism*: every object is a fusion of atoms.
   - *Atomlessness*: every object has a proper part (gunk — every object is infinitely divisible).
   - *Hybrid*: some atoms, some gunky regions.

## Relationship to Set Theory

Classical mereology and set theory have different ontological profiles:

| | Set Theory | Mereology |
|---|------------|-----------|
| Primitive | ∈ (membership) | P (parthood) |
| Abstract objects | Yes (sets are abstract) | Not necessarily |
| Empty object | ∅ | Typically excluded |
| Singleton | {a} ≠ a | No distinction (part of = the individual) |
| Extensionality | Same members = same set | Same parts = same object |
| Iteration | Sets of sets of ... | No iteration (fusion of fusions = fusion) |

A key difference: the singleton {a} is distinct from a in set theory, but in mereology, there is no singleton — an object just is its parts, without an additional layer.

## Philosophical Applications

Mereology is the natural framework for:
- **Physical objects**: Parts of physical things are physical things.
- **Regions of spacetime**: Relativity physics quantifies over spacetime regions.
- **Constitution**: The statue/clay problem — do they share all parts? (Yes in GEM, not in non-extensional alternatives.)
- **Vagueness**: Fuzzy mereology allows degrees of parthood for vague predicates like "the cloud."

David Lewis used mereology extensively in his ontology of possible worlds (in *Parts of Classes*, 1991), arguing that set theory can be grounded in mereology plus a singleton function.
