# Chapter 34 Exercises

*The exercises in this chapter are different from those in earlier chapters. They ask you to think about open problems as a researcher: to sketch what a solution would look like, to propose conjectures, and to design experiments. There are no unique "correct" answers. The goal is to develop the habit of research thinking.*

## On Approximation Rate Bounds (Problem 34.1.1)

**Exercise 34.1.** *(Thought experiment: tight bounds)* Consider the class $\mathcal{F}_L$ of Lipschitz fading-memory functionals on $X_w$ with Lipschitz constant $L$. An $N$-unit random reservoir approximates each $H \in \mathcal{F}_L$ with error $\varepsilon_N$ (in the uniform norm on some compact $K$).

(a) What is the classical information-theoretic lower bound on $\varepsilon_N$ for any approximation scheme using $N$ parameters, in terms of the Kolmogorov entropy of $\mathcal{F}_L$ on $K$?

(b) Does a random reservoir achieve this lower bound? Sketch why or why not.

(c) Propose a conjecture: for $\mathcal{F}_L$ and a fixed compact $K$, the minimax optimal approximation rate for $N$-parameter approximators is $\varepsilon_N^* = \Theta(N^{-\beta})$ for some $\beta = \beta(L, K)$. What would $\beta$ be, and how would you prove it?

**Exercise 34.2.** *(Research exercise: gap between polynomial and neural approximation)* Let $\mathcal{F}(w, 2, R)$ be the class of second-order Sobolev functionals (as defined in Section 26.5.2). 

(a) The polynomial approximation upper bound gives error $O(N^{-\alpha})$ using degree-2 polynomial functionals with $N$ terms. Describe explicitly what this approximator looks like as a reservoir computer.

(b) A random $N$-unit tanh reservoir produces a random feature map. Is the approximation error of this feature map provably equal to, better, or worse than the polynomial approximation bound? State this as a precise conjecture and describe what techniques you would use to prove it.

(c) Propose a "hard functional" — a specific $H \in \mathcal{F}(w, 2, R)$ — that you believe would be hardest for a random reservoir to approximate. Justify your choice.

## On Optimal Reservoir Design (Problem 34.1.2)

**Exercise 34.3.** *(Thought experiment: designing for a task)* You are told that the target functional is $H(\mathbf{u}) = \int_{-\infty}^0 u(s) e^{s} ds$ (an exponential filter). 

(a) What is the optimal (exact) reservoir for this task? Describe explicitly the reservoir architecture and the readout.

(b) How does a random reservoir approximate this task? What is the approximation error as a function of $N$?

(c) Now suppose the target functional is $H(\mathbf{u}) = (\int_{-\infty}^0 u(s) e^{s} ds)^2 + (\int_{-\infty}^0 u(s) e^{2s} ds)^2$ (a quadratic combination of two exponential filters). How does the optimal reservoir change? How does the random reservoir performance change?

**Exercise 34.4.** *(Research exercise: connection to $n$-widths)* The Kolmogorov $n$-width of a set $\mathcal{F}$ in a normed space $X$ is $d_n(\mathcal{F}, X) = \inf_{Y_n} \sup_{H \in \mathcal{F}} \inf_{g \in Y_n} \|H - g\|_X$, where the outer infimum is over all $n$-dimensional subspaces $Y_n$ of $X$.

(a) Show that the approximation rate problem for reservoir computing (best error using $N$-unit reservoir) is at most as hard as the $n$-width problem (best $N$-dimensional linear approximation).

(b) For the class $\mathcal{F}(w, s, R)$ from Section 26.5.2, compute (or bound) $d_N(\mathcal{F}, C(K))$ using known results about widths of Sobolev spaces.

(c) Is the random reservoir the "optimal" $N$-dimensional subspace in the $n$-width sense? If not, what would the optimal subspace look like?

## On the ESP-Task Performance Gap (Problem 34.1.4)

**Exercise 34.5.** *(Thought experiment: approximate ESP)* Consider a reservoir with spectral radius $\rho(W) = 1 + \delta$ for small $\delta > 0$ (slightly above the ESP boundary). 

(a) Using the pullback attractor framework (Chapter 29), describe what happens to the pullback attractor when $\delta > 0$. Is it still a single point?

(b) For the task of predicting a slowly varying smooth signal $y(t) = \sin(t/100)$, do you expect the mildly chaotic regime to help or hurt? Justify your answer.

(c) Formalize the "practical ESP" condition: define a condition $\text{ESP}(\varepsilon, T)$ that requires the reservoir state to converge within $\varepsilon$ of the "true" echo state within $T$ time steps, for all inputs in a given class. What $\rho(W)$ condition is needed to satisfy $\text{ESP}(\varepsilon, T)$?

**Exercise 34.6.** *(Experimental design)* Design an experiment to test whether the ESP-task performance gap is real:

(a) Choose a specific task that you believe would benefit from mildly chaotic reservoir dynamics. Justify this choice theoretically.

(b) Design the experimental protocol: what reservoir sizes, spectral radii, and tasks would you test? What statistical tests would you use?

(c) What result would convince you that the mildly chaotic regime genuinely improves performance beyond what a well-tuned ESP reservoir achieves? What result would convince you it does not?

## On FORCE Learning (Problem 34.1.5)

**Exercise 34.7.** *(Thought experiment: FORCE convergence)* Consider the scalar version of FORCE: $\dot{w} = -P(t) x(t) e(t)$ where $x(t)$ is a scalar reservoir state, $e(t) = w(t) x(t) - y^*(t)$ is the error, and $P(t)^{-1} = \int_0^t x(s)^2 ds + \lambda$.

(a) Show that if $x(t)$ is i.i.d. (independent at each time step), FORCE converges to the optimal weight $w^* = \mathbb{E}[y^* x] / \mathbb{E}[x^2]$ at rate $O(1/t)$.

(b) Now suppose $x(t) = \rho x(t-1) + \xi(t)$ (AR(1) process). Does FORCE still converge? At what rate? What does the mixing coefficient of this AR(1) process tell you about the convergence rate?

(c) Propose a modification of FORCE that accounts for temporal correlations in $x(t)$, potentially giving faster convergence. Sketch a proof that your modification converges.

## Research-Style Exercises

**Exercise 34.8.** *(Propose a conjecture)* Based on your reading of Chapters 26-29 and 34, propose a precise mathematical conjecture about reservoir computing. Requirements: the conjecture should be (a) clearly stated with all quantifiers, (b) non-trivial (not immediately obvious), and (c) potentially provable with current techniques or modest extensions thereof. Write a 300-word argument for why the conjecture might be true and a 300-word argument for why it might fail.

**Exercise 34.9.** *(Propose an experimental design)* The "edge of chaos" hypothesis (Section 29.4.5) predicts that reservoir performance peaks when the maximal Lyapunov exponent $\lambda_{\max} \approx 0$. This hypothesis has been supported and challenged in various papers, but no definitive experiment has been done.

Design a rigorous experimental test of this hypothesis:
(a) Specify the task class: what temporal tasks would most cleanly distinguish the edge-of-chaos effect from artifacts (task memorability, input statistics)?
(b) Specify the reservoir class: what reservoir architectures allow precise control of $\lambda_{\max}$ as an independent variable?
(c) Specify the statistical analysis: what test would distinguish "performance peaks at $\lambda_{\max} = 0$" from "performance peaks at $\lambda_{\max} = $ (some positive value)" with high confidence?
(d) What confounds must be controlled, and how?

**Exercise 34.10.** *(Research program)* Write a 2-3 page research proposal for a 5-year program addressing one of the open problems in Section 34.1. The proposal should include:
(a) A precise statement of the problem.
(b) A review of what is currently known.
(c) A research approach with specific technical steps.
(d) Expected intermediate results (what you would be able to prove/demonstrate in 1 year, 3 years, 5 years).
(e) Impact: why does this matter for reservoir computing and for machine learning more broadly?
