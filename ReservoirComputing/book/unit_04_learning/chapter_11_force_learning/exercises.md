# Chapter 11 Exercises

## Analytical Exercises

**Exercise 11.1 (RLS derivation — rank-1 update).**
Derive the rank-1 update formula for $P(t)$ from first principles.

(a) Write $P(t)^{-1} = P(t-1)^{-1} + \mathbf{r}(t)\mathbf{r}(t)^\top$. Apply the Sherman-Morrison formula $(A + \mathbf{u}\mathbf{v}^\top)^{-1} = A^{-1} - \frac{A^{-1}\mathbf{u}\mathbf{v}^\top A^{-1}}{1 + \mathbf{v}^\top A^{-1}\mathbf{u}}$ with $A = P(t-1)^{-1}$, $\mathbf{u} = \mathbf{v} = \mathbf{r}(t)$ to derive $P(t) = P(t-1) - \frac{P(t-1)\mathbf{r}(t)\mathbf{r}(t)^\top P(t-1)}{1+\mathbf{r}(t)^\top P(t-1)\mathbf{r}(t)}$.

(b) Show that the gain vector $\mathbf{k}(t) = P(t-1)\mathbf{r}(t)/(1 + \mathbf{r}(t)^\top P(t-1)\mathbf{r}(t))$ satisfies $P(t)\mathbf{r}(t) = \mathbf{k}(t)$.

(c) Show that $P(t)$ remains symmetric and positive definite if $P(0)$ is symmetric and positive definite.

(d) What is the limiting behavior of $P(t)$ as $t \to \infty$? (Hint: as more samples are observed, $P(t)^{-1}$ accumulates the outer products $\sum_s \mathbf{r}(s)\mathbf{r}(s)^\top$, so $P(t) \to 0$.) What does this imply for the size of the weight updates $\Delta\mathbf{w}(t)$?

---

**Exercise 11.2 (FORCE as gradient descent).**
Standard gradient descent on the instantaneous squared error $\mathcal{L}_t = \frac{1}{2}e(t)^2 = \frac{1}{2}(\mathbf{w}^\top\mathbf{r}(t) - f(t))^2$ gives the update $\Delta\mathbf{w} = -\eta e(t)\mathbf{r}(t)$.

(a) Compare this to the FORCE update $\Delta\mathbf{w} = -e(t)\mathbf{k}(t) = -e(t) P(t-1)\mathbf{r}(t)/(1+\mathbf{r}^\top P(t-1)\mathbf{r})$. What is the effective learning rate of FORCE for each direction in weight space?

(b) Show that FORCE uses an adaptive learning rate: it scales the update in proportion to $P(t-1)\mathbf{r}(t)$, which emphasizes directions in weight space that are most uncertain (have not been recently constrained by data).

(c) For a linear reservoir (tanh replaced by identity), show that after $T$ steps of FORCE, the weights converge to the batch ridge regression solution $\mathbf{w}^* = (R^\top R + \lambda I)^{-1} R^\top \mathbf{f}$ with $\lambda = 1/P(0)_{ii}$.

(d) What is the "natural gradient" interpretation of the FORCE update? (Hint: the Fisher information matrix of the squared-error objective in a linear regression model is proportional to $R^\top R$, the inverse of which appears in the RLS solution.)

---

**Exercise 11.3 (Teacher forcing removal — instability analysis).**
Consider the simplest possible FORCE network: $N = 1$, $W = \{w\}$, $w^{fb} = 1$. The dynamics are $r(t) = \tanh(wr(t-1) + z(t-1))$.

(a) During teacher forcing, $z(t) \approx f(t)$. Analyze the stability of the dynamics near the target trajectory $r^*(t)$ (the state when $z(t) = f(t)$ exactly). Linearize around $r^*(t)$ to find the Jacobian.

(b) After teacher forcing removal, $z(t) = w^{out} r(t)$. Analyze the stability of the autonomous dynamics. For what values of $w^{out}$ and $w$ is the autonomous trajectory stable?

(c) Show that even if the FORCE training converged perfectly (making $z(t) \approx f(t)$ during training), the autonomous stability depends on both $w^{out}$ and $w$ in a way that is *not* controlled by the FORCE training objective. This is the teacher forcing removal problem.

(d) For the Full-FORCE solution: show that training the internal weight $w^{int}$ (so that $w \to w + w^{int}$) can be used to ensure autonomous stability, even if the naive FORCE solution is unstable.

---

## Thought Experiments

**Thought Experiment 11.1: Is FORCE still reservoir computing?**

(a) The strict definition: "reservoir computing = fixed reservoir + trained linear readout." FORCE trains a linear readout (the output weights), but the output is fed back. Does the feedback violate the "fixed reservoir" condition?

(b) The FORCE feedback modifies the reservoir's effective dynamics: the reservoir with feedback is governed by $r(t) = \tanh(Wr(t-1) + w^{fb}z(t-1))$, which is a *different dynamical system* from $r(t) = \tanh(Wr(t-1))$. In what sense is the reservoir "fixed"?

(c) Compare to Full-FORCE, which explicitly modifies $W \to W + W^{int}$. Is Full-FORCE more or less like "reservoir computing" than standard FORCE? Is there a continuous family of methods interpolating between the two?

(d) What would it mean for the reservoir computing paradigm if FORCE outperforms standard ESN training on all tasks? Would this imply that the fixed-reservoir paradigm is suboptimal and should be abandoned?

---

**Thought Experiment 11.2: FORCE and biological neural circuits.**

Sussillo and Abbott's original motivation was biological: they argued that FORCE could model how the motor cortex learns to generate complex rhythmic movements.

(a) In a biological neural circuit, "output weights" corresponds to the strengths of synapses from motor cortex neurons to motor neurons. Are these the only synapses that change during motor learning? What does this imply about the biological realism of FORCE?

(b) The RLS algorithm requires maintaining an $N \times N$ matrix $P(t)$ and updating it at every timestep. This is not biologically plausible for $N$ on the order of $10^{10}$ neurons. What local learning rule approximation of FORCE would be biologically plausible?

(c) Full-FORCE modifies the recurrent weights $W$. In the brain, recurrent synapses (connections within a cortical area) do change during learning. Does Full-FORCE provide a better model of biological motor learning than standard FORCE? What predictions does this suggest about the time course of synaptic change during skill acquisition?

---

## Key Concepts

**1. FORCE (First Order Reduced and Controlled Error) Learning**
An online training algorithm for recurrent networks that uses recursive least squares to update output weights while simultaneously feeding the output back into the network. The key innovation is training a chaotic ($\rho > 1$) recurrent network to produce complex autonomous patterns.

**2. Recursive Least Squares (RLS)**
An online algorithm for minimizing the running sum of squared errors. Maintains an estimate $P(t) \approx (\sum_s \mathbf{r}(s)\mathbf{r}(s)^\top + \lambda I)^{-1}$ and updates it via the rank-1 formula $P(t) = P(t-1) - \mathbf{k}(t)\mathbf{r}(t)^\top P(t-1)$ at $O(N^2)$ cost per step. Converges to the batch ridge regression solution for a stationary target.

**3. Gain Vector $\mathbf{k}(t)$**
The vector $\mathbf{k}(t) = P(t-1)\mathbf{r}(t)/(1 + \mathbf{r}(t)^\top P(t-1)\mathbf{r}(t))$. It represents the most uncertain direction in weight space given the current state, normalized by the state's self-correlation. The weight update $\Delta\mathbf{w} = -e(t)\mathbf{k}(t)$ applies the correction in the direction that most reduces uncertainty.

**4. Teacher Forcing**
During FORCE training, the true target signal $f(t)$ (or an approximation) is fed into the network as the feedback, replacing the network's own output. This stabilizes training but creates a mismatch between the training (teacher-forced) and test (autonomous) regimes.

**5. Teacher Forcing Removal Problem**
The instability that occurs when the teacher-forced target signal is replaced by the network's autonomous output at test time. The reservoir states during autonomous operation may diverge from the training-time states, causing performance degradation or failure.

**6. Full-FORCE**
An extension of FORCE that trains both output weights and internal weights to ensure self-consistency: the target network provides desired state trajectories, and the internal weights are trained so the network autonomously generates those trajectories. More stable than standard FORCE but $O(N)$ times more expensive.

**7. Chaos Suppression**
The mechanism by which FORCE tames a chaotic reservoir: the feedback $z(t) = \mathbf{w}^\top\mathbf{r}(t)$ adds a rank-1 term to the effective weight matrix, which can bring the effective spectral radius below 1 and stabilize the dynamics.

**8. Self-Consistency in Full-FORCE**
The property that the trained autonomous network generates state trajectories close to the target network's trajectories. Enforced by training internal weights $W^{int}$ such that $\tanh^{-1}(\mathbf{r}^{tgt}(t)) = (W + W^{int})\mathbf{r}^{tgt}(t-1)$ approximately. This is the key advantage of Full-FORCE over standard FORCE.

---

## Key Researchers

**David Sussillo** is a senior research scientist at Google DeepMind, formerly at the Salk Institute and Stanford. He developed FORCE with Larry Abbott [SussilloAbbott2009] and has continued to work on the computational principles of recurrent neural networks, particularly in the context of neuroscience.

**Larry Abbott** is a professor at Columbia University and one of the most influential theoretical neuroscientists. His work spans neural dynamics, synaptic plasticity, and learning algorithms inspired by biology. The FORCE paper is one of many contributions to understanding how biological neural circuits can learn complex temporal patterns.

**Brian DePasquale** developed Full-FORCE [DePasquale2018] as a more biologically motivated and technically robust extension. His work addressed the teacher forcing removal problem and demonstrated significantly improved stability for complex motor-cortex-inspired generation tasks.

---

## Further Reading

**Sussillo, D., & Abbott, L. F. (2009). Generating coherent patterns of activity from chaotic neural networks. *Neuron*, 63(4), 544–557.**
[SussilloAbbott2009]
The original FORCE paper. Essential reading.

**DePasquale, B., Cueva, C. J., Rajan, K., Escola, G. S., & Abbott, L. F. (2018). full-FORCE: A target-based method for training recurrent networks. *PLoS ONE*, 13(2), e0191527.**
[DePasquale2018]
The Full-FORCE paper. Clear exposition of the teacher forcing problem and the target network solution.

**Haykin, S. (2002). *Adaptive Filter Theory* (4th ed.). Prentice Hall.**
The standard reference on RLS algorithms, with rigorous convergence analysis.
