# Section 34.2: Einstein's Elevator

---

## The Thought Experiment

1907\. Einstein is working at the patent office in Bern. He is trying to incorporate gravity into special relativity. Special relativity works beautifully for electromagnetism — Maxwell's equations are Lorentz-invariant. But Newton's gravity is instantaneous and Galilean, not relativistic. How to reconcile them?

Then the thought: *"A man falling freely from the roof of a house experiences no gravitational field during his fall."*

Einstein called this "the happiest thought of my life." It became the seed of GR.

---

## The Freely Falling Elevator

Consider an elevator in deep space, far from all masses, accelerating upward with acceleration $g$. A passenger inside releases a ball. From the elevator's perspective, the ball falls to the floor — it appears to accelerate downward at rate $g$. The passenger might conclude there is a gravitational field pulling things downward with acceleration $g$.

Now consider a stationary elevator on the surface of the Earth, in a gravitational field $g$. A passenger releases a ball. The ball falls to the floor, accelerating downward at rate $g$.

The passenger in the accelerating elevator and the passenger in the gravitational field observe exactly the same phenomena. If the walls are opaque and the windows are covered, no local experiment can distinguish them.

**This is the equivalence principle** in its elevator form: a uniformly accelerating reference frame is locally indistinguishable from a stationary frame in a uniform gravitational field.

Conversely: a freely falling elevator (cutting the cable) is locally indistinguishable from an inertial frame in empty space with no gravity. The passenger feels weightless; objects float beside them; light travels in straight lines; all the laws of special relativity hold. The gravitational field has been "transformed away" by choosing the freely falling frame.

---

## What "Locally" Means

The equivalence is *local*: it holds only in a small enough region of spacetime and for a short enough time. Over a large region or a long time, tidal forces appear — differential gravitational accelerations between different parts of the elevator — that cannot be removed by any change of frame. Two freely falling balls, released side-by-side in the Earth's gravitational field, slowly converge toward the Earth's center: they are drawn toward the same point. An elevator extending from one side of the Earth to the other would show this tidal effect very clearly.

Tidal forces are the physical signature of genuine gravitational curvature. They are what distinguishes a gravitational field (curved spacetime) from a mere accelerated reference frame (flat spacetime). The equivalence principle says: in a small enough region, you can always choose a free-fall frame that is locally flat. The curvature — the tidal effects — appears only when you look at second-order effects over a finite region.

Mathematically:
- At any point $p$, there exist normal coordinates in which $g_{\mu\nu}(p) = \eta_{\mu\nu}$ and $\partial_\rho g_{\mu\nu}(p) = 0$ (first derivatives vanish).
- The second derivatives $\partial_\rho\partial_\sigma g_{\mu\nu}(p)$ cannot in general be made to vanish — they encode the Riemann curvature tensor.

The size $\ell$ of the "locally flat" region is determined by the curvature scale: $\ell^2 \sim 1/|R_{\mu\nu\rho\sigma}|$.

---

## The Equivalence Principle as a Guiding Principle for GR

The equivalence principle does three things for us:

**1. It tells us that gravity must be described by a metric.** If the laws of physics in a freely falling frame are those of SR (metric $\eta_{\mu\nu}$), and a gravitational field is a change to a non-inertial frame, then gravity is encoded in how the metric changes from point to point — i.e., in the metric tensor $g_{\mu\nu}(x)$ of a curved spacetime.

**2. It provides the "minimal coupling" prescription for matter in curved spacetime.** To take any SR equation and make it valid in curved spacetime: (a) replace $\eta_{\mu\nu}$ with $g_{\mu\nu}$, (b) replace $\partial_\mu$ with the covariant derivative $\nabla_\mu$, and (c) replace $d^4x$ with $\sqrt{-g}\,d^4x$. This is the minimal coupling prescription; it follows from requiring that in any locally inertial frame, the equation reduces to its SR form.

**3. It constrains the form of the field equations.** The field equations must reduce to SR physics in a freely falling frame, and they must be generally covariant (valid in any coordinate system). These constraints, combined with the requirement that they reduce to Poisson's equation in the Newtonian limit, uniquely determine the Einstein field equations (up to the cosmological constant), as Lovelock's theorem shows.

---

## Inertia, Gravity, and Mach's Principle

The equivalence principle raises a deep question about the nature of inertia. Newton's mechanics requires absolute space: the resistance of an object to acceleration (inertial mass) is a resistance to acceleration relative to *something* — but what? Newton said it was absolute space itself (the bucket argument: a rotating bucket carries water that climbs the walls, even if you imagine all the matter in the universe removed).

Ernst Mach (1883) argued that inertia is not resistance to acceleration relative to absolute space but to acceleration relative to the *distribution of matter in the universe*. If you could remove all the distant stars and galaxies, there would be no inertia. On Mach's view, the inertial frame is determined by the large-scale matter distribution.

Einstein was deeply influenced by Mach. He hoped GR would be "Machian" — that the metric (and hence the inertial frames) would be entirely determined by the matter distribution, leaving no freedom for the metric in the absence of matter. This hope was not fully realized: GR admits solutions with no matter (empty Minkowski spacetime, empty de Sitter space) and curved solutions without matter (gravitational waves, Schwarzschild). The relationship between GR and Mach's principle remains an active area of debate.

The Lense-Thirring effect ("frame dragging") is GR's partial accommodation of Machian ideas: a rotating massive body drags the local inertial frames — the locally free-fall frames are slightly rotating relative to the distant stars. This was confirmed by Gravity Probe B (2011): the gyroscopes on the satellite precessed at 39.2 milliarcseconds per year (geodetic precession, from the curvature of spacetime around Earth) and 6.1 milliarcseconds per year (frame-dragging precession), both consistent with GR.

---

## The Principle of General Covariance

The equivalence principle implies that the equations of physics must be **generally covariant**: they must take the same form in any coordinate system. This is not merely a mathematical requirement (that equations be tensorial) but a physical requirement (that there are no preferred coordinate systems — no absolute space or time).

General covariance is closely related to, but distinct from, diffeomorphism invariance. A diffeomorphism is a smooth bijection from the spacetime manifold to itself. Diffeomorphism invariance says that physically, different-looking metric descriptions that are related by a diffeomorphism describe the same physical spacetime. This is the GR analog of gauge invariance in electromagnetism.

The gauge group of GR is the group of diffeomorphisms $\text{Diff}(M)$, which is infinite-dimensional. This makes GR much harder to quantize than Yang-Mills gauge theories, whose gauge groups are finite-dimensional Lie groups. The reconciliation of GR's diffeomorphism invariance with quantum mechanics is the central problem of quantum gravity.

---

## Summary

The equivalence principle — $m_i = m_g$, or equivalently, gravity vanishes in a freely falling frame — is the cornerstone of GR. It:

- Explains why all objects fall with the same acceleration (the geodesic equation has no mass).
- Implies that gravity curves light (light bends because it follows geodesics in curved spacetime).
- Implies gravitational redshift (clocks run slower in stronger gravitational fields).
- Tells us that the locally measured physics must be that of SR (the metric is locally Minkowskian).
- Mandates the minimal coupling prescription for matter in curved spacetime.
- Requires general covariance (the equations of physics must be tensorial, independent of coordinates).

Every one of these predictions has been tested and confirmed. The equivalence principle is, without any qualification, one of the best-tested principles in all of science.

