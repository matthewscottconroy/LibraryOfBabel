# Realism, Antirealism, and the Logic of Truth

Do mathematical objects — and, more generally, the subject matters of our theories — exist and have their properties *independently of us*? The naive form of the question invites a picture: a platonic heaven of numbers we inspect from afar. Michael Dummett's decisive move was to argue that the picture is a distraction, and that the real content of realism is a thesis about **meaning and logic**. On his reframing, to be a realist about a domain is to accept **bivalence** for it — every statement is determinately true or false, independently of our capacity to decide which — and hence to accept **classical logic**. Antirealism denies verification-transcendent truth, and with it the unrestricted law of excluded middle. The metaphysical dispute becomes, literally, a dispute over which logic is correct.

## The Positions, and What Divides Them

The familiar ontological positions ([§3](../03_abstract/01_abstract_objects.md)) can be arranged by their answers about truth:

- **Platonism (mathematical realism).** Abstract objects exist mind-independently; mathematical truths are *discovered*. Its supports are the indispensability argument (§3), the phenomenology of discovery (the Mandelbrot set "was already there"), and the objectivity of open problems (the Riemann Hypothesis is true or false regardless of us). Its liability is Benacerraf's epistemology.
- **Structuralism.** Mathematical objects are *positions in structures*, not independent entities — "$2$" is simply the second place in the natural-number structure, with no properties beyond its structural role (Benacerraf's multiple-reductions problem, §3). *Eliminative* structuralism reads arithmetic as universally quantified over all structures satisfying the axioms; *non-eliminative* structuralism (Shapiro) takes structures themselves to be abstract objects.
- **Fictionalism.** Mathematical statements are *false* (or truth-valueless) but assertible "in the story of mathematics," their utility explained without literal abstracta (Field, §3; Yablo).

What genuinely separates these, Dummett urges, is not the inventory of objects but whether they license **bivalent, recognition-transcendent truth**. A platonist and a full-blooded structuralist both help themselves to classical logic; a fictionalist or intuitionist need not.

## Dummett's Challenge: Manifestation and Acquisition

Dummett's argument against realism is an argument from the **theory of meaning**. Following Wittgenstein, he holds that meaning must be exhaustively **manifest in use**: to grasp a sentence's meaning is to master the practice of asserting and recognizing it. Two arguments then bite against realist truth-conditions:

- **The manifestation argument.** If the meaning of $S$ consisted in a truth-condition that could obtain *undetectably* — as for an undecided statement like Goldbach's conjecture, or "a city will never be built on this spot" — then a speaker's grasp of that condition could never be *fully* exhibited in any exercise of a recognitional capacity, since by hypothesis the condition may hold with nothing to recognize. So a use-based semantics cannot underwrite verification-transcendent truth.
- **The acquisition argument.** We learn language from publicly available circumstances of use; we could never have acquired the concept of a truth-condition that outruns all possible verification.

The conclusion is to replace **truth-conditional** semantics with **assertibility-** or **proof-conditional** semantics: a statement is warranted not when a mind-independent fact obtains but when we possess a verification. For mathematics, "verification" means *proof*, and the resulting logic is **intuitionistic**.

## Intuitionism as Mathematical Antirealism

L.E.J. Brouwer's intuitionism, formalized by Heyting, is antirealism about mathematical objects made precise: they are **mental constructions**, and a statement is true only if we can construct a proof of it. The meanings of the connectives are given not by truth tables but by the **Brouwer–Heyting–Kolmogorov (BHK) interpretation** (Chapter 11, Curry–Howard):

- a proof of $\varphi \land \psi$ is a pair of proofs;
- a proof of $\varphi \lor \psi$ is a proof of one disjunct *together with a marker of which*;
- a proof of $\exists x\,\varphi(x)$ is a construction of a **witness** $a$ together with a proof of $\varphi(a)$;
- a proof of $\varphi \to \psi$ is a method transforming any proof of $\varphi$ into one of $\psi$.

On this reading the **law of excluded middle** $\varphi \lor \neg\varphi$ is *not* a general law: to assert it for arbitrary $\varphi$ would be to claim, for every statement, either a proof or a refutation — which we plainly lack (Goldbach again). Rejecting LEM is not skepticism but a *different meaning* for "or" and "not." Intuitionistic $\exists$ carries genuine existential import that classical $\exists$ lacks: a classical existence proof by contradiction need exhibit no witness, whereas an intuitionistic one must — the constructive/classical divide of [Chapter 5](../../ch05_proof_strategies/). Antirealism about mathematical existence *is* the demand for constructive proof.

## The Choice of Logic Is Downstream of Meaning

The deep point is that **which logic is valid is settled by one's theory of truth.** Bivalent, realist truth validates classical logic — LEM, double-negation elimination $\neg\neg\varphi \to \varphi$, proof by contradiction. Assertibility-based, antirealist truth validates intuitionistic logic, where these fail. This is the precise sense in which metaphysics "bears on logic": the realism debate is not decorative commentary on a fixed logic but a dispute about *which inferences preserve warrant*.

Two formal facts sharpen it. First, **Kripke semantics for intuitionistic logic** (Chapter 12) models the antirealist picture directly: truth is *forcing* at a node in a partially ordered frame of information states, monotone as knowledge grows, and $\varphi \lor \neg\varphi$ can fail at a node where neither is yet forced — a formal image of the ever-incomplete body of construction. Second, the **Gödel–Gentzen double-negation translation** embeds classical logic into intuitionistic: $\varphi$ is classically provable iff its translation $\varphi^{\neg\neg}$ is intuitionistically provable. So the antirealist can *interpret* everything the realist says; the dispute is not about which inferences are available but about what they *mean* and which are unrestrictedly valid.

## A Test Case: Independence in Set Theory

The realism debate meets concrete mathematics in the **independence phenomena** (Chapter 6). The Continuum Hypothesis is neither provable nor refutable from ZFC (Gödel, Cohen). The realist — Gödel foremost, who held that we possess a quasi-perceptual mathematical **intuition** — insists CH nonetheless has a determinate truth-value that our axioms merely fail to capture, and seeks new axioms (large cardinals, Woodin's programs) to decide it: the **universe** view, one true set-theoretic reality. The antirealist or formalist reads independence as showing there is **no fact of the matter** — the concept of set is not fully determinate — a stance congenial to Hamkins's set-theoretic **multiverse**, on which CH is simply true in some universes and false in others. Whether "CH is determinately true or false" is itself the realism question, posed about the sharpest available example.

## Antirealism at the Keyboard

Nowhere is the dispute more concrete than in a proof assistant, where the choice of logic is a *configuration setting* with computational stakes. **Coq** and **Agda** are intuitionistic by default: excluded middle is not assumed, and a closed proof of $\exists x\,\varphi(x)$ literally *computes* a witness, so verified programs can be **extracted** from proofs (Chapter 11). Adding `Classical` (Coq) or an LEM axiom blocks extraction for the affected proofs. **Lean's Mathlib**, by contrast, is unapologetically classical — `Classical.em` and choice are available throughout — because its goal is to formalize ordinary (realist) mathematics with maximum convenience. The working mathematician's realism and the constructivist's antirealism are thus not merely philosophical postures but alternative, and interconvertible, foundations one selects when building verified mathematics. The metaphysics of truth has become, in part, an engineering decision.

## Exercises
See [problems/ch22_metaphysics/](../../../problems/ch22_metaphysics/)
