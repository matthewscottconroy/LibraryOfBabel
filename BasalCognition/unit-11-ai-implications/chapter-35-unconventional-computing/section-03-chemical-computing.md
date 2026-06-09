# Section 3: Chemical Computing

## Oscillations in a Dish

In the 1950s, a Soviet chemist named Boris Belousov observed something that, by the prevailing understanding of thermodynamics, should not exist. He was studying a chemical analogue of the Krebs cycle — attempting to demonstrate oxidation of citric acid by bromate in the presence of a cerium catalyst — when he noticed that the solution periodically changed color. Yellow. Colorless. Yellow. Colorless. Oscillating, with metronomic regularity, for minutes at a time.

Belousov was certain that sustained oscillations in a closed chemical system violated the second law of thermodynamics. He was wrong — the system is not at equilibrium, and the second law does not forbid non-equilibrium oscillations — but the referees of the journals to which he submitted his work apparently agreed with his initial self-doubt, and rejected the paper repeatedly. The oscillating reaction was eventually published posthumously (Belousov, 1959 / republished 1985) and extended by Anatol Zhabotinsky in the 1960s, giving the system its modern name: the Belousov-Zhabotinsky (BZ) reaction.

What makes the BZ reaction scientifically extraordinary is not just that it oscillates, but how it oscillates. In a thin layer of solution, the reaction does not oscillate uniformly — it generates beautiful spiral waves: rotating patterns of oxidation and reduction that propagate through the medium at constant speed, annihilate each other upon collision, and spontaneously regenerate from irregularities. The waves look, and in some respects behave, like action potentials propagating through excitable neural tissue. The system is an excitable medium — a chemical system that, like a neuron, can be pushed past a threshold and then recovers, propagating its excitation to neighboring regions.

## The BZ Reaction as a Computing Medium

Andrew Adamatzky and colleagues have systematically explored the computational properties of the BZ reaction. The key insight is that the interaction of chemical waves in an excitable medium implements logical operations. When two waves meet, they annihilate — no signal passes through. When a wave encounters a region that has recently been excited, it is blocked — the medium is refractory, like a neuron after an action potential. When a wave encounters a region with no prior excitation, it propagates freely. These collision rules — pass/block/annihilate — can implement AND, OR, and NOT operations if the spatial geometry of the medium is arranged appropriately (Adamatzky, 2010).

Adamatzky demonstrated that BZ reaction systems can solve maze problems (Steinbock, Tóth, & Showalter, 1995), implement logical operations (Adamatzky, De Lacy Costello, & Asai, 2005), and perform image processing through wave interaction dynamics. The computation is analog and parallel — many wave fronts are propagating and interacting simultaneously — and it is performed by pure chemistry, without any external control other than the initial conditions.

The BZ reaction is not a practical computing platform. It is slow (waves propagate at millimeters per second), difficult to program (the initial conditions must be precisely set up), and the medium degrades over time as reactants are consumed. But it is a profound demonstration that the laws of chemistry, applied to an appropriate non-equilibrium mixture, spontaneously generate information processing. The waves carry information; their interactions transform it; the outcome encodes a computation.

This is not metaphorical computation. The BZ system can literally find the shortest path through a maze: when you fill a maze with BZ solution, wavefronts propagate from the entrance in all directions, but the wavefront traveling the shortest path reaches the exit first. The pattern of wave activity in the solved maze, when recorded photographically, reads out the answer (Steinbock, Tóth, & Showalter, 1995). No algorithm, no data structure, no silicon — just chemistry.

## Turing Patterns and Morphogenetic Computation

In 1952, Alan Turing published a paper that is unusual in his corpus: it is not about computation in the conventional sense, but about chemistry and biology. "The Chemical Basis of Morphogenesis" proposed a mathematical model for how a chemical system could spontaneously generate spatial patterns — the spots on a leopard, the stripes on a fish, the branching of blood vessels — from an initially uniform state (Turing, 1952).

Turing's model involved two chemical species: an activator, which promoted its own production (autocatalysis), and an inhibitor, which repressed the activator. The key was differential diffusion: if the inhibitor diffused faster than the activator, the system would spontaneously develop spatial patterns of activator concentration — regularly spaced spots or stripes — from an initially nearly uniform state. These are now called Turing patterns or reaction-diffusion patterns.

Turing's 1952 paper was theoretical. The experimental demonstration that specific molecular systems implement Turing dynamics in developing organisms has come much later — and is still being worked out. In the skin patterning of fish, the zebrafish Danio rerio has been shown to use a reaction-diffusion system involving two pigment cell types, with the activator-inhibitor dynamics predicted by Turing's model (Kondo & Asai, 1995). In mammalian development, Turing patterns have been implicated in the formation of digits (Sheth et al., 2012), the patterning of hair follicles, and the spatial organization of certain brain circuits.

What does Turing pattern formation have to do with cognition? The connection is indirect but important. Turing patterns are a form of computation: they take an initial condition (a nearly uniform state plus small perturbations) and transform it into a structured output (a regular spatial pattern). The computation is performed by the reaction-diffusion dynamics — by chemistry and diffusion — and its result encodes spatial information that shapes subsequent developmental processes. In this sense, the developing embryo is computing its own structure, using Turing dynamics as the computational substrate.

This is morphogenetic intelligence: the capacity of a biological system to generate complex spatial structure through self-organizing chemical processes, without any external blueprint or instruction set. The pattern is generated by the dynamics of the system itself, in response to its own initial conditions and boundary conditions. It is, in a precise sense, computation without a programmer.

## Chemical Neural Networks

A more speculative but theoretically important direction asks whether chemical systems can implement neural network-like computations — not by being trained, like deep learning models, but by being designed so that their equilibrium chemistry implements the right functional relationships.

The mathematical connection is well-established. The mass-action kinetics of chemical networks are, for certain network topologies, equivalent to the equations of artificial neural networks. A chemical system with the right structure of reaction rates and equilibria can implement arbitrary continuous functions of its input concentrations — and with the right design, it can implement the specific functional relationships of a trained neural network (Hjelmfelt, Weinberger, & Ross, 1991).

In practice, implementing a useful chemical neural network requires designing reaction networks with precise rate constants, which is extraordinarily difficult with current chemical synthesis capabilities. The theoretical result demonstrates, however, that chemistry is computationally universal for the class of functions implementable by continuous neural networks — which is a large and important class.

More practically, there are natural chemical systems that implement function approximation. The allosteric regulation of enzymes — in which the binding of one molecule to a regulatory site changes the enzyme's activity at its catalytic site — implements a nonlinear input-output relationship of exactly the kind that neurons implement. A cascade of allosterically regulated enzymes, with their inputs being substrate concentrations and their outputs being product concentrations, implements a multi-layer function approximator in chemistry. This is, in a structural sense, how metabolic sensing works in living cells — and it may be one of the oldest forms of computation on Earth.

## The Deep Lesson: Computation Is Physical

The BZ reaction, Turing patterns, chemical neural networks, DNA strand displacement, and synthetic gene circuits all point to the same deep lesson: computation is not a special property of silicon, or of neurons, or of any particular material. It is a property of the organization of physical processes. Wherever matter is organized in the right way — with the right feedback relationships, the right spatial structure, the right dynamics — computation happens. The universe computes.

This is not mysticism. It is a straightforward implication of the theory of computation: any physical system whose dynamics can be described by Turing-computable functions is, in principle, a universal computer. The constraint is the organization of the physical process, not the substrate in which it is instantiated.

What biology has demonstrated, over four billion years, is that this organizational insight can be realized in chemistry: that molecular networks, genetic circuits, cytoplasmic dynamics, and tissue-level reaction-diffusion processes can all implement adaptive information processing in ways that are relevant to the organism's survival. The organisms studied in this book are not using cognition despite being made of chemistry. They are using cognition because of how that chemistry is organized.

The recognition that chemical computation is real, ancient, and enormously sophisticated should inform both the science of AI — directing attention toward the principles of organization that make computation effective — and the science of basal cognition — directing attention toward the chemistry underlying the adaptive behaviors we observe.

---

## References

Adamatzky, A. (2010). *Physarum Machines: Computers from Slime Mould*. World Scientific.

Adamatzky, A., De Lacy Costello, B., & Asai, T. (2005). *Reaction-Diffusion Computers*. Elsevier.

Belousov, B. P. (1985). A periodic reaction and its mechanism. In R. J. Field & M. Burger (Eds.), *Oscillations and Traveling Waves in Chemical Systems* (pp. 605–613). Wiley. (Original report circulated 1959.)

Hjelmfelt, A., Weinberger, E. D., & Ross, J. (1991). Chemical implementation of neural networks and Turing machines. *Proceedings of the National Academy of Sciences*, 88(24), 10983–10987.

Kondo, S., & Asai, R. (1995). A reaction-diffusion wave on the skin of the marine angelfish *Pomacanthus*. *Nature*, 376(6543), 765–768.

Sheth, R., Marcon, L., Bastida, M. F., Junco, M., Quintana, L., Dahn, R., ... & Ros, M. A. (2012). Hox genes regulate digit patterning by controlling the wavelength of a Turing-type mechanism. *Science*, 338(6113), 1476–1480.

Steinbock, O., Tóth, Á., & Showalter, K. (1995). Navigating complex labyrinths: Optimal paths from chemical waves. *Science*, 267(5199), 868–871.

Turing, A. M. (1952). The chemical basis of morphogenesis. *Philosophical Transactions of the Royal Society of London, Series B*, 237(641), 37–72.
