# 16.1.4 Semiconductor Laser Neuron Rate Equations

The physical intuition of the previous subsections — a fast field, a slow carrier reservoir, a threshold, a refractory recovery — becomes precise once we write the coupled rate equations and study their fixed points. This subsection derives the excitable structure and its mapping to FitzHugh–Nagumo, and shows with a short linear-stability calculation where the boundary between excitable and self-pulsating behavior lies.

## The coupled field–carrier equations

In a normalized form (time measured in units of the photon lifetime $\tau_p$), the slowly varying complex field amplitude $E$ and the carrier number $N$ of a semiconductor laser obey

$$\dot{E} = \tfrac{1}{2}(G-1)E + F_E, \qquad \dot{N} = \frac{I}{e} - \frac{N}{\tau_s} - G\,|E|^2, \tag{1}$$

with the optical gain a linear function of carrier number above transparency,

$$G = g\,(N - N_{tr}). \tag{2}$$

Here $I$ is the pump current, $e$ the electron charge, $\tau_s$ the carrier lifetime, $N_{tr}$ the transparency carrier number, $g$ the differential gain, and $F_E$ a Langevin noise term representing spontaneous emission. The gain has been normalized so that the lasing threshold is $G = 1$: when $G < 1$ the round-trip gain cannot overcome the cavity loss and the field decays; when $G>1$ it grows. The noise $F_E$ matters physically because it provides the small random kicks that occasionally push a poised, near-threshold laser across its firing threshold, but for the deterministic stability analysis below we set $F_E = 0$.

## Fast and slow: the origin of excitability

The two lifetimes differ by three orders of magnitude: $\tau_p \sim 1\text{–}10\ \text{ps}$ for the photons versus $\tau_s \sim 1\ \text{ns}$ for the carriers. In the normalized time of (1) this means the field $E$ (or its intensity $|E|^2$) is a **fast** variable while $N$ is a **slow** variable. This is exactly the structure of the FitzHugh–Nagumo (FHN) model,

$$\dot{v} = v - \tfrac{1}{3}v^3 - w + I_{\text{ext}}, \qquad \dot{w} = \varepsilon\,(v + a - b\,w), \tag{3}$$

with a fast activator $v$ and a slow recovery $w$ ($\varepsilon \ll 1$). The optical intensity plays the role of the fast activator $v$; the carrier number (or, in a saturable-absorber device, the absorber variable) plays the role of the slow recovery $w$. The cubic-like nonlinearity of FHN is supplied physically by gain saturation and by the absorber's bleaching. When the fast/slow separation is large, a super-threshold kick sends the fast variable on a long stereotyped loop through phase space — the spike — while the slow variable barely moves, and only afterward does the slow variable recover, enforcing the refractory period. This is why lasers with the timescale hierarchy of (1) are natural FHN neurons.

## Fixed points

Write the intensity $P = |E|^2$. From (1), $\dot{P} = (G-1)P$ (deterministic). Setting $\dot P = 0$ and $\dot N = 0$ gives two fixed points.

The **lasing (on) state** has $P^* \neq 0$, which forces $G^*=1$; by (2) the carrier number clamps at $N^* = N_{tr} + 1/g$, and the carrier balance gives $P^* = I/e - N^*/\tau_s > 0$.

The **quiescent (off) state** has $P^*=0$, so $N^* = I\tau_s/e$ and $G^* = g(I\tau_s/e - N_{tr})$. This is the resting state of the neuron; it is stable precisely when the pump sits below threshold, $G^* < 1$.

## Linear stability: the Jacobian

Linearize the deterministic system $\dot P = (g(N-N_{tr})-1)P$, $\dot N = I/e - N/\tau_s - g(N-N_{tr})P$ about a fixed point. The Jacobian is

$$J = \begin{pmatrix} \dfrac{\partial \dot P}{\partial P} & \dfrac{\partial \dot P}{\partial N} \\[2mm] \dfrac{\partial \dot N}{\partial P} & \dfrac{\partial \dot N}{\partial N} \end{pmatrix} = \begin{pmatrix} G-1 & gP \\[1mm] -G & -\dfrac{1}{\tau_s} - gP \end{pmatrix}. \tag{4}$$

**At the lasing state** ($G^*=1$, $P^*>0$) this becomes

$$J_{\text{on}} = \begin{pmatrix} 0 & gP^* \\ -1 & -\left(\tfrac{1}{\tau_s} + gP^*\right) \end{pmatrix}, \qquad \operatorname{tr}J_{\text{on}} = -\left(\tfrac{1}{\tau_s} + gP^*\right) < 0, \quad \det J_{\text{on}} = gP^* > 0.$$

The eigenvalues are $\lambda_\pm = \tfrac{1}{2}\!\left(\operatorname{tr}J \pm \sqrt{(\operatorname{tr}J)^2 - 4\det J}\right)$. Because the discriminant $(\tfrac{1}{\tau_s}+gP^*)^2 - 4gP^*$ is negative in the well-above-transparency regime, the eigenvalues form a **complex-conjugate pair**

$$\lambda_\pm = \underbrace{-\tfrac{1}{2}\!\left(\tfrac{1}{\tau_s}+gP^*\right)}_{\text{Re}\,\lambda\;=\;\frac{1}{2}\operatorname{tr}J} \;\pm\; i\,\omega_{\text{RO}}, \qquad \omega_{\text{RO}} \approx \sqrt{gP^*}. \tag{5}$$

The imaginary part $\omega_{\text{RO}}$ is the **relaxation-oscillation frequency**: perturb the lasing laser and the intensity rings at $\omega_{\text{RO}}$ while the ring decays. Restoring physical units, $\omega_{\text{RO}}$ scales as $1/\sqrt{\tau_p\tau_s}$, i.e. the characteristic pulse time is the geometric mean $\sqrt{\tau_p\tau_s}$ — the same $\sim\!45\ \text{ps}$ estimate obtained in §16.1.1.

## The excitable–oscillatory boundary is a sign condition on $\text{Re}\,\lambda$

Equation (5) is the crux. The **real part of the eigenvalue pair equals half the trace of the Jacobian**:

$$\boxed{\;\operatorname{Re}\lambda_\pm = \tfrac{1}{2}\operatorname{tr}J\;}$$

so the qualitative behavior is controlled by the *sign* of the trace:

- $\operatorname{tr}J < 0 \;\Rightarrow\; \operatorname{Re}\lambda < 0$: the rest state is a **stable focus**. Perturbations spiral back to rest; a large enough kick first makes one big excursion (a spike). This is the **excitable** regime.
- $\operatorname{tr}J > 0 \;\Rightarrow\; \operatorname{Re}\lambda > 0$: the rest state is **unstable**; trajectories spiral out onto a limit cycle. The laser fires periodically with no input — the **self-pulsating / oscillatory** regime.
- $\operatorname{tr}J = 0 \;\Rightarrow\; \operatorname{Re}\lambda = 0$: the eigenvalues are purely imaginary. This is a **Hopf bifurcation**, the exact boundary between the two behaviors.

For the bare two-variable field–carrier system, $\operatorname{tr}J_{\text{on}} = -(1/\tau_s + gP^*)$ is always negative, so this model only ever gives *damped* relaxation oscillations — it is on the excitable/quiescent side but cannot spontaneously pulse. Genuine, tunable excitability requires a **third, slow variable** — a saturable-absorber loss $Q$ (the Yamada model of §16.1.1), an injected-field phase, or the free-carrier/thermal shift of a microring (§16.1.3). Such a variable contributes a *positive* term to the trace; as a control parameter (pump current, absorber recovery rate, injection detuning) is increased, this positive term grows until it cancels the damping. At that point $\operatorname{tr}J = 0$: the system crosses the Hopf boundary, $\operatorname{Re}\lambda$ changes sign, and the laser passes from the excitable single-pulse regime into self-sustained pulsation. A well-designed laser neuron is biased **just on the excitable side** of this Hopf boundary, where the rest state is stable, noise rarely triggers spurious spikes, and a deliberate super-threshold input evokes exactly one clean pulse.

## Worked Example: locating the boundary

Take the schematic trace $\operatorname{tr}J = \gamma_Q - (1/\tau_s + gP^*)$, where $\gamma_Q>0$ is the effective destabilizing rate contributed by the slow absorber/injection variable. Excitability requires $\operatorname{Re}\lambda = \tfrac12\operatorname{tr}J < 0$, i.e.

$$\gamma_Q < \frac{1}{\tau_s} + gP^*.$$

Using representative normalized values $\tau_s = 10^3$ (in units of $\tau_p$) and $gP^* = 5\times10^{-3}$, the damping is $1/\tau_s + gP^* = 10^{-3} + 5\times10^{-3} = 6\times10^{-3}$. The device is excitable for $\gamma_Q < 6\times10^{-3}$ and self-pulsating for $\gamma_Q > 6\times10^{-3}$; the Hopf boundary sits at $\gamma_Q = 6\times10^{-3}$. The algebra is schematic, but the structure is exact: **the neuron's operating regime is set by the sign of the real part of a complex-conjugate eigenvalue pair, and the useful excitable window is the stable side immediately below a Hopf bifurcation.**

## References

- Prucnal, P.R., Shastri, B.J., Ferreira de Lima, T., Nahmias, M.A. & Tait, A.N. (2016). "Recent progress in semiconductor excitable lasers for photonic spike processing." *Advances in Optics and Photonics*, 8(2), 228–299.
- Nahmias, M.A., Shastri, B.J., Tait, A.N. & Prucnal, P.R. (2013). "A leaky integrate-and-fire laser neuron for ultrafast cognitive computing." *IEEE J. Sel. Top. Quantum Electron.*, 19(5), 1800212.
- FitzHugh, R. (1961). "Impulses and physiological states in theoretical models of nerve membrane." *Biophysical Journal*, 1(6), 445–466.
- Nagumo, J., Arimoto, S. & Yoshizawa, S. (1962). "An active pulse transmission line simulating nerve axon." *Proceedings of the IRE*, 50(10), 2061–2070.
