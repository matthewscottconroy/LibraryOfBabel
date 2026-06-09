# Global Optimization

Here is a frustrating truth about biological parameter estimation: there is usually more than one set of parameters that fits your data reasonably well. The objective function — the sum of squared residuals between your model's prediction and your experimental observations — has many valleys, not one. Different combinations of production rates, degradation rates, and Hill coefficients can produce trajectories that are nearly indistinguishable given the noise in your measurements. Run a local optimizer from one starting point and you find one valley. Run it from a different starting point and you find another. Which one is the biological truth? You cannot know unless you have explored the full landscape.

Local gradient-based methods converge to the nearest minimum, which may not be the global optimum. Biological optimization problems are notoriously multimodal: ODE parameter landscapes have many local minima with similar fit quality but very different biological interpretations; protein energy landscapes have thousands of local minima; combinatorial design spaces for synthetic gene circuits have discrete structure. **Global optimization** methods are designed to explore these landscapes broadly before exploiting promising regions.

## Simulated Annealing

**Simulated annealing (SA)** is a probabilistic global optimization algorithm inspired by the physical annealing process: a system is heated to high temperature (allowing large, random moves) and then slowly cooled, settling into a low-energy state.

The **Metropolis criterion** governs state transitions: a move from state $\theta$ to $\theta'$ with $\Delta f = f(\theta') - f(\theta)$ is:
- Always accepted if $\Delta f < 0$ (improvement)
- Accepted with probability $e^{-\Delta f / T}$ if $\Delta f > 0$ (uphill move)

As the temperature $T$ decreases ("cooling schedule"), uphill moves become increasingly rare, focusing the search near the current best solution.

```python
import numpy as np
from scipy.optimize import dual_annealing
from scipy.integrate import solve_ivp

# Multimodal test: fit a 4-parameter ODE to noisy data
# This objective has multiple local minima

def repressilator_rhs(t, u, params):
    alpha, alpha0, n, beta = params
    m1, m2, m3, p1, p2, p3 = u
    dm1 = -m1 + alpha / (1 + p3**n) + alpha0
    dm2 = -m2 + alpha / (1 + p1**n) + alpha0
    dm3 = -m3 + alpha / (1 + p2**n) + alpha0
    dp1 = -beta * (p1 - m1)
    dp2 = -beta * (p2 - m2)
    dp3 = -beta * (p3 - m3)
    return [dm1, dm2, dm3, dp1, dp2, dp3]

# Synthetic data
true_params = [100, 1e-4, 2.0, 1.0]
u0 = [0.1, 0.2, 0.3, 0.1, 0.2, 0.3]
t_data = np.linspace(10, 80, 30)

rng = np.random.default_rng(0)
sol_true = solve_ivp(lambda t, u: repressilator_rhs(t, u, true_params),
                     (0, 80), u0, t_eval=t_data, method='Radau',
                     rtol=1e-8, atol=1e-10)
data = sol_true.y[3:, :] + 2.0 * rng.standard_normal((3, 30))

def objective(params):
    alpha, alpha0, n, beta = params
    try:
        sol = solve_ivp(lambda t, u: repressilator_rhs(t, u, params),
                       (0, 80), u0, t_eval=t_data, method='Radau',
                       rtol=1e-5, atol=1e-7)
        if not sol.success:
            return 1e10
        return np.sum((sol.y[3:, :] - data)**2)
    except Exception:
        return 1e10

# Parameter bounds: [alpha, alpha0, n, beta]
bounds = [(10, 500), (1e-6, 1e-2), (1.0, 5.0), (0.1, 10.0)]

# Dual annealing: combines simulated annealing with local search
result_da = dual_annealing(
    objective,
    bounds,
    maxiter=1000,
    seed=42,
    minimizer_kwargs={'method': 'L-BFGS-B'}  # local polish
)

print(f"Dual annealing result:")
print(f"  alpha={result_da.x[0]:.1f}, alpha0={result_da.x[1]:.2e}, "
      f"n={result_da.x[2]:.2f}, beta={result_da.x[3]:.3f}")
print(f"  Objective: {result_da.fun:.2f}")
```

## Differential Evolution

**Differential evolution (DE)** maintains a population of $N_p$ candidate solutions. At each generation, each individual is updated by combining it with three randomly chosen population members:

1. **Mutation:** $v = x_{r1} + F(x_{r2} - x_{r3})$ where $F \in [0.5, 1.0]$ (differential weight)
2. **Crossover:** trial vector combines $v$ and the current individual with probability $CR$
3. **Selection:** keep whichever (trial or current) has lower objective value

DE is particularly robust for continuous parameter spaces with 10–100 dimensions and is the standard global optimizer for ODE parameter estimation:

```python
from scipy.optimize import differential_evolution

result_de = differential_evolution(
    objective,
    bounds,
    seed=42,
    maxiter=500,
    popsize=15,       # N_p = 15 * len(bounds) = 60 individuals
    mutation=(0.5, 1.0),   # F range
    recombination=0.7,     # CR
    tol=1e-6,
    workers=1,        # set workers=-1 for CPU-parallel evaluation
    polish=True       # final L-BFGS-B polish
)

print(f"\nDifferential evolution result:")
print(f"  alpha={result_de.x[0]:.1f}, n={result_de.x[2]:.2f}, beta={result_de.x[3]:.3f}")
print(f"  Objective: {result_de.fun:.2f}")
print(f"  Evaluations: {result_de.nfev}")
```

## Genetic Algorithms with DEAP

For combinatorial or mixed-integer optimization problems — such as designing synthetic promoter sequences or optimizing a gene circuit topology — **genetic algorithms (GAs)** operate on discrete or mixed representations using selection, crossover, and mutation operators:

```python
from deap import base, creator, tools, algorithms
import random

# Optimize a 6-element regulatory network topology
# Each gene can regulate each other: binary adjacency matrix
# Fitness: achieve oscillatory behavior with minimal connections

creator.create("FitnessMin", base.Fitness, weights=(-1.0,))
creator.create("Individual", list, fitness=creator.FitnessMin)

toolbox = base.Toolbox()
toolbox.register("attr_bit", random.randint, 0, 1)
toolbox.register("individual", tools.initRepeat, creator.Individual,
                 toolbox.attr_bit, n=36)  # 6x6 adjacency matrix
toolbox.register("population", tools.initRepeat, list, toolbox.individual)

def eval_circuit(individual):
    """Evaluate regulatory circuit topology: reward oscillations, penalize edges."""
    adjacency = np.array(individual).reshape(6, 6)
    n_edges = adjacency.sum()
    
    # Rough oscillation heuristic: need at least one odd-length negative cycle
    # (simplified; full evaluation would simulate the circuit ODE)
    period_score = detect_oscillation_potential(adjacency)
    
    return (n_edges - 10 * period_score,)  # minimize edges, maximize oscillation

toolbox.register("evaluate", eval_circuit)
toolbox.register("mate", tools.cxTwoPoint)
toolbox.register("mutate", tools.mutFlipBit, indpb=0.05)
toolbox.register("select", tools.selTournament, tournsize=3)

population = toolbox.population(n=100)
result, log = algorithms.eaSimple(population, toolbox, 
                                   cxpb=0.5, mutpb=0.2, ngen=50,
                                   verbose=True)
```

## Basin Hopping

**Basin hopping** alternates between random perturbations of the current solution and local minimization. It is particularly effective for molecular energy minimization (finding protein conformations) and for smooth biological objective functions with multiple basins:

```python
from scipy.optimize import basinhopping

# Random step: perturb parameters by ±30% log-normal
class BiologicalStep:
    def __init__(self, stepsize=0.3):
        self.stepsize = stepsize
    def __call__(self, x):
        x += self.stepsize * np.random.standard_normal(len(x))
        return x

result_bh = basinhopping(
    objective,
    x0=[90, 5e-5, 2.2, 0.9],
    minimizer_kwargs={'method': 'L-BFGS-B', 'bounds': bounds},
    niter=50,
    take_step=BiologicalStep(stepsize=0.5),
    seed=42
)
```

## Multi-Start Strategy

For practical biological parameter estimation, a simple but effective approach is **multi-start local optimization**: run L-BFGS-B from many random starting points and keep the best result:

```python
from scipy.optimize import minimize
import numpy as np

n_starts = 50
best_result = None

for i in range(n_starts):
    # Random start within bounds
    x0 = np.array([rng.uniform(lo, hi) for lo, hi in bounds])
    result = minimize(objective, x0, method='L-BFGS-B', bounds=bounds,
                     options={'maxiter': 200})
    if best_result is None or result.fun < best_result.fun:
        best_result = result

print(f"Best over {n_starts} starts: {best_result.fun:.2f}")
```

## Why This Matters

Parameter estimation in systems biology is inherently a global optimization problem: ODE landscapes are multimodal, and local methods starting from a single point find only one basin. Differential evolution, dual annealing, and basin hopping explore the full feasible space, providing confidence that the fitted parameters are globally optimal rather than trapped in a local minimum. This matters enormously when using fitted parameters to make predictions about biological behavior or drug effects.
