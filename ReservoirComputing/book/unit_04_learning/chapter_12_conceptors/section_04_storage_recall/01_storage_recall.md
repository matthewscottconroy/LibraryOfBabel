# Storing and Recalling Patterns with Conceptors

## The Two-Phase Architecture

The conceptor framework separates pattern management into two distinct phases: storage and recall. This separation is principled — storage is a data-collection and matrix-computation process that requires driving the reservoir; recall is a constrained dynamical evolution that requires no external input. The two phases can be executed sequentially, and crucially, stored conceptors for different patterns can coexist without interfering with each other [Jaeger 2014].

## Storage Phase

For each pattern $p_k$ ($k = 1, \ldots, K$), the storage procedure is as follows.

**Step 1: Drive the reservoir.** Present pattern $p_k$ as input to the reservoir for $T_{\text{store}}$ time steps, collecting reservoir states:

$$\mathbf{x}_t^{(k)} = \tanh(\mathbf{W}^{\text{rec}} \mathbf{x}_{t-1}^{(k)} + \mathbf{W}^{\text{in}} p_k(t) + \mathbf{b}).$$

Discard an initial washout period of $T_w$ steps to eliminate transient dependence on the initial condition.

**Step 2: Compute the state correlation matrix.** Estimate

$$\mathbf{R}_k = \frac{1}{T_{\text{store}} - T_w} \sum_{t=T_w+1}^{T_{\text{store}}} \mathbf{x}_t^{(k)} \mathbf{x}_t^{(k) \top} \in \mathbb{R}^{N \times N}.$$

**Step 3: Compute the conceptor.** For a chosen aperture $\alpha > 0$:

$$\mathbf{C}_k = \mathbf{R}_k (\mathbf{R}_k + \alpha^{-2} \mathbf{I})^{-1}.$$

This requires computing the eigendecomposition of $\mathbf{R}_k$, which costs $O(N^3)$. For $K$ patterns, the total storage cost is $O(K N^2 T_{\text{store}} + K N^3)$.

Additionally, store the trained readout weights $\mathbf{W}_k^{\text{out}}$ for each pattern, which are learned by FORCE or ridge regression during the same drive phase.

## Recall Phase

To recall pattern $k$, apply the conceptor as a gating operation on the reservoir update:

$$\mathbf{x}_t = \mathbf{C}_k \tanh(\mathbf{W}^{\text{rec}} \mathbf{x}_{t-1} + \mathbf{b}),$$

$$y_t = \mathbf{W}_k^{\text{out}} \mathbf{x}_t.$$

The conceptor projection ensures that at each time step, the reservoir state is constrained to the subspace that pattern $k$ occupies. States in orthogonal directions — which might correspond to other patterns or noise — are suppressed. The result is stable autonomous generation of pattern $k$ [Jaeger 2014].

To switch to pattern $j$, simply replace $\mathbf{C}_k$ with $\mathbf{C}_j$ and $\mathbf{W}_k^{\text{out}}$ with $\mathbf{W}_j^{\text{out}}$. The transition requires no retraining.

## The Aperture Parameter

The aperture $\alpha$ in the conceptor formula $\mathbf{C} = \mathbf{R}(\mathbf{R} + \alpha^{-2} \mathbf{I})^{-1}$ interpolates between two limiting behaviors:

- **$\alpha \to 0$:** $\mathbf{C} \to \mathbf{0}$. The conceptor suppresses all activity — total constraint, zero recall.
- **$\alpha \to \infty$:** $\mathbf{C} \to \mathbf{I}$. The conceptor has no effect — no constraint, unconstrained reservoir dynamics.

For intermediate $\alpha$, the conceptor softly projects onto the subspace of typical states for the pattern. The singular values of $\mathbf{C}_k$ are $\sigma_i = \lambda_i^R / (\lambda_i^R + \alpha^{-2})$, which form a smooth decreasing function of the inverse aperture $\alpha^{-2}$ relative to the eigenvalues of $\mathbf{R}_k$.

Choosing $\alpha$ requires balancing recall fidelity against interference suppression. Empirically, Jaeger [2014] recommends choosing $\alpha$ such that the mean singular value of $\mathbf{C}_k$ is approximately 0.5, ensuring the conceptor neither over- nor under-constrains the recalled pattern.

## Interference Management

A key advantage of conceptors is that patterns stored in different subspaces of the reservoir state space are naturally separated. If patterns $p_j$ and $p_k$ use approximately orthogonal subspaces — formally, $\text{tr}(\mathbf{C}_j \mathbf{C}_k) \approx 0$ — then the conceptors do not interfere. Pattern $j$'s characteristic directions are suppressed by $\mathbf{C}_k$ and vice versa.

The interference between two patterns can be quantified by the Frobenius inner product $\langle \mathbf{C}_j, \mathbf{C}_k \rangle_F = \text{tr}(\mathbf{C}_j^\top \mathbf{C}_k)$. Patterns with high inner product share reservoir subspace and will interfere during recall. The number of patterns that can be stored without significant interference is bounded by the reservoir dimension $N$ — specifically, by the number of orthogonal subspaces of dimension $d_k = \text{tr}(\mathbf{C}_k)$ (the effective rank of the conceptor) that can fit in $\mathbb{R}^N$ [Jaeger 2014].

## Conceptor Logic: NOT, AND, OR

Conceptors form a Boolean algebra. The **NOT** conceptor $\neg \mathbf{C} = \mathbf{I} - \mathbf{C}$ excludes the subspace of pattern $\mathbf{C}$. The **AND** conceptor $\mathbf{C}_j \wedge \mathbf{C}_k$ projects onto the intersection of the two patterns' subspaces. The **OR** conceptor $\mathbf{C}_j \vee \mathbf{C}_k$ spans their union.

These operations enable a form of pattern algebra. To generate activity that includes pattern $j$ but excludes pattern $k$, apply $\mathbf{C}_j \wedge \neg \mathbf{C}_k$. To generate a mixture of two patterns, apply $\mathbf{C}_j \vee \mathbf{C}_k$. This logical structure has no analog in standard readout-based approaches and is one of the most distinctive features of the conceptor framework [Jaeger 2014].

## Stability Conditions

For recalled patterns to be stable, the reservoir modified by the conceptor must have a contracting fixed point or stable limit cycle at the target trajectory. A sufficient condition is that the conceptor-modified system $\mathbf{x} \mapsto \mathbf{C}_k \tanh(\mathbf{W}^{\text{rec}} \mathbf{x} + \mathbf{b})$ has spectral radius less than one in all directions within the support of $\mathbf{C}_k$. This is equivalent to requiring

$$\|\mathbf{C}_k \mathbf{W}^{\text{rec}} \text{diag}(\tanh'(\cdot))\|_2 < 1.$$

Since $\mathbf{C}_k \preceq \mathbf{I}$, applying the conceptor always reduces the effective spectral radius relative to the unconstrained reservoir, which helps stability — but does not guarantee it when the target pattern is complex [Jaeger 2014].

---

## References

- Jaeger, H. (2014). Controlling recurrent neural networks by conceptors. *arXiv preprint*, arXiv:1403.3369.
