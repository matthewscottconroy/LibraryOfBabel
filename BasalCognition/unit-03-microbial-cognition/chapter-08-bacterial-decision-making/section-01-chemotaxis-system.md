# Section 1: The Chemotaxis System

The chemotaxis system of *Escherichia coli* is, by some measures, the best-understood signal transduction network in biology. We know the identity, structure, stoichiometry, kinetics, and in-vivo concentrations of essentially every molecular component. We have mathematical models that reproduce the system's behavior with quantitative accuracy. And we understand, at a level of mechanistic depth that is rare in biology, how the molecular components collectively generate the behavioral output — the biased random walk — that constitutes chemotaxis.

This system is the foundation for this chapter's broader discussion of bacterial decision-making. By understanding the chemotaxis system in detail, we gain a template for thinking about bacterial cognition more generally: how molecular machinery implements sensing, integration, memory, and action.

---

## The Run-Tumble Algorithm

The behavioral logic of chemotaxis was described by Berg and Brown (1972) and further analyzed by Berg and colleagues in a series of elegant experiments using the three-dimensional tracking microscope. *E. coli* alternates between two swimming modes:

**Runs**: All flagella rotate counterclockwise (CCW), forming a helical bundle that acts as a single propeller. The cell moves roughly in a straight line at about 20-30 micrometers per second. Run duration is exponentially distributed with a mean of about 1 second under adaptation conditions.

**Tumbles**: One or more flagella transiently switch to clockwise (CW) rotation, disrupting the bundle. The cell body rotates erratically, reorienting randomly. Tumble duration is about 0.1 seconds. After tumbling, the cell resumes running in a direction that is nearly (but not completely) random relative to its pre-tumble direction.

In a uniform environment, runs and tumbles alternate with a bias (motor bias) that produces approximately 1 tumble per second. In an attractant gradient, the cell suppresses tumbling when moving up-gradient (extending runs in the favorable direction) and increases tumbling when moving down-gradient. The result is a biased random walk that efficiently samples space while drifting up the gradient.

The comparison that drives this behavioral bias is temporal, not spatial. The cell compares its current receptor occupancy to its occupancy approximately one second ago (the time window set by the adaptation system's methylation chemistry). If current occupancy is higher than past occupancy (moving up-gradient), tumbling is suppressed. If lower (moving down-gradient), tumbling is increased. The cell is asking: "Is things getting better or worse?" — and adjusting its behavior accordingly.

---

## The Molecular Components

The *E. coli* chemotaxis signaling network consists of approximately seven core proteins, whose interactions have been worked out over decades of biochemical, genetic, and structural studies (Sourjik & Wingreen, 2012).

**The receptors (MCPs)**: *E. coli* has five major methyl-accepting chemoreceptor proteins (Tar, Tsr, Trg, Tap, Aer). Each is a homodimer with an extracellular ligand-binding domain, a transmembrane segment, and an intracellular signaling domain. The different receptors respond to different ligands: Tar detects aspartate and maltose; Tsr detects serine; Aer detects oxygen via a FAD cofactor. Binding of attractant (or loss of repellent) causes a conformational change in the receptor dimer — a small, nanometer-scale piston movement — that propagates to the intracellular domain.

**CheA**: A histidine kinase that is constitutively associated with the receptor complex. In its active state (when the receptor is unoccupied, or occupied by repellent), CheA phosphorylates itself at a specific histidine residue — autophosphorylation. Receptor occupancy by attractant inhibits CheA autophosphorylation.

**CheY**: A small response regulator protein (the "switch signal"). Phosphorylated CheA rapidly transfers its phosphoryl group to CheY, generating CheY-P. CheY-P binds to the flagellar switch protein FliM, increasing the probability of CW (tumbling) rotation. CheY-P is spontaneously hydrolyzed to CheY + Pi within seconds, ensuring that the signal is transient.

**CheW**: An adaptor protein that connects the receptor complex to CheA, coupling receptor conformational changes to CheA activity. CheW is required for efficient receptor-CheA signaling.

**CheB and CheR**: The adaptation enzymes. CheR is a constitutively active methyltransferase that adds methyl groups to specific glutamate residues on the receptor cytoplasmic domain, increasing receptor activity (promoting CheA activation) regardless of ligand occupancy. CheB is a methylesterase that removes methyl groups, decreasing receptor activity. CheB is activated by phosphorylation from CheA, creating a negative feedback loop: when attractant suppresses CheA and thus CheY-P, CheB activity also decreases, allowing CheR-mediated methylation to increase receptor activity back toward baseline.

**CheZ**: A phosphatase that accelerates the dephosphorylation of CheY-P, sharpening the temporal response of the system.

---

## Signal Amplification: The Receptor Cluster

One of the most striking features of the chemotaxis system is its extraordinary sensitivity: *E. coli* can respond to changes in attractant concentration as small as 1-5 nM against a background of micromolar concentrations — a relative sensitivity (delta-C/C) of about 0.1%. This sensitivity is far greater than would be expected from the affinity of individual receptors for their ligands. Where does the amplification come from?

The answer is receptor clustering and cooperativity. Chemoreceptors in *E. coli* are not distributed uniformly around the cell surface; they cluster at the cell poles, forming large assemblies of hundreds of receptor trimers of dimers, organized by the scaffold proteins CheA and CheW into a hexagonal lattice (Sourjik & Bhatt, as reviewed in Sourjik & Wingreen, 2012). Within this cluster, receptor activity is coupled: binding of a single attractant molecule to one receptor in the cluster suppresses the activity not just of that receptor but of many neighboring receptors in the cluster, through allosteric communication within the lattice.

This cooperative activity is quantified by the Hill coefficient — the steepness of the response curve. For a single independent receptor, the Hill coefficient is 1; the system's response increases with the first power of ligand concentration. For the clustered *E. coli* receptors, the effective Hill coefficient is approximately 10 — meaning that the cluster amplifies small changes in occupancy by a factor of about 10 relative to what individual receptors would produce. This cooperative amplification is the source of the system's exquisite sensitivity.

The clustering also provides another function: noise averaging. If the receptors were spatially independent, the stochastic fluctuations in ligand binding to each individual receptor would add independently, producing high noise at the output. In the cluster, the coupling of receptor activities means that the cluster reports on the average occupancy of many receptors simultaneously — a spatial averaging that dramatically reduces the noise in the signal sent to CheA.

---

## Methylation Memory: The Temporal Integration Mechanism

The adaptation system implemented by CheR and CheB is the mechanism by which *E. coli* achieves temporal comparison — the ability to compare current receptor occupancy to past occupancy and thus detect changes rather than absolute concentrations.

The logic works as follows. In a constant attractant environment:
- Attractant binds receptors, suppressing CheA activity and thus CheY-P.
- Reduced CheA activity reduces CheB activation.
- CheR continues to methylate receptors (CheR is constitutively active).
- Methylation increases receptor activity (and CheA activity), counteracting the effect of attractant binding.
- Eventually, methylation rises until CheA activity — and thus CheY-P and tumbling frequency — returns to baseline.

The result is adaptation: the cell stops responding to a constant attractant and returns to its baseline tumbling frequency. The methylation level has "set" to a value that compensates for the current attractant concentration. This new methylation level is the "memory" of recent conditions.

Now if the attractant concentration increases:
- Receptor occupancy increases, suppressing CheA even more transiently.
- CheY-P drops, extending runs.
- CheB activity falls further; CheR methylation raises the methylation level.
- Eventually, new adaptation occurs at higher methylation, restoring CheY-P to baseline.

During the brief period of adaptation — typically seconds to tens of seconds — the cell is running extended runs in the up-gradient direction. The timescale of adaptation is the timescale of temporal comparison: the cell is comparing "now" to "1-10 seconds ago."

This is an elegant implementation of derivative detection — the system responds to the time derivative of attractant concentration, not its absolute value. The mathematical analysis shows that the CheB/CheR adaptation system implements integral feedback control: it integrates the error (deviation of tumbling frequency from baseline) over time and uses this integral to drive adaptation (Yi et al., 2000). This integral control property ensures that adaptation is perfect — the tumbling frequency always returns exactly to baseline regardless of the attractant concentration — and robust to parameter variation. It is a beautiful example of how molecular biology implements engineering principles.

---

## Comparison with Gradient Algorithms

The run-tumble algorithm can be compared to mathematical gradient ascent algorithms, which are well-studied in optimization theory. Standard gradient ascent moves deterministically in the direction of steepest gradient increase. The run-tumble algorithm, by contrast, is stochastic: it does not move deterministically in the steepest direction but biases a random walk uphill.

This might seem inferior, but it has important advantages in realistic environments. Deterministic gradient ascent is vulnerable to local maxima — it will get stuck at the first local peak it encounters. The stochastic exploration of run-tumble allows occasional "wrong-way" runs that explore alternative directions, providing a mechanism for escaping local maxima. The effective "temperature" of the search — the extent to which stochastic exploration competes with deterministic uphill drift — is set by the tumble frequency and run length, which are themselves regulated by adaptation.

Berg (2004) made an astute comparison: the run-tumble algorithm is computationally similar to simulated annealing — a global optimization strategy that uses stochastic exploration with decreasing randomness to escape local minima. Whether evolution "discovered" simulated annealing independently, or whether this is a post-hoc analogy, is hard to say. But the functional convergence is striking.

---

## References

Berg, H. C. (2004). *E. coli in Motion*. Springer.

Berg, H. C., & Brown, D. A. (1972). Chemotaxis in *Escherichia coli* analysed by three-dimensional tracking. *Nature*, *239*(5374), 500–504.

Sourjik, V., & Wingreen, N. S. (2012). Responding to chemical gradients: bacterial chemotaxis. *Current Opinion in Cell Biology*, *24*(2), 262–268.

Yi, T. M., Huang, Y., Simon, M. I., & Doyle, J. (2000). Robust perfect adaptation in bacterial chemotaxis through integral feedback control. *Proceedings of the National Academy of Sciences USA*, *97*(9), 4649–4653.
