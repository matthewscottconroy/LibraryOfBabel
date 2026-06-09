# 13.4 Classification of Fatou Components

The Fatou set of a rational map is an open set, and its connected components are called *Fatou components*. Understanding what can happen inside a Fatou component — what the dynamics looks like there — is a central question of complex dynamics, and it was essentially settled by Sullivan in 1985.

## The Five Types

**Definition 13.4.1.** A *Fatou component* is a connected component of $\mathcal{F}(f)$. The possible types of *periodic* Fatou components are:

1. *Attracting basin:* the basin of attraction of an attracting periodic cycle. The iterates $f^{qn}$ (for $q$ the period) converge to the periodic orbit uniformly on compact subsets.

2. *Parabolic basin:* the basin of a fixed point (or periodic point) with eigenvalue $e^{2\pi i p/q}$ — a root of unity. The orbit converges to the fixed point, but tangentially (sub-exponentially). The convergence is like $n^{-1/(q-1)}$ rather than exponential.

3. *Siegel disk:* a Fatou component on which $f^q$ is conformally conjugate to an irrational rotation $z \mapsto e^{2\pi i\alpha} z$ on the unit disk. The orbit is quasi-periodic, not converging. This requires a Diophantine condition on the rotation number $\alpha$ — if $\alpha$ is too well approximated by rationals, the Siegel disk does not exist (Cremer point instead).

4. *Herman ring:* an annulus on which $f^q$ is conformally conjugate to an irrational rotation of an annulus. Herman rings only occur for rational maps, not polynomials (a polynomial cannot have a periodic annulus in its Fatou set). The name honors Michael Herman, who proved their existence.

5. *Böttcher domain* (superattracting basin): the basin of a superattracting periodic cycle (eigenvalue 0). The dynamics is like $z \mapsto z^d$ near the periodic point.

## Sullivan's No Wandering Domains Theorem

Before Sullivan's 1985 theorem, there was a genuine question: could a Fatou component fail to be periodic? Could there be a *wandering* component — one whose forward orbit $U, f(U), f^2(U), \ldots$ consists of infinitely many distinct Fatou components, never cycling back?

For many years, no one knew. The classification of periodic components was understood (the five types above), but the possibility of wandering domains remained open.

**Theorem 13.4.2 (Sullivan's No Wandering Domains, 1985).** There are no *wandering* Fatou components: every Fatou component is preperiodic — it eventually maps into a periodic Fatou component of one of the five types above.

*(Proof sketch)* The proof uses *quasiconformal deformation theory*. If there were a wandering domain $U$, we could use it to construct an infinite-dimensional family of quasiconformal deformations of $f$ — essentially by independently choosing a Beltrami coefficient on each iterate $f^n(U)$, giving infinitely many degrees of freedom. But Teichmüller theory says the quasiconformal conjugacy class of a rational map of degree $d$ is finite-dimensional (it is parameterized by the moduli of the spaces associated to the critical points — there are only $2d-2$ of them). An infinite-dimensional family cannot fit into a finite-dimensional one — contradiction.

What this proof actually requires is the Measurable Riemann Mapping Theorem (Section 13.5), which allows translating a measurable Beltrami coefficient into an actual quasiconformal homeomorphism. This is why Sullivan's proof was so surprising and influential: it used tools from Teichmüller theory — a subject that had seemed entirely unrelated to complex dynamics — to resolve a purely dynamical question.

The contrast with transcendental entire functions is instructive. For maps like $z \mapsto e^z$, there is no analog of the Riemann-Hurwitz formula controlling the critical points, and wandering domains do in fact exist — a result due to Herman, Sullivan, and others. The finite critical point count is essential.
