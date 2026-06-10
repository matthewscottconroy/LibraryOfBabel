# Section 2.2: Tsodyks-Markram Synapse Model

## 2.6 Why Static Synapses Are Not Enough

In the simplest LSM model (as in the ESN), synaptic weights are fixed: a spike from neuron $j$ always delivers the same amount of current to neuron $i$, regardless of the history of $j$'s firing. This is the **static synapse** assumption.

Real biological synapses do not work this way. They are **dynamical**: the amount of neurotransmitter released by a presynaptic spike, and hence the postsynaptic current it generates, depends on the history of the synapse. Specifically:

- If neuron $j$ has been firing rapidly, the presynaptic terminals tend to become depleted of releasable neurotransmitter. Subsequent spikes from $j$ release less transmitter and produce smaller postsynaptic currents. This is **synaptic depression**.

- In some synapses, the residual Ca$^{2+}$ from previous spikes accumulates at the presynaptic terminal, increasing the probability of neurotransmitter release for subsequent spikes. This means rapid firing leads to *larger* postsynaptic currents. This is **synaptic facilitation**.

Depending on the synapse type and the firing history, a synapse can exhibit one or the other, or a mixture of both. The dominance of facilitation or depression depends on the presynaptic neuron type (excitatory vs. inhibitory, cortical layer) and the synapse target.

These synaptic dynamics are not merely biological details — they add significant computational richness to the liquid. A liquid with dynamic synapses has effectively more state: the synaptic variables $u$ and $R$ (defined below) are themselves dynamical, adding to the dimensionality of the liquid state and extending its temporal memory in complex ways.

---

## 2.7 The Tsodyks-Markram Model: Derivation and Variables

The Tsodyks-Markram (TM) model [TsodykMarkram1997] is a phenomenological model of short-term synaptic plasticity. It models the synapse with two variables:

- $R(t) \in [0, 1]$: the fraction of synaptic resources (neurotransmitter vesicles) currently **available** for release. $R = 1$ means fully recovered (maximal resources); $R < 1$ means partially depleted.
- $u(t) \in [0, 1]$: the **utilization factor** — the fraction of available resources that will be released by the next spike. This is related to the release probability at the synapse.

**Between spikes**, both variables relax toward their resting values:

$$\frac{dR}{dt} = \frac{1 - R}{\tau_D} \tag{2.11}$$

$$\frac{du}{dt} = \frac{U - u}{\tau_F} \tag{2.12}$$

where:
- $\tau_D > 0$ is the **depression time constant** (recovery time of vesicle replenishment), typically hundreds of milliseconds to seconds.
- $\tau_F > 0$ is the **facilitation time constant** (decay time of Ca$^{2+}$ accumulation effect), also typically tens to hundreds of ms.
- $U \in (0, 1]$ is the **baseline utilization factor** (resting release probability).

At resting state (no firing): $R \to 1$ and $u \to U$.

**At each spike** from the presynaptic neuron (at time $t_k$), the state jumps discontinuously:

**Step 1: Facilitation.** The utilization factor jumps up (Ca$^{2+}$ influx increases release probability):
$$u \to u^- + U(1 - u^-) \tag{2.13}$$

where $u^-$ is the value just before the spike. The increment $U(1 - u^-)$ saturates $u$ toward 1 as spikes accumulate.

**Step 2: Depletion.** A fraction $u$ of the available resources is consumed:
$$R \to R^- - u \cdot R^- = R^-(1 - u) \tag{2.14}$$

Note that the depletion uses the *updated* $u$ (after facilitation).

**Step 3: Postsynaptic current.** The postsynaptic current pulse has amplitude:
$$A_{syn} = w \cdot u \cdot R^- \tag{2.15}$$

where $w$ is the maximum synaptic weight. The actual amplitude depends on both the utilization $u$ (how much of the available is released) and the availability $R^-$ (how much is available). After the spike, $R$ decreases by $u \cdot R^-$ (the amount released).

---

## 2.8 Physical Interpretation of the Variables

Let us interpret each variable physically.

**$R(t)$: Resource availability.** Think of the synaptic terminal as a warehouse with a maximum capacity of $W$ neurotransmitter vesicles. $R(t)$ is the fraction of this capacity that is currently stocked. When a spike arrives, $u \cdot R$ fraction of the warehouse is shipped (released into the synapse). The warehouse then slowly restocks at rate $1/\tau_D$.

For a highly depressing synapse ($\tau_D$ large, $\tau_F$ small): after rapid firing, $R$ decreases substantially because the warehouse is depleted faster than it can restock. The synapse becomes progressively weaker during high-frequency firing — it acts as a **high-pass filter** on the spike train (only the first few spikes in a burst produce strong postsynaptic responses).

**$u(t)$: Utilization factor / release probability.** This represents the fraction of available vesicles released per spike, which increases with recent firing history due to Ca$^{2+}$ accumulation. For a highly facilitating synapse ($\tau_F$ large, $\tau_D$ small): early spikes have low release probability ($u \approx U$, small), but as spikes accumulate, $u$ increases toward 1. The synapse becomes progressively stronger during a burst — it acts as a **low-pass filter** on the spike train (sustained firing produces increasingly strong responses).

**The product $u \cdot R$**: the actual released fraction, sometimes called the "effective weight" or "synaptic efficacy." This is what the postsynaptic neuron "sees." It combines both effects: whether there are resources to be released and whether they will be released.

---

## 2.9 The Four Canonical Parameter Regimes

The TM model spans a rich parameter space. The ratio $\tau_F/\tau_D$ and the baseline utilization $U$ determine the qualitative behavior:

**1. Depressing synapse** ($\tau_D \gg \tau_F$, $U$ moderate to large):
At rest: $u \approx U$ (moderate release probability), $R \approx 1$ (full resources).
During rapid firing: $R$ decreases rapidly (depletion), $u$ changes little (facilitation is fast, so $u$ quickly recovers). The net effect: progressively smaller postsynaptic currents.
**Computational role:** High-pass filtering. Sensitive to the onset of a burst, not its sustained rate.

**2. Facilitating synapse** ($\tau_F \gg \tau_D$, $U$ small):
At rest: $u \approx U \ll 1$ (low release probability), $R \approx 1$ (full resources).
During rapid firing: $u$ accumulates (facilitation), $R$ decreases but slowly (slow depletion). The net effect: progressively larger postsynaptic currents.
**Computational role:** Low-pass filtering. Sensitive to sustained high-frequency input.

**3. Balanced synapse** ($\tau_F \approx \tau_D$):
Intermediate behavior; complex non-monotone response to firing rate.

**4. Static synapse** ($\tau_D = \tau_F = 0$ or effectively both much smaller than the signal timescale):
$u \approx U$ and $R \approx 1$ always; $A_{syn} \approx wU = const$. Reduces to the fixed-weight model.

---

## 2.10 Why Dynamic Synapses Enrich Liquid Dynamics

The TM model adds $2N \cdot N_c$ state variables to the liquid ($u$ and $R$ for each of the $N_c$ synaptic connections), dramatically increasing the state dimension. But more importantly, it changes the *character* of the dynamics.

**Extended memory.** A depressing synapse effectively "remembers" recent firing history for a time $\tau_D$ (via the depletion state $R$). A facilitating synapse remembers via $u$ for time $\tau_F$. Both time constants are independent of the membrane time constant $\tau_m$, adding separate timescales to the liquid. A liquid with both types has temporal memory at multiple timescales.

**Nonlinear mixing.** The product $u \cdot R$ is intrinsically nonlinear in the spike history. Even if each individual spike has a linear effect on the postsynaptic neuron, the synaptic efficacy $u \cdot R$ is a nonlinear function of the recent spike train, creating complex, nonlinear temporal features.

**Selective amplification.** Different neurons in the liquid may have different synapse types. Excitatory-to-excitatory connections in the cortex tend to be depressing (at high firing rates), while excitatory-to-inhibitory connections tend to be facilitating [Thomson1997]. This heterogeneity means that the liquid automatically applies different temporal filters to different parts of the circuit, creating a rich diversity of temporal representations.

---

## 2.11 Full Mathematical Specification

The complete TM model for a connection from presynaptic neuron $j$ to postsynaptic neuron $i$ is:

**Between spikes** (for $t$ not equal to any spike time of $j$):
$$\frac{dR_{ij}}{dt} = \frac{1 - R_{ij}}{\tau_{D,ij}} \tag{2.16a}$$
$$\frac{du_{ij}}{dt} = \frac{U_{ij} - u_{ij}}{\tau_{F,ij}} \tag{2.16b}$$

**At the $k$-th spike of neuron $j$** (at time $t_j^{(k)}$):
$$u_{ij} \leftarrow u_{ij}^- + U_{ij}(1 - u_{ij}^-) \tag{2.17a}$$
$$R_{ij} \leftarrow R_{ij}^-(1 - u_{ij}) \tag{2.17b}$$

(Note: (2.17a) is applied first, then (2.17b) uses the updated $u_{ij}$.)

**Postsynaptic current:**
$$I_{ij}(t) = w_{ij} \cdot u_{ij} \cdot R_{ij}^- \cdot \kappa(t - t_j^{(k)}) \tag{2.18}$$

where $\kappa(t) = e^{-t/\tau_{s,ij}} \cdot \mathbf{1}[t \geq 0]$ is the synaptic kernel with synaptic time constant $\tau_{s,ij}$.

The total current into neuron $i$ from the network:
$$I_i^{rec}(t) = \sum_{j \neq i} I_{ij}(t) \tag{2.19}$$

Combined with the external input current $I_i^{ext}(t)$, the full neuron dynamics are given by (2.10a) with $I_i(t) = I_i^{rec}(t) + I_i^{ext}(t)$.

---

## 2.12 Parameter Values for Cortically Realistic LSM

For simulations intended to be biologically realistic (e.g., matching the parameters of the Maass et al. 2002 paper), typical values are:

**Membrane parameters:**
- $C_m = 300$ pF (excitatory neurons), $200$ pF (inhibitory)
- $R_m = 1/0.03$ M$\Omega$ (excitatory), $1/0.05$ M$\Omega$ (inhibitory) — giving $\tau_m = 10$-$30$ ms
- $V_{th} = -50$ mV, $V_{reset} = V_{rest} = -65$ mV
- $\tau_{ref} = 3$ ms (excitatory), $2$ ms (inhibitory)

**Synapse parameters** (from [Maass2002]):
- Excitatory-to-excitatory (E→E): $U = 0.5$, $\tau_D = 1100$ ms, $\tau_F = 50$ ms (depressing)
- Excitatory-to-inhibitory (E→I): $U = 0.05$, $\tau_D = 125$ ms, $\tau_F = 1200$ ms (facilitating)
- Inhibitory-to-excitatory (I→E): $U = 0.25$, $\tau_D = 700$ ms, $\tau_F = 20$ ms (depressing)
- Inhibitory-to-inhibitory (I→I): $U = 0.32$, $\tau_D = 144$ ms, $\tau_F = 60$ ms (weak depression)

The striking asymmetry between E→E (strongly depressing) and E→I (strongly facilitating) connections has important consequences for network dynamics: strong excitatory input tends to strongly activate inhibitory neurons (via facilitation) but weakly excite other excitatory neurons in sustained fashion (via depression). This creates a natural self-regulating excitation-inhibition balance.

---

## 2.13 Computational Cost

Simulating a TM-LSM is significantly more expensive than simulating an ESN. For each of the $N_c = pN^2$ synaptic connections, we maintain two additional state variables ($R$ and $u$). For $N = 100$ neurons with $p = 0.2$ connectivity, $N_c = 2000$, and we track $4000$ additional variables per time step. With a simulation time step of $\Delta t = 0.1$ ms and a simulation duration of $T = 1$ s, the simulation requires $10^4$ steps and roughly $10^4 \times 4000 = 4 \times 10^7$ updates.

For larger networks ($N = 1000$, $p = 0.1$, $N_c = 10^5$), the cost becomes $10^9$ operations per second of simulation time — feasible on modern hardware but not trivial.

The ESN, by contrast, requires $O(N^2)$ operations per time step with step sizes of $\Delta t = 1$ (in units of the ESN's discrete time), making it typically 100x-1000x faster for equivalent reservoir sizes. This computational advantage is a major practical reason for preferring ESNs when biological realism is not required.
