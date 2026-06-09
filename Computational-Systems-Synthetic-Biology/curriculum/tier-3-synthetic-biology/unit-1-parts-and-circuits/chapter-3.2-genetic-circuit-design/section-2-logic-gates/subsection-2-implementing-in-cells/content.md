# Implementing Genetic Logic in Cells: Technologies and Trade-offs

Every technology has a regime where it shines and a regime where it breaks down. Silicon transistors fail at high temperatures; vacuum tubes handle extreme voltages that destroy semiconductors. The same principle applies to the three main technologies for building genetic logic gates in living cells. Transcription factor-based gates, CRISPR-based gates, and RNA-based gates each carry distinct strengths, characteristic failure modes, and practical limits. Choosing the wrong technology for a given application — using CRISPR gates for a twelve-stage circuit, say — is a reliable way to spend months building something that fails for reasons you could have predicted in advance. This section maps out those trade-offs so you can choose before you build.

## Transcription Factor-Based Gates

### Natural and Engineered Transcription Factors

The most characterized gate libraries use **bacterial transcription factors** (TFs) as signal carriers. A TF gate consists of:
1. A promoter that drives expression of the TF (input stage)
2. A promoter regulated by that TF (output stage)

The TF concentration is the intermediate signal: the input promoter controls how much TF is produced; the TF concentration determines output gene expression.

**Key TF families used in gate libraries**:
- **Zinc finger proteins**: can be engineered to bind arbitrary DNA sequences; used in early synthetic circuits but largely superseded by TALEs and CRISPR
- **TALEs (Transcription Activator-Like Effectors)**: programmable DNA-binding proteins from *Xanthomonas*; one TALE per target sequence; used to create orthogonal TF-promoter pairs
- **LuxR family quorum-sensing regulators**: naturally orthogonal acyl-homoserine lactone (AHL) sensors; 30+ characterized members; used for multi-input circuits

**The CELLO gate library** uses characterized *E. coli* TF-promoter pairs with measured Hill function parameters. Twelve TF-promoter pairs that show minimal cross-activity with each other provide the orthogonal "gates" from which circuits are assembled.

### Signal Levels and Compatibility

For TF-based gates to function reliably in circuits, the output concentration of each gate must fall within the input sensitivity range of the next gate. This requires careful attention to:
- **TF expression levels**: too low → insufficient input for next gate; too high → saturates downstream
- **K₅₀ values**: must match the expected TF concentrations in the cellular environment
- **Hill coefficients**: determine how sharp the threshold is; n > 2 preferred for digital-like behavior

## CRISPR-Based Gates

### Multiplexing with dCas9

A single dCas9 protein can simultaneously regulate many genes when provided with multiple guide RNAs. This multiplexability makes CRISPR-based gates attractive for complex circuits:

**CRISPR NOT gate**: sgRNA directs dCas9 to a target promoter → represses expression.
- Input: sgRNA concentration (controlled by an inducible promoter)
- Output: expression of the target gene
- ON/OFF ratio: up to 1000-fold in bacteria with strong targeting

**CRISPR NOR gate**: one dCas9 with two guide RNAs, each targeting the output gene's promoter. If either sgRNA directs dCas9 to the promoter, output is repressed.

**CRISPR AND gate**: two different dCas9 proteins (e.g., Sp-dCas9 and Sa-dCas9), each with its own guide RNA targeting different halves of a synthetic two-part promoter. Both must bind to activate transcription:
- Split T7 promoter: each dCas9-split-VP16 half activates only when both are bound
- Output only when both Input A (sgRNA-A) and Input B (sgRNA-B) are present

### Trade-offs with CRISPR gates

**Advantages**:
- Programmable: change the guide RNA to change the gene target
- Multiplexable: many genes regulated simultaneously from one dCas9
- No new protein for each gate (unlike TF-based gates)

**Disadvantages**:
- **Resource competition**: dCas9 is a shared resource. High levels of one sgRNA can titrate dCas9 away from other gates in the same cell, creating unexpected cross-talk.
- **Saturation**: at high circuit complexity (> 10–15 gates), dCas9 availability becomes limiting.
- **Slower dynamics**: dCas9 complexes are very stable; unbinding is slow, reducing response speed.

A practical limit for dCas9-based circuits in *E. coli* is approximately 5–8 simultaneous gates before resource competition becomes problematic. This can be partially addressed by using multiple orthogonal dCas9 proteins (Sp-dCas9, Sa-dCas9, Lb-Cas12a).

## RNA-Based Gates

### Toehold Switch Gates

RNA-based gates using toehold switches (section 5.4) operate entirely at the post-transcriptional level:
- **NOT gate**: a toehold switch controls translation of a repressor protein. In the absence of trigger RNA: switch is OFF → repressor absent → output expressed. Trigger RNA present → switch ON → repressor produced → output repressed.
- **AND gate**: output mRNA has two toehold switches in series. Translation requires both switches to be opened by their respective trigger RNAs. If either trigger is absent, the second switch blocks translation.

RNA-based gates are particularly powerful in **cell-free systems** (section 3.6), where:
- There are no resource competition issues (the extract is replenished)
- Toehold switches can be characterized rapidly (hours per iteration)
- Large libraries of orthogonal gates can be assembled without cellular burden concerns

**Limitation**: RNA gates require all signals to be RNA molecules. Interfacing with protein-level signals requires transcription of trigger RNAs, adding a layer of complexity.

## Worked Example: Designing a Three-Input Majority Circuit

A majority circuit produces output = 1 when the majority (2 or 3 of 3) inputs are high. Its truth table:

| A | B | C | Output |
|---|---|---|---|
| 0 | 0 | 0 | 0 |
| 1 | 0 | 0 | 0 |
| 0 | 1 | 0 | 0 |
| 0 | 0 | 1 | 0 |
| 1 | 1 | 0 | 1 |
| 1 | 0 | 1 | 1 |
| 0 | 1 | 1 | 1 |
| 1 | 1 | 1 | 1 |

Boolean expression: Majority(A, B, C) = AB + AC + BC

Implementation using NOR gates (applying De Morgan's theorem):
$$AB + AC + BC = \overline{\overline{AB} \cdot \overline{AC} \cdot \overline{BC}}$$

This requires:
- Three NAND gates (AB, AC, BC) — each implemented as AND followed by NOT
- One final NOR gate over all three outputs

CELLO would take this Boolean specification and automatically assign characterized TF-gates to implement each logical operation, selecting gate combinations that have mutually compatible signal levels.

## Why This Matters

The choice between TF-based, CRISPR-based, and RNA-based gate implementations is an engineering decision with significant downstream consequences. TF-based gates offer the most characterized performance in living cells and form the foundation of automated design tools. CRISPR gates offer programmability but are limited by shared dCas9 resource constraints. RNA gates offer speed and cell-free applicability but require careful signal-level management. Understanding these trade-offs allows practitioners to select the right technology for their application — and to anticipate where circuits will fail before committing to a design.
