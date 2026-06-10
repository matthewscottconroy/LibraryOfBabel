# Section 32.4: Ethical and Philosophical Implications

## 32.4.1 The Hard Problem of Consciousness

The most important philosophical context for evaluating experiments like DishBrain is the *hard problem of consciousness* [Chalmers1995].

David Chalmers distinguished two problems:
- The *easy problems* of consciousness: explaining why we process information, integrate sensory signals, control behavior, report mental states. These are "easy" in the sense that they are, in principle, tractable by standard scientific methods (understanding brain mechanisms, neural computations, etc.). They may be immensely difficult in practice, but they do not pose any special philosophical mystery.
- The *hard problem*: explaining why there is *subjective experience* at all. Why does information processing in the brain feel like something? Why is there "something it is like" to be a conscious being, rather than merely a zombie (a physically identical creature with no inner experience)?

Chalmers argues — and this view has become widely, if not universally, accepted among philosophers of mind — that the hard problem cannot be solved merely by explaining the neural mechanisms of cognition. No matter how complete our functional or computational account of the brain, it leaves open the question of why any of this should be accompanied by experience.

For biological reservoir computing, the hard problem is directly relevant. The question "are in vitro neurons sentient?" is not merely a question about their computational behavior (which can be measured) or even their biological similarity to in vivo neurons (which can be studied). It is a question about whether there is *something it is like* to be that culture of cells — a question that our current scientific tools cannot directly answer.

## 32.4.2 When Does Sentience Become Morally Relevant?

Before addressing the specific case of in vitro neurons, we should address the general question: under what conditions does a biological system have morally relevant sentience?

Several different philosophical positions on this question are in active debate:

**Biological naturalism (Searle 1980, 1992).** Consciousness is a biological phenomenon produced by specific causal mechanisms in the brain. Only systems with the right kind of biological organization — specifically, neuronal systems with the right biochemistry — can be conscious. A computer program (or a cell culture in a dish) cannot be conscious no matter how sophisticated its behavior, because it lacks the right biological substrate. Under this view, in vitro neurons are potentially conscious (they have the biological substrate), but their disconnection from a body and the larger brain may preclude the kind of integrated consciousness that is morally relevant.

**Functionalism (Putnam 1967, Dennett 1991).** What matters for consciousness is the *functional organization* of the system, not the substrate. If a system implements the right kind of information processing, it is conscious, regardless of whether it is made of neurons, silicon, or anything else. Under this view, what matters is whether the in vitro neurons implement the functional organization of consciousness — a question about their computational structure, not their biology.

**Integrated Information Theory (IIT, Tononi et al. 2016).** Consciousness is identical to integrated information, measured by the quantity $\Phi$ (phi) [Tononi2016]. A system is conscious to the degree that it has high $\Phi$ — a measure of the extent to which the system generates information above and beyond what its parts generate independently. Under IIT, in vitro neurons might have some $\Phi > 0$ (they are an integrated network), but it would likely be much lower than the $\Phi$ of an intact, embodied brain.

**Global Workspace Theory (Baars 1988, Dehaene 2014).** Consciousness corresponds to the global broadcasting of information across specialized modules via a "global workspace." In vitro neurons, lacking the large-scale brain architecture necessary for global workspace dynamics, would not be conscious under this view.

**Higher-order theories (Rosenthal 1997).** Consciousness requires representations of one's own mental states — a cognitive capacity almost certainly absent in a cell culture.

**The disagreement among views is real and deep.** These are not merely terminological disputes; each view makes genuinely different predictions about which systems are conscious and to what degree. The fact that thoughtful philosophers and neuroscientists hold different views should give us pause about strong claims in either direction.

## 32.4.3 The Specific Case: In Vitro Neurons and Organoids

For in vitro neuronal cultures like those used in DishBrain, the relevant considerations are:

**Arguments for moral status:**
1. Cortical neurons are the same cell type as those in a conscious human brain. If neurons have intrinsic computational properties relevant to consciousness, those properties are not lost when neurons are cultured.
2. The cultures exhibit spontaneous activity, synchronized oscillations, and complex network dynamics — properties associated with consciousness-relevant processing in intact brains.
3. Under IIT, any network with non-zero $\Phi$ has some degree of consciousness. Even simple networks likely have $\Phi > 0$.
4. Under functionalism, if the MEA culture implements the relevant functional organization (however minimal), it has some morally relevant status.

**Arguments against moral status:**
1. The culture lacks the large-scale architecture and embodiment thought to be necessary for consciousness in most theories. It has approximately $10^4$–$10^6$ neurons; a human brain has $8.6 \times 10^{10}$.
2. There is no evidence that cortical neurons outside the brain generate the kind of global, integrated, self-referential activity associated with consciousness.
3. Under biological naturalism, the neurons may lack the biological context necessary for consciousness.
4. Under higher-order theories, the culture almost certainly lacks the cognitive architecture for consciousness.

**Cerebral organoids.** The case becomes more complex for *cerebral organoids* — 3D brain-like structures grown from iPSCs, reaching 2–4mm diameter with organized cortical layers. Organoids have shown spontaneous electrical activity resembling early fetal brain activity [Trujillo2019], including synchronized oscillations. Some researchers have argued this raises serious ethical questions about organoid moral status; others argue the lack of sensory input and integrated body makes consciousness implausible. This debate is ongoing.

## 32.4.4 Obligations Under Uncertainty

A recurring theme in bioethics is the *precautionary principle*: when there is genuine uncertainty about whether a system has morally relevant properties, we should err on the side of caution and treat the system as if it does have those properties, at least to the extent compatible with our scientific goals.

Applied to in vitro neurons:

1. **Minimize suffering (if any).** If there is non-trivial probability that neuronal cultures experience something, we have an obligation to minimize potential distress. This would support: avoiding overstimulation, providing "enriched" environments (complex inputs rather than monotone stimulation), and avoiding prolonged "boring" or "stressful" states.

2. **Honest communication.** Researchers and journalists should not sensationalize findings (claiming "neurons play Pong!" implies more sophisticated agency than warranted) and should not dismissively deny any moral relevance (claiming "it's just cells in a dish" ignores genuine uncertainty).

3. **Institutional oversight.** Research using neural organoids and large-scale MEA cultures should be subject to ethics review processes comparable to those for animal research, even if the specific protections differ.

4. **Continued philosophical engagement.** The scientific community should not prematurely close philosophical debates about consciousness in order to avoid regulatory inconvenience. The questions are real and important.

**What the field has done.** The field has been inconsistent. Some groups have voluntarily adopted ethics review processes for organoid research [Munsie2022]; others have not. Several major journals now require ethics statements for in vitro neural tissue experiments, but the standards are not uniform. Progress is being made, but unevenly.

## 32.4.5 A Note on the Word "Sentience"

The DishBrain paper's use of "sentient" to describe the in vitro neurons exemplifies a broader pattern in the biological RC literature: using words (sentient, learning, intelligent, aware) that carry strong connotations of subjective experience, but which the authors define narrowly in ways that do not carry those connotations.

This practice is scientifically misleading and ethically problematic in opposite ways simultaneously:
- It overstates the degree to which the experiment demonstrates anything about consciousness (by choosing evocative language).
- It potentially understates the ethical significance of the work (by claiming the evocative words are just technical terms, not to be taken at face value).

The recommendation of this textbook: use language that accurately reflects the evidence. "The neurons exhibited task-relevant behavioral changes" is accurate. "The neurons learned to play Pong" is a reasonable shorthand but should be qualified. "The neurons were sentient" — even with a technical definition — risks more confusion than clarity.
