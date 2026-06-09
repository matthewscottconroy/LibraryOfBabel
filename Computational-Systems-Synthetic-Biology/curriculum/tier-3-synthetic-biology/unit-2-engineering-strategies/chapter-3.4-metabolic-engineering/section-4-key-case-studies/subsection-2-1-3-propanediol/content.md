# Case Study: 1,3-Propanediol

Every carpet made of Sorona fiber — that silky, resilient material used in everything from athletic wear to home furnishings — is produced in part by *E. coli*. Not *E. coli* that evolved to do this; *E. coli* that was engineered over nearly two decades to convert glucose into 1,3-propanediol, the key monomer in Sorona's polymer backbone. 1,3-Propanediol (1,3-PDO) holds a unique place in metabolic engineering history as the first product of a metabolically engineered organism to reach commercial production. The DuPont/Genencor project demonstrated that metabolic engineering could compete with petroleum-derived chemicals on cost, paving the way for an entire industry of bio-based chemicals.

## The Target and Its Value

**1,3-Propanediol (1,3-PDO)**: a three-carbon diol (HOCH₂CH₂CH₂OH) used as a monomer for polytrimethylene terephthalate (PTT) fiber — sold by DuPont as Sorona. PTT has superior properties to polyethylene terephthalate (PET) for carpet and textile applications: better stretch recovery, easier dyeing, and softer texture.

**Petroleum route**: 1,3-PDO is synthesized from acrolein (from propylene) by hydration and hydrogenation, or from ethylene oxide by hydroformylation — both processes requiring high temperatures, pressures, and petroleum feedstocks.

**Biological route target**: produce 1,3-PDO from glucose fermentation at competitive cost ($<1/kg), enabling sustainable fiber production.

## The Natural Pathways

1,3-PDO is produced naturally in two biological contexts, both requiring glycerol as intermediate:

**Klebsiella pneumoniae (anaerobic)**: glucose → glycerol → 3-hydroxypropionaldehyde (3-HPA) → 1,3-PDO
- Glycerol dehydratase (DhaB): glycerol → 3-HPA + H₂O (requires vitamin B12 cofactor)
- 1,3-PDO oxidoreductase (DhaT): 3-HPA + NADH → 1,3-PDO + NAD⁺

**Problem with Klebsiella**: Klebsiella is a pathogen (BSL-2), difficult to scale; also produces byproducts (2,3-butanediol, lactate). Not suitable for commercial food-grade or direct fermentation product at scale.

## DuPont's Engineering Strategy in *E. coli*

The key challenge: *E. coli* does not naturally produce 1,3-PDO, and the natural route requires glycerol as intermediate. DuPont's strategy combined two heterologous pathways:

**Module 1 (glycerol from glucose)**: *E. coli* produces glycerol only as a minor byproduct. Introduce:
- gps1 from *S. cerevisiae* (glycerol-3-phosphate synthase): glucose-6-phosphate → glycerol-3-phosphate
- gpp2 from *S. cerevisiae* (glycerol-3-phosphate phosphatase): glycerol-3-phosphate → glycerol

**Module 2 (glycerol to 1,3-PDO)**: from *Klebsiella pneumoniae*:
- dhaB1, dhaB2, dhaB3 (glycerol dehydratase + reactivase): glycerol → 3-HPA
- dhaT (1,3-PDO oxidoreductase): 3-HPA → 1,3-PDO

### The Vitamin B12 Problem

Glycerol dehydratase requires vitamin B12 (cobalamin) as cofactor. *E. coli* cannot synthesize vitamin B12 and must import it. Adding commercial B12 to fermentation medium is possible but expensive.

DuPont's solution: engineer cobalamin biosynthesis into *E. coli* by introducing 30 genes from *Salmonella enterica* that encode the complete B12 biosynthetic pathway. This was one of the largest metabolic engineering projects attempted at the time (early 2000s) and remains a demonstration of what large-scale metabolic retrofitting can accomplish.

### Pathway Stoichiometry

From glucose (C₆H₁₂O₆) to 1,3-PDO (C₃H₈O₂):

$$\text{Glucose} \rightarrow 2 \text{ Pyruvate} \rightarrow 1 \text{ Glycerol} \rightarrow 1 \text{ 1,3-PDO}$$

Simplified: 1 mol glucose → 1 mol 1,3-PDO (theoretical yield = 74/180 = 0.41 g/g)

Achieved yield: ~0.35 g/g (85% of theoretical), with titer >130 g/L and productivity >3.5 g/L/h in optimized fed-batch fermentation.

## Regulatory and Commercial Path

The 1,3-PDO fermentation process received FDA GRAS (Generally Recognized as Safe) status and regulatory clearance by 2000. Commercial production began in 2006, when DuPont's Tennessee plant began producing Sorona fiber from bio-based 1,3-PDO.

**Commercial impact**:
- Annual production: ~100,000 metric tons by 2010
- Price competitive with petroleum-derived 1,3-PDO (<$1/kg)
- 37% reduction in energy use vs. petroleum process
- Lower greenhouse gas emissions (bio-based carbon)

## Lessons for Metabolic Engineering

**1. Multi-organism pathway combination**: the DuPont strain combined pathways from *E. coli* (host), *S. cerevisiae* (glycerol module), and *K. pneumoniae* (1,3-PDO module). This modular approach from diverse organisms is standard in modern metabolic engineering.

**2. Cofactor engineering at scale**: solving the B12 problem by introducing 30 biosynthetic genes illustrates that even difficult cofactor limitations can be addressed by extensive metabolic engineering — if the commercial value justifies the effort.

**3. Scale-up timeline exceeds academic expectations**: from initial proof-of-concept (~1988) to commercial production (2006) was ~18 years. This timeline includes metabolic engineering, fermentation optimization, regulatory approval, and process engineering. For commodity chemicals, 10–20 year development timelines are the norm, not the exception.

**4. The full process must be engineered**: metabolic engineering of the producing organism is only one component. Fermentation process design (fed-batch optimization, pH, aeration, temperature profiles), downstream processing (distillation, purification), and process economics all must be solved before a product can compete commercially.

## Why This Matters

The 1,3-PDO case study established that metabolic engineering is not just an academic exercise — it is a commercially viable technology capable of displacing petroleum chemistry for specialty chemicals. It demonstrated the feasibility of multi-organism pathway assembly, large-scale cofactor engineering, and the economic viability of bio-based chemicals. Every subsequent bio-based chemical project (succinic acid, adipic acid, isobutanol, farnesene) built directly on the template established by DuPont's 1,3-PDO program. The project also set the realistic expectation that commercializing a new bio-based chemical is a decade-long endeavor, not a two-year grant cycle — a sobering and important lesson for the field.
