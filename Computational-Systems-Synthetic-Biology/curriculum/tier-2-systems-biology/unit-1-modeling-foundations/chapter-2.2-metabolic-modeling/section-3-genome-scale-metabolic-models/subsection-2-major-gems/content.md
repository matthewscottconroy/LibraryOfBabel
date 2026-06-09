# Major Genome-Scale Metabolic Models

## The Ecosystem of GEMs

You will rarely build a GEM from scratch. For the vast majority of organisms you are likely to work with — *E. coli*, yeast, human cells, gut microbes — high-quality, carefully curated models already exist, refined over years by large research communities. Knowing this landscape, and knowing how to choose the right model for your question, is one of the most practical skills in computational metabolic biology.

Hundreds of GEMs have been published for organisms spanning all domains of life. A curated selection of the most important, widely used, and methodologically significant models follows. These are the models researchers actually use, cite, and build upon.

## *Escherichia coli*: The Gold Standard

**iJO1366** (Orth et al. 2011) remains the most thoroughly validated bacterial GEM. Constructed for *E. coli* K-12 MG1655, it covers:
- 1,366 genes (out of ~4,300 total)
- 2,583 reactions, 1,805 metabolites
- 28 subsystems (glycolysis, TCA, amino acid biosynthesis, etc.)
- Validated against 1,500+ growth phenotypes from Biolog phenotype microarray

**iML1515** (Monk et al. 2017) updated iJO1366 with 149 additional genes, improved GPR associations, and revised biomass composition based on updated proteomics. It includes revised bounds for all exchange reactions and improved thermodynamic directionality assignments.

Both models are available at BiGG Models (http://bigg.ucsd.edu) in SBML format.

```python
import cobra

# Load from BiGG via cobra
model = cobra.io.load_model('iJO1366')  # downloads from BiGG
print(f"E. coli iJO1366: {len(model.genes)} genes, {len(model.reactions)} reactions")
```

## *Saccharomyces cerevisiae*: Consensus Modeling

**yeast8** (Lu et al. 2019) is the current consensus model of *S. cerevisiae* metabolism, maintained by the Systems Biology community through a GitHub-based continuous update process. Key features:
- 900+ genes, ~3,900 reactions
- Includes all compartments: cytoplasm, mitochondria, ER, Golgi, vacuole, peroxisome
- Compatible with GECKO for enzyme-constrained modeling
- Used as the base for modeling yeast strains in industrial biotechnology

The yeast consensus model represents a new paradigm: community-maintained models that improve continuously rather than being frozen at publication. Treating a model as a living object — versioned in GitHub, updated as new biochemistry is discovered — is a healthier philosophy than the "publish and archive" approach of early GEM development.

## Human Metabolism: Recon Series

**Recon1** (Duarte et al. 2007): first genome-scale human metabolic reconstruction. Initiated a decade of human metabolomics research.

**Recon2** (Thiele et al. 2013): greatly expanded; included disease-specific pathways.

**Recon3D** (Brunk et al. 2018): 3D structural information for metabolites; 10,600 reactions; includes 3,288 human metabolic genes. Used for:
- Drug target identification (which metabolic enzyme to inhibit for cancer)
- Personalized medicine (patient-specific models from transcriptomics)
- Inborn errors of metabolism (which reactions are non-functional in metabolic diseases)

**AGORA2** (Heinken et al. 2023): collection of 7,302 GEMs for individual gut microbiome species. Each model is manually curated and validated. Used for:
- Microbiome-drug interaction predictions
- Diet-microbiome metabolism
- Community FBA simulations

## *Mycobacterium tuberculosis*: Drug Discovery

**iEK1011** (Kavvas et al. 2018): comprehensive GEM for *M. tuberculosis* H37Rv. Used to identify:
- Metabolic drug targets (reactions essential for growth in macrophage)
- Synergistic antibiotic combinations
- Mechanisms of antibiotic resistance

The tuberculosis GEM has been particularly impactful because *Mtb* is an obligate intracellular pathogen with unique metabolic needs (lipid-rich cell wall, glyoxylate cycle, cholesterol catabolism), and many of its unique metabolic features are captured in the GEM.

## Plant Metabolism

**AraGEM** (*Arabidopsis thaliana*): plant metabolism is highly compartmentalized (chloroplast, mitochondria, peroxisome) and includes unique pathways (C4 photosynthesis, secondary metabolite biosynthesis). Plant GEMs are more complex than bacterial ones and have seen intense development in the 2010s–2020s for crop engineering applications.

## Accessing Models

The primary repositories:

| Repository | URL | Content |
|---|---|---|
| **BiGG Models** | bigg.ucsd.edu | Curated GEMs; standardized identifiers |
| **BioModels** | ebi.ac.uk/biomodels | All types of systems biology models; SBML |
| **ModelSEED** | modelseed.org | Automated GEMs; large collection |
| **EMBL-EBI** | identifiers.org | Metabolite and reaction identifiers |

```python
import cobra

# Access BiGG models
available_models = ['iJO1366', 'iML1515', 'e_coli_core', 'yeast8', 'Recon3D']

# Download and summarize
for model_id in ['iJO1366', 'e_coli_core']:
    m = cobra.io.load_model(model_id)
    print(f"{model_id}: {len(m.genes)}g / {len(m.reactions)}r / {len(m.metabolites)}m")

# The e_coli_core model is ideal for teaching: 72 reactions, 54 metabolites
# Captures central carbon metabolism with full annotation
core = cobra.io.load_model('e_coli_core')
sol = core.optimize()
print(f"Core E. coli growth rate: {sol.objective_value:.4f} h⁻¹")
```

## Choosing the Right Model

The model choice depends on the question:

| Question | Recommended Model |
|---|---|
| Learning FBA | e_coli_core (simple, fast, well-documented) |
| *E. coli* metabolic engineering | iML1515 (most current, curated) |
| Yeast bioprocessing | yeast8 + GECKO |
| Human disease metabolism | Recon3D |
| Gut microbiome ecology | AGORA2 |
| Novel organism (de novo) | CarveMe (auto-reconstruction) |

## Why This Matters

The availability of high-quality GEMs for major organisms has transformed metabolic modeling from a specialized technique to a routine analytical tool. Instead of constructing models from scratch, researchers now begin with a validated GEM and modify it for their specific strain, condition, or question. Understanding the landscape of available models — their sizes, organisms, validation levels, and use cases — is the practical knowledge needed to select the right model and apply it correctly.
