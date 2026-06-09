# The Repressilator: A Synthetic Genetic Oscillator

In the same issue of *Nature* that published the toggle switch — January 2000 — a second group announced something equally audacious. Michael Elowitz and Stanislas Leibler had built a genetic clock. Not adapted a natural clock, not borrowed oscillatory machinery from a circadian system. They had designed one from scratch: three genes, connected in a ring of mutual repression, ticking back and forth with a period of roughly 160 minutes in living *E. coli* cells. If the toggle switch proved you could give a cell a memory, the repressilator proved you could give it a rhythm.

The **repressilator** is the first demonstration of a synthetic genetic oscillator — a circuit that produces sustained periodic oscillations in protein concentration without any external rhythmic input. It remains one of the most cited papers in synthetic biology and serves as the canonical example of how dynamic behavior emerges from the topology of a gene regulatory network.

## Design Principle: The Three-Gene Repressor Ring

The repressilator consists of three transcriptional repressors arranged in a cyclic inhibitory loop:

$$\text{TetR} \dashv \lambda\text{CI} \dashv \text{LacI} \dashv \text{TetR}$$

- TetR represses the promoter (P_R) that drives λcI expression
- λcI (lambda phage cI repressor) represses the promoter (P_LacI1) that drives LacI expression
- LacI represses the promoter (P_Tet) that drives TetR expression

No repressor can accumulate indefinitely: whenever one repressor rises, it represses the next, which allows the third to rise, which represses the first, causing it to fall. This cyclic chase creates oscillations.

Each repressor is also fused to a GFP variant for visualization, and one repressor (cI) is additionally fused to mCherry. This enables direct observation of oscillation phase and amplitude in single cells by fluorescence microscopy.

## ODE Model and Conditions for Oscillation

Let $m_i$ denote the mRNA concentration and $p_i$ the protein concentration for gene $i$ ($i$ = TetR, cI, LacI):

$$\frac{dm_i}{dt} = -m_i + \frac{\alpha}{1 + p_j^n} + \alpha_0$$

$$\frac{dp_i}{dt} = -\beta(p_i - m_i)$$

Where:
- $p_j$ is the repressor that represses gene $i$ (the preceding gene in the cycle)
- $\alpha$: maximum transcription rate in the absence of repressor
- $\alpha_0$: basal (leaky) transcription rate
- $n$: Hill coefficient
- $\beta$: ratio of protein decay rate to mRNA decay rate

This gives a 6-dimensional ODE system (3 mRNA + 3 protein variables).

**Conditions for sustained oscillation** (from linear stability analysis of the fixed point):

1. **Sufficient repression** ($\alpha$ large enough): if all repressors are too weak, the system settles to a stable fixed point where all proteins are at equal intermediate concentrations.

2. **Sufficient cooperativity** ($n$ large enough): with $n = 1$ (no cooperativity), the system has a single stable fixed point. As $n$ increases, the system undergoes a **Hopf bifurcation** at a critical value $n_c$, above which the fixed point becomes unstable and a limit cycle (oscillation) emerges.

3. **Nonzero $\beta$** (protein decay relative to mRNA): the delay between mRNA synthesis and protein accumulation/degradation provides the phase lag necessary for oscillation.

For the specific parameter regime Elowitz and Leibler used:
- $\alpha \approx 216$ (repressor-off / repressor-on ratio)
- $n \approx 2$ (cooperative dimerization)
- $\beta \approx 1$ (equal protein and mRNA half-lives)

**Period**: approximately 160 minutes under standard growth conditions in *E. coli*.

## Key Findings from the Experiment

1. **Oscillation is noisy**: not all cells oscillate in synchrony. The period and amplitude vary substantially from cell to cell. Compared to natural oscillators (e.g., the circadian clock), the repressilator has poor precision: coefficient of variation of period ≈ 0.4–0.8.

2. **Single-cell oscillation**: microfluidic trapping of individual cells confirmed that oscillations occur in single cells (not population-level synchronization), verifying that the network topology is sufficient for oscillation without quorum sensing.

3. **Protein degradation is critical**: the original repressilator used ssrA degradation tags on all three proteins. These tags direct proteins to the ClpXP/ClpAP protease complex, giving fast turnover (half-life ~10 min vs. ~60 min dilution-only). Removing ssrA tags eliminates oscillation, demonstrating that fast degradation is essential.

## Why Is the Original Repressilator Noisy?

Elowitz and Leibler acknowledged that the repressilator's oscillations were irregular compared to natural clocks. The noise arises from:

1. **Low protein copy numbers**: each repressor is present at only ~10–100 molecules per cell. Stochastic fluctuations at low copy number are large relative to the mean — enough to disrupt the phase relationship between repressors.

2. **No synchronization mechanism**: each cell oscillates autonomously. Without coupling between cells, populations desynchronize rapidly.

3. **Parameter variability**: slight differences in plasmid copy number or growth rate between cells cause different periods.

## Improved Repressilator Designs

Subsequent work addressed these limitations:

**Stricker et al. (2008)** added positive feedback within each gene's regulation, creating an activator-repressor design. The mixed positive-negative feedback:
- Increases the amplitude of oscillation (more robust)
- Allows period tuning by changing the balance of positive and negative feedback
- Reduces period variability (CV ≈ 0.1–0.2)

**Danino et al. (2010)** coupled repressilator-like oscillators in *E. coli* biofilms through quorum sensing (AHL molecules). Thousands of cells synchronized their oscillations, demonstrating emergent synchrony from individual noisy oscillators.

**Castillo-Hair et al. (2020)** designed a repressilator with degradation rate-balanced parts, achieving period CV ≈ 0.05 — approaching the precision of natural circadian clocks.

## Practical Oscillator Design Checklist

Before building a genetic oscillator:

1. Verify that the selected repressors have Hill coefficients n > 1.5
2. Ensure all repressors have comparable expression levels (asymmetry dampens oscillation)
3. Add ssrA (or equivalent) degradation tags to all protein components
4. Verify degradation is functional: pulse-chase experiment to measure protein half-life in the chassis
5. Use microfluidics or single-cell imaging to detect oscillations (bulk measurements obscure asynchronous oscillations)
6. Expect period drift and noise; quantify period CV across at least 50 individual cells

## Why This Matters

The repressilator demonstrated that oscillatory behavior — a dynamic property we associate with sophisticated biological timing circuits — can be designed de novo from first principles, without any knowledge of natural oscillators. The analysis also revealed a general principle: any odd number of mutually repressing genes in a cyclic network can potentially oscillate, given sufficient nonlinearity and delay. This generalizes beyond the repressilator itself to natural gene regulatory networks, providing a framework for understanding how oscillatory behavior arises in circadian clocks, cell cycle regulation, and developmental patterning.
