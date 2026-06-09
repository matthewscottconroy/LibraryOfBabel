# Learning Sequence

A year-by-year progression assuming ~20 hours/week of focused study and practice.

---

## Year 1 — The Ground

### Goal
Build the mathematical, biological, and computational foundations strong enough that every tier above can stand on them. Do not rush this year.

### Quarter 1: Math and Programming (Months 1-3)
- [0.1 Mathematics](curriculum/tier-0-bedrock/0.1-mathematics.md): Work through calculus, linear algebra, ODEs
  - Resources: MIT OpenCourseWare 18.06 (linear algebra), 18.03 (ODEs)
  - Practice: implement Euler and RK4 solvers by hand before using scipy
- [0.4 Computer Science](curriculum/tier-0-bedrock/0.4-computer-science.md): Python, NumPy, Pandas, Git
  - Practice: solve Project Euler problems 1-20 in Python; build a simple CLI
- Begin [0.3 Biology](curriculum/tier-0-bedrock/0.3-biology.md): Cell biology and molecular biology sections

### Quarter 2: Chemistry and Biology Core (Months 4-6)
- [0.2 Chemistry](curriculum/tier-0-bedrock/0.2-chemistry.md): Biochemistry in full — enzyme kinetics, protein structure, nucleic acid chemistry
  - Practice: fit Michaelis-Menten equations to published kinetic data
- Complete [0.3 Biology](curriculum/tier-0-bedrock/0.3-biology.md): Genetics, microbiology, evolutionary biology
- Begin [1.1 Sequence Analysis](curriculum/tier-1-bioinformatics/1.1-sequence-analysis.md): alignment, BLAST

### Quarter 3: Bioinformatics Core (Months 7-9)
- Complete [1.1 Sequence Analysis](curriculum/tier-1-bioinformatics/1.1-sequence-analysis.md)
- [1.2 Genomics](curriculum/tier-1-bioinformatics/1.2-genomics.md): variant calling, annotation
- [1.3 Transcriptomics](curriculum/tier-1-bioinformatics/1.3-transcriptomics.md): bulk RNA-seq through DESeq2

**Year 1 Project**: Complete end-to-end RNA-seq analysis on a public dataset from GEO
- Raw FASTQ → trimming → alignment → quantification → DESeq2 → GO enrichment
- Write a short report: what genes changed? What pathways?
- This project touches every bioinformatics tier 1 concept in one run

### Quarter 4: Transcriptomics Completion (Months 10-12)
- Complete [1.3 Transcriptomics](curriculum/tier-1-bioinformatics/1.3-transcriptomics.md): single-cell RNA-seq
  - Analyze a 10x Genomics public dataset from 10x Genomics website or GEO
  - Seurat or Scanpy pipeline; annotate cell types
- [1.6 Phylogenetics](curriculum/tier-1-bioinformatics/1.6-phylogenetics.md): build a gene tree; run selection analysis

**Year 1 Milestone**: You can process and analyze any sequencing dataset from raw reads to biological interpretation.

---

## Year 2 — The Structure

### Goal
Learn systems biology modeling and the rest of bioinformatics. Build the first integrative analysis (Cathedral III preview).

### Quarter 5: Systems Biology Foundations (Months 13-15)
- [2.1 Mathematical Modeling](curriculum/tier-2-systems-biology/2.1-mathematical-modeling.md): ODE models in full
  - Read: Tyson, Chen & Novak (2003) — read this paper 3 times
  - Read: Alon *Introduction to Systems Biology* chapters 1-4
  - Practice: implement toggle switch model in Python; find bistability conditions analytically and numerically
- [5.1 Foundational Papers](curriculum/tier-5-literature-research/5.1-foundational-papers.md): Read papers 1-9 (systems biology section)

### Quarter 6: Metabolic Modeling (Months 16-18)
- [2.2 Metabolic Modeling](curriculum/tier-2-systems-biology/2.2-metabolic-modeling.md): FBA through genome-scale models
  - Install COBRApy; run FBA on iJO1366 (E. coli GEM)
  - Reproduce: essential gene list; compare to experimental essentiality data
- [1.4 Proteomics and Metabolomics](curriculum/tier-1-bioinformatics/1.4-proteomics-metabolomics.md)

### Quarter 7: Network Biology (Months 19-21)
- [2.3 Gene Regulatory Networks](curriculum/tier-2-systems-biology/2.3-gene-regulatory-networks.md)
- [4.4 Network Analysis](curriculum/tier-4-computational-tools/4.4-network-analysis.md)
- [2.4 Signaling Networks](curriculum/tier-2-systems-biology/2.4-signaling-networks.md)

**Year 2 Project (Cathedral III preview)**: Multi-omics integration of public E. coli carbon source shift data
- RNA-seq + metabolomics
- Network analysis
- FBA with expression data
- This becomes a publishable analysis

### Quarter 8: Structural Bioinformatics (Months 22-24)
- [1.5 Structural Bioinformatics](curriculum/tier-1-bioinformatics/1.5-structural-bioinformatics.md): AlphaFold2, homology modeling, docking
  - Predict structure for your protein of interest with AlphaFold
  - Run a virtual screen against a published binding site
- [2.5 Multiscale Modeling](curriculum/tier-2-systems-biology/2.5-multiscale-wholecell.md): overview; read Karr et al. 2012

**Year 2 Milestone**: You can model biological networks at the metabolic, genetic regulatory, and signaling levels.

---

## Year 3 — The Engineering Layer

### Goal
Learn synthetic biology from parts to circuits to metabolic engineering. Build a Cathedral I or II project.

### Quarter 9: Synthetic Biology Foundations (Months 25-27)
- [3.1 Genetic Parts and Devices](curriculum/tier-3-synthetic-biology/3.1-genetic-parts-devices.md)
- [3.2 Genetic Circuit Design](curriculum/tier-3-synthetic-biology/3.2-genetic-circuit-design.md)
  - Model: repressilator, toggle switch, FFL pulse generator
  - Read: Gardner et al. (2000), Elowitz & Leibler (2000) in depth
- [3.3 Genome Editing](curriculum/tier-3-synthetic-biology/3.3-genome-editing.md)
  - CRISPR mechanism; guide RNA design; base editors; prime editing

### Quarter 10: Metabolic Engineering (Months 28-30)
- [3.4 Metabolic Engineering](curriculum/tier-3-synthetic-biology/3.4-metabolic-engineering.md)
  - Study artemisinin case in detail; read Keasling 2010
- [3.5 Directed Evolution](curriculum/tier-3-synthetic-biology/3.5-directed-evolution.md)
- [3.6 Cell-Free Systems](curriculum/tier-3-synthetic-biology/3.6-cell-free-systems.md)
- [3.7 Biosafety and Ethics](curriculum/tier-3-synthetic-biology/3.7-biosafety-ethics.md)

### Quarter 11: Scientific Computing for Simulation (Months 31-33)
- [4.1 Scientific Computing](curriculum/tier-4-computational-tools/4.1-scientific-computing.md): stiff solvers, numerical methods for PDEs
  - Implement: min protein oscillation PDE model; compare to published period vs. cell length data
- [4.3 Molecular Dynamics](curriculum/tier-4-computational-tools/4.3-molecular-dynamics.md)
  - Run: first MD simulation of a small protein (villin headpiece or ubiquitin)
  - Analyze: RMSD, RMSF, hydrogen bonds

**Year 3 Project**: Cathedral I (metabolic engineering campaign)
- Choose target molecule; run full FBA analysis
- Compare predictions to literature; write report
- If wet lab access: build one engineered strain

### Quarter 12: ML for Biology (Months 34-36)
- [4.2 Machine Learning for Biology](curriculum/tier-4-computational-tools/4.2-machine-learning-biology.md)
  - Train a random forest on sequence-fitness data
  - Generate ESM-2 embeddings; compare to one-hot baseline
- [4.5 Software Engineering for Research](curriculum/tier-4-computational-tools/4.5-software-engineering-research.md)
  - Package one of your previous analyses as a proper Python package

**Year 3 Milestone**: You can design, model, and evaluate metabolic engineering strategies computationally. You have basic ML and MD capabilities.

---

## Year 4 — Integration and Contribution

### Goal
Choose your focus area; execute one cathedral; publish.

### Quarter 13-14: Cathedral Selection and Deep Dive (Months 37-42)
- Choose your cathedral based on interests and available resources
- Go deep into the relevant literature for that cathedral
- Begin the project
- Seek collaboration or mentorship: find someone doing experimental work who complements your computational skills

### Quarter 15-16: Execution and Publication (Months 43-48)
- Complete the cathedral project
- Write for publication:
  - Methods paper (Cathedral VII)
  - Research article (Cathedrals I-VI)
- Submit preprint to bioRxiv
- Submit to journal
- Present at a conference (if possible: ISCB, SMBM, SynBioBeta, iGEM Jamboree)

**Year 4 Milestone**: One published (or submitted) first-author paper.

---

## Continuous Habits (Every Week, All Years)

- **Read 2 new papers**: 1 in your focus area; 1 outside it
- **Code daily**: even 30 minutes of practice compounds dramatically
- **Maintain a research notebook**: every analysis documented
- **Review foundational papers**: read one canonical paper per month; they reveal new things at each reading level

---

## Common Traps

**Trap 1: Skipping Tier 0**
Jumping to GWAS or ML without solid statistics, or to metabolic modeling without biochemistry. Models built on weak foundations fail silently.

**Trap 2: Breadth without depth**
Reading about everything, mastering nothing. Pick one cathedral after Year 2 and go deep.

**Trap 3: Not making things**
Reading and implementing examples are necessary but not sufficient. You must build something original before you understand at the research level.

**Trap 4: Lone wolf syndrome**
The most important learning happens in collaboration — journal clubs, lab meetings, conference conversations. Find your community.
