# Section 3: The Three Conditions for LSM Computation

## 3.1 Overview

The computational theory of the LSM rests on three conditions that a liquid network must satisfy. These conditions are analogous to the conditions needed for a universal function approximator: just as a single-hidden-layer network with enough neurons can approximate any continuous function (universal approximation), a liquid satisfying the three conditions can approximate any continuous, causal, time-invariant functional with fading memory.

The three conditions are:
1. **The Separation Property (SP):** Different input histories produce different liquid states.
2. **The Approximation Property (AP):** Any smooth function of the liquid state can be computed by a memoryless readout.
3. **The Fading Memory Property (FMP):** Input histories that agree on recent events produce similar liquid states.

Together, they form a sufficient condition for the LSM to be a universal approximator of the class of target computations that a brain-inspired computing system would typically need to perform.

---

## 3.2 Setting Up: Input Streams and Liquid States

We formalize the setting. Let $\mathcal{U} \subset \mathbb{R}^K$ be the (compact) input space and let $\mathbf{u} = (\ldots, u_{-2}, u_{-1}, u_0, u_1, \ldots) \in \mathcal{U}^{\mathbb{Z}}$ be a bi-infinite input stream.

A **liquid** is a dynamical system that maps input streams to liquid state trajectories. Given an input stream $\mathbf{u}$, the liquid at time $t$ produces a state $L_M(\mathbf{u})(t) \in \mathbb{R}^N$ — the "surface of the pond" at time $t$.

We assume the liquid satisfies the **echo state property** (i.e., its state at time $t$ depends only on the semi-infinite past input $u_{(-\infty, t]}$ and not on the initial condition). So $L_M(\mathbf{u})(t)$ is a well-defined functional of the input history.

A **readout** is a map $f : \mathbb{R}^N \to \mathbb{R}^L$ from the liquid state to an output. For LSMs (as for ESNs), the readout is typically trained to be linear: $f(x) = W^{out} x$.

The **target computation** is a target functional $\mathcal{T} : \mathcal{U}^{\mathbb{Z}} \to \mathcal{Y}^{\mathbb{Z}}$ that we want the LSM to approximate. We assume $\mathcal{T}$ is causal, time-invariant, and continuous.

---

## 3.3 The Separation Property

**Definition 3.1 (Separation Property).** A liquid $M$ has the **separation property** on input class $\mathcal{C} \subseteq \mathcal{U}^{\mathbb{Z}}$ if for any two distinct input streams $\mathbf{u} \neq \mathbf{u}' \in \mathcal{C}$, there exists a time $t$ such that:

$$L_M(\mathbf{u})(t) \neq L_M(\mathbf{u}')(t) \tag{3.1}$$

In words: distinct inputs produce distinct liquid states at some time $t$. The liquid can, in principle, distinguish between them.

**Why this matters.** The separation property is the minimum requirement for computation. If two different input histories produce the same liquid state, then no readout — linear or not — can distinguish them, and hence no readout can compute any function that distinguishes them. The liquid must be a sufficient statistic for the relevant input differences.

**The separation property is not the ESP.** The ESP requires convergence of two trajectories with the *same* input and *different* initial conditions. The SP requires distinguishability of two trajectories with *different* inputs and the *same* (or arbitrary) initial conditions. These are different conditions, though both relate to the sensitivity of the liquid to its history.

**Quantifying separation.** The **kernel quality** (Maass et al. 2002) measures how well a liquid satisfies the SP. Given a set of input streams $\{\mathbf{u}^{(1)}, \ldots, \mathbf{u}^{(M)}\}$, the kernel quality is essentially the discriminability of the corresponding liquid states:

$$\kappa = \text{rank}\left(\begin{pmatrix} L_M(\mathbf{u}^{(1)})(t)^\top \\ \vdots \\ L_M(\mathbf{u}^{(M)})(t)^\top \end{pmatrix}\right) \tag{3.2}$$

High rank = high separation = many inputs can be distinguished. Full rank ($= \min(N, M)$) means all $M$ inputs produce linearly independent states — perfect separation.

---

## 3.4 The Approximation Property

**Definition 3.2 (Approximation Property).** A liquid $M$ has the **approximation property** (with respect to a class $\mathcal{F}$ of readout functions) if the set of functions of the form $f \circ L_M$ (where $f \in \mathcal{F}$) is dense in the target function class $\mathcal{T}$.

In the most common formulation, $\mathcal{F}$ is the class of continuous functions $f : \mathbb{R}^N \to \mathbb{R}^L$. The condition is that the image of the liquid map — the set of all possible liquid states — is "rich enough" that continuous functions of it can approximate any target functional.

**Practical interpretation.** For a finite-dimensional liquid (i.e., $x_t \in \mathbb{R}^N$), the AP means: the set of liquid states $\{x_t : \mathbf{u} \in \mathcal{C}\}$ generates a rich enough distribution that any continuous readout function can be realized. For a linear readout, this requires the liquid states to span a large subspace of $\mathbb{R}^N$ — i.e., the state matrix $X$ should have high rank.

**Connection to the separation property.** If the separation property holds, the map $\mathbf{u} \mapsto L_M(\mathbf{u})(t)$ is injective (one-to-one). A one-to-one continuous map on a compact domain has a continuous inverse (by invariance of domain), meaning the liquid state uniquely encodes the input history and any continuous function of the history can be expressed as a continuous function of the state. This is the sense in which SP implies AP: injectivity implies that the function $g(\mathbf{u}) = f(L_M(\mathbf{u}))$ can represent any $g$ if $f$ is unrestricted.

For linear readouts specifically, the approximation property requires the liquid states to span the relevant subspace — a weaker condition than injectivity.

---

## 3.5 The Fading Memory Property

**Definition 3.3 (Fading Memory Property).** A liquid $M$ has the **fading memory property** if for any $\epsilon > 0$, there exists $T > 0$ and $\delta > 0$ such that for any two input streams $\mathbf{u}, \mathbf{u}'$ with $\|u_s - u_s'\| < \delta$ for all $s \in [t-T, t]$ (i.e., they agree approximately on the recent past of length $T$), then:

$$\|L_M(\mathbf{u})(t) - L_M(\mathbf{u}')(t)\| < \epsilon \tag{3.3}$$

In words: if two input streams are close over the recent past of length $T$, their liquid states at time $t$ are close. The parameter $T$ is the effective "memory length" — input differences older than $T$ do not matter.

**Equivalence to ESP.** The fading memory property for the liquid is essentially equivalent to the echo state property: the ESP says the liquid forgets its initial conditions, and the FMP says it forgets distant-past inputs. Formally, both are implied by the reservoir being a uniform contraction (Section 2.2 of Chapter 5). The difference is that ESP talks about trajectories from different *initial conditions* driven by the *same* input, while FMP talks about trajectories from the *same* initial condition driven by *different* inputs. For a contractive liquid, both decay exponentially.

---

## 3.6 The LSM Computation Theorem

With the three conditions defined, we can state the main result. The following is a version of the theorem from [Maass2002], adapted for clarity.

**Theorem 3.1 (LSM Computation Theorem; Maass, Natschläger, Markram 2002).**

Let $M$ be a liquid (e.g., a randomly connected network of LIF neurons with TM synapses) that satisfies the Separation Property, the Approximation Property, and the Fading Memory Property. Let $\mathcal{T}$ be any continuous, causal, time-invariant functional from the input stream space $\mathcal{C}$ to outputs, with the fading memory property.

Then for any $\epsilon > 0$, there exists a memoryless readout function $f : \mathbb{R}^N \to \mathbb{R}^L$ such that:

$$\sup_{t, \mathbf{u} \in \mathcal{C}} \|f(L_M(\mathbf{u})(t)) - \mathcal{T}(\mathbf{u})(t)\| < \epsilon \tag{3.4}$$

That is, the LSM (liquid plus readout) can approximate any such target functional to arbitrary accuracy.

**What "generic" means.** The word "generic" in the original theorem statement refers to the fact that the theorem holds for *almost all* random liquid configurations, in the sense that the set of configurations that fail to satisfy the three conditions has measure zero (with respect to the natural probability measure on random networks). You do not need to design the liquid carefully; a random liquid will work with probability one.

This is a remarkable claim. It says that the specific wiring of the liquid does not matter — as long as the liquid is "generic" (which is almost certainly the case for random networks), it will satisfy the three conditions. The only thing that matters is the readout, which is trained to approximate the target.

---

## 3.7 Proof Outline

The proof of Theorem 3.1 follows a Stone-Weierstrass argument, analogous to the universal approximation theorem for feedforward networks. We sketch the main steps.

**Step 1: The target functional has fading memory.**
By assumption, $\mathcal{T}$ has the fading memory property: for any $\epsilon > 0$, there exists $T$ such that input differences older than $T$ do not affect $\mathcal{T}$'s output by more than $\epsilon$. This means $\mathcal{T}$ is effectively a continuous function of the finite-dimensional "recent input" $u_{t-T+1}, \ldots, u_t \in \mathcal{U}^T$.

**Step 2: The liquid has fading memory.**
By the FMP, the liquid state $x_t = L_M(\mathbf{u})(t)$ is also effectively determined by the recent input $u_{t-T+1}, \ldots, u_t$ (for $T$ large enough). So both $x_t$ and $\mathcal{T}(\mathbf{u})(t)$ can be treated as functions of the finite-dimensional input $\phi_t = (u_{t-T+1}, \ldots, u_t) \in \mathcal{U}^T$.

**Step 3: The separation property gives injectivity.**
The SP says the map $\phi_t \mapsto x_t$ is injective on $\mathcal{U}^T$. By the invariance of domain theorem (since $\mathcal{U}^T$ is compact), this map has a continuous inverse.

**Step 4: Approximate the target by a function of the liquid state.**
We want to approximate $g : \phi_t \mapsto \mathcal{T}(\mathbf{u})(t)$ by $f(x_t)$ for some continuous $f$. Since $\phi_t \mapsto x_t$ is a continuous injection with continuous inverse $h : x_t \mapsto \phi_t$, we can write $g(\phi_t) = g(h(x_t))$. The composition $f = g \circ h$ is a continuous function of $x_t$ that exactly represents the target.

**Step 5: Use the AP to realize $f$ as a readout.**
The AP says that any continuous function of $x_t$ can be approximated (in $L^2$ norm) by a function in the readout class $\mathcal{F}$. For a polynomial readout (finite-order polynomial in the components of $x_t$), the Stone-Weierstrass theorem guarantees that any continuous function on a compact set can be approximated. For a linear readout (the standard ESN/LSM case), the AP requires the target $f$ to be linear in $x_t$, which is generally not exactly achievable but holds approximately when the liquid provides rich enough nonlinear features.

**Combining:** For any $\epsilon > 0$, we can choose $T$ large enough (Step 1-2), and the readout $f$ close enough to $g \circ h$ (Step 5), to get $\|f(x_t) - \mathcal{T}(\mathbf{u})(t)\| < \epsilon$. $\square$

---

## 3.8 Discussion: What the Theorem Means and Does Not Mean

**What it means:**
The theorem establishes that the LSM is a **universal computational architecture** for the class of computations with fading memory. You do not need to train the recurrent connections — only the readout. The randomly connected spiking network, for all its biological disorder, is computationally sufficient.

**What it does not mean:**

**(a) Efficiency:** The theorem is an existence result. It says a readout *exists* that achieves $\epsilon$ accuracy, but it does not say how large the liquid must be, or how much training data is needed. In practice, harder tasks require larger liquids, and the theorem gives no bound on the required size.

**(b) Linear readout is sufficient.** The proof requires a continuous (possibly nonlinear) readout $f$. For a linear readout, we need the target functional to be *approximately* linear in the liquid state — which is true when the liquid provides a rich enough nonlinear feature space. If the target is highly nonlinear and the liquid is small, a linear readout may not suffice. In practice, this is handled by using larger liquids.

**(c) Random liquids are always good.** The theorem applies to liquids that satisfy the three conditions. Not every random network satisfies them — a network that is too weakly connected (ordered phase) may fail the SP, and a network that is too strongly connected (chaotic phase) may fail the FMP. The theorem is most relevant when the network is near the edge of chaos.

**(d) The theorem applies to a specific input class.** The three conditions are defined with respect to a specific input class $\mathcal{C}$. A liquid may be computationally adequate for one input class and inadequate for another.

The upshot is that the LSM theorem is a theoretical foundation that justifies the reservoir computing architecture without over-specifying the implementation. It leaves ample room for both biological realization (spiking networks) and efficient engineering (ESNs).
