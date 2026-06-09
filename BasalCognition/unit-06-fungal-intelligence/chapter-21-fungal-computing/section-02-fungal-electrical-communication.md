# Section 2: Fungal Electrical Communication and the "Language" Hypothesis

## Spike Trains in Mycelium

In 2022, Andrew Adamatzky published a paper in *Royal Society Open Science* with a striking title: "Language of fungi derived from their electrical spiking activity." The paper reported that electrical spikes recorded from mycelium of several fungal species were not randomly distributed in time but showed clustering patterns that, when analyzed with tools borrowed from computational linguistics, yielded a distribution resembling that of human words in natural language. The paper proposed that these spike patterns might constitute a form of communication — a "language" with a vocabulary of up to fifty "words."

The paper received enormous popular attention. Major news outlets covered it under headlines like "Fungi might be talking to each other in electrical language" and "Scientists find fungi communicate using 50 words." Science communicators on social media amplified the claim. Within days, the idea of fungal language had propagated far beyond the scientific literature.

The scientific reception was far more cautious. Many mycologists and neuroscientists responded with significant skepticism, not toward the underlying electrical measurements — which appear methodologically sound — but toward the interpretive leap from "structured electrical spike patterns" to "language."

To understand this controversy, we need to examine both what the data show and what "language" means.

## What the Data Actually Show

Adamatzky's group implanted small iridium-coated steel electrodes into the mycelium of several fungal species, including ghost fungi (*Omphalotus nidiformis*), caterpillar fungus (*Ophiocordyceps*), and split-gill fungus (*Schizophyllum commune*). The electrodes recorded extracellular electrical potentials over periods of hours to days.

What they recorded were discrete electrical spikes: transient changes in extracellular potential with amplitudes in the millivolt range and durations of several minutes. These spikes were not continuously present; they occurred in bursts separated by quiet periods. The temporal structure of the spike trains was not random: spikes within a burst tended to be more regularly spaced than chance would predict, and the intervals between bursts showed a distribution that was not exponential (which would be expected for a random Poisson process).

Adamatzky then applied an analysis drawn from computational linguistics: he asked whether the distribution of spike-train "words" — defined as clusters of spikes separated by inter-spike intervals above a threshold — resembled the distribution of words in human language. The relevant statistical property is Zipf's law: in natural language, the frequency of a word is inversely proportional to its rank in the frequency table, a relationship that holds across languages and even across some non-linguistic information sources. Adamatzky found that the spike-train "words" in fungal electrical activity showed a distribution consistent with Zipf's law.

This is the core of the data. It is worth being very precise about what this does and does not show.

## What the Data Do Not Show

Zipf's law is ubiquitous. It appears not only in natural language but in the frequency distributions of cities by population, earthquakes by magnitude, and income by wealth rank. It emerges from a wide variety of generative processes, some of which involve communication and many of which do not. The fact that fungal spike trains show a Zipf distribution is, by itself, no evidence that these spike trains constitute language in any meaningful sense.

To claim that a signal system constitutes language — or even that it constitutes communication — requires several things that the Adamatzky data do not provide:

**Semantic content**: Each "word" in a language refers to something outside the signal system. The word "danger" in bee dance communication corresponds to a real property of the environment. What does each of the up to fifty "words" in fungal spike trains refer to? The paper does not establish any mapping between specific spike patterns and specific environmental states or internal states of the fungus. Without such a mapping, the word "word" is purely formal — it means only "a cluster of spikes by a particular operational definition," not "a unit of meaningful information."

**Receiver**: Language is communication, and communication requires a receiver that is changed by the signal in an appropriate way. Who receives the fungal "words"? Other hyphae? The broader mycelium? Some other organism? The paper does not address this. We have evidence that electrical signals propagate through mycelium, but not that specific spike patterns elicit specific responses in distant parts of the network.

**Arbitrariness and convention**: In human language, the relationship between a word's form and its meaning is largely arbitrary and conventional — "cat" means cat only because English speakers agree that it does. In chemical signaling, by contrast, the relationship between signal and response is mechanistic: a specific molecule triggers a specific receptor. Fungal electrical spikes almost certainly work mechanistically, not conventionally. Calling the pattern units "words" imports the wrong conceptual framework.

**Encoding vs. correlation**: The spike patterns are almost certainly correlated with environmental or physiological conditions — that is what makes their structure non-random. But correlation between a signal and a state of the world is not the same as encoding. A barometer correlates with weather but does not communicate information in the sense that a weather forecast does.

None of this means the spike patterns are unimportant. Structured, non-random electrical activity in mycelium is genuinely interesting, and the question of what that structure represents is worth investigating. The criticism is not of the observations but of the interpretive language — specifically, of the claim that these patterns constitute "language" in any sense that is meaningfully comparable to human or animal communication.

## Scientific Reception

The reception of Adamatzky's 2022 paper within the scientific community ranged from cautious interest to outright skepticism. Several mycologists publicly questioned the language interpretation while acknowledging the interest of the electrical measurements.

The core of the scientific criticism was that the paper had not established the minimum requirements for demonstrating communication — semantic content, a receiver, and evidence that the signal actually influences the behavior of the recipient. Without these elements, applying the label "language" was regarded by many researchers as premature at best and misleading at worst.

Adamatzky and colleagues have acknowledged these limitations while maintaining that the findings open productive research directions. This is, on its face, a reasonable position: if mycelium produces structured electrical signals, then characterizing those signals and investigating their function is worthwhile science, even if the "language" framing turns out to be an overreach.

There is also a legitimate concern about the role of popular media amplification. When a paper in a respected journal proposes that fungi communicate using a language of fifty words, it will be reported by non-specialist journalists in ways that strip away nuance and qualification. The scientific community bears some responsibility for framing its work in ways that resist such distortion — and the "language of fungi" framing, whatever its methodological merits, clearly did not resist it.

## Responses to Environmental Stimuli

Setting aside the language debate, the evidence for environmental stimulus response in fungal electrical activity is more straightforward and more defensible.

Adamatzky and others have documented that electrical spiking activity in mycelium changes in response to localized environmental stimuli. Chemical attractants (nutrients) applied to one part of a mycelium alter the pattern of spiking in other parts, with a delay consistent with signal propagation at the observed rate. Mechanical stimulation — touching or wounding a hyphal region — generates propagating electrical disturbances. Exposure to light alters spiking patterns in several species (Adamatzky, 2018).

These responses are consistent with a signaling function: a local event generates a network-wide change in electrical activity that could, in principle, coordinate a network-wide behavioral response. Whether it does so — whether the electrical response to a stimulus actually influences growth or other behaviors in distant parts of the mycelium — is less clearly established and remains an important open question.

The distinction matters. In a neural system, we can follow the chain from stimulus to electrical signal to behavioral output at each step. We know that action potentials in sensory neurons lead (through complex circuits) to motor output. In the mycelium, we have good evidence for the first step (stimulus generates electrical activity) and circumstantial evidence for the last step (mycelium responds adaptively to stimuli), but the middle of the chain — how electrical signals translate into growth behavior — is not well understood.

## The Honest Summary

What can we confidently say about fungal electrical communication?

1. Fungal mycelium generates discrete electrical spikes that propagate through the network. This is well-established by multiple research groups.

2. The temporal structure of spike trains is non-random and shows clustering patterns. This is established by Adamatzky's data and consistent with, though not proved by, other work.

3. Spiking activity responds to environmental stimuli in ways consistent with a signaling function. This is reasonably well-established.

4. The specific claim that spike patterns constitute a "language" analogous to human or animal communication is not supported by the available evidence. The Zipf distribution alone is insufficient to support this claim, and the semantic, receiver, and encoding requirements for language have not been met.

5. The functional role of electrical spike patterns in coordinating mycelial behavior remains an important open question.

This is not a dismissal of the research. It is a calibration of the evidence. The honest position is that something interesting is happening in those electrode traces. What exactly it means is not yet known.

---

## References

Adamatzky, A. (2018). Towards fungal computer. *Interface Focus*, 8(6), 20180029.

Adamatzky, A. (2022). Language of fungi derived from their electrical spiking activity. *Royal Society Open Science*, 9(4), 211926.
