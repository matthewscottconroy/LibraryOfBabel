# Section 4: Cell Motility and the Embodied Decision

Cognition that never produces action is a strange kind of cognition. If the preceding sections have argued that cells sense, integrate, and decide, this section is about behavior — how cellular decisions translate into movement through the world. Cell motility is where the information-processing capacities of the cell become embodied in physical action, and it is in motility that we can most clearly observe the cell as an agent navigating its environment.

We focus particularly on chemotaxis — directed movement along chemical gradients — because it is the most thoroughly studied example of cellular navigation and because it exemplifies, with unusual clarity, the relationship between sensing, computation, and action. But we also examine the underlying machinery: the cytoskeleton, the flagellar motor, and the molecular logic of locomotion.

---

## Chemotaxis: Navigation Without a Brain

The bacterium *Escherichia coli* can navigate reliably up a gradient of attractant (such as glucose or amino acids) or away from a gradient of repellent (such as certain organic acids) — a behavior called chemotaxis. Given that *E. coli* is about two micrometers long and chemical gradients in its environment are extremely shallow over such tiny distances, this navigational feat is remarkable. The cell cannot sample the gradient spatially (it is too small to detect a meaningful concentration difference between its two ends). It must therefore sample temporally — compare current conditions to recent past conditions — and translate that temporal comparison into a behavioral change.

Howard Berg and colleagues provided the first detailed characterization of how *E. coli* accomplishes this (Berg & Brown, 1972). The bacterium alternates between two modes of swimming. During "runs," flagella rotate counterclockwise, forming a bundle that propels the cell in a roughly straight line. During "tumbles," one or more flagella briefly switch to clockwise rotation, disrupting the bundle, causing the cell to reorient randomly. The bacterium then begins a new run in the new direction.

In the absence of gradients, runs and tumbles alternate approximately randomly, producing a random walk. But in a gradient of attractant, the probability of tumbling decreases when the cell is moving in a favorable direction (attractant concentration increasing) and increases when moving in an unfavorable direction. The result is that the cell spends more time moving up-gradient than down-gradient — a biased random walk that efficiently navigates toward attractant sources even though individual run directions are random.

---

## The Flagellar Motor: A Nanoscale Rotary Engine

The flagellar motor of *E. coli* is one of the most remarkable molecular machines in biology. It is a rotary engine powered by the proton motive force — the electrochemical gradient of protons across the bacterial membrane — that rotates a helical flagellum at up to 100 revolutions per second. The motor is composed of more than 20 distinct protein species organized into a stator, which anchors to the cell wall and generates torque, and a rotor, which turns the flagellum (Berg, 2003).

Crucially for chemotaxis, the motor is bidirectional. It can rotate counterclockwise (generating runs) or clockwise (generating tumbles), and switches between these states in less than a millisecond. The switching is regulated by a single protein, CheY, which in its phosphorylated form (CheY-P) binds to the flagellar switch complex and increases the probability of clockwise rotation — tumbling. The intracellular concentration of CheY-P is therefore the key signal that controls swimming behavior: high CheY-P means more tumbling; low CheY-P means more running.

The flagellar switch is itself a bistable system with cooperative properties. The FliM ring at the base of the motor contains approximately 34 subunits, each of which can bind CheY-P. The switch has a very sharp, cooperative response to CheY-P concentration — much sharper than would be expected from independent binding of CheY-P to individual subunits (Cluzel et al., 2000). This cooperativity ensures that the switch is either fully counterclockwise or fully clockwise, not stalled in intermediate states, and that the threshold is sharp enough to be responsive to the relatively small changes in CheY-P that the chemotaxis signaling system generates.

---

## The Run-Tumble Algorithm as Gradient Descent

The run-tumble behavioral algorithm of *E. coli* can be understood as a stochastic gradient ascent algorithm — a close relative of the gradient descent algorithms that underlie modern machine learning. The bacterium samples its environment, computes an approximate gradient (rising or falling attractant?), and adjusts its probability of tumbling accordingly. Runs in favorable directions are extended; runs in unfavorable directions are terminated early by tumbling.

Unlike deterministic gradient ascent algorithms, the random reorientation during tumbles allows *E. coli* to escape local maxima in complex environments — it occasionally "tries" new directions even when it is currently moving well. This is stochastic exploration, analogous to the temperature parameter in simulated annealing algorithms. The balance between exploitation (extending favorable runs) and exploration (tumbling to sample new directions) is tuned by the motor bias — the baseline tumble probability — which is adapted by the methylation state of chemoreceptors over longer timescales.

Berg (2004) described the elegant simplicity of this algorithm as a key insight: *E. coli* solves the navigation problem not through spatial comparison (which its small size prevents) but through temporal comparison, using a short-term memory implemented in receptor methylation to ask "is the world getting better or worse?" This is the simplest imaginable form of goal-directed behavior — and yet it navigates toward food sources with a reliability that would be difficult to engineer without this principled algorithmic structure.

---

## The Cytoskeleton as Decision-Making Apparatus

In eukaryotic cells, chemotaxis is more complex and involves a different set of molecular machines. Eukaryotic cells — white blood cells, fibroblasts, cancer cells, amoebae — navigate not by alternating run-tumble but by extending membrane protrusions (pseudopodia) in the direction of the gradient and retracting them elsewhere. This requires the cell to "know" which direction is up-gradient and to polarize its motility machinery accordingly.

The cytoskeleton — particularly actin — is central to this process. Actin polymerizes at the cell's leading edge, pushing the membrane forward to form a pseudopod, and depolymerizes at the trailing edge, allowing the cell body to follow. The spatial regulation of actin polymerization — actin dynamics localized to the up-gradient side of the cell — implements a form of spatial signal integration (Devreotes & Bhatt, 1999; Devreotes & Janetopoulos, 2003).

The key signaling molecules in eukaryotic chemotaxis include the lipid kinase PI3K, which produces the lipid second messenger PIP3 at the leading edge, and the phosphatase PTEN, which degrades PIP3 at the trailing edge. The complementary localization of PI3K and PTEN creates a sharp spatial boundary — a PIP3 gradient across the cell — that is far steeper than the external chemoattractant gradient. The cell amplifies the external gradient into a steep internal gradient, then uses that internal gradient to spatially direct cytoskeletal dynamics.

This amplification and spatial filtering are implemented by a network of mutually inhibiting and locally activating signaling modules that has been described as a reaction-diffusion system: local activation spreads along the membrane but is simultaneously suppressed by global inhibition (Meinhardt, 1999). The result is a self-organizing spatial polarity — the cell establishes a stable front/back axis aligned with the external gradient, even if the gradient itself is too shallow to be directly sensed at the molecular level.

---

## Amoeboid Motion: The Intelligence of the Pseudopod

The social amoeba *Dictyostelium discoideum* has become one of the premier model organisms for studying chemotaxis precisely because it navigates chemical gradients with a sophistication that rivals that of mammalian immune cells. During its life cycle, *Dictyostelium* cells can exist as free-living single cells that hunt bacteria, as chemotactically active cells that aggregate toward cAMP gradients during starvation, or as differentiated cells within a multicellular fruiting body.

The pseudopod extension behavior of *Dictyostelium* reveals the sophistication of single-cell navigation. Pseudopods are not simply extended in the direction of highest cAMP concentration; they are generated stochastically, with probability influenced by but not entirely determined by the chemoattractant gradient (Insall, 2010). The cell explores multiple directions simultaneously through multiple pseudopods, then retracts those that are not reinforced by continuing chemoattractant signal. This exploratory pseudopod dynamics looks less like a deterministic gradient-follower and more like a stochastic decision process with gradient-biased exploration.

Peter Devreotes and his colleagues have used *Dictyostelium* to dissect the molecular underpinnings of this behavior with exceptional precision. Work from the Devreotes lab has established the roles of Ras GTPases, PI3K signaling, and the reaction-diffusion dynamics of TORC2 and PTEN in generating the self-organizing polarization that underlies chemotaxis (Xiong et al., 2010). This work provides one of the clearest available pictures of how molecular-scale signaling dynamics produce cell-scale oriented behavior.

---

## Flagellar Locomotion Beyond Bacteria

Flagellar locomotion is not unique to bacteria. Eukaryotic flagella and cilia — found in sperm, protists, respiratory epithelium, and the embryonic node — use a fundamentally different mechanism: the dynein-driven sliding of axonemal microtubules. Despite the mechanistic difference, eukaryotic flagella also support chemotaxis. Sperm cells navigate chemical gradients of attractants released by eggs; this sperm chemotaxis can be remarkably sensitive, tracking gradients of picomolar concentrations over millimeter distances.

Cilia on the embryonic node — a structure present during early vertebrate development — beat in a coordinated fashion to generate a leftward fluid flow that establishes the left-right body axis. The mechanosensory detection of this flow by cilia on the lateral plate — which opens calcium-regulated ion channels in response to fluid flow — converts a hydrodynamic signal into an intracellular calcium signal that initiates the molecular cascade determining which organs develop on the left versus right side of the body. We examine this bioelectric aspect of left-right determination in Chapter 5.

---

## From Motility to Agency

We return now to the question with which we opened Chapter 4. Is the cell a "problem-solving agent"? Having traced the architecture of sensing, integration, decision-making, and motility, we are better equipped to answer.

The cell senses its environment through a diverse array of receptor proteins that collectively sample multiple dimensions of the chemical, physical, and optical world. It integrates these inputs through signaling networks that implement logical operations, perform temporal and spatial filtering, and produce internal representations of environmental conditions. It makes discrete behavioral commitments through bistable switches that provide irreversibility and robustness. And it translates these internal states into coordinated physical action — motility — that systematically changes its relationship to its environment in directions that tend to improve its circumstances.

This is problem-solving in any reasonable functional sense of the term. It satisfies criteria from cybernetics (negative feedback driving a system toward a set point), from information theory (the cell processes environmental information to generate adaptive responses), and from the kind of functional analysis philosophers of mind use to define mental states. The cell does not have a brain. It does not have consciousness — almost certainly. But it has the physical substrate for a rudimentary form of cognition: a body that senses, computes, decides, and acts.

That is where biological intelligence begins.

---

## References

Berg, H. C. (2003). The rotary motor of bacterial flagella. *Annual Review of Biochemistry*, *72*, 19–54.

Berg, H. C. (2004). *E. coli in Motion*. Springer.

Berg, H. C., & Brown, D. A. (1972). Chemotaxis in *Escherichia coli* analysed by three-dimensional tracking. *Nature*, *239*(5374), 500–504.

Bray, D. (2009). *Wetware: A Computer in Every Living Cell*. Yale University Press.

Cluzel, P., Surette, M., & Leibler, S. (2000). An ultrasensitive bacterial motor revealed by monitoring signaling proteins in single cells. *Science*, *287*(5458), 1652–1655.

Devreotes, P. N., & Bhatt, S. (1999). Signaling in the chemotaxis of *Dictyostelium discoideum*. In B. E. Bhatt (Ed.), *Signal Transduction*. Cold Spring Harbor Press.

Devreotes, P., & Janetopoulos, C. (2003). Eukaryotic chemotaxis: distinctions between directional sensing and polarization. *Journal of Biological Chemistry*, *278*(23), 20445–20448.

Insall, R. H. (2010). Understanding eukaryotic chemotaxis: a pseudopod-centred view. *Nature Reviews Molecular Cell Biology*, *11*(6), 453–458.

Meinhardt, H. (1999). Orientation of chemotactic cells and growth cones: models and mechanisms. *Journal of Cell Science*, *112*(17), 2867–2874.

Xiong, Y., Huang, C. H., Iglesias, P. A., & Devreotes, P. N. (2010). Cells navigate with a local-excitation, global-inhibition-biased excitable network. *Proceedings of the National Academy of Sciences USA*, *107*(40), 17079–17086.
