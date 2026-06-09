# Compartmentalization Strategies

The eukaryotic cell discovered something that engineers are now rediscovering: the most effective way to manage incompatible chemistries is to separate them in space. Mitochondria run oxidative phosphorylation behind a membrane that maintains a proton gradient. Peroxisomes carry out oxidative reactions that would destroy cytoplasmic components if left loose. The nucleus keeps the genome physically separated from the busy metabolism of the cytoplasm. What the cell does with lipid membranes, metabolic engineers can now do with protein shells, organelle retargeting, and synthetic encapsulation. Compartmentalization concentrates pathway enzymes and their substrates within a bounded volume, increasing local concentrations and reducing unproductive interactions with the bulk cytoplasm. Unlike scaffolds that co-localize enzymes through direct interaction, compartmentalization uses physical enclosure — either protein shells or lipid membranes — to create distinct chemical microenvironments.

## Bacterial Microcompartments (BMCs)

Bacterial microcompartments are proteinaceous organelles: polyhedral protein shells that encapsulate enzymes and intermediates within a defined interior. They are fundamentally different from lipid-bounded organelles — the shell is entirely protein, self-assembled from thousands of copies of hexameric and pentameric shell proteins.

### The Carboxysome: Natural Prototype

The **carboxysome** is the most studied BMC, found in cyanobacteria and some autotrophic bacteria:
- Shell proteins: CsoSI (hexamers forming flat shell facets), CsoSIV (pentamers at vertices)
- Interior enzymes: RuBisCO (ribulose-1,5-bisphosphate carboxylase/oxygenase) + carbonic anhydrase
- Function: CO₂ is concentrated inside the carboxysome to high local concentration, enabling RuBisCO to operate efficiently despite its low CO₂ affinity

**Key properties**:
- Pore size in shell hexamers: ~4–7 Å — small enough to retain proteins but allow passage of small molecules
- Shell is **selectively permeable**: CO₂ and O₂ pass through; larger molecules are excluded
- Size: 80–150 nm diameter, enclosing ~200–300 RuBisCO molecules

### Propanediol Utilization (Pdu) BMC

The **Pdu BMC** in *Salmonella* encapsulates the enzymes for vitamin B12-dependent propanediol degradation:
- Contains: PduCDE (diol dehydratase), PduGH (reactivase), PduP (aldehyde dehydrogenase)
- Purpose: propanaldehyde (a toxic intermediate) is retained inside the BMC rather than diffusing to damage DNA and cellular proteins
- Shell proteins: PduA, PduB, PduJ, PduK, PduN (homologous to carboxysome shell proteins)

This is precisely the situation relevant to metabolic engineering: a toxic intermediate is sequestered inside the compartment, reducing cytoplasmic exposure.

### Engineering BMCs for Heterologous Pathways

The strategy: express the shell protein genes from the carboxysome or Pdu system in the host, then target heterologous pathway enzymes to the compartment interior using **encapsulation peptides** (short N-terminal peptide sequences found in native BMC-targeted proteins that are both necessary and sufficient for encapsulation).

**Published examples**:
- Cai et al. (2008): targeted heterologous pyruvate decarboxylase and alcohol dehydrogenase to a recombinant Pdu-like BMC in *E. coli*, sequestering acetaldehyde intermediate → 5-fold reduced acetaldehyde toxicity in the cell
- Lawrence et al. (2014): demonstrated that encapsulation peptides from Pdu proteins (18 aa N-terminal) are sufficient to target any fused protein into recombinant BMC shells

**Challenges**:
- Shell proteins must self-assemble correctly in the heterologous host: requires careful promoter balancing for correct stoichiometry of hexamers and pentamers
- Encapsulated enzymes must fold correctly inside the restricted shell volume
- Shell permeability must allow necessary substrates and cofactors to enter while retaining intermediates

## Organelle Engineering in Yeast

Eukaryotic cells have compartmentalized organelles with distinct chemical environments. Engineering pathway enzymes into specific organelles can provide access to unique cofactors, co-factors, and pH environments.

### Peroxisome Engineering

The **peroxisome** is a lipid-bounded organelle in eukaryotes containing oxidative enzymes. Key properties for metabolic engineering:
- High abundance of acetyl-CoA (from fatty acid β-oxidation)
- Reducing environment distinct from cytoplasm
- Contains its own pool of NADH/NADPH
- Readily accessible in yeast for heterologous enzyme targeting using the PTS1 (peroxisome targeting sequence 1) tripeptide: SKL or variants

**Application**: terpenoid biosynthesis in the peroxisome. Acetyl-CoA supply for the mevalonate pathway in the peroxisome is high (from β-oxidation), potentially bypassing cytoplasmic acetyl-CoA limitations. Several groups have routed mevalonate pathway enzymes to the peroxisome using SKL signal peptides and reported 2–3-fold improvement in sesquiterpene titers.

**Example**: Farhi et al. (2011) targeted complete lycopene biosynthesis pathway to yeast peroxisomes, achieving 5-fold higher lycopene titer compared to cytoplasmic expression by exploiting peroxisomal NADPH and acetyl-CoA pools.

### Mitochondria Engineering

Mitochondria have:
- Active TCA cycle providing abundant NADH
- Acetyl-CoA produced from pyruvate by mitochondrial PDC
- Distinct membrane potential and pH gradient

**Application**: route isoprenoid pathway to mitochondria where acetyl-CoA and NADPH are more abundant. Mitochondrial targeting sequences (N-terminal presequences) direct proteins to the mitochondrial matrix.

**Challenge**: mitochondrial import requires protein unfolding then refolding, which may inactivate some enzymes. Cofactors that cannot cross the inner membrane (CoA, NADPH) may limit reactions even in the matrix.

### Vacuole and Lipid Droplets

**Lipid droplets**: hydrophobic core enclosed by phospholipid monolayer. Ideal for sequestering hydrophobic terpenoids (β-carotene, lycopene) that otherwise intercalate into cell membranes and cause toxicity. Some reports suggest that targeting carotenoid biosynthesis enzymes to lipid droplet membranes improves stability and reduces toxicity.

**Vacuole**: acidic compartment (pH 4.5–5.5 in yeast) containing proteases and hydrolases. Not typically used for biosynthesis (the proteolytic environment degrades most enzymes) but useful for storing acid-stable products.

## Synthetic Lipid Vesicle Compartments

For cell-free metabolic engineering (relevant to Chapter 3.6), pathway enzymes can be encapsulated inside lipid vesicles (liposomes or polymer vesicles — polymersomes) to create an artificial organelle:

1. Reconstitute pathway enzymes in solution
2. Extrude with lipid film using freeze-thaw + extrusion → liposome encapsulation of enzymes
3. Verify encapsulation by size exclusion + activity assay

**Application**: multi-step cell-free reactions where intermediates must be concentrated. Demonstrated for ATP-regenerating systems and multi-enzyme cascades in synthetic biology research contexts.

## Why This Matters

Compartmentalization represents the most sophisticated level of spatial organization for metabolic engineering. By creating distinct chemical environments within the cell — or in cell-free systems — compartmentalization can solve problems that no amount of expression optimization can address: toxic intermediates that cannot be eliminated, cofactor pools that cannot be accessed from the bulk cytoplasm, and competing reactions that cannot be genetically abolished. The engineering of BMCs and peroxisomes is still in early stages, but the potential is significant: a truly contained metabolic module that accepts substrates from the cytoplasm and exports products, while sequestering all toxic or unstable intermediates within its interior, would represent the ultimate compartmentalized factory module — analogous to how natural metabolism compartmentalizes the most reactive biochemistry inside protein shells or membrane-bounded organelles that have been refined by billions of years of evolution.
