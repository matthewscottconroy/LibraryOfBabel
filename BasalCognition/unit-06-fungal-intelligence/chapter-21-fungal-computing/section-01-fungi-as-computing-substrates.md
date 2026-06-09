# Section 1: Fungi as Computing Substrates

## What Does It Mean to Compute?

Before we can ask whether fungi compute, we need to be clear about what computing means. This is not a trivial question: the word "computation" is used in at least three distinct senses in the scientific literature, and conflating them is a reliable source of confusion.

The most rigorous sense is formal computation in the tradition of Turing: a process that implements a well-defined algorithm, manipulating symbols according to precise rules to produce outputs from inputs. Turing machines, digital computers, and the neurons of the cerebral cortex (if certain theoretical frameworks are correct) all compute in this sense.

A weaker but still meaningful sense is physical computation: a physical process that reliably produces outputs that covary with inputs in a way that is useful for solving some class of problems. This is the sense in which we might say that a chemical equilibrium "computes" the equilibrium concentration from the initial concentrations and rate constants. No symbolic manipulation is involved, but the physical process reliably solves a problem.

The weakest sense — and the one most prone to misuse — is analogical computation: a physical process that resembles computation in some structural way, without necessarily performing any well-defined information processing. When we say that evolution "computes" fitness, we are using computation in this analogical sense, and it is important to recognize that this tells us very little about the underlying mechanisms.

When Adamatzky and colleagues ask whether fungi can compute, they are primarily asking whether fungal networks can implement physical computation: whether the growth patterns, electrical signals, and cytoplasmic flows of the mycelium can be engineered (or trained) to reliably produce outputs that solve computational problems. This is a more tractable empirical question than the question of formal computation, and it is the question we will examine here.

## Logical Operations in Mycelial Networks

The most direct approach to fungal computing is to ask whether a growing mycelium can implement logical gates — the basic operations (AND, OR, NOT) from which all digital computation can be constructed. Adamatzky and colleagues have attempted this by setting up experimental geometries in which the growth of hyphae through specific paths corresponds to the assertion of logical inputs, and the presence or absence of growth at a specific output location corresponds to the value of a logical output.

The concept draws on earlier work with Physarum: Tsuda and colleagues demonstrated that slime mold growth through a carefully designed spatial arrangement of chemical attractants could implement AND and OR gate functions (Tsuda et al., 2004). The slime mold grows toward attractants, and by positioning attractants at specific locations, researchers can engineer the network's growth to implement simple logical operations.

Adamatzky's group attempted similar experiments with fungal mycelium, using the growth of hyphae through physically designed substrates as the computing medium. In these experiments, blocks or channels of nutritive substrate are arranged so that fungal growth through specific routes corresponds to specific logical operations. The approach is ingenious but limited: the resulting "gates" are slow (fungal growth takes hours to days), non-reconfigurable (once the mycelium has grown through a channel, it cannot easily be reset), and sensitive to biological variability.

The published results suggest that some logical operations can be implemented in this way — that the growth patterns of fungi through constrained spatial architectures do show input-output relationships consistent with simple logical functions (Adamatzky, 2022). But the experimental demonstrations are proof-of-concept rather than practical implementations. The gates are not fast enough, reliable enough, or flexible enough to form the basis of useful computing devices with current approaches.

## Memory-Like Properties

A more interesting question, from a cognitive perspective, is whether mycelial networks retain information about past encounters with their environment — whether they exhibit something that functions like memory.

The network architecture itself constitutes a form of memory, as we discussed in Chapter 20: the pattern of reinforced cords and pruned hyphae is a record of the network's history of encounters with food sources and environmental challenges. This is memory in the dispositional sense — not memory as a stored representation, but memory as a change in the system's future behavior caused by past experience.

Can this dispositional memory be demonstrated experimentally? Experiments on Physarum have shown that it retains information about previous environmental conditions even after those conditions have been removed. In a striking study, Physarum grown under rhythmic, periodic desiccation events continued to anticipate those events — slowing its growth rate in preparation for desiccation — even when the periodic stress was discontinued, for at least several cycles (Saigusa et al., 2008). This is behavioral habituation, one of the simplest forms of learning, implemented in an organism with no neurons.

Whether true fungal mycelium shows analogous memory-like properties has been less extensively studied, but the mechanisms that would support such memory — cytoplasmic flow patterns that persist after their triggering conditions have changed, network architectures shaped by past encounters that bias future growth — are plausible and consistent with what is known about mycelial biology. Demonstrating these effects cleanly is technically challenging; it requires distinguishing the effects of past experience from the ongoing effects of current conditions, which is harder than it sounds in a living, growing organism.

## Maze-Solving: Evidence and Interpretation

The most visually compelling demonstration of problem-solving in fungal-related biology is the Physarum maze experiment — not, strictly speaking, a fungal experiment, but one that has directly inspired work on true fungi. In Nakagaki et al.'s original experiment (Nakagaki et al., 2000), Physarum placed in a maze with food at the start and end retracted from dead ends and concentrated its cytoplasm in the path connecting the two food sources — which happened to be the shortest path. The network had, in some sense, solved the shortest-path problem.

This result has been replicated and extended, but it is important to understand what it does and does not demonstrate. The Physarum does not "think about" the maze in any conventional sense. It does not represent the maze, explore alternatives mentally, and select the best option. Instead, it physically explores all possible paths simultaneously, and the positive feedback process of cytoplasmic reinforcement naturally amplifies the shortest path, because the shortest path has the lowest hydraulic resistance and therefore carries the most flow.

This is physical computation in the purest sense: the maze's geometry is directly encoded in the physical substrate, and the computing is done by cytoplasmic flow physics operating on that geometry. There is no representation, no symbol manipulation. The "solution" is produced by the same physical laws that govern fluid flow in networks.

Analogous experiments with true fungal mycelium have been performed. *Phanerochaete velutina* and other cord-forming species navigate around obstacles, connect food sources with efficient paths, and reroute around severed connections (Boddy, 1999). The mechanisms are similar to those in Physarum: exploration followed by reinforcement of productive routes. Whether this constitutes "maze-solving" in any meaningful computational sense depends on how much you want the word "solving" to require representation and deliberation, versus how much you are satisfied with reliable production of adaptive outcomes.

## Prospects and Limits of Fungal Computing

The honest assessment of fungal computing as a practical technology is that, despite interesting proof-of-concept demonstrations, the obstacles are substantial.

**Speed**: Fungal computation based on growth operates at timescales of hours to days. Even fungal electrical signals, propagating at millimeters per minute, are orders of magnitude slower than the picosecond timescales of modern semiconductor computation. For most computing applications, this is a crippling limitation.

**Reliability**: Biological systems are inherently variable. Individual fungal hyphae do not behave identically to one another, and the same colony may produce different growth patterns in response to the same inputs on different occasions. The noise tolerance of a biological computing substrate is very different from that of a deterministic digital circuit.

**Reconfigurability**: A digital computer can be reset and reprogrammed. A mycelium that has grown through a particular spatial configuration has physically changed. Resetting it requires autolysis and regrowth, which takes time.

**Interface**: Encoding inputs into a fungal computing system and reading outputs from it remain technically challenging. Chemical inputs can be provided, and electrical outputs can be recorded, but the bandwidth and precision of this interface are currently very limited.

These limitations do not mean fungal computing is uninteresting. They mean that fungal computing is interesting for different reasons than those that animate conventional computer science. The fungus is not going to replace the silicon chip. But it might offer something that silicon cannot: a substrate that integrates sensing, computation, and adaptive response in a single physical system, that operates at room temperature without electrical power supply, that repairs itself, and that can compute directly in chemical and biological domains where silicon requires complex transduction.

These properties — which we will explore further in Section 3 — are why some researchers remain excited about biological computing despite the obvious practical limitations. The fungus is not a better version of a computer. It is a different kind of thing, and it might be useful in ways that are orthogonal to the things that make computers useful.

---

## References

Adamatzky, A. (2022). Language of fungi derived from their electrical spiking activity. *Royal Society Open Science*, 9(4), 211926.

Boddy, L. (1999). Saprotrophic cord-forming fungi: Meeting the challenge of heterogeneous environments. *Mycologia*, 91(1), 13–32.

Nakagaki, T., Yamada, H., & Tóth, Á. (2000). Maze-solving by an amoeboid organism. *Nature*, 407(6803), 470.

Saigusa, T., Tero, A., Nakagaki, T., & Kuramoto, Y. (2008). Amoebae anticipate periodic events. *Physical Review Letters*, 100(1), 018101.

Tsuda, S., Aono, M., & Gunji, Y. P. (2004). Robust and emergent *Physarum* logical-computing. *Biosystems*, 73(1), 45–55.
