# 7.5 Mixing

Ergodicity says: time averages equal space averages. Mixing asks for something stronger: as time goes on, knowing where you started tells you less and less about where you are. The correlation between past and future decays.

**Definition 7.5.1.** An ergodic MPT $(X, \mathcal{B}, \mu, f)$ is:
- *Weakly mixing* if for all $A, B \in \mathcal{B}$: $\frac{1}{N}\sum_{n=0}^{N-1} |\mu(f^{-n}(A) \cap B) - \mu(A)\mu(B)| \to 0$
- *Strongly mixing* (or just *mixing*) if for all $A, B \in \mathcal{B}$: $\mu(f^{-n}(A) \cap B) \to \mu(A)\mu(B)$ as $n \to \infty$

Mixing $\Rightarrow$ weak mixing $\Rightarrow$ ergodic.

Let's unpack these. Strong mixing says: the proportion of $B$ that $f^n$ maps into from $A$ converges to $\mu(A)\mu(B)$ — that is, the events "$x \in A$" and "$f^n(x) \in B$" become asymptotically independent as $n \to \infty$. The past becomes uncorrelated with the arbitrarily far future.

Weak mixing is subtler. The Cesàro average of the correlations $|\mu(f^{-n}(A) \cap B) - \mu(A)\mu(B)|$ tends to zero, but the individual terms might not. This is mixing "on average" — there might be exceptional times when the correlation is large, but they become increasingly rare.

---

## Spectral Characterizations

The mixing hierarchy has elegant formulations in terms of the Koopman operator (which we'll develop fully in Section 7.6).

**Spectral Characterizations:**
- $f$ is ergodic iff $1$ is a simple eigenvalue of $U_f$
- $f$ is weakly mixing iff $U_f$ has no eigenvalue other than $1$ (equivalently, $U_f$ has purely continuous spectrum on $L^2_0 = \{g \in L^2 : \int g = 0\}$)
- $f$ is mixing iff $\langle U_f^n g, h \rangle \to 0$ for all $g, h \in L^2_0$ (matrix elements of $U_f^n$ tend to 0)

These spectral characterizations are not just convenient — they show that mixing is fundamentally a property of how $U_f$ acts on the orthogonal complement of constants. Ergodicity means: constants are the only invariant direction. Weak mixing means: there are no periodic directions (no eigenvalues other than 1 on $L^2_0$). Strong mixing means: everything in $L^2_0$ gets "spread out" by $U_f^n$ — all inner products decay to zero.

---

## Examples

**Example 7.5.2 (Mixing and Nonmixing).**
- Bernoulli shifts are mixing: $\mu(\sigma^{-n}(A) \cap B) \to \mu(A)\mu(B)$ since distant coordinates are independent.
- Irrational rotation $R_\alpha$ is NOT mixing: take $A = B = [0, \varepsilon]$. Along the subsequence where $n\alpha$ is close to 0, $\mu(R_\alpha^{-n}(A) \cap A) \approx \mu(A)^2 \cdot 1/\mu(A) = \mu(A)$, not $\mu(A)^2$.
- The Chacon system is weakly mixing but not mixing.

The failure of mixing for irrational rotations is intuitive: the rotation is rigid. The image of a small arc $A$ under $R_\alpha^n$ is another arc of the same size, rotating around the circle. It must return close to $A$ (by the recurrence theorem), so the correlation $\mu(R_\alpha^{-n}(A) \cap A)$ doesn't decay — it oscillates. This is the Fourier interpretation: the rotation has pure point spectrum (eigenvalues $e^{2\pi i n\alpha}$), and pure point spectrum prevents mixing.

Bernoulli shifts are at the opposite extreme. Since the coordinates are independent, the correlation between what happens at time 0 and what happens at time $n$ is exactly zero when $n > 0$. Mixing is immediate.

The Chacon system sits between these extremes. Constructed by Chacon in the 1960s, it is ergodic with no non-trivial eigenvalues (hence weakly mixing), but it is not mixing. It requires a careful "cutting and stacking" construction.

In the next section, we develop the spectral theory that underlies all of this.
