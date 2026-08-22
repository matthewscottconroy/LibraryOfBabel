# Published MLDE Examples

Ideas in science are only as good as the experimental evidence behind them. Machine learning-guided directed evolution has an elegant theoretical framework — the oracle problem, surrogate models, Bayesian acquisition functions — but whether all of this sophisticated machinery actually helps find better proteins faster in a real lab is an empirical question. Over the past decade, that question has been answered with increasing rigor. The field has moved from proof-of-concept demonstrations on well-characterized benchmark datasets to applications in real pharmaceutical and industrial settings where the cost of experiments makes the efficiency gain decisive. What follows is a tour of the most instructive published examples: what was done, what was found, and what the results teach us about when MLDE works and when it doesn't.

Machine learning-guided directed evolution has progressed from theoretical proposals to rigorous experimental demonstrations across a range of protein engineering targets. This section examines published case studies that illustrate the practical performance, methodological choices, and limitations of MLDE in real experimental contexts.

## GB1 Protein: The Benchmark Landscape

**Wu et al. (2016)**: measured fitness (IgG binding) for all $20^4 = 160,000$ combinations at four positions in the GB1 immunoglobulin-binding domain (V39, D40, G41, V54). This exhaustively mapped landscape became the standard benchmark for MLDE methods.

**Key findings for MLDE**:
- Starting from 96 random measurements, a GP + one-hot model predicts fitness for all 160,000 variants with Spearman ρ = 0.65
- UCB acquisition outperforms random selection at every measurement budget tested
- Using ESM embeddings instead of one-hot: ρ = 0.75 with the same 96 training points (computed retrospectively)
- BO reaches 90% of the optimum fitness within 300 measurements, vs. >5,000 measurements needed for exhaustive search

**Lesson**: even a simple model with limited training data provides substantial improvement over random search on a well-characterized landscape.

## GFP Brightness: Sarkisyan et al. (2016)

**Experiment**: measured fluorescence of ~51,000 green fluorescent protein (GFP) variants at an average of 2.7 substitutions from a dim ancestor.

**MLDE application** (retrospective analysis by multiple groups):
- Training a GP on 1,000 random variants from the 51,000 dataset predicts the brightest variants with Spearman ρ ≈ 0.4–0.6 (depending on model)
- Bayesian optimization on this landscape (simulated, using the measured data as an in silico oracle) reaches the top 1% of fluorescence within 200–300 measurements

**Lesson**: the GFP landscape is more complex than GB1 (longer sequence, higher-order epistasis), reducing model accuracy but still enabling MLDE to outperform random search.

## Antibody Affinity Maturation (Mason et al. 2021)

**System**: evolve a therapeutic anti-VEGF antibody for higher affinity (lower Kd).

**Method**:
1. Measure binding Kd for 40 initial CDR3 variants by SPR (surface plasmon resonance)
2. Train a GP on one-hot encoded CDR3 sequences + Kd values
3. EI acquisition proposes 40 new CDR3 sequences per round
4. 3 BO rounds + measurement → converged on Kd = 3 pM from starting Kd = 50 pM (16-fold improvement)
5. Total measurements: ~160 SPR experiments

**Comparison**: classical random mutagenesis + FACS screening of the same CDR3 library required >5,000 measurements to find the same 3 pM variant.

**Lesson**: for expensive assays (SPR requires purified protein per measurement; cost ~$50–100/measurement), MLDE's 30-fold reduction in experimental cost is decisive. Total cost: BO approach ~$8,000 vs. random approach ~\$250,000.

## TEM Beta-Lactamase: Functional Prediction Across Homologs

**Dallago et al. (2021)** / **Riesselman et al. (2018)**: demonstrated that deep evolutionary models (EVmutation, DeepSequence) trained only on multiple sequence alignments (no experimental fitness data) predict mutational effects on beta-lactamase function with ρ ≈ 0.6–0.7 on the experimentally measured Ambler et al. dataset.

**Significance**: zero-shot prediction (no experimental fitness data required) from evolutionary information alone approaches the accuracy achievable with 100–500 measured variants when using experimental data.

**Current best zero-shot**: ESM-2 log-likelihood under the masked language model predicts single mutant effects on protein function with ρ ≈ 0.5 (from Meier et al. 2021) — without any functional data.

**Lesson**: evolutionary models can bootstrap MLDE, providing useful predictions before any experimental measurements, especially for single mutations.

## EVOLVEpro (Hsu et al. 2022 / Liu Lab)

**System**: generalized MLDE framework applied to multiple proteins including T7 RNA polymerase (expanded promoter recognition) and Cas9 (improved PAM flexibility).

**Method**:
1. Systematic collection of training data from deep mutational scanning + targeted measurements
2. Train ensemble of neural networks on ESM-2 embeddings
3. Use ensemble uncertainty for Thompson sampling acquisition
4. Iterate 3–5 rounds; validate top predictions

**Results**:
- T7 RNAP: identified variants with 3-fold improved activity on non-consensus promoters (beyond what prior rational engineering achieved)
- SpCas9: identified single amino acid changes in the REC domain that improve activity at near-PAMless sites by 2-fold vs. SpRY

**Lesson**: MLDE can discover improvements beyond what rational design and classical DE find, particularly for targets where structure-function relationships are poorly understood.

## Protein Stability Engineering: ThermoFisher/Genentech Applications

Multiple pharmaceutical companies now use MLDE internally for stability engineering:

**Antibody thermal stability**: train model on Tm measurements for a family of similar antibody frameworks → predict sequences with Tm > 80°C. Published reports show 2–3 rounds of MLDE consistently finds variants with 10–15°C Tm improvement from antibodies starting at 65°C.

**Standard industrial workflow** (approximate):
1. Measure Tm for 96–384 variants from initial saturation mutagenesis of predicted hotspot positions
2. Train random forest + physicochemical features
3. EI acquisition → 96 new candidates
4. Measure → update model → 2–3 more rounds
5. Top candidate with 10°C Tm improvement confirmed by DSC

## Lessons from Published MLDE Examples

**1. Model accuracy is highly target-dependent**: prediction quality varies from ρ = 0.4 (complex enzymes) to ρ = 0.8 (simple binding proteins). Always validate model accuracy on held-out data before committing to BO.

**2. ESM embeddings improve results across the board**: in every comparison published, ESM-based representations outperform one-hot or physicochemical features, often by 10–20 percentage points in Spearman correlation.

**3. Exploration matters more in early rounds**: pure exploitation (always measure predicted best) is consistently outperformed by UCB or EI acquisition that includes exploration.

**4. MLDE is most valuable when assays are expensive**: for fluorescent screening (low cost/measurement), the overhead of training ML models may not save time. For SPR, calorimetry, or in vivo titer measurements, MLDE's reduction in required measurements is transformative.

**5. The starting library quality matters**: MLDE cannot invent new mutations that weren't in the initial diversity generation. Starting from a saturation mutagenesis library of key positions + initial epPCR variants ensures the training data captures a diverse range of sequence-fitness relationships.

## Why This Matters

Published MLDE demonstrations have moved the field from theoretical promise to practical application in pharmaceutical and industrial biotechnology settings. The key lesson is not that MLDE is universally superior to classical directed evolution — in many contexts, high-throughput screening remains faster and cheaper. Rather, MLDE is transformative in specific conditions: expensive assays, large sequence spaces, and applications where understanding the fitness landscape (not just finding the optimum) is itself valuable. The convergence of large protein language models (ESM-2), principled Bayesian optimization frameworks (BoTorch), and affordable gene synthesis has made MLDE technically accessible to any laboratory — the remaining challenge is knowing when to use it.
