# The Modal Ontological Argument

There is something philosophically surprising about the modal ontological argument: its crucial premise is not "God exists" but merely "God possibly exists." If it works, this modest claim — that the existence of a maximally great being is not logically impossible — is sufficient to establish that God actually exists. The argument exploits a feature of necessary existence: if a necessarily existing being is possible at all, it exists in every possible world, including the actual one.

The argument was developed by Charles Hartshorne, Norman Malcolm, and most influentially by Alvin Plantinga, drawing on the resources of modal logic — the formal logic of possibility and necessity.

## Background: Modal Logic and S5

Modal logic introduces operators for necessity (□) and possibility (◇):

- □P: "It is necessarily the case that P" (P holds in every possible world)
- ◇P: "It is possibly the case that P" (P holds in at least one possible world)

The system S5 of modal logic adds the axiom: ◇□P → □P. In possible-worlds terms: if there is some possible world in which P is necessary — true in all possible worlds — then P is true in this world. This axiom is the pivot on which the modal ontological argument turns.

## Plantinga's Version

Plantinga defines a *maximally great being* (MGB) as one that is maximally excellent in every possible world where it exists — omnipotent, omniscient, and perfectly good — and, crucially, *necessarily existing*, meaning it exists in every possible world.

- P1: It is possible that a maximally great being exists. [◇∃x (MGB(x))]
- P2: A maximally great being, if it exists in any possible world, exists in all possible worlds (since maximal greatness includes necessary existence). [MGB(x) → □∃x MGB(x)]
- P3: If a maximally great being possibly exists, and its existence in any world entails its existence in all worlds, then it actually exists. [◇□∃x MGB(x) → □∃x MGB(x) → ∃x MGB(x)]
- C: Therefore, a maximally great being (God) exists.

The argument is logically valid in S5. Given the S5 axiom ◇□P → □P, the move from "possibly necessarily, a MGB exists" to "necessarily, a MGB exists" to "a MGB actually exists" is formally correct.

## Why Validity Doesn't End the Debate

The argument is valid but not obviously sound. The sole disputed question is P1: is it genuinely possible that a maximally great being exists?

Notice what asserting P1 commits us to. To say X is possible is to say there is a possible world in which X exists. If necessary existence is built into the definition of a MGB, then saying "it is possible that a MGB exists" means "there is a possible world in which a being exists in *every* possible world" — which, by S5, entails that the MGB actually exists. So asserting P1 is, in the context of S5, equivalent to asserting the conclusion. Critics — Mackie, Sobel — argue that P1 therefore begs the question.

Plantinga's response is illuminating. The argument shows that if MGB-existence is possible, it is actual. This reveals that the question "Does God exist?" is the same question as "Is a maximally great being possible?" — and that is philosophically non-trivial. If we have some reason to think the concept of a MGB is coherent and instantiable, we have reason to think God actually exists. Plantinga concedes that the argument does not compel rational acceptance of P1 — one might simply deny it. But he argues that a rational person who has no good reason to think the concept of a MGB is incoherent can rationally accept P1 and thus rationally accept the conclusion.

## The Symmetry Problem

A significant objection: define a *maximally evil being* (MEB) as omnipotent, omniscient, and perfectly evil in every possible world in which it exists. If we assert "it is possible that a MEB exists," then by parallel reasoning we conclude that a MEB actually exists. But "God (MGB) exists" and "a MEB exists" cannot both be true — contradiction. This mirrors Gaunilo's parody at the classical level.

Plantinga's response is that a MEB is incoherent: omnipotence and omniscience combined with perfect evil would be internally unstable, because an omniscient and omnipotent being would know and be able to achieve the best outcomes, and a being that systematically chooses the worst is defective in some way. Critics find this response ad hoc — the defender of MEB could run the same response in reverse, arguing that a MGB is incoherent because omnipotence combined with perfect goodness would be unable to permit any evil at all, which creates problems for the free will defense.

## Malcolm's Version

Norman Malcolm distinguished two ontological arguments in Anselm's *Proslogion*:

- **Chapter II**: Existence is a great-making property (Kant's target).
- **Chapter III**: *Necessary* existence is a great-making property (not obviously Kant's target).

Malcolm argues that the Chapter III argument evades Kant's objection: necessary existence is not bare existence but a modal property — the property of being unable to fail to exist, of being self-sustaining, of owing existence to nothing outside oneself. This is plausibly a greatness-property, and it is distinct from the bare addition of "exists" to a concept. If a being has necessary existence, then either it is impossible (it exists in no possible world) or it is actual (it exists in all possible worlds). Since the concept of God is not logically contradictory, God's existence is not impossible; and since it is not impossible, God necessarily exists; and therefore God actually exists.

## Hartshorne's Formalization

Hartshorne formalized the modal ontological argument explicitly. Let G = "God exists":

1. Either ~◇G (God's existence is impossible), or ◇G (God's existence is possible).
2. If ◇G and G is defined as a necessary being, then □G — if God possibly exists and is necessary, God necessarily exists (by S5).
3. □G → G (whatever is necessary is actual).
4. ~◇G is implausible (there is no obvious contradiction in the concept of God).
5. C: Therefore, G (God actually exists).

## The S5 Axiom Controversy

The argument depends on S5, specifically ◇□P → □P. Whether this axiom is true for metaphysical modality is itself contested. In weaker modal systems (S4, B), the argument does not go through. Defenders of the argument argue that S5 is the appropriate logic for metaphysical possibility and necessity, and there is significant support for this in the philosophical literature on possible worlds. Critics argue that the accessibility relation between possible worlds may not be universal — the assumption underlying S5 — in which case ◇□P does not entail □P.

## Key Primary Sources

- Alvin Plantinga, *The Nature of Necessity* (1974), Chapter X
- Alvin Plantinga, *God, Freedom, and Evil* (1974), Part II
- Norman Malcolm, "Anselm's Ontological Arguments," *Philosophical Review* (1960)
- Charles Hartshorne, *Man's Vision of God* (1941) and *The Logic of Perfection* (1962)
- J. Howard Sobel, *Logic and Theism* (2004), Chapter 4
- Graham Oppy, *Ontological Arguments and Belief in God* (1995)
