# Section 11.3: Full-FORCE

## 11.3.1 The Teacher Forcing Problem

FORCE as described in Section 11.2 trains the network *with* the target signal fed back as the output $z(t) \approx f(t)$. During training, the feedback $z(t)$ gradually approaches $f(t)$, and by the end of training, the weights $\mathbf{w}$ have been adjusted so that the network's autonomous output closely tracks $f$.

But there is a subtle problem: during training, the network is never truly autonomous. The output $z(t) = \mathbf{w}(t)^\top\mathbf{r}(t)$ uses the *current* (changing) weights and the *current* reservoir state, which is driven partly by the feedback $z(t-1) \approx f(t-1)$. After training is complete and the teacher (the target signal) is removed, the network must run autonomously: $z(t) = \mathbf{w}^\top\mathbf{r}(t)$ with the *final fixed* weights, and the reservoir state $\mathbf{r}(t)$ is now driven only by $z(t-1)$, without any external target to stabilize it.

This transition from "teacher-forced" training to "autonomous" operation is the **teacher forcing removal problem**. The reservoir state trajectory during training (driven by $f(t)$) may be very different from the autonomous trajectory (driven by $z(t)$). Small perturbations — inevitable due to the finite weight updates — can cause the autonomous trajectory to diverge from the trained one.

The practical manifestation: FORCE often trains successfully (error converges during training) but the autonomous test performance is poor — the network generates the right signal for a while, then drifts away into chaotic behavior.

## 11.3.2 The Full-FORCE Solution

DePasquale, Cueva, Rajan, and Abbott [DePasquale2018] developed **Full-FORCE** to address the teacher forcing removal problem. The core idea is to train the network using a *target network* that generates the desired reservoir state trajectory, rather than teacher-forcing with the target signal directly.

**Target network.** Define a "target network" that runs autonomously on the desired output $f(t)$:

$$\mathbf{r}^{tgt}(t) = \tanh\!\bigl(W\mathbf{r}^{tgt}(t-1) + \mathbf{w}^{fb} f(t-1)\bigr).$$

This is the same reservoir network, but driven by the *true target signal* $f(t)$ as its feedback. The states $\mathbf{r}^{tgt}(t)$ are the states the network *should* be in if it were generating the correct output.

**Full-FORCE objective.** Train the output weights $\mathbf{w}$ so that:
1. The output matches the target: $\mathbf{w}^\top\mathbf{r}^{tgt}(t) \approx f(t)$.
2. The dynamics are self-consistent: the network driven by its own output generates states close to $\mathbf{r}^{tgt}(t)$.

Condition 2 requires that the network, when run autonomously, generates state trajectories close to $\{\mathbf{r}^{tgt}(t)\}$. This is a self-consistency condition that is harder to enforce than condition 1 alone.

## 11.3.3 Full-FORCE Architecture and Algorithm

Full-FORCE trains two sets of weights simultaneously:

1. **Output weights** $\mathbf{w}^{out}$ for the readout: $z(t) = \mathbf{w}^{out\top}\mathbf{r}(t)$.
2. **Internal weights** $\mathbf{w}^{int}$ for each neuron's input: $r_j^{int}(t) = \mathbf{w}^{int\top}_j \mathbf{r}(t)$, trained to match the "target input" for neuron $j$.

**Step 1: Run target network.**

$$\mathbf{r}^{tgt}(t) = \tanh\!\bigl(W\mathbf{r}^{tgt}(t-1) + \mathbf{w}^{fb} f(t-1)\bigr).$$

**Step 2: Compute target pre-activation for each neuron.**

The target input to neuron $j$ — what neuron $j$ *should* receive to reproduce $\mathbf{r}^{tgt}(t)$ — is:

$$x_j^{tgt}(t) = \tanh^{-1}(r_j^{tgt}(t)).$$

The actual input to neuron $j$ in the "learned" network (which uses $\mathbf{w}^{int}_j$ trained weights) is:

$$x_j^{learned}(t) = (W\mathbf{r}^{tgt}(t-1))_j + (\mathbf{w}^{int}_j)^\top \mathbf{r}^{tgt}(t-1).$$

**Step 3: Train $\mathbf{w}^{int}_j$ to minimize $\|x_j^{learned} - x_j^{tgt}\|^2$.**

Using FORCE (RLS) for each neuron $j$:

$$e_j(t) = x_j^{learned}(t) - x_j^{tgt}(t) = (W\mathbf{r}^{tgt}(t-1))_j + (\mathbf{w}^{int}_j)^\top \mathbf{r}^{tgt}(t-1) - \tanh^{-1}(r_j^{tgt}(t)).$$

The RLS update for $\mathbf{w}^{int}_j$ using state $\mathbf{r}^{tgt}(t-1)$ and target $\tanh^{-1}(r_j^{tgt}(t)) - (W\mathbf{r}^{tgt}(t-1))_j$ is:

$$\mathbf{k}_j(t) = \frac{P_j(t-1)\mathbf{r}^{tgt}(t-1)}{1 + \mathbf{r}^{tgt}(t-1)^\top P_j(t-1)\mathbf{r}^{tgt}(t-1)},$$

$$P_j(t) = P_j(t-1) - \mathbf{k}_j(t)\mathbf{r}^{tgt}(t-1)^\top P_j(t-1),$$

$$\mathbf{w}^{int}_j(t) = \mathbf{w}^{int}_j(t-1) - e_j(t)\mathbf{k}_j(t).$$

**Step 4: Train $\mathbf{w}^{out}$ to match the target output.**

Using standard FORCE (Section 11.2) on the target network states:

$$e^{out}(t) = \mathbf{w}^{out\top}\mathbf{r}^{tgt}(t) - f(t),$$

with the same RLS update formula.

**Step 5: Construct the final network.**

The learned network has modified connectivity $W^{eff} = W + W^{int}$, where $(W^{int})_{ji} = (w^{int}_j)_i$ (the trained internal weights for each neuron). After training, the network runs autonomously as:

$$\mathbf{r}(t) = \tanh\!\bigl(W^{eff}\mathbf{r}(t-1)\bigr), \quad z(t) = \mathbf{w}^{out\top}\mathbf{r}(t).$$

Note: the recurrent weight matrix $W^{eff} = W + W^{int}$ has been modified — the internal weights $W^{int}$ are a *rank-$N$* correction (not rank-1 as in FORCE). This is why Full-FORCE is more powerful but also more computationally expensive.

## 11.3.4 Why Full-FORCE Is More Stable

The key stability property of Full-FORCE is the **self-consistency** of the training procedure: the network is trained on states $\mathbf{r}^{tgt}(t)$ that were generated by a network driven by the true target $f(t)$, and the internal weights $W^{int}$ are trained to make those states self-consistent (the network should generate those states autonomously). This means that after training, the autonomous network dynamics are close to the target dynamics — not just in the output, but in the *internal states*.

By contrast, standard FORCE trains $\mathbf{w}^{out}$ to match the output on states generated during teacher-forced training, but does not constrain the internal dynamics to be self-consistent. When teacher forcing is removed, the states drift.

**Theorem (informal, DePasquale et al.):** If Full-FORCE converges (errors $e_j(t) \to 0$ and $e^{out}(t) \to 0$), then the autonomous network generates states $\{\mathbf{r}(t)\}$ that are close to $\{\mathbf{r}^{tgt}(t)\}$, and the output $z(t)$ is close to $f(t)$.

The convergence requires the same chaos suppression condition as standard FORCE, but is empirically more reliable — Full-FORCE succeeds in a larger portion of the parameter space.

## 11.3.5 Computational Cost

Full-FORCE trains $N+1$ sets of weights ($N$ internal, 1 output), each using RLS with an $N \times N$ matrix $P_j$. The total cost per training step is:

$$\text{Cost per step} = (N+1) \times O(N^2) = O(N^3).$$

Compared to standard FORCE's $O(N^2)$ per step. For $N = 1000$: standard FORCE costs $10^6$ operations/step; Full-FORCE costs $10^9$ operations/step. Full-FORCE is orders of magnitude more expensive and is practical only for modest $N$ (typically $N \leq 300$ with current hardware).

A practical compromise: train only a subset of the internal weights using FORCE, applying the correction only to the most influential neurons (those with large $|e_j|$ during target-network simulation). This reduces the cost while retaining most of the stability benefit.

## 11.3.6 Summary: FORCE vs. Full-FORCE

| Property | FORCE | Full-FORCE |
|----------|-------|------------|
| What is trained | Output weights $\mathbf{w}^{out}$ | Output weights + internal weights $W^{int}$ |
| Training feedback | Current output $z(t)$ | True target $f(t)$ (target network) |
| Self-consistency | Not enforced | Enforced by internal weight training |
| Autonomous stability | Often fails on complex patterns | More robust |
| Computational cost | $O(TN^2)$ | $O(TN^3)$ |
| Reservoir modification | $W$ unchanged | $W \to W + W^{int}$ (modified) |

---

*FORCE and Full-FORCE show that learning in recurrent networks can go beyond the readout. The next chapter considers a very different approach to flexibility: Jaeger's conceptors, which allow a fixed reservoir to store, recall, and compose multiple learned patterns.*
