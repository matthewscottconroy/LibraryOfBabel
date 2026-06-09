# 7.3 The Ergodic Theorems

The ergodic theorems are the central results of the subject. There are two versions — one in $L^2$, one pointwise — and they were both proved in 1931. Von Neumann proved the $L^2$ version first, in the fall of 1931. Birkhoff proved the pointwise version shortly after. The competition between them is a famous episode in twentieth-century mathematics.

---

## Von Neumann's Mean Ergodic Theorem

The mean ergodic theorem is a statement about convergence in $L^2$, and its proof is essentially a statement about functional analysis: about what the Koopman operator does to orthogonal complements.

**Theorem 7.3.1 (Von Neumann, 1931).** Let $(X, \mathcal{B}, \mu, f)$ be an MPT. For $\varphi \in L^2(\mu)$, the time averages $A_N \varphi = \frac{1}{N}\sum_{n=0}^{N-1} \varphi \circ f^n$ converge in $L^2$ to the projection $P\varphi$ onto the closed subspace of $f$-invariant functions: $\{g \in L^2 : g \circ f = g \text{ a.e.}\}$.

*(proof)* The Koopman operator $U_f: L^2 \to L^2$, $U_f(\varphi) = \varphi \circ f$, is an isometry (since $\mu$ is $f$-invariant) and hence unitary if $f$ is invertible. The theorem reduces to showing $(1/N)\sum_{n=0}^{N-1} U_f^n$ converges strongly to the orthogonal projection onto $\ker(U_f - I)$. This follows from the spectral theory: $P$ is the spectral projection onto eigenvalue $1$.

What this is really saying: if you average an observable along the orbit, the average converges in the $L^2$ sense to the function's projection onto the space of invariant functions. If the system is ergodic (the only invariant functions are constants), then $P\varphi = \int \varphi\,d\mu$ is just the constant equal to the space average. So $L^2$-ergodicity gives you: time averages converge in $L^2$ to the space average.

The mean ergodic theorem is elegant and clean, but $L^2$ convergence leaves something to be desired: it only tells you about convergence "on average," not about what happens at any specific initial condition. For that, you need Birkhoff.

---

## Birkhoff's Pointwise Ergodic Theorem

Birkhoff proved this in 1931, and it immediately reorganized how people thought about statistical mechanics. The proof is harder — it requires the maximal ergodic theorem as a technical lever — but the conclusion is everything you'd hope for.

**Theorem 7.3.2 (Birkhoff, 1931).** Let $(X, \mathcal{B}, \mu, f)$ be an MPT and $\varphi \in L^1(\mu)$. Then for $\mu$-a.e. $x$, the time averages converge:
$$\lim_{N \to \infty} \frac{1}{N} \sum_{n=0}^{N-1} \varphi(f^n(x)) = \varphi^*(x),$$
where $\varphi^* \in L^1(\mu)$ satisfies $\varphi^* \circ f = \varphi^*$ a.e. (invariance) and $\int \varphi^*\,d\mu = \int \varphi\,d\mu$.

*(proof sketch)* The hard part is the a.e. convergence. The key tool is the *Maximal Ergodic Theorem*: $\mu\{x : \sup_N A_N \varphi(x) > \alpha\} \leq \frac{1}{\alpha}\int_{\{\sup A_N \varphi > \alpha\}} \varphi\,d\mu$. From this, one shows the set where $\limsup A_N \varphi > \liminf A_N \varphi$ has measure zero, using a "truncation and approximate" argument (Riesz's sunrise lemma).

**Key observation:** If $f$ is ergodic (Definition 7.4.1), then $\varphi^*$ is constant a.e., equal to $\int \varphi\,d\mu$. This is the precise statement that time averages equal space averages.

What this is really saying: for almost every starting point $x$, the time average of $\varphi$ along the orbit of $x$ converges to a well-defined limit. That limit is an invariant function — it looks the same at $x$ and at $f(x)$. If the system is ergodic, the only invariant functions are constants, so the limit must be the constant $\int \varphi\,d\mu$. One orbit, run long enough, gives you the same answer as integrating over the whole space.

---

## Applications

The Birkhoff theorem has an astonishing range of applications. Two classical ones:

**Normal numbers:** $x \in [0,1]$ is *normal in base 2* if the density of 1s in its binary expansion is $1/2$. Birkhoff's theorem applied to the doubling map and $\varphi = \mathbf{1}_{[1/2,1]}$ gives: Lebesgue-a.e. $x$ is normal in base 2.

This is a beautiful result. Birkhoff's theorem says that for Lebesgue-a.e. $x$, the fraction of time the doubling map spends in $[1/2, 1]$ equals $\mu([1/2, 1]) = 1/2$. But spending time in $[1/2, 1]$ is exactly having a 1 in the binary expansion. So almost every real number has equally many 0s and 1s in its binary expansion. Proving this directly from the definition of normality would be painful. The ergodic theorem gives it for free.

**Borel-Cantelli:** The Borel-Cantelli lemma follows from Birkhoff applied to suitable characteristic functions — another instance of ergodic theory providing "free" probability results.

The power of the ergodic theorem is that it converts dynamical information (orbit behavior) into probabilistic information (long-run statistics). The next section sharpens this by studying what extra condition — ergodicity — makes the limit constant.
