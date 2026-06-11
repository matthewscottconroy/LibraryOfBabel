# Domain Survey: Mereology

## Overview

Mereology — the formal theory of parts and wholes — is metaphysics at its most rigorous. It provides the logical scaffolding for questions that span the entire range of the discipline: when do things compose a further thing? What is it for one thing to be a part of another? Can two distinct things share all their parts? How do objects persist through changes in their material composition? These questions are not merely technical puzzles in formal ontology; they have immediate consequences for the metaphysics of material objects, the theory of personal identity, and the ontology of ordinary middle-sized objects.

The discipline has formal roots in Stanisław Leśniewski's *Foundations of Mathematics* (1916–1931), where he developed mereology as an alternative to set theory that did not posit abstract entities. Leśniewski's system was developed and popularized by Leonard and Goodman (1940) in their "Calculus of Individuals," and later systematized by Peter Simons (*Parts: A Study in Ontology*, 1987) in the most comprehensive philosophical treatment of mereological systems. Contemporary mereology in analytic metaphysics has been shaped above all by David Lewis (on modal questions about parts), Peter van Inwagen (*Material Beings*, 1990, on the Special Composition Question), and Ted Sider (*Four-Dimensionalism*, 2001, on temporal parts and the formal theory of composition).

The **Special Composition Question** (van Inwagen): Under what conditions do some objects compose a further object? The three most defensible positions mark the extremes and a restricted middle ground. **Mereological universalism** (or "unrestricted composition") holds that for any plurality of objects, there is a further object — their mereological sum. **Mereological nihilism** holds that no plurality of objects ever composes a further object; only simple, partless objects (simples or "atoms") exist. **Restricted composition** holds that some but not all pluralities compose objects, and the task is to specify the conditions. Van Inwagen's own "life" criterion — that composition occurs only when the activity of the composing things constitutes a life — is the most influential restricted view, but it has counterintuitive consequences for artifacts and non-living organic structures.

The **constitution problem** — whether a statue and the clay it is made of are numerically distinct despite occupying the same spatial region — is a mereological puzzle because both objects appear to be composed of the same parts. If mereological identity (same parts ↔ same object) is correct, they must be identical. But Leibniz's Law arguments show that they have different modal properties, suggesting they are distinct. The four-dimensionalist strategy — that "statue" and "clay" pick out distinct four-dimensional objects sharing a temporal stage — is the dominant resolution, though the constitution view (Baker) and sortal-relative identity (Wiggins) are serious alternatives.

---

## Major Positions and Their Logical Relations

### The Special Composition Question

Van Inwagen's taxonomy of answers to "when do some objects *x*s compose something?":

**Nihilism**: Never. There is a composed object for no plurality. Only simples exist. The word "table" does not refer to a composite object but to nothing at all, or to a plurality of simples arranged table-wise. (Van Inwagen 1990 half-endorses this for artifacts while accepting composition for organisms.)

**Universalism** (Lewis, Sider): Always. For any plurality of objects (however scattered, however gerrymandered), there is a composed object that is their mereological sum. The sum of my nose and the Eiffel Tower is a legitimate object.
- *Advantages*: simplicity, no principled vagueness about composition, no need to specify when exactly something comes into existence.
- *Problem*: ontological extravagance; commitment to coincident objects; gerrymandered "objects" that seem metaphysically inert.

**Van Inwagen's organicism**: Composition occurs if and only if the activities of the *x*s constitute a life. Living organisms are the only composite objects; tables, chairs, rocks, and artefacts generally are not composite objects (though simples may be arranged to constitute them).
- *Problem*: this seems to have no principled metaphysical grounding — why should "constituting a life" be the privileged criterion?

**Contact, fastening, cohesion** (various): Composition occurs when the components are in contact, or physically fastened, or cohesive. *Problem*: vagueness about when contact/fastening/cohesion is sufficient; two things in loose contact don't seem to compose a further thing.

**Restricted composition with indeterminate boundaries**: Whether some objects compose a further object may be a vague matter — there may be no sharp cutoff. This allows ordinary objects while allowing that there is no determinate answer to questions like "at exactly which moment does a collection of bricks become a wall?"

### Classical Mereology

Classical mereology (Leśniewski, Leonard-Goodman) is defined by the following axioms (using PPT for proper part, O for overlaps, Fu for fusion/sum):

1. **Irreflexivity of PP**: *¬(x PP x)* — nothing is a proper part of itself.
2. **Asymmetry**: *x PP y → ¬(y PP x)*
3. **Transitivity**: *(x PP y ∧ y PP z) → x PP z*
4. **Weak supplementation**: *x PP y → ∃z(z PP y ∧ ¬(z O x))* — if x is a proper part of y, y has at least one other proper part disjoint from x.
5. **Strong supplementation**: *¬(y PP x) → ∃z(z PP y ∧ ¬(z O x))* — if y is not a proper part of x, then something is part of y but not of x.
6. **Unrestricted fusion** (Universal Composition): *∀φ∃y(y = Fu(φ))* — for any (non-empty) condition φ, there exists a fusion of all the things satisfying φ.

Strong supplementation + Unrestricted fusion entails the **Uniqueness of Composition**: if x and y have exactly the same parts, then x = y. This means that coincident objects (same parts, distinct objects) are impossible in classical mereology.

### The Coincidence Problem

**Leibniz's Law argument for non-identity of statue and clay**:
- The statue has the property "necessarily, if squashed, ceases to exist."
- The clay does not have this property.
- Therefore (Leibniz's Law), statue ≠ clay.

But if statue ≠ clay, and they share all their parts at a time, then mereological identity fails (or classical mereology is false for this case).

**Responses**:

*Constitution view* (Baker): Reject the principle that same parts ↔ same object. The statue and clay are distinct objects that stand in the *constitution* relation — a real, non-identity relation. Deny classical mereology's uniqueness of composition.

*Four-dimensionalism*: "Statue" and "clay" refer to distinct four-dimensional objects that share a temporal part right now. Their parts are not identical across time; they share a present temporal stage but differ elsewhere. Classical mereology is preserved for four-dimensional objects.

*Temporary identity* (Gibbard): Accept that the statue = the clay at time t; but identity is temporary and contingent. This is contextual identity rather than absolute identity — a controversial thesis.

*Sortal-relative identity* (Wiggins): Identity is always identity under a concept (sortal). The statue and clay are identical-as-lump but distinct-as-statue. This requires giving up the absoluteness of identity — equally controversial.

### Atomism, Gunk, and the Question of Composition

**Atomism**: Every object has parts, and those parts have parts, but the process terminates in simples — partless entities ("atoms"). Most everyday and scientific ontologies are atomistic.

**Gunk** (a term from Lewis): Possible entities in which every part has a further proper part — there are no atoms. Gunky space and gunky matter are possibilities considered in the literature. Classical mereology is compatible with gunk; it does not require atoms.

**Atomless mereology** (developed by Leśniewski and Tarski): Formal mereology without the assumption of atoms. Consistent with gunk if gunk is possible.

**Nihilism about composition** (combined with atomism): If nothing composes, and there are atoms, then the only existing objects are the atoms. Everyday objects are useful fictions or pluralities of atoms.

---

## Key Arguments and Counterarguments

### The Argument from Vagueness for Universalism (Sider)

**P1.** Composition cannot be vague — there cannot be a sharp cutoff to when composition occurs.
**P2.** Any restricted composition theory implies there is a sharp cutoff (below which composition fails, above which it obtains).
**P3.** But sharp cutoffs for vague properties are counterintuitive (any such line would seem arbitrary).
**C.** Composition must be universalistic — it always occurs, and there is no restricted case to be vague about.

**Van Inwagen's response**: vagueness in composition is no worse than vagueness in other cases; just as "baldness" admits of borderline cases without implying there are no bald people, "composition" admits of borderline cases without implying universalism.

**Nihilist response**: there is no vagueness problem because composition never occurs; all apparent cases of vague composition are resolved by saying no composition obtains.

### The Problem of the Many (Unger 1980)

Consider a cloud. Near its boundaries, individual water droplets may or may not be "part of" the cloud. For each droplet that might or might not be included, we get a distinct candidate for "the cloud." If each collection of droplets is a cloud, we have many clouds where we thought there was one. 

**Universalist response**: all candidate collections of droplets compose distinct overlapping objects; one of them is "the cloud" in context, the others are unnamed but real.

**Nihilist response**: no collection of droplets composes anything; there is no cloud, only droplets.

**Mereological essentialism** (Chisholm): Composition is always essential — an object has exactly the parts it has necessarily. If even one droplet is removed, the cloud is destroyed and a new cloud is created. This avoids the many but at the cost of eliminating ordinary persistence through material change.

### The Timing of Composition (Puzzles for Four-Dimensionalism)

Given four-dimensionalism, there are infinitely many four-dimensional "worms" that share any given temporal stage. If I am one such worm, and my qualitatively identical twin (existing at the same time) is another, then every temporal stage I share with my twin corresponds to at least two worms. But then there is not just one person sitting in this chair — there are many.

**Sider's response**: We individuate persons by their entire worms; each worm is distinct; there is one worm that counts as "me" in context (my whole temporal extent). The problem of the many is resolved by contextual conventions about which worm is salient.

---

## Key Papers and Books

1. **Simons, Peter, *Parts: A Study in Ontology* (1987)** — The most comprehensive philosophical treatment of mereological systems; covers classical, extensional, and non-classical systems with full formal rigor.

2. **van Inwagen, Peter, *Material Beings* (1990)** — Introduces the Special Composition Question systematically; defends organicism; the central text for composition debates.

3. **Lewis, David, *Parts of Classes* (1991)** — Applies mereology to set theory; develops the thesis that membership in sets is analyzable mereologically; important for formal ontology.

4. **Sider, Theodore, *Four-Dimensionalism* (2001)** — Develops the argument from vagueness for universalism; connects temporal parts to the formal theory of composition.

5. **Varzi, Achille, "Mereology" (*Stanford Encyclopedia of Philosophy*)** — The definitive survey article; systematic treatment of classical and non-classical mereological systems.

6. **Unger, Peter, "The Problem of the Many" (1980)** — Introduces the problem of the many; the founding puzzle for contextual and universalist responses.

7. **Baker, Lynne Rudder, "Why Constitution is not Identity" (1997)** — Defends the constitution view against the identity thesis; part of the debate about coincident objects.

8. **Thomson, Judith Jarvis, "The Statue and the Clay" (1998)** — Develops the coincidence view; careful treatment of the identity conditions of statues and clay.

---

## Live Debates and Open Questions

1. **Mereological essentialism**: Chisholm's view that objects have their parts essentially has been developed more recently by Rosen and Dorr (2002). If mereological essentialism is true, ordinary objects do not persist through material change — which conflicts with common sense.

2. **Temporal parts in physics**: Do physical objects have temporal parts in the way that perdurantism requires? Physics describes field configurations at spacetime regions; it does not obviously individuate "temporal stages." The relationship between physical ontology and metaphysical temporal-parts theory remains contested.

3. **Mereological nihilism and existence**: If nihilism is true, everyday objects (tables, chairs, persons) do not exist. What is the appropriate response to this conclusion? Eliminativism? Reconceptualization (van Inwagen)? Fictionalism?

4. **Emergent composition**: Can composition give rise to genuinely emergent properties — properties of wholes not predictable from the properties of parts? Standard mereology is committed to no emergent properties (a fusion has all and only the properties determined by its parts). Strong emergence would require revision of mereological principles.

5. **Non-classical mereologies**: Some philosophers (Cotnoir) defend non-classical mereologies that allow for circular parthood (x is part of y, y is part of x), reflexivity failure, or non-extensionality. These are motivated by cases in social ontology and Buddhist metaphysics.

---

## Connections to Other Domains

**Mereology and Identity/Persistence (Domain 03)**: The coincidence problem is simultaneously a mereological puzzle (same parts → same object?) and a persistence puzzle (when does a lump of clay become a statue?). Four-dimensionalism is explicitly a mereological thesis about temporal parts. Mereological essentialism has direct implications for personal identity (persons do not survive material replacement of cells).

**Mereology and Ontology (Domain 01)**: Mereological universalism generates an enormous ontology of arbitrary fusions; nihilism eliminates almost everything. The debate connects directly to questions about parsimony, ontological commitment, and what is in the fundamental domain.

**Mereology and Time (Domain 06)**: If temporal parts exist, then objects extend through time mereologically just as they extend through space. The mereology of time is part of four-dimensionalist metaphysics; the question of whether time has an atom-like or gunky structure is part of temporal ontology.

**Mereology and Metaphysics of Science (Domain 10)**: The levels-of-organization picture in biology (molecules, cells, organisms, populations) presupposes that wholes at one level are made of parts at the next level down. Whether this compositional hierarchy corresponds to a genuine mereological part-whole hierarchy depends on the theory of levels.

**Mereology and Truth/Reality (Domain 12)**: Truthmaker theory requires that truths be made true by (appropriately existing) entities; the truthmakers for claims about complex objects may be the fusions or states of affairs involving those objects. Mereology determines what complex entities exist to serve as truthmakers.
