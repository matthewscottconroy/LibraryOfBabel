# Organic Reaction Mechanisms

If functional groups are the vocabulary of biochemistry, then reaction mechanisms are the grammar. Knowing that an enzyme active site contains a serine and a histidine tells you the nouns; knowing the mechanism — the order of proton transfers, the formation and collapse of the tetrahedral intermediate, the departure of the leaving group — tells you the sentence.

Understanding organic reaction mechanisms — the step-by-step electron movements that convert reactants to products — is essential for understanding enzyme catalysis at a deep level. Every enzymatic reaction uses one of a small number of fundamental organic mechanisms. Recognizing these mechanisms allows you to understand how inhibitors work, why certain substrates are preferred, and how new catalytic activities can be engineered.

There are really only a handful of fundamental reaction types in organic chemistry: nucleophilic substitution, elimination, addition to carbonyls, and oxidation-reduction. Nature has elaborated these into hundreds of distinct enzyme families, but the underlying chemistry is always recognizable. Once you've internalized a few key mechanisms, every new enzyme you encounter will have a familiar ring.

## Nucleophilic Substitution: SN1 and SN2

In **nucleophilic substitution**, a nucleophile (electron-rich species) displaces a leaving group from an electrophilic carbon:

**SN2 (bimolecular):** The nucleophile attacks directly from the back face (opposite the leaving group) while the bond to the leaving group breaks — both events occur simultaneously in a concerted step. Rate $= k[\text{Nu}^-][\text{substrate}]$.

- **Inversion of stereochemistry** (Walden inversion) — the product has opposite configuration at the reaction center
- Favored by: strong nucleophile, primary carbon, polar aprotic solvent
- Biological example: methyltransferases using S-adenosylmethionine (SAM): the methyl group is transferred in an SN2 reaction, with the methyl acceptor nucleophile attacking and the adenosylthioether leaving.

**SN1 (unimolecular):** The leaving group departs first, forming a carbocation intermediate; then the nucleophile attacks. Rate $= k[\text{substrate}]$ (independent of nucleophile concentration).

- **Racemization** possible (carbocation is planar)
- Favored by: tertiary carbon, stable carbocation, polar protic solvent
- Less common in enzyme catalysis (carbocations are destabilized in aqueous enzyme active sites)

The SAM methyltransferase example is one to remember. SAM is the universal methyl donor in biology — it methylates DNA bases, histone lysines, RNA, and small molecules. Every one of these reactions is an SN2 attack, meaning the product always has inverted configuration at the transferred methyl group. This has been exploited experimentally: by feeding cells isotopically labeled methyl groups and measuring whether the product is inverted, researchers can confirm an SN2 mechanism without observing the reaction directly.

## Elimination Reactions (E1, E2)

Elimination removes two groups from adjacent carbons to form a double bond:

**E2:** Concerted; base abstracts a proton from carbon adjacent to the leaving group while the leaving group departs. Required: anti-periplanar arrangement of H and leaving group.

**E1:** Stepwise via carbocation intermediate.

**Biological example:** Fumarase in the TCA cycle catalyzes the stereospecific anti-addition of water to fumarate (or its reverse, anti-elimination). The *trans* stereochemistry of fumarate is preserved in the product malate. Aconitase catalyzes dehydration/rehydration with strict stereochemical control.

## Nucleophilic Addition to Carbonyls

The carbonyl group (C=O) is electrophilic. Nucleophiles add across the C=O double bond:

$$\text{Nu}^- + \text{C=O} \to \text{Nu-C-O}^-$$

The mechanism forms a **tetrahedral intermediate**, which is the cornerstone of many enzymatic mechanisms: serine proteases, cysteine proteases, lipases, and transglutaminases all form tetrahedral intermediates during catalysis.

**Hemiacetal and hemiketal formation:** Attack of an alcohol (-OH) on an aldehyde or ketone forms a hemiacetal. The ring closure of glucose (open chain aldehyde) to glucopyranose (ring form) is an intramolecular hemiacetal formation — the source of the anomeric carbon.

**Acyl transfer:** A nucleophile attacks an acyl group (R-CO-), the tetrahedral intermediate collapses, and the leaving group departs. This is the mechanism of:
- **Ester hydrolysis:** Water attacks the ester carbonyl
- **Amide hydrolysis (peptide bond cleavage):** Water attacks the amide carbonyl — requires enzyme catalysis due to the stability of the amide bond
- **Thioester reactions:** Thioester hydrolysis and acyl transfer (fatty acid synthesis, CoA reactions)

The tetrahedral intermediate is the pivot point around which all these reactions turn. It is not stable — it forms and collapses within microseconds — but its existence can be confirmed experimentally by isotope effects and kinetic analyses. Enzyme active sites are specifically shaped to stabilize this intermediate through the so-called oxyanion hole, a cluster of backbone amide N-H groups that hydrogen-bond to the developing negative charge on the oxygen.

## Esterification and Hydrolysis

Esterification (acid + alcohol → ester + water) is thermodynamically favorable but kinetically slow without catalysis. The reverse (ester hydrolysis) is also slow without enzymes.

**In biological systems:**
- **Lipases** catalyze ester hydrolysis and synthesis of triglycerides
- **Phosphodiesterases** cleave the phosphodiester backbone of DNA and RNA, and cyclic nucleotides (cAMP → AMP by PDE — critical signaling control point)
- **Acetylcholinesterase** hydrolyzes the neurotransmitter acetylcholine (an ester) — targeted by nerve agents and many insecticides that inhibit it irreversibly

## Oxidation and Reduction

In organic chemistry, oxidation of carbon means gaining bonds to oxygen or losing bonds to hydrogen; reduction means the reverse. In biological redox chemistry:

- **Oxidation** removes electrons/hydrogens; the electron carrier NAD$^+$ is reduced to NADH
- **Reduction** adds electrons/hydrogens; NADH is oxidized back to NAD$^+$

**Redox reactions in metabolism:**
- **Glycolysis:** Glyceraldehyde-3-phosphate dehydrogenase oxidizes G3P (aldehyde → carboxylate), reducing NAD$^+$ to NADH
- **TCA cycle:** Isocitrate dehydrogenase, $\alpha$-ketoglutarate dehydrogenase, and malate dehydrogenase all oxidize organic acids, producing NADH
- **Fatty acid oxidation ($\beta$-oxidation):** Each cycle oxidizes the $\beta$-carbon, producing FADH$_2$ and NADH

The key mechanistic principle: **alcohol → aldehyde/ketone → carboxylic acid** represents two sequential two-electron oxidations.

## Why This Matters for Computational Biology

Reaction mechanisms are the molecular grammar underlying enzyme function. When you model an enzymatic reaction with a rate law (Michaelis-Menten, Hill, etc.), you are abstracting away the mechanism — but knowing the mechanism tells you:
1. **What inhibitors look like:** Transition state analogs as enzyme inhibitors (e.g., phosphonate analogs of phosphate esters as phosphatase inhibitors)
2. **Why some reactions are irreversible:** Thioester formation in fatty acid synthesis has a specific directionality enforced by the mechanism
3. **How to engineer new activities:** Understanding the nucleophile-electrophile complementarity allows rational design of new enzyme activities
4. **What goes wrong in disease:** Many drugs and toxins target specific mechanistic steps (aspirin acetylates the active site serine of COX; penicillin acylates the active site serine of transpeptidase)
