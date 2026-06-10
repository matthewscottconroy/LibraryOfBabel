# 1.3.3 Hidden Markov Models and Probabilistic State

## The Probabilistic Turn

Linear filters handle temporal structure through deterministic, algebraic recursions. But many temporal signals are better described probabilistically — the future is not a deterministic function of the past, but a distribution over possibilities that depends on a hidden state of the world.

Hidden Markov Models (HMMs) [Rabiner1989] are the classical probabilistic approach to temporal sequence modeling. They have been enormously successful in speech recognition, bioinformatics, and natural language processing, and they provide a conceptual bridge between the deterministic dynamical systems perspective of this book and the probabilistic sequence models of modern ML.

## The HMM Architecture

A Hidden Markov Model has:

- A finite set of **hidden states** $\mathcal{S} = \{1, 2, \ldots, K\}$
- A **transition matrix** $A$ where $A_{ij} = P(s_t = j \mid s_{t-1} = i)$
- An **emission distribution** $B_k(\cdot)$ giving the probability of observing output $y$ in state $k$: $B_k(y) = P(y_t = y \mid s_t = k)$
- An **initial distribution** $\pi_k = P(s_1 = k)$

The generative story is: the hidden state sequence $(s_1, s_2, \ldots, s_T)$ evolves as a Markov chain with transition matrix $A$. At each time step, an observation $y_t$ is sampled from the emission distribution of the current state $B_{s_t}$.

The observations are all we see. The states are hidden — hence the name.

## What HMMs Compute

An HMM defines a joint distribution over sequences of observations: $P(y_1, y_2, \ldots, y_T)$. This is computed by summing over all possible hidden state sequences:

$$P(y_1, \ldots, y_T) = \sum_{s_1, \ldots, s_T} \pi_{s_1} \prod_{t=2}^T A_{s_{t-1}, s_t} \prod_{t=1}^T B_{s_t}(y_t)$$

This sum has exponentially many terms, but the **forward algorithm** (a dynamic programming recursion) computes it in $O(TK^2)$ time.

The key computational object is the **forward variable**:

$$\alpha_t(k) = P(y_1, \ldots, y_t, s_t = k)$$

Updated as:

$$\alpha_{t+1}(j) = \left(\sum_{i=1}^K \alpha_t(i) A_{ij}\right) B_j(y_{t+1})$$

This forward variable is the hidden state of the HMM computation — a $K$-dimensional vector that summarizes everything the model knows about the history of the sequence up to time $t$. It is the HMM's version of the reservoir state.

## The Markov Assumption and Its Limits

The defining assumption of HMMs is the **Markov property**: the current hidden state $s_t$ contains all information about the past that is relevant to the future. Formally:

$$P(s_{t+1} \mid s_1, \ldots, s_t, y_1, \ldots, y_t) = P(s_{t+1} \mid s_t)$$

This assumption is powerful because it makes the model tractable. It is limiting because it requires that all temporal dependencies be mediated through the $K$-state hidden state. For sequences with rich, long-range dependencies, this may require very large $K$ — and HMMs with large state spaces are difficult to train.

**Training** is done with the **Baum-Welch algorithm** (Expectation-Maximization applied to HMMs), which finds the model parameters $(A, B, \pi)$ that maximize the likelihood of the observed data. The algorithm is elegant but prone to local optima and sensitive to initialization.

## What HMMs Cannot Do

HMMs have the following hard limitations:

1. **Discrete hidden states only.** The hidden state is one of $K$ discrete values. Representing continuous latent dynamics requires a different model (the Kalman filter for linear Gaussian dynamics, or a state-space model for nonlinear dynamics).

2. **The Markov order is fixed.** An HMM assumes that one step of hidden state is sufficient for prediction. Higher-order dependencies require either higher-order HMMs (with $K^m$ states for order $m$) or a continuous latent space.

3. **The emission model is fixed.** HMMs typically use Gaussian or discrete emissions. Arbitrary nonlinear relationships between state and observation require extensions (HMMs with neural network emissions).

4. **Gradient-based training is difficult.** Baum-Welch is EM, which converges to local optima. Gradient-based methods exist but can be unstable.

## HMMs and Reservoir Computing: The Conceptual Comparison

The contrast between HMMs and reservoir computers illuminates what each is doing:

| Aspect | HMM | Reservoir Computer |
|--------|-----|--------------------|
| State space | Discrete, finite ($K$ states) | Continuous, high-dimensional ($\mathbb{R}^N$) |
| State transition | Probabilistic (transition matrix $A$) | Deterministic (reservoir dynamics $f$) |
| State dimension | $K$ (must be chosen and trained) | $N$ (random, fixed) |
| Temporal dependency | Markov (one step) | Fading memory (unlimited depth, finite effective window) |
| Training | EM (Baum-Welch), prone to local optima | Linear regression on reservoir states (convex, global optimum) |
| Expressiveness | Any distribution over sequences | Any fading-memory functional (by Boyd-Chua) |
| Generative capability | Natural (sample from the model) | Possible with output feedback (Chapter 10) |

The reservoir's continuous, high-dimensional state space is its great advantage: it can represent fine-grained distinctions between input histories that a discrete $K$-state HMM would collapse together. And because the reservoir state is deterministic given the input history, the readout training problem is simply linear regression — a convex problem with a unique global solution.

HMMs retain advantages for explicitly probabilistic inference tasks — computing posterior distributions over hidden states, generating samples from a learned distribution, computing exact likelihoods. These are cases where the probabilistic structure of the model is not incidental but central to the task.

---

## References

- [Rabiner1989] Rabiner, L.R. (1989). A tutorial on Hidden Markov Models and selected applications in speech recognition. *Proceedings of the IEEE*, 77(2), 257–286. **[The definitive tutorial — 20,000+ citations. Essential reading.]**
- [Baum1970] Baum, L.E. et al. (1970). A maximization technique occurring in the statistical analysis of probabilistic functions of Markov chains. *Annals of Mathematical Statistics*, 41(1), 164–171.
- [Bengio1994] Bengio, Y. & Frasconi, P. (1994). An input output HMM architecture. *Advances in Neural Information Processing Systems*, 7.
- [Murphy2012] Murphy, K.P. (2012). *Machine Learning: A Probabilistic Perspective*. MIT Press. Chapters 17–18 cover HMMs and state-space models comprehensively.
