# 15.1.1 The Leaky Integrate-and-Fire Model

## From membrane biophysics to a circuit

A neuron's membrane is a thin lipid bilayer separating charged ions. It stores charge like a **capacitor** ($C_m$) and leaks charge through ion channels like a **resistor** ($R_m$). The simplest faithful model of the soma is therefore an RC circuit driven by the synaptic input current $I(t)$. Charge conservation (Kirchhoff's current law) at the membrane node states that the injected current splits between charging the capacitor and leaking through the resistance:

$$C_m \frac{dV}{dt} + \frac{V - V_{rest}}{R_m} = I(t)$$

Multiplying by $R_m$ and defining the **membrane time constant** $\tau_m \equiv R_m C_m$ gives the **leaky integrate-and-fire (LIF)** equation:

$$\tau_m \frac{dV}{dt} = -(V - V_{rest}) + R_m\, I(t) \tag{1}$$

The $-(V - V_{rest})$ term is the *leak*: with no input, $V$ relaxes exponentially to the resting potential $V_{rest}$ with time constant $\tau_m$. The $R_m I$ term is the *integrate*: input current drives $V$ upward (or downward, for inhibitory input). Equation (1) captures sub-threshold dynamics only; it says nothing about the spike itself.

The spike is imposed as a **threshold-and-reset** rule. When $V(t)$ reaches the threshold $V_{th}$, the model declares a spike at that instant and resets the voltage to $V_{reset}$, optionally holding it there for an absolute refractory period $t_{ref}$ before integration resumes:

$$\text{if } V \ge V_{th}: \quad \text{emit spike}, \quad V \rightarrow V_{reset} \tag{2}$$

This is a deliberate caricature. The full biophysics — the regenerative opening of voltage-gated Na$^+$ and K$^+$ channels that shapes the action potential — was described by **Hodgkin and Huxley (1952)** with four coupled nonlinear differential equations. LIF discards that machinery, keeping only the fact that a spike is all-or-nothing and stereotyped, so its exact shape carries no information and can be replaced by an event. The integrate-and-fire idea predates Hodgkin–Huxley entirely: **Lapicque (1907)** modeled the neuron as exactly this kind of capacitive threshold device.

## Sub-threshold solution and the f–I curve

For a constant input current $I$, equation (1) is linear and solvable. Its steady state is

$$V_\infty = V_{rest} + R_m I,$$

and starting from $V_{reset}$ the voltage rises as

$$V(t) = V_\infty + (V_{reset} - V_\infty)\, e^{-t/\tau_m}. \tag{3}$$

Two regimes follow. If $V_\infty \le V_{th}$ — that is, if $R_m I \le V_{th} - V_{rest}$ — the voltage asymptotes below threshold and the neuron **never fires**. The boundary current

$$I_{rh} = \frac{V_{th} - V_{rest}}{R_m}$$

is the **rheobase**, the minimum constant current that elicits firing. If $I > I_{rh}$, the neuron fires periodically. Setting $V(t_{th}) = V_{th}$ in (3) and solving for the time to threshold:

$$t_{th} = \tau_m \ln\!\left(\frac{V_\infty - V_{reset}}{V_\infty - V_{th}}\right). \tag{4}$$

Including the refractory period, the steady firing rate is $f = 1/(t_{ref} + t_{th})$, the neuron's **f–I curve**:

$$\boxed{\,f(I) = \left[\,t_{ref} + \tau_m \ln\!\left(\frac{R_m I + V_{rest} - V_{reset}}{R_m I + V_{rest} - V_{th}}\right)\right]^{-1}, \quad I > I_{rh}\,}$$

The rate is zero below rheobase, rises steeply just above it, and saturates toward $1/t_{ref}$ at large $I$.

## Worked Example: will it fire, and how fast?

Take biologically representative parameters: $\tau_m = 10$ ms, membrane resistance $R_m = 100\ \text{M}\Omega$ (hence $C_m = \tau_m/R_m = 100$ pF), $V_{rest} = V_{reset} = -70$ mV, $V_{th} = -55$ mV, and $t_{ref} = 2$ ms.

**Rheobase.** The gap to threshold is $V_{th} - V_{rest} = 15$ mV, so

$$I_{rh} = \frac{15\ \text{mV}}{100\ \text{M}\Omega} = 0.15\ \text{nA}.$$

**Sub-threshold case.** For $I = 0.10$ nA, $R_m I = (100\ \text{M}\Omega)(0.10\ \text{nA}) = 10$ mV, so $V_\infty = -70 + 10 = -60$ mV, which is below $V_{th}$. The membrane charges toward $-60$ mV and stops — **no spike**.

**Supra-threshold case.** For $I = 0.20$ nA, $R_m I = 20$ mV and $V_\infty = -50$ mV, above $V_{th}$. From (4),

$$t_{th} = 10\ \text{ms} \times \ln\!\left(\frac{-50-(-70)}{-50-(-55)}\right) = 10\ \text{ms} \times \ln\!\left(\frac{20}{5}\right) = 10\ \text{ms} \times \ln 4 = 13.9\ \text{ms}.$$

Adding the 2 ms refractory period, the steady firing rate is

$$f = \frac{1}{t_{ref} + t_{th}} = \frac{1}{(2 + 13.9)\ \text{ms}} = \frac{1}{15.9\ \text{ms}} \approx 63\ \text{Hz}.$$

So this neuron is silent at 0.10 nA and fires a regular ~63 Hz train at 0.20 nA — squarely within the biological range, whose maximum is a few hundred hertz.

## Why LIF suits hardware

LIF reduces a neuron to three operations — leak, integrate, threshold-reset — each of which maps onto a compact physical primitive. In electronics it is a capacitor, a resistor, and a comparator. In photonics, as Chapter 16 develops, an excitable semiconductor laser biased just below threshold performs the same computation: the carrier population integrates injected optical energy and leaks on its recombination time, and the laser emits one all-or-nothing optical pulse when the gain crosses lasing threshold — a physical realization of equations (1)–(2) operating some eight orders of magnitude faster than its biological namesake. This dynamical equivalence between a spiking neuron and an excitable laser is what makes the LIF model the natural bridge from neuroscience to neuromorphic photonics.

## References

Lapicque, L. (1907). "Recherches quantitatives sur l'excitation électrique des nerfs." *J. Physiol. Pathol. Gen.*, 9, 620–635.

Hodgkin, A.L. & Huxley, A.F. (1952). "A quantitative description of membrane current and its application to conduction and excitation in nerve." *Journal of Physiology*, 117(4), 500–544.

Gerstner, W. & Kistler, W.M. (2002). *Spiking Neuron Models: Single Neurons, Populations, Plasticity*. Cambridge University Press.
