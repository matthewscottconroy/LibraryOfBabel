# Carbohydrates

Glucose and cellulose are made of the same monomer — glucose. Every glucosyl unit in a cellulose chain is chemically identical to every glucosyl unit in starch. The difference between a material tough enough to form plant cell walls and a fuel that a human digestive system can process completely reduces to a single stereochemical distinction: whether the glycosidic bond connecting adjacent glucose units is $\alpha$ or $\beta$.

This is a remarkable illustration of what organic chemistry can do with a simple handle. The $\alpha$-1,4 linkage of starch allows the chain to curl into a helix, making it water-soluble and easily accessed by amylases. The $\beta$-1,4 linkage of cellulose keeps the chain extended and rigid, allowing adjacent chains to hydrogen-bond into crystalline microfibrils with tensile strength rivaling steel. Same monomer, same molecular formula, profoundly different material properties — all from the orientation of a single hydroxyl group.

Carbohydrates serve as the primary energy source for most organisms, the structural scaffolding of cell walls and extracellular matrices, and the information-rich glycans that decorate proteins and lipids for cell-cell recognition and signaling. Their chemical diversity — arising from different stereochemistries, ring sizes, and linkage positions — encodes biological information in ways that are only beginning to be computationally tractable.

## Monosaccharides

Monosaccharides are the simplest carbohydrates — single sugar units with the general formula $(CH_2O)_n$. The most important are hexoses ($n = 6$, three key isomers):

**Glucose (Glc):** The primary metabolic fuel; central node of glycolysis, the pentose phosphate pathway, and glycogen metabolism. The aldehyde form ($\alpha$-D-glucopyranose) predominates in aqueous solution as the ring form (95% $\alpha$ and $\beta$ anomers in equilibrium).

**Fructose (Fru):** A ketohexose; found in fruit and honey; forms high-fructose corn syrup. Phosphorylated to fructose-6-phosphate in glycolysis.

**Galactose (Gal):** Differs from glucose only at C4 (epimer). Found in lactose (milk sugar); converted to glucose-1-phosphate by the Leloir pathway.

**Ribose (Rib) and Deoxyribose (dRib):** Pentoses ($n = 5$) that form the backbone of RNA and DNA, respectively. The 2'-hydroxyl distinguishes ribose (RNA) from deoxyribose (DNA).

**The anomeric carbon:** In ring form, the C1 carbon of aldoses (or C2 of ketoses) becomes a new stereocenter — the **anomeric carbon**. The $\alpha$-anomer has the hydroxyl axial (in glucose: below the ring plane in Haworth projection); the $\beta$-anomer has it equatorial (above in Haworth). This distinction determines glycosidic bond type and digestibility.

## Glycosidic Bonds

Monosaccharides are linked by **glycosidic bonds** — covalent bonds between the anomeric carbon of one sugar and a hydroxyl group of another, with release of water.

- **$\alpha$-1,4-glycosidic bonds (starch/glycogen):** Glucose-glucose links in the $\alpha$ configuration, forming helical chains. Digestible by $\alpha$-amylase (humans have multiple $\alpha$-amylase genes, reflecting the evolutionary importance of starch digestion).
- **$\beta$-1,4-glycosidic bonds (cellulose):** Glucose-glucose links in the $\beta$ configuration, forming straight chains that pack into crystalline microfibrils. Humans lack cellulase — indigestible dietary fiber. Some bacteria and termite gut microbes produce cellulases.
- **$\alpha$-1,6-glycosidic bonds (branching):** Creates branch points in glycogen and amylopectin every ~8-12 glucose units for glycogen.

The stereochemistry of the glycosidic bond completely determines the biological properties of the polysaccharide — the same glucose monomer produces an energy store (starch) or a structural material (cellulose) based solely on $\alpha$ vs. $\beta$ linkage.

## Polysaccharides

**Starch:** Storage polysaccharide in plants. Two forms: amylose ($\alpha$-1,4 linear chains, helical) and amylopectin ($\alpha$-1,4 with $\alpha$-1,6 branch points). Rapidly mobilized by amylases and phosphorylases.

**Glycogen:** Storage polysaccharide in animals, analogous to amylopectin but more highly branched (branch point every 8-12 residues vs. 24-30 in amylopectin). Stored primarily in liver and muscle. The shorter chains increase the number of available non-reducing ends for rapid mobilization.

**Cellulose:** Structural polysaccharide in plant cell walls; $\beta$-1,4 glucose. Forms hydrogen-bonded microfibrils with tensile strength approaching that of steel (per weight). Chitin ($\beta$-1,4 N-acetylglucosamine) is the analogous structural polymer in insect exoskeletons and fungal cell walls.

**Hyaluronic acid:** Alternating $\beta$-1,4 GlcA and $\beta$-1,3 GlcNAc; major component of the extracellular matrix (ECM). Can hold enormous amounts of water; provides viscoelasticity to cartilage and synovial fluid.

## Glycosylation of Proteins

Approximately 50% of all human proteins are glycosylated. Protein glycosylation occurs:

**N-linked glycosylation:** Attachment to asparagine (Asn) in the consensus sequence N-X-S/T (where X is any amino acid except Pro). Occurs in the ER lumen. The core glycan is a preassembled 14-sugar structure transferred from a dolichol-phosphate carrier by oligosaccharyltransferase. Trimmed and processed in the ER and Golgi to form high-mannose, hybrid, or complex type glycans.

**O-linked glycosylation:** Attachment to serine or threonine. Sequential addition by glycosyltransferases in the Golgi. Much more diverse than N-glycosylation.

**Functions of glycosylation:**
- **Protein folding:** N-glycans act as chaperone recognition signals (calnexin/calreticulin cycle) — misfolded glycoproteins are retained in the ER
- **Stability:** Glycosylation protects from proteolysis; increases serum half-life of therapeutic proteins
- **Cell-cell recognition:** ABO blood group antigens, selectin ligands (sialyl Lewis X for leukocyte rolling), MHC-peptide presentations
- **Signaling:** O-GlcNAc modification of nuclear/cytoplasmic proteins (competes with phosphorylation at the same or nearby Ser/Thr residues)

## Why This Matters for Computational Biology

Glycobiology is the least computationally tractable part of biochemistry — the "glycome" is vastly more complex than the proteome because glycans are not templated (they are enzymatically assembled with variable branching). But its importance is growing:
- **Glycan databases (GlycoDB, GlycoMod)** and the emerging field of **glycoproteomics** require specialized bioinformatics tools
- **Protein engineering in therapeutic contexts:** Fc-region glycosylation of antibody therapeutics dramatically affects their pharmacokinetics and effector function — something to optimize
- **Metabolic flux through the pentose phosphate pathway** (which produces ribose-5-phosphate for nucleotide biosynthesis and NADPH) is an important metabolic engineering target
- **Lectin-glycan interactions** drive important cell-cell adhesion events modeled in cancer biology and immunology
- Glycolytic enzyme kinetics provide some of the best-characterized examples of allosteric regulation and metabolic control analysis
