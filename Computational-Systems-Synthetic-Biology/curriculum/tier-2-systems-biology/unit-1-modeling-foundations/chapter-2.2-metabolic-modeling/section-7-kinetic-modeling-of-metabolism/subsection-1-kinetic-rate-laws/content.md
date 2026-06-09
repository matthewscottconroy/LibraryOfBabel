# Kinetic Rate Laws for Metabolic Reactions

## Beyond Stoichiometry

Steady-state FBA tells you the flux through every reaction at equilibrium growth. ¹³C MFA confirms those fluxes experimentally. But neither tells you what happens in the first 30 seconds after you switch the carbon source, or how glycolytic flux responds in the milliseconds after a sudden glucose pulse, or whether your engineered pathway will oscillate or converge. For those questions, you need to know not just how much flux moves but how fast it moves — and how that speed depends on metabolite concentrations. That is the domain of kinetic models.

Stoichiometric models (FBA) and ¹³C MFA tell us how much flux moves through each reaction at steady state, but they do not describe how fluxes change dynamically in response to perturbations. For that, we need **kinetic models**: mathematical descriptions of reaction rates as explicit functions of metabolite concentrations and enzyme parameters. Kinetic models can predict transient dynamics, response to genetic perturbations (enzyme level changes), and the consequences of allosteric regulation.

## The Michaelis-Menten Rate Law

The canonical kinetic rate law for an enzyme-catalyzed reaction $S \to P$ is:

$$v = \frac{k_{\text{cat}} [E] [S]}{K_M + [S]} = \frac{V_{\max} [S]}{K_M + [S]}$$

where:
- $V_{\max} = k_{\text{cat}} [E]_{\text{total}}$: maximum reaction velocity (depends on enzyme level)
- $K_M$: Michaelis constant — the substrate concentration at which $v = V_{\max}/2$; roughly the substrate affinity constant
- $k_{\text{cat}}$: catalytic rate constant (turnover number, units: s⁻¹)

At low $[S] \ll K_M$: $v \approx (V_{\max}/K_M)[S]$ — first-order kinetics  
At high $[S] \gg K_M$: $v \approx V_{\max}$ — zero-order (saturated) kinetics

## Reversible Michaelis-Menten

Most metabolic reactions are reversible under cellular conditions. The reversible Michaelis-Menten rate law:

$$v = \frac{V_f \frac{[S]}{K_S} - V_r \frac{[P]}{K_P}}{1 + \frac{[S]}{K_S} + \frac{[P]}{K_P}}$$

where $V_f, V_r$ are forward and reverse maximum velocities, and $K_S, K_P$ are Michaelis constants for substrate and product. The Haldane constraint ensures thermodynamic consistency:

$$\frac{V_f K_P}{V_r K_S} = K_{eq}$$

This constraint eliminates one free parameter — if we know $K_{eq}$ (from eQuilibrator), $K_S$, $K_P$, and $V_f$, then $V_r$ is determined.

## Multi-Substrate Rate Laws

Most metabolic reactions involve multiple substrates. For a bi-substrate reaction $A + B \to P + Q$, the common mechanisms produce different rate laws:

**Ordered bi-bi** (one substrate binds first):
$$v = \frac{V_f [A][B] - V_r [P][Q]/K_{eq}}{K_{iA} K_B + K_B [A] + K_A [B] + [A][B] + \ldots}$$

**Random bi-bi** (either substrate can bind first):
$$v = \frac{V_f \frac{[A][B]}{K_A K_B}}{(1 + \frac{[A]}{K_A})(1 + \frac{[B]}{K_B}) + \ldots}$$

In practice, distinguishing mechanisms experimentally requires extensive kinetic data. For computational modeling, simplified rate laws that capture the essential nonlinearities are often preferred.

## Allosteric Regulation

Many metabolic enzymes are allosterically regulated — their activity is modulated by metabolites that bind at sites distinct from the active site. Common regulatory patterns:

**Competitive inhibition** (inhibitor competes with substrate):
$$v = \frac{V_{\max} [S]}{K_M (1 + [I]/K_I) + [S]}$$

**Uncompetitive inhibition** (inhibitor binds only to enzyme-substrate complex):
$$v = \frac{V_{\max} [S]}{K_M + [S](1 + [I]/K_I)}$$

**Hill kinetics** (cooperative allosteric activation or inhibition):
$$v = V_{\max} \cdot \frac{[S]^n}{K_{0.5}^n + [S]^n} \cdot \frac{K_I^m}{K_I^m + [I]^m}$$

where $n$ is the Hill coefficient for cooperativity and $m$ for inhibition steepness.

**Example — phosphofructokinase-1 (PFK-1)**: The key regulatory enzyme of glycolysis is activated by AMP and ADP, and inhibited by ATP and citrate. This creates a feedback mechanism: when energy charge is high (high ATP/AMP ratio), PFK-1 is inhibited, slowing glucose consumption. The rate law for PFK-1 involves multiple allosteric terms and is one of the more complex in central metabolism.

## Modular Rate Laws

For large-scale kinetic models where detailed mechanisms are unknown, **modular rate laws** (Liebermeister & Klipp 2006) provide a systematic, thermodynamically consistent framework:

$$v_j = E_j \cdot k_j^+ \cdot \prod_i \left(\frac{[S_i]}{K_{Mi}}\right)^{|s_{ij}^-|} \cdot (1 - e^{\Delta_r G'_j / RT}) \cdot \text{regulation terms}$$

This form factors the rate into:
- **Enzyme concentration**: $E_j$ (allows direct coupling to proteomics)
- **Thermodynamic factor**: $(1 - e^{\Delta_r G'_j/RT})$ — ensures $v=0$ at equilibrium
- **Saturation factor**: product of scaled substrate concentrations
- **Regulation factor**: allosteric modifiers

Modular rate laws reduce the number of parameters while maintaining thermodynamic consistency and correct limiting behavior.

## A Worked Example: Glycolytic Model

A minimal ODE model for the PGI-PFK segment:

```python
import numpy as np
from scipy.integrate import solve_ivp

def glycolysis_segment(t, y, params):
    G6P, F6P, FBP, ATP, ADP = y
    Vf_PGI, Vr_PGI, KS_PGI, KP_PGI = params['PGI']
    Vf_PFK, KS_PFK, KI_ATP, KA_AMP = params['PFK']
    AMP = 5e-3 - ATP - ADP  # adenylate conservation
    
    # PGI: G6P ⇌ F6P (reversible MM)
    v_PGI = (Vf_PGI * G6P/KS_PGI - Vr_PGI * F6P/KP_PGI) / \
            (1 + G6P/KS_PGI + F6P/KP_PGI)
    
    # PFK: F6P + ATP → FBP + ADP (allosteric inhibition by ATP)
    v_PFK = Vf_PFK * F6P/(KS_PFK + F6P) * \
            KI_ATP**2/(KI_ATP**2 + ATP**2)  # sigmoidal ATP inhibition
    
    dG6P = -v_PGI
    dF6P = v_PGI - v_PFK
    dFBP = v_PFK
    dATP = -v_PFK
    dADP = v_PFK
    return [dG6P, dF6P, dFBP, dATP, dADP]

# Initial conditions and integration
y0 = [2e-3, 0.4e-3, 0.1e-3, 3e-3, 0.5e-3]  # molar concentrations
params = {'PGI': (10, 5, 0.5e-3, 0.3e-3), 'PFK': (5, 0.1e-3, 1e-3, 0.1e-3)}
sol = solve_ivp(glycolysis_segment, [0, 100], y0, args=(params,), 
                dense_output=True, method='Radau')
```

## Why This Matters

Kinetic rate laws are the mechanistic foundation of dynamic metabolic models. While FBA is powerful for steady-state analysis, kinetic models are required whenever time-dependent responses matter: predicting metabolic shifts after nutrient switches, understanding glycolytic oscillations, or designing feed strategies for bioreactors. Mastering the standard rate law forms and their parameterization provides the vocabulary for constructing mechanistic models of cellular metabolism.
