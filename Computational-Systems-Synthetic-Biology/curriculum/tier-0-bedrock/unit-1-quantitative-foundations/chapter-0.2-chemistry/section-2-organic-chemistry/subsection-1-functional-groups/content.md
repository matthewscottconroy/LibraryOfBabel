# Functional Groups in Biochemistry

In 1828, Friedrich Wöhler synthesized urea from ammonium cyanate — inorganic starting materials — and shattered the doctrine that biological molecules required some special "vital force" to create them. What he demonstrated, though he could not have articulated it this way, is that biochemistry is just chemistry: the same electron-pushing principles, the same reactive groups, the same reaction types. The molecules in your cells differ from those on a chemist's shelf mainly in their specificity, their organization, and the elegant context in which they operate.

Organic chemistry is the chemistry of carbon-based molecules, and functional groups are the reactive units that define molecular behavior. Every biomolecule — amino acid, nucleotide, lipid, carbohydrate, coenzyme — is characterized by its functional groups. Understanding functional group chemistry is the key to understanding why biomolecules react the way they do, how enzymes work, and how to design molecules for synthetic biology applications. When you encounter a new molecule in the biochemical literature, the first question to ask is always: what functional groups does it carry, and what can each one do?

## Carboxylic Acids (–COOH)

The carboxylate group consists of a carbonyl (C=O) and a hydroxyl (–OH) attached to the same carbon. In water at physiological pH, carboxylic acids with pKa ~4 are predominantly deprotonated (carboxylate anion, –COO$^-$), bearing a negative charge.

**Biological roles:**
- The C-terminus of every polypeptide chain
- The side chains of Asp and Glu (acidic residues)
- Metabolic intermediates: citrate, succinate, fumarate in the TCA cycle
- Acyl groups for thioester bonds in CoA metabolism

**Key reaction:** **Peptide bond formation** is a condensation reaction between a carboxylate and an amine, releasing water. This reaction is thermodynamically unfavorable in water ($\Delta G^{\circ'} \approx +21$ kJ/mol) but is driven to completion on the ribosome by ATP-driven amino acid activation (aminoacyl-AMP and then aminoacyl-tRNA).

## Amines (–NH$_2$)

Primary amines ($-\text{NH}_2$) are basic and nucleophilic. At physiological pH, amino groups with pKa ~10 are protonated ($-\text{NH}_3^+$, positive charge).

**Biological roles:**
- The N-terminus of every polypeptide chain
- Lysine and arginine side chains (positive charge at physiological pH)
- The exocyclic amino groups on adenine, guanine, and cytosine bases (important for H-bonding in Watson-Crick base pairs)
- Amino sugars (glucosamine in peptidoglycan, chitin)

**Key reactions:** Transamination (amino group transfer in amino acid biosynthesis/catabolism), Schiff base formation (with aldehydes — mechanism of pyridoxal phosphate-dependent enzymes).

## Hydroxyl Groups (–OH)

Hydroxyl groups are polar and can form hydrogen bonds as both donors and acceptors. They are the targets of phosphorylation (serine, threonine, tyrosine kinases).

**Biological roles:**
- Serine, threonine, tyrosine: phosphorylation targets in signaling cascades
- Ribose vs. deoxyribose distinction: the 2'-OH of ribose makes RNA labile to hydrolysis; DNA's 2'-H makes it more stable
- Sugar hydroxyls: provide hydrogen bonding sites for glycan-protein interactions and form glycosidic bonds

It turns out that the seemingly minor difference between a 2'-OH (RNA) and a 2'-H (DNA) has enormous consequences for the molecule's half-life. At neutral pH, the 2'-OH of RNA can act as an intramolecular nucleophile, attacking the adjacent phosphodiester bond. This makes RNA intrinsically labile — a half-life of hours to months depending on conditions, compared to the geological stability of DNA. The same chemical feature that makes RNA more reactive also makes it a better catalyst: many ribozymes use the 2'-OH as a nucleophile in catalysis. Chemistry has consequences.

## Thiol Groups (–SH)

Cysteine's thiol group has unique reactivity — it is an excellent nucleophile and can be oxidized to form **disulfide bonds** (–S–S–). Thiol groups are soft nucleophiles that react readily with electrophilic carbon and with metal ions.

**Biological roles:**
- **Disulfide bonds:** Stabilize extracellular proteins (immunoglobulins, insulin) in the oxidizing environment of the ER and extracellular space
- **Metal coordination:** Zinc finger proteins (Cys and His coordinate Zn$^{2+}$); iron-sulfur clusters in electron transport
- **Thioester bonds in CoA:** The high-energy thioester bond carries acyl groups in fatty acid metabolism and the TCA cycle

## Carbonyl Groups (Aldehydes and Ketones)

The carbonyl group (C=O) is **electrophilic** at the carbon. Aldehydes (R–CHO) are more reactive than ketones (R–CO–R) because the carbon is less sterically hindered.

**Biological roles:**
- Reducing sugars (glucose, fructose) have aldehyde or ketone groups (in open-chain form)
- The Schiff base intermediate in many enzyme mechanisms (PLP enzymes, Class I aldolases)
- Products of lipid peroxidation (malondialdehyde, 4-hydroxynonenal) — reactive species that modify proteins and DNA

## Phosphate Esters

Phosphate groups (–OPO$_3^{2-}$) carry two negative charges at physiological pH and form ester bonds to hydroxyl groups. They are the backbone of DNA and RNA and are central to energy metabolism and signaling.

**Biological roles:**
- **Nucleic acid backbone:** Phosphodiester bonds link nucleotides in DNA and RNA
- **Energy currency:** ATP (three phosphate groups connected by high-energy anhydride bonds)
- **Signaling:** Protein phosphorylation (serine/threonine/tyrosine), second messengers (cAMP, IP3, PIP2/PIP3)
- **Metabolic intermediates:** Glucose-6-phosphate, phosphoenolpyruvate, 1,3-bisphosphoglycerate

The **high energy** of ATP's anhydride bonds arises from charge repulsion between the adjacent phosphate groups and resonance stabilization of the hydrolysis products.

## Worked Example: Serine Protease Mechanism

The catalytic triad of serine proteases (His, Asp, Ser) illustrates how functional groups cooperate in enzyme catalysis:

1. His (acting as general base, pKa ~6) deprotonates the Ser–OH, making it a better nucleophile
2. Asp stabilizes the positive charge on His through an H-bond
3. Ser–O$^-$ performs nucleophilic attack on the peptide carbonyl carbon
4. A tetrahedral intermediate forms; oxyanion hole stabilizes the negative charge on oxygen
5. The peptide bond breaks; the N-terminal fragment leaves; the acyl-enzyme intermediate is hydrolyzed
6. Water attacks the acyl-enzyme, regenerating free enzyme

This mechanism demonstrates carboxylate (Asp), amine/imidazole (His), and hydroxyl (Ser) chemistry working in concert — the textbook example of functional group cooperation in enzyme catalysis. What makes it remarkable is how elegantly each group is positioned to play its specific role: the Asp does not participate in catalysis directly but holds the His in exactly the right protonation state; the His does not break the peptide bond but activates the Ser to do so. This is functional group chemistry in its most refined biological expression.

## Why This Matters for Computational Biology

In molecular dynamics simulations and docking calculations, functional groups determine atom types, partial charges, and non-bonded interaction parameters. Protonation state assignment (which depends on pKa and local environment) must be done before any simulation. In synthetic biology, modifying functional groups changes reactivity — adding a phosphate creates a phosphorylation site, removing a thiol prevents disulfide formation. In metabolic engineering, knowing that the CoA thioester is the activated form of acyl groups directs you to enzymes that activate fatty acids before they can enter $\beta$-oxidation. Functional group chemistry is not background knowledge — it is the molecular basis of every biochemical reaction you will model.
