# Reservoir Computing — Lab Manual

Hands-on companion to *Reservoir Computing: From First Principles to the Frontier*.
Each lab is a self-contained Python script that can be run directly or converted to a
Jupyter notebook.  Labs build on each other: the early ones introduce core concepts with
minimal code; later ones involve full experimental pipelines.

---

## Setup

```bash
cd labs
pip install -r requirements.txt   # numpy, scipy, matplotlib, scikit-learn, jupyterlab
```

Verify the shared utilities load correctly:

```bash
python -c "from utils import EchoStateNetwork, narma, nmse; print('utilities OK')"
```

All lab scripts import from `utils/` using a relative path, so **run them from inside
the `labs/` directory**:

```bash
cd labs
python 01_fading_memory/fading_memory.py
```

To open a lab as a notebook:

```bash
jupytext --to notebook 01_fading_memory/lab.py   # requires: pip install jupytext
jupyter lab
```

---

## Lab Overview

| # | Script | Topic | Textbook Chapter |
|---|--------|-------|-----------------|
| 01 | `01_fading_memory/fading_memory.py` | Fading memory & the echo state property | Ch. 4 |
| 02a | `02_dynamical_systems/lorenz_phase_portrait.py` | Lorenz attractor: phase portrait & Lyapunov exponent | Ch. 5 |
| 02b | `02_dynamical_systems/bifurcation_diagram.py` | Logistic map bifurcations & Feigenbaum constant | Ch. 5 |
| 03 | `03_echo_state_networks/first_esn.py` | Building and driving an ESN on NARMA-10 | Ch. 6 |
| 04 | `04_echo_state_property/esp_test.py` | Empirical ESP test across spectral radii | Ch. 7 |
| 05 | `05_spectral_radius/rho_sweep.py` | Spectral radius sweep: NMSE and memory capacity | Ch. 8 |
| 06 | `06_memory_capacity/memory_capacity.py` | Jaeger (2002) linear memory capacity | Ch. 9 |
| 07 | `07_narma10/narma10_benchmark.py` | NARMA-10 benchmark end-to-end | Ch. 10 |
| 08 | `08_mackey_glass/mackey_glass_prediction.py` | Mackey-Glass prediction & closed-loop rollout | Ch. 11 |
| 09a | `09_hyperparameters/hyperparameter_scan.py` | Grid search & random search over (ρ, σ_in, λ) | Ch. 12 |
| 09b | `09_hyperparameters/leak_rate_effect.py` | Leak rate effect on memory and timescale | Ch. 12 |
| 10 | `10_online_rls/rls_online.py` | Online readout training with RLS | Ch. 14 |
| 11 | `11_force_learning/force_pattern.py` | FORCE learning: training recurrent weights | Ch. 15 |
| 12 | `12_conceptors/conceptors.py` | Conceptors for multi-pattern storage/retrieval | Ch. 17 |
| 13 | `13_deep_esn/deep_esn.py` | Deep (layered) ESNs | Ch. 19 |
| 14 | `14_nvar/nvar_lorenz.py` | Next-Generation RC: NVAR without a reservoir | Ch. 20 |
| 15 | `15_physical_rc/delay_reservoir.py` | Physical RC: single-node delay-line reservoir | Ch. 22 |
| 16 | `16_lorenz_prediction/lorenz_prediction.py` | Lorenz-63 autonomous prediction & VPT | Ch. 24 |
| 17 | `17_central_pattern_generator/cpg.py` | CPG: rhythmic pattern generation with FORCE | Ch. 26 |

---

## Recommended Learning Path

### Beginner (Labs 1–5)
Start here if you are new to reservoir computing or recurrent neural networks.

- **Lab 01** demonstrates why a fixed random network can act as a memory — the key
  insight that makes RC possible.
- **Lab 02** builds intuition for reservoir dynamics by visualising state trajectories.
- **Lab 03** assembles a complete ESN pipeline: generate data → drive reservoir → train
  readout → evaluate.
- **Lab 04** tests the Echo State Property empirically: do different initial states
  converge?
- **Lab 05** reveals the critical role of spectral radius through a sweep experiment.

### Core RC (Labs 6–9)
Deeper dives into memory, canonical benchmarks, and practical tuning.

- **Lab 06** measures linear memory capacity and verifies the MC ≤ N bound.
- **Lab 07** tackles NARMA-10, a classic benchmark that couples memory and nonlinearity.
- **Lab 08** covers the Mackey-Glass time series, the standard RC regression benchmark.
- **Lab 09** explores hyperparameter sensitivity with grid and random search.

### Advanced Training (Labs 10–12)
Move beyond batch ridge regression to online and recurrent learning.

- **Lab 10** replaces the batch readout with online Recursive Least Squares (RLS).
- **Lab 11** implements FORCE learning (Sussillo & Abbott 2009) to train recurrent weights.
- **Lab 12** introduces Conceptors for storing and morphing multiple patterns.

### Extensions (Labs 13–17)
Modern variants, physical systems, and challenging benchmarks.

- **Lab 13** stacks multiple reservoirs into a deep ESN and compares to a single layer.
- **Lab 14** replaces the reservoir with polynomial delay features (NVAR/NG-RC).
- **Lab 15** simulates a delay-line physical reservoir and runs the spoken-digit task.
- **Lab 16** predicts the Lorenz-63 attractor autonomously and measures Valid Prediction
  Time.
- **Lab 17** generates CPG-style rhythmic outputs, connecting RC to motor control.

---

## Shared Utilities (`utils/`)

### `utils/esn.py` — EchoStateNetwork

```python
from utils import EchoStateNetwork

esn = EchoStateNetwork(
    n_reservoir    = 200,    # N: reservoir size
    spectral_radius= 0.95,   # ρ: controls memory/stability
    leak_rate      = 1.0,    # α: leaky integrator (1.0 = no leak)
    input_scaling  = 0.5,    # scales W_in
    connectivity   = 0.1,    # fraction of non-zero entries in W_rec
    ridge_alpha    = 1e-6,   # λ: ridge regression regularisation
    washout        = 100,    # initial steps discarded from training
    seed           = 42,
)

# Inputs must be 2-D: (T, n_inputs)
esn.fit(u_train.reshape(-1, 1), y_train)
y_pred = esn.predict(u_test.reshape(-1, 1))
```

Also includes `RLSReadout` for online training (see Lab 10).

### `utils/benchmarks.py` — Data Generators

```python
from utils import narma, mackey_glass, lorenz, lorenz_rk4
from utils.benchmarks import kuramoto_sivashinsky, channel_equalization

u, y  = narma(T=5000, order=10)         # NARMA-10
x     = mackey_glass(T=5000, tau=17)    # Mackey-Glass τ=17
xyz   = lorenz(T=10000, dt=0.02)        # Lorenz-63, shape (T, 3)
ks    = kuramoto_sivashinsky(T=5000)    # KS PDE, shape (T, 64)
u, d  = channel_equalization(T=5000)    # channel eq. task
```

### `utils/metrics.py` — Evaluation

```python
from utils import nmse, nrmse, valid_prediction_time, memory_capacity
from utils.metrics import information_processing_capacity

print(nmse(y_true, y_pred))                    # 0 = perfect, 1 = mean predictor
print(nrmse(y_true, y_pred))                   # sqrt(NMSE)
vpt = valid_prediction_time(y_true, y_pred, threshold=0.05, dt=0.02)

total_mc, mc_curve = memory_capacity(esn)
ipc = information_processing_capacity(esn)
print(ipc['total'], ipc['linear'], ipc['nonlinear'])
```

---

## Key References

| Topic | Reference |
|-------|-----------|
| ESN original | Jaeger, H. (2001). *The "echo state" approach to analysing and training recurrent neural networks.* GMD Report 148. |
| Practical guide | Lukoševičius, M. (2012). A practical guide to applying echo state networks. *Neural Networks: Tricks of the Trade*, 659–686. |
| Memory capacity | Jaeger, H. (2002). Short term memory in echo state networks. *GMD Report 152*. |
| FORCE learning | Sussillo, D. & Abbott, L. F. (2009). Generating coherent patterns of activity from chaotic neural networks. *Neuron 63*, 544–557. |
| Conceptors | Jaeger, H. (2014). Controlling recurrent neural networks by conceptors. *arXiv:1403.3369*. |
| NVAR / NG-RC | Gauthier, D. J. et al. (2021). Next generation reservoir computing. *Nature Communications 12*, 5564. |
| Lorenz prediction | Pathak, J. et al. (2018). Model-free prediction of large spatiotemporally chaotic systems. *Physical Review Letters 120*, 024102. |
| IPC | Dambre, J. et al. (2012). Information processing capacity of dynamical systems. *Scientific Reports 2*, 514. |

---

## Tips for Students

- **Always fix the random seed** (`seed=42`) when comparing configurations — a single
  reservoir sample can mislead you.
- **Washout matters.** For a reservoir with spectral radius near 1.0, use at least 200
  steps of washout.
- **Ridge alpha is your first tuning knob.** Start with `1e-6`; if the readout overfits,
  try `1e-4` or `1e-3`.
- **NMSE > 1 means you are worse than predicting the mean.** This is a useful sanity
  check that something is badly mis-configured.
- **Extend with input.** Keeping `extend_with_input=True` (the default) adds direct
  input-to-output connections and almost always helps.

---

## Running All Labs

```bash
cd labs
for script in $(find . -name "*.py" ! -path "*/utils/*" | sort); do
    echo "--- $script ---"
    python "$script" && echo "OK" || echo "FAILED"
done
```

Expected total runtime: ~5–10 minutes on a modern laptop (Lab 12/Conceptors and
Lab 17/CPG are the slowest at ~1–2 min each).
