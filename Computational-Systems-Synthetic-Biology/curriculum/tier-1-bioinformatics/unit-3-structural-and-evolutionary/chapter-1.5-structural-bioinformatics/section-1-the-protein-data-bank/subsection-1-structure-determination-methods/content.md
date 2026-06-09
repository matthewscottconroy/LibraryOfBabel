# Structure Determination Methods

Here is a remarkable fact to hold in your mind: in 1958, John Kendrew published the first atomic-resolution structure of a protein — myoglobin — and received the Nobel Prize for it. The structure took years of painstaking experiment to solve and filled the pages of Nature with a hand-built, three-dimensional wire model constructed from measured diffraction data. Today, a graduate student can download a structure of almost any protein in minutes, and for most proteins the structure was never even experimentally determined — it was predicted by a neural network. That transition, from years of crystallographic labor to overnight prediction, is the central story of this chapter. But to understand where the field has arrived, you need to understand where it came from. The experimental methods that built the Protein Data Bank — X-ray crystallography, cryo-electron microscopy, and NMR spectroscopy — remain the bedrock of structural biology. They provide the ground truth that computational methods are trained and validated against.

Understanding these three methods is not merely historical. It turns out that the way a structure was determined shapes what you can trust about it, what artifacts might be present, and which biological questions it can and cannot answer. The Protein Data Bank now contains more than 220,000 structures, but these structures are not all equal, and knowing how they were made is the first step in knowing how to use them.

## X-ray Crystallography

**X-ray crystallography** has been the dominant structure determination method since Perutz and Kendrew solved the first protein structures in the 1950s–60s. The method requires growing a protein crystal — a highly ordered, periodic array of protein molecules — and illuminating it with X-rays.

Why does it work? Proteins are far too small to image directly with light — even the best optical microscopes cannot resolve features smaller than the wavelength of visible light (~400–700 nm), and a protein's atomic bonds are roughly a thousand times smaller than that, at ~1.5 Å (0.15 nm). But X-rays have wavelengths on the order of interatomic distances, and crystalline order amplifies the signal by constructive interference across billions of identically oriented protein molecules simultaneously. A crystal is not just a container; it is an atomic-scale diffraction amplifier.

**Principle**: X-rays are diffracted by the electrons in the crystal. The diffraction pattern (collected by a detector) is the **Fourier transform** of the electron density in the crystal. By collecting diffraction data at many crystal orientations (rotating the crystal), the complete 3D Fourier transform is obtained. Inverse Fourier transformation of the structure factors (amplitudes + phases) yields the electron density map, from which the atomic model is built.

**The phase problem**: This is where crystallography gets subtle. The detector records intensities (|F|²) but not phases. Phases are recovered by: (1) **MIR** (multiple isomorphous replacement — soak crystals with heavy atoms to perturb phases); (2) **MAD/SAD** (anomalous diffraction from selenium-methionine or other anomalous scatterers); (3) **molecular replacement** (use a known similar structure as a starting phase estimate, then refine). The phase problem is why protein crystallography was once an art form — the experimental and computational gymnastics required to recover phase information from intensity-only data drove decades of method development. It is also why AlphaFold2's ability to produce nearly correct structures as molecular replacement probes has quietly accelerated experimental crystallography even in the deep learning era.

**Quality metrics**:
- **Resolution** (Å): The minimum distance between resolvable features. 1.0–1.5 Å = excellent (side chain details clear, water positions visible); 2.0–2.5 Å = good (backbone well-defined, most side chains clear); >3.0 Å = limited (backbone visible, side chains uncertain).
- **R-factor** ($R = \sum|F_{\text{obs}} - F_{\text{calc}}|/\sum F_{\text{obs}}$): The discrepancy between observed and model-calculated diffraction intensities. Well-refined structures: R ~0.15–0.22.
- **R_free**: R-factor calculated on a subset (~5%) of reflections excluded from refinement. Must be close to R-factor; R_free − R > 0.05 suggests overfitting.

## Cryo-Electron Microscopy (Cryo-EM)

**Cryo-EM** has transformed structural biology since the "resolution revolution" in 2013. For the first forty years after Perutz and Kendrew, crystallography reigned almost unchallenged. Then cryo-EM arrived and changed everything. The 2017 Nobel Prize went to Jacques Dubochet, Joachim Frank, and Richard Henderson for developing the technology. What happened? Better detectors and better algorithms.

The key conceptual shift is this: instead of forcing a protein to crystallize — a notoriously fickle process that fails for many important proteins, including most membrane proteins and large dynamic complexes — cryo-EM works with proteins in near-native solution conditions. Rather than growing crystals, proteins in solution are **vitrified** — rapidly plunged into liquid ethane to form amorphous ice (not crystalline, which would damage the protein) — and imaged in a transmission electron microscope at ~−170°C.

You might expect that individual protein molecules would be too faint to image. And you'd be right — a single protein molecule is nearly invisible against the noise floor of the detector. It turns out the solution is to collect images of millions of particles, then use statistical averaging to extract signal from noise.

**Single-particle analysis**: Tens of thousands to millions of 2D projection images of individual protein particles in random orientations are collected. Computational methods (RELION, cryoSPARC) classify and align these images, determine the orientation of each particle, and reconstruct a 3D electron density map by back-projection.

**Resolution criterion**: The **FSC (Fourier Shell Correlation) 0.143 criterion** defines resolution as the spatial frequency at which the correlation between two half-dataset reconstructions falls below 0.143.

Modern cryo-EM routinely achieves 2–3 Å resolution for well-behaved specimens, and sub-2 Å resolution is increasingly common. Structures of flexible, heterogeneous complexes (ribosomes, ion channels, GPCRs, viral capsids) that cannot be crystallized are now accessible. The ability to image proteins in multiple conformational states (by 3D classification of particles) makes cryo-EM uniquely powerful for mechanistic studies. The structure of the spliceosome — a 1.5 MDa RNA-protein machine with many flexible components — would have been nearly impossible to determine by crystallography. Cryo-EM delivered it at near-atomic resolution, opening an entire new chapter in pre-mRNA splicing biology.

## NMR Spectroscopy

**NMR spectroscopy** determines protein structure in solution, avoiding the need for crystallization. It is the only method that gives you a structure *and* dynamics simultaneously, making it uniquely suited for asking questions about motion, disorder, and the timescales on which proteins change conformation. The protein must be isotopically labeled (¹⁵N, ¹³C, ²H) for modern multidimensional NMR experiments (³D/⁴D heteronuclear NMR). Structure determination relies on:

- **Chemical shifts**: The frequency of NMR signals encodes local secondary structure (α-helix vs. β-strand recognition by characteristic ¹³Cα shifts).
- **NOE** (Nuclear Overhauser Effect): Cross-peaks in 2D NOESY spectra arise between protons within ~5 Å of each other. These **NOE distance constraints** define which parts of the protein are spatially close.
- **Dihedral angle constraints**: From J-coupling constants (relating to backbone dihedral angles φ, ψ).

An ensemble of ~20 structures consistent with all constraints is reported (not a single best structure), reflecting both measurement uncertainty and genuine conformational flexibility. This ensemble is not a sign of failure — it is a feature. The spread of the ensemble at different positions tells you which regions are rigidly ordered and which are genuinely disordered in solution, information that no single crystal structure can provide.

**Best application**: Proteins < 50 kDa in solution. NMR is uniquely suited for studying protein dynamics, intrinsically disordered proteins, and protein-ligand interactions (chemical shift perturbation mapping). When a drug candidate changes the chemical shifts of specific residues in a target protein, NMR tells you exactly which residues are affected — a direct readout of the binding interface.

## Comparison of Methods

| Feature | X-ray | Cryo-EM | NMR |
|---|---|---|---|
| Sample state | Crystal | Vitrified solution | Solution |
| Protein size | Any (small–large) | >100 kDa preferred | <50 kDa |
| Resolution range | 0.6–3.5 Å | 1.8–10 Å | 1–3 Å (NOE-based) |
| Crystallization required | Yes | No | No |
| Dynamic information | Limited | Limited | Rich |
| Ligand binding study | Crystal soaking | Feasible | CSP mapping |
| Typical timeline | Weeks–months | Days–weeks | Months |

## Why This Matters

Understanding how structures are determined is not an academic exercise. It shapes every decision downstream: which structures to trust, which regions to treat with caution, which method would be most informative for a new biological question. Structure determination methods provide the experimental foundation for mechanistic understanding of how proteins function — how enzymes catalyze reactions, how receptors bind ligands, how molecular machines generate force — and all structure-based drug discovery campaigns begin with an experimentally determined or computationally predicted structure of the target. Every structure in the Protein Data Bank began with one of the three methods described here. When you download a PDB file, you are reading the compressed result of someone's months or years of experimental work. Knowing what that work involved lets you use the result wisely.
