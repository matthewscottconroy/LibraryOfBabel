# What Does FORCE Learning Actually Produce?

## Beyond the Readout: Structure in the Closed-Loop System

A naive reading of FORCE learning suggests it merely finds a good readout vector $\mathbf{w}^{\text{out}}$ for a fixed random reservoir. This reading is incomplete. The learned $\mathbf{w}^{\text{out}}$, through the feedback connection $\mathbf{W}^{\text{fb}}$, modifies the effective recurrent dynamics of the network. The closed-loop system that emerges from FORCE training has structural properties — low-rank weight modifications, stable attractors, rotational state-space dynamics — that are absent in the untrained reservoir and are not imposed by construction. Understanding what FORCE produces is essential for understanding why it works.

## Low-Rank Structure in the Effective Weight Matrix

The effective recurrent matrix after FORCE training is

$$\mathbf{W}_{\text{eff}} = \mathbf{W}^{\text{rec}} + \mathbf{W}^{\text{fb}} \mathbf{w}^{\text{out} \top}.$$

The perturbation $\mathbf{W}^{\text{fb}} \mathbf{w}^{\text{out} \top}$ is a rank-one matrix (for scalar output) or rank-$d$ matrix (for $d$-dimensional output). This is a low-rank modification of the initial random connectivity. More generally, if FORCE is applied to learn $K$ output dimensions simultaneously, the total perturbation has rank at most $K$:

$$\mathbf{W}_{\text{eff}} = \mathbf{W}^{\text{rec}} + \mathbf{W}^{\text{fb}} \mathbf{W}^{\text{out}}, \quad \text{rank}(\mathbf{W}^{\text{fb}} \mathbf{W}^{\text{out}}) \leq K.$$

For tasks where $K \ll N$, the learned network is, in a precise sense, a low-rank perturbation of the random reservoir. Ganguli et al. [2008] showed that for many cognitive and motor tasks, the relevant dynamics live in a low-dimensional subspace of the full $N$-dimensional state space. FORCE learning discovers this subspace and modifies the recurrence to stabilize trajectories within it, while leaving the high-dimensional random dynamics largely intact.

## Why Low-Rank? Subspace Relevance

The low-rank structure has a geometric explanation. The target trajectory $z^*(t)$ is determined by the projection of the reservoir state onto $\mathbf{w}^{\text{out}}$. The only component of the reservoir dynamics that matters for the output is the projection onto the readout direction. FORCE modifies the recurrence only along the feedback direction $\mathbf{W}^{\text{fb}}$, which acts on the readout output. The orthogonal complement of this subspace is untouched.

More precisely, let $\mathbf{v} = \mathbf{w}^{\text{out}} / \|\mathbf{w}^{\text{out}}\|$ be the normalized readout direction. The rank-one perturbation $\mathbf{W}^{\text{fb}} \mathbf{w}^{\text{out} \top}$ modifies only the dynamics in the direction $\mathbf{v}$. Reservoir neurons that do not project onto $\mathbf{v}$ are unaffected. This parsimony — modifying only the subspace relevant to the task — is what Ganguli et al. [2008] identified as characteristic of efficient neural computation.

## The Backbone Attractor

After successful FORCE training, the network has an attractor at the target trajectory. Specifically, the closed-loop dynamical system

$$\mathbf{x}_{t+1} = \tanh(\mathbf{W}_{\text{eff}} \mathbf{x}_t)$$

has a stable limit cycle (for periodic targets) or stable chaotic attractor (for chaotic targets) that approximates the target trajectory in state space. The target trajectory is the "backbone" around which the learned dynamics are organized. Perturbations from initial conditions or noise decay back to this trajectory, provided the perturbation is not too large [Sussillo & Abbott 2009].

This attractor structure is not present in the original random reservoir. FORCE learning constructs it by shaping the low-rank perturbation so that the Lyapunov exponents transverse to the target trajectory are negative. The RLS update accomplishes this implicitly: by keeping $e(t) \approx 0$ throughout training, it ensures the closed-loop trajectory never leaves the neighborhood of the target, and the resulting $\mathbf{w}^{\text{out}}$ stabilizes this neighborhood.

## Rotational Dynamics in Trained Networks

Churchland et al. [2012] reported a striking empirical finding: neural activity in motor cortex during reaching tasks exhibits rotational structure in the principal component (PC) subspace of the state trajectory. The leading PCs of the firing rate matrix trace ellipses and spirals rather than converging to fixed points. This rotational structure had not been predicted by rate-coding models and was initially surprising.

Sussillo & Abbott [2009] demonstrated that FORCE-trained networks spontaneously reproduce rotational dynamics when trained on motor-cortex-like tasks. The intuition is that rotational dynamics are the most natural attractor geometry for a network that must generate periodic or quasi-periodic output: the state spirals around the target trajectory rather than approaching it radially. The low-rank perturbation introduced by FORCE naturally generates such rotational structure by coupling orthogonal state dimensions through the feedback loop.

## Kolmogorov Complexity and Task Difficulty

The complexity of the learned dynamics is ultimately bounded by the complexity of the target signal. A target $z^*(t)$ with low Kolmogorov complexity — such as a pure sine wave — requires only a small perturbation to the random reservoir (low-rank modification of small magnitude). A target with high Kolmogorov complexity — such as a chaotic Lorenz trajectory — requires a larger and more structured perturbation.

Quantitatively, if the target can be generated by a dynamical system of dimension $d$ and Lyapunov exponent $\lambda_{\max}$, then the FORCE-trained network must contain an attractor of at least dimension $d$ and must contend with instabilities on timescales $\sim 1/\lambda_{\max}$. This imposes a lower bound on the reservoir size $N$ and the RLS regularization parameter $\lambda$ for successful training [Sussillo & Abbott 2009].

---

## References

- Sussillo, D., & Abbott, L. F. (2009). Generating coherent patterns of activity from chaotic neural networks. *Neuron*, 63(4), 544–557.
- Ganguli, S., Huh, D., & Sompolinsky, H. (2008). Memory traces in dynamical systems. *Proceedings of the National Academy of Sciences*, 105(48), 18970–18975.
- Churchland, M. M., Cunningham, J. P., Kaufman, M. T., Foster, J. D., Nuyujukian, P., Ryu, S. I., & Shenoy, K. V. (2012). Neural population dynamics during reaching. *Nature*, 487(7405), 51–56.
