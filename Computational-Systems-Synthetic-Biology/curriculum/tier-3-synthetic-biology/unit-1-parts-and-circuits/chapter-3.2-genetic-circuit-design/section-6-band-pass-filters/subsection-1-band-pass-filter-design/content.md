# Band-Pass Filter Design: Responding Only to Intermediate Signal Levels

Copper is essential for life — dozens of bacterial enzymes require it as a cofactor. But copper is also toxic at high concentrations, poisoning the same enzymatic machinery it normally supports. A bacterium living in soil does not need a circuit that responds whenever copper is present; it needs a circuit that says "copper is here, but not too much." Too little copper means something is wrong; too much means something worse is wrong. What you want is a response that is high in the middle of the concentration range and suppressed at both extremes. You want, in other words, a band-pass filter.

A **band-pass filter** in electronics transmits signals within a specific frequency range while attenuating signals outside that range. In synthetic biology, the analog is a circuit that produces high output only when its input (typically a molecular concentration) falls within a defined range — too low or too high, and the output is suppressed. This concentration-band selectivity is valuable for detecting "normal" levels of a metabolite, responding only to physiologically relevant stimulus concentrations, or discriminating between different disease states based on biomarker levels.

## The Design Principle: Two Thresholds

A band-pass filter requires two thresholds:
- **Lower threshold** ($K_{low}$): input must exceed this to produce output
- **Upper threshold** ($K_{high}$): input above this suppresses output

**Output**: high when $K_{low} <$ Input $< K_{high}$; low otherwise.

The simplest implementation uses two regulatory interactions:
1. An **activating** pathway with threshold $K_{low}$: input activates an activator that drives output
2. A **repressing** pathway with threshold $K_{high}$: high input activates a repressor that inhibits output

## Two-Stage Implementation

**Stage 1 (Low-pass activation)**:
$$A(\text{Input}) = \frac{\alpha_{act} \cdot (\text{Input}/K_{low})^n}{1 + (\text{Input}/K_{low})^n}$$

Activator A is produced in response to Input with threshold $K_{low}$.

**Stage 2 (High-pass repression)**:
$$R(\text{Input}) = \frac{\alpha_{rep} \cdot (\text{Input}/K_{high})^m}{1 + (\text{Input}/K_{high})^m}$$

Repressor R is produced in response to Input with threshold $K_{high} > K_{low}$.

**Output**:
$$\text{Output} = \frac{\alpha_{out} \cdot A}{1 + (R/K_R)^p}$$

The output is activated by A (rises when Input > $K_{low}$) and repressed by R (falls when Input > $K_{high}$). For Input in the band, A is high and R is still low → output is high. For Input above the band, R rises and suppresses output.

```python
import numpy as np
import matplotlib.pyplot as plt

def band_pass_output(input_conc, alpha_act=5, K_low=1, n=2,
                     alpha_rep=5, K_high=10, m=2,
                     alpha_out=10, K_R=1, p=2):
    A = alpha_act * (input_conc/K_low)**n / (1 + (input_conc/K_low)**n)
    R = alpha_rep * (input_conc/K_high)**m / (1 + (input_conc/K_high)**m)
    output = alpha_out * A / (1 + (R/K_R)**p)
    return output

input_range = np.logspace(-2, 3, 500)
output = band_pass_output(input_range)

plt.figure(figsize=(8, 4))
plt.semilogx(input_range, output)
plt.xlabel('Input concentration (au)')
plt.ylabel('Output (au)')
plt.title('Band-Pass Filter Transfer Function')
plt.axvline(1, color='g', linestyle='--', label='K_low')
plt.axvline(10, color='r', linestyle='--', label='K_high')
plt.legend()
plt.show()
```

## Biological Implementation: Molecular Components

### In Bacteria: Two TF Circuits

A practical implementation uses two different transcription factors from characterized libraries:

**Low-threshold activator (A)**: an AHL quorum sensing receptor (e.g., LuxR) that activates a reporter promoter at low AHL concentrations (nM range).

**High-threshold repressor (R)**: a different AHL receptor (e.g., CinR) that at higher concentrations activates expression of a repressor, which then silences the reporter.

**Key design constraint**: $K_{high}/K_{low}$ sets the width of the pass-band. The ratio must be at least 5–10-fold for a clear band-pass response; ratios of 50–100-fold give a narrow, sharp band.

### In Mammalian Cells: Transcription Factor Cascades

In mammalian cells, a two-TF implementation is more complex because:
- Eukaryotic promoters require specific upstream activation sequences
- The activator and repressor must compete at the same promoter

A common approach uses:
- Low threshold: input induces expression of an activator via Tet-On system (low [Dox] activates)
- High threshold: input induces expression of a KRAB-repressor fusion via a different sensitive promoter (high [Dox] activates KRAB)
- Both target the same synthetic promoter: KRAB-mediated chromatin silencing at high doses overrides activator-driven expression

## Case Study: Detecting a "Goldilocks" Range of Copper in *E. coli*

Copper is essential for enzyme function but toxic at high concentrations. A band-pass filter for copper:

**Low threshold**: CueR (Cu-activated) drives expression of GFP at Cu > 1 µM
**High threshold**: CueR at > 100 µM Cu also activates copA (copper ATPase); when CopA expression is linked to a LacI-TetR circuit that represses GFP, cells in high Cu have GFP OFF

**Pass-band**: 1–100 µM Cu → GFP ON; outside this range → GFP OFF

**Application**: environmental biosensor for copper contamination that signals a specific concentration range (too low = insufficient for detection; too high = hazardous).

## Design Variants: Ratiometric Band-Pass

Rather than absolute concentration, some applications require detecting a *ratio* between two signals. For example, a cell might need to respond to a cytokine only when the ratio of pro-inflammatory signal A to anti-inflammatory signal B falls within a specific range.

Ratiometric band-pass uses division-type logic:
- Output is activated when A/B > threshold₁
- Output is repressed when A/B > threshold₂

Implementation typically uses a **ratiometric synthetic gene circuit** where A and B control opposing regulatory elements (activator and repressor) of the output, so the output level reflects the ratio rather than either signal alone.

## Bandwidth and Selectivity

The sharpness of a band-pass filter is quantified by its **selectivity** (analogous to Q-factor in electronics):

$$Q = \frac{\text{Center concentration}}{K_{high} - K_{low}}$$

For a pass-band between 5 and 50 µM (center = 25 µM, bandwidth = 45 µM): Q = 25/45 ≈ 0.55 (broad filter).
For a pass-band between 8 and 12 µM (center = 10 µM, bandwidth = 4 µM): Q = 10/4 = 2.5 (narrow filter).

Higher Q requires:
- Steeper Hill functions (larger n, m)
- Better-matched activation and repression thresholds
- Lower noise in protein expression (to prevent false positives near thresholds)

## Why This Matters

Band-pass filters represent a fundamentally more sophisticated input-output relationship than simple activation or repression. In therapeutic applications, they enable cells to respond to disease-specific concentrations of a biomarker while ignoring both background (low) and toxic (high) levels. In metabolic engineering, a band-pass sensor can detect when a pathway intermediate is at optimal concentration and throttle upstream flux automatically. In environmental monitoring, band-pass biosensors can distinguish "acceptable" pollutant concentrations from both absence (no contamination to detect) and excess (beyond detection range). The design principle — combining activating and repressing pathways with different thresholds — is general and can be adapted to any pair of regulatory elements with sufficiently separated K values.
