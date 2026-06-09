# Section 30.2: Morphogenetic Intelligence

## The Shape Problem

There is a puzzle at the heart of developmental biology so familiar that it has almost ceased to seem puzzling: how does an organism know what shape to build? A fertilized egg divides, and the resulting cells differentiate, migrate, and organize into an organism of staggering anatomical complexity — with the correct number of limbs in the correct positions, a nervous system of the correct architecture, organs of the correct size, shape, and connectivity. How does this happen?

The standard answer emphasizes genetic programs: genes encode transcription factors, signaling molecules, and their receptors; these activate in precise spatiotemporal patterns that direct cells to adopt appropriate identities and behaviors; and the result is the correct anatomy. This answer is correct as far as it goes, but it is incomplete in a subtle way that becomes visible when we ask about regeneration and adaptation.

Consider: a tadpole embryo has been manipulated to have an entirely scrambled facial anatomy — eyes on the jaw, nostrils displaced, mouth in the wrong position. The genes are as they were before; only the initial positioning has been changed. If development were purely a genetic program executing step by step from initial conditions, this scrambled embryo should produce a scrambled adult. But in experimental work, tadpoles with scrambled facial anatomy often develop, through a process of active cellular relocation and reorganization, into frogs with normal facial anatomy — eyes in the right place, mouth correctly positioned, nostrils correctly sited (Vandenberg et al., 2012).

This result is deeply important. It suggests that the cells are not simply executing a program that says "if you are a muscle cell at position X, express these genes and migrate to position Y." They are instead *pursuing a target* — the normal anatomy — and using whatever cellular behaviors are available to achieve that target, regardless of the initial conditions. The cells exhibit what the cognitive scientist would call *equifinality*: reaching the same endpoint by different routes from different starting points.

## Body Plans as Attractors

Levin's framework explains this equifinality by proposing that body plans are **attractors in a high-dimensional bioelectrical state space**. An attractor, in dynamical systems theory, is a state or region of states that the system tends toward regardless of starting conditions (within a basin of attraction). A ball rolling in a bowl will end up at the bottom regardless of where on the rim it starts. Similarly, a developing organism "rolls" through a high-dimensional state space of cellular bioelectrical configurations toward the attractor state that corresponds to the correct adult anatomy.

The state space in question has many dimensions — at minimum, one for the membrane potential of each cell in the organism, but also dimensions for gap junction connectivity, extracellular ion concentrations, and other bioelectrical variables. The attractor is a region of this space that corresponds to the correct mature anatomy: the pattern of bioelectrical states that the correctly formed organism exhibits.

During normal development, cells start in states consistent with an early embryo and traverse the state space toward the adult attractor, guided by the developmental signaling cascades that standard developmental biology describes. But the key insight is that the dynamics are attractor dynamics: perturbations are corrected not because the program explicitly corrects them, but because any state off the attractor trajectory experiences a restoring force back toward it. The scrambled tadpole's cells are off the trajectory toward the normal attractor, and the developmental dynamics push them back.

This framing converts the shape problem from a question about program execution to a question about attractor landscapes: what is the shape of the bioelectrical state space? What are the attractors? How are they specified, and can they be changed?

### Bioelectricity as the Encoding Medium

Levin's specific hypothesis is that the attractors in this state space are encoded primarily in **bioelectrical patterns**: the spatial distribution of membrane potentials across the developing organism, maintained and communicated through gap junctions and other electrical coupling mechanisms.

Gap junctions are protein channels that directly connect the cytoplasm of neighboring cells, allowing small molecules and electrical current to flow between them. The pattern of gap junction connectivity determines how bioelectrical information flows across the developing tissue, and the resulting spatial pattern of membrane potentials constitutes a kind of "anatomical code" — a representation of where each body part should be, encoded in the bioelectrical language that cells use to coordinate their behavior.

Evidence for this hypothesis comes from multiple experimental systems. In planaria, the spatial pattern of membrane potentials predicts and controls which end will form a head and which will form a tail during regeneration — and manipulating this pattern changes the resulting anatomy (Oviedo et al., 2010). In *Xenopus* tadpoles, specific membrane potential patterns predict the positioning of organs weeks before those organs begin to form (Levin et al., 2002). In *Xenopus* eyes, the bioelectrical state of the developing retina influences its connectivity with the brain (Blackiston & Levin, 2013). In all these cases, the bioelectrical pattern appears to be upstream of gene expression: it specifies the target, and gene expression executes the targeting.

## Regeneration as Goal-Directed Problem-Solving

Regeneration — the ability of organisms to rebuild lost or damaged structures — is one of the most striking examples of what Levin calls **morphogenetic intelligence**. The planarian flatworm exemplifies this capacity most dramatically, but many organisms exhibit significant regenerative abilities: salamanders can regenerate limbs, the human liver can regenerate from as little as 25% of its original mass, and some organisms can regenerate almost entirely from small fragments.

What is cognitively interesting about regeneration is its *flexibility*. When an organism regenerates a lost structure, it must do so in a context that is different from normal development: the starting point is not a fertilized egg but a wound edge in an adult organism, with different tissue organization, different cellular composition, and different bioelectrical state. The regenerating tissue must "assess" the current state and determine what needs to be rebuilt — and then rebuild it correctly using mechanisms that may have evolved primarily for normal development, not for the specific conditions of the wound.

Levin treats this as problem-solving: the organism is solving the problem of "what is the correct final state, given my current state and the target anatomy?" This is a genuinely cognitive problem, in the sense that it requires integrating information about the current state, retrieving or encoding information about the target state, and selecting and executing behaviors that close the gap between them.

The strongest evidence for the genuinely goal-directed character of regeneration comes from experiments that create novel challenges that the organism has never faced in its evolutionary history. If an organism is cut in a way that creates a regenerative challenge it has never encountered before — for example, if a planarian is cut so that regeneration requires forming structures in positions relative to each other that are unusual in normal development — and yet it successfully regenerates the correct anatomy, this is strong evidence for genuine goal-directedness rather than the execution of a pre-programmed routine.

Studies on the regeneration of the correct number of heads and tails in planaria after unusual cuts, and on the recovery of correct organ positioning after surgical rearrangement in tadpoles, provide evidence of this kind. The organisms appear to be solving problems, not executing programs — though the distinction, as always, requires careful theoretical analysis.

## Xenobots and Designed Cognition

In 2020, Levin's laboratory in collaboration with Josh Bongard's computational group at the University of Vermont reported the creation of **Xenobots**: living machines designed by an evolutionary algorithm and constructed from the cells of *Xenopus laevis* (African clawed frog) embryos (Kriegman et al., 2020).

The design process used a computer simulation of thousands of candidate configurations of frog cells — varying their arrangement, the fraction of each cell type (skin cells vs. cardiac muscle cells), and other properties — and applied a simulated evolutionary algorithm to identify configurations that exhibited desired behaviors. The most promising configurations from the simulation were then physically constructed from actual frog cells using microsurgery.

The resulting Xenobots are novel organisms that have never existed in evolutionary history. They are alive — the cells are living frog cells — but their behavior is determined by the configuration discovered by the evolutionary algorithm rather than by the normal developmental program of *Xenopus*. They can move, maintain their shape for days, and — in later experiments — self-replicate by gathering loose cells and organizing them into new Xenobots.

For the cognitive light cone framework, Xenobots are significant because they demonstrate that the cognitive targets of cells can be re-specified by changing the cells' physical arrangement, without changing their genetic makeup. Frog cells in normal development pursue the goal of building a frog. Xenobot cells, arranged differently, pursue the goal of building a Xenobot. The cells' competencies — their ability to self-organize toward a coherent structure — remain intact, but the target of that self-organization has been changed by the physical context.

This is a proof of concept for "designed cognition" at the morphogenetic level: we can, at least in principle, engineer the cognitive targets of living cells to produce desired behaviors. The implications for regenerative medicine are obvious: if we could re-specify the targets that cells pursue in a damaged or diseased tissue, we might be able to induce regeneration of damaged organs or correct developmental defects.

### Ethical Dimensions of Xenobots

The creation of Xenobots raises ethical questions that the cognitive light cone framework helps to frame but cannot answer. If cells exercise genuine cognition, and if Xenobots are genuinely goal-directed systems with cognitive light cones of their own — pursuing the goal of self-maintenance and self-replication — then what ethical status do Xenobots have? Are they the kinds of things we can and should have moral consideration for?

These are not idle questions. As bioengineering capabilities advance, the creation of novel organisms with designed cognitive targets will become more common, and society will need frameworks for thinking about their moral status. The cognitive light cone framework suggests that the relevant variables include: the scale of the system's cognitive light cone (larger light cones may correlate with greater cognitive sophistication), the complexity of the goals being pursued, and the degree to which the system exhibits genuine flexibility and problem-solving in pursuit of those goals.

This is not a complete ethics — the relationship between cognitive sophistication and moral status is itself a contested philosophical question — but it provides starting points for a more principled discussion than simple intuition allows.

## A Critical Assessment

Levin's framework is bold, creative, and empirically productive. It has generated experiments that have revealed surprising facts about bioelectrical control of morphogenesis and about the flexibility of regenerative processes. At the same time, it is worth being clear about what the framework does and does not establish.

**What it establishes**: That biological systems at every level of organization exhibit attractor dynamics, equifinality, and flexibility in achieving target states. That bioelectrical signals play important and previously underappreciated roles in specifying and maintaining these target states. That cancer, regeneration, and development can all be productively analyzed in terms of the cognitive-like properties of biological systems at multiple scales.

**What it does not establish**: That cells are conscious or have subjective experience in any robust sense. That "goal-directedness" in cells is the same kind of thing as goal-directedness in humans. That the cognitive vocabulary applied to cells is anything more than a useful heuristic for predicting and understanding biological dynamics.

**What remains open**: The specific mechanisms by which bioelectrical patterns encode body plan information. The relationship between bioelectrical attractors and the gene regulatory networks that implement developmental programs. The question of whether scale-free cognition will turn out to require new physical principles or can be fully explained within the existing framework of molecular biology and biophysics.

These are not criticisms that undermine the framework's value; they are reminders that the framework is a productive research program in progress, not an established theory. Graduate students entering this field should engage with it as such: with enthusiasm for its empirical productivity and philosophical ambition, and with appropriate caution about its more speculative claims.

---

## References

Blackiston, D.J., & Levin, M. (2013). Ectopic eyes outside the head in *Xenopus* tadpoles provide sensory data for light-mediated learning. *Journal of Experimental Biology*, 216(6), 1031–1040.

Kriegman, S., Blackiston, D., Levin, M., & Bongard, J. (2020). A scalable pipeline for designing reconfigurable organisms. *Proceedings of the National Academy of Sciences*, 117(4), 1853–1859.

Levin, M. (2019). The computational boundary of a "self": Developmental bioelectricity drives multicellularity and scale-free cognition. *Frontiers in Psychology*, 10, 2688.

Levin, M., Thorlin, T., Robinson, K.R., Nogi, T., & Mercola, M. (2002). Asymmetries in H+/K+-ATPase and cell membrane potentials comprise a very early step in left-right patterning. *Cell*, 111(1), 77–89.

Oviedo, N.J., Morokuma, J., Walentek, P., Kema, I.P., Gu, M.B., Ahn, J.M., ... & Levin, M. (2010). Long-range neural and gap junction protein-mediated cues control polarity during planarian regeneration. *Developmental Biology*, 339(1), 188–199.

Vandenberg, L.N., Adams, D.S., & Levin, M. (2012). Normalized shape and location of perturbed craniofacial structures in the *Xenopus laevis* tadpole reveal an innate ability to achieve correct morphology. *Developmental Dynamics*, 241(5), 863–878.
