# Causal Dynamical Triangulation: Causality Built In

## Background: Path Integrals and Quantum Gravity

In quantum mechanics, the probability amplitude for a particle to travel from one point to another is computed as a sum over all possible paths between the two points (the Feynman path integral). Each path contributes with a phase given by the classical action. The resulting quantum theory correctly reproduces the quantum mechanical predictions for particle motion.

Applying this idea to quantum gravity leads to a sum over geometries: the amplitude for spacetime to evolve from one geometry to another is computed as a sum over all possible spacetime geometries interpolating between them, each weighted by the Einstein-Hilbert action. This is the "gravitational path integral" or "quantum gravity partition function."

The challenge: which geometries should be summed over, and how should the sum be regularized? In ordinary path integrals for particle mechanics, one can discretize the path into a sequence of line segments (a "piecewise linear" approximation) and take the continuum limit. The analogous approach for quantum gravity is to approximate spacetime by a piecewise linear (triangulated) manifold — a "triangulation" — and sum over triangulations.

## Euclidean Dynamical Triangulation and Its Problems

Early work on "Euclidean dynamical triangulation" (EDT) summed over triangulations with Euclidean (rather than Lorentzian) signature — replacing the Minkowski metric with a Euclidean metric to make the path integral well-defined. The hope was that a Wick rotation (analytically continuing back to Lorentzian signature at the end) would recover realistic spacetime physics.

The EDT approach faced a severe problem: the typical configurations in the sum were either "crumpled" (geometrically pathological, with infinite fractal dimension) or "branched polymers" (thin, tree-like structures with no resemblance to four-dimensional spacetime). Neither of these phases looks like our universe. The EDT approach failed to produce four-dimensional classical spacetime as the continuum limit of the quantum gravity path integral.

## The Causal Constraint in CDT

The key innovation of CDT is to impose a *causal constraint* on the triangulations that are summed over. Rather than summing over all Euclidean triangulations, CDT sums over *causal* triangulations: discrete spacetimes that have a well-defined foliation into spacelike hypersurfaces, with each hypersurface corresponding to a fixed "time step."

The causal constraint means that the triangulation has a global time coordinate: each simplex (the fundamental building block) is causally oriented, with spacelike faces (connecting simultaneity slices at the same time step) and timelike faces (connecting adjacent time steps). The causal structure is fixed from the beginning; configurations in which the foliation is reversed ("baby universes" branching off backward in time) are excluded.

Ambjorn, Jurkiewicz, and Loll (2000, 2004) showed that with this causal constraint, the sum over triangulations produces physically sensible results. The large-scale geometry of the resulting quantum spacetime is four-dimensional and has the right Hausdorff and spectral dimension. The continuum limit of CDT appears to describe a de Sitter-like spacetime at large scales — consistent with our universe's observed cosmological constant-dominated expansion.

## Why Causality Is Necessary

The contrast between EDT (without causal constraint) and CDT (with causal constraint) illustrates a profound point: the causal structure of spacetime — the relationship between timelike and spacelike directions, the distinction between past and future — is not merely a derived feature of the metric. It is a structural constraint on which configurations can contribute to the quantum path integral.

In EDT, the absence of any causal constraint allows configurations in which the causal structure is inconsistent: baby universes branch off and reconnect, time flows forward in some regions and backward in others, and the topology of space fluctuates wildly. The result is not a well-defined four-dimensional spacetime.

In CDT, the causal constraint enforces a consistent temporal ordering from the start. It says: configurations in which time flows backward, or in which causal structure is inconsistent, do not contribute. This is a built-in asymmetry between the temporal and spatial directions at the fundamental level.

## Philosophical Significance

CDT has several philosophically important implications.

**Causality as fundamental.** CDT demonstrates that the causal structure of spacetime — the temporal ordering of events — must be enforced as a fundamental constraint, not derived as an emergent consequence of some non-causal structure. This supports a view of causation and temporal order as basic features of physical reality, not reducible to other properties.

**Time as part of quantum gravity.** In CDT, time (in the form of the causal foliation) is present from the beginning. This is different from LQG's relational approach (which aims to eliminate external time) and from the Wheeler-DeWitt equation (which appears to eliminate time entirely). CDT suggests that some notion of temporal structure is ineliminable from quantum gravity.

**Emergence of four dimensions.** That four-dimensional spacetime emerges from the CDT sum over geometries — without being assumed a priori — is a striking result. The four-dimensionality of spacetime is not an input but an output. This supports the view that the dimensionality of spacetime (including the one time dimension) is a consequence of quantum gravitational dynamics rather than a fundamental fact about the universe.

**References**

Ambjorn, Jan, Jerzy Jurkiewicz, and Renate Loll. 2000. "A Non-Perturbative Lorentzian Path Integral for Gravity." *Physical Review Letters* 85 (5): 924–927.

Ambjorn, Jan, Jerzy Jurkiewicz, and Renate Loll. 2004. "Emergence of a 4D World from Causal Quantum Gravity." *Physical Review Letters* 93 (13): 131301.

Loll, Renate. 2019. "Quantum Gravity from Causal Dynamical Triangulations: A Review." *Classical and Quantum Gravity* 37 (1): 013002.
