# Quantitative Characterization Methods in Cell-Free Systems

Synthetic biology has long aspired to be an engineering discipline — to treat biological parts the way electrical engineers treat resistors and op-amps: interchangeable, predictable, and characterized by a small number of parameters on a datasheet. For that aspiration to be realized, someone has to actually measure those parameters with rigor. Cell-free systems offer a unique opportunity for quantitative characterization of biological parts — promoters, ribosome binding sites, transcription factors, and protein-protein interactions — under conditions where concentrations are known, inputs are controllable, and outputs are directly measurable without the confounding variables of cell growth. This section describes the primary quantitative characterization methods and the key parameters they measure, with emphasis on the units, experimental designs, and data interpretation that make cell-free characterization scientifically rigorous.

## Promoter Strength: PoPS (Polymerases Per Second)

**PoPS** (Polymerases Per Second) is a standardized unit for promoter activity that is intended to be comparable across different expression contexts and laboratories. The concept was introduced by the BioFAB group as part of the effort to define a "datasheet" standard for genetic parts.

**Definition**: PoPS is the rate at which RNA polymerase initiates transcription at a promoter, measured in molecules of RNAP per promoter per second.

**Measurement in cell-free**:
At low DNA concentrations (sub-nM range), protein output from a simple promoter-RBS-GFP construct is proportional to DNA concentration. This is because at low concentrations, not all promoters are occupied simultaneously — the rate-limiting step is promoter-RNAP encounter.

$$\frac{d[\text{GFP}]}{dt}\bigg|_{t=0} = \alpha_{PoPS} \times [\text{DNA}]$$

The slope of initial GFP production rate vs. DNA concentration gives $\alpha_{PoPS}$ in units of **REU (Relative Expression Units)/nM DNA/hour**, which can be converted to absolute PoPS with a calibration curve.

**Experimental protocol**:
1. Prepare a dilution series of the reporter DNA: 0.1, 0.3, 1, 3, 10 nM
2. Add each concentration to identical cell-free reactions (same extract batch, same energy supplement)
3. Monitor GFP fluorescence at 37°C for 4–8 hours
4. For each concentration, compute $d[\text{GFP}]/dt$ over the first 60 minutes (before resource depletion)
5. Plot initial rate vs. [DNA]; slope = $\alpha_{PoPS}$

**Critical controls**: the same DNA at the same concentrations should be tested in parallel with a known reference promoter (e.g., J23101 from the Anderson promoter library). PoPS is then reported relative to the reference promoter.

## RBS Strength: Translation Initiation Rate

**Ribosome Initiation Rate** (RIR) characterizes how efficiently a ribosome binding site initiates translation per mRNA molecule per unit time.

**Measurement strategy**:
Fix the promoter; vary the RBS sequence (e.g., use the Salis RBS Calculator to design RBS variants spanning 1000× range of predicted initiation rates). Measure protein output at each RBS.

Because promoter activity determines mRNA concentration and RBS efficiency determines translation rate per mRNA, the two contributions multiply:

$$[\text{Protein}] \propto \alpha_{PoPS} \times \alpha_{RIR} \times [\text{DNA}]$$

By keeping $\alpha_{PoPS}$ fixed (same promoter) and varying RIR, the relative translation initiation rate is directly proportional to relative protein output. This is confirmed by also measuring mRNA levels (by RT-qPCR or molecular beacon assay) to verify that promoter activity is constant across RBS variants.

**Salis RBS Calculator validation**: a standard use of cell-free RBS characterization is to validate or calibrate the Salis RBS Calculator predictions. Plot observed protein output vs. predicted RBS strength (in TIR, translation initiation rate units). Spearman correlation ρ > 0.85 indicates the calculator is predictive for the tested sequence context.

## Transcription Factor Transfer Functions

**Transfer function**: for a transcriptional repressor or activator, the transfer function relates the TF concentration (input) to the reporter output. The Hill function:

$$y = y_{min} + \frac{y_{max} - y_{min}}{1 + \left(\frac{[TF]}{K_{1/2}}\right)^n}$$

where $y_{max}$ is maximum expression (no repression), $y_{min}$ is minimum expression (full repression), $K_{1/2}$ is the TF concentration for half-maximal repression, and $n$ is the Hill coefficient (cooperativity).

**Cell-free transfer function measurement protocol**:
1. Prepare two DNA constructs: (a) reporter DNA (T7 promoter + RBS + GFP); (b) repressor DNA (T7 promoter + RBS + repressor gene)
2. Fix reporter DNA concentration (e.g., 2 nM plasmid)
3. Vary repressor DNA concentration: 0, 0.05, 0.1, 0.3, 1, 3, 10, 30 nM
4. Set up cell-free reactions for each repressor DNA concentration
5. Measure endpoint GFP fluorescence (at 8 hours, when plateau is reached)
6. Fit the Hill function to the data: $y$ vs. $[\text{repressor DNA}]$

The fit yields $K_{1/2}$, $n$, $y_{min}$, $y_{max}$ — the four parameters needed to characterize the gate for use in CELLO or other automated circuit design tools.

**Example data** (hypothetical TetR characterization):
```
[TetR DNA] (nM)  |  GFP output (REU)
0                |  850
0.1              |  820
0.3              |  710
1.0              |  450
3.0              |  180
10               |  45
30               |  12
```
Fit gives: $y_{max}=850$, $y_{min}=10$, $K_{1/2}=2.1$ nM, $n=1.8$

## Protein-Protein Interaction Characterization

Cell-free systems enable rapid measurement of protein-protein binding:

**Split-GFP complementation**: if two proteins interact, fused split-GFP halves (GFP1-10 on protein A; GFP11 on protein B) will reconstitute fluorescence upon interaction. Fluorescence signal quantifies binding.

**LUMIER assay (cell-free adaptation)**: Express bait protein tagged with luciferase (NanoLuc) and prey protein tagged with HaloTag in separate cell-free reactions. Mix reactions; capture bait with anti-NanoLuc antibody; measure NanoLuc activity in pellet vs. supernatant after washing. Ratio (pellet/total) quantifies binding.

**In vitro pull-down**: express His-tagged bait + untagged prey in a single TX-TL reaction. Add Ni-NTA magnetic beads; elute; detect prey by western blot or mass spectrometry. Quantify bound prey as fraction of total.

## mRNA Quantification by Molecular Beacons

Measuring mRNA levels independently from protein output allows decoupling of transcription from translation:

**Molecular beacon**: a hairpin-loop oligonucleotide probe with 5' fluorophore and 3' quencher. Hybridizes to target mRNA → loop opens → fluorophore unquenches.

Protocol for real-time mRNA measurement in cell-free:
1. Design molecular beacon complementary to the mRNA of interest (typically targeting the coding sequence)
2. Add 100–500 nM beacon to cell-free reaction at setup
3. Monitor fluorescence in parallel with protein reporter (different wavelength channels)
4. Result: simultaneous mRNA and protein time courses

This enables computation of the instantaneous translation rate:
$$\alpha_{translation} = \frac{d[\text{protein}]/dt}{[\text{mRNA}]}$$

Variation in this ratio across different RBS variants or incubation conditions reveals translation efficiency independently of transcription.

## Standardized Reporting: The TXTL Data Plane

The Noireaux lab and collaborators have proposed a standardized "data plane" for cell-free characterization:

**Minimum required data for a characterized part**:
1. Raw fluorescence time course (all technical replicates)
2. Reference promoter fluorescence time course (same extract batch)
3. Extract batch identifier (to allow cross-batch comparisons)
4. DNA concentration used
5. Temperature and incubation duration

**Derived quantities**:
- Normalized expression (relative to reference promoter)
- Kinetic parameters: $t_{1/2}$ (half-maximal time), plateau level, initial slope
- Batch-normalized PoPS value

Python pseudocode for PoPS analysis:
```python
import numpy as np
from scipy.stats import linregress

def compute_pops(time_h, fluorescence_matrix, dna_conc_nM, calibration_factor):
    """
    time_h: array of timepoints
    fluorescence_matrix: shape (n_dna_conc, n_timepoints)
    dna_conc_nM: array of DNA concentrations
    calibration_factor: fluorescence/REU (from GFP standard curve)
    """
    initial_rates = []
    for i, conc in enumerate(dna_conc_nM):
        fl = fluorescence_matrix[i]
        # Fit linear to first 60 min (t < 1 h)
        mask = time_h < 1.0
        slope, intercept, r, p, se = linregress(time_h[mask], fl[mask])
        rate_reu_per_h = slope / calibration_factor
        initial_rates.append(rate_reu_per_h)
    
    # PoPS = slope of rate vs. [DNA]
    pops_slope, _, r2, _, _ = linregress(dna_conc_nM, initial_rates)
    return pops_slope  # REU / nM / hour
```

## Inter-Laboratory Reproducibility

One of the key challenges in cell-free characterization is batch-to-batch variability in extract quality. Standard approaches to address this:

1. **Reference part normalization**: always express a standard reference part (e.g., Anderson J23101 + B0034 RBS + GFP) in the same plate as the part being characterized. Report all values as a ratio to the reference.

2. **Calibrated fluorescence units**: convert arbitrary fluorescence units to µM GFP equivalents using a purified GFP standard curve measured on the same plate reader. Enables comparisons across instruments.

3. **Extract batch qualification**: require each new extract batch to express the reference part within ±20% of the historical mean before using for new characterizations.

4. **Cross-lab reference strains**: the BIOFAB and iGEM communities maintain reference constructs that can be characterized in multiple labs' cell-free systems to establish inter-laboratory calibration factors.

## Why This Matters

Quantitative characterization of genetic parts in cell-free systems provides the parameter measurements needed to move synthetic biology from empirical trial-and-error to model-driven design. The CELLO circuit design tool requires Hill function parameters ($K_{1/2}$, $n$, $y_{min}$, $y_{max}$) for each gate — and cell-free characterization is the established method for measuring these parameters at throughput sufficient to populate a gate library. More broadly, PoPS and RIR measurements in standardized cell-free systems are the foundation for the BioBrick parts registry's aspiration of interchangeable, characterized parts: a promoter characterized in one lab's cell-free system should function predictably in another lab's genetic circuit because the shared standard reduces the variability that makes biological engineering unreliable. Every advance in standardizing cell-free measurement protocols directly improves the predictability of synthetic biology design.
