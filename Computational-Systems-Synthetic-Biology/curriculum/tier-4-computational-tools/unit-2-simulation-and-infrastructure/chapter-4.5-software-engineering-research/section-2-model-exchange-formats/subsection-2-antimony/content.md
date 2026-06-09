# Antimony: Human-Readable Model Language

What would it take to write a model of the repressilator — Elowitz and Leibler's three-gene oscillator — in a format that a biologist, not just a programmer, could read? The repressilator consists of three repressor proteins, each inhibiting the transcription of the next, arranged in a ring. You want to write that down in a way that makes the biology visible. In raw SBML, expressing this requires over two hundred lines of verbose XML: compartment declarations, species lists, kinetic law mathematics encoded in MathML, reaction definitions, parameter tables. By the time you've written it all out, the biological structure you started with has been buried under syntax.

**Antimony** is a human-readable model definition language for systems biology, designed as a higher-level alternative to raw SBML. Writing an SBML model by hand requires hundreds of lines of XML; the same model in Antimony might take 10 lines. Antimony compiles to SBML, integrates with roadrunner for simulation, and is the native language of the **tellurium** Python environment for systems biology.

## Antimony Syntax

Antimony syntax is designed to be read like a biological model description:

```python
import tellurium as te

# Basic Antimony syntax demonstration
repressilator_model = """
# Elowitz-Leibler Repressilator (2000)
# Three repressor proteins in a negative feedback cycle

# Species declarations
var m1, m2, m3;      # mRNAs (variable species)
var p1, p2, p3;      # Proteins (variable species)

# Reactions: arrows define stoichiometry; after semicolons = kinetic law
# Transcription (repressed by preceding protein)
J1: -> m1; alpha / (1 + p3^n) + alpha0 - delta*m1;
J2: -> m2; alpha / (1 + p1^n) + alpha0 - delta*m2;
J3: -> m3; alpha / (1 + p2^n) + alpha0 - delta*m3;

# Translation
J4: -> p1; beta * (m1 - p1);
J5: -> p2; beta * (m2 - p2);
J6: -> p3; beta * (m3 - p3);

# Parameters
alpha  = 216;      # max transcription rate (fold induction)
alpha0 = 0.0001;   # basal transcription rate
n      = 2;        # Hill coefficient
beta   = 5;        # translation/degradation ratio
delta  = 1;        # mRNA degradation rate

# Initial conditions (asymmetric to break symmetry)
m1 = 0; m2 = 0; m3 = 0;
p1 = 0; p2 = 0; p3 = 0;
"""

# Load and simulate
r = te.loadAntimonyModel(repressilator_model)
result = r.simulate(0, 200, 2000)
r.plot()
```

Read the reaction lines: `J1: -> m1; alpha / (1 + p3^n) + alpha0 - delta*m1;` — production of mRNA m1 at a rate governed by Hill repression from protein p3, minus first-order degradation. That is a complete statement of biology and mathematics in one line. The model reads almost like a caption.

## Antimony Language Reference

### Reaction Syntax

```python
# General form:
# reaction_id: [reactants] -> [products]; kinetic_law;

# Examples:
simple_model = """
# Unimolecular conversion
J1: A -> B; k1 * A;

# Bimolecular association
J2: A + B -> C; k2 * A * B;

# Reversible reaction
J3: C -> A + B; k3 * C - k4 * A * B;   # net rate expression

# Degradation (sink)
J4: A -> ; delta_A * A;

# Production (source)
J5: -> A; k_prod;

# Michaelis-Menten enzyme kinetics
J6: S -> P; (Vmax * S) / (Km + S);

# Parameters
k1 = 0.1; k2 = 0.05; k3 = 0.2; k4 = 0.01;
delta_A = 0.3; k_prod = 1.0; Vmax = 10; Km = 0.5;

# Initial conditions
A = 10; B = 0; C = 0; S = 20; P = 0;
"""

r_simple = te.loadAntimonyModel(simple_model)
result = r_simple.simulate(0, 50, 1000)
```

### Compartments

```python
compartment_model = """
# Model with multiple compartments
compartment cell = 1e-15;           # volume in litres
compartment nucleus = 1e-16;

# Species in specific compartments
var mRNA in nucleus;                # transcription in nucleus
var Protein in cell;               # translation in cytoplasm

# Reactions specify compartment volume for rate conversion
J_transcription: -> mRNA; nucleus * k_transcribe;
J_export:        mRNA -> Protein;  k_export * mRNA;
J_degradation:   Protein -> ;      cell * k_degrade * Protein;

k_transcribe = 1.0;
k_export     = 0.5;
k_degrade    = 0.2;

mRNA = 0; Protein = 0;
"""
```

### Assignment Rules and Events

```python
advanced_model = """
# Negative autoregulation with pulsed input

var mRNA, Protein;
const input_signal;

# Transcription regulated by protein (negative autoregulation)
J_tx:   -> mRNA;    k_tx / (1 + Protein/K) - delta_m * mRNA;
J_tl:   -> Protein; k_tl * mRNA - delta_p * Protein;

# Parameters
k_tx = 10; k_tl = 5;
delta_m = 1; delta_p = 0.5;
K = 2;  # repression threshold

# Event: switch signal on at t=20
at (time >= 20): k_tx = 20;   # double transcription rate
at (time >= 40): k_tx = 10;   # return to baseline

mRNA = 0; Protein = 0; input_signal = 0;
"""

r_adv = te.loadAntimonyModel(advanced_model)
result = r_adv.simulate(0, 80, 2000)
r_adv.plot()
```

## Converting Between Antimony and SBML

```python
import antimony
import tellurium as te

def antimony_to_sbml(antimony_string, sbml_level=3, sbml_version=2):
    """
    Convert an Antimony model string to SBML format.
    """
    # Load into the antimony module
    antimony.clearPreviousLoads()
    ret = antimony.loadAntimonyString(antimony_string)
    if ret < 0:
        raise ValueError(f"Antimony parsing error: {antimony.getLastError()}")

    # Get SBML output
    module_name = antimony.getModuleNames()[0] if antimony.getModuleNames() else "__main"
    sbml = antimony.getSBMLString(module_name, sbml_level, sbml_version)

    if not sbml:
        raise RuntimeError("Failed to generate SBML from Antimony model")

    print(f"Antimony → SBML conversion successful")
    print(f"  Module: {module_name}")
    print(f"  SBML Level {sbml_level} Version {sbml_version}")
    print(f"  SBML string length: {len(sbml)} characters")
    return sbml

def sbml_to_antimony(sbml_file):
    """
    Convert an SBML file to Antimony format.
    """
    antimony.clearPreviousLoads()
    ret = antimony.loadSBMLFile(sbml_file)
    if ret < 0:
        raise ValueError(f"SBML loading error: {antimony.getLastError()}")

    ant_string = antimony.getAntimonyString()
    print(f"SBML → Antimony conversion: {len(ant_string)} characters")
    return ant_string

# Roundtrip test: Antimony → SBML → Antimony
model_str = repressilator_model  # defined above
sbml_output = antimony_to_sbml(model_str)
ant_recovered = sbml_to_antimony_from_string(sbml_output)
```

## Parameter Scanning with Tellurium

```python
import numpy as np
import matplotlib.pyplot as plt

def parameter_scan_1d(model_string, parameter_name, param_values,
                       t_end=200, observable="p1"):
    """
    Scan a single parameter and record steady-state or trajectory of observable.
    """
    r = te.loadAntimonyModel(model_string)
    results = {}

    for value in param_values:
        r.resetAll()
        setattr(r, parameter_name, value)
        result = r.simulate(0, t_end, 2000)

        # Extract trajectory of observable
        obs_col = f"[{observable}]"
        if obs_col in result.colnames:
            # Last 20% of trajectory = approximate steady state
            traj = result[result.colnames.index(obs_col)]
            n_ss = len(traj) // 5
            results[value] = {"mean": traj[-n_ss:].mean(), "trajectory": traj}

    # Plot parameter scan
    fig, axes = plt.subplots(1, 2, figsize=(12, 4))

    # Bifurcation diagram
    ss_values = [results[v]["mean"] for v in param_values]
    axes[0].plot(param_values, ss_values, "o-", color="steelblue")
    axes[0].set_xlabel(parameter_name)
    axes[0].set_ylabel(f"Mean [{observable}] (last 20% of trajectory)")
    axes[0].set_title(f"Steady-state scan of {parameter_name}")

    # Example trajectories at 3 parameter values
    indices = [0, len(param_values)//2, -1]
    for i in indices:
        v = param_values[i]
        time_points = np.linspace(0, t_end, 2000)
        axes[1].plot(time_points, results[v]["trajectory"],
                     label=f"{parameter_name} = {v:.2f}", alpha=0.7)
    axes[1].set_xlabel("Time")
    axes[1].set_ylabel(f"[{observable}]")
    axes[1].legend()
    axes[1].set_title("Trajectories at selected parameter values")

    plt.tight_layout()
    return results

# Scan Hill coefficient n in the repressilator
n_values = np.arange(1, 5.5, 0.5)
scan_results = parameter_scan_1d(repressilator_model, "n", n_values,
                                  t_end=300, observable="p1")
```

## Worked Example: Toggle Switch

```python
toggle_switch = """
# Gardner-Cantor-Collins toggle switch (2000)
# Two mutually repressing proteins; bistable system

var u, v;   # protein concentrations (dimensionless)

# Protein u transcription (repressed by v)
J1: -> u; alpha1 / (1 + v^beta) - u;

# Protein v transcription (repressed by u)
J2: -> v; alpha2 / (1 + u^gamma) - v;

# Parameters
alpha1 = 3;   # effective transcription rate of protein u
alpha2 = 3;   # effective transcription rate of protein v
beta   = 2;   # cooperativity of repression of u by v
gamma  = 2;   # cooperativity of repression of v by u

# Initial conditions: low-u state
u = 0.1; v = 3.0;
"""

r_toggle = te.loadAntimonyModel(toggle_switch)

# Demonstrate bistability: two initial conditions converge to different states
fig, ax = plt.subplots(figsize=(8, 4))

initial_conditions = [(0.1, 3.0), (3.0, 0.1)]
labels = ["Low-u state (u₀=0.1, v₀=3.0)", "High-u state (u₀=3.0, v₀=0.1)"]
colors = ["blue", "red"]

for (u0, v0), label, color in zip(initial_conditions, labels, colors):
    r_toggle.resetAll()
    r_toggle.u = u0
    r_toggle.v = v0
    result = r_toggle.simulate(0, 30, 1000)
    time = result[:, 0]
    u_traj = result[:, 1]  # [u]
    v_traj = result[:, 2]  # [v]
    ax.plot(time, u_traj, color=color, lw=2, label=f"u: {label}")
    ax.plot(time, v_traj, color=color, lw=2, ls="--", alpha=0.7)

ax.set_xlabel("Time")
ax.set_ylabel("Protein concentration (dimensionless)")
ax.set_title("Toggle switch bistability: solid=u, dashed=v")
ax.legend(fontsize=8)
plt.tight_layout()
print("Toggle switch: bistable — both final states are valid steady states")
print("  The cell 'remembers' its initial state (epigenetic-like memory)")
```

## Why This Matters

Antimony dramatically lowers the barrier to creating and sharing systems biology models. Where writing a repressilator model in raw SBML requires 200+ lines of XML, the same model in Antimony takes 12 lines and is immediately readable. This matters for education (models should be comprehensible), collaboration (others can understand and modify your models), and publication (model code that appears in papers should be readable). The tellurium ecosystem — Antimony for model specification, roadrunner for simulation, and SBtab for data — provides a complete Python-native systems biology workflow that produces publication-quality simulations directly from human-readable model descriptions.
