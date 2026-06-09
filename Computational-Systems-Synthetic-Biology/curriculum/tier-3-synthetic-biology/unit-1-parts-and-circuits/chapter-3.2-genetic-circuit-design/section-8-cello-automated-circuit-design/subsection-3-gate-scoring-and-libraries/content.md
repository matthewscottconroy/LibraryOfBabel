# Gate Scoring and Gate Libraries in CELLO

A design tool is only as reliable as the metric it optimizes. If the circuit score perfectly predicted which designs would work in cells, the 2016 CELLO paper would have reported 60 out of 60 successes instead of 45. It did not, and understanding why the score sometimes fails is as instructive as understanding why it usually succeeds. This section examines both: the **circuit score** that drives CELLO's optimization and the **gate libraries** — stored as UCF JSON documents — that define which building blocks are available in any given chassis.

The output of CELLO's optimization is only as good as the metric it optimizes and the library of parts it draws from. This section examines the circuit score that drives CELLO's gate assignment search and the gate libraries — stored as UCF (User Constraint File) JSON documents — that define the biological building blocks available in any given chassis.

## The Circuit Score: Quantifying How Well a Circuit Works

CELLO assigns a scalar score to every candidate gate assignment. This score captures the fundamental requirement of digital logic: the ON output of a gate must be clearly distinguishable from its OFF output, even when those signals must drive subsequent gates.

### Signal Ranges and REU

Gate transfer functions are parameterized in **REU (Relative Expression Units)**, a normalized unit defined relative to a reference promoter. Using REU rather than absolute protein concentrations makes measurements portable across labs and experimental conditions.

Each gate $g$ in a given organism's library is characterized by two properties derived from its Hill-function transfer curve:

- $y_{ON}(g)$: the output expression level when the gate's repressor input is absent (gate output is HIGH)
- $y_{OFF}(g)$: the output expression level when the gate's repressor input is saturating (gate output is LOW)

A well-performing gate has high $y_{ON}$ and low $y_{OFF}$, giving a large **dynamic range**:

$$\text{DR}(g) = \frac{y_{ON}(g)}{y_{OFF}(g)}$$

Values of DR $> 100$ are considered good; many characterized gates achieve DR of 50–500 REU.

### The ON/OFF Score

For a complete circuit assignment, CELLO evaluates whether each gate produces outputs compatible with the next gate's input requirements across all $2^n$ input combinations. The circuit is simulated for every input combination, and the output signal is read for the cases that should be HIGH and the cases that should be LOW. The circuit score is:

$$S = \min_{(\text{input combos})} \frac{y_{circuit,ON}}{y_{circuit,OFF}}$$

The minimum is taken across all pairs of an ON-required and OFF-required output from the full truth table. This **worst-case** scoring forces the optimizer to ensure every logic combination works, not just the average case.

**Worked example**: Consider a 2-input NOR gate driving a 2-input NOR gate:

Input | Gate 1 Output | Gate 2 Output | Required Logic Output
------|--------------|--------------|---------------------
(0,0) | HIGH         | LOW          | 0
(0,1) | LOW          | HIGH         | 1
(1,0) | LOW          | HIGH         | 1
(1,1) | LOW          | HIGH         | 1

If Gate 1 output is 200 REU when HIGH and 0.3 REU when LOW, and Gate 2 saturates below 0.5 REU input (meaning HIGH output) and is repressed above 50 REU (meaning LOW output), then this assignment works. The score would be:

$$S = \frac{y_{ON,\text{final}}}{y_{OFF,\text{final}}} = \frac{180}{1.2} = 150$$

A circuit score above ~10 is typically required for reliable behavior in cells. The 2016 Nielsen et al. paper found empirically that circuits scoring above ~30 were more likely to function correctly.

## Gate Library Structure: UCF Files

The gate library defines every biological component available for circuit construction in a specific chassis. CELLO stores this information in **UCF (User Constraint File)** JSON documents. A UCF file for *E. coli* K-12 contains the following sections:

### Collection 1: Collection Metadata
```json
{
  "collection": "header",
  "description": "E. coli K-12 MG1655 gate library",
  "version": "Eco1C1G1T1",
  "date": "2016",
  "author": "Nielsen et al."
}
```

### Collection 2: Gates
Each gate is an entry with its Hill function parameters, measured in standardized conditions:

```json
{
  "collection": "gates",
  "gate_name": "AmtR",
  "gate_type": "NOR",
  "system": "TetR-family repressors",
  "regulator": "AmtR",
  "color_hexcode": "#00AA00",
  "functions": {
    "response_function": "ymax / (1 + (x / K)^n) + ymin",
    "parameters": {
      "ymax": 4.082,
      "ymin": 0.025,
      "K": 0.267,
      "n": 1.684
    }
  }
}
```

The parameter **ymax** is $y_{ON}$ in REU, **ymin** is $y_{OFF}$, **K** is the Hill constant in REU (input concentration at half-maximal repression), and **n** is the Hill coefficient governing cooperativity. A higher n produces sharper switching.

### Collection 3: Parts
For each gate, the UCF stores the actual DNA part sequences:

```json
{
  "collection": "parts",
  "gate_name": "AmtR",
  "promoter_sequence": "ATTTATCCCGAT...GTGAGCGGATA",
  "rbs_sequence": "AAGTTAAGAGG",
  "cds_sequence": "ATGACCATGATTACG...",
  "terminator_sequence": "CCAGGCATCAAAT"
}
```

This direct coupling of functional parameters to sequence enables CELLO to output synthesis-ready DNA without any additional part lookup step.

### Collection 4: Input Sensors and Output Reporters

Input sensors (e.g., araC responding to arabinose, luxR responding to AHL) are stored with their own response function parameters and the concentration range of their cognate inducer. Output reporters (e.g., YFP, RFP) are stored with their excitation/emission spectra and any calibration factors converting fluorescence to REU.

## The 12-Gate Library: Orthogonal TetR-Family Repressors

The original CELLO gate library (Eco1C1G1T1) contained **12 gates** based on TetR-family transcriptional repressors, chosen for their modularity: each repressor binds its cognate operator with high affinity and specificity and cross-reacts minimally with other operators in the set.

| Gate Name | Repressor | DR (REU) | Hill n |
|-----------|-----------|----------|--------|
| AmtR      | AmtR      | 163      | 1.68   |
| AmeR      | AmeR      | 204      | 1.81   |
| BM3R1     | BM3R1     | 89       | 2.04   |
| BetI      | BetI      | 71       | 1.77   |
| HlyIIR    | HlyIIR    | 213      | 1.72   |
| IcaRA     | IcaRA     | 117      | 2.10   |
| LitR      | LitR      | 98       | 1.55   |
| McbR      | McbR      | 78       | 1.91   |
| PhIF      | PhIF      | 142      | 1.69   |
| PhlF      | PhlF      | 190      | 1.83   |
| QacR      | QacR      | 61       | 1.52   |
| SrpR      | SrpR      | 127      | 2.01   |

The Hill coefficients cluster around 1.5–2.1, providing moderate cooperativity — sufficient for clear digital switching but not so nonlinear as to require extreme inducer concentrations.

## Why the Library Size Limits Circuit Complexity

With 12 orthogonal gates, CELLO can in principle assign up to 12 unique gates per circuit. However, the 2016 paper showed that reliability degrades rapidly beyond ~7 gates due to resource competition and accumulated signal attenuation. Expanded libraries are therefore not just about more complex circuits — they must also improve individual gate performance.

Recent library expansion efforts have pursued two strategies:

1. **New TF-promoter pairs**: screening metagenomics databases for novel TetR-family members orthogonal to existing ones. Libraries of 60+ candidates have been screened, with ~15–20 passing orthogonality and performance criteria.

2. **Non-TF-based gates**: Ribozyme gates (RNA-based), CRISPR-dCas9-based gates (CRISPRi), and sigma factor-based gates offer distinct mechanisms and fewer cross-talk concerns. Each requires its own transfer function characterization methodology.

## Extending the Library: Characterizing New Gates

To add a new gate to a UCF file, the following measurements are required under identical standardized conditions (same plasmid backbone, same chassis, same growth protocol):

```python
# Pseudocode for gate characterization workflow
for input_concentration in [0, 0.01, 0.1, 1, 10, 100, 1000]:  # REU units
    cells = transform(gate_construct, input_sensor_construct)
    fluorescence = plate_reader(cells, time=16h, wavelength='YFP')
    reu = fluorescence / reference_promoter_fluorescence
    data.append((input_concentration, reu))

# Fit Hill function
ymax, ymin, K, n = fit_hill_function(data)
# Validate: DR = ymax/ymin > 50 required for inclusion
```

This standardization is what enables library portability: a gate characterized in one lab can be combined computationally with gates from another lab, because both used REU as the common unit.

## Why This Matters

Gate scoring and library management are the infrastructure that makes automated circuit design scalable. The circuit score provides a principled, predictive metric — circuits scoring above ~30 have meaningful odds of working when synthesized. The UCF format separates biological knowledge (what parts exist and how they behave) from design algorithms (how to assemble them into circuits), enabling the library to grow independently of the design software. Every new orthogonal gate added to a validated UCF library exponentially increases the design space accessible to CELLO: a library of $n$ gates can support circuits with up to $n$ logic stages, and the number of distinct Boolean functions realizable scales combinatorially with $n$. This is the same principle that made silicon chip design scalable: a standard cell library whose cells are well-characterized enables tools to design circuits of arbitrary complexity without revisiting physics.
