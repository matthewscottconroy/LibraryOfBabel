# Key Resources by Domain

---

## Primary Texts

| Domain | Primary | Supplementary |
|---|---|---|
| Biochemistry | Lehninger *Principles of Biochemistry* (Nelson & Cox) | Stryer *Biochemistry* |
| Cell/Molecular Biology | Alberts *Molecular Biology of the Cell* | Lodish *Molecular Cell Biology* |
| Systems Biology | Alon *An Introduction to Systems Biology* (2nd ed.) | Strogatz *Nonlinear Dynamics and Chaos* |
| Bioinformatics | Durbin et al. *Biological Sequence Analysis* | Pevzner *Bioinformatics Algorithms* |
| Statistics | Efron & Hastie *Computer Age Statistical Inference* | Holmes & Huber *Modern Statistics for Modern Biology* |
| Synthetic Biology | Brophy & Voigt (2014 review) + iGEM learning resources | |
| Metabolic Modeling | Palsson *Systems Biology: Constraint-based Reconstruction* | COBRApy documentation |
| MD Simulation | Frenkel & Smit *Understanding Molecular Simulation* | GROMACS tutorials |
| ML | Bishop *Pattern Recognition and Machine Learning* | Goodfellow *Deep Learning* |
| Python | Python docs; NumPy docs | *Scientific Python Lectures* (free online) |

---

## Online Courses

| Course | Platform | Topics |
|---|---|---|
| MIT 18.06 Linear Algebra (Gilbert Strang) | OCW | Linear algebra |
| MIT 18.03 ODEs | OCW | Differential equations |
| Bioinformatics Specialization (Pevzner) | Coursera | Algorithms for bioinformatics |
| Statistical Learning (Hastie & Tibshirani) | Stanford Online / YouTube | Machine learning theory |
| Deep Learning Specialization (Ng) | Coursera | Neural networks |
| Systems Biology course (Uri Alon) | YouTube (Weizmann) | Network motifs, synthetic biology |
| EMBL-EBI training | EMBL-EBI website | Bioinformatics tools, databases |

---

## Software Tools Reference

### Bioinformatics
| Tool | Purpose | Language |
|---|---|---|
| FastQC / MultiQC | Sequencing QC | Python |
| fastp | Read trimming | C++ |
| BWA-MEM2 | DNA read alignment | C++ |
| STAR | RNA-seq alignment | C++ |
| samtools / bcftools | BAM/VCF processing | C |
| bedtools | Genomic intervals | C++ |
| GATK | Variant calling | Java |
| DESeq2 | Differential expression | R/Bioconductor |
| Seurat / Scanpy | Single-cell analysis | R / Python |
| HMMER | Profile HMM search | C |
| MAFFT | Multiple alignment | C |
| IQ-TREE2 | Phylogenetics | C++ |
| BEAST2 | Bayesian phylogenetics | Java |

### Systems Biology
| Tool | Purpose | Language |
|---|---|---|
| COPASI | ODE simulation + analysis | C++ / Python |
| tellurium | SBML + Antimony simulation | Python |
| COBRApy | Metabolic modeling (FBA) | Python |
| libSBML | SBML read/write | C++ / Python |
| BioNetGen | Rule-based modeling | Perl / Python |
| Smoldyn | Spatial stochastic | C / Python |
| VCell | Spatial deterministic + stochastic | Java GUI |
| XPPAUT | Phase plane + bifurcation | C |

### Computational Tools
| Tool | Purpose | Language |
|---|---|---|
| SciPy | ODE solving, optimization, statistics | Python |
| DifferentialEquations.jl | ODE/SDE/DDE solving | Julia |
| GROMACS | Molecular dynamics | C++ |
| OpenMM | Molecular dynamics | Python / C++ |
| MDAnalysis | Trajectory analysis | Python |
| PyTorch | Deep learning | Python |
| JAX | Differentiable programming | Python |
| NetworkX | Graph analysis | Python |
| Snakemake | Workflow management | Python |

---

## Databases

### Sequence and Genome
| Database | URL | Content |
|---|---|---|
| NCBI GenBank | ncbi.nlm.nih.gov | Nucleotide sequences |
| UniProt | uniprot.org | Protein sequences + function |
| Ensembl | ensembl.org | Annotated eukaryotic genomes |
| NCBI RefSeq | ncbi.nlm.nih.gov/refseq | Reference sequences |

### Structure
| Database | URL | Content |
|---|---|---|
| RCSB PDB | rcsb.org | Experimental protein structures |
| AlphaFold DB | alphafold.ebi.ac.uk | Predicted structures for most proteins |
| SCOP | scop.mrc-lmb.cam.ac.uk | Structural classification |
| CATH | cathdb.info | Domain classification |

### Function
| Database | URL | Content |
|---|---|---|
| KEGG | kegg.jp | Metabolic pathways, orthology |
| Reactome | reactome.org | Human pathways (curated) |
| Gene Ontology | geneontology.org | Functional annotation |
| InterPro / Pfam | ebi.ac.uk/interpro | Protein domains |
| MetaCyc | metacyc.org | Metabolic pathways from literature |

### Metabolic Models
| Database | URL | Content |
|---|---|---|
| BiGG Models | bigg.ucsd.edu | Curated GEMs + standard IDs |
| BioModels | ebi.ac.uk/biomodels | SBML models from publications |
| BRENDA | brenda-enzymes.org | Enzyme kinetic parameters |
| eQuilibrator | equilibrator.weizmann.ac.il | Thermodynamic data (ΔrG°') |

### Variants and Population
| Database | URL | Content |
|---|---|---|
| dbSNP | ncbi.nlm.nih.gov/snp | Common variants |
| ClinVar | ncbi.nlm.nih.gov/clinvar | Clinically significant variants |
| gnomAD | gnomad.broadinstitute.org | Population allele frequencies |
| COSMIC | cancer.sanger.ac.uk/cosmic | Somatic mutations in cancer |

### Synthetic Biology
| Database | URL | Content |
|---|---|---|
| iGEM Registry | parts.igem.org | BioBrick parts |
| JBEI ICE | public-registry.jbei.org | Plasmid registry |
| SynBioHub | synbiohub.org | Synthetic biology design repository |
| NCBI SRA | ncbi.nlm.nih.gov/sra | Raw sequencing data |
| GEO | ncbi.nlm.nih.gov/geo | Processed expression data |

---

## Communities and Conferences

### Conferences
- **ISCB/ISMB**: International Society for Computational Biology — main computational biology conference
- **RECOMB**: Research in Computational Molecular Biology
- **SynBioBeta**: synthetic biology industry/research
- **Metabolic Engineering (ME)**: Gordon conference + ACS symposium
- **EMBO Systems Biology**: European focus
- **iGEM Jamboree**: undergraduate/graduate synthetic biology competition

### Online Communities
- **Biostars**: Q&A for bioinformatics
- **SEQanswers**: sequencing-focused forum
- **reddit.com/r/bioinformatics**: general bioinformatics discussion
- **Twitter/X biology lists**: preprint alerts, lab discussions
- **GitHub**: where most tools live; issues = best support forum for tools

### Journals to Follow
| Journal | Focus |
|---|---|
| *Nature Methods* | Significant new methods |
| *Nature Biotechnology* | Applied/translational methods |
| *Molecular Systems Biology* | Systems biology research |
| *ACS Synthetic Biology* | Synthetic biology |
| *Nucleic Acids Research* (annual database issue) | Databases and web servers |
| *Bioinformatics* | Computational methods |
| *PLOS Computational Biology* | Broad computational biology |
| *Cell Systems* | Systems-level cell biology |
| *eLife* | Open access; broad scope |
