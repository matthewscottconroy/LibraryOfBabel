# INCA: Isotopomer Network Compartmental Analysis

## What INCA Is

You have your isotope labeling data. You have your network model. You need software that implements the EMU framework, runs the nonlinear regression properly, and gives you confidence intervals that actually mean something. That software is INCA.

**INCA** (Isotopomer Network Compartmental Analysis) (Young et al. 2008) is the industry-standard MATLAB-based software package for ¹³C metabolic flux analysis. It implements the full EMU computational framework for simulating isotope labeling, nonlinear least-squares optimization to fit measured mass isotopologue distributions (MIDs) to a metabolic model, and rigorous statistical methods for confidence interval estimation.

INCA is used by leading academic and industrial metabolic engineering groups worldwide, and its outputs are required by major journals for publication of ¹³C MFA studies.

## Core Workflow

The INCA workflow proceeds in five stages:

### Stage 1: Model Specification

Define the metabolic network in INCA's notation:

```matlab
% Define reactions with atom mappings
m = inca.model();
m.add_rxn('PGI', 'G6P(abcdef) -> F6P(abcdef)', 'bi');  % reversible
m.add_rxn('PFK', 'F6P(abcdef) + ATP -> FBP(abcdef) + ADP', 'fwd');  % irreversible
m.add_rxn('FBA', 'FBP(abcdef) -> DHAP(cba) + GAP(def)', 'bi');
% ... all pathway reactions

% Specify tracer inputs
m.add_tracer('GLC', [0.5, 0, 0, 0, 0, 0, 0.5], 'U-13C6: 50%');
% MID of glucose: 50% M+0 (natural) + 50% M+6 (fully labeled)
```

Critically, every reaction requires a complete atom map — specifying the positional rearrangement of every carbon atom. INCA includes built-in maps for common central carbon metabolism reactions.

### Stage 2: Measurement Specification

Load the experimental MID data with associated measurement uncertainties:

```matlab
% Measured MIDs from GC-MS (after natural abundance correction)
m.add_measurement('ALA', [2,3], [0.620, 0.003; ...  % m0, m1, m2, m3
                                  0.289, 0.004;
                                  0.073, 0.003;
                                  0.018, 0.002]);
% [fragment carbons, [mean, std] for each mass isotopologue]

m.add_measurement('SUC', [1,2,3,4], [...]); % succinate, symmetric correction applied
m.add_measurement('GLU', [1,2], [...]);     % glutamate fragment, C1-C2 only
```

External flux measurements (glucose uptake rate, biomass growth rate, secretion rates) are added as equality constraints, anchoring the absolute flux scale.

### Stage 3: Flux Estimation

INCA solves the nonlinear least-squares problem:

$$\min_{\mathbf{v}} \sum_j \sum_k \frac{\left(m_{jk}^{\text{measured}} - m_{jk}^{\text{simulated}}(\mathbf{v})\right)^2}{\sigma_{jk}^2}$$

subject to stoichiometric and thermodynamic constraints (reversibility bounds).

The optimizer uses a trust-region method starting from multiple random initial points to avoid local minima:

```matlab
opts = inca.options('nstart', 50, 'tol', 1e-6);
results = inca.estimate(m, opts);

% Best fit
best_flux = results(1).flux;
best_sst = results(1).sst;  % sum of squared residuals
fprintf('Best SST: %.4f (DOF = %d)\n', best_sst, results(1).dof);
```

The goodness of fit is evaluated using the $\chi^2$ statistic: $\text{SST}/\text{DOF}$ should be close to 1 for a well-fitting model.

### Stage 4: Confidence Interval Estimation

INCA provides two methods for confidence intervals:

**Bootstrap method**: Resample the measured MID data within their uncertainties $n$ times (typically $n = 100-500$), refit the model for each resampled dataset, and report the distribution of flux estimates. This is computationally intensive but assumption-free.

**Continuation method**: Systematically vary each flux and find the boundaries where the SST increases beyond an acceptable threshold ($\chi^2$ at 95% confidence level). Faster than bootstrap.

```matlab
% Confidence intervals by continuation
ci = inca.confidence_intervals(m, results(1), 'method', 'continuation', ...
                                'alpha', 0.05);
ci_table = array2table([best_flux.value, ci.lower, ci.upper], ...
    'VariableNames', {'flux', 'CI_lower', 'CI_upper'});
```

Confidence intervals in ¹³C MFA can be asymmetric — especially for near-zero fluxes (which are bounded below by 0) or for reversible reactions where the net and exchange fluxes are correlated.

### Stage 5: Output and Validation

INCA produces flux maps that can be exported for visualization:

```matlab
% Export to CSV
writetable(ci_table, 'flux_results.csv');

% Check model adequacy: residuals
for j = 1:length(results(1).measurements)
    fprintf('%s residuals: %.3f\n', m.measurements(j).name, ...
            results(1).residuals(j));
end
```

A good ¹³C MFA result shows:
- SST/DOF close to 1 (not significantly greater — would indicate model inadequacy)
- Residuals $<3\sigma$ for all measurements
- Narrow confidence intervals for key fluxes (TCA cycle, PPP/glycolysis split)

## Interpreting Key Outputs

**Reversible exchange fluxes**: For reversible reactions, INCA reports both the net flux $v_{\text{net}}$ and the exchange flux $v_{\text{ex}}$ (forward = reverse). Exchange fluxes encode how quickly the two substrate pools equilibrate isotopically. High exchange flux but low net flux indicates a near-equilibrium reaction.

**Symmetry corrections**: Succinate and fumarate are symmetric molecules — the two ends are chemically identical, so mass spectrometry cannot distinguish which carbons came from which branch of the TCA cycle. INCA automatically applies symmetry corrections for these metabolites.

## Practical Considerations

- **Isotopic steady state verification**: measure MIDs at two time points; if they agree, isotopic steady state was reached. If not, use isotopic non-stationary MFA (INST-MFA) — also supported by INCA.
- **Sample size**: 3-5 biological replicates are typically sufficient; more replicates improve confidence interval precision
- **Tracer selection**: design experiments to maximize information about the fluxes of interest using in silico tracer optimization tools (IsoDesign)
- **Network topology**: include all known alternative routes; omitting reactions can introduce bias in estimated fluxes

## Why This Matters

INCA has become the reference implementation for ¹³C MFA because it rigorously handles uncertainty, implements the EMU framework for computational efficiency, and produces confidence intervals that properly account for the nonlinearity of the fitting problem. Learning INCA gives you access to a methodology that has revealed the actual quantitative flux distributions in cancer cells, industrial fermentations, and plant metabolism — knowledge that is simply inaccessible to FBA alone.
