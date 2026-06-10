# Entropy

Consider a gas compressed into one corner of a box. The gas expands spontaneously until it fills the box uniformly. The process never runs the other way: we never observe a uniform gas spontaneously retreating to one corner. The direction of this process defines an arrow — the tendency toward uniformity, disorder, the spreading out of energy. Clausius gave this arrow a name in 1865: entropy. The second law of thermodynamics states that entropy never decreases in an isolated system. But what is entropy, really? Why does it increase? And what does it have to do with the microscopic dynamics of molecules?

These questions sit at the intersection of thermodynamics, statistical mechanics, information theory, and cosmology, and debates about their answers touch on fundamental issues about the direction of time, the nature of probability, and whether thermodynamics reduces to mechanics.

## Thermodynamic Entropy

Clausius defined entropy as a state function S of a thermodynamic system such that:

dS = δQ_rev / T

where δQ_rev is the reversible heat transferred to the system and T is the absolute temperature. For a reversible process, the entropy change of the universe (system plus surroundings) is zero; for any irreversible process, the entropy of the universe increases.

Entropy in this thermodynamic sense measures the *unavailability of energy for doing work*. A high-entropy state has energy distributed uniformly; a low-entropy state has energy concentrated, in a hot body surrounded by cold. Low-entropy states are ordered in the sense of having available energy gradients; high-entropy states are disordered in the sense of uniform energy distribution.

## Boltzmann's Statistical Interpretation

Ludwig Boltzmann, in the 1870s-1890s, provided a statistical mechanical interpretation. The key equation (inscribed on Boltzmann's grave in Vienna) is:

S = k_B log W

where k_B is Boltzmann's constant (1.38 × 10⁻²³ J/K) and W is the number of microstates (specific configurations of the microscopic constituents) compatible with the macrostate.

A macrostate is a description of a system's macroscopic properties — temperature, pressure, volume, total energy. Many different microstates (specific arrangements of molecules) correspond to the same macrostate. W is the statistical weight of the macrostate. High-entropy macrostates have very large W — many different microscopic configurations realize them. Low-entropy macrostates have small W — few microstates realize them. A gas compressed into the corner of a box is a low-W macrostate: only configurations with all molecules in that corner qualify. A uniformly distributed gas is a high-W macrostate: an enormous number of configurations have this property.

The second law, on the statistical interpretation, holds with overwhelming probability rather than with necessity. Systems evolve toward higher-entropy macrostates because there are vastly more high-entropy microstates than low-entropy ones. The probability of spontaneous fluctuation to a significantly lower-entropy macrostate is not zero but is astronomically small for macroscopic systems.

## Phase Space and Shannon Entropy

In the phase space formulation (Gibbs, Boltzmann), a microstate of a system of N classical particles is a point in a 6N-dimensional phase space. The thermodynamic macrostate corresponds to a region in phase space. Entropy can be formalized as the logarithm of the volume of this region (measured in appropriate units). The approach to equilibrium corresponds to the evolution of the phase space volume toward the largest available region.

Liouville's theorem shows that phase space volume is conserved under Hamiltonian dynamics. But the *shape* of the region becomes more complex (like a stirred mixture of dyes), spreading throughout the accessible phase space while maintaining constant volume. The *coarse-grained* entropy — measured at scales accessible to macroscopic observation — increases even though the fine-grained entropy is conserved. This reconciles time-symmetric Hamiltonian dynamics with the apparent time-asymmetry of entropy increase.

Claude Shannon's 1948 information theory introduced an entropy measure for probability distributions:

H = -Σᵢ pᵢ log pᵢ

where {pᵢ} is a probability distribution. Shannon entropy measures the uncertainty of the distribution: it is maximized when all outcomes are equally probable and minimized when one outcome has probability 1. The formal similarity between Shannon entropy and Boltzmann/Gibbs entropy is not coincidental: both measure the "spread" of a distribution. Jaynes's maximum entropy principle interprets statistical mechanics as applied probability theory: given macroscopic constraints, the equilibrium distribution over microstates is that which maximizes Shannon entropy subject to those constraints. This generates the Boltzmann distribution and the entire formalism of statistical mechanics from information-theoretic reasoning. Landauer's principle, confirmed experimentally, connects information to thermodynamics: erasing one bit of information necessarily dissipates at least k_B T ln 2 of energy as heat.

## Philosophical Issues

Is thermodynamic entropy an objective physical property of a system, or is it relative to a description or coarse-graining? Boltzmann's statistical entropy is defined relative to a macrostate — which requires specifying a coarse-graining of phase space. Different coarse-grainings yield different entropy values, suggesting that entropy is not a purely objective, mind-independent quantity but depends on the description we choose. Opponents argue that the physical coarse-graining is fixed by the practical capacities of observers embedded in the physical world: the relevant macrostates are those that can be manipulated and measured by macroscopic agents, making the coarse-graining physically determined even if not purely observer-independent.

The probabilities appearing in statistical mechanics are also contested. Are they objective chances (propensities of physical systems)? Frequencies in a long-run sequence of systems? Degrees of belief of an agent with incomplete knowledge? Boltzmann's original interpretation was frequentist; Jaynes's interpretation is Bayesian — probabilities represent an agent's uncertainty, and statistical mechanics is the application of Bayesian reasoning to many-body systems. The ontic versus epistemic character of thermodynamic probability connects to the philosophy of probability more broadly, and to the question — taken up in the next file — of how thermodynamics relates to the underlying statistical mechanics.
