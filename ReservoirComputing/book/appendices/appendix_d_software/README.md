# Appendix D: Software and Libraries

This appendix provides practical guides to the main software tools used for reservoir computing experiments. For each tool, we provide installation instructions, basic usage patterns, and complete code examples.

---

## D.1 ReservoirPy — Comprehensive Tutorial

ReservoirPy is the primary Python library for reservoir computing, developed by Inria Bordeaux (Nathan Trouvain, Xavier Hinaut). It provides modular, composable reservoir computing models with a scikit-learn-compatible API.

### Installation

```bash
pip install reservoirpy
# For full dependencies including visualization:
pip install reservoirpy[extras]
```

### D.1.1 Basic ESN

```python
import numpy as np
import reservoirpy as rpy
from reservoirpy.nodes import Reservoir, Ridge, Input
from reservoirpy.datasets import mackey_glass

# Set global seed for reproducibility
rpy.set_seed(42)

# Load Mackey-Glass time series
X = mackey_glass(n_timesteps=2000, tau=17)  # shape: (2000, 1)

# Split: 1500 train, 500 test
X_train, X_test = X[:1500], X[1500:]

# Construct ESN: Input -> Reservoir -> Ridge readout
reservoir = Reservoir(
    units=500,               # N = 500 neurons
    sr=0.9,                  # spectral radius
    lr=0.3,                  # leaking rate (alpha)
    input_scaling=0.5,       # input weight scaling
    rc_connectivity=0.1,     # fraction of non-zero weights (sparsity)
    seed=42,
)

readout = Ridge(ridge=1e-6)   # L2 regularization parameter

esn = reservoir >> readout    # chaining with >> operator

# Train: the reservoir state is collected, then ridge regression is solved
esn.fit(X_train[:-1], X_train[1:], warmup=100)

# Predict
predictions = esn.run(X_test[:-1])

# Evaluate
from reservoirpy.observables import nrmse
error = nrmse(X_test[1:], predictions)
print(f"NRMSE: {error:.4f}")
```

### D.1.2 Hyperparameter Search

ReservoirPy integrates with `hyperopt` and `optuna` for hyperparameter optimization:

```python
from reservoirpy.nodes import Reservoir, Ridge
from reservoirpy.hyper import research, plot_hyperopt_report
import numpy as np

def objective(dataset, config, *, ys, y_test):
    """Objective function for hyperparameter optimization."""
    X_train, X_test = dataset
    
    reservoir = Reservoir(
        units=config["N"],
        sr=config["sr"],
        lr=config["lr"],
        input_scaling=config["input_scaling"],
        rc_connectivity=config["rc_connectivity"],
        seed=config["seed"],
    )
    readout = Ridge(ridge=config["ridge"])
    esn = reservoir >> readout
    
    esn.fit(X_train[:-1], X_train[1:], warmup=100)
    predictions = esn.run(X_test[:-1])
    
    # NMSE as the objective to minimize
    loss = np.mean((ys - predictions)**2) / np.var(ys)
    return {"loss": loss, "status": "ok"}


# Define hyperparameter search space
hyperopt_config = {
    "exp": "mackey_glass_hp_search",
    "hp_max_evals": 100,          # Number of trials
    "hp_method": "random",        # "random", "tpe", or "bayesian"
    "seed": 42,
    "instances_per_trial": 3,     # Average over 3 random seeds per trial
    "hp_space": {
        "N":              ["choice", [100, 200, 500, 1000]],
        "sr":             ["loguniform", 0.1, 2.0],
        "lr":             ["uniform", 0.01, 1.0],
        "input_scaling":  ["loguniform", 0.01, 10.0],
        "rc_connectivity":["choice", [0.05, 0.1, 0.2]],
        "ridge":          ["loguniform", 1e-8, 1.0],
    }
}

X = mackey_glass(2500, tau=17)
X_train, X_test = X[:2000], X[2000:]

best_params = research(
    objective,
    dataset=(X_train, X_test),
    config=hyperopt_config,
    ys=X_test[1:],
    y_test=X_test[1:],
)
print("Best parameters:", best_params)
```

### D.1.3 Online Learning

ReservoirPy supports online (recursive) readout training using the FORCE learning algorithm or recursive least squares (RLS):

```python
from reservoirpy.nodes import Reservoir, RLS
import numpy as np

# Mackey-Glass data
X = mackey_glass(2000, tau=17)

# RLS readout: updates weights online at each timestep
reservoir = Reservoir(units=300, sr=0.9, lr=0.3, seed=42)
rls_readout = RLS(
    alpha=1e-4,    # forgetting factor (1 = no forgetting)
    delta=1.0,     # initial covariance matrix diagonal
)
esn_online = reservoir >> rls_readout

# Online learning: fit processes each timestep sequentially
print("Online training...")
for t in range(100, 1500):  # start after washout
    esn_online.train(X[t:t+1], X[t+1:t+2])

# Test (frozen weights)
predictions = esn_online.run(X[1500:-1], reset=False)
```

### D.1.4 Custom Reservoir Nodes

ReservoirPy's node system allows building custom reservoir architectures:

```python
from reservoirpy.node import Node
import numpy as np


class LeakyIntegratorReservoir(Node):
    """
    Leaky integrator reservoir with configurable time constant.
    State equation: x(t) = (1-alpha)*x(t-1) + alpha*tanh(W_res@x + W_in@u + b)
    """
    
    def initialize(self, x=None, y=None):
        # Called on first run to set dimensions
        if x is not None:
            N = self.output_dim
            d_in = x.shape[-1]
            rng = np.random.RandomState(self.params.get("seed", 42))
            
            # Reservoir weights
            W = rng.randn(N, N)
            W[rng.rand(N, N) > 0.1] = 0.0
            ev = np.linalg.eigvals(W)
            sr = self.params.get("sr", 0.9)
            W *= sr / (np.max(np.abs(ev)) + 1e-10)
            
            self.set_param("W", W)
            self.set_param("W_in", self.params.get("s_in", 0.5) * rng.randn(N, d_in))
            self.set_param("b", 0.1 * rng.randn(N))
            self.set_state(np.zeros((1, N)))
    
    def forward(self, x):
        # x shape: (batch, d_in)
        W = self.params["W"]
        W_in = self.params["W_in"]
        b = self.params["b"]
        alpha = self.params.get("lr", 0.3)
        
        h = self.state()  # (1, N)
        pre = h @ W.T + x @ W_in.T + b
        h_new = (1 - alpha) * h + alpha * np.tanh(pre)
        self.set_state(h_new)
        return h_new


# Usage
custom_res = LeakyIntegratorReservoir(
    output_dim=500, sr=0.95, lr=0.2, s_in=0.3, seed=42
)
```

---

## D.2 Brian2 for Spiking Liquid State Machines

Brian2 is a Python simulator for spiking neural networks, appropriate for implementing Liquid State Machines (LSMs) with biologically realistic spiking dynamics.

### Installation

```bash
pip install brian2
```

### Basic LSM with Brian2

```python
from brian2 import *
import numpy as np

# Suppress startup messages
prefs.codegen.target = 'numpy'

def build_lsm(
    N_exc: int = 400,
    N_inh: int = 100,
    input_rate_hz: float = 50.0,
    run_time_ms: float = 1000.0,
) -> tuple:
    """
    Build and run a Liquid State Machine using Brian2.
    
    Returns reservoir states (spike rates) at each timestep.
    """
    start_scope()
    
    # Neuron parameters (leaky integrate-and-fire)
    tau_m = 20 * ms
    tau_ref = 2 * ms
    v_rest = -65 * mV
    v_thresh = -50 * mV
    v_reset = -65 * mV
    
    neuron_eqs = '''
    dv/dt = (v_rest - v + I_syn) / tau_m : volt (unless refractory)
    dI_syn/dt = -I_syn / (5*ms) : volt
    '''
    
    # Excitatory neurons
    exc_group = NeuronGroup(
        N_exc, neuron_eqs,
        threshold='v > v_thresh',
        reset='v = v_reset',
        refractory=tau_ref,
        method='euler',
    )
    exc_group.v = v_rest
    
    # Inhibitory neurons
    inh_group = NeuronGroup(
        N_inh, neuron_eqs,
        threshold='v > v_thresh',
        reset='v = v_reset',
        refractory=tau_ref,
        method='euler',
    )
    inh_group.v = v_rest
    
    # Poisson input
    input_group = PoissonGroup(10, rates=input_rate_hz * Hz)
    
    # Synapses (random connectivity, fixed weights)
    rng = np.random.RandomState(42)
    
    syn_ee = Synapses(exc_group, exc_group, on_pre='I_syn_post += 1.5*mV')
    syn_ee.connect(p=0.1)
    
    syn_ei = Synapses(exc_group, inh_group, on_pre='I_syn_post += 1.5*mV')
    syn_ei.connect(p=0.25)
    
    syn_ie = Synapses(inh_group, exc_group, on_pre='I_syn_post -= 3.0*mV')
    syn_ie.connect(p=0.25)
    
    syn_in = Synapses(input_group, exc_group, on_pre='I_syn_post += 5.0*mV')
    syn_in.connect(p=0.5)
    
    # Spike monitors
    mon_exc = SpikeMonitor(exc_group)
    
    # Rate monitor (binned firing rates — the reservoir "state")
    rate_mon = PopulationRateMonitor(exc_group)
    
    # Run
    run(run_time_ms * ms)
    
    return mon_exc, rate_mon


# Run and extract states
mon, rate_mon = build_lsm(run_time_ms=500.0)
print(f"Total spikes: {mon.num_spikes}")
print(f"Mean firing rate: {rate_mon.rate.mean() / Hz:.1f} Hz")
```

---

## D.3 PyTorch Integration

PyTorch is useful for: (1) differentiable reservoir variants where some weights are trained by backprop, (2) using pre-trained embeddings as reservoir inputs, (3) combining a fixed reservoir with a deep readout.

```python
import torch
import torch.nn as nn
import numpy as np


class TorchESN(nn.Module):
    """
    Echo State Network implemented in PyTorch.
    
    The reservoir weights (W_res, W_in) are fixed buffers.
    Only the readout layer (W_out) is a trainable parameter.
    
    This allows combining the ESN with a nonlinear readout
    if desired (e.g., add a hidden layer before the output).
    """
    
    def __init__(
        self,
        n_input: int,
        n_reservoir: int = 500,
        n_output: int = 1,
        spectral_radius: float = 0.9,
        input_scaling: float = 0.5,
        leaking_rate: float = 0.3,
        sparsity: float = 0.9,
        seed: int = 42,
    ):
        super().__init__()
        torch.manual_seed(seed)
        
        N = n_reservoir
        
        # Fixed reservoir weights (registered as buffers, not parameters)
        W = torch.randn(N, N)
        W[torch.rand(N, N) < sparsity] = 0.0
        ev = torch.linalg.eigvals(W).abs().max()
        W *= spectral_radius / ev
        self.register_buffer('W_res', W)
        self.register_buffer('W_in', input_scaling * torch.randn(N, n_input))
        self.register_buffer('bias', 0.1 * torch.randn(N))
        
        self.alpha = leaking_rate
        
        # Trainable readout
        self.readout = nn.Linear(N, n_output, bias=True)
        
        # Initialize readout to zero
        nn.init.zeros_(self.readout.weight)
        nn.init.zeros_(self.readout.bias)
    
    def forward(self, u_seq: torch.Tensor, return_states: bool = False):
        """
        Process a sequence.
        
        Parameters
        ----------
        u_seq : Tensor, shape (batch, T, n_input) or (T, n_input)
        
        Returns
        -------
        output : Tensor, shape (batch, T, n_output) or (T, n_output)
        """
        if u_seq.dim() == 2:
            u_seq = u_seq.unsqueeze(0)  # add batch dim
        
        B, T, _ = u_seq.shape
        x = torch.zeros(B, self.W_res.shape[0], device=u_seq.device)
        
        states = []
        for t in range(T):
            pre = x @ self.W_res.T + u_seq[:, t] @ self.W_in.T + self.bias
            x = (1 - self.alpha) * x + self.alpha * torch.tanh(pre)
            states.append(x)
        
        states = torch.stack(states, dim=1)  # (B, T, N)
        output = self.readout(states)        # (B, T, n_output)
        
        if return_states:
            return output.squeeze(0), states.squeeze(0)
        return output.squeeze(0)
    
    def fit_ridge(self, X_states: torch.Tensor, Y: torch.Tensor, ridge: float = 1e-4):
        """
        Fit the readout layer using ridge regression (closed-form).
        X_states: (T, N), Y: (T, n_output)
        """
        with torch.no_grad():
            A = X_states.T @ X_states + ridge * torch.eye(X_states.shape[1])
            b = X_states.T @ Y
            W = torch.linalg.solve(A, b)  # (N, n_output)
            self.readout.weight.copy_(W.T)
            self.readout.bias.zero_()


# Example: NARMA-10 with PyTorch ESN
def narma10_pytorch_example():
    from reservoirpy.datasets import narma  # use reservoirpy for NARMA generation
    
    # Generate NARMA-10 data
    np.random.seed(42)
    u = np.random.uniform(0, 0.5, 2200)
    y = np.zeros(2200)
    for t in range(10, 2200):
        y[t] = (0.3*y[t-1]
                + 0.05*y[t-1]*np.sum(y[t-10:t])
                + 1.5*u[t-10]*u[t-1]
                + 0.1)
    
    U = torch.FloatTensor(u[:-1, None])
    Y = torch.FloatTensor(y[1:, None])
    
    # Train
    model = TorchESN(n_input=1, n_reservoir=500, n_output=1)
    
    # Collect states (no gradient needed for fixed reservoir)
    with torch.no_grad():
        _, states = model(U[:1700], return_states=True)
    
    # Ridge regression on training states (after washout)
    washout = 100
    model.fit_ridge(states[washout:], Y[washout:1700], ridge=1e-5)
    
    # Evaluate on test set
    with torch.no_grad():
        preds = model(U[1700:])
    
    nmse = ((Y[1700:] - preds)**2).mean() / Y[1700:].var()
    print(f"NARMA-10 NMSE (PyTorch ESN): {nmse.item():.4f}")
```

---

## D.4 Qiskit for Quantum Reservoir Computing

Qiskit (IBM Quantum) enables simulation of quantum reservoir computing circuits.

```bash
pip install qiskit qiskit-aer
```

```python
from qiskit import QuantumCircuit, transpile
from qiskit_aer import AerSimulator
import numpy as np


class QuantumReservoir:
    """
    Quantum reservoir computing using parameterized quantum circuits.
    
    Input is encoded as rotation angles on qubits.
    Reservoir dynamics: ZZ-interaction Hamiltonian.
    Readout: expectation values of Pauli operators.
    """
    
    def __init__(
        self,
        n_qubits: int = 4,
        n_layers: int = 2,
        J: float = 1.0,    # coupling strength
        h: float = 0.5,    # transverse field
        dt: float = 0.3,   # evolution time
        shots: int = 1024,
        seed: int = 42,
    ):
        self.n_qubits = n_qubits
        self.n_layers = n_layers
        self.J = J
        self.h = h
        self.dt = dt
        self.shots = shots
        self.sim = AerSimulator(seed_simulator=seed)
    
    def _build_circuit(self, u: float) -> QuantumCircuit:
        """Build QRC circuit for scalar input u."""
        n = self.n_qubits
        qc = QuantumCircuit(n, n)
        
        # Input encoding: rotate qubit 0 by u
        qc.rx(u * np.pi, 0)
        
        # Reservoir: ZZ + transverse field evolution (Trotterized)
        for layer in range(self.n_layers):
            # Transverse field (Rx rotations)
            for q in range(n):
                qc.rx(2 * self.h * self.dt, q)
            
            # ZZ coupling
            for q in range(n - 1):
                qc.cx(q, q + 1)
                qc.rz(2 * self.J * self.dt, q + 1)
                qc.cx(q, q + 1)
        
        # Measure
        qc.measure(range(n), range(n))
        return qc
    
    def get_state(self, u: float) -> np.ndarray:
        """Get reservoir readout state for input u."""
        qc = self._build_circuit(u)
        compiled = transpile(qc, self.sim)
        result = self.sim.run(compiled, shots=self.shots).result()
        counts = result.get_counts()
        
        # Compute expectation values of sigma_z for each qubit
        n = self.n_qubits
        expvals = np.zeros(n)
        for bitstring, count in counts.items():
            for q, bit in enumerate(reversed(bitstring)):
                expvals[q] += (1 - 2*int(bit)) * count / self.shots
        
        return expvals
    
    def process_sequence(self, inputs: np.ndarray) -> np.ndarray:
        """Process a sequence of inputs, returning readout states."""
        states = [self.get_state(u) for u in inputs]
        return np.array(states)


# Demonstration
def qrc_demo():
    qrc = QuantumReservoir(n_qubits=4, shots=512)
    
    # NARMA-5 (short, for quick demo)
    np.random.seed(42)
    u = np.random.uniform(0, 0.5, 300)
    y = np.zeros(300)
    for t in range(5, 300):
        y[t] = 0.4*y[t-1] + 0.4*y[t-1]*y[t-5] + 0.6*u[t-5]**3 + 0.1
    
    print("Processing quantum reservoir (this takes a moment)...")
    states = qrc.process_sequence(u[:200])  # training
    
    washout = 20
    X_tr = states[washout:]
    y_tr = y[washout+1:201]
    
    # Ridge regression readout
    from sklearn.linear_model import Ridge
    clf = Ridge(alpha=1e-3).fit(X_tr, y_tr)
    
    states_te = qrc.process_sequence(u[200:])
    y_pred = clf.predict(states_te)
    nmse = np.mean((y[201:] - y_pred)**2) / np.var(y[201:])
    print(f"NARMA-5 NMSE (Quantum Reservoir, 4 qubits): {nmse:.4f}")
```

---

## D.5 MLflow for Experiment Tracking

MLflow provides experiment tracking, model versioning, and result comparison.

```bash
pip install mlflow
```

```python
import mlflow
import mlflow.sklearn
import numpy as np
from datetime import datetime

# Start MLflow tracking server (local):
# mlflow ui --port 5000
# Navigate to http://localhost:5000

mlflow.set_tracking_uri("mlruns")  # local directory
mlflow.set_experiment("reservoir_computing_benchmarks")


def run_esn_experiment(
    task: str = "NARMA10",
    n_reservoir: int = 500,
    spectral_radius: float = 0.9,
    leaking_rate: float = 0.3,
    ridge_alpha: float = 1e-4,
    seed: int = 42,
) -> dict:
    """Run one ESN experiment and log to MLflow."""
    
    run_name = f"{task}_N{n_reservoir}_rho{spectral_radius}_seed{seed}"
    
    with mlflow.start_run(run_name=run_name):
        # Log all parameters
        mlflow.log_params({
            "task": task,
            "n_reservoir": n_reservoir,
            "spectral_radius": spectral_radius,
            "leaking_rate": leaking_rate,
            "ridge_alpha": ridge_alpha,
            "seed": seed,
            "timestamp": datetime.now().isoformat(),
        })
        
        # Import and run task (placeholder for actual task code)
        rng = np.random.RandomState(seed)
        
        # Simulate results (replace with actual ESN code)
        nmse_train = rng.uniform(0.01, 0.05)
        nmse_test  = nmse_train + rng.uniform(0.0, 0.02)
        mc         = rng.uniform(8.0, 15.0)
        
        # Log metrics
        mlflow.log_metrics({
            "nmse_train": nmse_train,
            "nmse_test": nmse_test,
            "memory_capacity": mc,
        })
        
        results = {
            "nmse_train": nmse_train,
            "nmse_test": nmse_test,
            "memory_capacity": mc,
        }
    
    return results


# Run a grid search and log all results
grid = [
    {"spectral_radius": rho, "leaking_rate": lr}
    for rho in [0.7, 0.9, 0.95, 0.99]
    for lr in [0.1, 0.3, 0.7]
]

for config in grid:
    results = run_esn_experiment(
        task="NARMA10",
        n_reservoir=500,
        **config
    )

print("All runs logged. View results with: mlflow ui")
```

### Comparing Runs

```python
import mlflow

# Load all runs from an experiment
experiment = mlflow.get_experiment_by_name("reservoir_computing_benchmarks")
runs = mlflow.search_runs(
    experiment_ids=[experiment.experiment_id],
    order_by=["metrics.nmse_test ASC"],
)

# Show top 5 runs
print(runs[["params.spectral_radius", "params.leaking_rate",
            "metrics.nmse_train", "metrics.nmse_test"]].head(5))
```
