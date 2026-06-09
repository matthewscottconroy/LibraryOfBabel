# Exercises — Chapter 6

These exercises range from direct verifications of definitions to connections with research problems. Work through them in order; the later ones build on the earlier ones.

---

**Exercise 6.1.** Let $f: X \to X$ be minimal. Show that $f$ has no proper closed invariant subsets. Conversely, show that if $X$ is compact and every closed invariant subset equals $X$ or $\emptyset$, then $f$ is minimal.

**Exercise 6.2.** Classify the omega-limit sets of the logistic map $f_4(x) = 4x(1-x)$ on $[0,1]$. (*Hint:* $f_4$ is conjugate to the tent map via $x = \sin^2(\pi\theta/2)$.)

**Exercise 6.3.** Prove that the doubling map $f: x \mapsto 2x \pmod{1}$ on $[0,1]$ is topologically mixing but not minimal.

**Exercise 6.4.** Let $R_\alpha: {\mathbb T} \to {\mathbb T}$ be irrational rotation. Show $R_\alpha$ is minimal using the following: if $F \subseteq {\mathbb T}$ is closed and $R_\alpha$-invariant, then $F$ is closed under translation by $\alpha$, hence by $n\alpha$ for all $n$, hence $F$ must be all of ${\mathbb T}$ (density of $\{n\alpha \pmod{1}\}$).

**Exercise 6.5.** Prove the Poincaré Recurrence Theorem for topological systems: show that if $f: X \to X$ is a homeomorphism of a compact metric space, every open set $U$ satisfies $f^n(U) \cap U \neq \emptyset$ for some $n \geq 1$.

**Exercise 6.6.** Let $f: X \to X$ be uniquely ergodic with invariant measure $\mu$. For $\mu$-a.e. $x$, the orbit of $x$ equidistributes: for every continuous $\varphi$, $\frac{1}{N}\sum_{n<N} \varphi(f^n(x)) \to \int \varphi\,d\mu$. By unique ergodicity, this convergence is *uniform* in $x$. Verify this for the rotation $R_\alpha$ using Fourier analysis.

**Exercise 6.7.** (Li-Yorke) The map $f(x) = 4x(1-x)$ on $[0,1]$ has a period-3 orbit. (a) Find it numerically. (b) Conclude by the Li-Yorke theorem that $f$ is chaotic in the Li-Yorke sense.

**Exercise 6.8.** (Research Connection) The Collatz map $T$ on ${\mathbb N}$ does not have a compact phase space, so the Krylov-Bogoliubov theorem does not directly apply. Describe the obstacles to finding an invariant probability measure for $T$ with respect to counting measure. What would such a measure look like?
