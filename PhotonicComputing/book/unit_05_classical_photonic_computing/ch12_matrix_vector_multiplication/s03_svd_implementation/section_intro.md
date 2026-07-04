# Section 12.3: Singular Value Decomposition Implementation

## What This Section Is About

The MZI mesh of Section 12.2 implements unitary matrices — length-preserving rotations of the optical state vector. But neural network weight matrices are not unitary. They stretch some directions, compress others, and are frequently rectangular. The bridge between what optics naturally does (unitary transformations) and what machine learning needs (arbitrary linear maps) is the singular value decomposition: every matrix $W$ factors as $W = U \Sigma V^\dagger$ — a rotation, followed by a per-axis scaling, followed by another rotation. Two MZI meshes and a column of amplitude modulators therefore suffice to implement any matrix.

This section develops the SVD architecture and examines how it has fared in the laboratory and the marketplace:

**12.3.1: SVD and Neural Network Weights** — The mathematical decomposition; parameter counting; why non-unitarity costs optical power; rank truncation as hardware compression; a worked $2 \times 2$ example programmed down to explicit phase angles.

**12.3.2: The Shen et al. 2017 Experiment** — The MIT demonstration that launched the modern photonic deep learning field: a 56-MZI programmable nanophotonic processor performing vowel recognition, its measured accuracy, and the error analysis that set the research agenda for the following decade.

**12.3.3: Commercial Architectures** — Lightmatter, Lightelligence, and the first generation of venture-funded photonic AI companies; how to read a "photonic TOPS" specification critically; and what the industry's partial pivot from computing to interconnect reveals about which problems are actually hard.
