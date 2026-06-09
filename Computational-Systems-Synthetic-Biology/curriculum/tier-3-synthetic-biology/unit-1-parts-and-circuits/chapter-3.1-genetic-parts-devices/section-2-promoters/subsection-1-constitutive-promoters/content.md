# Constitutive Promoters: Structure, Strength, and Characterization

Here is a fact worth sitting with: a single six-base sequence — TATAAT — has been doing the same job for billions of years in bacteria ranging from *E. coli* to *Thermus thermophilus*. Every time a cell needs to make a housekeeping protein, RNA polymerase finds that sequence, pries open the double helix, and begins copying the gene. The promoter seems simple. But hidden inside this apparent simplicity is a quantitative mechanism capable of driving expression levels that differ by a factor of 250 across promoter variants, all from six-letter changes in the same short sequence. For a synthetic biologist trying to dial in a specific expression level, understanding that mechanism is the difference between guessing and engineering.

A **constitutive promoter** drives transcription continuously, at a rate determined by its sequence rather than by regulatory inputs. In a world where most biologically important genes are regulated, constitutive promoters seem simple—but their apparent simplicity conceals rich mechanistic detail that determines exactly how much RNA a cell produces from a given gene, under what conditions, and how reliably.

## Sigma Factor Recognition and the Core Promoter Elements

In *Escherichia coli*, the primary sigma factor **σ70** (encoded by *rpoD*) directs the RNA polymerase holoenzyme (α₂ββ'ωσ) to most housekeeping gene promoters. σ70 recognizes two hexameric sequences centered at positions −35 and −10 relative to the transcription start site (+1):

- **−35 element**: consensus `TTGACA`; recognized by σ70 domains 4.2
- **−10 element**: consensus `TATAAT` (Pribnow box); recognized by σ70 domain 2.4; important for strand separation during open complex formation
- **Spacer length**: optimal 17 ± 1 bp; deviations from 17 bp reduce promoter strength substantially
- **Extended −10**: sequence `TGn` immediately upstream of the −10 hexamer; present in strong promoters lacking a consensus −35

The promoter strength is not simply the sum of individual element matches to consensus. The two elements must cooperate: the RNAP holoenzyme makes simultaneous contacts to both hexamers, and the spacer length determines the phase relationship between them on the double helix. A promoter with a perfect −35 but a suboptimal spacer will underperform a promoter with moderate −35 and optimal spacer.

## From Closed to Open Complex

Transcription initiation proceeds through a series of conformational changes:

$$\text{RNAP} + P \xrightarrow{k_f} \text{RP}_c \xrightarrow{k_2} \text{RP}_o \xrightarrow{k_3} \text{RP}_{init} \xrightarrow{} \text{Elongation}$$

Where $\text{RP}_c$ is the closed complex, $\text{RP}_o$ the open complex (with melted DNA), and $\text{RP}_{init}$ the initiating complex. Strong promoters accelerate the $k_2$ step (isomerization to open complex), which is rate-limiting for most bacterial promoters. The half-life of the open complex ($t_{1/2}$) varies from seconds (weak promoters) to hours (ribosomal RNA promoters).

## Promoter Strength: Units and Measurement

**Promoter strength** is most rigorously expressed as **PoPS (Polymerases Per Second)**—the rate of RNAP molecules initiating transcription per promoter per second. Measuring absolute PoPS requires knowing both the mRNA synthesis rate and the fraction of active promoters in a cell, which is technically demanding.

In practice, promoter strength is reported in **Relative Expression Units (REUs)** measured against a reference promoter. The Anderson Promoter Collection (iGEM Registry, J23 series) uses J23101 as a common reference. Strengths are expressed as the ratio of fluorescent reporter output to that of J23101 under identical conditions:

$$\text{REU}_i = \frac{[\text{fluorescence}]_i}{[\text{fluorescence}]_{\text{J23101}}}$$

The J23 series spans roughly 10-fold in expression from the weakest (J23114, ~0.01 REU) to the strongest (J23119, ~2.5 REU relative to J23101).

## The Anderson Promoter Library: A Worked Example

The Anderson library was constructed by systematically varying the −35, −10, and spacer regions of a base promoter sequence. A subset of variants with measured strengths:

| Part | Sequence (-35 ... spacer ... -10) | Relative strength |
|---|---|---|
| J23119 | TTGACAGCTAGCTCAGTCCTAGGTATAATGCTAGC | 1.00 (strong) |
| J23101 | TTTACAGCTAGCTCAGTCCTAGGTATTATGCTAGC | 0.70 |
| J23106 | TTTACGGCTAGCTCAGTCCTAGGTATAGTGCTAGC | 0.47 |
| J23116 | TTGACAGCTAGCTCAGTCCTAGGGACTATGCTAGC | 0.16 |
| J23114 | TTTATGGCTAGCTCAGTCCTAGGTACAATGCTAGC | 0.01 |

Notice that the strongest promoters (J23119) have sequences closest to the consensus TTGACA (−35) and TATAAT (−10). Mutations away from consensus generally decrease strength in a roughly additive fashion for individual substitutions, although epistatic interactions can complicate this.

## Strong Promoters: The UP Element and Ribosomal RNA Operons

The *rrnB* P1 promoter—which drives ribosomal RNA synthesis—is among the strongest promoters known in *E. coli*. Its exceptional strength arises from three features:
1. Near-consensus −35 (TTGTCA) and −10 (TATAAT) elements
2. An **UP element**: an A/T-rich sequence between −38 and −60 that directly contacts the C-terminal domain of the RNAP α-subunit, increasing initiation rate by ~30-fold
3. A **discriminator element** at −1 to +3 that enables rapid promoter escape

For synthetic biology, the T7 promoter system offers an orthogonal high-strength option: T7 RNA polymerase (encoded by phage T7 gene 1) is completely specific for T7 promoters and transcribes ~5× faster than *E. coli* RNAP. Placing T7 RNAP under inducible control (e.g., via a lac promoter) creates an amplified two-stage expression system used commercially for high-yield recombinant protein production.

## Measurement Best Practices

Characterizing a constitutive promoter correctly requires:

1. **Identical genetic context**: clone the promoter upstream of the same reporter (typically GFP or mCherry) in the same vector backbone and chromosomal location. Promoter strength measured from a high-copy plasmid will differ from chromosomal measurement.
2. **Growth phase normalization**: expression per cell varies with growth rate. Report under standardized conditions (e.g., exponential phase in M9 + glucose at 37°C, OD₆₀₀ = 0.3–0.5).
3. **Autofluorescence correction**: subtract signal from a no-reporter control.
4. **Multiple biological replicates**: at least three independent colonies, measured in duplicate.
5. **Reference included**: always measure your reference promoter in the same experiment; inter-day variation can exceed 20%.

## Why This Matters

Constitutive promoters are the most common type used in metabolic engineering and circuit design—whenever you want a fixed, reliable level of a protein, you reach for a constitutive promoter. The existence of a well-characterized library like the Anderson collection means that you can dial in a desired expression level by part selection rather than by re-engineering. The deeper lesson is that promoter strength is not an intrinsic property: it is a function of the sequence, the σ factor, the growth conditions, and the genetic context. Treating it as a context-free parameter is the source of many failed circuit predictions.
