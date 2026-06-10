# Chapter 29 Exercises

## Section 29.1: Ergodic Theory Basics

**Exercise 29.1.** *(Ergodicity of circle rotation)* Let $T_\alpha: [0,1) \to [0,1)$ be rotation by $\alpha$: $T_\alpha(x) = x + \alpha \pmod 1$.
(a) Show that $T_\alpha$ is ergodic with respect to Lebesgue measure if and only if $\alpha$ is irrational. (Hint: use Fourier analysis — a $T_\alpha$-invariant $L^2$ function must have zero Fourier coefficients at non-zero frequencies when $\alpha$ is irrational.)
(b) Show that $T_\alpha$ is not mixing when $\alpha$ is irrational by computing $\lim_{n \to \infty} \mu(T_\alpha^{-n}([0,1/2)) \cap [0, 1/4))$ and showing it does not equal $\mu([0,1/2))\mu([0,1/4)) = 1/8$.

**Exercise 29.2.** *(von Neumann ergodic theorem)* Prove the $L^2$ version of the ergodic theorem: for $T$ measure-preserving and $f \in L^2(\mu)$, $\frac{1}{n}\sum_{k=0}^{n-1} f \circ T^k \to \mathbb{E}[f|\mathcal{I}]$ in $L^2$ norm, where $\mathcal{I}$ is the $\sigma$-algebra of $T$-invariant sets. (Hint: decompose $L^2$ into the subspace of invariant functions and its orthogonal complement $\{g - g \circ T\}$, and show the Cesàro averages converge to 0 on the complement.)

**Exercise 29.3.** *(Ergodic theorem for reservoir training)* Let an ESN with ESP be driven by an ergodic input process. The training loss is $L(W) = \lim_{T \to \infty} \frac{1}{T}\sum_{t=1}^T \|W x(t) - y(t)\|^2$.
(a) Express $L(W)$ as an ensemble expectation using the ergodic theorem.
(b) Show that the minimizer $W^* = \arg\min_W L(W)$ (the optimal readout weights) is given by $W^* = \mathbb{E}[y(t)x(t)^\top](\mathbb{E}[x(t)x(t)^\top])^{-1}$ (when the latter matrix is invertible).
(c) Under what conditions does the empirical minimizer $\hat{W}^T = \arg\min_W \frac{1}{T}\sum_{t=1}^T \|Wx(t) - y(t)\|^2$ converge to $W^*$ as $T \to \infty$?

**Exercise 29.4.** *(Mixing and fading memory)* Let the input process be $\phi$-mixing with coefficients $\phi(k) \leq Ce^{-\gamma k}$. Let the reservoir have the ESP with exponential contraction rate $\beta < 1$: $\|x_\omega(t) - x_{\omega'}(t)\| \leq C' \beta^t$ whenever $\omega$ and $\omega'$ agree on the last $t$ steps.
(a) Show that the output process $y(t) = W x(t)$ is also $\phi$-mixing, with mixing coefficients $\phi_y(k) \leq C'' \max(e^{-\gamma k}, \beta^k)$.
(b) Why does the mixing condition on the output justify applying the ergodic theorem with a bound on the convergence rate?

## Section 29.3: Pullback Attractors

**Exercise 29.5.** *(Pullback attractor computation)* Consider the scalar non-autonomous system $x(t+1) = a \cdot x(t) + u(t)$ where $|a| < 1$ and $u(t) \in [-M, M]$.
(a) Show that the pullback attractor at time 0, driven by input sequence $(u(0), u(-1), u(-2), \ldots)$, is the single point $x^* = \sum_{k=0}^\infty a^k u(-k)$.
(b) Verify that $x^*$ satisfies the invariance condition $x^*(T\omega) = a \cdot x^*(\omega) + u(\omega)$.
(c) Show that $|x^*(\omega)| \leq M/(1-|a|)$, confirming the state is bounded.

**Exercise 29.6.** *(Non-unique pullback attractor)* Consider the scalar system $x(t+1) = 2 \cdot x(t)(1 - x(t))$ (logistic map with $r=2$) with no input.
(a) Find the fixed points and show the system does NOT have the ESP.
(b) Describe the pullback attractor: is it a single point or a set? What does this mean for reservoir computing?
(c) If we add a small input $u(t) \in [-\delta, \delta]$, for what values of $\delta$ does the system regain the ESP?

**Exercise 29.7.** *(ESP conditions)* For each of the following reservoirs, determine whether the ESP holds. If yes, find the contraction rate. If not, describe the failure mode.
(a) Linear reservoir: $x(t+1) = Wx(t) + u(t)$ with $W = \begin{pmatrix} 0.5 & 0.4 \\ 0 & 0.5 \end{pmatrix}$.
(b) Linear reservoir with $W = \begin{pmatrix} 0 & 2 \\ 0.4 & 0 \end{pmatrix}$ (compute $\rho(W)$ and $\|W\|_{\text{op}}$).
(c) Nonlinear reservoir: $x(t+1) = \tanh(2x(t) + u(t))$ (scalar case).

**Exercise 29.8.** *(Structural stability)* Prove Theorem 29.3.2 in the scalar case: if $x^*(\omega) = \sum_{k=0}^\infty \gamma^k u(-k)$ (from Exercise 29.5) and $x^*(\omega') = \sum_{k=0}^\infty \gamma^k u'(-k)$, where $u(-k) = u'(-k)$ for $0 \leq k \leq K-1$ but $u(-k) \neq u'(-k)$ for $k \geq K$, show $|x^*(\omega) - x^*(\omega')| \leq C\gamma^K$ and find the constant $C$.

## Section 29.4: Skew-Product Systems

**Exercise 29.9.** *(Skew-product ergodicity)* Let $(\Omega, T, \mu)$ be an ergodic system and $\varphi$ a cocycle over it. Show that the skew-product $\Theta(\omega, x) = (T\omega, F(x, u(\omega)))$ is ergodic with respect to the measure $P = \int_\Omega \delta_{x^*(\omega)}\, d\mu(\omega)$ (when the ESP holds), provided $T$ is ergodic. (Hint: a $\Theta$-invariant function must be constant on the graph of $x^*$, and by ergodicity of $T$, constant on $\Omega$.)

**Exercise 29.10.** *(Lyapunov exponents for linear reservoir)* For a linear reservoir $x(t+1) = Wx(t) + W_{\text{in}}u(t)$ with $W$ diagonalizable over $\mathbb{C}$, $W = PDP^{-1}$ where $D = \text{diag}(\lambda_1, \ldots, \lambda_N)$:
(a) Show the Lyapunov exponents are $\log|\lambda_i|$, $i = 1, \ldots, N$.
(b) The ESP holds iff $\max_i \log|\lambda_i| < 0$, i.e., $\rho(W) < 1$. Verify this agrees with Theorem 29.4.4.
(c) For a non-diagonalizable $W$ with a Jordan block $\begin{pmatrix}\lambda & 1 \\ 0 & \lambda\end{pmatrix}$, $|\lambda| < 1$, show the Lyapunov exponent is still $\log|\lambda|$ (not $\log|\lambda| + 0$, despite the non-trivial Jordan structure).

**Exercise 29.11.** *(Edge of chaos)* Simulate an $N = 100$ unit reservoir with tanh activation and i.i.d. Gaussian inputs. Vary the spectral radius $\rho(W)$ from 0.5 to 1.5. For each value, estimate the maximal Lyapunov exponent $\lambda_{\max}$ by simulating two nearby trajectories and measuring their divergence rate. Plot $\lambda_{\max}$ vs. $\rho(W)$. Where does $\lambda_{\max} = 0$? Test reservoir performance on a NARMA-10 task across this range and compare the performance peak to $\lambda_{\max} = 0$.

**Exercise 29.12.** *(Research problem — measurable selection)* The Castaing-Varadarajan measurable selection theorem (Theorem 29.4.3) guarantees the existence of a measurable echo state response even when the ESP fails (when the pullback attractor is not a singleton). However, different measurable selections give different stationary solutions, and hence different reservoir behaviors.
(a) Construct a concrete example of a reservoir with two distinct stationary solutions (i.e., two distinct echo state responses).
(b) Show that both solutions are stable under small perturbations of the input.
(c) Discuss: in this case, can the reservoir still be used for computation? What additional structure or training procedure would be needed to select a unique solution?
