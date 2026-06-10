# The Multiple-Pattern Problem in Reservoir Computing

## Goal: Storing Multiple Temporal Patterns

A single reservoir can be trained to generate one target time series by FORCE learning or teacher forcing. A natural and important extension is to store multiple temporal patterns $p_1, p_2, \ldots, p_K$ in a single reservoir and recall any of them on demand. This is the multiple-pattern problem. It arises in motor control (a robot must produce $K$ distinct movement trajectories), in speech generation (multiple phoneme-level patterns), and in neuroscience models of pattern memory.

The challenge is not merely technical but structural. Each pattern, when being recalled, requires the reservoir to visit a specific sequence of states. Different patterns require different state sequences. A single reservoir cannot simultaneously be in two different state sequences — the recall cue must selectively route the reservoir dynamics to the appropriate attractor [Jaeger 2014].

## The Naive Approach and Its Limits

The naive solution is to train a separate readout $\mathbf{w}^{\text{out}}_k$ for each pattern $p_k$. During recall of pattern $k$, use $\mathbf{w}^{\text{out}}_k$ to read out and feed back. This works in the sense that, if each readout is trained by FORCE, each pattern can be generated stably.

However, the naive approach has a critical failure mode: when the reservoir is asked to generate multiple patterns using different readouts connected through the same feedback weights $\mathbf{W}^{\text{fb}}$, the effective recurrent matrices for different patterns are

$$\mathbf{W}_k = \mathbf{W}^{\text{rec}} + \mathbf{W}^{\text{fb}} \mathbf{w}_k^{\text{out} \top}.$$

These are $K$ different dynamical systems sharing the same reservoir. The attractor for pattern $k$ is a property of $\mathbf{W}_k$, not of $\mathbf{W}^{\text{rec}}$ alone. When the readout is switched from $k$ to $j$, the dynamical system changes abruptly, and the current reservoir state may be far from the attractor of the new system. Recovery is slow or impossible. The patterns interfere [Jaeger 2014].

A deeper problem is that the naive approach does not provide a principled mechanism for managing the reservoir's activity during recall. Without such a mechanism, patterns stored in overlapping subspaces of the reservoir state space will interfere regardless of how carefully each individual readout is trained.

## Conceptors as the Solution

Conceptors, introduced by Jaeger [2014], provide a principled solution to the routing problem. A conceptor $\mathbf{C}_k$ is a matrix that projects the reservoir state onto the subspace characteristic of pattern $k$'s activity. By applying $\mathbf{C}_k$ to the reservoir state during recall, the dynamics are constrained to the subspace that pattern $k$ inhabits, suppressing interference from other patterns.

## Conceptor Matrix Derivation

During a storage phase, the reservoir is driven by pattern $p_k$ for a long sequence of time steps, collecting reservoir states $\{\mathbf{x}_t^{(k)}\}$. The state correlation matrix for pattern $k$ is:

$$\mathbf{R}_k = \mathbb{E}[\mathbf{x}^{(k)} \mathbf{x}^{(k) \top}].$$

In practice, $\mathbf{R}_k$ is estimated as $\mathbf{R}_k = \frac{1}{T} \sum_{t=1}^T \mathbf{x}_t^{(k)} \mathbf{x}_t^{(k) \top}$.

The conceptor for pattern $k$ with aperture parameter $\alpha > 0$ is defined as:

$$\mathbf{C}_k = \mathbf{R}_k (\mathbf{R}_k + \alpha^{-2} \mathbf{I})^{-1}.$$

This is a positive semidefinite matrix satisfying $\mathbf{0} \preceq \mathbf{C}_k \preceq \mathbf{I}$, with eigenvalues

$$\lambda_i^C = \frac{\lambda_i^R}{\lambda_i^R + \alpha^{-2}},$$

where $\lambda_i^R$ are the eigenvalues of $\mathbf{R}_k$. Directions in state space with large variance (large $\lambda_i^R$) receive weight near 1; directions with small variance receive weight near 0. The conceptor is, in essence, a soft projection onto the principal subspace of typical reservoir states for pattern $k$ [Jaeger 2014].

## Geometric Interpretation

Let the eigendecomposition of $\mathbf{R}_k$ be $\mathbf{R}_k = \mathbf{U}_k \boldsymbol{\Lambda}_k \mathbf{U}_k^\top$. The conceptor applies a direction-dependent gain:

$$\mathbf{C}_k = \mathbf{U}_k \text{diag}\!\left(\frac{\lambda_i}{\lambda_i + \alpha^{-2}}\right) \mathbf{U}_k^\top.$$

This is exactly the operation of projecting onto the ellipsoid

$$\mathcal{E}_k = \{\mathbf{x} : \mathbf{x}^\top \mathbf{R}_k^{-1} \mathbf{x} \leq 1\} \cap \text{support}(\mathbf{R}_k),$$

with a smooth, continuous transition rather than a sharp cutoff. States that lie inside the characteristic ellipsoid of pattern $k$ pass through nearly unchanged; states orthogonal to the pattern's subspace are suppressed.

The aperture parameter $\alpha$ controls the width of the ellipsoid. Large $\alpha$: the conceptor approaches the identity (no constraint); small $\alpha$: the conceptor approaches the projector onto the dominant subspace of $\mathbf{R}_k$. The choice of $\alpha$ balances fidelity (large $\alpha$ allows more states) against selectivity (small $\alpha$ enforces tight constraint on recalled pattern).

## Pattern Recall Mechanism

During recall of pattern $k$, the conceptor modifies the reservoir update:

$$\mathbf{x}_t = \mathbf{C}_k \tanh(\mathbf{W}^{\text{rec}} \mathbf{x}_{t-1} + \mathbf{b}),$$

where $\mathbf{b}$ is a bias. The conceptor acts as a gating mechanism: at each step, the pre-activation is projected onto the subspace of pattern $k$ before being stored as the new state. This drives the reservoir to remain within the attractor of pattern $k$, enabling stable autonomous generation.

---

## References

- Jaeger, H. (2014). Controlling recurrent neural networks by conceptors. *arXiv preprint*, arXiv:1403.3369.
