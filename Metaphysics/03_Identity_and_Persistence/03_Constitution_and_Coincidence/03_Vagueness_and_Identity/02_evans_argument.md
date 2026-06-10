# Evans's Argument

*The argument that identity statements cannot be indeterminately true or false.*

---

Gareth Evans's brief 1978 paper "Can There Be Vague Objects?" presents a logical argument that identity cannot be genuinely indeterminate. The argument is compressed — the paper is barely a page — but has generated decades of discussion, because if it succeeds, it closes off a natural avenue for treating ontic vagueness.

## The Argument

Suppose it is indeterminate whether a = b (symbolized: ∇(a = b)). Consider the property λx.∇(x = a) — the property of *being such that it is indeterminate whether one is identical to a*.

- **(1)** b has the property λx.∇(x = a), since by assumption ∇(a = b), and therefore ∇(b = a) [by symmetry of identity], and therefore b has λx.∇(x = a).
- **(2)** a does NOT have the property λx.∇(x = a), because it is *determinate* that a = a (by reflexivity, ∀x(x = x) is a logical truth, and logical truths are determinately true). So a determinately has a = a, meaning ¬∇(a = a), and therefore a lacks λx.∇(x = a).
- **(3)** By Leibniz's Law: if a = b, then ∀F(Fa ↔ Fb). Since a lacks λx.∇(x = a) but b has it (from (1) and (2)), we have ∃F(Fa ∧ ¬Fb) or ∃F(¬Fa ∧ Fb). Therefore, by Leibniz's Law, a ≠ b.
- **(C)** So ∇(a = b) implies a ≠ b — which means ∇(a = b) implies ¬∇(a = b). Contradiction.

Conclusion: ∇(a = b) is impossible. Identity statements cannot be indeterminately true or false.

## Significance

If Evans's argument is sound, then any apparent indeterminacy in identity claims must be semantic (arising from vague terms or descriptions) rather than ontic (arising from the nature of the objects themselves). The world cannot contain individuals a and b such that it is a brute, non-semantic, metaphysical fact that the identity a = b is indeterminate.

This has consequences for personal identity, coincident objects, and vague spatial boundaries. If it is apparently indeterminate whether the person after a gradual replacement procedure is the same as the person before, Evans's argument suggests this indeterminacy is semantic — arising from vagueness in our concept of personal identity — not a genuine metaphysical indeterminacy.

## The Metalinguistic Response

The most influential response: Evans's argument uses the property λx.∇(x = a), which is not a genuine property of objects but a semantic property — a relation between an object and a *name* or *description* "a." Leibniz's Law applies only to genuine properties of objects, not to semantically loaded ones. So the argument fails.

More precisely: λx.∇(x = a) is a description-sensitive property. Whether b has it depends on how b is designated — as "b" or as "a" — not on b's intrinsic nature. But Leibniz's Law concerns description-independent properties. Apparent differences in description-sensitive "properties" do not establish non-identity. This response is widely accepted as the standard reply. It shows that Evans's argument at best establishes: if identity is indeterminate, one cannot apply Leibniz's Law via the property λx.∇(x = a) — not that identity cannot be indeterminate.

## The Classical Logic Assumption

Evans's argument assumes:

1. Classical logic (every statement is either true or false).
2. ∀x(x = x) is a logical truth (and hence determinately true).
3. Leibniz's Law holds without restriction.

Challenging any of these blocks the argument. If we allow truth-value gaps — "it is indeterminate whether a = b" being a gap rather than a third truth value — the classical principle of bivalence fails, and Evans's argument, which proceeds by deriving a classical contradiction, cannot be run. If "a" is a vague name (with indeterminate reference), then "a = a" might fail to have a determinate truth value, blocking step (2). If Leibniz's Law is restricted to exclude intensional properties (like λx.∇(x = a)), step (3) is unavailable.

Perhaps the most interesting response: perhaps it is itself indeterminate whether a has the property λx.∇(x = a). If ∇(a = a) is indeterminate — perhaps because "a" fails to determinately refer — then a's having the property λx.∇(x = a) is indeterminate, blocking the derivation that a *determinately* lacks that property. This response challenges the classical logic assumption built into step (2).

Evans's argument remains one of the most discussed in contemporary metaphysics, and no consensus has emerged about whether it is sound. Its significance lies not in settling the debate about vague identity but in making precise what would be required for such identity to be possible — and in showing that anyone who wants to defend ontic vagueness must be prepared to revise some aspect of classical logic or classical accounts of property attribution.
