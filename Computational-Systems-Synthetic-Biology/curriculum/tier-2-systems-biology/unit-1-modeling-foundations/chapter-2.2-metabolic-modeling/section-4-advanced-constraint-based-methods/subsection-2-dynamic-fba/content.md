# Dynamic FBA: Coupling Metabolism to Changing Environments

## The Limitation of Static FBA

An *E. coli* batch culture starts with 10 mM glucose. Over the course of 10 hours, glucose is consumed, cells grow, CO₂ accumulates, pH shifts, oxygen becomes limiting, and eventually the culture exhausts its carbon source and growth halts. Throughout this process, the metabolic state is constantly changing — the flux through glycolysis early in the growth curve is different from the flux when glucose is nearly depleted. Standard FBA, which assumes a fixed environment, cannot describe any of this time-course behavior. It is a photograph; batch fermentation is a film.

Standard FBA produces a single flux distribution representing a metabolic steady state. It does not describe how fluxes change over time as nutrients are consumed and products accumulate. For batch fermentation — the most common bioprocess format — metabolite concentrations change continuously, and the metabolic state must be updated dynamically.

**Dynamic Flux Balance Analysis (dFBA)** couples FBA to kinetic equations for the extracellular environment, producing time-course predictions of growth, substrate consumption, and product secretion.

## The dFBA Framework

dFBA consists of two components:

**1. FBA (inner problem)**: at each time point, solve FBA given the current external metabolite concentrations to obtain optimal flux vector $\mathbf{v}^*(t)$ and growth rate $\mu^*(t)$.

**2. Dynamic equations (outer problem)**: integrate ordinary differential equations describing changes in extracellular metabolite concentrations $[S_i]$ and biomass $[X]$:

$$\frac{d[S_i]}{dt} = -v_i^{\text{exchange}}([S_1], \ldots, [S_k]) \cdot [X]$$

$$\frac{d[X]}{dt} = \mu^*([S_1], \ldots, [S_k]) \cdot [X]$$

The exchange fluxes $v_i^\text{exchange}$ depend on current substrate concentrations through Michaelis-Menten-like uptake kinetics:

$$v_\text{uptake,i} = v_\text{max,i} \cdot \frac{[S_i]}{K_{m,i} + [S_i]}$$

This uptake rate is imposed as the lower bound of the exchange reaction in FBA at each time step, making the inner problem depend on the outer state.

## Implementation

```python
import numpy as np
from scipy.integrate import solve_ivp
import cobra

def dfba(model, s0, x0, t_end, dt=0.1, 
         vmax_glc=10.0, km_glc=0.5,
         vmax_o2=20.0, km_o2=0.001):
    """
    Dynamic FBA for batch culture.
    
    s0: dict of initial substrate concentrations {rxn_id: concentration}
    x0: initial biomass (gDW/L)
    """
    t_points = [0.0]
    x_points = [x0]
    s_points = [{k: v for k, v in s0.items()}]
    mu_points = [0.0]
    
    t = 0.0
    x = x0
    s = dict(s0)
    
    while t < t_end and x > 1e-6:
        # Set uptake constraints based on current concentrations
        with model:
            # Glucose: Michaelis-Menten uptake
            glc_conc = max(s.get('EX_glc__D_e', 0), 0)
            glc_uptake = vmax_glc * glc_conc / (km_glc + glc_conc)
            model.reactions.get_by_id('EX_glc__D_e').lower_bound = -glc_uptake
            
            # Oxygen: Michaelis-Menten uptake
            o2_conc = max(s.get('EX_o2_e', 0), 0)
            o2_uptake = vmax_o2 * o2_conc / (km_o2 + o2_conc)
            model.reactions.get_by_id('EX_o2_e').lower_bound = -o2_uptake
            
            # Solve FBA
            sol = model.optimize()
            if sol.status != 'optimal':
                break
            
            mu = sol.objective_value
            fluxes = sol.fluxes
        
        # Update state using Euler integration
        x_new = x + mu * x * dt
        s_new = dict(s)
        for rxn_id in s.keys():
            exchange_flux = fluxes.get(rxn_id, 0)  # mmol/gDW/h
            s_new[rxn_id] = max(0, s[rxn_id] + exchange_flux * x * dt)
        
        t += dt
        x = x_new
        s = s_new
        
        t_points.append(t)
        x_points.append(x)
        s_points.append(dict(s))
        mu_points.append(mu)
    
    return (np.array(t_points), np.array(x_points), 
            s_points, np.array(mu_points))

# Run dFBA
model = cobra.io.load_model('e_coli_core')

s_initial = {
    'EX_glc__D_e': 10.0,    # mM glucose
    'EX_o2_e': 0.21,         # mM (dissolved O2, air-saturated)
}

t, X, S_list, mu = dfba(model, s_initial, x0=0.01, t_end=10.0)

import matplotlib.pyplot as plt
glc = [s['EX_glc__D_e'] for s in S_list]
o2  = [s['EX_o2_e'] for s in S_list]

fig, axes = plt.subplots(3, 1, figsize=(10, 9))
axes[0].plot(t, X, 'steelblue'); axes[0].set_ylabel('Biomass (gDW/L)')
axes[1].plot(t, glc, 'coral', label='Glucose'); 
axes[1].plot(t, [x*10 for x in o2], 'green', label='O₂ ×10')
axes[1].set_ylabel('Concentration (mM)'); axes[1].legend()
axes[2].plot(t, mu, 'purple'); axes[2].set_ylabel('Growth rate (h⁻¹)')
plt.xlabel('Time (h)'); plt.tight_layout()
```

## Diauxic Growth

One of the most important applications of dFBA is modeling **diauxic growth** — the phenomenon where bacteria (especially *E. coli*) preferentially consume glucose over alternative carbon sources. During glucose consumption, catabolite repression suppresses genes for alternative carbon source metabolism. When glucose is exhausted, a lag phase occurs while the bacterium upregulates the alternative pathways, then growth resumes on the second substrate.

dFBA can model this by incorporating regulatory constraints (rFBA):
- Phase 1: glucose present → catabolite repression → only glucose metabolism active → rapid growth
- Lag phase: glucose depleted → regulatory switch → time delay (modeled as a first-order gene expression response)
- Phase 2: glucose absent → lactose/acetate metabolism active → slower growth

Matching dFBA predictions to measured batch culture time courses (biomass, glucose, acetate, lactose) validates both the metabolic model and the regulatory logic.

## dFBA Limitations and Extensions

**Euler integration is simple but inaccurate**: for long batch cultures, use higher-order ODE integrators (RK4, adaptive step control). The FBA solution must be recomputed at every integration step.

**Computational cost**: each time step requires solving an LP. For long simulations with many time steps, this can be computationally expensive (seconds to minutes per simulation for large GEMs).

**Sudden depletion artifacts**: when a substrate approaches zero, Michaelis-Menten uptake goes to zero smoothly — but the FBA still returns a small nonzero flux below threshold. Numerical care is needed to prevent negative concentrations.

**ME-models + dFBA**: coupling ME-models (Section 2.5.5) to dynamic equations adds proteome dynamics — enzyme levels change in response to growth conditions. This produces more realistic lag phases and regulatory responses.

## Why This Matters

dFBA bridges the gap between the static FBA snapshot and the dynamic reality of a fermentation batch. It is the standard tool for simulating fed-batch and batch fermentation processes in metabolic engineering: predicting how a designed strain will behave over a 24-hour fermentation, identifying when nutrients become limiting, and optimizing feeding strategies. It also enables comparison with the wealth of time-course metabolomics and proteomics data from fermentation experiments, providing a multi-scale validation of GEM accuracy in dynamic conditions.
