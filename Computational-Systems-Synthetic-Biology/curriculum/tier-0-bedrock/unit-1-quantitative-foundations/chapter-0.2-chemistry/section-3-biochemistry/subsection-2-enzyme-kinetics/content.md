# Enzyme Kinetics

Somewhere in your liver right now, a molecule of ethanol is being converted to acetaldehyde by alcohol dehydrogenase. The enzyme catalyzes this oxidation by binding the ethanol molecule in a precise orientation, positioning it next to a molecule of NAD$^+$, and orchestrating the transfer of a hydride ion. Then it releases the acetaldehyde and gets ready for the next substrate molecule. How many times per second can it do this? Under what conditions is it working at full capacity? How does the blood alcohol concentration in your portal vein affect its speed?

These are the questions of enzyme kinetics. It is the quantitative study of how fast enzymes catalyze reactions and how this rate depends on substrate concentration, inhibitor presence, and environmental conditions. The Michaelis-Menten equation is one of the most important equations in biochemistry and systems biology — it appears in virtually every ODE model of a metabolic or gene regulatory network. Understanding where it comes from, what its parameters mean, and when it breaks down is foundational for the entire modeling enterprise in this curriculum.

## The Michaelis-Menten Model

The Michaelis-Menten mechanism posits:

$$E + S \underset{k_{-1}}{\overset{k_1}{\rightleftharpoons}} ES \overset{k_{\text{cat}}}{\longrightarrow} E + P$$

where $E$ is free enzyme, $S$ is substrate, $ES$ is the enzyme-substrate complex, and $P$ is product.

**The quasi-steady-state assumption (QSSA, Briggs-Haldane):** Under the condition that $[E]_{\text{total}} \ll [S]$, the complex $ES$ reaches a pseudo-steady state rapidly: $d[ES]/dt \approx 0$.

From $d[ES]/dt = k_1[E][S] - (k_{-1} + k_{\text{cat}})[ES] = 0$:

$$[ES] = \frac{[E][S]}{K_m}, \quad K_m = \frac{k_{-1} + k_{\text{cat}}}{k_1}$$

Using conservation $[E]_T = [E] + [ES]$:

$$v = k_{\text{cat}}[ES] = \frac{k_{\text{cat}}[E]_T [S]}{K_m + [S]} = \frac{V_{\text{max}}[S]}{K_m + [S]}$$

This is the **Michaelis-Menten equation**, where $V_{\text{max}} = k_{\text{cat}} [E]_T$.

**Parameters:**
- **$K_m$** (Michaelis constant): substrate concentration at half-maximal velocity. Approximately equal to the substrate-enzyme dissociation constant when $k_{\text{cat}} \ll k_{-1}$ (the original Michaelis-Menten assumption). Typical values: $10^{-6}$ to $10^{-2}$ M.
- **$V_{\text{max}}$**: maximum velocity achieved at saturating substrate; proportional to enzyme concentration.
- **$k_{\text{cat}}$** (turnover number): reactions per enzyme molecule per second. Typical values: $1 - 10^7$ s$^{-1}$.
- **$k_{\text{cat}}/K_m$** (specificity constant): the best measure of catalytic efficiency; limited by diffusion at $\sim 10^8 - 10^9$ M$^{-1}$s$^{-1}$.

**Validity conditions for QSSA:** $[E]_T \ll K_m + [S]$. Violated in single-molecule experiments or when enzyme concentration approaches substrate concentration.

The specificity constant $k_{\text{cat}}/K_m$ deserves special attention. It measures how well an enzyme performs when substrate is limiting — when it's in the linear part of the curve. An enzyme with $k_{\text{cat}}/K_m$ near $10^9$ M$^{-1}$s$^{-1}$ is said to be "diffusion-limited": it processes every substrate molecule it encounters, limited only by how fast diffusion can bring the two together. Enzymes like catalase and triosephosphate isomerase achieve this. They are, in the evolutionary sense, perfect — they cannot be made faster.

## Enzyme Inhibition

**Competitive inhibition:** Inhibitor I binds the free enzyme (at or near the active site), competing with substrate:

$E + I \rightleftharpoons EI \quad (K_i = [E][I]/[EI])$

Effect: increases apparent $K_m$ by factor $(1 + [I]/K_i)$; $V_{\text{max}}$ is unchanged.

$$v = \frac{V_{\text{max}}[S]}{K_m(1 + [I]/K_i) + [S]}$$

Competitive inhibition is fully reversed by increasing substrate concentration.

**Uncompetitive inhibition:** Inhibitor binds only the ES complex:

$ES + I \rightleftharpoons ESI \quad (K_i' = [ES][I]/[ESI])$

Effect: both apparent $K_m$ and $V_{\text{max}}$ decrease by factor $(1 + [I]/K_i')$.

**Noncompetitive inhibition:** Inhibitor binds enzyme and ES complex equally ($K_i = K_i'$). $V_{\text{max}}$ decreases; $K_m$ unchanged.

**Mixed inhibition:** General case where $K_i \neq K_i'$ — both $K_m$ and $V_{\text{max}}$ are altered.

**Irreversible inhibition:** Inhibitor forms a covalent bond with the enzyme. Examples: aspirin acetylates COX (serine hydroxyl), penicillin acylates transpeptidase (serine hydroxyl), nerve agents phosphorylate acetylcholinesterase.

## Cooperativity and the Hill Equation

**Cooperative enzymes** (like ATCase, phosphofructokinase, hemoglobin) show sigmoidal kinetics — the relationship between velocity and substrate concentration is steeper than Michaelis-Menten:

$$v = \frac{V_{\text{max}} [S]^n}{K_{0.5}^n + [S]^n}$$

where $n$ is the **Hill coefficient** and $K_{0.5}$ is the half-saturation constant.

- $n = 1$: Michaelis-Menten (no cooperativity)
- $n > 1$: positive cooperativity (sigmoidal) — binding of substrate facilitates further binding
- $n < 1$: negative cooperativity — binding inhibits further binding

The Hill equation produces an **ultrasensitive** switch-like response. The **response coefficient** (how fold-change in substrate produces fold-change in rate) is steeper for higher $n$. For a 10-fold change in substrate: the range over which velocity goes from 10% to 90% of $V_{\text{max}}$ spans an 81-fold substrate concentration range for $n=1$, but only a 9-fold range for $n=2$, and a 3-fold range for $n=4$.

This ultrasensitivity is critically important in signaling cascades and gene expression: the Hill function provides an "all-or-none" switch, and the Hill coefficient measures the sharpness of the transition.

**MWC (Monod-Wyman-Changeux) model:** More mechanistically detailed than Hill. The enzyme exists in two states — T (tense, low affinity) and R (relaxed, high affinity) — in equilibrium. Substrate shifts the equilibrium toward R (positive cooperativity). Allosteric activators stabilize R; allosteric inhibitors stabilize T.

## Parameter Estimation

Modern enzyme kinetics uses **nonlinear least squares** to fit the Michaelis-Menten equation directly to $v$ vs. $[S]$ data. The historical **Lineweaver-Burk plot** (double reciprocal: $1/v$ vs. $1/[S]$) gives a linear relationship but amplifies error at low substrate concentrations and is now primarily used pedagogically.

```python
import numpy as np
from scipy.optimize import curve_fit
import matplotlib.pyplot as plt

def michaelis_menten(S, Vmax, Km):
    return Vmax * S / (Km + S)

def hill_equation(S, Vmax, K05, n):
    return Vmax * S**n / (K05**n + S**n)

# Simulate noisy enzyme kinetics data
np.random.seed(42)
Vmax_true, Km_true, n_true = 100, 5, 2
S_conc = np.array([0.5, 1, 2, 4, 8, 16, 32, 64])
v_mm = michaelis_menten(S_conc, Vmax_true, Km_true)
v_hill = hill_equation(S_conc, Vmax_true, Km_true, n_true)
noise = np.random.randn(len(S_conc)) * 3

# Fit MM
popt_mm, _ = curve_fit(michaelis_menten, S_conc, v_mm + noise,
                        p0=[80, 4], bounds=(0, [500, 100]))
print(f"MM fit: Vmax = {popt_mm[0]:.1f}, Km = {popt_mm[1]:.2f}")

# Fit Hill
popt_hill, _ = curve_fit(hill_equation, S_conc, v_hill + noise,
                          p0=[80, 4, 1.5], bounds=(0, [500, 100, 6]))
print(f"Hill fit: Vmax = {popt_hill[0]:.1f}, K0.5 = {popt_hill[1]:.2f}, n = {popt_hill[2]:.2f}")

# Plot
S_fine = np.linspace(0, 64, 200)
fig, ax = plt.subplots(figsize=(8, 5))
ax.scatter(S_conc, v_hill + noise, label='Data (Hill n=2)', zorder=5)
ax.plot(S_fine, michaelis_menten(S_fine, *popt_mm), label=f'MM fit (Km={popt_mm[1]:.1f})')
ax.plot(S_fine, hill_equation(S_fine, *popt_hill), label=f'Hill fit (n={popt_hill[2]:.1f})')
ax.axhline(Vmax_true, linestyle='--', color='gray', label='Vmax')
ax.set_xlabel('[S] (mM)'); ax.set_ylabel('v (nM/min)')
ax.set_title('Enzyme Kinetics Fitting'); ax.legend()
plt.tight_layout()
```

## Why This Matters for Computational Biology

The Michaelis-Menten equation is the rate law in virtually every ODE model of metabolic or gene regulatory networks. The Hill equation is the rate law for cooperative binding and gene expression (the Hill function approximates the fraction of promoter occupied by a cooperatively binding activator). Understanding which parameters ($K_m$, $V_{\text{max}}$, $n$) control the behavior of these rate laws — and how they are measured — is foundational for model construction, parameter estimation, and sensitivity analysis. The Hill coefficient $n$ is the single most important parameter in determining whether a gene expression response is graded or switch-like.
