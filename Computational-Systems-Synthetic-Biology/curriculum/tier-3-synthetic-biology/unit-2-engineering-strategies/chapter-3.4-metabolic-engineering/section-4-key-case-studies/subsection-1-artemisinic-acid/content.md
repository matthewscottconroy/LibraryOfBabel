# Case Study: Artemisinic Acid

In 2013, a paper in *Nature* described a yeast strain capable of producing 25 grams of artemisinic acid per liter of fermentation broth. That number may not sound striking until you know what artemisinic acid is, where it used to come from, and what it took to get there. Artemisinic acid is the biosynthetic precursor to artemisinin, the most effective antimalarial drug known — and until that work, the world's entire supply came from hand-harvested leaves of a tropical shrub. The Keasling lab's engineering of yeast to produce artemisinic acid is the most celebrated success story in metabolic engineering. It demonstrates that a complex natural product, previously available only by extraction from a plant that is difficult to cultivate, can be produced reliably and at commercial scale using engineered microorganisms.

## The Problem: Artemisinin Supply and Access

**Artemisinin** is a sesquiterpenoid endoperoxide isolated from the plant *Artemisia annua* (sweet wormwood). It is the most effective antimalarial drug known, particularly against drug-resistant *Plasmodium falciparum*, and forms the basis of all WHO-recommended artemisinin combination therapies (ACTs).

**Supply problem**: artemisinin is produced by the plant only at 0.01–1% dry weight. Cultivation of *A. annua* requires tropical climate, specific agricultural conditions, and 12–18 months from planting to harvest. The supply chain is dominated by smallholder farmers in sub-Saharan Africa and Asia, and the price fluctuates dramatically with growing season conditions, creating shortfalls in the exact years when malaria burden is highest.

The price per unit of artemisinin from plant extraction ranges from \$200–700/kg depending on the year — putting it out of reach for many patients in the countries with highest malaria burden.

**The Keasling group's goal** (Jay Keasling, UC Berkeley): engineer *Saccharomyces cerevisiae* to produce artemisinic acid at titers that could supply >100 million doses per year, at prices competitive with plant-derived material.

## The Biosynthetic Pathway

### From Acetyl-CoA to FPP (Mevalonate Pathway)

The mevalonate (MVA) pathway converts acetyl-CoA to isopentenyl pyrophosphate (IPP) and dimethylallyl pyrophosphate (DMAPP), which condense to form farnesyl pyrophosphate (FPP, C15):

$$2 \text{ Acetyl-CoA} \rightarrow \text{Acetoacetyl-CoA} \rightarrow \text{HMG-CoA} \xrightarrow{\text{HMGR}} \text{Mevalonate} \rightarrow \text{IPP/DMAPP} \rightarrow \text{FPP}$$

This pathway is native to yeast. The **rate-limiting step** is HMG-CoA reductase (HMGR), subject to strong feedback regulation.

### From FPP to Artemisinic Acid (Heterologous Steps)

FPP is converted to artemisinic acid in three steps using plant enzymes:

1. **FPP → amorphadiene** (C15 sesquiterpene): catalyzed by **amorphadiene synthase (ADS)** from *A. annua*. ADS is a class I terpene cyclase with kcat = 0.008 s⁻¹ (slow).

2. **Amorphadiene → artemisinic alcohol/aldehyde/acid**: catalyzed by a cytochrome P450, **CYP71AV1**, also from *A. annua*. CYP71AV1 performs three sequential oxidations:
   - Amorphadiene → amorphadienol (by hydroxylation)
   - Amorphadienol → amorphadienealdehyde (by further oxidation)
   - Amorphadienealdehyde → artemisinic acid (by third oxidation)
   CYP71AV1 requires electron supply from a P450 reductase (CPR1 from *A. annua* or yeast ATR2).

3. The final step from artemisinic acid to artemisinin requires a unique chemical dihydroperoxide cyclization, which is currently performed by chemical synthesis outside the yeast cell.

### Stoichiometry

$$1 \text{ FPP} + 3 \text{ O}_2 + 3 \text{ NADPH} \rightarrow \text{Artemisinic acid} + 3 \text{ H}_2\text{O}$$

The 3 NADPH requirement makes NADPH availability critical in the production strain.

## Engineering Steps: From 0.025 mg/L to 25 g/L

The Keasling group's engineering over 2003–2013 spans 10 years and illustrates the iterative nature of complex metabolic engineering:

### Phase 1 (2003): Proof of Concept in *E. coli*

Martin et al. (2003) introduced the complete MVA pathway from *S. cerevisiae* into *E. coli* (7 genes) plus ADS from *A. annua*. Produced 0.025 mg/L amorphadiene. This proved the heterologous pathway concept but titer was ~1000× below commercial viability.

### Phase 2 (2006): Engineering in Yeast (Ro et al., Nature)

Switched to *S. cerevisiae* (which has native MVA pathway):
- Overexpressed **tHMGR** (truncated HMGR, lacking sterol feedback domain) from a high-copy plasmid → 5-fold increase in FPP production
- Added ADS from *A. annua*
- Added CYP71AV1 + CPR1 for artemisinic acid production
- Downregulated ERG9 (squalene synthase) with repressible promoter (MET3) to reduce sterol competition for FPP

**Result**: 115 mg/L artemisinic acid in fed-batch. Still below commercial target but demonstrated the complete pathway.

### Phase 3 (2013): Commercial-Scale Engineering (Paddon et al., Nature)

Comprehensive optimization over 7 years:
- Multiple integration of tHMGR (8 copies) → massive HMGR overexpression
- Overexpression of farnesyl pyrophosphate synthase (ERG20) for FPP production
- ERG9 knockdown maintained; additional sterol biosynthesis flux reduced
- Optimized CYP71AV1 expression + co-expression of cytochrome b5 (CYB5) as electron shuttle to increase P450 activity
- Added alcohol dehydrogenase (ADH1) and aldehyde dehydrogenase (ALD2) for intermediate oxidation
- Optimized fermentation: fed-batch, pH control, oxygen regulation
- Added **artemisinic acid transporter** to secrete product and reduce toxicity

**Final result**: 25 g/L artemisinic acid in 40-hour fed-batch fermentation.

**Commercial impact**: Sanofi licensed the technology. By 2013, semi-synthetic artemisinin from fermentation-derived artemisinic acid was available. The process produces ~35 tons/year — equivalent to ~100 million doses of artemisinin.

## Lessons Learned

**1. Rate-limiting enzymes require extreme overexpression**: tHMGR required 8 integrated copies to achieve sufficient HMG-CoA → mevalonate flux. No single optimization made it work; accumulation of improvements was essential.

**2. P450s require their own electron supply**: CYP71AV1 is a membrane-associated P450 that requires a cytochrome P450 reductase (CPR1) partner for electron supply. Co-expressing the wrong CPR or incorrect stoichiometry of CPR:P450 dramatically reduces activity. Cytochrome b5 (CYB5) as additional electron carrier increased P450 activity 3-fold.

**3. Product toxicity requires active management**: artemisinic acid at high concentrations is toxic to yeast. A secretion strategy (identifying or overexpressing a suitable transporter) was required to reduce intracellular accumulation.

**4. Competing pathways are not just about flux**: ERG9 (squalene synthase) competes for FPP. Knocking it out completely kills yeast (sterols are essential). The MET3-based conditional repression (off in methionine-free medium, on with methionine) is a creative solution to partially limit a competing essential pathway.

## Why This Matters

The artemisinic acid project is the canonical demonstration that metabolic engineering can address global health challenges at scale. Starting from a plant-derived compound requiring tropical agriculture, the Keasling group engineered a fermentation process capable of supplying the global artemisinin market. The technical lessons — MVA pathway amplification, P450 expression, competitive pathway management, product toxicity — recur in every complex terpenoid engineering project. The timeline (10 years from proof of concept to commercial production) also sets realistic expectations: unlike simple plasmid expression of a single enzyme, complex pathway engineering requires sustained, iterative optimization. The artemisinin project is the reference point against which all subsequent metabolic engineering efforts are measured.
