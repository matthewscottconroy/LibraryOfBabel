# Section 14.1: Newton's Laws and the Foundations of Classical Mechanics

---

## Section Introduction

Newton's laws of motion are stated in three sentences, but their implications fill entire libraries. The first law defines what "no force" means; the second quantifies the response to force; the third establishes action-reaction pairs. Together, they define an inertial frame and give meaning to the concept of "force."

But Newton's mechanics rests on a metaphysical foundation that became increasingly uncomfortable: absolute space and absolute time. The claim that there exists a preferred state of rest — absolute space — in which Newton's laws take their simplest form, was challenged by Leibniz in the seventeenth century, by Mach in the nineteenth, and by Einstein in the twentieth. Understanding Newton's absolute space and its problems is essential preparation for understanding why Einstein had to replace it.

---

## 14.1.1 The Three Laws of Motion

**Newton's First Law** (Law of Inertia): Every body persists in its state of rest or uniform motion in a straight line unless acted upon by an external force.

This law is not merely the special case F = 0 of the second law — it defines the class of **inertial frames**: those in which a body with no net force moves in a straight line at constant speed. The first law is a conceptual framework before it is a computational tool.

**Newton's Second Law**: The net force on a body equals its mass times its acceleration:

$$\mathbf{F} = m\mathbf{a} = m\frac{d^2\mathbf{r}}{dt^2}$$

Two things are hidden in this simple equation:
1. **Inertial mass** m: the constant of proportionality between force and acceleration. It measures a body's resistance to change of motion.
2. **The Euclidean/absolute structure**: the acceleration d²r/dt² is defined with respect to absolute space and measured by absolute time. The law presupposes a specific geometric and temporal background.

**Newton's Third Law**: For every force that body A exerts on body B, body B exerts an equal and opposite force on body A: F_{AB} = −F_{BA}.

The third law, combined with the second, implies conservation of total momentum: d/dt(m_A v_A + m_B v_B) = F_{AB} + F_{BA} = 0. Momentum conservation follows directly.

---

## 14.1.2 Absolute Space, Absolute Time, and Their Critics

Newton's *Principia* (1687) contains a famous scholium asserting:

> "Absolute, true and mathematical time, of itself, and from its own nature, flows equably without relation to anything external."
> "Absolute space, in its own nature, without relation to anything external, remains always similar and immovable."

**The rotating bucket argument**: Newton argued for the existence of absolute space via the bucket experiment. A bucket of water, initially at rest, is set spinning. Initially the water surface is flat. As the water begins to spin with the bucket, the surface becomes concave. The concavity reflects real centrifugal acceleration — acceleration with respect to absolute space, not with respect to the bucket. Even if the bucket and water rotate "together," the concavity reflects true rotation.

**Leibniz's critique**: Gottfried Leibniz (1646–1716) argued that absolute space is meaningless — there is no observational difference between "the universe at rest" and "the universe moving uniformly," so absolute rest is a metaphysical fiction. Only relative positions and motions are physically meaningful (the Leibniz shift argument).

**Mach's principle**: Ernst Mach (1838–1916) argued in *The Science of Mechanics* (1883) that inertia itself arises from the distribution of matter in the universe. The rotating bucket's surface is concave because the water rotates with respect to the fixed stars — not with respect to an abstract absolute space. If the fixed stars were removed, Mach argued, there would be no centrifugal effect.

**Einstein's response**: Einstein was profoundly influenced by Mach. His GR was partially motivated by Mach's principle. GR partially embeds Mach's insight: the metric (which determines "inertial frames" locally) is determined by the distribution of matter and energy via the Einstein equations. However, GR spacetimes can exist with trivial matter content (Minkowski, de Sitter), so Mach's principle is not fully embodied in GR.

---

## 14.1.3 Inertial Frames and Galilean Relativity

**Inertial frame**: A reference frame in which Newton's first law holds — a non-accelerating, non-rotating frame. Any frame moving at constant velocity relative to an inertial frame is also inertial.

**Galilean transformation**: Between two inertial frames S and S' moving at relative velocity v:

$$t' = t, \quad \mathbf{r}' = \mathbf{r} - \mathbf{v}t$$

Newton's laws are invariant under Galilean transformations (Galilean relativity): F = ma is the same in all inertial frames. However, the position and velocity transform; only acceleration is invariant.

**The symmetry group of Newtonian mechanics**: The Galilean group consists of:
- Spatial rotations (SO(3)): r → Rr
- Spatial translations: r → r + a
- Time translations: t → t + s
- Boosts: r → r − vt, t → t

This is a 10-parameter group. The corresponding conserved quantities (by Noether's theorem, Section 15.2) are: angular momentum, linear momentum, energy, and the position of the center of mass.

**The failure of Galilean symmetry**: Maxwell's equations (Unit VI) are not invariant under Galilean transformations — they predict a specific wave speed c regardless of the observer's motion. This contradiction led to the Michelson-Morley experiment (1887), special relativity (1905), and the replacement of the Galilean group by the Poincaré group (Lorentz transformations + translations).

---

## 14.1.4 Gravitational Mass and the Equivalence Principle

Newton's law of gravitation introduces a second kind of mass: **gravitational mass** m_G, the property that determines how strongly gravity acts on a body.

The gravitational force on a body at position r in a gravitational field g is:

$$\mathbf{F}_{\rm grav} = m_G \mathbf{g}$$

The second law gives the body's acceleration: $m_I \mathbf{a} = m_G \mathbf{g}$, so $\mathbf{a} = (m_G/m_I)\mathbf{g}$.

**Galileo's observation**: All bodies fall with the same acceleration in a gravitational field, regardless of mass or composition. This is Galileo's experiment from the Tower of Pisa (perhaps legendary) and his inclined plane experiments (real). It implies m_G/m_I = constant — which can be set to 1 by choosing units.

**Eötvös experiment (1890)**: Hungarian physicist Loránd Eötvös (1848–1919) measured the equality of gravitational and inertial mass with a torsion balance to 5 parts in 10⁹. Modern Eötvös experiments (Braginsky and Panov, 1971; Schlamminger et al., 2008) have confirmed m_G/m_I = 1 to better than 1 part in 10¹³ for a variety of materials.

**The Weak Equivalence Principle**: The equality m_G = m_I for all bodies, regardless of composition. All test bodies fall with the same acceleration in a gravitational field.

**Einstein's Equivalence Principle**: Stronger version. Not only do all bodies fall the same way, but there is no local experiment (mechanical or electromagnetic) that can distinguish between a uniform gravitational field and a uniformly accelerating reference frame. Gravity is locally indistinguishable from acceleration.

This is the foundation of GR: if gravity is locally equivalent to acceleration, then gravity must be a property of the reference frame — and changing reference frames is equivalent to changing the geometry of spacetime. Gravity is geometry.

---

## 14.1.5 The Newtonian Limit of GR

GR must reduce to Newtonian mechanics in the limit of:
- **Weak fields**: the metric is nearly flat, g_{μν} ≈ η_{μν} + h_{μν} with |h_{μν}| ≪ 1.
- **Slow motion**: v ≪ c, so dt/dτ ≈ 1 and the spatial components of the geodesic equation dominate.
- **Stationary fields**: ∂g_{μν}/∂t ≈ 0.

In this limit, the geodesic equation $\ddot{x}^\mu + \Gamma^\mu_{\nu\rho}\dot{x}^\nu\dot{x}^\rho = 0$ becomes:

$$\frac{d^2 x^i}{dt^2} \approx -\Gamma^i_{00} = -\frac{1}{2}\partial^i h_{00}$$

Comparing with Newton's second law $\ddot{x}^i = -\partial^i \Phi$ (where Φ is the Newtonian potential), we get:

$$h_{00} = -\frac{2\Phi}{c^2}$$

The Schwarzschild metric gives h_{00} = −r_s/r = −2GM/(rc²) = 2Φ/c² (with Φ = −GM/r). ✓

The full post-Newtonian expansion — corrections to Newtonian gravity ordered in powers of (v/c)² and (Φ/c²) — is the framework for precision tests of GR in the solar system and for the template calculations needed for gravitational wave observations.

---

## References

- Newton, I. (1687). *Philosophiae Naturalis Principia Mathematica.* London. Translated by I.B. Cohen and A. Whitman (1999), University of California Press. [The original; the three laws and the law of universal gravitation are in Book I. The famous scholium on absolute space and time is in the beginning of the Definitions.]
- Mach, E. (1883). *Die Mechanik in ihrer Entwickelung historisch-kritisch dargestellt.* F.A. Brockhaus. English translation: *The Science of Mechanics*, Open Court, 1960. [Mach's critique of Newton's absolute space; the bucket experiment analysis; Mach's principle. Directly influenced Einstein.]
- Eötvös, R., Pekar, D., and Fekete, E. (1922). "Beiträge zum Gesetze der Proportionalität von Trägheit und Gravität." *Annalen der Physik*, 68, 11–66. [The Eötvös experiment; measured the equality of gravitational and inertial mass to parts in 10⁹.]
- Will, C.M. (2014). "The confrontation between general relativity and experiment." *Living Reviews in Relativity*, 17, 4. [The comprehensive review of tests of GR; includes current limits on the equivalence principle, post-Newtonian parameters, and GR predictions vs. experiment.]
