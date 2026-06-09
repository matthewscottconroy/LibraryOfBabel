# Practical Biosafety for Computational Biologists

You might think that working at a computer puts you at a safe remove from the biosafety concerns of the wet lab. No organisms, no cultures, no centrifuge rotors releasing aerosols. In one sense you are right: you are not going to acquire a laboratory infection by analyzing sequence data. But the biosafety landscape for computational biologists is not actually simpler than for experimental researchers — it is differently complicated. When you train a machine learning model on influenza hemagglutinin sequences to predict which mutations increase transmissibility, you are doing work that is directly dual-use. When you develop an algorithm that screens synthesis orders for dangerous sequences, the same algorithm could, if its logic is fully published, be used to understand how to evade screening. When you receive unpublished genome sequences from a BSL-3 pathogen laboratory under a data sharing agreement, you are handling controlled information with real security obligations. The sequence-first era has brought the biosecurity perimeter right into the computational biology workflow.

Computational biologists working in synthetic biology occupy an unusual position in the biosafety landscape: they primarily work with data rather than organisms, yet their work can have significant biosafety and biosecurity implications. The design of CRISPR guide RNAs, the modeling of pathogen evolution, the development of gene synthesis screening algorithms, and the analysis of dangerous pathogen sequences all involve biosecurity-relevant information — even when no organisms are physically handled. This section addresses the specific biosafety and biosecurity responsibilities that computational synthetic biologists carry.

## The "Sequence-First" Problem

Traditional biosafety frameworks were built around physical containment — preventing organisms and biological materials from leaving controlled spaces. These frameworks assume that the primary risk is physical: a vial of pathogen, a culture of bacteria, an engineered organism.

Computational biology inverts this: the primary product of computational work is information (sequences, structures, models, algorithms), not physical materials. And in the post-genomics era, biological information directly enables construction of biological materials. A genome sequence is a blueprint; a directed evolution algorithm is a method for finding more dangerous variants; a pathogen transmissibility model is a map of vulnerabilities.

The **sequence-first problem** is that information biosecurity — preventing dangerous sequence information from being misused — is much harder than physical biosecurity because:
1. Information can be copied and transmitted globally in milliseconds
2. Information is non-rivalrous: sharing it does not diminish the sharer's access
3. The line between dangerous and beneficial information is blurry: the same sequence database that enables vaccine development enables pathogen reconstruction

## What Data Can Computational Biologists Safely Access?

**Open access, no restrictions**:
- Complete genome sequences of non-Select-Agent organisms in public databases (NCBI GenBank, Ensembl, UniProt)
- Published literature on pathogen biology, even when it involves virulence and transmissibility
- Computational tools for sequence analysis, structure prediction, variant calling
- Published algorithmic methods for drug target identification, protein engineering, metabolic pathway design

**Requires institutional oversight but generally accessible**:
- Controlled access databases: some human genomic databases (dbGaP, UK Biobank) require institutional authorization and data use agreements to protect participant privacy — these are privacy issues, not biosecurity issues
- Sequences from Select Agent organisms: publicly available in NCBI (e.g., Ebola genomes are deposited in GenBank) and can be accessed for analysis; the regulatory constraint is on physical possession, not digital access

**Requires institutional biosafety committee or biosecurity review**:
- Designing sequences intended to encode virulence factors or toxins (even for research purposes)
- Developing algorithms specifically for enhancing pathogen transmissibility
- Working with datasets from BSL-3/4 laboratories that contain unpublished pathogen data under data sharing agreements

**Explicitly covered by biosecurity norms**:
- Analyzing Select Agent genome sequences to identify novel virulence mechanisms: legal, but should be disclosed to IBC if part of funded research
- Predicting protein structures of toxins or virulence factors: legal; relevant to vaccine and drug development; should consider dual-use implications of publication

## Dual-Use Considerations in Computational Work

Several categories of computational synthetic biology work carry specific dual-use implications:

### Gain-of-Function Prediction

Machine learning models trained to predict or optimize pathogen fitness, transmissibility, or immune evasion are directly dual-use. If a model can predict which mutations make a pathogen more transmissible — even if developed to inform vaccine design or pandemic preparedness — it can be misused to guide gain-of-function engineering.

**Responsible practice**:
- Include an explicit dual-use assessment in the paper/presentation
- Consider whether the model or its outputs should be restricted (e.g., available to registered researchers only rather than openly deployed as a public API)
- Consult with institutional biosecurity officers before publishing such models

**Examples**: the Bloom lab's work on influenza hemagglutinin fitness landscapes includes discussion of dual-use implications; the paper presents findings in a way that provides public health value (understanding which mutations are beneficial) while not providing a step-by-step guide to enhancing transmissibility.

### Toxin and Virulence Factor Design

Protein design algorithms (RoseTTAFold, AlphaFold2+RosettaDesign, ProteinMPNN) can design novel proteins with arbitrary functions — including functions that do not exist in nature. If applied to designing novel toxins or immunosuppressive proteins, these algorithms could produce highly dangerous sequences.

**Responsible practice**:
- Most protein design researchers focus on beneficial applications; the dual-use risk from standard protein design workflows (designing enzymes, antibodies, fluorescent proteins) is low
- For research involving explicit design of proteins with potential biological activity (antimicrobial peptides, immune-modulating proteins), consider whether sequences should be screened against toxin function databases before publication
- If a designed protein unexpectedly has toxic or pathogenic activity in experimental testing, report this to your IBC and consider whether the design method requires disclosure to biosecurity authorities

### Sequence Screening Algorithm Development

Computational biologists who develop gene synthesis screening algorithms (to identify dangerous sequences) are working directly on biosecurity tools. This creates a positive dual-use situation: the algorithm is itself a biosecurity measure. However:
- Publications describing screening algorithms in detail may help adversaries understand how to evade screening (e.g., what codon shuffling methods would avoid detection)
- The research community developing screening tools is small and well-connected; communication within this community is appropriate, but publications should be reviewed for evasion-enabling content

### Accessing Controlled Pathogen Sequence Data

Researchers who collaborate with BSL-3/4 laboratories sometimes receive unpublished genome sequences or experimental data involving Select Agents or dangerous pathogens under data transfer agreements. Responsibilities:
- Store data in secure systems (institutional compute servers, not personal laptops)
- Do not share with unauthorized parties
- Follow data management requirements of the data transfer agreement
- Understand that you may have access to classified or controlled-unclassified information that carries specific handling requirements

## Practical Security Practices for Computational Researchers

**Data security**:
- Store sensitive biological data (pathogen sequences under data sharing agreements, patient genomic data) on institutional servers with access logging, not on personal devices or unencrypted cloud storage
- Use strong authentication (multi-factor) for accounts with access to sensitive databases
- Follow your institution's data classification and handling policies

**Code and algorithm security**:
- Public GitHub repositories are appropriate for most bioinformatics code, but consider whether code that directly analyzes dangerous pathogen sequences should be restricted to institutional repositories or require authentication to access
- When sharing code that involves pathogen analysis, include documentation that makes the legitimate research purpose clear

**Publication review**:
- Before submitting a paper involving dual-use computational tools or analyses of dangerous pathogens, consider whether the paper should be reviewed for dual-use content beyond standard peer review
- Some journals (Nature Methods, Bioinformatics, PLOS Computational Biology) have explicit policies for dual-use computational biology submissions

## When to Consult Your BSO or IBC

Computational biologists should consult their institutional biosafety officer when:
1. Beginning analysis of Select Agent sequences as part of a funded research project
2. Developing machine learning models intended for pathogen fitness or transmissibility prediction
3. Receiving sensitive biological data from partner institutions under data sharing agreements
4. Designing novel protein sequences with potential toxin or virulence-like activity
5. Collaborating with experimental labs on projects that involve DURC categories

The BSO and IBC are resources, not obstacles. Most computational projects do not require formal review — but the ones that do are consequential enough that the consultation is clearly worthwhile.

## The Dual Responsibility of Computational Synthetic Biologists

Computational synthetic biologists have a **unique dual responsibility** that physical biologists largely do not: they can both create biosecurity risks (through dual-use algorithms and analyses) and provide biosecurity solutions (through screening algorithms, pandemic surveillance tools, viral evolution models that inform public health).

**Biosecurity contributions**:
- Sequence screening algorithms that identify dangerous synthesis orders
- Pandemic surveillance systems that detect novel variants with enhanced fitness
- Structural prediction tools that accelerate vaccine antigen design
- Deep mutational scanning analyses that identify antibody escape mutations — informing both vaccine booster design and pandemic preparedness

Each of these represents a computational biology contribution that directly enhances global biosecurity. Computational biologists who recognize this dual role — as potential risk creators and as active contributors to biosecurity solutions — are the ones most likely to develop both the technical skills and the ethical judgment needed to navigate the field responsibly.

## Why This Matters

Biosafety for computational biologists is not a niche concern — it is increasingly central to the field. As machine learning models become capable of designing novel pathogens, as synthetic DNA becomes a commodity, and as the line between digital biological information and physical biological materials continues to blur, computational biology moves from the periphery of biosecurity to its center. The next generation of biosecurity challenges — AI-designed organisms, computationally optimized toxins, algorithmically generated synthetic biology designs — will be shaped by choices that computational researchers make now about what to build, how to share it, and how to govern it. Understanding biosafety — including its application to computational work — is therefore not just background knowledge. It is a professional competency with direct consequences for how safe and beneficial the synthetic biology era will be.
