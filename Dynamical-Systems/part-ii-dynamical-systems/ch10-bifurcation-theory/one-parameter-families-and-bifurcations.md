# 10.1 One-Parameter Families and Bifurcations

The setup for bifurcation theory is simple: you have a family of dynamical systems indexed by a parameter $\mu$, and you want to understand how the dynamics change as $\mu$ varies. The interesting thing is that for "most" values of $\mu$, the qualitative behavior is stable — small changes in $\mu$ don't change the orbit structure. But at certain special values, qualitative changes occur. These are the bifurcation values.

**Definition 10.1.1.** A *one-parameter family* of vector fields is a smooth map $f: {\mathbb R}^n \times {\mathbb R} \to {\mathbb R}^n$, $(x, \mu) \mapsto f_\mu(x)$. A *bifurcation value* is a parameter $\mu_0$ where the phase portrait of $f_\mu$ changes qualitatively as $\mu$ passes through $\mu_0$.

**Definition 10.1.2.** A bifurcation is *local* if the qualitative change occurs near a fixed point or periodic orbit; *global* if it involves changes in large-scale orbit structure (homoclinic/heteroclinic connections, period-infinity limits).

The distinction between local and global is important both conceptually and technically. Local bifurcations can be analyzed using only the behavior near the equilibrium — the center manifold theorem reduces the problem to a finite-dimensional one, and normal forms reduce it further to a canonical polynomial. Global bifurcations require understanding the whole phase space, and the analysis is correspondingly harder and less complete.

The classification of local bifurcations is a beautiful success story: there are only a few "generic" bifurcations of codimension 1, and each is completely described by a normal form. The next section develops the three basic ones for equilibria.
