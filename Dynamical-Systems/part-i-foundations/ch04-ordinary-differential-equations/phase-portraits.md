# 4.6 Phase Portraits and Qualitative Analysis

Qualitative analysis asks: without solving an ODE explicitly, what can you say about the long-time behavior of solutions? The key objects are the *limit sets* — the sets of accumulation points of orbits as time goes to $\pm\infty$ — and the structures connecting them.

## 4.6.1 Equilibria, Limit Cycles, and Heteroclinic Orbits

**Definition 4.6.1.** For a flow $\Phi_t$ on a phase space $M$:
- The *orbit* of $p$: $\mathcal{O}(p) = \{\Phi_t(p) : t \in \mathbb{R}\}$
- The *omega-limit set*: $\omega(p) = \bigcap_{T>0} \overline{\{\Phi_t(p) : t > T\}}$ — the set of all accumulation points of the positive orbit as $t \to +\infty$
- The *alpha-limit set*: $\alpha(p) = \bigcap_{T>0} \overline{\{\Phi_t(p) : t < -T\}}$ — as $t \to -\infty$
- A *periodic orbit* (limit cycle): $\Phi_T(p) = p$ for some minimal $T > 0$
- A *homoclinic orbit*: an orbit in $W^s(p) \cap W^u(p)$ for the same equilibrium $p$
- A *heteroclinic orbit*: an orbit in $W^s(q) \cap W^u(p)$ for distinct equilibria $p, q$

The omega-limit set is the "attractor" of the orbit. If the orbit converges to a fixed point, $\omega(p) = \{q\}$. If it converges to a periodic orbit, $\omega(p) =$ the periodic orbit. If the system is chaotic, $\omega(p)$ can be a complicated Cantor set-like object.

**Properties of Omega-Limit Sets:**
1. $\omega(p)$ is closed and positively invariant (the flow maps it to itself).
2. $\omega(p)$ is connected if the orbit of $p$ is bounded.
3. $\omega(p) = \emptyset$ iff the orbit escapes to infinity.

Property (2) is a key constraint. In two dimensions, the Jordan curve theorem implies $\omega(p)$ must be one of: a fixed point, a periodic orbit, or a homoclinic/heteroclinic connection. This is the content of Poincaré-Bendixson (Section 3.8). In three and higher dimensions, $\omega(p)$ can be a strange attractor.

## 4.6.2 Poincaré Maps

One of the most powerful reduction techniques in dynamical systems: study a periodic orbit by studying the *return map* to a transverse section.

**Definition 4.6.2.** Let $\Sigma$ be a smooth hypersurface transverse to the flow near a periodic orbit $\gamma$. The *Poincaré return map* is $P: \Sigma \to \Sigma$ defined by $P(x) = \Phi_{\tau(x)}(x)$, where $\tau(x)$ is the first return time.

The return map is a diffeomorphism of the $(n-1)$-dimensional section $\Sigma$. Studying the periodic orbit $\gamma$ reduces to studying the fixed point $p = \gamma \cap \Sigma$ of $P$. This is a remarkable dimension reduction: a question about a periodic orbit in $\mathbb{R}^n$ becomes a question about a fixed point in $\mathbb{R}^{n-1}$.

The stability of $\gamma$ is determined by the eigenvalues of $DP(p)$ at the fixed point — these are the *Floquet multipliers*. If all Floquet multipliers have absolute value less than 1, the periodic orbit is asymptotically stable. If any multiplier has absolute value greater than 1, the orbit is unstable.

The Poincaré map is how we study all questions about periodic orbits: their stability, their bifurcations, the structure of nearby orbits. It converts continuous-time dynamics into discrete-time dynamics, which is often easier to analyze. This is the bridge from the ODE theory of this chapter to the discrete dynamics of the rest of the book.
