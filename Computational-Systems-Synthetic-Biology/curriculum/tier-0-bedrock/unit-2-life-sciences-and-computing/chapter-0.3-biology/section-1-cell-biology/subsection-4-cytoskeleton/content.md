# The Cytoskeleton

Ask a materials scientist to describe a structure that is simultaneously a compression-bearing scaffold, a tension-generating contractile network, a set of railway tracks for molecular cargo transport, and a dynamic assembly that can be completely reorganized in minutes — and they will tell you that nothing like it exists in engineered materials. But cells build exactly this structure, and rebuild it continuously. The cytoskeleton is one of biology's most astonishing feats of materials engineering, and understanding it is not merely academic for computational biologists: cytoskeletal mechanics, polymerization kinetics, and motor protein dynamics are all quantitatively tractable, and models of cell mechanics and intracellular transport depend on them.

The cytoskeleton is the cell's internal scaffold, providing mechanical support, organizing intracellular space, driving cell movement and division, and serving as tracks for intracellular transport. Understanding the cytoskeleton is important for cell biology models, for understanding cell mechanics, and for the growing field of cellular mechanotransduction — how cells sense and respond to physical forces.

## Actin Filaments (Microfilaments)

**Actin filaments** (F-actin) are dynamic, polar polymers of globular actin monomers (G-actin, 42 kDa). Key properties:

- **Diameter:** ~6 nm
- **Polarity:** Filaments have a "barbed" (+) end (fast-growing) and "pointed" (–) end (slow-growing)
- **Dynamics:** Actin polymerization and depolymerization are ATP-dependent. **Treadmilling** occurs when the barbed end polymerizes while the pointed end depolymerizes at the same rate — the filament "moves" without changing length.
- **Critical concentration:** Below $c_c \approx 0.1\ \mu$M, monomers do not polymerize; above it, polymerization is spontaneous.

**Structures formed by actin:**
- **Cortical actin:** Dense meshwork beneath the plasma membrane — controls cell shape and resists deformation
- **Stress fibers:** Thick bundles of actin + myosin II — generate tension, resist mechanical stress
- **Filopodia and lamellipodia:** Thin protrusions and flat sheets at the leading edge of migrating cells, driven by actin polymerization

**Motor protein:** Myosin uses actin as tracks. **Myosin II** (non-muscle) contracts actomyosin networks; **myosin V** (processive) carries vesicular cargo.

**Regulation:** Arp2/3 complex nucleates new actin branches; cofilin severs filaments and accelerates depolymerization; profilin promotes monomer addition at barbed ends; formin (Diaphanous) nucleates and elongates unbranched filaments.

## Microtubules

**Microtubules** (MTs) are hollow cylinders assembled from $\alpha$/$\beta$-tubulin heterodimers (55 kDa each):

- **Diameter:** 25 nm (13 protofilaments arranged in a ring)
- **Dynamic instability:** MTs undergo GTP-dependent stochastic switching between growth and rapid depolymerization (**catastrophe**). Growing MTs have a "GTP cap" — a region of GTP-tubulin at the (+) end that stabilizes the structure. Loss of the GTP cap triggers catastrophe; accumulation of GTP-tubulin rescues shrinking MTs. This **dynamic instability** allows the MT network to rapidly explore intracellular space.
- **Organization:** Nucleated at the **centrosome** (MTOC — microtubule organizing center) in animal cells; (-) ends are anchored at the centrosome; (+) ends extend outward.

**Functions:**
- **Mitotic spindle:** Organizes chromosomes during cell division; kinetochore MTs attach to chromosomes and generate the force that separates chromatids
- **Intracellular transport tracks:** Motor proteins kinesin (moves toward + end, away from centrosome) and dynein (moves toward – end, toward centrosome) carry cargo along MTs
- **Cell polarity:** In neurons, axon MTs are uniformly oriented with + ends toward the axon tip; dendrite MTs are mixed orientation

**Motor proteins:** Kinesin-1 (conventional kinesin): processive, takes 8 nm steps per ATP hydrolysis, moves organelles toward the cell periphery. Dynein: retrograde transport (+ to – end, toward centrosome/nucleus).

## Intermediate Filaments

**Intermediate filaments (IFs)** are fibrous structures with 10 nm diameter, purely structural:

- **Composition:** Coiled-coil dimers assembled into ropelike filaments; highly tissue-specific
- **Keratin:** Epithelial cells (hair, skin, nails)
- **Vimentin:** Mesenchymal cells, fibroblasts
- **Desmin:** Muscle cells
- **Neurofilaments:** Axons
- **Lamins:** Line the inner nuclear membrane; form the nuclear lamina; provide nuclear mechanical integrity

IFs are **not dynamic** like actin and MTs — they do not undergo treadmilling or dynamic instability. They provide purely mechanical support. Mutations in lamins cause **laminopathies** (including progeria, Emery-Dreifuss muscular dystrophy, dilated cardiomyopathy).

## Quantitative Aspects: Polymerization Kinetics

The kinetics of actin/tubulin polymerization are well-described by the ODE:

$$\frac{d[F]}{dt} = k_+ [G] - k_-$$

where $[G]$ is monomer concentration, $k_+$ is the on-rate (M$^{-1}$s$^{-1}$), and $k_-$ is the off-rate (s$^{-1}$). The critical concentration $c_c = k_-/k_+$.

For actin at the barbed (+) end: $k_+ \approx 10\ \mu\text{M}^{-1}\text{s}^{-1}$, $k_- \approx 1\ \text{s}^{-1}$, $c_c \approx 0.1\ \mu\text{M}$.
For microtubules at the (+) end: $k_+ \approx 4\ \mu\text{M}^{-1}\text{s}^{-1}$, $k_- \approx 20\ \text{s}^{-1}$ (plus catastrophe/rescue kinetics).

The **stochastic treatment** of dynamic instability requires a Markov chain model with states for growing and shrinking MTs and rates for catastrophe and rescue — a canonical example of applying Markov chains to cell biology. It is worth pausing on why the ODE treatment is insufficient here: because catastrophe is a rare, random event (not a continuous smooth process), the deterministic ODE correctly predicts the average behavior but misses the variability between individual microtubules that is biologically important — for example, in chromosome capture by the mitotic spindle, where individual MT catastrophes and rescues govern whether a kinetochore is found and attached.

## Why This Matters for Computational Biology

The cytoskeleton affects nearly every aspect of cell physiology that one might model. Mechanical models of cell deformation require specifying cytoskeletal rheology (viscoelastic properties). Models of intracellular transport along MTs involve drift-diffusion equations. In synthetic biology, cell mechanics affect the performance of mechanosensing biosensors. The spatial organization of mRNA within cells (localization to specific regions via cytoskeletal attachment) affects protein distribution and function — important for models that include spatial heterogeneity. Force balance models of cell division require understanding the forces generated by the mitotic spindle (motor proteins on MTs).
