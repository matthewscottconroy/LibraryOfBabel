# 20.1.1 Takens' Embedding Theorem and Reservoir State Spaces

## The Reconstruction Problem

Suppose you observe a chaotic system through a scalar measurement: you can measure only one quantity $y(t) = h(\mathbf{s}(t))$, where $\mathbf{s}(t) \in \mathbb{R}^n$ is the full system state and $h: \mathbb{R}^n \to \mathbb{R}$ is the observation function. From this scalar record, can you reconstruct the full attractor of the system?

The answer, due to Takens [Takens1981], is yes — generically, under mild smoothness conditions.

## The Delay Embedding Construction

**Definition (Delay Embedding).** Given a scalar time series $y(t)$, the delay embedding of dimension $d$ with delay $\tau$ is:

$$\mathbf{Y}(t) = \left[y(t), y(t-\tau), y(t-2\tau), \ldots, y(t-(d-1)\tau)\right]^\top \in \mathbb{R}^d$$

The map $\Phi: \mathbf{s}(t) \mapsto \mathbf{Y}(t)$ sends points on the attractor in the original state space to points in the delay space $\mathbb{R}^d$.

**Takens' Embedding Theorem (1981, simplified statement).** Let $M$ be a compact smooth manifold of dimension $n$ and $\phi: M \to M$ be a smooth diffeomorphism with attractor $A \subseteq M$ of box-counting dimension $d_A$. Let $h: M \to \mathbb{R}$ be a smooth observation function. If the embedding dimension satisfies:

$$d \geq 2d_A + 1$$

then for a **generic** choice of $h$ and $\phi$, the delay embedding map $\Phi: M \to \mathbb{R}^d$ is an **embedding** — a diffeomorphism onto its image. That is, the delay space reconstruction $\{\mathbf{Y}(t)\}$ is topologically equivalent to the original attractor $A$.

**Proof sketch.** The argument has two steps.

*Step 1 (Sard's theorem application).* We need to show that $\Phi$ is injective (no two distinct states map to the same delay vector) and that its derivative is non-degenerate. By the smooth genericity version of Sard's theorem, the set of observation functions $h$ for which $\Phi$ fails to be an embedding has measure zero in the space of all smooth observation functions. Hence "generic" choices of $h$ succeed.

*Step 2 (Dimension count).* The attractor $A$ is a $d_A$-dimensional set. Its image under the non-embedding map $\Phi$ in $\mathbb{R}^d$ could have collisions (two attractor points mapping to the same reconstruction point) only if $d < 2d_A + 1$. The Whitney embedding theorem guarantees that a $d_A$-dimensional manifold can be embedded in $\mathbb{R}^{2d_A+1}$. So $d \geq 2d_A + 1$ is sufficient to avoid collisions generically.

The combination: for $d \geq 2d_A + 1$ and generic $h$, the map $\Phi$ is an embedding. $\square$

## Quantitative Version: Attractor Dimension

For the Lorenz system with standard parameters $(\sigma = 10, \rho = 28, \beta = 8/3)$, the attractor dimension is approximately $d_A \approx 2.05$ [Grassberger1983]. Takens' theorem guarantees that a delay embedding of dimension $d \geq 2 \times 2.05 + 1 = 5.1$, i.e., $d \geq 6$, is generically an embedding.

In practice, embeddings of dimension $d = 3$ (using all three Lorenz variables) or even $d = 2$ (using two variables) work well, because the theorem gives a sufficient condition, not a tight lower bound. The actual minimum embedding dimension depends on the observation function $h$ and can be much lower than $2d_A + 1$.

## Connection to Reservoir Computing

The reservoir state at time $t$ is:

$$\mathbf{x}_t = F(\mathbf{x}_{t-1}, \mathbf{u}_t; W^{rec}, W^{in}) = F^{(k)}(\mathbf{u}_t, \mathbf{u}_{t-1}, \ldots, \mathbf{u}_{t-k+1})$$

for some effective depth $k$ set by the leaking rate and spectral radius. This is a **nonlinear observation of the input history** — precisely the kind of object that Takens' theorem applies to, but in a much richer setting:

1. **Multi-observation:** The reservoir provides an $N$-dimensional observation $\mathbf{x}_t$, not just a scalar. This gives more information and is much more than the minimum required by Takens' theorem.

2. **Nonlinear history function:** The reservoir does not form a simple delay embedding; it nonlinearly mixes the past inputs. This is a **generalized embedding**: a mapping from input history to a high-dimensional space that is (generically) injective on the attractor.

3. **Redundancy and robustness:** With $N \gg 2d_A + 1$ (typical: $N = 100$–$1000$, $d_A = 2$–$10$), the reservoir state over-embeds the attractor — giving the linear readout many redundant "views" of the attractor geometry to work with.

**Theorem (informal, connection to RC).** If the reservoir satisfies the echo state property, and if the input $\mathbf{u}_t = h(\mathbf{s}_t)$ is a smooth observation of a compact attractor $A$ of dimension $d_A$, then for $N \geq 2d_A + 1$ and generic $W^{rec}$, $W^{in}$, the reservoir state $\mathbf{x}_t$ constitutes a generalized embedding of $A$.

**Implication.** The linear readout $\hat{\mathbf{s}}_t = W^{out} \mathbf{x}_t$ can reconstruct the full attractor from the reservoir state, because the reservoir state is (generically) a diffeomorphic copy of the attractor. In particular, the future trajectory — which is determined by the current attractor state — is determined by the current reservoir state, making the prediction problem well-posed.

## Mañé's Theorem and the Practical Dimension Requirement

Takens' theorem has been generalized to handle multi-channel observations, noisy measurements, and sampled (discrete-time) systems. The most practically relevant generalization, due to Mañé [Mañé1981], is:

**Theorem (Mañé 1981).** If $d > 2d_A$, then the delay embedding is generically an embedding, without the "+1" correction of Takens.

For the Lorenz system ($d_A \approx 2.05$), Mañé's theorem requires only $d > 4.1$, i.e., $d \geq 5$.

**In the reservoir context:** With $N = 100$ reservoir neurons and $d_A \approx 2$ for Lorenz, the reservoir state is massively over-embedded. The extra dimensions do not hurt (they give the readout more information) but they do introduce estimation noise (too many features for the training set), which is why regularization (ridge regression) is essential.

---

## References

- [Takens1981] Takens, F. (1981). Detecting strange attractors in turbulence. In *Dynamical Systems and Turbulence, Warwick 1980*. Lecture Notes in Mathematics, 898. Springer. 366–381.
- [Mañé1981] Mañé, R. (1981). On the dimension of the compact invariant sets of certain nonlinear maps. In *Dynamical Systems and Turbulence*. Lecture Notes in Mathematics, 898. Springer. 230–242.
- [Grassberger1983] Grassberger, P. & Procaccia, I. (1983). Characterization of strange attractors. *Physical Review Letters*, 50(5), 346–349.
- [Pathak2018] Pathak, J., Hunt, B., Girvan, M., Lu, Z., & Ott, E. (2018). Model-free prediction of large spatiotemporally chaotic systems from data: A reservoir computing approach. *Physical Review Letters*, 120(2), 024102.
- [Sauer1991] Sauer, T., Yorke, J.A., & Casdagli, M. (1991). Embedology. *Journal of Statistical Physics*, 65(3–4), 579–616.
