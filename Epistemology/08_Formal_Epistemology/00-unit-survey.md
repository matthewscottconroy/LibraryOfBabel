# Unit 8: Formal Epistemology — Unit Survey

## Central Questions

Formal epistemology brings the tools of probability theory, decision theory, and modal logic to bear on classical epistemological questions. The central claim is that the concept of a *degree of belief* (credence) is at least as important as the concept of a binary belief (believing or not believing), and that rational norms govern credences — specifically, the laws of probability.

The central questions are: Should rational agents have beliefs that conform to the probability calculus? What is the best argument that they should (Dutch book, accuracy, calibration)? How should agents update their credences in light of new evidence — is Bayesian conditionalization the uniquely rational update rule, or is there a role for non-Bayesian updating? How should we reason with both precise probabilities and imprecise probabilities (sets of credence functions)? And what do formal models tell us about classical epistemological puzzles such as the preface paradox, the lottery paradox, and the relationship between credence and binary belief?

This unit is more technically demanding than the preceding units, requiring familiarity with basic probability theory and some exposure to decision theory. But it is not merely technical — the formal tools illuminate genuine philosophical questions and generate new ones. The preface paradox (it can be rational to believe each proposition in a book while also believing that at least one is false) and the lottery paradox (it seems rational to believe of each lottery ticket that it will lose, but irrational to believe that some ticket will win) are genuine puzzles about the relationship between credence and binary belief that resist purely technical resolution.

## Map of Major Positions and Their Logical Relations

**Bayesian Epistemology.** The core commitments: (1) rational agents have credences that satisfy the probability axioms (coherence); (2) rational agents update by conditionalization: new credence(p) = old credence(p | e), where e is the total evidence received. The Dutch book argument (Ramsey, de Finetti) provides a pragmatic justification: an agent whose credences violate the probability axioms is vulnerable to a set of bets (a "Dutch book") that guarantee a net loss. Joyce's accuracy argument provides an epistemic justification: any credence function that violates the probability axioms is accuracy-dominated by one that satisfies them.

**Objective vs. Subjective Bayesianism.** Subjective Bayesianism (de Finetti, Jeffrey): any prior credence function that satisfies the probability axioms is rationally permissible. The emphasis is on coherent updating rather than on correct priors. Objective Bayesianism (Jaynes, Williamson): the prior credence function is constrained by a principle of maximum entropy or indifference — in the absence of evidence, credences should be as uniform as possible.

**The Problem of Priors.** If any prior is permissible (subjective Bayesianism), how can Bayesian updating generate agreement from diverse starting points? The Bayesian convergence results show that agents with different priors will converge to the same posterior given enough evidence, provided both assign non-zero probability to the truth. But critics argue that this requires too much evidence and that the permissiveness of subjective Bayesianism allows irrational priors.

**Accuracy-First Epistemology.** Joyce (1998) argued that the Dutch book argument is too pragmatic — it grounds the rationality of probabilistic credences in a prudential norm, not a purely epistemic one. The accuracy argument holds that credences that violate the probability axioms are accuracy-dominated: there exists an alternative credence function that is closer to the truth at every possible world. This provides an epistemic (non-pragmatic) justification for Bayesian norms.

**The Preface Paradox.** An author who has carefully researched a non-fiction book may rationally believe each claim in the book while also rationally believing, in the preface, that the book contains at least one error. This generates a contradiction (the author believes each proposition in a large set and also believes not all of them). Resolution strategies: revise the logic of belief (use credences, not binary belief); accept that rational belief does not require consistency; accept that rational belief requires only locally consistent commitments.

**The Lottery Paradox.** Suppose a fair lottery with 1,000,000 tickets. For each ticket, the probability of winning is 1/1,000,000, so it seems rational to believe (with very high credence) that each ticket will lose. But it is known that exactly one ticket will win, so believing of each ticket that it will lose implies believing a known contradiction. This illustrates the tension between binary and credence-based models of belief.

**Sleeping Beauty.** A paradigm case for debates about self-locating belief. Sleeping Beauty is told about an experiment: she will be put to sleep; if a coin lands heads, she will be woken once; if tails, twice (with memory erased between wakings). When she wakes and is asked about her credence that heads landed, should she say 1/2 (halfer position) or 1/3 (thirder position)? The problem is live and unresolved; it bears on how to incorporate information about one's own position in time.

**Epistemic Logic.** Modal logic for knowledge and belief: K(φ) for "the agent knows φ", B(φ) for "believes φ." The system S5 for knowledge (serial, transitive, Euclidean accessibility relation) is the standard. Epistemic logics are used to model multi-agent knowledge in game theory, computer science, and social epistemology.

**Belief Revision (AGM theory).** The AGM framework (Alchourrón, Gärdenfors, Makinson) specifies rationality postulates for belief revision: how should an agent revise their beliefs when they receive new information that is inconsistent with their current beliefs? The postulates constrain but do not uniquely determine rational revision. Belief revision theory provides a formal framework intermediate between binary belief and Bayesian credence.

## Sub-Chapter Contributions

**Unit 1: Bayesian Epistemology.** Chapter 1 presents the probability calculus and the concept of credence in detail. Chapter 2 develops Bayesian conditionalization and the arguments for it (Dutch book, convergence). Chapter 3 examines the problem of priors and the debate between subjective and objective Bayesianism.

**Unit 2: Epistemic Logic.** Chapter 1 introduces the syntax and semantics of epistemic logic, including the accessibility relation and the main axioms (T, 4, 5, D, B). Chapter 2 addresses common knowledge and its role in game theory and social epistemology. Chapter 3 examines the logic of justified belief and the formal representation of the Gettier problem.

**Unit 3: Belief Revision / Formal Justification.** Chapter 1 presents the AGM framework and its rationality postulates. Chapter 2 examines the relationship between belief revision and Bayesian updating. Chapter 3 addresses formal approaches to justification, including epistemic utility theory.

## Key Philosophers and Arguments

**Frank P. Ramsey (1903–1930).** "Truth and Probability" (1926) founded the subjective Bayesian framework. Ramsey proposed that credences are measurable through betting dispositions and showed that coherent credences must satisfy the probability axioms. The Dutch book argument as standardly presented derives from Ramsey's insight, though the argument was sharpened by de Finetti and others.

**Bruno de Finetti (1906–1985).** *Theory of Probability* (2 vols.) developed subjective Bayesianism most thoroughly. De Finetti's exchangeability theorem provides a bridge from subjective to frequentist probability: a sequence of exchangeable random variables has a unique representation as a mixture of i.i.d. sequences, recovering frequentist methods within a subjective framework.

**Leonard J. Savage (1917–1971).** *The Foundations of Statistics* (1954) unified Bayesian credences with decision theory, deriving both credences and utilities from preferences. Savage's "sure-thing principle" — if you prefer act A to act B regardless of the state of the world, you should prefer A to B — became the focal point for debates about decision under uncertainty.

**James M. Joyce (1960–).** "A Nonpragmatic Vindication of Probabilism" (*Philosophy of Science*, 1998) provided the accuracy-dominance argument for Bayesian credences. Joyce argued that any credence function violating the probability axioms is accuracy-dominated: there is a credence function satisfying the axioms that is closer to the truth at every possible world under any Brier-type scoring rule. This bypassed the pragmatic assumptions of the Dutch book argument.

**David Lewis (1941–2001).** The "Principal Principle" (*Philosophical Papers*, 1980): rational credences must defer to known objective chances. If one knows that the objective chance of event E is p, one's credence in E should be p (in the absence of "inadmissible" evidence). This bridges subjective credence and objective probability.

**Henry Kyburg (1928–2007).** The lottery paradox (from *Probability and the Logic of Rational Belief*, 1961) is Kyburg's contribution. He proposed a threshold view: believe p iff credence in p exceeds some threshold t. This generates the paradox when applied to large lotteries.

**David Christensen (1956–).** *Putting Logic in Its Place* (2004) addresses the tension between formal requirements of rationality and the psychological reality of human belief. Christensen's work on peer disagreement (with Elga) connects formal epistemology to social epistemology: should agents revise their credences when they discover that peers with equal evidence have reached different conclusions?

**Adam Elga (1971–).** "Self-Locating Belief and the Sleeping Beauty Problem" (*Philosophy and Phenomenological Research*, 2000) introduced the Sleeping Beauty problem and defended the thirder position. The problem is now a focal point for debates about rational belief under uncertainty about one's own location in time.

## Essential Readings

1. **Joyce, James M., "A Nonpragmatic Vindication of Probabilism," *Philosophy of Science*, 65:4 (1998), pp. 575–603.** The accuracy-dominance argument for probabilism. More philosophically satisfying than the Dutch book argument because it grounds the norm epistemically rather than pragmatically. Essential for formal epistemology.

2. **Lewis, David, "A Subjectivist's Guide to Objective Chance," in *Philosophical Papers* Vol. II** (1980, Oxford). The Principal Principle and its defense. Essential for understanding the relationship between subjective credence and objective probability.

3. **Christensen, David, *Putting Logic in Its Place*, Chapters 1–3** (2004, Oxford). The most careful treatment of the relationship between logic and rational belief. Chapters 1–3 address the lottery and preface paradoxes.

4. **Elga, Adam, "Self-Locating Belief and the Sleeping Beauty Problem," *Philosophy and Phenomenological Research*, 62:2 (2000), pp. 153–164.** The canonical presentation of the Sleeping Beauty problem and the thirder argument.

5. **van Fraassen, Bas, "Belief and the Will," *Journal of Philosophy*, 81:5 (1984), pp. 235–256.** Develops the reflection principle — a rational agent's current credence in p should equal their expected future credence in p — and examines its connection to Bayesian updating.

## Open Questions and Live Debates

**Is there a uniquely correct prior?** Objective Bayesianism holds that there is; subjective Bayesianism denies it. The debate about priors is connected to the epistemological question of whether there is a priori knowledge of probabilities (Lewis's "chance" and "credence" concepts; indifference principle).

**Is Bayesian conditionalization the uniquely rational update rule?** Jeffrey conditionalization (for uncertain evidence) and other non-standard update rules have been proposed. The question of whether any update rule can be uniquely mandatory connects to debates about diachronic rationality.

**Does the Sleeping Beauty problem have a correct answer?** The halfer/thirder debate has generated a large literature without convergence. Some philosophers argue that the disagreement reflects genuine indeterminacy in the concept of rational credence under self-locating uncertainty.

**What is the relationship between credence and binary belief?** The standard Bayesian framework works with credences; folk epistemology and action theory work with binary belief. Lockean thresholds, stability accounts (Leitgeb), and dualist approaches (both credence and belief are fundamental) offer different resolutions.

## Connections to Other Units

- **Unit 1 (Foundations):** Bayesian epistemology is, in part, a response to the problem of induction that Hume identified. The question of rational learning from experience is reframed in probabilistic terms.
- **Unit 4 (Justification):** Bayesian models of justified belief revision interact with classical models of internalism and coherentism. The coherence of a belief system has a natural Bayesian analog in terms of joint probability.
- **Unit 5 (Skepticism):** Probability-based approaches to knowledge face lottery-like challenges: even very high credence does not yield knowledge. The relationship between credence and knowledge is central to both units.
- **Unit 9 (Social Epistemology):** Peer disagreement is analyzed both in formal epistemology (what credences should agents adopt after learning of peer disagreement?) and social epistemology (what social structures promote epistemic reliability?).
