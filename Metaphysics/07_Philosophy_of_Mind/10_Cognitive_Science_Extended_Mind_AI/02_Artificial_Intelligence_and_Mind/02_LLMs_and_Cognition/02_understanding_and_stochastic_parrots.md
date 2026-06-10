# Understanding and Stochastic Parrots

A parrot can reproduce human sounds without understanding a word of what it says. The question Bender, Gebru, McMillan-Major, and Shmitchell raise in their 2021 paper is whether large language models are, in the relevant sense, doing the same thing at enormously greater sophistication — stochastically generating text that statistically resembles meaningful language without any of the meaning. The "stochastic parrots" label is a philosophical diagnosis, not a technical description: it claims that LLMs operate on the form of language without its content.

The underlying distinction is the classical one between syntax and semantics, or form and content. LLMs operate on the formal properties of text — statistical co-occurrences, surface structure, patterns of tokens — without grounding in the world that gives language its meaning. The argument has a simple structure:

- P1: Understanding language requires grasping the meaning of linguistic expressions.
- P2: Meaning is constituted by the relationship of expressions to things in the world.
- P3: LLMs process only formal, statistical relationships among expressions, not their relationships to things in the world.
- C: LLMs do not grasp the meaning of linguistic expressions; they do not understand language.

This connects directly to Searle's Chinese Room: the person in the room manipulates Chinese symbols according to formal rules without understanding what they mean. LLMs manipulate tokens according to their statistical relationships in training data without understanding what those tokens refer to.

## The Symbol Grounding Problem

The deepest issue is the **symbol grounding problem** (Harnad, 1990): how do symbols acquire meaning? In a purely formal symbol system, symbols are defined by their relationships to other symbols — an infinite regress of form with no semantic anchoring. Meaning requires that symbols be grounded — connected to the things they refer to in a way that goes beyond their relationships to other symbols. Human language is grounded: "red" is connected to perceptual experiences of redness; "chair" is connected to interactions with physical objects. This grounding gives language its content. LLMs lack this grounding: they know the statistical relationships among words but have no perceptual access to the things those words refer to. They generate text about red things without knowing what red looks like; text about pain without having felt any.

Bender et al. make a further point about the cultural dimension: LLM outputs are shaped by patterns in training data that reflect the biases, assumptions, and worldviews of those who wrote the text. LLMs do not understand; they reproduce, at vast scale, the patterns of their training corpora, amplifying whatever is overrepresented and underrepresenting whatever is absent. The fluency of LLM outputs is evidence of the statistical richness of the training data, not of understanding — the ELIZA effect at scale.

## Responses

The **functionalist response** holds that understanding is characterized by functional capacities — responding appropriately to questions, drawing relevant inferences, using concepts correctly in context. If LLMs exhibit these capacities, they exhibit a form of understanding that deserves the name. The distinction between "genuine" understanding and "sophisticated simulation" may not carve nature at its joints; what we call understanding in humans may also be a sophisticated form of pattern recognition, grounded in richer patterns but not different in kind.

**Distributional semantics** research (Landauer, Turney, Baroni) provides theoretical backing for this response: much of the semantic content of words can be recovered from their statistical distributions in large text corpora. "Meaning" is substantially captured by patterns of co-occurrence. If this is right, LLMs may be capturing genuine semantic relationships through statistical learning, not merely form without content.

The **emergent grounding response** observes that LLMs trained on text describing perceptual and physical experiences may have learned something about what those experiences are like through the statistical structure of that text. Text about red things encodes information about their salience, their typical contexts, the metaphors associated with them. This is not the same as direct perceptual grounding, but it may be sufficient for a form of semantic engagement.

The **scaling response** (LeCun and others) argues that the limitations of purely text-based LLMs reflect current architecture and training rather than a principled barrier. Multimodal training — incorporating images, video, sensorimotor data — might supply the worldly grounding that text alone lacks. The stochastic parrots critique may apply to current systems without applying to all possible AI systems.

## The Deeper Issue

The dispute is ultimately about what understanding requires. If understanding requires worldly grounding through direct perceptual and causal contact — if the meaning of "red" is necessarily tied to having experienced redness — then LLMs cannot understand. If understanding is a functional-dispositional property — using concepts appropriately, drawing the right inferences, responding correctly — then LLMs may understand in the relevant sense. This maps onto the debate between semantic externalism (Putnam, Burge) — content is partly constituted by relations to the environment — and inferentialism (Brandom) — content is constituted by inferential role within a conceptual system. The stochastic parrots critique implicitly endorses an externalist grounding view; its critics tend toward inferentialist or functionalist views. The LLM debate has made a classical abstract dispute concrete and practically urgent.
