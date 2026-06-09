# Laboratory Fermentation

A shake flask is not a bioreactor. It is a convenient screening tool — useful for comparing dozens of strains quickly, tolerant of crude conditions, forgiving of operator variation — but it is not a controlled system. The pH swings. The oxygen transfer rate varies with flask fill volume and shaker speed in ways that are difficult to quantify. The growth curve depends on the starting OD. Results from shake flasks are semi-quantitative at best and, critically, they often do not predict bioreactor behavior. If you want to know what your strain will actually produce, and why, and how to improve it, you need a bioreactor. Laboratory fermentation is the quantitative science of controlling microbial culture conditions to maximize production of a desired compound. It bridges the gap between shake flask results (semi-quantitative, high variability) and pilot-scale bioreactors (quantitative, reproducible). Understanding fermentation parameters and their effects is essential for optimizing metabolic engineering strains before scale-up.

## Bioreactor Types

### Stirred-Tank Bioreactor (STR)

The standard laboratory bioreactor is the stirred-tank reactor: a cylindrical vessel with a mechanical stirrer (impeller), temperature control, dissolved oxygen (DO) sensor, pH sensor, and ports for air sparging, nutrient addition, and sampling.

**Volumes**: laboratory scale is typically 1–15 L working volume. Pilot scale: 50–1000 L. Industrial: 10,000–500,000 L.

**Impeller types**:
- Rushton disk turbine: standard for aerobic bacteria; high mixing intensity; good oxygen transfer
- Marine impeller: gentler mixing; used for shear-sensitive cells (mammalian cells, mycelial fungi)

### Airlift Bioreactor

Gas (air or O₂) injected at the bottom of the vessel drives circulation without mechanical agitation. Advantages: no mechanical agitation (less shear), simpler design. Disadvantages: less flexible mixing/oxygen transfer control. Used for industrial fermentation of certain organisms.

## Key Monitored and Controlled Variables

### Dissolved Oxygen (DO)

DO is the primary variable in aerobic fermentation. It is measured by a polarographic or optical DO probe (% saturation, where 100% = air-saturated medium at process temperature).

**Control**: DO is maintained at 20–30% air saturation (below which aerobic metabolism becomes limited) by increasing:
1. Agitation speed (more mixing → better O₂ transfer from bubbles to medium)
2. Air flow rate (more O₂ supplied)
3. O₂ fraction in sparge gas (supplement air with pure O₂ at high cell density)

**Oxygen transfer rate (OTR)**: the rate at which O₂ transfers from gas to liquid:
$$OTR = k_L a \cdot (C^* - C_L)$$

Where $k_L a$ is the **volumetric mass transfer coefficient** (h⁻¹), $C^*$ is the oxygen saturation concentration (~7 mg/L at 37°C), and $C_L$ is the current DO concentration. $k_L a$ depends on agitation speed, impeller design, air flow rate, and broth viscosity. It is the central parameter for oxygen supply capacity.

### pH Control

Microbial metabolism produces or consumes acid. Production of organic acids (lactate, acetate, succinate) acidifies the medium; nitrogen utilization (in ammonium-containing medium) alkalinizes it.

**pH control**: automated addition of base (NaOH, KOH, NH₄OH) or acid (H₃PO₄, H₂SO₄) via peristaltic pumps triggered by pH controller. Typical set points: pH 7.0 ± 0.05 for *E. coli*; pH 5.5–6.5 for yeast.

**pH as metabolic indicator**: a sudden pH drop indicates increased organic acid production (may signal oxygen limitation); pH increase may indicate reduced growth (cells no longer consuming NH₄⁺).

### Temperature

Optimal temperature for most laboratory *E. coli* strains: 37°C for growth, sometimes shifted to 28–30°C for protein production or at high OD to reduce metabolic rate and improve recombinant protein folding. Yeast: 30°C standard.

Temperature control: water-jacketed bioreactor with circulating water from a bath/chiller, or Peltier-element-based systems for small reactors.

### OD (Optical Density) as Biomass Proxy

Biomass is monitored online by OD₆₀₀ (absorbance at 600 nm) using in-line or at-line spectrophotometers. OD₆₀₀ is approximately linear with cell concentration up to OD ~0.5; above this, dilution is required for accurate measurements.

Conversion to dry cell weight (DCW): OD₆₀₀ × 0.34 = g/L DCW (approximate; varies by strain and condition).

## Fermentation Modes

### Batch Fermentation

All nutrients added at inoculation. Cells grow until a nutrient is exhausted (typically glucose in M9 medium). Simple but product titer is limited by the initial nutrient charge.

**Growth phases**: lag (adaptation) → exponential (μ = μmax) → stationary (nutrient limitation) → death.

**Kinetic model** (Monod):
$$\mu = \mu_{max} \frac{[S]}{K_s + [S]}$$

Where $\mu_{max}$ is the maximum specific growth rate (h⁻¹), [S] is the substrate (glucose) concentration, and $K_s$ is the substrate affinity constant. For *E. coli* on glucose: $\mu_{max} \approx 0.9$ h⁻¹; $K_s \approx 0.1$ mM.

### Fed-Batch Fermentation

The most widely used industrial mode: nutrients are added continuously during fermentation to maintain optimal concentration without accumulation to toxic levels. The feeding rate controls growth rate and cell density.

**Exponential feeding strategy**: to maintain constant $\mu$ at a set-point below $\mu_{max}$:
$$F(t) = F_0 \cdot e^{\mu_{set} \cdot t}$$

Where $F(t)$ is the glucose feed rate (g/h) at time $t$ and $F_0$ is the initial feed rate calculated from initial biomass and maintenance requirements.

**Advantages**: achieves very high cell densities (OD 100–200, equivalent to 30–60 g DCW/L); allows separation of growth and production phases by changing feed composition; prevents catabolite repression by limiting glucose to non-repressing concentrations.

**Dissolved oxygen cascade**: in fed-batch, as cell density increases, oxygen demand increases. Standard protocol: increase agitation and aeration progressively to maintain DO set-point; if maximum agitation/aeration is reached, reduce feeding rate to limit growth.

### Continuous Fermentation (Chemostat)

Steady-state operation at fixed dilution rate. Rarely used for production due to contamination risk over long times, but essential for physiological studies and ALE (section 3.4.5).

## Common Media Components

**Carbon source**: glucose (most common, well-characterized), glycerol, xylose, sucrose, methanol (Pichia).

**Nitrogen source**: ammonium sulfate (1–3 g/L in minimal medium), ammonium hydroxide (base + N source in fed-batch).

**Phosphate**: KH₂PO₄ (5–10 g/L in M9), sometimes limiting in high-density culture.

**Trace elements**: Mg²⁺, Fe²⁺, Zn²⁺, Mn²⁺, Co²⁺, Cu²⁺, Mo; required at mM–µM levels; often provided as a trace element stock solution.

**Antifoam**: silicone-based antifoam added to prevent foam formation (which reduces oxygen transfer). Added at 0.01–0.1 mL/L as needed.

## Analyzing Fermentation Data

**Mass balance**: track glucose consumed + O₂ consumed → CO₂ produced + biomass + product produced. Any carbon unaccounted for suggests byproduct accumulation or measurement error.

**Yield coefficients**:
- $Y_{x/s}$: biomass yield per substrate (g DCW/g glucose); typically 0.4–0.5 for aerobic *E. coli*
- $Y_{p/s}$: product yield per substrate (g product/g glucose)
- $Y_{p/x}$: product yield per biomass (g product/g DCW): specific productivity

**Productivity**: volumetric productivity $Q_p$ = $dP/dt$ (g/L/h) at any time point; $\overline{Q_p}$ = total product/total time = average volumetric productivity.

## Why This Matters

Fermentation science transforms a promising shake flask result into a reproducible, quantifiable production process. The parameters — kLa, pH control, feeding strategy, DO cascades — each affect product yield and titer through specific biological mechanisms. Understanding that increased OD correlates with increased oxygen demand (and that this sets a practical limit on how fast cells can grow without oxygen limitation) is essential for designing feeding strategies. Understanding that pH oscillations occur at scale (due to poor mixing and localized acid injection) explains why some strains that work perfectly in the lab bioreactor fail at pilot scale. These scale-up challenges — the subject of the next section — are rooted in the fundamental fermentation parameters established in laboratory bioreactors.
