# Chapter 10 Exercises

## Analytical Exercises

**Exercise 10.1 (Bayesian ridge regression as GP).**
Show explicitly that Bayesian linear regression with prior $\mathbf{w} \sim \mathcal{N}(\mathbf{0}, \sigma_w^2 I)$ and likelihood $y_t \mid \mathbf{r}_t, \mathbf{w} \sim \mathcal{N}(\mathbf{w}^\top\mathbf{r}_t, \sigma_n^2)$ is equivalent to GP regression with the linear kernel $k(\mathbf{r}, \mathbf{r}') = \sigma_w^2 \mathbf{r}^\top\mathbf{r}'$.

(a) Compute the posterior $p(\mathbf{w} \mid \mathbf{y}, R)$ and show it is Gaussian. Give the posterior mean and covariance.

(b) Show the predictive distribution $p(y^* \mid \mathbf{r}^*, \mathbf{y}, R)$ has mean $\mathbf{r}^{*\top}(R^\top R + \lambda I)^{-1}R^\top\mathbf{y}$ where $\lambda = \sigma_n^2/\sigma_w^2$.

(c) Show the predictive variance is $\sigma_n^2 + \sigma_w^2 \mathbf{r}^{*\top}(R^\top R/\sigma_n^2 + I/\sigma_w^2)^{-1}\mathbf{r}^*/\sigma_n^2$.

(d) Verify that the predictive mean equals the ridge regression estimate. Show that the predictive variance is larger than $\sigma_n^2$ and depends on the "distance" of $\mathbf{r}^*$ from the training data.

---

**Exercise 10.2 (GP marginal likelihood and Occam's razor).**
The log marginal likelihood for the GP with kernel $K$ is $\mathcal{L} = -\frac{1}{2}\mathbf{y}^\top C^{-1}\mathbf{y} - \frac{1}{2}\log\det C - \frac{T}{2}\log(2\pi)$, where $C = K + \sigma_n^2 I$.

(a) Show that $-\frac{1}{2}\mathbf{y}^\top C^{-1}\mathbf{y}$ is the "data fit" term and $-\frac{1}{2}\log\det C$ is the "complexity penalty."

(b) For the linear kernel $K = \sigma_w^2 RR^\top$, show that as $\sigma_w^2 \to \infty$ (infinite prior variance, minimal regularization), the complexity penalty grows without bound, counterbalancing the improving data fit. This is the automatic regularization of the GP.

(c) Let $C = \sigma_n^2(I + \text{SNR} \cdot P)$ where $P$ is the projection onto the column space of $R$ and $\text{SNR} = \sigma_w^2 / \sigma_n^2$. Show that $\log\det C = (N-T)\log\sigma_n^2 + T\log(\sigma_n^2 + \sigma_w^2 \sigma_{data}^2)$ approximately (for rank-$T$ data matrix), and interpret the behavior as $\text{SNR} \to \infty$.

(d) Numerically: for a reservoir with $N = 10$, $T = 50$, compute the marginal likelihood as a function of $\sigma_w$ with $\sigma_n = 0.01$. Where is the maximum? How does the optimal $\sigma_w$ relate to the ridge parameter $\lambda^* = \sigma_n^2/\sigma_w^2$?

---

**Exercise 10.3 (ARD as feature selection).**
Consider a GP with ARD linear kernel $k(\mathbf{r}, \mathbf{r}') = \sum_i r_i r_i'/l_i^2$. Show that:

(a) The length scale $l_i \to \infty$ for feature $i$ makes neuron $i$ irrelevant: the kernel becomes independent of $r_i$ and $r_i'$.

(b) The optimal length scales (maximizing marginal likelihood) satisfy a balance between signal variance in direction $i$ and noise variance: roughly $l_i^{-2} \propto \text{SNR}_i$ where $\text{SNR}_i$ is the signal-to-noise ratio of the projection of the target onto neuron $i$'s activation direction.

(c) Derive the gradient $\partial \mathcal{L}/\partial l_i$ for the ARD linear kernel and show it equals zero when $l_i^{-2} = (\alpha_i^2 - (C^{-1})_{ii}) \cdot \hat{l}_i^4$ where $\alpha_i$ is the $i$-th component of $C^{-1}\mathbf{y}$ weighted by the $i$-th column of the kernel matrix. (Hint: use the formula for the gradient of the marginal likelihood given in the section.)

---

## Thought Experiments

**Thought Experiment 10.1: When Does Uncertainty Matter?**

(a) Consider a reservoir-based controller for a robotic arm. The readout predicts the next joint angle. In which scenarios does the uncertainty quantification (predictive variance) from a GP readout provide actionable information that ridge regression cannot?

(b) Now consider anomaly detection: the GP readout flags a sample as anomalous when the predictive variance exceeds a threshold. When would this anomaly detector fail? (Hint: consider what happens when anomalous states are similar to training states in kernel space.)

(c) There is a known failure mode of GP uncertainty quantification: a GP can be highly confident (${\sigma^*}^2 \approx 0$) at a test point far from all training data, if the test point happens to lie in the span of the training states in kernel space. For the reservoir GP readout, how does this manifest? What property of the reservoir states would cause this?

---

**Thought Experiment 10.2: Nonlinear Readouts and the Reservoir Computing Paradigm.**

The "rule" of reservoir computing is: fixed random reservoir, linear readout. Using a nonlinear readout (like a GP with a nonlinear kernel, or a small neural network) blurs the boundary.

(a) If you use a deep neural network as the readout, are you still doing "reservoir computing"? What has been lost (if anything) from the original paradigm?

(b) The GP with RBF kernel is a nonlinear readout. But it is also a principled probabilistic model. Does the probabilistic interpretation justify using a nonlinear readout, or is it just a soft excuse for adding complexity?

(c) Consider the following argument: "If the reservoir is doing its job, all relevant information is already in a linearly accessible form in the reservoir states. A nonlinear readout means the reservoir failed, not that we need a better readout." Evaluate this argument. When is it true? When is it false?

---

## Key Concepts

**1. Gaussian Process (GP)**
A distribution over functions, fully specified by a mean function and a covariance (kernel) function. Any finite collection of function values is jointly Gaussian. GPs are the canonical Bayesian nonparametric model for regression.

**2. Predictive Distribution**
The posterior distribution over the output at a test point, given the training data: $p(y^* \mid \mathbf{r}^*, \mathbf{y}, R)$. For a GP, this is Gaussian with analytically computable mean (predictive mean) and variance (predictive variance).

**3. Marginal Likelihood (Log Evidence)**
The marginal probability of the training targets $p(\mathbf{y} \mid R, \boldsymbol\theta)$ integrated over the GP prior. Maximizing this over GP hyperparameters $\boldsymbol\theta$ provides a principled way to set regularization without cross-validation.

**4. Automatic Relevance Determination (ARD)**
A per-dimension length scale in the ARD kernel $k(\mathbf{r}, \mathbf{r}') = \sigma_f^2 \exp(-\frac{1}{2}\sum_i(r_i-r_i')^2/l_i^2)$. After optimization of $\{l_i\}$ via marginal likelihood maximization, neurons with large $l_i$ are identified as irrelevant to the task. ARD provides principled, automatic feature selection.

**5. Matrix Inversion Lemma (Woodbury Identity)**
The identity $(A + UCV)^{-1} = A^{-1} - A^{-1}U(C^{-1}+VA^{-1}U)^{-1}VA^{-1}$. Critical for switching between the "feature space" view (operating in $\mathbb{R}^N$ with $N \times N$ matrices) and the "kernel space" view (operating in $\mathbb{R}^T$ with $T \times T$ matrices). Allows GP inference to be performed in whichever space is smaller.

**6. Bayesian Ridge Regression**
Bayesian linear regression with a Gaussian prior on the weight vector, equivalent to GP regression with a linear kernel. Provides the posterior distribution over weights in addition to the MAP (ridge regression) estimate. The posterior mean is the ridge regression solution; the posterior variance provides per-prediction uncertainty.

**7. Sparse GP**
An approximation to the full GP that reduces the computational cost from $O(T^3)$ to $O(TM^2)$ by using $M \ll T$ inducing points. The inducing points are selected or optimized to summarize the training data. Essential for large-scale reservoir computing applications.

**8. Calibrated Uncertainty**
A predictive distribution is calibrated if, e.g., 90% of the true values fall within the predicted 90% credible interval. Calibrated uncertainty is essential for safety-critical applications. GPs are known to produce calibrated uncertainty for well-specified models; miscalibration occurs when the kernel is misspecified or hyperparameters are poorly estimated.

**9. Occam's Razor in GPs**
The automatic complexity control provided by the GP marginal likelihood: models that are more complex (larger prior variance, smaller length scales) are penalized by the log-determinant term in the marginal likelihood, even if they fit the training data better. This prevents overfitting without explicit regularization.

**10. Linear Readout Limitations**
The linear readout captures only linear functions of the reservoir state. Nonlinear relationships between states and outputs are missed. In high-dimensional state spaces ($N$ large), most tasks of interest can be linearized in the state space by choosing the reservoir correctly, but for small $N$ or tasks with extreme nonlinearity, a nonlinear readout (GP with nonlinear kernel, or otherwise) may be necessary.

---

## Key Researchers

**Carl Rasmussen and Christopher Williams.** Authors of *Gaussian Processes for Machine Learning* [Rasmussen2006], the definitive reference on GP regression. Their GP framework is the standard toolkit used for GP readouts in reservoir computing.

**Neil Lawrence.** Lawrence developed the sparse GP framework [Lawrence2007] and the connection between GPs and probabilistic PCA, which provides a path to efficient GP readouts for large-scale reservoir experiments.

**Michiel Hermans and Benjamin Schrauwen.** Applied GP readouts to reservoir computing in a systematic study [HermansSchrauwen2012], demonstrating the uncertainty quantification and ARD benefits in practice.

---

## Further Reading

**Rasmussen, C. E., & Williams, C. K. I. (2006). *Gaussian Processes for Machine Learning*. MIT Press.**
[Rasmussen2006]
The standard reference. Chapters 2 and 5 cover regression and kernel design; freely available at gaussianprocess.org/gpml/.

**Quinonero-Candela, J., & Rasmussen, C. E. (2005). A unifying view of sparse approximate Gaussian process regression. *Journal of Machine Learning Research*, 6, 1939–1959.**
[QuinoneroCandela2005]
Unifies sparse GP approximations. Important for scaling GP readouts to large training sets.

**Hermans, M., & Schrauwen, B. (2012). Reservoirs are universal approximators. In *Advances in Neural Information Processing Systems 25*.**
[HermansSchrauwen2012]
Applies GP analysis to reservoir states and discusses the universality of nonlinear readouts on reservoir states.
