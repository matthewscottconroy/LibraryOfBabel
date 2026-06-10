# Appendix G: Symbol Glossary

This appendix provides a complete reference for all mathematical symbols used in the book. Symbols are organized by category. Where a symbol has both a standard mathematical meaning and a specific RC meaning, both are listed.

---

## G.1 Matrices

| Symbol | Dimensions | Meaning | First used |
|---|---|---|---|
| $W_{\text{res}}$ or $W$ | $N \times N$ | Reservoir recurrent weight matrix | Ch. 2 |
| $W_{\text{in}}$ | $N \times d_{\text{in}}$ | Input weight matrix | Ch. 2 |
| $W_{\text{out}}$ | $d_{\text{out}} \times N$ | Output (readout) weight matrix | Ch. 2 |
| $W_{\text{fb}}$ | $N \times d_{\text{out}}$ | Feedback weight matrix (output to reservoir) | Ch. 5 |
| $X$ | $N \times T$ or $T \times N$ | Reservoir state matrix (states as columns or rows) | Ch. 3 |
| $U$ | $d_{\text{in}} \times T$ | Input matrix | Ch. 3 |
| $Y$ | $d_{\text{out}} \times T$ | Target/output matrix | Ch. 3 |
| $A$ | $n \times n$ | Generic square matrix | App. A |
| $\Sigma$ | $m \times n$ | Diagonal matrix of singular values (in SVD) | App. A |
| $U_{\text{SVD}}$ | $m \times m$ | Left singular vector matrix | App. A |
| $V_{\text{SVD}}$ | $n \times n$ | Right singular vector matrix | App. A |
| $\Lambda$ | $n \times n$ | Diagonal eigenvalue matrix | App. A |
| $\Sigma_{\text{cov}}$ | $d \times d$ | Covariance matrix (context: probability) | App. B |
| $\Lambda_N$ | $N \times N$ | Posterior precision matrix (Bayesian regression) | App. B |
| $H$ | $T \times T$ | Hat matrix (projection matrix, ridge regression) | App. B |
| $P$ | $N \times N$ | Inverse covariance matrix (RLS algorithm) | Ch. 9 |
| $M$ | $n \times n$ | Mass matrix (mechanical systems) | Ch. 18 |
| $K$ | $n \times n$ | Stiffness matrix (mechanical systems) | Ch. 18 |
| $D$ | $n \times n$ | Damping matrix (mechanical systems) | Ch. 18 |
| $\rho$ | $d \times d$ | Density matrix (quantum systems) | Ch. 19 |
| $I_n$ or $I$ | $n \times n$ | Identity matrix (subscript omitted when clear) | Throughout |

---

## G.2 Vectors

| Symbol | Dimensions | Meaning | First used |
|---|---|---|---|
| $\mathbf{x}(t)$ | $N \times 1$ | Reservoir state vector at time $t$ | Ch. 2 |
| $\mathbf{u}(t)$ | $d_{\text{in}} \times 1$ | Input vector at time $t$ | Ch. 2 |
| $\mathbf{y}(t)$ | $d_{\text{out}} \times 1$ | Output vector at time $t$ | Ch. 2 |
| $\mathbf{t}(t)$ | $d_{\text{out}} \times 1$ | Target vector at time $t$ | Ch. 3 |
| $\mathbf{w}$ | $N \times 1$ | Weight vector (single-output readout) | Ch. 3 |
| $\mathbf{v}_i$ | $N \times 1$ | $i$-th eigenvector of a matrix | App. A |
| $\mathbf{u}_i$ | $m \times 1$ | $i$-th left singular vector | App. A |
| $\mathbf{b}$ | $N \times 1$ | Bias vector | Ch. 5 |
| $\boldsymbol{\mu}$ | $d \times 1$ | Mean vector (probability context) | App. B |
| $\mathbf{m}_N$ | $N \times 1$ | Posterior mean vector (Bayesian regression) | App. B |
| $\mathbf{m}$ | $3 \times 1$ | Magnetization unit vector (spintronics) | Ch. 19 |
| $\boldsymbol{\theta}$ | $N \times 1$ | Joint angle vector (mechanical systems) | Ch. 18 |
| $\mathbf{r}$ | $2 \times 1$ or $3 \times 1$ | Position vector (skyrmion or robot) | Ch. 18–19 |
| $\mathbf{e}_k$ | $V \times 1$ | One-hot vector for class/word $k$ | Ch. 25 |
| $\hat{\mathbf{p}}$ | $V \times 1$ | Predicted probability vector (softmax output) | Ch. 25 |

---

## G.3 Scalars

| Symbol | Meaning | Typical range / units | First used |
|---|---|---|---|
| $N$ | Number of reservoir neurons | 10–10,000 | Ch. 2 |
| $d_{\text{in}}$ | Input dimensionality | 1–1000 | Ch. 2 |
| $d_{\text{out}}$ | Output dimensionality | 1–1000 | Ch. 2 |
| $T$ | Number of training timesteps | 100–100,000 | Ch. 2 |
| $\rho$ or $\rho(W)$ | Spectral radius of $W_{\text{res}}$ | 0–2 (typical: 0.8–0.99) | Ch. 2 |
| $\alpha$ | Leaking rate (leaky-integrator ESN) | $(0, 1]$ | Ch. 5 |
| $\lambda$ | Ridge regression regularization | $10^{-8}$–$10^0$ | Ch. 3 |
| $\sigma_{\text{in}}$ or $s_{\text{in}}$ | Input scaling coefficient | 0.01–2 | Ch. 5 |
| $t$ | Discrete time index (integer) | $0, 1, 2, \ldots$ | Throughout |
| $k$ | Lag or delay | positive integer | Ch. 10 |
| $\Delta t$ | Timestep (continuous-time systems) | depends on application | Ch. 17 |
| $\tau$ | Time constant or delay | seconds or steps | Throughout |
| $\tau_{\text{mem}}$ | Memory time constant | seconds or steps | Ch. 18 |
| $\sigma_i$ | $i$-th singular value of a matrix | $\geq 0$ | App. A |
| $\lambda_i$ | $i$-th eigenvalue of a matrix | $\in \mathbb{C}$ | App. A |
| $\kappa$ | Condition number of a matrix | $\geq 1$ | App. C |
| $\mu$ | Mean (probability context) | $\in \mathbb{R}$ | App. B |
| $\sigma^2$ | Variance | $\geq 0$ | App. B |
| $\alpha_{\text{Bayes}}$ | Weight precision (Bayesian regression) | $> 0$ | App. B |
| $\beta_{\text{Bayes}}$ | Noise precision (Bayesian regression) | $> 0$ | App. B |
| $\gamma$ | Discount factor (reinforcement learning) | $(0, 1)$ | Ch. 23 |
| $\eta$ | Learning rate | $10^{-4}$–$10^{-1}$ | Ch. 9 |
| $\omega$ | Angular frequency | rad/s | Ch. 17–19 |
| $f$ | Frequency | Hz | Ch. 17–19 |
| $R_{\text{ON}}$ | On-state resistance (memristor) | $\Omega$ | Ch. 19 |
| $R_{\text{OFF}}$ | Off-state resistance (memristor) | $\Omega$ | Ch. 19 |
| $w$ | Internal state (memristor doped-region width) | $[0, D]$ nm | Ch. 19 |
| $D$ | Memristor film thickness | nm | Ch. 19 |
| $\mu_v$ | Ion mobility (memristor) | cm²/(V·s) | Ch. 19 |
| $\alpha_{\text{Gilbert}}$ | Gilbert damping parameter (spintronics) | 0.01–0.1 (dimensionless) | Ch. 19 |
| $\gamma_{\text{gyro}}$ | Gyromagnetic ratio | $1.76 \times 10^{11}$ rad/(T·s) | Ch. 19 |
| $Q$ | Topological charge (skyrmion) | $\pm 1$ (integer) | Ch. 19 |
| $\text{MC}$ | Memory capacity | $[0, N]$ | Ch. 10 |
| $\text{NMSE}$ | Normalized mean square error | $[0, \infty)$ (0 = perfect) | Ch. 10 |
| $\text{PPL}$ | Perplexity (language modeling) | $[1, V]$ | Ch. 25 |
| $\text{AUROC}$ | Area under ROC curve | $[0, 1]$ (1 = perfect) | Ch. 22 |
| $F_1$ | F1 score | $[0, 1]$ | Ch. 22 |
| $V$ | Vocabulary size (NLP) | 27–100,000 | Ch. 25 |

---

## G.4 Functions and Operators

| Symbol | Meaning | First used |
|---|---|---|
| $\tanh(\cdot)$ | Hyperbolic tangent (standard reservoir activation) | Ch. 2 |
| $\sigma(\cdot)$ | Sigmoid function $\sigma(x) = 1/(1+e^{-x})$ | Ch. 23 |
| $\text{softmax}(\cdot)$ | Softmax function $\text{softmax}(\mathbf{z})_k = e^{z_k}/\sum_j e^{z_j}$ | Ch. 25 |
| $\text{ReLU}(\cdot)$ | Rectified linear unit: $\max(0, x)$ | Ch. 18 |
| $F_t(u(\cdot); \mathbf{x}_0)$ | Reservoir state function at time $t$ given input history | Ch. 4 |
| $\det(\cdot)$ | Matrix determinant | App. A |
| $\text{tr}(\cdot)$ | Matrix trace | App. A |
| $\text{rank}(\cdot)$ | Matrix rank | App. A |
| $A^+$ | Moore-Penrose pseudoinverse of $A$ | App. A |
| $A^{-1}$ | Matrix inverse (assumes square and nonsingular) | Throughout |
| $A^\top$ | Matrix transpose | Throughout |
| $A^*$ | Complex conjugate (or Moore-Penrose, context-dependent) | App. A |
| $\otimes$ | Kronecker product | App. A |
| $\odot$ | Element-wise (Hadamard) product | Ch. 23 |
| $\|\mathbf{x}\|_2$ or $\|\mathbf{x}\|$ | Euclidean (L2) norm of vector $\mathbf{x}$ | Throughout |
| $\|\mathbf{x}\|_1$ | L1 norm of vector $\mathbf{x}$: $\sum_i |x_i|$ | Throughout |
| $\|A\|_2$ | Spectral norm (largest singular value) | App. A |
| $\|A\|_F$ | Frobenius norm | App. A |
| $\|A\|_*$ | Nuclear norm (sum of singular values) | App. A |
| $\text{vec}(\cdot)$ | Column-vectorization operator | App. A |
| $\text{diag}(\cdot)$ | Creates diagonal matrix from vector, or extracts diagonal | Throughout |
| $\mathbb{E}[\cdot]$ | Expectation of a random variable | App. B |
| $\text{Var}(\cdot)$ | Variance of a random variable | App. B |
| $\text{Cov}(\cdot, \cdot)$ | Covariance of two random variables | App. B |
| $\mathbb{1}[\cdot]$ | Indicator function (1 if condition true, 0 otherwise) | Throughout |
| $\arg\max$ | Argument achieving the maximum | Throughout |
| $\arg\min$ | Argument achieving the minimum | Throughout |
| $\nabla_\theta$ | Gradient with respect to parameter $\theta$ | Ch. 23 |
| $\frac{\partial}{\partial x}$ | Partial derivative with respect to $x$ | Throughout |
| $\int_a^b \cdot \, dt$ | Definite integral | Throughout |
| $\sum_{t=1}^T$ | Summation over discrete time index $t$ | Throughout |
| $\prod_{t=1}^T$ | Product over discrete time index $t$ | Ch. 25 |
| $\mathcal{O}(\cdot)$ | Big-O asymptotic notation | Throughout |
| $\lfloor \cdot \rfloor$ | Floor function | Throughout |
| $\lceil \cdot \rceil$ | Ceiling function | Throughout |
| $|\mathcal{S}|$ | Cardinality (number of elements) of set $\mathcal{S}$ | Throughout |

---

## G.5 Probability Distributions

| Symbol | Meaning | Parameters |
|---|---|---|
| $\mathcal{N}(\mu, \sigma^2)$ | Univariate Gaussian | mean $\mu$, variance $\sigma^2$ |
| $\mathcal{N}(\boldsymbol{\mu}, \Sigma)$ | Multivariate Gaussian | mean $\boldsymbol{\mu}$, covariance $\Sigma$ |
| $\mathcal{U}[a, b]$ | Uniform distribution on $[a, b]$ | bounds $a < b$ |
| $\text{Bernoulli}(p)$ | Bernoulli distribution | success probability $p \in [0,1]$ |
| $\text{Categorical}(\mathbf{p})$ | Categorical distribution | probability vector $\mathbf{p}$ |
| $\text{Poisson}(\lambda)$ | Poisson distribution | rate $\lambda > 0$ |
| $p(\mathbf{x})$ | Generic probability density/mass function | context-dependent |
| $p(\mathbf{x} | \mathbf{y})$ | Conditional distribution of $\mathbf{x}$ given $\mathbf{y}$ | context-dependent |

---

## G.6 Subscript and Superscript Conventions

| Convention | Meaning | Example |
|---|---|---|
| Subscript $t$ | Discrete time index | $\mathbf{x}(t)$ or $\mathbf{x}_t$ |
| Subscript $i$ or $j$ | Component index (neuron, input, output) | $x_i(t)$: activation of neuron $i$ |
| Subscript $k$ | Class or category index | $y_k$: probability of class $k$ |
| Subscript $n$ | Sample or sequence index | $(\mathbf{x}_n, y_n)$: $n$-th training pair |
| Superscript $(l)$ | Layer index (deep RC) | $\mathbf{x}^{(l)}$: state of layer $l$ |
| Superscript $\top$ | Transpose | $A^\top$ |
| Superscript $+$ | Moore-Penrose pseudoinverse | $A^+$ |
| Superscript $-1$ | Matrix inverse | $A^{-1}$ |
| Superscript $*$ | Optimal value or complex conjugate | $\mathbf{w}^*$: optimal weights |
| Superscript $\hat{}$ | Estimated/predicted quantity | $\hat{y}(t)$: predicted output |
| Superscript $\tilde{}$ | Normalized or transformed version | $\tilde{\mathbf{x}}$: normalized state |
| Subscript $\text{res}$ or $\text{rec}$ | Reservoir / recurrent | $W_{\text{res}}$: reservoir weights |
| Subscript $\text{in}$ | Input | $W_{\text{in}}$: input weights |
| Subscript $\text{out}$ | Output | $W_{\text{out}}$: output weights |
| Subscript $\text{fb}$ | Feedback | $W_{\text{fb}}$: feedback weights |
| Subscript $\text{train}$ | Training set | $\mathbf{x}_{\text{train}}$ |
| Subscript $\text{test}$ | Test set | $\mathbf{x}_{\text{test}}$ |
| Subscript $\text{eff}$ | Effective (after approximation) | $d_{\text{eff}}$: effective dimension |

---

## G.7 Special Notation and Sets

| Symbol | Meaning |
|---|---|
| $\mathbb{R}$ | Real numbers |
| $\mathbb{C}$ | Complex numbers |
| $\mathbb{Z}$ | Integers |
| $\mathbb{R}^{m \times n}$ | Space of real $m \times n$ matrices |
| $\mathcal{N}$ | Used for both "natural" in linguistics and "normal distribution" — context determines which |
| $[a, b]$ | Closed interval from $a$ to $b$ |
| $(a, b)$ | Open interval from $a$ to $b$ (or ordered pair — context-dependent) |
| $\{0, 1\}^N$ | Binary strings of length $N$ |
| $\{x : P(x)\}$ | Set of all $x$ satisfying predicate $P$ |
| $\mathcal{V}$ | Vocabulary (NLP context) |
| $\mathcal{B}$ | Target behavior (robotics context) |
| $\mathcal{F}$ | Body dynamics (morphological computation context) |
| $\partial \mathcal{D}$ | Boundary of domain $\mathcal{D}$ |
| $\perp$ | Orthogonal / independent |
| $\propto$ | Proportional to |
| $\approx$ | Approximately equal |
| $\triangleq$ | Defined as equal to |
| $\checkmark$ | Verified / confirmed |

---

## G.8 Acronyms

| Acronym | Expansion |
|---|---|
| RC | Reservoir Computing |
| ESN | Echo State Network |
| LSM | Liquid State Machine |
| BPDC | Backpropagation Decorrelation (Steil's method) |
| RLS | Recursive Least Squares |
| FORCE | First Order Reduced and Controlled Error (Sussillo & Abbott) |
| NARMA | Nonlinear Autoregressive Moving Average |
| NMSE | Normalized Mean Square Error |
| MFCC | Mel-Frequency Cepstral Coefficients |
| CMN | Cepstral Mean Normalization |
| HMM | Hidden Markov Model |
| LSTM | Long Short-Term Memory |
| CPG | Central Pattern Generator |
| VPT | Valid Prediction Time |
| SVD | Singular Value Decomposition |
| GCV | Generalized Cross-Validation |
| LOO | Leave-One-Out |
| AUROC | Area Under the Receiver Operating Characteristic Curve |
| SER | Symbol Error Rate |
| SNR | Signal-to-Noise Ratio |
| STNO | Spin-Torque Nano-Oscillator |
| LLG | Landau-Lifshitz-Gilbert |
| STT | Spin-Transfer Torque |
| QRC | Quantum Reservoir Computing |
| PPL | Perplexity |
| PTB | Penn Treebank |
| FSDD | Free Spoken Digit Dataset |
| MC | Memory Capacity |
| DDE | Delay Differential Equation |
| RK4 | 4th-order Runge-Kutta method |
| CG | Conjugate Gradient |
| PSD | Positive Semidefinite |
| SPD | Symmetric Positive Definite |
| RNG | Random Number Generator |
| ES | Evolution Strategies |
| RL | Reinforcement Learning |
| MDP | Markov Decision Process |
| MLP | Multilayer Perceptron |
| GCM | Generalized Convolution Memory |
| DeepESN | Deep Echo State Network |
| STFT | Short-Time Fourier Transform |
| PCA | Principal Component Analysis |
