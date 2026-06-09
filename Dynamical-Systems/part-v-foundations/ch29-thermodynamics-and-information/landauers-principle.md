# 29.2 Landauer's Principle

The Maxwell demon story begins in 1867. James Clerk Maxwell imagined a tiny intelligent being — later called a demon — controlling a small door in a partition dividing a gas-filled box. The demon watches molecules approaching the door and opens it only to let fast molecules through to one side and slow molecules through to the other. Over time, one side gets hotter and one gets cooler, apparently reducing entropy without doing work. This seems to violate the second law.

For nearly a century, this paradox sat unresolved. Szilard in 1929 realized the demon must *measure* the molecules — and that measurement is information. But measurement alone doesn't obviously cost entropy. The resolution came from Rolf Landauer in 1961, working at IBM, thinking about what computation costs: the bottleneck isn't acquiring information, it's *erasing* it.

**Theorem 29.2.1 (Landauer's Principle, 1961).** Erasing one bit of information in a system at temperature $T$ requires dissipating at least $k_BT\ln 2$ joules of energy as heat.

*(physical argument)* A bit stores 1 bit of entropy, $S = k_B\ln 2$. Erasing the bit (mapping both states to one) reduces the system entropy by $k_B\ln 2$. By the second law, this entropy must be transferred to the environment as heat $Q \geq k_BT\ln 2$.

The argument is elegantly simple once you accept the Boltzmann-Gibbs-Shannon identity. Erasing a bit means taking a two-state system and forcing it to one state — reducing entropy by $k_B \ln 2$. The second law says entropy doesn't spontaneously decrease. So that entropy must go somewhere: into the environment, as heat. The minimum heat generated is exactly $k_BT \ln 2$.

**Maxwell's Demon Resolution:** Maxwell's demon can reduce the entropy of a gas by observing and sorting molecules, seeming to violate the second law. Resolution: the demon must erase its memory (a classical storage device) to operate cyclically. The erasure cost exactly compensates the entropy reduction.

The demon accumulates one bit of information per molecule sorted. After processing many molecules, its memory fills up. To keep operating, it must erase old records. Each bit erased costs $k_BT \ln 2$ of heat dissipated. The entropy gained in the gas is exactly compensated by the entropy generated in erasing the demon's memory. The second law stands.

Landauer's principle was controversial for decades — some argued it was circular, others that it was unnecessary. But Bérut et al. (Nature, 2012) experimentally verified it for the first time, using a colloidal particle in a double-well potential as a one-bit memory. The measured heat dissipation agreed with Landauer's bound to within experimental error.

**Theorem 29.2.2 (Szilard Engine).** A Szilard engine (a one-molecule gas) can extract $k_BT\ln 2$ of work from a single bit of information. The information erasure cost $k_BT\ln 2$ makes the cycle thermodynamically consistent with the second law.

The Szilard engine is the flip side: if you *have* one bit of information (you know which side of a box a single molecule is in), you can extract $k_BT \ln 2$ of work by letting the molecule push a piston. This is work extracted *from information*. Together with Landauer's principle, it gives a complete thermodynamic accounting: information is worth exactly $k_BT \ln 2$ per bit, and erasing it costs exactly the same.

The next section extends this accounting to systems far from equilibrium, where individual trajectories fluctuate wildly and the second law holds only on average.
