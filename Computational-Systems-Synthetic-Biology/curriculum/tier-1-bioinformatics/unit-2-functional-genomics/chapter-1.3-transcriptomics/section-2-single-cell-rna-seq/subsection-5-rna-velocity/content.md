# RNA Velocity

Pseudotime orders cells based on transcriptional similarity but cannot indicate the direction of transitions without external information — you have to tell it where the root is. If you do not already know which cell state is the progenitor, pseudotime can be reversed and you would not know. **RNA velocity** provides a fundamentally different and more powerful approach: it infers the direction and speed of transcriptional change at the level of individual cells by exploiting the kinetics of mRNA biogenesis and degradation.

The key insight, published by La Manno et al. in Nature in 2018, is that cells are not just characterized by what genes they express now — they carry within them a record of where their transcriptome is heading. And this record is written in an unexpected place: the ratio of immature (unspliced) to mature (spliced) mRNA for each gene.

## The Biological Basis: Spliced vs. Unspliced mRNA

When a gene is actively transcribed, pre-mRNA containing **unspliced** (intronic) sequences is produced. This unspliced mRNA is rapidly processed into **spliced** mature mRNA, which is then either translated or degraded. If transcription is currently increasing, the amount of unspliced mRNA will be rising faster than the spliced mRNA. If transcription is decreasing, unspliced mRNA will fall first, followed by spliced mRNA.

This means the ratio of unspliced to spliced mRNA for each gene encodes information about whether that gene is being induced (increasing) or repressed (decreasing) in each cell — a molecular "arrow" pointing toward the future transcriptional state.

Formally, the dynamics follow:

$$\frac{du}{dt} = \alpha - \beta u \qquad \frac{ds}{dt} = \beta u - \gamma s$$

where $u$ = unspliced mRNA, $s$ = spliced mRNA, $\alpha$ = transcription rate, $\beta$ = splicing rate, $\gamma$ = degradation rate. At steady state, $s = \frac{\beta}{\gamma} u$, tracing a line in the $(u, s)$ phase portrait.

The elegance of this formulation is that it uses the cell's own biochemistry as a clock. Splicing and degradation happen on timescales of minutes to hours. A cell that has just turned on a gene will have more unspliced mRNA than a cell that has been expressing it at steady state. By observing where a cell sits relative to the expected steady-state ratio, you can infer whether that gene is being turned up or turned down — and extending this across thousands of genes gives you a velocity vector pointing toward where the cell is transcriptionally headed.

## Counting Unspliced Reads with velocyto

The first challenge is quantifying unspliced reads. Standard alignment assigns reads to annotated exons; **velocyto** re-annotates BAM files to count reads overlapping introns (unspliced), exon-exon junctions (spliced), or spanning exon-intron boundaries (ambiguous). The output is two count matrices: `spliced.mtx` and `unspliced.mtx`, one for each gene in each cell.

Velocyto's `run10x` command processes Cell Ranger output directly:

```bash
velocyto run10x -m repeat_mask.gtf \
    cellranger_output_dir/ \
    genes.gtf
```

## scVelo: Stochastic and Dynamical Models

The original RNA velocity paper (La Manno et al., 2018) used a steady-state model: cells at the gene-specific steady-state ratio of spliced/unspliced set the baseline, and cells above or below this ratio are inferred to be inducing or repressing the gene. **scVelo** (Bergen et al., 2020) extended this with more principled models:

**Stochastic model**: accounts for transcriptional bursting and cell-to-cell noise, fitting velocity by maximizing likelihood over the distribution of spliced/unspliced counts.

**Dynamical model**: fits the full kinetic parameters ($\alpha$, $\beta$, $\gamma$) for each gene and each cell, allowing genes to be in induction, repression, or steady-state phases. This requires more computation but produces velocities for a larger fraction of genes.

```python
import scvelo as scv

adata = scv.read('loom_file.loom')
scv.pp.filter_and_normalize(adata, min_shared_counts=20, n_top_genes=2000)
scv.pp.moments(adata, n_pcs=30, n_neighbors=30)
scv.tl.velocity(adata, mode='dynamical')  # or 'stochastic'
scv.tl.velocity_graph(adata)
scv.pl.velocity_embedding_stream(adata, basis='umap')
```

The result is a UMAP plot overlaid with streamlines — arrows that show which direction each region of the cell state space is moving. A cluster of early progenitors will have arrows pointing toward mature cell types. If the biology is working as expected, these arrows should recapitulate known differentiation flows. When they do not, it is either a biological surprise or an artifact worth investigating.

## The Phase Portrait

For each gene, the **phase portrait** — a scatter plot of spliced (x-axis) vs. unspliced (y-axis) mRNA counts across cells — reveals the kinetic behavior. Under the steady-state model, cells at equilibrium lie along a diagonal "fit line." Cells above the line (excess unspliced) are inducing the gene; cells below (excess spliced) are repressing it.

The dynamical model identifies distinct "on" (induction) and "off" (repression) phases, with cells traversing a characteristic loop in the phase portrait as they pass through activation and deactivation. A gene whose phase portrait shows a clean loop — cells cycling through induction, steady state, and repression in a coherent order — is a high-confidence velocity gene. A gene with a scattered, shapeless cloud is contributing noise.

## Limitations

RNA velocity is a powerful but imperfect tool, and understanding its limitations is essential for interpreting results:

- **Sparsity**: The unspliced signal is weaker and sparser than spliced, because intronic sequences are shorter and less stable. Velocities are noisy for most individual genes and become informative only when averaged over many informative genes.
- **Steady-state assumption**: The velocity computation assumes that some cells have reached a gene-specific equilibrium, which may not be true in rapidly changing systems.
- **Intronic reads in 3' capture data**: 10x Genomics data captures primarily the 3' end of transcripts, which may include some intronic reads from the 3' UTR, creating ambiguity in the unspliced count.
- **Causal interpretation**: As with pseudotime, velocity shows the likely direction of change but does not establish causality between gene expression changes.

These limitations do not invalidate RNA velocity — they define its appropriate domain of use. In a steadily differentiating system with thousands of cells, velocity arrows are remarkably informative. In a rapidly stressed system where all cells are simultaneously responding, the steady-state assumption may fail. Knowing when to trust the arrows is part of the skill.

## Why This Matters

RNA velocity provides a directional arrow on single-cell data that would otherwise only show static snapshots. This has revealed surprising features of differentiation dynamics — including bidirectional transitions between states once thought unidirectional, suggesting far greater cellular plasticity than the classical view of differentiation allowed. In cancer biology, RNA velocity has shown that some tumor cell populations are not frozen in an aberrant state but are actively transitioning between states, with implications for understanding drug resistance. In developmental biology, it has provided molecular resolution on the timing of transcription factor activation during lineage commitment. The ability to infer direction from molecular biochemistry, without any time-lapse imaging, is one of the most conceptually elegant achievements of modern computational biology.
