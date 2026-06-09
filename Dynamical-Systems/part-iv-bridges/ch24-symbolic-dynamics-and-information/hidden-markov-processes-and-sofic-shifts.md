# 24.3 Hidden Markov Processes and Sofic Shifts

Hidden Markov models are one of the most widely used statistical models in engineering and machine learning: speech recognition, bioinformatics, financial modeling, natural language processing. The model has two components — a hidden Markov chain evolving in the background, and an observation process that emits symbols based on the hidden state (plus noise).

Symbolic dynamics has its own class of systems that goes beyond SFTs: sofic shifts, defined as factor images of SFTs. These might seem like a purely mathematical construction. But they're exactly the same thing.

**Definition 24.3.1.** A *hidden Markov model (HMM)* consists of:
- A Markov chain $(S_n)$ on a finite state space (hidden states)
- An observation function: at each step, emit symbol $Y_n = g(S_n, N_n)$ (where $N_n$ is i.i.d. noise)

The output process $(Y_n)$ is the *hidden Markov process*.

**Theorem 24.3.2.** The output process of an HMM (with a finite-state Markov chain) is exactly a *sofic process* — a process whose support is a sofic shift.

*(proof)* The states of the Markov chain are the hidden states of the SFT that presents the sofic shift. The observation function is the factor map.

Let's see why. The Markov chain $(S_n)$ lives on a finite state space $\mathcal{S}$. The transitions $(S_n, S_{n+1})$ define an SFT on $\mathcal{S}$. The observation $Y_n = g(S_n)$ (in the deterministic emission case) is a function of the current state — this is a sliding block code of window size 1. The image of the SFT under this factor map is a sofic shift.

In the noisy emission case ($Y_n = g(S_n, N_n)$ with i.i.d. noise), the analysis is similar but we need to think about measures rather than supports. The support of the output process is still a sofic shift, but the measure on it is the image of the Markov measure on the hidden-state SFT under the factor map.

This identification explains why sofic shifts are natural: they're what you see when you observe a Markov chain through a noisy channel. Any real-world system described by an HMM — speech, DNA, financial data — has its observable process living in a sofic shift.

**Entropy of HMMs:** The entropy rate of a sofic process is $h = H(Y_n | Y_{n-1}, Y_{n-2}, \ldots)$ — the conditional entropy given all past observations. Computing this requires knowledge of the *Blackwell measure* (the stationary distribution over the belief states of the hidden Markov filter). This is generally hard and does not have a closed form.

The Blackwell measure lives on the simplex of probability distributions over hidden states. As you observe the output sequence, you update your belief about the hidden state using Bayes' theorem — this is the hidden Markov filter. The stationary distribution of this belief-update process is the Blackwell measure, named after David Blackwell who studied it in the context of information theory.

Computing the entropy rate of a sofic process (or equivalently, an HMM) is one of the hard open problems in the area. There is no general closed-form formula. The entropy rate can be computed as:
$$h = -\int_{\Delta} \sum_y \pi(y | p) \log \pi(y | p) \, d\mathbb{B}(p),$$
where $\mathbb{B}$ is the Blackwell measure on belief states $p \in \Delta(\mathcal{S})$ and $\pi(y|p)$ is the probability of observing $y$ given belief state $p$. This integral is usually computed numerically.

The difficulty of computing HMM entropy rate is connected to the difficulty of Lyapunov exponents for random matrix products (the transfer matrices of the HMM form a random matrix product). Computing the leading Lyapunov exponent of a random matrix product is generically hard — and the entropy rate of a sofic process is essentially such a Lyapunov exponent.

This is one of the places where the dictionary between symbolic dynamics and information theory points to an open problem rather than a closed answer.
