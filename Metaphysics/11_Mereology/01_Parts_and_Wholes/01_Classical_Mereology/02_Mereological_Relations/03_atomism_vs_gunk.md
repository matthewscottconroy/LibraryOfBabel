# Atomism vs. Gunk

Classical mereology leaves open a question about the ultimate structure of the part-whole hierarchy: does it bottom out, or does it go all the way down? We might expect physics to settle the matter — and indeed contemporary physics posits fundamental particles, quarks and leptons, that appear to be mereological atoms, entities with no spatial or material parts. But history counsels caution. What was once considered fundamental has repeatedly turned out to have further structure: atoms were divided into nuclei and electrons, nuclei into protons and neutrons, protons into quarks. Whether the sequence terminates is an empirical question mereology cannot settle alone. And there is a genuine alternative: *gunk*, David Lewis's term for matter in which every part has further proper parts, with no atoms at any level.

Both possibilities can be characterized precisely within classical mereology, and both yield consistent systems.

**Atom:** AT(x) =df ¬∃y PP(y, x)

**Atomism:** ∀x ∃y [P(y, x) ∧ AT(y)]    — every object has at least one atomic part

**Atomlessness (Gunk):** ∀x ∃y PP(y, x)    — every object has a proper part, with no bottom level

Notice that atomism and atomlessness are *independent* of the core mereological axioms M1–M6. Classical mereology is neutral: you can add either as an additional axiom and get a consistent system. Atomism gives *Atomistic Classical Mereology (ACM)*; atomlessness gives *Gunky Classical Mereology (GCM)*.

## Models of Each System

The natural model for ACM is the power-set algebra of a set of atoms, minus the empty set: objects are non-empty subsets, parthood is subset inclusion, fusion is set union. This is the "standard" model taught in most presentations.

GCM has its natural model in the real interval (0, 1] with the overlap relation defined by non-empty intersection of open subintervals. Every interval has proper subintervals; there are no atomic intervals. This confirms that gunky mereology is not just conceivable but formally consistent.

## Why It Matters Philosophically

The atomism question is not merely technical. Consider first the *fusion axiom*: if mereological atomism holds, we can give a simpler characterization of fusion — z is the fusion of the φ-ers iff every atom that is a part of z is an atom that is a part of some φ-er, and vice versa. In GCM, no such atomic characterization is available, and the definition of fusion must proceed through the overlap condition. Arguments that work smoothly in ACM may require careful revision in GCM.

Consider also the *composition-as-identity thesis* (Baxter, Cotnoir): the view that a whole is *identical* to its parts taken collectively. In ACM, this thesis has a clear content — a composite object just is its atoms. In GCM, there are no atoms to serve as the ultimate describer, and the thesis becomes harder to state. If every whole is identical to its parts, which themselves are identical to *their* parts, the regress never grounds out.

David Lewis himself said he saw no reason why gunk should be impossible. Many philosophers accept that gunky worlds are at least conceivable and that nothing in our concept of an object rules out infinite divisibility without remainder. If that is right, mereology must be consistent with gunky models — and ACM is too strong as a claim about all possible worlds.

There is also a connection to ancient debates. Aristotle's response to Zeno was that matter is *potentially* infinitely divisible but never *actually* divided into infinitely many parts. Gunky mereology formalizes the actualist version: in a gunky world, every part *actually has* further parts. The division is not merely potential but realized.

A gunky world need not be incoherent, but it requires care about how size and measure are assigned. If every part has proper parts, and parts are assigned positive measure (volume), then the total measure of a gunky object must be assigned in a way that does not require atomic mass distributions. The mathematics of non-atomic measure spaces — where measures can be defined on continua without any atom having positive measure — provides the tools. Whether a gunky ontology is empirically or metaphysically preferable remains an open question.

## The Argument from Intrinsic Properties

Peter Simons has argued that mereological atomism is required for there to be genuine *intrinsic properties* — properties that characterize objects independently of their relations to other things:

- P1. An intrinsic property of an object must be grounded in something about the object itself, not its relations to other objects.
- P2. If every object has proper parts, then the properties of any object are partly constituted by the properties and arrangement of its parts.
- P3. If properties are always constituted by sub-object properties and arrangements, then no property is fully grounded in the object itself — every property becomes a relational fact about the object's parts.
- C. Without atoms — objects that have no parts — there is no level at which properties are genuinely intrinsic; they are relational all the way down.

The objection to Simons is that this argument assumes relational properties cannot be intrinsic, but the notion of intrinsicness is contested. Lewis, himself an atomist, defines intrinsic properties functionally: a property is intrinsic iff its distribution is independent of what else exists. By this definition, a gunky object could still have intrinsic size, shape, and mass — these being properties that supervene on the complete internal structure of the object, not on its relations to external things. The worry about infinite regress in grounding may apply equally to atomistic worlds, where the grounding terminates in atoms but atoms themselves have properties that are presumably not grounded in anything further.

## Hypergunk and the Well-Foundedness Question

Jonathan Schaffer and others have discussed *hypergunk*: a world where not only is every object divisible, but the divisibility goes through transfinite ordinals — there is no ordinal level at which the parts run out. This is technically coherent in the set-theoretic sense (transfinite descending chains of proper parts), though physically even more exotic than ordinary gunk. Hypergunk raises the question of whether well-foundedness of proper parthood — every descending chain terminates — is a metaphysical necessity, an empirical conjecture, or merely a convenient idealization. Classical mereology (M1–M6) is silent on this question too. Which extension is correct, if any, belongs to the intersection of mereology, physics, and the theory of possibility, not to formal mereology alone.
