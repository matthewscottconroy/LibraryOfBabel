# Bacterial Growth

A single *E. coli* cell in rich medium at 37°C will divide in 20 minutes. If nothing limits its growth, its descendants would fill the observable universe in a few days. Of course, nutrients run out and growth slows — but that initial exponential phase, while it lasts, is one of the most mathematically pure phenomena in all of biology. A cell doubles; two cells double; four cells double. It is the simplest possible dynamical system: $dN/dt = \mu N$. Yet from this simple equation emerge practical consequences that pervade every experiment you will ever do with bacteria: how cell density changes over time, how long you have before cultures saturate, and why the physiological state of your cells at 9 AM matters deeply for the reproducibility of your results.

Bacterial growth is the increase in cell number over time. Unlike higher organisms, where growth refers to cell or organism size, bacteria grow primarily by cell division. The quantitative description of bacterial growth — exponential kinetics, nutrient-limited growth, and continuous culture dynamics — is foundational for metabolic modeling, synthetic circuit design, and interpretation of any experiment done in bacteria.

## The Growth Curve: Four Phases

When bacteria are inoculated into fresh liquid medium, growth follows a characteristic four-phase pattern:

**1. Lag phase**: Cells adapt to new conditions — synthesizing enzymes for available nutrients, repairing damage, adjusting internal pools. No net increase in cell number. Duration depends on the culture's prior history and the degree of environmental change.

**2. Exponential (log) phase**: Cells divide at a constant rate; the number doubles every **doubling time $\tau$**. This is the physiologically defined "growth phase" — cells are at maximum metabolic activity, uniform composition, and genetically most reproducible. Most quantitative measurements should be made on exponential-phase cells.

**3. Stationary phase**: Nutrient depletion (usually carbon or nitrogen) causes growth to cease. Cell number plateaus: growth rate ≈ death rate. Cells activate stress responses (σS/σ38 in *E. coli*), alter morphology, and produce secondary metabolites. Stationary-phase cells are physiologically distinct from exponential cells.

**4. Death phase**: Cell viability declines as lytic enzymes and toxins accumulate. Typically exponential, but some cells can persist for days to weeks.

## Exponential Growth Kinetics

During the exponential phase, the growth rate is:

$$\frac{dN}{dt} = \mu N$$

where $N$ is cell number (or optical density as a proxy) and $\mu$ is the **specific growth rate** (units: h$^{-1}$ or min$^{-1}$). The solution is:

$$N(t) = N_0 \cdot e^{\mu t} = N_0 \cdot 2^{t/\tau}$$

The **doubling time** $\tau = \ln 2 / \mu \approx 0.693/\mu$.

*E. coli* growing in LB (rich medium): $\tau \approx 20$ min → $\mu \approx 2.1$ h$^{-1}$
*E. coli* in M9 minimal + glucose: $\tau \approx 40$–60 min → $\mu \approx 0.7$–1.0 h$^{-1}$
*Mycobacterium tuberculosis*: $\tau \approx 24$ h → $\mu \approx 0.03$ h$^{-1}$

Measuring $\mu$: take the slope of $\ln(OD_{600})$ vs. time during the exponential phase:

$$\mu = \frac{\ln(OD_2) - \ln(OD_1)}{t_2 - t_1}$$

## The Monod Equation: Nutrient-Limited Growth

In minimal media, growth rate depends on the concentration of the limiting nutrient $[S]$ (usually carbon source). The relationship was quantified by Jacques Monod (1942) with an equation formally identical to Michaelis-Menten kinetics:

$$\mu = \mu_{\max} \cdot \frac{[S]}{K_s + [S]}$$

where:
- $\mu_{\max}$: maximum specific growth rate (nutrient-saturated)
- $K_s$: half-saturation constant (the nutrient concentration at which $\mu = \mu_{\max}/2$); for *E. coli* with glucose, $K_s \approx 0.01$–0.1 mM

For glucose in rich media, $[S] \gg K_s$, so $\mu \approx \mu_{\max}$. In natural environments, nutrient concentrations are typically $\sim K_s$, meaning cells grow at ~50% of their maximum rate.

Monod kinetics are the standard growth model in metabolic flux analysis and chemostat theory.

## Batch vs. Continuous Culture

**Batch culture**: nutrients are consumed over time; growth transitions through all four phases. Most laboratory experiments are batch.

**Chemostat (continuous culture)**: fresh medium flows in at a fixed **dilution rate $D$** (h$^{-1}$) and culture flows out at the same rate. At steady state, the growth rate must equal the dilution rate:

$$\mu^* = D$$

Substituting into the Monod equation, the steady-state substrate concentration is:

$$[S]^* = \frac{K_s D}{\mu_{\max} - D}$$

The steady-state cell density is:

$$X^* = Y \cdot (S_0 - [S]^*)$$

where $Y$ is the **yield coefficient** (g cells produced per g substrate consumed) and $S_0$ is the feed concentration.

The chemostat imposes a controlled, steady growth rate — cells are in a perpetual exponential phase. This makes it ideal for studying the physiology of defined growth rates, for evolutionary experiments (where sustained selection pressure can be applied), and for industrial fermentation.

## Worked Example: Chemostat Steady State

*E. coli* chemostat with glucose feed: $S_0 = 1$ g/L, $\mu_{\max} = 1.0$ h$^{-1}$, $K_s = 0.02$ g/L, $Y = 0.5$ g cells/g glucose. Dilution rate $D = 0.6$ h$^{-1}$.

$$[S]^* = \frac{0.02 \times 0.6}{1.0 - 0.6} = \frac{0.012}{0.4} = 0.03 \text{ g/L}$$

$$X^* = 0.5 \times (1.0 - 0.03) = 0.485 \text{ g/L dry cell weight}$$

Note: at $D = \mu_{\max}$ (or greater), cells are washed out — washout is a pitfall in chemostat operation.

## Cell Size, Ribosome Content, and Growth Rate

Growth rate is not just a number — it is tightly linked to cellular physiology. Faster-growing *E. coli* cells are larger, contain more ribosomes (which dominate the proteome at fast growth), and have multiple simultaneous rounds of DNA replication. The **growth law** describes the correlation:

$$\phi_R = \phi_R^0 + \kappa \cdot \lambda$$

where $\phi_R$ is the ribosome mass fraction, $\lambda$ is growth rate, and the constants $\phi_R^0$ and $\kappa$ are empirically determined (~0.07 and 0.17 h, respectively). This "proteome allocation" perspective has been formalized in resource allocation models (Scott & Hwa lab) that explain why overexpressing a synthetic circuit slows host growth — ribosomes are finite and must be distributed among all proteins.

This last point deserves emphasis. When you express a synthetic gene circuit in *E. coli*, you are not operating in a vacuum. You are competing with the cell's entire proteome for a finite pool of ribosomes. A circuit that demands a large fraction of cellular resources will slow growth, which will in turn affect the circuit's own dynamics (because dilution by growth is a major degradation mechanism for proteins). The circuit and the host physiology are coupled, and ignoring that coupling produces models that fail to predict behavior in real cells.

## Why This Matters for Computational Biology

Bacterial growth models are the foundation of metabolic modeling. Steady-state flux balance analysis (FBA) assumes growth at a defined rate; the Monod equation connects nutrient supply to growth rate. Chemostat experiments provide controlled conditions for measuring gene expression, proteome composition, and metabolic fluxes at fixed growth rates — these data parameterize genome-scale models. The carrying capacity of a batch culture ($N_{\max}$) matters for interpreting synthetic gene circuit experiments: circuits that impose burden reduce $\mu$, changing cell density at a given time point. Growth rate as a function of circuit expression is measured by comparing OD traces of circuit-bearing vs. circuit-free strains — a standard characterization measurement in synthetic biology.
