# Glossary of Computational Systems and Synthetic Biology

*Terms are organised thematically rather than alphabetically, reflecting the conceptual dependencies among them. Each entry includes a definition and a contextualising sentence or example.*

---

## Section 1: Computational Concepts

**Boolean network** — A discrete dynamical model of a gene regulatory network in which each gene (node) takes one of two states (0 = off, 1 = on), and its state at the next time step is determined by a logical function of the states of its regulators. Boolean networks were introduced by Stuart Kauffman (1969) as a model of cell-type determination; their attractor states correspond to stable gene expression patterns.

**Attractor state** — A stable state or set of states toward which a dynamical system converges from many initial conditions. In Boolean networks, attractors correspond to fixed points (single-state cycles) or limit cycles (multi-state cycles); in gene regulatory networks, each attractor has been proposed to correspond to a distinct cell type (Kauffman 1993). The number and structure of attractors determine the differentiation capacity of a network.

**Basin of attraction** — The set of initial states from which a dynamical system's trajectory converges to a particular attractor. Large basins of attraction correspond to robust phenotypes: the system returns to the same stable state after many different perturbations. In a Boolean network for a mammalian cell line, the basin corresponding to the pluripotent state might be smaller than the basins for differentiated cell types.

**Boolean steady state** — An attractor state of a Boolean network that is a fixed point: the network's state does not change under further application of the update rule. Biologically, a Boolean steady state corresponds to a stable differentiated cell phenotype. All genes in the network maintain constant (on or off) expression indefinitely.

**Stochastic simulation algorithm (SSA) / Gillespie algorithm** — An exact simulation algorithm (Gillespie 1977) for systems of chemical reactions, accounting for the inherently discrete, probabilistic nature of molecular interactions. The algorithm treats each reaction as a random event, sampling reaction times from exponential distributions with rates proportional to molecular copy numbers. The SSA is computationally expensive for large systems but is exact in the sense that it samples from the true probability distribution of the master equation. Essential for modelling gene expression noise, stochastic switching between bistable states, and low-copy-number phenomena.

**ODE modelling** — The representation of biochemical reaction networks as systems of ordinary differential equations (ODEs), treating molecular concentrations as continuous deterministic variables. ODEs are the workhorse of systems biology modelling: fast to simulate, analytically tractable for small systems, and interpretable in terms of nullclines and bifurcations. The trade-off is the loss of stochastic information: ODE models are mean-field approximations valid when molecular numbers are large.

**Phase plane** — A graphical method for analysing a two-variable ODE system by plotting trajectories in the space of the two variables. The phase plane shows all possible trajectories simultaneously, revealing the global dynamics: stable equilibria appear as attractors, unstable equilibria as repellors or saddle points. Example: the phase plane of a toggle switch shows two stable fixed points (the two "on" states) separated by a saddle point.

**Nullcline** — A curve in the phase plane on which the rate of change of one variable is zero. For a two-variable system $\dot{x} = f(x,y)$, $\dot{y} = g(x,y)$, the $x$-nullcline is $\{f(x,y)=0\}$ and the $y$-nullcline is $\{g(x,y)=0\}$. Intersections of nullclines are fixed points (equilibria). The geometry of nullclines — how many times they intersect and in what configuration — determines the qualitative behaviour of the system.

**Bifurcation** — A qualitative change in the dynamics of a system as a parameter is varied continuously. Examples include saddle-node bifurcations (two fixed points merge and annihilate, converting a bistable system to a monostable one), Hopf bifurcations (a fixed point loses stability and a limit cycle appears, producing oscillations), and period-doubling bifurcations. Bifurcation analysis is the systematic study of how the qualitative behaviour of a model changes with parameters; it is essential for understanding robustness and for designing switches and oscillators in synthetic biology.

**Limit cycle** — A closed, isolated trajectory in phase space to which nearby trajectories converge. Limit cycles correspond to sustained oscillations; they appear through Hopf bifurcations. The repressilator (Elowitz and Leibler 2000) was designed to produce a limit cycle in gene expression: three mutually repressing genes cycle through a sequence of high-expression states.

**Transfer function** — The input-output relationship of a signalling component, specifying how the output (typically a concentration or activity) depends on the input. Transfer functions are often sigmoidal (switch-like) due to cooperative binding; their characterisation is essential for designing predictable genetic circuits.

**Parameter sensitivity** — The degree to which the output of a model changes in response to changes in parameter values. High sensitivity to a parameter means the system's behaviour is strongly dependent on that parameter's value — a potential fragility. Low sensitivity (robustness) means the behaviour is maintained across a wide range of parameter values. Sensitivity analysis is a standard step in validating models and identifying rate-limiting steps.

---

## Section 2: Molecular Biology Fundamentals

**Transcription factor (TF)** — A protein that binds to specific DNA sequences (promoter or enhancer elements) and either activates or represses the transcription of downstream genes. Transcription factors are the primary effectors of gene regulatory networks. Their activity is controlled by ligand binding, post-translational modifications, and protein-protein interactions. The combinatorial action of multiple TFs at a promoter enables complex Boolean-like logic: a gene can be "on" only when TF1 is present AND TF2 is absent.

**Promoter** — A DNA sequence upstream of a gene where RNA polymerase binds to initiate transcription. Promoter strength (the rate of transcription initiation) determines basal expression level; transcription factor binding sites embedded in the promoter regulate this strength. In synthetic biology, characterised promoters with known strengths and regulatory properties are collected in registries (e.g., iGEM Registry of Standard Biological Parts).

**Operator** — A DNA sequence within or near a promoter that is bound by a repressor protein, blocking RNA polymerase from transcribing the downstream gene. The lac operon's operator (studied by Jacob and Monod, 1961) is the paradigm case; when the lac repressor binds the operator, the lac genes are not expressed.

**Repressor** — A transcription factor that, when bound to its operator, reduces or eliminates transcription of the associated gene. Repressors are the building blocks of many synthetic circuits: toggle switches (Gardner et al. 2000) use two mutually repressing repressors; the repressilator (Elowitz and Leibler 2000) uses three.

**Activator** — A transcription factor that, when bound to its promoter, increases the rate of transcription. Activators can create positive feedback loops (gene A activates itself) or feed-forward loops. The relative prevalence of repressor-based vs. activator-based regulation differs between prokaryotes and eukaryotes.

**Hill coefficient** — A parameter (often written $n$) describing the cooperativity of a binding or regulatory interaction. A Hill coefficient $n = 1$ describes non-cooperative (hyperbolic) binding. For $n > 1$, the response is sigmoidal — switch-like — because multiple ligands must bind cooperatively for the response to occur. In gene regulation, transcription factors often bind cooperatively to multiple sites, giving Hill coefficients of 2–4 and sharp, switch-like transitions in gene expression.

**Cooperativity** — The phenomenon in which binding of one ligand to a macromolecule increases (positive cooperativity) or decreases (negative cooperativity) the affinity for subsequent ligand binding. Cooperativity underlies the steep sigmoidal responses that make biological switches sharp; it arises from conformational changes, protein-protein interactions, or DNA looping.

**Negative feedback loop** — A regulatory circuit in which a gene product inhibits its own production, either directly or through a chain of intermediaries. Negative feedback reduces variability, speeds up response, and promotes homeostasis. Example: the lac repressor system — the repressor is produced from the lac gene cluster and represses its own synthesis.

**Positive feedback loop** — A regulatory circuit in which a gene product promotes its own production. Positive feedback can produce bistability (two stable states), irreversibility (all-or-none switching), and memory-like phenomena. Example: the toggle switch uses two mutually repressing genes, each of which is a positive feedback element for itself via double negation.

**Feedforward loop (FFL)** — A three-node network motif in which a "master" regulator X controls both an intermediate Y and an output Z, and Y also regulates Z. FFLs can function as sign-sensitive accelerators, pulse generators, or persistence detectors. The coherent type 1 FFL (in which X and Y are both activators of Z) delays activation but not deactivation — a filter for transient signals.

**Network motif** — A pattern of connectivity that recurs in biological networks significantly more often than expected by chance. Alon et al. (2002) systematically identified network motifs in the *E. coli* transcription regulatory network; the feedforward loop and autoregulation loop are among the most common. Network motifs are thought to be "information processing units" — small circuits selected for specific computational functions.

---

## Section 3: Systems Biology

**Network topology** — The pattern of connections in a biological network, independent of the quantitative strengths of those connections. Topology determines qualitative properties (which nodes can regulate which), while quantitative parameters determine the dynamics (how strongly each regulation occurs). Understanding topology is prerequisite to understanding dynamics.

**Scale-free network** — A network in which the degree distribution follows a power law: $P(k) \propto k^{-\gamma}$. Scale-free networks have a small number of "hub" nodes with very high connectivity and a large number of nodes with low connectivity. Biological networks (protein-protein interactions, gene regulatory networks, metabolic networks) tend to be approximately scale-free; hubs are often essential genes.

**Small-world network** — A network combining high clustering (most nodes are connected to each other's neighbours) with short average path lengths (any two nodes are connected by a short path). Small-world networks were characterised by Watts and Strogatz (1998) and are relevant to biological networks because they enable efficient information flow with local processing.

**Robustness** — The ability of a biological system to maintain its function across a range of perturbations (parameter variations, mutations, environmental fluctuations). Robustness is a design principle of biological networks: essential functions tend to be maintained even when individual components are perturbed. Robustness is quantified by sensitivity analysis and by the size of the parameter space over which a desired behaviour is maintained.

**Modularity** — The organisation of a biological system into semi-autonomous modules that each perform a specific function and are relatively insulated from each other. Modularity facilitates evolution (modules can be rewired without disrupting others) and is a target of synthetic biology design. Transcription factor networks, metabolic pathways, and signalling cascades all exhibit modularity.

**Degeneracy** — The ability of structurally distinct elements of a biological system to perform the same function; distinct molecular pathways can achieve the same physiological output. Distinguished from redundancy (which refers to identical components performing the same function) by the structural non-identity of the degenerate elements.

**Redundancy** — The presence of multiple identical or near-identical components capable of performing the same function; provides robustness against component failure. Redundancy is common in regulatory networks for essential functions.

**Flux balance analysis (FBA)** — A linear programming method for predicting the steady-state metabolic fluxes in a metabolic network, assuming that the network is in a steady state and that some objective function (typically growth rate) is optimised. FBA uses the stoichiometric matrix to constrain fluxes and identifies the optimal flux distribution. It is the primary method for genome-scale metabolic modelling.

**Metabolic control analysis (MCA)** — A framework for quantifying how the control of a metabolic pathway is distributed among its enzymes. The flux control coefficient of an enzyme measures the fractional change in pathway flux for a fractional change in enzyme activity. MCA reveals that control is distributed across multiple enzymes rather than concentrated in a single "rate-limiting step."

**Stoichiometric matrix** — The matrix $S$ whose entry $S_{ij}$ is the stoichiometric coefficient of metabolite $i$ in reaction $j$ (positive for products, negative for substrates). At steady state, $S \cdot v = 0$, where $v$ is the vector of reaction rates (fluxes). The null space of $S$ defines the feasible steady-state flux distributions and is the basis of FBA.

---

## Section 4: Synthetic Biology

**BioBricks** — Standardised, interchangeable genetic parts (promoters, ribosome binding sites, coding sequences, terminators) with defined flanking restriction sites that allow modular assembly. Developed by Knight (2003) and adopted by the iGEM competition. The BioBrick standard was an early attempt to create a "parts catalogue" for genetic circuit design.

**Genetic circuit** — A network of regulatory interactions among genes, proteins, and other molecules that performs a specific information-processing function. Genetic circuits are analogous to electronic circuits: they can implement logic gates, oscillators, switches, counters, and memory elements. The design of genetic circuits is the central engineering task of synthetic biology.

**Orthogonality** — The property of a synthetic biological component of not interacting with native cellular components (other than through intentional design). Orthogonal genetic circuits are "insulated" from the host's regulatory network, enabling predictable behaviour. Examples include orthogonal ribosomes (that translate only specific mRNAs), orthogonal transcription factors, and orthogonal tRNA/aaRS pairs for non-canonical amino acid incorporation.

**Chassis organism** — The host cell used to implement a synthetic genetic circuit. *E. coli*, *S. cerevisiae*, and *B. subtilis* are common chassis organisms for synthetic biology, chosen for their genetic tractability, fast growth, and well-characterised biology. "Chassis design" refers to the engineering of the host to make it more suitable as a platform (e.g., by removing competing metabolic pathways or reducing protease burden).

**Toggle switch** — A bistable genetic circuit implementing a memory element: the system can be in one of two stable states and can be switched between them by transient inputs. Gardner et al. (2000) constructed a toggle switch in *E. coli* using two mutually repressing transcription factors; each repressor inhibits the other, creating two stable states in which one repressor is high and the other is low.

**Repressilator** — A synthetic genetic oscillator (Elowitz and Leibler 2000) consisting of three genes ($lacI$, $tetR$, $cI$) arranged in a cycle of mutual repression: $lacI$ represses $tetR$, $tetR$ represses $cI$, and $cI$ represses $lacI$. The circuit produces approximately periodic oscillations in gene expression with a period of approximately 150 minutes in *E. coli*. The repressilator was the first demonstration that artificial oscillators could be constructed from genetic components.

**AND gate** — A genetic circuit element that produces an output only when both of two inputs are present. Biologically implemented using a promoter that requires two transcription factors for activation, or by expressing a protein only when two independently regulated components are both present (e.g., two-component split reporter systems).

**NOT gate** — A genetic circuit element that inverts its input: the output is high when the input is low, and vice versa. Implemented biologically using a repressor: a transcription factor that represses expression of the output gene. NOT gates are fundamental to circuit design; cascades of NOT gates create NAND gates, which are functionally complete (any Boolean function can be computed with NAND gates alone).

**Pulse generator** — A genetic circuit that produces a transient output in response to a sustained input. Biologically, an incoherent type 1 feedforward loop (iFFl-1) implements a pulse generator: the input X activates both the output Z (quickly) and a repressor Y (slowly), which then represses Z. The result is a transient pulse of Z expression followed by steady-state repression.

**CRISPR-Cas9** — A genome editing system adapted from bacterial adaptive immunity. The Cas9 endonuclease is directed to a specific DNA sequence by a guide RNA (gRNA); Cas9 creates a double-strand break at the target, which can be repaired by error-prone NHEJ (creating insertions/deletions) or precise HDR (using a template). CRISPR-Cas9 has revolutionised synthetic biology by enabling precise, programmable genome editing in virtually any organism.

**Guide RNA (gRNA)** — The RNA component of the CRISPR-Cas9 system that directs the Cas9 protein to a specific DNA sequence through Watson-Crick base pairing. A 20-nucleotide spacer sequence in the gRNA determines the target sequence. By changing the spacer, researchers can direct Cas9 to any sequence of interest, enabling programmable genome editing.

---

## Section 5: Bioinformatics

**Sequence alignment** — The process of arranging two or more biological sequences (DNA, RNA, or protein) to identify regions of similarity, inferring evolutionary, structural, or functional relationships. Global alignment (Needleman-Wunsch algorithm) aligns entire sequences; local alignment (Smith-Waterman algorithm) identifies locally similar regions. Alignment is scored using substitution matrices (BLOSUM, PAM) that quantify the likelihood of each substitution.

**BLAST** — Basic Local Alignment Search Tool: a heuristic algorithm for rapidly searching sequence databases for sequences similar to a query. BLAST is not exact (it misses some alignments that exact algorithms would find) but is orders of magnitude faster, enabling database-scale searches. Used for homology detection, functional annotation, and as a first step in phylogenetic analysis.

**Read mapping** — The computational process of aligning short sequencing reads (from Illumina or other next-generation sequencing platforms) to a reference genome. Aligners such as BWA, Bowtie2, and STAR use index structures (BWT, hash tables) to map millions of short reads efficiently. The output is a SAM/BAM file recording each read's alignment position and quality.

**Differential expression** — The identification of genes whose expression levels differ between experimental conditions (e.g., treated vs. control, diseased vs. healthy). Differential expression analysis uses statistical models that account for biological variability and the multiple testing problem. DESeq2 and edgeR are the dominant tools for bulk RNA-seq; the negative binomial distribution models count data.

**DESeq2** — A Bioconductor/R package for differential expression analysis from RNA-seq count data, using a negative binomial model with empirical Bayes shrinkage of dispersion estimates. DESeq2 is the most widely used tool for bulk RNA-seq differential expression and is appropriate for small sample sizes (n ≥ 2 per group). Its main output is a ranked list of differentially expressed genes with adjusted p-values (Benjamini-Hochberg).

**Principal component analysis (PCA) in genomics** — A dimensionality reduction technique applied to gene expression data to identify the major axes of variation across samples. In RNA-seq, PCA is used as a quality control step (identifying outlier samples, batch effects) and as an exploratory tool (visualising sample clustering by condition, cell type, or developmental stage). The first few principal components often capture biologically meaningful variation.

**k-means clustering of expression data** — An unsupervised clustering algorithm that partitions gene expression data into $k$ clusters, minimising the within-cluster sum of squared distances. Applied to genes (clustering genes with similar expression patterns across conditions) or to samples (clustering samples with similar global expression profiles). The choice of $k$ is a key methodological decision; silhouette analysis and the elbow method are used to select appropriate values.

**Genome assembly** — The computational process of reconstructing a genome sequence from short sequencing reads. De novo assembly (without a reference) uses overlap-layout-consensus or de Bruijn graph algorithms to assemble reads into contigs (contiguous sequences). Assembly is challenging due to repeat sequences, which create ambiguities in the assembly graph.

**Variant calling** — The identification of single-nucleotide polymorphisms (SNPs) and small insertions/deletions (indels) in sequencing data relative to a reference genome. GATK (Genome Analysis Toolkit) is the standard tool for variant calling in human genomics. Variant calling involves alignment, base quality score recalibration, and joint genotyping across multiple samples.

**Gene Ontology (GO) enrichment** — A statistical analysis identifying Gene Ontology terms (standardised descriptions of gene function, biological process, and cellular component) that are overrepresented in a gene list (e.g., a list of differentially expressed genes) relative to a background set. GO enrichment helps translate statistical results (a list of significant genes) into biological interpretation (a list of affected processes and pathways).
