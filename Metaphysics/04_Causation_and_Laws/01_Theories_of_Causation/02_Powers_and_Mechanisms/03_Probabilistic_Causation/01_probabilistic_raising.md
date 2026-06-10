# Probabilistic Causation: Probability Raising

Smoking causes lung cancer. We say this with confidence, yet we know perfectly well that not every smoker develops cancer and that some non-smokers do. The causal claim here is clearly not the claim that smoking is deterministically sufficient for cancer. It is something weaker: smoking significantly raises the probability of cancer. Probabilistic theories of causation hold that this probabilistic raising is not a pale approximation to the "real" causal relation but is itself what causation consists in (at least in these cases). The basic formula is P(E|C) > P(E|¬C) — the probability of E given C exceeds the probability of E given not-C.

This is not merely the science of an interesting special case. Quantum mechanics tells us that at the fundamental physical level, all causation may be probabilistic. Radioactive decay causes Geiger counter clicks, but it does so probabilistically. Any adequate theory of causation must accommodate cases like these.

## Suppes's Framework

Patrick Suppes, in *A Probabilistic Theory of Causality* (1970), built the first systematic account. He distinguished a **prima facie cause** (C is a prima facie cause of E iff P(E|C) > P(E) — C raises the unconditional probability of E) from a **genuine cause** (C is a genuine cause iff C is a prima facie cause and no factor Z screens off C from E). Screening off is the key concept: Z screens off C from E iff P(E|C & Z) = P(E|Z) — conditioning on Z renders C and E probabilistically independent.

The screening-off condition captures the common-cause case. The barometer's falling is a prima facie cause of the storm: P(Storm | Barometer falls) > P(Storm). But atmospheric pressure screens off the barometer from the storm: P(Storm | Barometer falls & Low pressure) = P(Storm | Low pressure). Atmospheric pressure is the genuine cause; the barometer is spurious.

## The Probability-Lowering Problem

The most important objection to the probability-raising account is this: some genuine causes lower the probability of their effects. Consider Rosen's 1978 example. A golfer hits a wayward shot heading for a sandtrap. The ball strikes a tree branch, deflects, and falls into the hole for a birdie. The branch-strike *lowered* the probability of the ball going into the hole — most such strikes send the ball further away. But the branch-strike was causally crucial to the birdie; it was part of the causal chain that produced it.

More generally: in a chain C → D → E, if C raises P(D) and D raises P(E), but C, via D, lowers P(E) overall (because D is not the most likely intermediary), then C lowers the probability of E while still causing it. The simple probability-raising account classifies C as a preventer of E, even if C was causally responsible for E's occurrence in the actual case.

## The Reference Class Problem

The reference class problem is endemic to probabilistic accounts. The probability of E given C depends on which reference class we use to define the background probabilities. What is the probability that this patient recovers given that they took aspirin? Reference class: all patients who took aspirin → P(Recovery | Aspirin) = 0.65. Reference class: all patients with this specific diagnosis who took aspirin → 0.71. Reference class: all patients matching this patient exactly → varies further.

Different reference classes yield different probabilities and hence different verdicts about whether aspirin caused recovery. The probability-raising account requires a principled specification of the reference class, but there is no uniquely correct one.

## Transitivity Problems

Probability-raising is not transitive, but causation intuitively is. If A raises the probability of B, and B raises the probability of C, it does not follow that A raises the probability of C — the chain may lower overall probability. Yet if A causes B and B causes C, surely A causes C.

## Responses

Ellery Eells, in *Probabilistic Causality* (1991), introduced a context-dependent account that defines causal relations relative to a specified background context. Causation is assessed by holding fixed all causally relevant background factors. This handles many cases but faces similar reference class difficulties in specifying the relevant background.

Lewis's later work incorporated probabilistic causation through "influence": C influences E if varying C changes the probability of E in various conditions. This captures the intuition that genuine causes make a probabilistic difference while allowing for cases where the overall probability change is negative.

## Significance for Philosophy of Science

Probabilistic causation is not merely a theoretical curiosity. It is essential for understanding epidemiology, where causal claims are established by showing probability-raising in controlled studies. It is essential for quantum mechanics, where probabilities are irreducible — if causation requires deterministic sufficiency, quantum events are causally ungrounded. And it is essential for risk assessment, where legal and ethical judgments of causal responsibility rely on probabilistic causal claims. The probability-raising framework, despite its problems, provides the formal structure for these assessments. The challenges it faces — the probability-lowering problem, the reference class problem, the transitivity failure — have driven the development of more sophisticated accounts. But the basic intuition that causes are probability-raisers remains one of the most productive ideas in the philosophy of causation.
