# Noether's Theorem

## Emmy Noether and the 1915 Paper

Emmy Noether (1882–1935) was one of the greatest mathematicians of the twentieth century, described by Albert Einstein as "the most significant creative mathematical genius thus far produced, since the higher education of women began" (Einstein 1935). Her contributions to abstract algebra — in particular the theory of rings, ideals, and modules — were foundational. But for physicists and philosophers of physics, her most important single result is the theorem she proved in 1915 and published in 1918: "Invariante Variationsprobleme" ("Invariant Variation Problems"), in the proceedings of the Royal Society of Sciences at Göttingen (Noether 1918).

The paper was motivated by a problem that David Hilbert and Felix Klein had encountered in the early development of general relativity: the conservation of energy in Einstein's theory was behaving strangely, in ways that seemed to threaten its applicability. Noether's theorem resolved the puzzle by revealing a general relationship between symmetries and conservation laws that applied to any theory formulable in terms of an action principle.

The result has two parts (corresponding to Noether's two "theorems" in the paper). The first, which is the one most relevant here, concerns theories with finite-dimensional symmetry groups. The second concerns theories with infinite-dimensional symmetry groups (like general relativity). We focus on the first.

## The Action Principle

To understand Noether's theorem, we need to understand the action principle, which is the most general formulation of classical mechanics. Rather than describing physics by forces and accelerations, the action formulation describes it by a *functional*: the *action* S, which assigns a number to each possible trajectory of a system. The actual trajectory of the system is the one that *extremizes* (minimizes or makes stationary) the action.

For a system with generalized coordinates *q* and velocities *dq/dt*, the action is:

*S = ∫ L(q, dq/dt, t) dt*

where *L* is the *Lagrangian* of the system, which encodes all the physical information about its dynamics. The condition that S is stationary with respect to variations of the path *q(t)* gives the *Euler-Lagrange equations*, which are the equations of motion.

The action formulation is not merely a computational convenience; it is the deepest and most general way to state the laws of classical and quantum physics. Almost every fundamental theory in physics — classical mechanics, electromagnetism, general relativity, the Standard Model of particle physics — can be formulated in terms of an action principle. Noether's theorem, stated at this level of generality, applies to all of them.

## The Theorem Stated

Noether's theorem (first theorem): *If the action of a physical system is invariant under a continuous one-parameter group of transformations, then there exists a corresponding conserved quantity.*

Let us unpack this carefully.

**"Invariant under a continuous one-parameter group of transformations":** A symmetry transformation is a change to the variables of the system that leaves the action unchanged. Continuous means the transformation can be performed by gradually turning a dial — there is a real-number parameter *ε*, with the identity transformation at *ε* = 0, and the symmetry transformation at any other value of *ε*. Translation in time, rotation in space, and velocity boosts are all examples of continuous symmetry transformations.

**"Corresponding conserved quantity":** For each such symmetry, there is a function of the position and momentum variables of the system that remains constant throughout the system's motion — it has the same value at every point along any solution to the equations of motion.

## Time-Translation Symmetry and Energy Conservation

The most important application of Noether's theorem for our purposes is the connection between *time-translation symmetry* and *energy conservation*.

Time-translation symmetry is the statement that the laws of physics are the same at all times: if you shift every event in a physical process forward by one hour (replacing *t* with *t + τ* for some constant *τ*), the equations of motion are unchanged. This is a symmetry of the action in the sense that the Lagrangian *L* does not depend explicitly on time: *∂L/∂t = 0*.

Noether's theorem tells us that this symmetry corresponds to a conserved quantity. The calculation is straightforward: if *L* does not depend explicitly on *t*, then one can show that the quantity:

*E = (dq/dt)(∂L/∂(dq/dt)) - L*

is conserved — its time derivative is zero along any solution. For a particle moving under a potential *V*, this quantity turns out to be exactly the total energy: kinetic energy plus potential energy. The conservation of energy is not a brute fact about the world; it follows from the time-translation symmetry of the laws.

This is the content of Noether's theorem as applied to time: energy is conserved *because* the laws of physics are the same at all times. Energy conservation and temporal uniformity are two sides of the same coin.

## The Spatial Analogues

To appreciate how non-trivial this connection is, consider the spatial analogues. Noether's theorem also tells us that:

- **Translational symmetry** (the laws are the same everywhere in space) → **conservation of linear momentum**
- **Rotational symmetry** (the laws are the same in all directions) → **conservation of angular momentum**

These are all cases of the same general principle: symmetry of the laws under a transformation produces a corresponding conserved quantity. The conservation laws we learn in introductory physics are not independent facts to be verified separately; they are all consequences of the symmetries of the physical laws.

## When Time-Translation Symmetry Breaks: Cosmology

The consequences become philosophically interesting when we consider situations where time-translation symmetry is violated. If energy conservation follows from the time-translation symmetry of the laws, what happens in systems where that symmetry is broken?

The most important such situation is cosmology. The universe is expanding. As it expands, the scale factor *a(t)* — a measure of the size of the universe — changes with time. In a universe with a cosmological expansion, the background spacetime is not time-translation symmetric: conditions today are different from conditions a billion years ago (the universe was smaller, hotter, denser). General relativity's field equations, which govern this expansion, are not globally time-translation invariant.

The consequence, by Noether's theorem, is that energy is not globally conserved in an expanding universe. This is a profound and often-misunderstood point. When we see distant galaxies, the light reaching us has been redshifted — its wavelength has been stretched by the expansion of space, and its energy reduced. The "missing" energy is not stored anywhere; it is genuinely not conserved. This is not a problem with our physics; it is a direct consequence of the fact that the expanding universe is not time-translation symmetric.

This does not mean that physics is arbitrary or unpredictable in a cosmological context — general relativity gives precise, testable predictions about the expansion. But it does mean that energy conservation, far from being an absolute law of nature, is a consequence of a symmetry that holds in many contexts but not universally.

## What This Tells Us About Time and Energy

Noether's theorem reveals a deep structural relationship between time and energy that goes beyond what any particular experiment could show. Energy is not just a useful quantity to track because it happens to be conserved; it is *defined* — in the Noether sense — by the temporal structure of the theory. The total energy of a closed system is the quantity that is conjugate to time under the dynamics: it is the generator of time translations in the Hamiltonian formalism.

This conjugacy between time and energy has consequences elsewhere in physics. In quantum mechanics, the energy-time uncertainty relation *ΔE·Δt ≥ ℏ/2* reflects the same conjugacy (though with important differences from the position-momentum case, as we will discuss in Chapter 24). In quantum field theory, the Hamiltonian — the operator representing total energy — is the generator of time evolution of the quantum state. Time and energy are linked not just contingently but structurally: what we mean by "time" in a dynamical system and what we mean by "energy" in that system are two aspects of a single mathematical relationship.

This is among the most beautiful results in theoretical physics, and it is entirely due to Emmy Noether, a woman who was denied a professorship at Göttingen for years because of her gender, who was expelled from Germany in 1933 by the Nazi regime, and who died in 1935 at the age of 53, two years after finding refuge at Bryn Mawr College in Pennsylvania. The depth of her contribution deserves acknowledgment alongside its elegance.

**References**

Noether, Emmy. 1918. "Invariante Variationsprobleme." *Nachrichten von der Gesellschaft der Wissenschaften zu Göttingen, Mathematisch-Physikalische Klasse* 1918: 235–257. English translation by M. A. Tavel: "Invariant Variation Problems," *Transport Theory and Statistical Physics* 1 (3): 186–207, 1971.

Einstein, Albert. 1935. "The Late Emmy Noether: Professor Einstein Writes in Appreciation of a Fellow-Mathematician." *New York Times*, May 4, 1935.

Kosmann-Schwarzbach, Yvette. 2011. *The Noether Theorems: Invariance and Conservation Laws in the Twentieth Century*. Translated by Bertram E. Schwarzbach. New York: Springer.

Baez, John C. 2012. "The Noether Theorem in a New Light." *AMS Notices* 59 (4): 538–540.
