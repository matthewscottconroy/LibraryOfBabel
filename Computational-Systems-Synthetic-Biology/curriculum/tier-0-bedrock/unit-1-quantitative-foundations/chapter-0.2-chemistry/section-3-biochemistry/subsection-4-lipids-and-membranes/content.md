# Lipids and Membranes

Drop a handful of phospholipids into water and walk away. Come back in an hour, and those amphipathic molecules will have spontaneously organized themselves into bilayer vesicles — sealed, hollow spheres with a hydrophobic interior sandwiched between two hydrophilic surfaces. No assembly instructions, no cellular machinery, no template. Just the thermodynamics of the hydrophobic effect, operating on molecules with the right geometry.

This self-assembly is one of the most remarkable facts in biology, and it is what makes cells possible. Without a membrane, there is no inside and outside; without that distinction, there is no concentration gradient, no electrochemical potential, no possibility of the organized chemistry we call life. Biological membranes are dynamic, self-organizing barriers that define cellular compartments and serve as platforms for signaling, transport, and energy transduction. The physical chemistry of lipid bilayers — how they form, how fluid they are, how they segregate into domains — is the basis for understanding membrane protein function, drug delivery, and synthetic membrane design.

## Lipid Classes

**Fatty acids** are the building blocks of most lipids: a carboxylic acid head group attached to a long hydrocarbon chain.
- **Saturated:** All C-C single bonds; fully extended; pack tightly (high melting point). Palmitic acid (16:0), stearic acid (18:0).
- **Monounsaturated:** One C=C double bond introduces a kink; disrupts packing (lower melting point). Oleic acid (18:1 $\Delta$9, *cis*).
- **Polyunsaturated:** Multiple double bonds; highly fluid. Linoleic (18:2), arachidonic (20:4), EPA (20:5), DHA (22:6).

**Glycerophospholipids** are the primary components of cellular membranes:
- **Backbone:** Glycerol esterified at sn-1 and sn-2 to fatty acids, and at sn-3 to a phosphate head group
- **Head groups:** Phosphatidylcholine (PC), phosphatidylethanolamine (PE), phosphatidylserine (PS, negative charge), phosphatidylinositol (PI, precursor to IP3/PIP2/PIP3 signaling lipids)
- The sn-2 position typically carries an unsaturated fatty acid; the sn-1 position a saturated one

**Sphingolipids:**
- Sphingosine backbone instead of glycerol; amide-linked fatty acid at the amino group
- **Sphingomyelin:** Phosphocholine head group; enriched in myelin sheaths and lipid rafts
- **Glycosphingolipids:** Sugar head groups; crucial for cell-cell recognition; ABO blood group antigens are glycolipids

**Sterols:**
- **Cholesterol** in animal membranes: modulates fluidity — at physiological temperatures, cholesterol inserts between phospholipid acyl chains, increasing rigidity of fluid membranes but preventing the gel phase
- **Ergosterol** in fungi (target of antifungal drugs like amphotericin B, azoles)
- **Hopanoids** in bacteria (analogous function to cholesterol)

## Membrane Biophysics

**Self-assembly:** Phospholipids are amphipathic — they have hydrophilic head groups and hydrophobic tails. In water, they spontaneously form bilayers (closed vesicles/liposomes, flat sheets) driven by the hydrophobic effect. The critical micelle concentration (CMC) is the concentration above which bilayer assembly becomes favored over monomers.

**The fluid mosaic model** (Singer and Nicolson, 1972): The membrane is a two-dimensional fluid of lipids with embedded proteins. Proteins can diffuse laterally (unless anchored to the cytoskeleton), rotate about the membrane normal, but cannot flip transversely (flip-flop requires enzyme activity by flippases).

**Membrane fluidity** depends on:
- **Temperature:** Below the gel-liquid crystalline transition temperature $T_c$, lipids are in a gel phase (rigid); above $T_c$, they are fluid. $T_c$ increases with chain length and decreases with unsaturation.
- **Fatty acid unsaturation:** cis double bonds create kinks that disrupt packing and lower $T_c$.
- **Cholesterol:** Acts as a "fluidity buffer." Above $T_c$: cholesterol restricts lateral movement (increases viscosity). Below $T_c$: cholesterol prevents gel formation (decreases $T_c$).

**Quantitative measure of fluidity:** The lateral diffusion coefficient $D_{\text{lat}} \approx 1 \mu m^2/s$ in fluid membranes. A protein or lipid can diffuse across a $10\ \mu m$ cell in $10^2/D_{\text{lat}} \approx 100$ s. Anchored proteins (e.g., those linked to the actin cortex) are essentially immobile: measured by FRAP (fluorescence recovery after photobleaching).

**Lipid rafts:** Ordered microdomains (liquid-ordered phase, $L_o$) enriched in sphingomyelin, cholesterol, and GPI-anchored proteins. These microdomains have higher cholesterol/sphingomyelin density, are more rigid, and are $\sim 10-200$ nm in diameter. They may compartmentalize signaling receptors (Ras, Src kinases), concentrating components of the same pathway to increase signaling efficiency. Their existence and biological significance remain subjects of active research.

## Membrane Potential and Transport

**The Nernst equation** gives the equilibrium potential for an ion with concentration gradient across a membrane:

$$E_{\text{ion}} = \frac{RT}{zF} \ln \frac{[\text{ion}]_{\text{out}}}{[\text{ion}]_{\text{in}}}$$

where $z$ is the charge, $F = 96485$ C/mol is the Faraday constant, $T$ is temperature, and $R$ is the gas constant.

**Example (K$^+$ at 37°C, $[\text{K}^+]_{\text{in}} = 140$ mM, $[\text{K}^+]_{\text{out}} = 5$ mM):**
$$E_{K^+} = \frac{8.314 \times 310}{1 \times 96485} \ln\frac{5}{140} = -88\ \text{mV}$$

The resting membrane potential of most cells (–70 to –90 mV) is close to the K$^+$ equilibrium potential because the resting membrane is selectively permeable to K$^+$.

**Transport:**
- **Simple diffusion:** Small nonpolar molecules (O$_2$, CO$_2$, N$_2$, steroid hormones) cross freely
- **Facilitated diffusion:** Polar/charged molecules use protein channels (ion channels: selective, fast, passive) or transporters (uniporters, symporters, antiporters: slower, saturable)
- **Active transport:** Against electrochemical gradient, requires energy. Primary: ATPases (Na$^+$/K$^+$-ATPase, Ca$^{2+}$-ATPase). Secondary: coupled to ion gradient (Na$^+$-glucose symporter, Na$^+$/Ca$^{2+}$ exchanger)

## Why This Matters for Computational Biology

Membrane modeling is relevant in several contexts. Pharmacokinetic models of drug absorption and distribution depend on lipid partition coefficients (logP) and membrane permeability. In synthetic biology, membrane composition affects the performance of membrane-localized biosensors, channel proteins, and lipid-binding transcription factors. Molecular dynamics simulations of membrane proteins require accurate lipid force fields and the correct bilayer composition. The Nernst equation and cable equation underlie computational neuroscience models (Hodgkin-Huxley). Liposome-based drug delivery systems in nanomedicine are designed based on membrane biophysics principles. Understanding membranes is understanding the spatial organization of the cell.
