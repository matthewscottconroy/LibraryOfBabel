# Unit 2: Functional Genomics

The Human Genome Project, completed in 2003, gave us the full sequence of the human genome. It was a monumental achievement. And almost immediately, it became clear that the sequence alone was not enough.

Knowing the sequence of every gene is like having the complete script of a play but never being allowed to watch a performance. You know what words the actors could say. You do not know which scenes happen in which acts, which characters are on stage at the same time, or how the same actor delivers different lines in different productions. The genome is the script. What the cell does with it — which genes it turns on, which proteins it makes, which metabolites those proteins produce — is the performance.

Functional genomics is the study of that performance. Rather than asking "what is in the genome?", it asks "what is the genome doing?" The shift in question demands a shift in measurement: from sequencing DNA (which is the same in every cell) to measuring RNA, protein, and metabolite levels (which differ between cell types, tissues, time points, and disease states). This unit covers two chapters that together illuminate the major functional layers of the genome.

## The Layered Logic of Gene Expression

Gene expression is not a single event but a cascade. A gene is first transcribed into RNA — the transcriptome. That RNA is translated into protein — the proteome. Those proteins catalyze reactions that produce and consume small molecules — the metabolome. Each layer depends on the previous one, but each is also regulated independently. You can transcribe a gene and then suppress its translation. You can synthesize a protein and then prevent its activity through post-translational modification. You can have abundant enzyme but restrict its substrate.

This means that measuring any one layer gives you an incomplete picture of the cell's functional state. RNA abundance predicts protein abundance only moderately well — the correlation is typically around 0.4–0.6 across the transcriptome. Protein abundance predicts metabolite levels imperfectly, because enzyme activity is regulated by allosteric effectors, post-translational modifications, and substrate availability. The most complete understanding of what a cell is doing requires measuring at multiple layers simultaneously.

## Why Sequence Is Not Enough

You might expect that knowing the complete sequence of the genome — with all its regulatory regions, promoters, and enhancers — would allow you to predict the transcriptome. It turns out that this is not possible with current knowledge, for a fundamental reason: the regulatory logic that controls which genes are expressed in which cells involves combinatorial protein-DNA and protein-protein interactions that cannot yet be read directly from sequence alone. A promoter that drives liver-specific expression in the presence of HNF4α and C/EBPα will be silent in every other tissue — but the promoter sequence is the same everywhere. What changes is the regulatory environment, which must be measured functionally.

Similarly, knowing protein sequences does not predict which proteins are present at what abundance in a given cell type, which are post-translationally modified, which are assembled into complexes, or which are spatially localized to particular organelles. And knowing the genome's metabolic enzyme-coding genes does not predict the metabolome, because metabolite levels are determined by the integrated activity of hundreds of enzymes working in concert, regulated by thermodynamics, cofactor availability, and competing pathways.

This is the essential case for functional genomics: genome sequence gives you the hardware; functional measurements tell you what the software is doing.

## What This Unit Covers

**Chapter 1.3: Transcriptomics** is the most developed functional genomics layer, with the most mature experimental and computational methods. It covers the full arc of transcriptomics from bulk RNA-seq experimental design through the analysis pipeline and differential expression statistics, to single-cell RNA-seq and its advanced applications: pseudotime analysis, RNA velocity, cell-cell communication inference, and spatial transcriptomics. The chapter also addresses the expanding universe of non-coding RNA measurement: alternative splicing analysis, small RNA-seq, long non-coding RNA analysis, and ribosome profiling. The transcriptome is the most accessible and comprehensive functional layer — it is where functional genomics typically begins.

**Chapter 1.4: Proteomics & Metabolomics** covers the downstream functional layers, where the consequences of transcriptional programs are realized in protein activity and metabolic state. The chapter begins with the physical principles of mass spectrometry — the analytical instrument that underlies both proteomics and metabolomics — then develops shotgun proteomics, quantitative protein measurement, and post-translational modification analysis. Metabolomics follows: targeted vs. untargeted strategies, the NMR vs. MS choice, the LC-MS workflow, metabolite annotation, and metabolic flux analysis. The chapter closes with multi-omics integration: how to combine transcriptomic, proteomic, and metabolomic data into unified models of cellular state.

## The Complementary Relationship

These two chapters are best understood together, not in isolation. Transcriptomics tells you what the cell is trying to do — which programs it has activated at the level of gene expression. Proteomics and metabolomics tell you what the cell is actually accomplishing — what proteins are present and active, what small molecules are being produced and consumed. A transcriptomic signature of metabolic activation and a metabolomic signature of the same pathway provide complementary and mutually validating evidence. Discordance between layers — a transcript highly expressed but its protein product absent — points to post-transcriptional regulation that would be invisible in either data type alone.

The goal of this unit is to give you the conceptual foundations and practical skills to work with both layers, to understand their specific technical challenges, and to see how they fit together in the emerging science of multi-omics.
