# Deontic Logic

Moral and legal discourse is saturated with modal-sounding claims: what one must do, what one may do, what one is forbidden to do. Deontic logic applies the modal framework to normative concepts — obligation, permission, and prohibition — by reinterpreting the box and diamond. The box becomes the obligation operator O ("it is obligatory that") and the diamond becomes the permission operator P ("it is permissible that"). Prohibition — FA ("it is forbidden that A") — is defined as O¬A: what is forbidden is what one is obligated not to do.

## Formal Structure

Standard deontic logic (SDL) is built on the modal framework K + D, where:

- K: O(A → B) → (OA → OB) — Obligation distributes over implication
- D: OA → PA — Whatever is obligatory is permissible

The accessibility relation for deontic logic is interpreted as normative ideality: world v is deontically accessible from w if v is a world where all obligations are fulfilled — a morally ideal world relative to w's normative standards.

The D axiom (OA → PA) captures the consistency of morality: you cannot be both obligated to do A and obligated not to do A. Formally, OA ∧ O¬A would yield OA ∧ ¬PA (from D), a contradiction. The normative system is internally consistent.

Notice that SDL does not validate T (OA → A). That would require the actual world to be deontically ideal — that all obligations are fulfilled. Clearly they are not; deontic modality is not reflexive.

## Paradoxes and Their Lessons

The Good Samaritan Paradox is the most widely discussed difficulty for SDL. Consider:

1. O(helps the victim who was robbed) — It is obligatory to help the victim who was robbed.
2. [Helping a robbed victim entails there is a robbed victim] — a logical truth.
3. K axiom: O(A → B) ∧ OA → OB.

If we apply deontic distribution with (2): O(there is a victim who was robbed). But it seems absurd to be obligated to ensure someone has been robbed.

The lesson is not that deontic logic is hopeless, but that the K axiom requires careful handling when the antecedent of the embedded conditional describes a prior bad situation rather than something the agent brings about. Solutions include conditional deontic logic — where obligations are explicitly indexed to circumstances: "Given that there is a victim, help the victim" — and the distinction between "ought-to-do" and "ought-to-be." The paradox forces precision about what the obligation is *for*.

A second challenge is Chisholm's paradox (1963), which involves contrary-to-duty obligations — norms that kick in when a primary obligation is violated:

1. OG — You ought to go help your neighbor.
2. G → OT — If you go, you ought to tell your neighbor you're coming.
3. ¬G → O¬T — If you don't go, you ought not to tell your neighbor you're coming.
4. ¬G — You are not going.

SDL struggles to represent all four sentences consistently with the appropriate logical relations. Sentences (3) and (4) yield O¬T; but (1) and (2) seem to yield OT via the primary obligation. The four sentences generate inconsistency in SDL even though they describe a coherent normative situation. Solutions involve dyadic deontic logic — where obligation is relativized to conditions: O(B/A) means "Given A, B ought to be" — or priority logics that represent the hierarchy of primary and contrary-to-duty norms.

## Applications

Deontic logic has found important applications in ethics, legal reasoning, and computer science. In ethics, formal deontic logic provides a rigorous framework for representing and testing the consistency of moral principles. Deontological theories posit absolute prohibitions and permissions that can be formalized using O, P, and F; the formal apparatus reveals hidden inconsistencies. In law, legal codes can be partially formalized: "It is obligatory to pay taxes by April 15" (OTax), "It is permitted to own property" (Pproperty). The field of normative systems theory (Alchourrón and Bulygin) develops this application systematically, using deontic logic to identify inconsistencies, redundancies, and gaps in legal codes. In AI and computer science, normative multi-agent systems use deontic logic to specify norms governing agent behavior, and access-control policies in operating systems can be expressed deontic-logically.

## Connection to Metaphysical Modality

The relationship between deontic and metaphysical modality raises deep questions. Some philosophers hold that moral facts are metaphysically necessary: if torturing innocents for amusement is wrong, it is necessarily wrong — there is no possible world where such torture is morally permissible. On this view, deontic necessity is a species of metaphysical necessity.

Others hold that moral facts are contingent — what is obligatory depends on contingent features of agents, societies, or ideal agreements. On this view, deontic necessity is weaker than metaphysical necessity.

The "ought implies can" principle — OA → ◇A — connects deontic modality to physical or metaphysical modality: obligations cannot require the genuinely impossible. This bridge principle has been used to argue that moral obligations cannot demand what is physically or metaphysically impossible, and it connects the formal structure of deontic logic to substantive debates about free will and moral responsibility. Whether the "can" in "ought implies can" is physical possibility, agent possibility, or some other notion is a substantive question about which kind of modality bears on normative evaluation.
