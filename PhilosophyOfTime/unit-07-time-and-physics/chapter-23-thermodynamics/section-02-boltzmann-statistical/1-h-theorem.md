# Boltzmann's H-Theorem

## The H-Theorem and Its Claim

In 1872, Boltzmann published a paper proving what he called the H-theorem. The theorem involves a quantity H defined by:

*H = ∫ f(v, t) ln f(v, t) d³v*

where *f(v, t)* is the single-particle distribution function — the probability density for a molecule to have velocity *v* at time *t*. H is a measure of the "information" or "organization" in the velocity distribution. The theorem states: given certain assumptions, *dH/dt ≤ 0* — H always decreases over time, or remains constant in equilibrium.

Since the negative of H is closely related to entropy (specifically, *S ≈ -k_B H* in appropriate units), this says that entropy always increases. The H-theorem appeared to prove the Second Law from pure mechanics.

When molecules are in thermal equilibrium — when the velocity distribution is the Maxwell-Boltzmann distribution — H is at its minimum (entropy at maximum) and dH/dt = 0. For any non-equilibrium distribution, H decreases toward this minimum: entropy increases toward its maximum value. The approach to equilibrium is thus a consequence of the H-theorem.

## The Stosszahlansatz: Where Time Asymmetry Enters

The proof of the H-theorem requires a key assumption, which Boltzmann called the *Stosszahlansatz*, or "molecular chaos assumption." It states: the velocities of two molecules that are about to collide are statistically independent — there are no correlations between them prior to the collision.

This assumption seems innocuous. Before molecules collide, they have not yet interacted, so why should their velocities be correlated? And indeed, in many situations this is a reasonable idealization.

But Loschmidt's critique reveals the problem. The Stosszahlansatz introduces a time asymmetry by assuming that correlations are absent *before* collisions but may be present *after* them. In fact, the mechanics of collisions build up correlations: after a collision, the velocities of the two molecules are no longer independent — they are related by the collision dynamics. The Stosszahlansatz says: before collisions, no correlations; after collisions, correlations may build up. This is a time-asymmetric assumption.

If you instead applied the reversed assumption — assuming correlations before collisions but not after — you would derive the reversed H-theorem: entropy always *decreases*. Neither version is forced on us by the time-symmetric mechanics. The choice of the Stosszahlansatz is the place where the arrow of time is inserted into the derivation.

## What This Reveals

Loschmidt's critique reveals that the H-theorem cannot be a purely mechanical result. It is a probabilistic result that holds on the assumption that initial conditions are "generic" in a particular sense — uncorrelated, random-looking — rather than the special, pre-correlated initial conditions that would lead to entropy decrease.

The deeper lesson is that the Second Law is not derivable from mechanics alone. It requires a statistical assumption about the distribution of initial conditions. The assumption Boltzmann made — the Stosszahlansatz — is justified (with some degree of approximation) by an appeal to the following reasoning: if we randomly draw a system from the ensemble of all possible initial conditions compatible with a given macrostate, the molecular velocities will be statistically uncorrelated (for independent particles) with overwhelming probability. The approach to equilibrium and the increase of entropy hold for the vast majority of possible initial conditions.

But "the vast majority" is doing all the work. There are initial conditions — a set of measure zero in the space of all initial conditions, but not literally empty — for which entropy would decrease. These conditions are simply overwhelmingly improbable, not physically impossible.

## Boltzmann's Legacy

The H-theorem, despite its flaws as a universal proof, was a transformative contribution. It established the framework for statistical mechanics, showed how thermodynamic quantities like entropy have a microscopic interpretation, and revealed that the approach to equilibrium is a consequence of the probabilistic structure of the initial state space.

Boltzmann's later work developed the interpretation of entropy as *S = k_B ln W* — a counting of microstates — which is more fundamental than the H-theorem and does not rely on the Stosszahlansatz. This formulation makes explicit that entropy increase is a consequence of measure: there are vastly more high-entropy microstates than low-entropy ones, so any system evolving through the space of microstates under chaotic dynamics will tend toward high entropy simply because high-entropy regions are overwhelmingly larger.

Boltzmann was aware that his statistical approach to the Second Law was philosophically controversial. His contemporaries — including Mach (who did not believe in atoms), Ostwald, and Planck at various stages — resisted the statistical interpretation. The debate over whether the Second Law is absolute or merely probabilistic was central to physics and philosophy of science at the turn of the twentieth century. Boltzmann's statistical interpretation is now universally accepted among physicists, but the philosophical questions it raises — about the source of the time asymmetry, about the nature of probability in physics, about the status of the initial conditions — remain actively debated.

**References**

Boltzmann, Ludwig. 1872. "Weitere Studien über das Wärmegleichgewicht unter Gasmolekülen." *Sitzungsberichte der kaiserlichen Akademie der Wissenschaften in Wien* 66: 275–370.

Brown, Harvey R., Wayne Myrvold, and Jos Uffink. 2009. "Boltzmann's H-Theorem, Its Discontents, and the Birth of Statistical Mechanics." *Studies in History and Philosophy of Modern Physics* 40 (2): 174–191.

Uffink, Jos. 2007. "Compendium of the Foundations of Classical Statistical Physics." In *Philosophy of Physics*, 923–1074. Amsterdam: Elsevier.
