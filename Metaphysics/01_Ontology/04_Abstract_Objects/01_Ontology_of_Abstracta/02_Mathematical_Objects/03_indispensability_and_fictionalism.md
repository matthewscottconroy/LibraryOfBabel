# Indispensability and Fictionalism

*Two contrasting views on mathematical ontology based on the role of mathematics in science.*

---

The Quine-Putnam indispensability argument and Hartry Field's fictionalist response form one of the most important exchanges in contemporary philosophy of mathematics.

Quine held that we should believe in the existence of whatever entities our best scientific theories are ontologically committed to. Our best physical theories — quantum mechanics, general relativity, statistical mechanics — are couched in the language of real numbers, differential equations, Hilbert spaces, and group theory. If these theories are true (not merely useful instruments) and if they quantify over mathematical objects, then mathematical objects exist. Putnam strengthened the argument: mathematics is not merely a convenient notation but is indispensable — there is no known way to state the laws of physics without quantifying over mathematical entities.

Field's response in *Science Without Numbers* (1980) is the most sophisticated nominalist challenge. Field argues that mathematics is *conservative* with respect to scientific theories — adding mathematical axioms to a purely nominalist physics does not yield any new nominalist consequences. If mathematics is conservative, then its indispensability is merely practical (it makes calculations easier) rather than theoretical (it commits us to new empirical claims). Field also provided a reconstruction of Newtonian gravitational theory using only geometric relations among spacetime points, without quantifying over numbers.

This project faces technical difficulties: quantum mechanics, with its Hilbert spaces and complex numbers, is much harder to nominalize. Field acknowledges the challenge but maintains that the difficulty is technical rather than in-principle.

*Fictionalism* (Balaguer, Yablo): mathematical statements are, taken at face value, false — there are no mathematical objects. But we should treat mathematics as useful fiction, and mathematical practice does not require the fiction to be true. The relationship between mathematical "truths" and genuine truths is analogous to the relationship between the *in-story truths* of a novel (Sherlock Holmes lived in Baker Street) and genuine truths about the world. Mathematics is a supremely useful story; its usefulness is explained by its structural applicability to physical reality, not by its truth.

Both sides of the indispensability debate accept the same methodology: start with our best science and read off the ontological commitments. The disagreement is about whether mathematics is a genuine part of our best science or a merely useful tool.

## The Quine-Putnam Argument Formalized

The indispensability argument can be stated in its canonical form:

**P1**: We ought to have ontological commitment to all and only those entities that are indispensable to our best scientific theories.

**P2**: Mathematical entities (numbers, sets, functions, spaces) are indispensable to our best scientific theories.

**C**: We ought to have ontological commitment to mathematical entities.

The argument is deductively valid. Both premises have been disputed.

**Against P1**: Quine's confirmational holism is required to support P1. On holism, every component of a scientific theory receives empirical confirmation together with every other component — confirmation is distributed holistically across the theory as a unit. If holism is correct, then mathematical components of physical theories receive empirical confirmation just as the physical components do, and we are committed to whatever entities make those components true. Maddy (*Realism in Mathematics*, 1990) argues that scientific practice contradicts holism: scientists regularly treat mathematical components of theories as idealizations they do not take to be literally true (they use continuous mathematics for phenomena they believe are fundamentally discrete; they use infinite idealizations for finite systems). If scientists do not take the mathematical components to be literally true, holism cannot generate ontological commitment to mathematical objects.

**Against P2**: Field's nominalization program challenges the claim that mathematics is genuinely indispensable. If classical mechanics can be reformulated without quantifying over numbers — using only qualitative comparisons of physical magnitudes among concrete spacetime regions — then mathematics is not indispensable to that theory, merely convenient. The extension of this program to quantum mechanics is the central contested question.

## Field's Conservativeness Claim

Field's central technical claim is that mathematics is *conservative*:

**Definition**: A mathematical theory M is conservative with respect to a nominalist theory N iff for any nominalistic statement S, if N + M ⊢ S, then N ⊢ S.

In other words, adding mathematics to a nominalist scientific theory generates no new nominalist consequences. Mathematics is a representational aid — a way of compactly expressing and computing what is already entailed by the nominalist theory alone.

Field proved conservativeness for his nominalized version of Newtonian mechanics using a metatheorem from mathematical logic: if a mathematical theory is consistent with any additional axioms about physical objects (it does not make false predictions about concrete reality), then adding it to a physical theory is conservative.

The proof uses model theory. If N is a nominalist theory with a model M, then M can be extended to a model of N + Mathematics (by adding the standard mathematical universe alongside the physical objects of M). If N + M ⊢ S for a nominalist S, then S is true in all models of N + M, including models extended from models of N — so S is true in all models of N, meaning N ⊢ S.

**The limitation**: This proof works only for mathematics that can be formulated without making new claims about physical objects. It does not show that mathematics with direct physical content (such as mathematics that asserts facts about the density or cardinality of physical spacetime regions) is conservative.

## Maddy's Naturalist Objection

Penelope Maddy's naturalism poses a challenge to both sides of the indispensability debate. Maddy argues that the proper methodology for mathematics is to look at mathematical practice itself — not to approach mathematics from the perspective of external philosophical requirements.

Mathematical practice does not conform to what the indispensability argument predicts. Scientists use continuous mathematics to describe phenomena they believe are actually discrete; they assume infinite quantities for finite physical systems; they invoke mathematical entities (delta functions, point masses) they know are not strictly physical. If scientists took mathematical ontology seriously as the indispensability argument requires, they would not use mathematical tools they know to be idealized.

Maddy concludes: the indispensability argument does not generate genuine ontological commitment to mathematical objects, because the mathematical components of physical theories are not treated as straightforwardly true by the scientists who use them. This leaves room for Maddy's own position — *naturalistic Platonism* — which grounds mathematical ontology in mathematical rather than scientific practice.

## Yablo's Figuralism and the No-Commitment View

Stephen Yablo's *figuralism* develops an alternative to both the indispensability argument and Field's fictionalism. Yablo argues that mathematical discourse is not even an attempt at literal truth — mathematical sentences are *figurative expressions* of concrete truths.

"The number of moons of Mars is 2" is figuratively expressed; what it really says, in literal terms, is that Mars has two moons — a concrete fact. The mathematical packaging is a figure of speech, a conventional way of expressing cardinality facts about concrete objects. Similarly, "the function f(x) = x² has a global minimum at x = 0" figuratively expresses something about how certain quantities covary.

On Yablo's view, mathematical sentences are neither true nor false *as literally interpreted* — they are figuratively appropriate or inappropriate. This avoids Field's awkward claim that mathematics is *false*: mathematics is not false, it is figuratively apt. It also avoids the indispensability argument's conclusion: the scientific use of mathematics does not commit us to mathematical objects any more than the use of other figurative expressions commits us to the existence of what those expressions literally describe.

**Challenge**: The figuralist must provide principled criteria for which mathematical expressions have figurative readings. Not all mathematical claims seem paraphrasable into concrete truths — claims about higher set theory, for instance, seem too abstract to have concrete figurative content. Yablo's response is that the relevant "figurative content" is often schematic — expressing something about the logical structure of concrete reality rather than any particular concrete fact.

## Beyond the Debate: Deflationary Approaches

Some philosophers argue that the indispensability debate rests on a questionable premise — that the point of quantifying over mathematical entities in scientific theories is to make ontological commitments. Jody Azzouni defends *ontological deflation*: quantification over mathematical objects in a scientific theory does not automatically commit us to the existence of those objects. Ontological commitment requires more than quantification — it requires *thick epistemic access*, some account of how we could in principle come to know about the objects. Mathematical objects fail this criterion.

On Azzouni's view, we can say "there exist prime numbers between 10 and 20" without being committed to abstract primes, just as we can say "the average American has 2.3 children" without being committed to the existence of fractional children. The quantification is nominal, not ontologically committing.

This deflationary approach reopens the question of what ontological commitment really consists in — whether it is semantic (a matter of what our theories quantify over) or epistemological (a matter of what we need to posit to explain our knowledge) or something else entirely.
