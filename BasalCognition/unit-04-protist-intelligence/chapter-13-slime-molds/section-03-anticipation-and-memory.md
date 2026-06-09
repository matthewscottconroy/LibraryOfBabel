# Section 3: Anticipation and Memory — Without a Single Neuron

## Introduction

In 2008, a paper appeared in *Physical Review Letters* — a physics journal, not a biology journal — that described something even stranger than maze-solving. Toshiyuki Nakagaki's group had exposed Physarum plasmodia to a series of periodic cold shocks: every 60 minutes, the temperature was lowered for 10 minutes (cold air slows the oscillation of cytoplasmic streaming). Predictably, the organism slowed down each time the cold hit.

Then the researchers stopped the cold shocks — but kept watching.

Physarum slowed down anyway. At approximately the time when the next cold shock would have arrived, the organism spontaneously reduced its streaming velocity. And it did this not once but twice or three times in succession before the pattern faded. The organism was, in some functional sense, anticipating the cold shock that never came (Saigusa et al., 2008).

This is a startling result, and it requires careful interpretation. We will engage with both the excitement and the appropriate skepticism.

---

## 3.1 The Experiment in Detail

The Saigusa et al. (2008) study began with a simple observation: Physarum's cytoplasmic streaming is temperature-sensitive, decreasing in frequency when the organism is cooled and increasing when it is warmed. Temperature therefore constitutes a stimulus that the organism reliably detects and responds to in a measurable way.

The training protocol was straightforward. Plasmodia were exposed to three periodic cold-warm cycles, each 60 minutes in duration. During the cold phase (10 minutes), streaming frequency dropped significantly. During the warm phase (50 minutes), it recovered. This produced a clear oscillatory pattern in the recorded streaming data, synchronized to the 60-minute temperature cycle.

After three cycles, the temperature was held constant and warm. The prediction from a naive model of Physarum as a simple stimulus-response system was that streaming would simply stabilize at its warm-temperature frequency. Instead, the organism's streaming frequency dipped approximately every 60 minutes — at the times when cold shocks had previously arrived — for two or three cycles before the spontaneous dips faded.

The effect was statistically significant and reproducible. Crucially, controls showed that the effect was specifically tied to the trained period: organisms trained to a 60-minute cycle showed spontaneous dips at 60 minutes, while organisms trained to a different period showed spontaneous dips at that period. The organism had, in some sense, learned the timing of the stimulus.

---

## 3.2 Mechanism: How Can a Protist Anticipate?

The most important question is mechanistic: what physical or biochemical process could implement this temporal anticipation in an organism with no neurons?

Saigusa et al. (2008) proposed that the answer lies in the oscillatory dynamics of cytoplasmic streaming itself. The key insight is that the spontaneous oscillation of streaming is not a simple sinusoid but a complex, potentially multi-frequency system. When the organism is exposed to periodic cold shocks, those shocks entrain the streaming oscillation — they lock it to the external period through resonance. Once entrained, the internal oscillation continues at the entrained frequency for some time after the entraining stimulus is removed, much as a pendulum continues to swing at its natural frequency after you stop pushing it at that frequency.

On this account, what Physarum is doing is not "remembering" the cold shock in any representational sense. It is not storing a time stamp or a prediction. Its internal oscillator has been phase-locked to the stimulus period, and the decay of that phase-locking produces the apparent anticipatory dips. The "memory" is implicit in the dynamics — encoded in the phase and amplitude of the streaming oscillation rather than in any discrete stored state.

This is conceptually similar to the way a tuning fork "remembers" a note it was struck at: it continues to vibrate at that frequency not because it has stored information about the note, but because its physical dynamics at that moment are dominated by that oscillation. The distinction between "remembering" and "continuing to oscillate" is not merely semantic — it maps onto a real difference in the physical substrate — but the functional consequence (the past stimulus influences future behavior) is the same.

More recent theoretical work has explored whether the tube network architecture of Physarum could implement something closer to genuine information storage — whether the pattern of tube diameters might encode past experience in a way that influences future network behavior (Oettmeier et al., 2017). This remains an open and interesting research question.

---

## 3.3 What Does "Memory" Mean Here?

The Saigusa experiment raises, in sharp form, a conceptual question that runs throughout this book: what do cognitive vocabulary terms mean when applied to organisms without nervous systems?

In the context of neuroscience, memory refers to a change in synaptic strength or connectivity that encodes past experience and allows it to influence future behavior. This is a specific mechanistic claim tied to a specific physical substrate. Physarum has no synapses, no synaptic plasticity, no hippocampus.

But behavioral memory is operationally defined not by its substrate but by its functional properties: behavior at time T2 is systematically related to experience at time T1, with T2 > T1, in a way that increases fitness. By this operational definition, Physarum's anticipatory behavior does look like memory. The past experience (cold shocks at 60-minute intervals) influences future behavior (spontaneous slowing at 60-minute intervals) in a way that would be adaptive in the natural environment (where periodic cold events might correlate with other threats or opportunities).

The philosophical question is whether the operational definition is sufficient for a concept like memory, or whether memory requires additional properties — representation, storage in a discrete physical state, the capacity for retrieval — that Physarum lacks.

There is no settled answer to this question. Different researchers come down on different sides, and the position one takes depends partly on one's broader philosophical commitments about what cognitive terms should mean. What is not defensible is simply assuming that Physarum "doesn't have memory" because it lacks neurons. The burden is to specify what properties are necessary for memory and then assess whether Physarum meets them — not to import the biological substrate as a hidden criterion.

---

## 3.4 Additional Evidence: Maze Re-navigation

Further evidence for something like experiential history in Physarum comes from studies of maze re-navigation. In some experiments, Physarum that had previously navigated a maze configuration was found to navigate it again more efficiently — using paths that it had previously reinforced — compared to its behavior on the first encounter (Nakagaki et al., 2004). If the tube network from the first navigation is partially preserved, the organism effectively starts the second navigation with a "head start" — its internal network architecture is already biased toward the efficient path.

This is perhaps better described as structural priming than memory in the full sense: the physical structure of the network retains information about past solutions. But again, the distinction between "stored in the network architecture" and "stored in synaptic weights" may be one of substrate rather than functional kind. Synaptic weights are, after all, physical structures of a network that encode past experience.

---

## 3.5 The Importance of Epistemic Care

It is important to be explicit about the limitations of the current evidence. The Saigusa experiment has been influential, but its interpretation remains contested. Some researchers have proposed that simpler explanations — such as non-specific effects of the training regime on the general oscillatory state of the organism — could account for the results without invoking anything analogous to temporal memory (Lyon, 2006).

Whether this weaker interpretation is correct depends on details of the experimental controls and the quantitative properties of the observed anticipatory dips. The field has not yet converged on a consensus mechanism.

What is not in dispute is the behavioral observation: Physarum, after training to periodic cold shocks, spontaneously exhibited behavioral changes at the trained period after the shocks ceased. How to interpret that observation — what concepts it licenses, what mechanisms it implies — remains an open question that is actively being investigated.

This is a perfectly appropriate situation for a young science at the frontier of what is known. The honest response is to describe what was observed, what mechanisms have been proposed, where the evidence supports those mechanisms, and where it does not. Premature certainty — in either direction — serves no one.

---

## References

Lyon, P. (2006). The biogenic approach to cognition. *Cognitive Processing*, 7(1), 11–29.

Nakagaki, T., Yamada, H., & Hara, M. (2004). Smart network solutions in an amoeboid organism. *Biophysical Chemistry*, 107(1), 1–5.

Oettmeier, C., Brix, K., & Döbereiner, H. G. (2017). Physarum polycephalum — a new take on a classic model system. *Journal of Physics D: Applied Physics*, 50(41), 413001.

Saigusa, T., Tero, A., Nakagaki, T., & Kuramoto, Y. (2008). Amoebae anticipate periodic events. *Physical Review Letters*, 100(1), 018101.
