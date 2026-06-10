# The Barcan Formula and Converse Barcan Formula

Ruth Barcan Marcus proved the formulas that bear her name in 1946, working within a system of quantified modal logic. They may appear at first to be technical curiosities about the interaction of quantifiers and modal operators. In fact they encode fundamental commitments about the ontological inventory of modal space — about whether there are merely possible objects that don't actually exist, and about whether actual objects exist necessarily.

The two formulas are:

**Barcan Formula (BF)**: ◇(∃x)Fx → (∃x)◇Fx

If possibly something is F, then there is something that is possibly F.

**Converse Barcan Formula (CBF)**: (∃x)◇Fx → ◇(∃x)Fx

If there is something that is possibly F, then possibly something is F.

In their universal forms: BF says (∀x)□Fx → □(∀x)Fx (if every individual is necessarily F, then necessarily every individual is F); CBF says □(∀x)Fx → (∀x)□Fx (if necessarily every individual is F, then every individual is necessarily F).

## What the Barcan Formula Claims

The BF says: if it is possible that there exists an F, then there actually exists something that is possibly F. Applied to existence itself: if it is possible that there exists a golden mountain, then there actually exists something that is possibly a golden mountain.

The striking consequence: if the BF holds universally, then every possible object is an actual object. There are no "merely possible" objects — objects that could have existed but don't. The BF forces the domain of quantification to be constant across worlds. If we allowed non-actual objects, the BF could fail: "possibly there exists a unicorn" might be true, but if unicorns don't actually exist, there would be nothing in the actual domain that is possibly a unicorn.

The BF is valid in QML with **constant domains**: if every world has the same domain of individuals, then "possibly something is F" means "some F exists at some accessible world"; since that individual is in the constant domain, it is also in the actual domain, and the BF holds.

The CBF says: if something actually exists that is possibly F, then possibly something is F. This direction is relatively uncontroversial — if an actual individual could be F, then there is a possible world where something is F. The CBF corresponds to domains that can only shrink as we move to accessible worlds: actual individuals persist (at least as non-concrete entities) across worlds. This is a version of the claim that actually existing things could not have been replaced by wholly different things.

## Kripke's Variable-Domain Counterexample to BF

Kripke showed that the BF is not valid in models with variable domains — Kripke models where different worlds have different domains. Consider:

- W = {w, v}, wRv (v is accessible from w)
- D(w) = {a} (only a exists at w)
- D(v) = {a, b} (both a and b exist at v)
- V(F, v, b) = T, V(F, v, a) = F, V(F, w, a) = F

At w: ◇(∃x)Fx is true (since at v, b is F, so ∃x(Fx) is true at v). But (∃x)◇Fx is false at w, since the only object in D(w) is a, and a is not F at any accessible world. The BF fails: it is possibly the case that something is F, but there is no actual thing that is possibly F.

This countermodel is philosophically interpretable: b is a "merely possible" individual, existing at v but not at w (the actual world). At w, it is possible that b exists and is F, but since b doesn't exist at w, there is no actual object that is possibly F. The BF fails precisely because of the existence of merely possible objects — objects that exist in other worlds but not in the actual one.

## Philosophical Implications

The debate over the Barcan Formula is nothing less than a debate about the ontological structure of modal space.

For Lewis's modal realism, a form of possibilism, the inhabitants of other worlds are fully real — though "merely possible" relative to us. The BF can fail for Lewis: "possibly something is a unicorn" might be true (unicorns exist at other worlds) but there may be no actual unicorn, since "actual" is indexical to our world and unicorns don't exist here.

For Plantinga's actualism, the BF fails in a different way: haecceities (individual essences) exist necessarily as abstract objects, but the individuals whose essences they are may not exist at all worlds. "Possibly there exists a Sherlock Holmes" might be true because there is a possible world where Holmes's haecceity is instantiated, but Holmes doesn't actually exist.

Williamson's necessitism takes a radical alternative route: necessarily everything necessarily exists, at least in a minimal sense — as an abstract "bare particular" even at worlds where it is not concrete. On this view the BF holds, because the domains are constant at the level of thin existence, even if not at the level of concreteness. Williamson's position defuses the BF's bite by interpreting existence thinly — but it is a substantial metaphysical commitment.

The BF and CBF are thus precise formal tests for fundamental positions in modal ontology. Whether there are merely possible individuals, whether actual individuals exist necessarily, and whether existence is itself a modal matter — these questions receive determinate answers depending on which of these formulas one accepts. What might have seemed like a technical matter about the interaction of quantifiers and operators turns out to be a matter of the deepest metaphysical significance.
