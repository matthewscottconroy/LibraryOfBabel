# The Goldbeter-Leloup Circadian Model

## A Landmark Quantitative Model

If you wanted a single example to convince a skeptic that mathematical modeling is genuinely useful in biology — not just a formal description of what we already know, but a tool that makes real predictions — the Goldbeter-Leloup model of the circadian clock would be a strong candidate. This is a model that correctly predicts the phenotypes of genetic mutations before they were fully characterized, explains how a clock mechanism can remain stable across a 10°C temperature range despite the temperature sensitivity of every individual reaction, and captures the shape of the phase response curve from mechanistic first principles.

The **Goldbeter-Leloup model** (Leloup & Goldbeter 2003, 2004) is a detailed kinetic ODE model of the mammalian circadian clock, encompassing 16–19 ordinary differential equations and approximately 70 parameters. It was developed by Albert Goldbeter's group to move beyond the original Drosophila clock models to a mammalian system incorporating both PER/CRY negative feedback and the REV-ERB/BMAL1 secondary loop.

The model is a landmark in computational biology because it:
- Predicts oscillations with a period close to 24 hours from known molecular mechanisms
- Explains how mutations in clock genes alter period (e.g., CK1ε tau mutation, FASPS)
- Captures light entrainment and the phase response curve (PRC)
- Predicts temperature compensation
- Is available in BioModels Database (BIOMD0000000073) for direct reproduction

## Model Structure

The 16-ODE model tracks the following species:

**PER cycle (7 variables):**
- $M_P$: Per mRNA
- $P_0$: unphosphorylated PER protein (cytoplasmic)
- $P_1$: singly phosphorylated PER
- $P_2$: doubly phosphorylated PER (cytoplasmic)
- $P_N$: nuclear PER (in PER:CRY complex)
- $C$: CRY protein (simplified; tracks CRY level)
- $PC_N$: nuclear PER:CRY complex

**CRY cycle (4 variables):** similar phosphorylation cascade

**BMAL1 cycle (5 variables):**
- $M_B$: Bmal1 mRNA
- $B_0, B_1, B_2$: phosphorylated forms of BMAL1
- $A_N$: nuclear CLOCK:BMAL1 complex

## Core Equations (Abbreviated)

The Per mRNA dynamics:

$$\frac{dM_P}{dt} = v_{sP} \frac{K_{IP}^{n_P}}{K_{IP}^{n_P} + A_N^{n_P}} - v_{mP} \frac{M_P}{K_{mP} + M_P}$$

Activation by CLOCK:BMAL1 ($A_N$) is repressed with Hill coefficient $n_P$ (cooperativity). The mRNA is degraded at rate $v_{mP}$ following Michaelis-Menten kinetics.

PER protein phosphorylation cascade:

$$\frac{dP_0}{dt} = k_{sP} M_P - V_1 \frac{P_0}{K_1 + P_0} + V_2 \frac{P_1}{K_2 + P_1}$$
$$\frac{dP_1}{dt} = V_1 \frac{P_0}{K_1 + P_0} - V_2 \frac{P_1}{K_2 + P_1} - V_3 \frac{P_1}{K_3 + P_1} + V_4 \frac{P_2}{K_4 + P_2}$$
$$\frac{dP_2}{dt} = V_3 \frac{P_1}{K_3 + P_1} - V_4 \frac{P_2}{K_4 + P_2} - k_d P_2 - k_3 P_2 C + k_4 PC_N$$

Each phosphorylation step uses zero-order Michaelis-Menten kinetics, enabling Goldbeter-Koshland switch-like behavior.

## Simulating the Goldbeter-Leloup Model

```python
import numpy as np
from scipy.integrate import solve_ivp

def goldbeter_leloup(t, y, params):
    """
    Simplified version of Goldbeter-Leloup circadian model (7 ODEs for illustration).
    Full 16-ODE model available from BioModels (BIOMD0000000073).
    """
    MP, P0, P1, P2, MB, B0, AN = y
    
    # Unpack key parameters
    vsP, KIP, nP = params['vsP'], params['KIP'], params['nP']
    vmP, KmP = params['vmP'], params['KmP']
    ksP = params['ksP']
    V1, K1, V2, K2 = params['V1'], params['K1'], params['V2'], params['K2']
    V3, K3, kd = params['V3'], params['K3'], params['kd']
    vsB, KAB, nB = params['vsB'], params['KAB'], params['nB']
    vmB, KmB, ksB = params['vmB'], params['KmB'], params['ksB']
    
    # Per mRNA: repressed by nuclear CLOCK:BMAL1 (AN) via PER:CRY inhibition
    dMP = vsP * (KIP**nP / (KIP**nP + P2**nP)) - vmP * MP / (KmP + MP)
    
    # PER protein phosphorylation
    dP0 = ksP * MP - V1 * P0 / (K1 + P0) + V2 * P1 / (K2 + P1)
    dP1 = V1 * P0 / (K1 + P0) - V2 * P1 / (K2 + P1) - V3 * P1 / (K3 + P1)
    dP2 = V3 * P1 / (K3 + P1) - kd * P2  # P2 = nuclear/active PER
    
    # BMAL1 mRNA: activated by AN (CLOCK:BMAL1) 
    dMB = vsB * (AN**nB / (KAB**nB + AN**nB)) - vmB * MB / (KmB + MB)
    dB0 = ksB * MB - 0.3 * B0  # simplified BMAL1 activation
    dAN = 0.3 * B0 - 0.1 * AN - P2 * AN  # nuclear complex formation/inhibition
    
    return [dMP, dP0, dP1, dP2, dMB, dB0, dAN]

# Reference parameter set (simplified for illustration)
params = {
    'vsP': 1.0, 'KIP': 1.0, 'nP': 4, 'vmP': 0.7, 'KmP': 0.2,
    'ksP': 0.9, 'V1': 3.0, 'K1': 0.15, 'V2': 1.5, 'K2': 0.15,
    'V3': 1.0, 'K3': 0.15, 'kd': 0.3,
    'vsB': 1.0, 'KAB': 0.5, 'nB': 2, 'vmB': 0.6, 'KmB': 0.2,
    'ksB': 0.8
}

y0 = [0.5, 0.5, 0.2, 0.2, 0.5, 0.5, 0.5]
sol = solve_ivp(goldbeter_leloup, [0, 200], y0, args=(params,),
                dense_output=True, method='Radau', rtol=1e-8)

# Extract period from oscillation of Per mRNA
from scipy.signal import find_peaks
peaks, _ = find_peaks(sol.y[0])
if len(peaks) > 1:
    periods = np.diff(sol.t[peaks])
    print(f"Estimated period: {np.mean(periods):.1f} hours")
```

## Key Predictions of the Model

**Period sensitivity to phosphorylation kinetics**: Reducing $V_1$ (CK1 phosphorylation rate of $P_0 \to P_1$) lengthens period; increasing it shortens period. This matches the tau mutation phenotype in CK1ε (shorter period) and FASPS (faster phosphorylation, shorter period, phase advance).

**Amplitude-period coupling**: The secondary REV-ERB loop in the full 16-ODE model increases the amplitude of oscillation without changing the period significantly — demonstrating how the interlocking loop architecture provides robustness.

**Temperature compensation**: by appropriate parameter relationships ($Q_{10}$ values that cancel between synthesis and degradation terms), the model produces period-insensitive oscillations over a 10°C range — one of the most remarkable properties of circadian clocks.

**Phase response curve (PRC)**: Adding a light input term that degrades CRY proteins (light activates CRY proteasomal degradation via F-box proteins) produces a PRC with correct shape: delay in early night, advance in late night, no phase shift during the day.

## Where the Model Succeeds and Where It Is Simplified

The model accurately captures: period, PRC shape, response to KO of individual clock genes (Per1/2, Cry1/2, Bmal1 knockouts), and the role of the secondary loop in robustness.

Simplifications: detailed spatial compartmentalization of PER/CRY complexes, the exact CK1 phosphorylation mechanism (recent data shows it is more complex than depicted), and population-level coupling between SCN neurons are not included.

The honest assessment of where a model fails is as important as understanding what it gets right. Models are not truth — they are productive simplifications. The Goldbeter-Leloup model successfully reproduces the system-level behavior of the clock (period, robustness, entrainment) while sacrificing molecular detail. That tradeoff was consciously made and has proven to be well-calibrated.

## Why This Matters

The Goldbeter-Leloup model demonstrates that ODE-based kinetic modeling can capture the essential dynamics of a complex, multi-component biological oscillator — predicting genetic perturbation phenotypes and environmental responses from known biochemical parameters. It is a template for how detailed mechanistic models should be constructed: starting from known mechanisms, parameterized from available biochemical data, and validated against independent phenotypic measurements.
