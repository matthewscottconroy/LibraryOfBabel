# Section 1: Wetware — Computing with Biology

## Adleman's Test Tube

The Hamiltonian path problem is a classic example of a combinatorially hard problem: given a directed graph, find a path that visits each node exactly once. For large graphs, the only known general algorithms require time exponential in the number of nodes — a characteristic shared with the traveling salesman problem and many other problems in the complexity class NP. For a graph with even a few dozen nodes, no classical computer could exhaustively check all possible paths in any reasonable time.

Leonard Adleman recognized that single-stranded DNA molecules naturally explore all possible paths simultaneously. Each city in his experiment was encoded as a specific 20-nucleotide sequence. Each directed edge was encoded as an oligonucleotide complementary to the last ten nucleotides of the origin city and the first ten nucleotides of the destination city. When these molecules were mixed in solution, Watson-Crick base pairing caused edge molecules to spontaneously hybridize to city molecules in all possible combinations — effectively generating all possible paths through the graph in parallel, simultaneously, in a single tube.

The next step was selection: identifying from this combinatorial soup only those DNA molecules that encoded valid Hamiltonian paths. This required a series of biochemical filtering steps: removing molecules of the wrong length (paths that visit too many or too few cities), amplifying molecules containing each city at least once, and sequencing the survivors. Adleman (1994) demonstrated that this process correctly identified all Hamiltonian paths through a seven-node graph — in a few days, using bench chemistry, with no silicon involved.

The parallelism was the point. A silicon computer exploring all possible paths sequentially would require 7! = 5040 operations for seven nodes; for twenty nodes, it would require 20! ≈ 2.4 × 10¹⁸ operations, requiring millions of years on the fastest available hardware. Adleman's DNA computer explored all possibilities simultaneously, with the number of parallel operations equal to the number of DNA molecules in the tube — roughly 10¹⁴, far exceeding any electronic parallel computer.

The practical limitations were also real. Biochemical filtering is slow and error-prone. Reading out the result required DNA sequencing. Programming the system for a new problem required synthesizing new oligonucleotides. Scaling to larger graphs required exponentially more DNA, which quickly became physically impractical. In the years since Adleman's paper, DNA computing has demonstrated solutions to other combinatorial problems — Boolean satisfiability, cryptographic operations, medical diagnostics — but has not displaced silicon for any general-purpose application (Winfree, 1998; Qian & Winfree, 2011).

What DNA computing has demonstrated, definitively, is that molecular systems can perform information processing that is not merely metaphorical. DNA hybridization is not "like" computation — it *is* computation, using physical law (thermodynamics, hydrogen bonding) as the computational medium. The molecules do not follow an algorithm written by a programmer; they obey their own chemistry, and that chemistry implements the algorithm.

## DNA Nanotechnology and Strand Displacement

Beyond Adleman's approach, a more engineerable form of DNA computing has emerged from the work of Erik Winfree, Lulu Qian, and collaborators: strand displacement cascades (Qian & Winfree, 2011). In strand displacement computing, single-stranded DNA "input" molecules react with double-stranded "gate" complexes, displacing one strand and releasing another. This release can serve as the input to another gate. By designing the sequences appropriately, one can implement NOT gates, AND gates, OR gates, threshold functions, and even amplifiers — all using only DNA hybridization, with no proteins or enzymes required.

Qian and Winfree (2011) demonstrated in principle that strand displacement circuits can implement arbitrary digital logic, and built a simple demonstration system implementing a square root computation using purely DNA-based reactions. More recently, the same group has demonstrated neural network-like computations in DNA, including classifiers that can distinguish between different molecular inputs based on a trained weight matrix implemented in DNA stoichiometry (Cherry & Qian, 2018).

This line of work is significant because it suggests that molecular information processing is not limited to simple logic gates. The same combinatorial chemistry that implements the information processing of living cells can, in principle, be engineered to implement complex learned functions — classifiers, pattern recognizers, even neural network-like architectures — using molecules as the computational substrate. The computational elements can be introduced into cells, react with cellular molecules, and implement logical or arithmetic operations on intracellular signals. This is the basis of molecular diagnostics — DNA circuits that detect specific sequences of pathogen nucleic acids and trigger amplified output signals — and it is beginning to merge with synthetic biology's project of programming cellular behavior.

## RNA Computing

RNA, the chemical cousin of DNA, offers additional computational possibilities. Unlike DNA, which is primarily a static information store, RNA is dynamically transcribed, processed, and degraded in living cells — making RNA-based circuits suitable for real-time, intracellular computation that responds to cellular state.

Ribozymes — RNA molecules with enzymatic activity — can implement self-cleaving and self-ligating reactions that function as molecular logic gates. Riboswitches — RNA structures in the 5' untranslated regions of messenger RNAs — fold into conformations that activate or repress translation in response to specific small-molecule ligands. These natural regulatory elements have been extensively characterized and engineered to respond to novel inputs (Breaker, 2002).

By designing riboswitches that respond to specific cellular metabolites, synthetic biologists can build RNA-based sensors that link cellular state to gene expression in programmable ways. Combining multiple riboswitches with different specificities — and designing mRNA architectures in which multiple switches must all be activated for translation to occur — allows the implementation of multi-input logic functions in RNA. This is intracellular computing using the cell's own transcriptional and translational machinery as the computational infrastructure.

## Brain Organoids: DishBrain and Beyond

In 2022, Brett Kagan and colleagues published a striking paper in *Neuron*: they had grown human cortical neurons in vitro, interfaced them with electrodes, and taught them to play a simplified version of Pong (Kagan et al., 2022). The system — which the authors named DishBrain — consisted of approximately 800,000 neurons growing on a multi-electrode array. The electrodes delivered stimuli representing the ball's position and paddle's position, and they read out neural activity as a control signal for the paddle.

The key finding was that the neurons, after exposure to feedback stimulation, modified their activity patterns in ways that improved paddle performance. They played the game. Not perfectly — they hit the ball more often than chance and improved over sessions — but demonstrably. The authors interpreted this as a form of biological learning under a free energy minimization framework (Friston, 2010), arguing that the neurons organized their activity to reduce the unpredictability of their incoming stimulation — and that playing the game successfully was, from the neurons' perspective, a way to achieve that predictability.

DishBrain raises immediate questions. Are the neurons performing anything that could be called cognition? Or are they simply exhibiting the kind of activity-dependent synaptic modification that any neurons would show when subjected to patterned stimulation? The authors are careful not to overclaim, but they are also willing to frame the result in explicitly cognitive terms — as learning, as adaptation — and to raise questions about the moral status of systems that might, in some sense, experience their computational context.

These questions will return in Chapter 37. For the present, DishBrain matters as a proof of concept: neural tissue can be grown, interfaced with technology, and trained to perform computational tasks in ways that are distinct from either pure biology or conventional AI. It is a step toward neuronal computing — using biological neurons, with all their molecular complexity and adaptive capacity, as the computational substrate for hybrid biological-digital systems.

The broader program of which DishBrain is a part — organoid computing — involves growing three-dimensional miniature versions of brain regions from induced pluripotent stem cells, interfacing them with electronic systems, and using them for computation or as models for drug testing and disease modeling (Lancaster & Knoblich, 2014). The computational applications remain in early stages, but the biological fidelity is improving rapidly: current brain organoids can generate complex electrical activity, form functional synapses, and — in some cases — develop the rudiments of functional circuitry reminiscent of specific brain regions.

The ethical dimension is not separable from the scientific one. As organoids become more neurologically sophisticated — as they begin to generate activity patterns that more closely resemble those of intact neural tissue — questions about their potential for experience become more pressing, not less. We will not resolve those questions here, but they are questions that researchers in this field must take seriously, and that this book's philosophical chapters will engage directly.

## The Common Thread: Matter as Process

What links Adleman's DNA computer, strand displacement circuits, RNA riboswitches, and neuronal computing is a single underlying principle: that matter — organized according to chemical and biological principles — can implement information processing. There is no sharp boundary between the substrate and the process, between the hardware and the software. The chemistry is the computation.

This is, in a deep sense, what life has always been doing. Every cell is a molecular computer: it reads signals from the environment, processes them through biochemical networks, stores information in molecular modifications, and generates adaptive behavioral outputs. The difference between a cell and a DNA computer is not architectural — both use molecular interactions as the computational medium. The difference is that the cell has been optimized by four billion years of natural selection for the specific computational problems posed by survival and reproduction, while DNA computers are being optimized by years of human engineering for specific combinatorial problems.

The lesson for AI is not that we should build neural computers instead of silicon ones, though that is one direction the technology may take. The lesson is deeper: intelligence does not require a specific substrate. It requires appropriate organization of matter. Understanding how living systems organize matter for computation — what principles guide that organization, what the physical limits are, what forms of adaptive behavior are achievable — is one of the most important frontiers in science.

---

## References

Adleman, L. M. (1994). Molecular computation of solutions to combinatorial problems. *Science*, 266(5187), 1021–1024.

Breaker, R. R. (2002). Engineered allosteric ribozymes as biosensor components. *Current Opinion in Biotechnology*, 13(1), 31–39.

Cherry, K. M., & Qian, L. (2018). Scaling up molecular pattern recognition with DNA-based winner-take-all neural networks. *Nature*, 559(7714), 370–376.

Friston, K. (2010). The free-energy principle: A unified brain theory? *Nature Reviews Neuroscience*, 11(2), 127–138.

Kagan, B. J., Kitchen, A. C., Tran, N. T., Habibollahi, F., Khajehnejad, M., Parker, B. J., ... & Razi, A. (2022). In vitro neurons learn and exhibit sentience when embodied in a simulated game-world. *Neuron*, 110(23), 3952–3969.e8.

Lancaster, M. A., & Knoblich, J. A. (2014). Organogenesis in a dish: Modeling development and disease using organoid technologies. *Science*, 345(6194), 1247125.

Qian, L., & Winfree, E. (2011). Scaling up digital circuit computation with DNA strand displacement cascades. *Science*, 332(6034), 1196–1201.

Winfree, E. (1998). On the computational power of DNA annealing and ligation. In L. F. Landweber & E. B. Shapiro (Eds.), *DNA Based Computers* (pp. 199–221). DIMACS Series in Discrete Mathematics and Theoretical Computer Science.
