# Subsection 12.3.1: SVD and Neural Network Weights

## Orientation

An MZI mesh implements unitary matrices: $U^\dagger U = I$, all singular values equal to 1, total optical power conserved. A trained neural network weight matrix $W$ satisfies none of these properties. It may be rectangular ($M \times N$ with $M \neq N$), it amplifies some input directions and suppresses others, and its entries are whatever gradient descent found. This subsection shows that the gap between unitary optics and arbitrary linear algebra is closed by one theorem of matrix analysis — the singular value decomposition — and quantifies what the closure costs in hardware and in optical power.

---

## 12.3.1.1 The Decomposition

Any complex $M \times N$ matrix $W$ can be factored as

$$W = U \Sigma V^\dagger$$

where $U$ is an $M \times M$ unitary matrix, $V$ is an $N \times N$ unitary matrix, and $\Sigma$ is an $M \times N$ diagonal matrix with non-negative entries $\sigma_1 \geq \sigma_2 \geq \cdots \geq \sigma_{\min(M,N)} \geq 0$, the **singular values**. For a real matrix, $U$ and $V$ can be taken real (orthogonal). The decomposition always exists; it is the workhorse factorization of numerical linear algebra.

The geometric reading is the useful one for photonics. Any linear map is: (1) a rotation of the input space into a special basis (the right singular vectors, columns of $V$), (2) a pure scaling of each basis direction by $\sigma_i$, and (3) a rotation into the output basis (the left singular vectors, columns of $U$). Rotations are what MZI meshes do; per-mode scaling is what amplitude modulators do. The optical implementation is therefore a literal transcription of the theorem:

$$\mathbf{x} \;\rightarrow\; \boxed{V^\dagger \text{ mesh}} \;\rightarrow\; \boxed{\Sigma \text{ modulator column}} \;\rightarrow\; \boxed{U \text{ mesh}} \;\rightarrow\; \mathbf{y} = W\mathbf{x}$$

The $\Sigma$ stage is realized as one amplitude modulator per mode. In practice each is itself a small MZI operated as a variable coupler: setting the internal phase $\theta$ routes a fraction $\cos^2(\theta/2)$ of the power to the "keep" port and dumps the rest to a terminated port. The field amplitude transmission is $\cos(\theta/2)$, so mode $i$ is scaled by $\sigma_i$ when

$$\theta_i = 2\arccos(\sigma_i)$$

**Parameter counting.** A complex $M \times N$ matrix has $2MN$ real degrees of freedom. The SVD hardware provides: $M^2$ phases in the $U$ mesh (an $M$-mode Clements mesh has $M(M-1)$ internal/external phases plus $M$ output phase shifters), $N^2$ phases in the $V^\dagger$ mesh, and $\min(M,N)$ modulator settings. For $M = N$ this is $\approx 2N^2 + N$ parameters controlling $2N^2$ matrix degrees of freedom — a slight overparameterization (global phases are redundant), confirming that the architecture is exactly expressive enough, with nothing wasted.

---

## 12.3.1.2 Non-Unitarity Is Optical Loss

A passive photonic circuit cannot amplify: every physically realizable field transmission has magnitude $\leq 1$. The singular values must therefore be rescaled before programming:

$$\hat{\Sigma} = \Sigma / \sigma_{\max}, \qquad W = \sigma_{\max} \, U \hat{\Sigma} V^\dagger$$

The scalar $\sigma_{\max}$ is applied electronically — as gain in the transimpedance amplifier or as a digital multiplication after readout. Optically, the chip implements $W/\sigma_{\max}$, whose largest singular value is exactly 1.

This has a consequence that is easy to state and important to internalize: **the optical power lost in the $\Sigma$ stage equals the degree to which $W$ is non-unitary.** If the input vector happens to align with the principal singular direction, transmission is unity. Averaged over random input directions, the power transmission is

$$\bar{T} = \frac{1}{N}\sum_{i=1}^{N} \left(\frac{\sigma_i}{\sigma_{\max}}\right)^2 = \frac{\|W\|_F^2}{N\,\sigma_{\max}^2}$$

For a well-conditioned matrix ($\sigma_{\min} \approx \sigma_{\max}$), $\bar{T} \approx 1$ and almost no light is wasted. For an ill-conditioned matrix with condition number $\kappa = \sigma_{\max}/\sigma_{\min} \gg 1$, most of the input power is deliberately dumped, and the surviving signal approaches the shot-noise and detector-noise floors analyzed in Section 12.1. A matrix with $\kappa = 100$ processed at fixed laser power sacrifices up to 40 dB of signal on its weakest singular direction — roughly 6–7 bits of the ENOB budget. Analog photonic linear algebra is therefore *condition-number sensitive* in a way that floating-point digital arithmetic (with its enormous dynamic range) is not. Fortunately, trained neural network layers tend to be reasonably well conditioned, in part because the regularization and normalization schemes used in training (weight decay, batch normalization) implicitly penalize extreme singular value spread.

---

## 12.3.1.3 Rank Truncation: Compression You Can Fabricate

The Eckart–Young theorem states that the best rank-$r$ approximation of $W$ (in either the Frobenius or spectral norm) is obtained by keeping the $r$ largest singular values and discarding the rest:

$$W_r = \sum_{i=1}^{r} \sigma_i \, \mathbf{u}_i \mathbf{v}_i^\dagger, \qquad \|W - W_r\|_F^2 = \sum_{i>r} \sigma_i^2$$

Deep network weight matrices are empirically close to low rank — a large fraction of their Frobenius norm is concentrated in the leading singular values. This is a gift to the hardware designer. A rank-$r$ factorization $W_r = (U\hat{\Sigma}^{1/2})(\hat{\Sigma}^{1/2}V^\dagger)$ can be implemented as an $N \times r$ mesh, an $r$-element modulator column, and an $r \times M$ mesh. The MZI count drops from $O(N^2)$ to $O(Nr)$, and — because mesh depth also shrinks — the accumulated insertion loss drops proportionally. At a typical 0.25 dB insertion loss per MZI stage, reducing the depth of a 64-mode Clements mesh (64 stages, $\sim$16 dB) to a rank-16 factorization ($\sim$4 dB per mesh) recovers roughly 8 dB of optical budget, worth more than one bit of shot-noise-limited precision.

An alternative to truncating a pre-trained matrix is to train *in the decomposed parameterization from the start*: treat the mesh phases $\{\theta_k, \phi_k\}$ and the singular values $\{\sigma_i\}$ as the learnable parameters and backpropagate through the (perfectly differentiable) product of MZI transfer matrices. This guarantees that the learned weights are exactly representable by the hardware — there is no post-hoc decomposition error — and it is the approach taken by the `neurophox` software framework and by several of the training strategies in Chapter 13.

---

## 12.3.1.4 Worked Example: Programming a $2 \times 2$ Matrix

Take the real symmetric matrix

$$W = \begin{pmatrix} 2 & 1 \\ 1 & 2 \end{pmatrix}$$

Its eigendecomposition is its SVD (symmetric positive-definite case): eigenvalues 3 and 1 with orthonormal eigenvectors $(1,1)/\sqrt{2}$ and $(1,-1)/\sqrt{2}$. Thus $\sigma_1 = 3$, $\sigma_2 = 1$, and

$$U = V = R(45°) = \frac{1}{\sqrt{2}}\begin{pmatrix} 1 & -1 \\ 1 & 1 \end{pmatrix}$$

**Step 1 — normalize:** $\sigma_{\max} = 3$, so program $\hat{\Sigma} = \mathrm{diag}(1, 1/3)$ and record the electronic post-gain factor 3.

**Step 2 — $V^\dagger$ mesh:** a single MZI configured as a $-45°$ rotation. Using the MZI parameterization of Section 12.2.1, a rotation by angle $\alpha$ requires internal phase $\theta = 2\alpha$ (splitting amplitude $\cos\alpha$), here $\theta_V = \pi/2$, with external phase $\phi_V$ chosen to make the matrix real.

**Step 3 — $\Sigma$ stage:** mode 1 passes untouched ($\theta = 0$). Mode 2 must be attenuated to field amplitude $1/3$: $\theta_2 = 2\arccos(1/3) = 2.462$ rad. The dumped power in mode 2 is $1 - 1/9 = 89\%$ — the price of $\kappa = 3$.

**Step 4 — $U$ mesh:** a single MZI as a $+45°$ rotation, $\theta_U = \pi/2$.

**Verification:** $\sigma_{\max} U \hat{\Sigma} V^\dagger = 3 \cdot R(45°)\,\mathrm{diag}(1, \tfrac{1}{3})\,R(-45°) = \begin{pmatrix} 2 & 1 \\ 1 & 2 \end{pmatrix}$ ✓

Three MZIs, five phase settings, one electronic gain constant: an arbitrary $2\times2$ real matrix. The same recipe, executed by the Clements algorithm instead of by hand, scales to any $N$.

---

## References

[1] Reck, M., Zeilinger, A., Bernstein, H.J., & Bertani, P. (1994). "Experimental realization of any discrete unitary operator." *Physical Review Letters*, 73, 58–61. [The unitary decomposition underlying both meshes in the SVD architecture.]

[2] Clements, W.R., Humphreys, P.C., Metcalf, B.J., Kolthammer, W.S., & Walmsley, I.A. (2016). "Optimal design for universal multiport interferometers." *Optica*, 3(12), 1460–1465. [The rectangular mesh used in essentially all modern SVD implementations.]

[3] Miller, D.A.B. (2013). "Self-configuring universal linear optical component." *Photonics Research*, 1(1), 1–15. [Shows that arbitrary linear maps — including the SVD architecture — can be configured progressively using only local feedback, without global calculation; conceptual foundation for self-aligning meshes.]

[4] Shen, Y., et al. (2017). "Deep learning with coherent nanophotonic circuits." *Nature Photonics*, 11, 441–446. [Proposed and demonstrated the mesh–attenuator–mesh SVD architecture for neural network layers; subject of the next subsection.]
