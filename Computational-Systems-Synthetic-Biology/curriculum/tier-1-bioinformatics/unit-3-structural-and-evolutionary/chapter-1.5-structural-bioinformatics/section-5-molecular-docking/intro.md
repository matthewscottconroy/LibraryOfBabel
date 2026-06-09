# Section 5: Molecular Docking

Structure is not static, and proteins are not alone. A protein structure is most biologically interesting in the context of what it binds — its substrates, its cofactors, its regulatory partners, its inhibitors. The question "how does this protein bind that molecule?" is one of the most practically important questions in all of biology, and it underlies the central challenge of drug discovery: how do you design a small molecule that fits precisely into a specific protein's binding site with high affinity and selectivity?

Molecular docking is the computational approach to answering this question. Given a 3D structure of a protein and a library of candidate ligands, docking predicts how each ligand would orient itself in the binding site and estimates how favorable that interaction would be. The predictions are approximate — docking involves substantial simplifications compared to the true physics of protein-ligand binding — but they are fast enough to screen millions of compounds computationally and accurate enough to enrich true binders in the top-ranked results. This combination of speed and directional accuracy makes docking one of the most practically useful tools in computational biology.

This section covers four interconnected aspects of docking. First, the conceptual foundations: what docking is trying to do, why it is hard (the search problem, the scoring problem, the conformational change problem), and how to interpret its outputs appropriately. Docking is a coarse filter, not a precise oracle, and understanding this distinction prevents a great deal of wasted effort and inappropriate interpretation.

Second, the protocols: the detailed preparation steps for receptor and ligand that are essential prerequisites for any reliable docking result. Poor preparation is the leading cause of docking failure, and knowing what to check and fix before running the algorithm is where most of the practical skill lies.

Third, scoring functions: the component that evaluates candidate poses. This is both the most important and the most poorly solved part of docking — the range of scoring function types (force-field, empirical, knowledge-based, machine learning) reflects the difficulty of accurately estimating binding free energy at millisecond-per-compound computational speed.

Fourth, virtual screening: the practical application of docking at scale, organized as a hierarchical funnel from millions of compounds down to dozens that are worth synthesizing and testing. The metrics (enrichment factor, ROC-AUC) and workflow (HTVS → SP → XP → MM-GBSA → visual inspection → experiment) are the practical vocabulary of computational drug discovery.

Together, these four topics give you a complete, realistic picture of what structure-based virtual screening can and cannot accomplish — the foundation for participating meaningfully in any computational drug discovery project.
