# Chapter 13 — Complex Dynamics

> *The Mandelbrot set is the most complex object in mathematics — infinitely intricate at every scale, yet defined by a single quadratic polynomial. Complex dynamics explains why.*

**Prerequisites:** Complex analysis (holomorphic functions, Riemann surfaces), Chapter 6 (topological dynamics), Chapter 11 (chaos, fractal dimension).

---

## What This Chapter Is About

There is a piece of mathematical folklore that the Mandelbrot set, defined by the utterly elementary iteration $z \mapsto z^2 + c$, contains more structure at every scale than any other object in mathematics. This is not hyperbole — it is a theorem, in the sense that Shishikura proved its boundary has Hausdorff dimension exactly 2, the maximum possible for a curve-like object in the plane. The boundary of the Mandelbrot set is as complicated as a planar set can possibly be.

The mathematics that explains this is complex dynamics: the study of iteration of holomorphic maps on the Riemann sphere. What makes complex analysis so powerful here is the rigidity of holomorphic functions. A holomorphic map is determined near any point by its power series, and this global rigidity — the analytic continuation principle — means that dynamical properties of complex maps propagate in ways that smooth real maps cannot. Small local information has global consequences.

The fundamental dichotomy of complex dynamics is the Fatou-Julia decomposition. The Fatou set is where the family of iterates is equicontinuous — where nearby orbits stay nearby, at least locally in time. The Julia set is everywhere else: the "chaotic" part where orbits diverge and the dynamics is sensitive. For a quadratic polynomial $f_c(z) = z^2 + c$, the Julia set is either a connected set (if the critical orbit stays bounded) or a Cantor set (if it escapes to infinity). This dichotomy determines the Mandelbrot set: $c \in \mathcal{M}$ precisely when the Julia set of $f_c$ is connected.

Sullivan's classification of Fatou components — proven in 1985 — completed a classification program that had been open since the 1920s. There are exactly five types of Fatou components: attracting basins, parabolic basins, Siegel disks, Herman rings, and Böttcher domains. And there are no *wandering* components: every Fatou component eventually maps into a periodic one. Sullivan's proof used quasiconformal deformation theory and the Measurable Riemann Mapping Theorem — tools from Teichmüller theory — in a completely unexpected way. It transformed complex dynamics into a subject that uses modern geometric analysis at its core.

Quasiconformal maps are the key tool. They generalize conformal maps by allowing bounded distortion of angles. The Measurable Riemann Mapping Theorem says that any measurable Beltrami coefficient (a "field of infinitesimal ellipses") can be integrated to a quasiconformal homeomorphism. This theorem is the engine of Sullivan's proof, and it is also the engine of the renormalization theory of the Mandelbrot set.

Renormalization is the deepest structural theme of the chapter. The Mandelbrot set contains infinitely many copies of itself — "baby Mandelbrot sets" — and the correspondence between a copy and the original is given by a renormalization operator: zoom in on $f_c^n$ acting on a small disk, and it looks like another quadratic polynomial. Douady and Hubbard's Straightening Theorem makes this precise. The baby Mandelbrot copies correspond exactly to renormalizable parameters. Yoccoz proved that the Mandelbrot set is locally connected at all finitely renormalizable parameters — one of the deepest partial results toward the MLC conjecture.

The MLC conjecture (Mandelbrot set is Locally Connected) is the central open problem in complex dynamics. A positive answer would give a complete combinatorial model of the parameter space — every point in $\mathcal{M}$ would be described by its "external angle" and the associated combinatorial data. The problem has been open for 40 years and continues to drive the field.

**What this chapter builds:** The Julia and Fatou sets as the fundamental dichotomy for complex iteration; the Mandelbrot set as parameter space; Sullivan's No Wandering Domains theorem; quasiconformal surgery; renormalization in complex dynamics; and the connections to hyperbolic geometry and Teichmüller theory.

---

## Sections

- [13.1 Iteration of Complex Maps](iteration-of-complex-maps.md) — Fatou and Julia sets, normality, and basic properties
- [13.2 Polynomial Dynamics](polynomial-dynamics.md) — Filled Julia sets, Böttcher coordinates, and external rays
- [13.3 Quadratic Polynomials and the Mandelbrot Set](quadratic-polynomials-and-the-mandelbrot-set.md) — The family $f_c(z) = z^2 + c$ and the structure of $\mathcal{M}$
- [13.4 Classification of Fatou Components](classification-of-fatou-components.md) — Five types, Sullivan's theorem, and the proof sketch
- [13.5 Quasiconformal Maps](quasiconformal-maps.md) — The Measurable Riemann Mapping Theorem and its applications
- [13.6 Renormalization in Complex Dynamics](renormalization-in-complex-dynamics.md) — Polynomial-like maps, the Straightening Theorem, and baby Mandelbrot sets
- [13.7 Entropy of Complex Maps](entropy-of-complex-maps.md) — Topological entropy, monotonicity, and the degree formula

---

- [Exercises](exercises.md)
- [Chapter Notes](notes.md)
