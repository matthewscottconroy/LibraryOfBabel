# Chapter 29: Key Concepts

**Measure-Preserving Transformation.** A map $T: (\Omega, \mu) \to (\Omega, \mu)$ that preserves the probability measure: $\mu(T^{-1}A) = \mu(A)$ for all measurable $A$. The mathematical model for a deterministic dynamical system with a natural invariant distribution, or equivalently a stationary stochastic process.

**Ergodicity.** A measure-preserving transformation $T$ is ergodic if every $T$-invariant set has measure 0 or 1. The system cannot be decomposed into two independent invariant subsystems. Ergodicity guarantees that time averages converge to ensemble averages (Birkhoff's theorem).

**Mixing.** A stronger property than ergodicity: $\mu(T^{-n}A \cap B) \to \mu(A)\mu(B)$ as $n \to \infty$. Events far apart in time become statistically independent. For reservoir computing, mixing of the input process enables the ergodic theorem to apply with quantitative convergence rates.

**Birkhoff's Ergodic Theorem.** For a measure-preserving, ergodic $T$ and $f \in L^1(\mu)$: $\frac{1}{n}\sum_{k=0}^{n-1} f(T^k\omega) \to \int f\, d\mu$ almost surely. The mathematical foundation for why training a reservoir on a long time series converges to the population optimum.

**Maximal Ergodic Lemma.** $\int_{\{f^* > 0\}} f\, d\mu \geq 0$ where $f^* = \sup_n A_n f$. The key technical lemma in the proof of Birkhoff's theorem, analogous to the rising sun lemma in harmonic analysis.

**Non-Autonomous Dynamical System.** A system $x(t+1) = F(x(t), u(t))$ where the map $F$ depends on an external input $u(t)$. Classical attractor theory does not directly apply; one needs the pullback attractor framework.

**Cocycle.** A map $\varphi: \mathbb{Z}_+ \times \Omega \times \mathcal{X} \to \mathcal{X}$ satisfying $\varphi(m+n, \omega, x) = \varphi(m, T^n\omega, \varphi(n,\omega,x))$. The abstract formulation of a reservoir driven by an input process $\omega$. The cocycle property encodes the composition law: evolving for $m+n$ steps equals $n$ steps then $m$ steps with the shifted input.

**Pullback Attractor.** A family of compact sets $A(\omega)$ that: (1) is invariant ($\varphi(1,\omega,A(\omega)) = A(T\omega)$), and (2) pullback-attracts all bounded sets: $\text{dist}(\varphi(t, T^{-t}\omega, B), A(\omega)) \to 0$. The pullback attractor captures the long-run state of the reservoir when driven by a fixed input sequence since $-\infty$.

**Echo State Property (ESP).** The reservoir's pullback attractor is a singleton $\{x^*(\omega)\}$ for $\mu$-almost every input sequence $\omega$. Equivalently: all initial conditions converge to the same state when driven by the same input. The echo state response $x^*(\omega)$ is the unique stationary solution.

**Echo State Response $x^*$.** The measurable function $\omega \mapsto x^*(\omega) \in \mathcal{X}$ satisfying $x^*(T\omega) = F(x^*(\omega), u(\omega))$ — the unique stationary solution of the driven reservoir. Exists (as a measurable function) by the measurable selection theorem when the ESP holds; is a measurable selection from the pullback attractor when it does not.

**Skew-Product System.** The autonomous extension of a non-autonomous system: $\Theta(\omega, x) = (T\omega, F(x, u(\omega)))$ on $\Omega \times \mathcal{X}$. Converts the driven reservoir into an autonomous dynamical system, enabling the use of classical ergodic theory and attractor theory.

**Stationary Measure.** A measure $P$ on $\Omega \times \mathcal{X}$ invariant under $\Theta$. When the ESP holds, $P = \int \delta_{x^*(\omega)}\, d\mu(\omega)$ is the unique stationary measure. When it fails, multiple stationary measures may exist (corresponding to different measurable selections from the pullback attractor).

**Measurable Selection Theorem.** Guarantees the existence of a measurable function $s: \Omega \to \mathcal{X}$ with $s(\omega) \in A(\omega)$ for $\mu$-a.e. $\omega$, when $A(\omega)$ is non-empty, closed, and measurably parameterized. Ensures the echo state response is well-defined even when the pullback attractor is not a singleton.

**Lyapunov Exponent.** $\lambda_{\max} = \lim_{t\to\infty} \frac{1}{t}\log\|D_x\varphi(t,\omega,x^*)\|_{\text{op}}$, measuring the average exponential divergence of nearby trajectories. The ESP holds if and only if all Lyapunov exponents are negative. Exists almost surely by Oseledets' multiplicative ergodic theorem.

**Oseledets' Multiplicative Ergodic Theorem.** For a cocycle of matrices $A(t,\omega) = D_x\varphi(t,\omega,x^*)$, the Lyapunov exponents $\lambda_1 \geq \lambda_2 \geq \cdots \geq \lambda_N$ exist almost surely and are deterministic for ergodic inputs. The Lyapunov spectrum generalizes the eigenvalues of a linear map to the non-autonomous nonlinear setting.

**Edge of Chaos.** The regime where $\lambda_{\max} \approx 0$ — the boundary between the ordered phase ($\lambda_{\max} < 0$, ESP holds, forgetting of initial conditions) and the chaotic phase ($\lambda_{\max} > 0$, ESP fails, sensitive dependence). Proposed as the regime of optimal computational capacity, though the empirical evidence is mixed.

**$\phi$-Mixing.** A quantitative measure of temporal dependence decay: $\phi(k) = \sup_{t} \sup_{A \in \mathcal{F}_{\leq t}, B \in \mathcal{F}_{\geq t+k}} |\mathbb{P}(A \cap B) - \mathbb{P}(A)\mathbb{P}(B)|$. $\phi(k) \to 0$ means events far apart in time become approximately independent. Relevant for quantitative ergodic theorem convergence rates in reservoir training.
