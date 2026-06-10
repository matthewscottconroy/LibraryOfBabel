# Rademacher Complexity and Distribution-Dependent Generalization Bounds

## 28.2.1 Motivation: Beyond Worst-Case Bounds

The VC dimension bound of Section 28.1 holds for any data distribution, but this generality comes at a cost: the bound depends only on the size of the hypothesis class and the number of examples, not on how the specific training data relates to the function class. For the reservoir readout, the VC bound yields sample complexity $O(N/\varepsilon)$, which can be very pessimistic when reservoir states are geometrically well-separated or confined to a subspace.

**Rademacher complexity** [Bartlett & Mendelson 2002] provides distribution-dependent bounds that can be substantially tighter. The key insight is to measure how well functions in $\mathcal{F}$ can correlate with pure noise — if the class cannot fit random labels well (on average over the training set), it cannot overfit the real labels either.

## 28.2.2 Definition and Properties

**Definition 28.4 (Empirical Rademacher Complexity).** Given a function class $\mathcal{F}$, a fixed sample $S = \{x_1, \ldots, x_m\}$, and Rademacher random variables $\sigma_1, \ldots, \sigma_m \overset{\text{i.i.d.}}{\sim} \mathrm{Uniform}(\{-1, +1\})$, the **empirical Rademacher complexity** is

$$
\hat{\mathcal{R}}_m(\mathcal{F}, S) = \mathbb{E}_\sigma\!\left[\sup_{f \in \mathcal{F}} \frac{1}{m} \sum_{i=1}^m \sigma_i f(x_i)\right].
$$

**Definition 28.5 (Rademacher Complexity).** The **Rademacher complexity** of $\mathcal{F}$ with respect to distribution $\mathcal{D}$ is

$$
\mathcal{R}_m(\mathcal{F}) = \mathbb{E}_{S \sim \mathcal{D}^m}\!\left[\hat{\mathcal{R}}_m(\mathcal{F}, S)\right] = \mathbb{E}_{\sigma, S}\!\left[\sup_{f \in \mathcal{F}} \frac{1}{m} \sum_{i=1}^m \sigma_i f(x_i)\right].
$$

Intuitively, $\mathcal{R}_m(\mathcal{F})$ measures the ability of $\mathcal{F}$ to fit random $\pm 1$ labels. A class with large Rademacher complexity can correlate with arbitrary noise; a class with small complexity cannot, and therefore its empirical performance is predictive of true performance.

**Theorem 28.3 (Generalization Bound via Rademacher Complexity [Bartlett & Mendelson 2002]).** For any $f \in \mathcal{F}$ with $f: \mathcal{X} \to [0,1]$ and any $\delta \in (0,1)$, with probability at least $1 - \delta$ over the draw of $m$ i.i.d. training examples from $\mathcal{D}$:

$$
\mathcal{L}_{\mathcal{D}}(f) \leq \hat{\mathcal{L}}_S(f) + 2\mathcal{R}_m(\mathcal{F}) + 3\sqrt{\frac{\ln(2/\delta)}{2m}}.
$$

The generalization gap is bounded by twice the Rademacher complexity plus a confidence term that decays as $O(1/\sqrt{m})$.

*Proof sketch.* The proof proceeds via McDiarmid's concentration inequality applied to the function $\phi(S) = \sup_{f \in \mathcal{F}}[\mathcal{L}_{\mathcal{D}}(f) - \hat{\mathcal{L}}_S(f)]$. A symmetrization argument introduces the Rademacher variables and bounds $\phi(S)$ by $2\mathcal{R}_m(\mathcal{F})$ in expectation; McDiarmid then concentrates around the expectation [Koltchinskii & Panchenko 2002]. $\square$

## 28.2.3 Rademacher Complexity of Linear Classes

For the reservoir readout, the relevant function class is

$$
\mathcal{F} = \left\{ \mathbf{x} \mapsto \mathbf{w}^T \mathbf{x} \;\middle|\; \|\mathbf{w}\|_2 \leq B \right\}.
$$

**Theorem 28.4 (Rademacher Complexity of Bounded Linear Class [Bartlett & Mendelson 2002]).** Let $\mathcal{F} = \{\mathbf{x} \mapsto \mathbf{w}^T \mathbf{x} : \|\mathbf{w}\|_2 \leq B\}$ and $S = \{\mathbf{x}_1, \ldots, \mathbf{x}_m\}$. Then

$$
\hat{\mathcal{R}}_m(\mathcal{F}, S) = \frac{B}{m} \mathbb{E}_\sigma\!\left\|\sum_{i=1}^m \sigma_i \mathbf{x}_i\right\|_2.
$$

*Proof.* By the Cauchy-Schwarz inequality and the definition of the dual norm:

$$
\sup_{\|\mathbf{w}\|_2 \leq B} \frac{1}{m} \sum_{i=1}^m \sigma_i \mathbf{w}^T \mathbf{x}_i = \frac{B}{m} \left\|\sum_{i=1}^m \sigma_i \mathbf{x}_i\right\|_2.
$$

Taking the expectation over $\sigma$ gives the result. $\square$

Applying the Khintchine inequality (for independent Rademacher variables) [Ledoux & Talagrand 1991]:

$$
\mathbb{E}_\sigma\!\left\|\sum_{i=1}^m \sigma_i \mathbf{x}_i\right\|_2 \leq \sqrt{\sum_{i=1}^m \|\mathbf{x}_i\|_2^2}.
$$

Therefore the empirical Rademacher complexity satisfies

$$
\hat{\mathcal{R}}_m(\mathcal{F}, S) \leq \frac{B}{m} \sqrt{\sum_{i=1}^m \|\mathbf{x}_i\|_2^2} = \frac{B}{\sqrt{m}} \sqrt{\frac{1}{m}\sum_{i=1}^m \|\mathbf{x}_i\|_2^2}.
$$

In expectation, as $m \to \infty$, $\frac{1}{m}\sum_{i=1}^m \|\mathbf{x}_i\|_2^2 \to \mathbb{E}[\|\mathbf{x}\|_2^2]$ by the law of large numbers. Hence:

$$
\mathcal{R}_m(\mathcal{F}) \leq \frac{B}{\sqrt{m}} \sqrt{\mathbb{E}[\|\mathbf{x}\|_2^2]} = \frac{B \cdot \mathbb{E}[\|\mathbf{x}\|_2]}{\sqrt{m}}\cdot\frac{\sqrt{\mathbb{E}[\|\mathbf{x}\|_2^2]}}{\mathbb{E}[\|\mathbf{x}\|_2]},
$$

and the last ratio is bounded by a universal constant by Jensen's inequality, giving the clean bound:

$$
\boxed{\mathcal{R}_m(\mathcal{F}) = O\!\left(\frac{B \sqrt{\mathbb{E}[\|\mathbf{x}\|_2^2]}}{\sqrt{m}}\right).}
$$

## 28.2.4 Application to Reservoir Readouts

For an ESN with state $\mathbf{x}(t) \in \mathbb{R}^N$ and readout $f_{\mathbf{w}}(\mathbf{x}) = \mathbf{w}^T \mathbf{x}$, subject to $\|\mathbf{w}\|_2 \leq B$, the Rademacher generalization bound (Theorem 28.3) becomes:

$$
\mathcal{L}_{\mathcal{D}}(f) \leq \hat{\mathcal{L}}_S(f) + \frac{2B}{\sqrt{T}}\sqrt{\frac{1}{T}\sum_{t=1}^T \|\mathbf{x}(t)\|_2^2} + 3\sqrt{\frac{\ln(2/\delta)}{2T}}.
$$

**Key insight.** The generalization bound depends on the *empirical second moment of the reservoir state norm*, not on the number of neurons $N$ directly. This has two important implications:

1. **Norm compression.** If the reservoir dynamics contract the state norm — for example, if the spectral radius $\rho(W^{\text{rec}}) < 1$ and there is no input — then $\|\mathbf{x}(t)\|_2$ is bounded independently of $N$. In this case, the generalization gap is $O(B/\sqrt{T})$ regardless of reservoir size.

2. **Large, sparse reservoirs.** A sparse reservoir with $N = 10{,}000$ neurons but typical state norm $\|\mathbf{x}\|_2 \sim C$ has the same Rademacher complexity as a dense reservoir with $N = 100$ and the same state norm. The VC bound would penalize the large reservoir by a factor of 100; the Rademacher bound does not.

## 28.2.5 Connecting to Ridge Regression

In practice, the readout is trained via ridge regression:

$$
\hat{\mathbf{w}} = \arg\min_{\mathbf{w}} \frac{1}{T}\sum_{t=1}^T (y_t - \mathbf{w}^T \mathbf{x}(t))^2 + \lambda \|\mathbf{w}\|_2^2.
$$

The ridge penalty $\lambda \|\mathbf{w}\|_2^2$ ensures $\|\hat{\mathbf{w}}\|_2 \leq B(\lambda, T)$ where $B(\lambda, T)$ depends on the training data. Substituting into the Rademacher bound yields a bound in terms of $\lambda$ and the state covariance matrix. Specifically, if $\Sigma = \frac{1}{T} \sum_t \mathbf{x}(t)\mathbf{x}(t)^T$ is the empirical state covariance, then:

$$
\|\hat{\mathbf{w}}\|_2^2 \leq \frac{\|\mathbf{y}\|_2^2}{4\lambda T},
$$

where $\mathbf{y} = (y_1, \ldots, y_T)^T$ is the target vector. Smaller $\lambda$ allows larger $\|\hat{\mathbf{w}}\|_2$, increasing the Rademacher complexity and thus the potential for overfitting. This formalizes the well-known rule of thumb that ridge regularization prevents overfitting in reservoir readouts.

## 28.2.6 State Norm Bounds from Reservoir Dynamics

The Rademacher bound requires $\mathbb{E}[\|\mathbf{x}(t)\|_2^2]$ to be finite and preferably small. For an ESN with tanh nonlinearity, $\|x(t)\|_2 \leq \sqrt{N}$ always holds. A tighter bound comes from the fixed-point analysis: if $\rho(W^{\text{rec}}) < 1$, then for bounded inputs $\|u(t)\| \leq U$:

$$
\|\mathbf{x}(t)\|_2 \leq \frac{\sigma_{\max}(W^{\text{in}}) \cdot U}{1 - \rho(W^{\text{rec}})},
$$

where $\sigma_{\max}(W^{\text{in}})$ is the largest singular value of the input weight matrix. This bound is independent of $N$ provided the matrices are normalized appropriately — a key observation for large reservoir design.

## References

- Bartlett, P. L. and Mendelson, S. (2002). Rademacher and Gaussian complexities: Risk bounds and structural results. *Journal of Machine Learning Research*, 3, 463–482.
- Koltchinskii, V. and Panchenko, D. (2002). Empirical margin distributions and bounding the generalization error of combined classifiers. *The Annals of Statistics*, 30(1), 1–50.
- Ledoux, M. and Talagrand, M. (1991). *Probability in Banach Spaces: Isoperimetry and Processes*. Springer, Berlin.
- Shalev-Shwartz, S. and Ben-David, S. (2014). *Understanding Machine Learning: From Theory to Algorithms*. Cambridge University Press.
