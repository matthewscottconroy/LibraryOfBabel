# Coenzymes and Metabolic Currency

Consider the problem that metabolism must solve. A cell needs to take the energy released when sugar is oxidized and use it to drive the synthesis of proteins, nucleotides, and lipids. But you cannot directly couple sugar oxidation to peptide bond formation — the two reactions involve completely different chemistries, different molecules, different timescales. You need a currency: a universal intermediate that can accept energy from wherever it is released and deliver it to wherever it is needed.

ATP is that currency. But ATP is only one member of a larger set of coenzymes that solve the more general problem of metabolic coupling: how to route electrons, acyl groups, methyl groups, and one-carbon units from where they are produced to where they are consumed. These small molecules are not catalysts in the usual sense — they are consumed and regenerated, carrying chemical groups from one enzyme to another. Without them, the metabolic network would be a disconnected set of isolated reactions. With them, it becomes an integrated, flexible, responsive system.

Metabolism is not possible without coenzymes — small organic molecules (and metal ions) that carry chemical groups between enzymes, transferring energy, electrons, and reactive species. The handful of coenzymes described here participate in hundreds of reactions throughout metabolism. Understanding their chemistry is essential for understanding how cells harvest energy, build macromolecules, and maintain redox balance.

## ATP: The Universal Energy Currency

**Adenosine triphosphate (ATP)** is the primary energy currency of the cell. It consists of adenine, ribose, and three phosphate groups connected by phosphoanhydride bonds.

**Hydrolysis reactions:**
- $\text{ATP} \to \text{ADP} + P_i$: $\Delta G^{\circ'} = -30.5$ kJ/mol
- $\text{ATP} \to \text{AMP} + PP_i$: $\Delta G^{\circ'} = -30.5$ kJ/mol (followed by $PP_i \to 2P_i$, $\Delta G^{\circ'} = -29$ kJ/mol, making total $\approx -60$ kJ/mol — drives reactions to completion)

**Why is ATP "high energy"?** Three factors:
1. **Charge repulsion:** The three negatively charged phosphate groups repel each other; hydrolysis relieves this strain
2. **Resonance stabilization:** The hydrolysis products (ADP + P$_i$) are more resonance-stabilized than ATP
3. **Solvation:** The hydrolysis products are better solvated by water

Under cellular conditions ($[\text{ATP}]/[\text{ADP}] \approx 10$, $[P_i] \approx 1-5$ mM), the actual $\Delta G$ is $-50$ to $-60$ kJ/mol — this drives the **phosphoryl transfer potential** of ATP, making it thermodynamically capable of driving many biosynthetic reactions.

**ATP production:**
- Glycolysis: 2 ATP (net, substrate-level phosphorylation)
- TCA cycle: 2 GTP (substrate-level)
- Oxidative phosphorylation: ~26-28 ATP (chemiosmotic coupling via the ATP synthase complex, driven by the proton gradient across the inner mitochondrial membrane)
- Total from glucose: ~30 ATP

## NAD$^+$/NADH: The Primary Electron Carrier in Catabolism

**Nicotinamide adenine dinucleotide (NAD$^+$)** carries high-energy electrons as hydride (H$^-$):

$$\text{NAD}^+ + \text{H}^- \to \text{NADH}$$

- $E^{\circ'} = -0.32$ V (reduction potential) — NAD$^+$/NADH can donate electrons to oxygen ($E^{\circ'} = +0.82$ V), releasing $\Delta G = -n F \Delta E = -2 \times 96485 \times 1.14 \approx -220$ kJ/mol
- This drives ATP synthesis via the electron transport chain

**Catabolic reactions producing NADH:**
- Glyceraldehyde-3-phosphate dehydrogenase (glycolysis): 2 NADH
- Pyruvate dehydrogenase: 2 NADH
- Isocitrate dehydrogenase, $\alpha$-ketoglutarate dehydrogenase, malate dehydrogenase (TCA): 6 NADH per glucose

**NAD$^+$ regeneration in anaerobic conditions:** Lactate fermentation (lactic acid bacteria, muscle under heavy exercise) or ethanol fermentation (yeast) regenerates NAD$^+$ from NADH — allowing glycolysis to continue when the ETC is inactive.

## NADP$^+$/NADPH: The Anabolic Reductant

**Nicotinamide adenine dinucleotide phosphate (NADP$^+$)** is structurally identical to NAD$^+$ except for a 2'-phosphate on the adenosine. This small difference is recognized by enzymes that specifically use NADPH rather than NADH — maintaining separate oxidative (NAD$^+$) and reductive (NADP$^+$) pools.

**NADPH is produced by:**
- Pentose phosphate pathway (G6P dehydrogenase and 6-PGL dehydrogenase): major source
- Isocitrate dehydrogenase (mitochondrial NADP$^+$-specific)
- Malic enzyme
- Folate cycle (methylenetetrahydrofolate reductase)

**NADPH is consumed by:**
- Fatty acid synthesis (acyl carrier protein reductase in FAS complex)
- Cholesterol synthesis (HMG-CoA reductase — the target of statins)
- Glutathione reductase: maintains reduced glutathione for antioxidant defense
- NADPH oxidase (immune cells): produces reactive oxygen species for pathogen killing

**The NAD$^+$/NADH and NADP$^+$/NADPH ratios** are key metabolic signals. High NADH/NAD$^+$ inhibits the TCA cycle (feedback inhibition); high NADPH/NADP$^+$ inhibits the pentose phosphate pathway. These ratios connect metabolic state to signaling (via SIRT1 deacetylase, which requires NAD$^+$) and transcriptional regulation.

It is worth appreciating how elegant it is that cells maintain two separate pools of almost identical molecules. NADH and NADPH differ by a single phosphate group — an almost trivial structural change. Yet this difference is enough for enzymes to distinguish between them with high selectivity. By keeping the NADH/NAD$^+$ ratio low (favoring oxidation) and the NADPH/NADP$^+$ ratio high (favoring reduction), the cell simultaneously harvests energy from catabolism and powers biosynthesis without the two opposing processes interfering with each other.

## FAD/FADH$_2$: Flavin Coenzymes

**Flavin adenine dinucleotide (FAD)** is tightly bound to its enzyme partner (a prosthetic group, not freely diffusible like NAD$^+$). FAD accepts two electrons:

$$\text{FAD} + 2\text{H} \to \text{FADH}_2 \quad E^{\circ'} = -0.18\ \text{V}$$

FAD is less negative than NAD$^+$/NADH ($-0.32$ V), meaning FADH$_2$ donates electrons to the ETC at a different point than NADH (Complex II vs. Complex I) and generates fewer ATP (~1.5 ATP vs. ~2.5 ATP per NADH via the chemiosmotic P/O ratio).

**Reactions using FAD:**
- Succinate dehydrogenase (TCA cycle / ETC Complex II): succinate → fumarate
- Acyl-CoA dehydrogenase ($\beta$-oxidation): oxidizes acyl-CoA to enoyl-CoA

## Coenzyme A and Acyl Groups

**Coenzyme A (CoA)** carries acyl groups via a high-energy thioester bond. The thioester bond (R-CO-S-CoA) has $\Delta G^{\circ'} \approx -31$ kJ/mol for hydrolysis — similar to ATP, making acyl-CoA a reactive, "activated" form of the acyl group.

**Key acyl-CoA molecules:**
- **Acetyl-CoA:** The central metabolic hub — produced from pyruvate (PDH), fatty acid $\beta$-oxidation, and amino acid catabolism; feeds into TCA cycle, cholesterol synthesis, and acetylation reactions
- **Malonyl-CoA:** The activated carbon donor for fatty acid biosynthesis; its accumulation signals anabolic state
- **Succinyl-CoA:** TCA intermediate; also substrate for heme synthesis and amino acid metabolism

## Other Key Coenzymes

**S-Adenosylmethionine (SAM):** The universal methyl donor. Methyl transfers from SAM are exergonic; this drives methylation of DNA (DNA methyltransferases), histones (lysine and arginine methyltransferases), RNA (m6A modification), and small molecules. SAM is regenerated by the methionine cycle.

**Tetrahydrofolate (THF):** One-carbon carrier. Carries one-carbon groups at different oxidation states (methyl, methylene, methenyl, formyl). Essential for nucleotide biosynthesis (thymidine, purines) and the methionine cycle. Antifolate drugs (methotrexate, trimethoprim) inhibit DHFR, which regenerates THF — the basis of some cancer chemotherapy and antibiotic treatments.

**Pyridoxal phosphate (PLP):** The active form of vitamin B6. A cofactor for amino acid metabolism — transamination, decarboxylation, racemization. The Schiff base between PLP and the substrate amino group is the key catalytic intermediate.

## Why This Matters for Computational Biology

Coenzyme stoichiometries are the currency of metabolic flux models. In flux balance analysis, ATP, NADH, NADPH, and CoA balances are explicit constraints. The number of NADH molecules produced per glucose determines the theoretical maximum ATP yield; deviation from this maximum reveals inefficiency or alternative routes. In metabolic engineering, controlling NADPH/NADP$^+$ balance is often the key challenge in redirecting carbon flux toward reduced products (alcohols, fatty acids). In synthetic biology, designing orthogonal metabolic pathways requires understanding which coenzyme pools they tap into and whether they will compete with essential cellular processes.
