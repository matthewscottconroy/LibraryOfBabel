# Vivarium: Compositional Multiscale Modeling

## The Problem Vivarium Solves

If you wanted to build a whole-cell model from scratch — to integrate metabolism, gene expression, DNA replication, and cell division into a single simulation — how would you do it? The traditional answer is: write custom code. Define a simulation loop, hand-craft the communication between submodels, build ad hoc structures for sharing state. The Karr 2012 model took this approach, and it worked — but it required 128 person-years and produced a codebase that is difficult to extend, nearly impossible to reuse, and effectively incompatible with models from other groups.

Building a multiscale model like the Karr whole-cell model requires integrating dozens of individual model components into a coherent simulation. The traditional approach is to write custom integration code for each project: defining the communication protocol between submodels, managing shared state, handling time synchronization. This custom code is:
- Difficult to reuse across projects
- Hard to test and validate independently
- Not amenable to incremental model building (adding one new submodel requires modifying all integration code)
- Incompatible with models built by other groups

**Vivarium** (Agmon et al. 2022, *eLife*) is a software framework that solves these problems through a principled compositional architecture: processes are Python classes with defined interfaces; they are connected through a shared store; new processes can be added without modifying existing code.

## Core Concepts

### Process

A **Process** is the fundamental computational unit in Vivarium. It represents any dynamical model — ODE, stochastic, FBA, rule-based, or any other framework. A Process has:

- **Ports**: named input/output variables (e.g., `inputs: {metabolites: {glucose: float}}`, `outputs: {metabolites: {glucose: float}, fluxes: {EX_glc: float}}`)
- **next_update(timestep, states)** method: takes current state and timestep, returns the update to apply
- No knowledge of other processes — only sees its own ports

```python
from vivarium.core.process import Process
from vivarium.core.types import Schema

class SimpleMichaelisMenten(Process):
    """A single enzyme reaction as a Vivarium Process."""
    
    defaults = {
        'Vmax': 1.0,  # mmol/gDW/h
        'Km': 0.1,    # mM
    }
    
    def ports_schema(self):
        return {
            'internal': {
                'substrate': {
                    '_default': 1.0,
                    '_units': 'mM',
                    '_updater': 'accumulate'
                },
                'product': {
                    '_default': 0.0,
                    '_units': 'mM',
                    '_updater': 'accumulate'
                }
            }
        }
    
    def next_update(self, timestep, states):
        S = states['internal']['substrate']
        v = self.parameters['Vmax'] * S / (self.parameters['Km'] + S)
        dS = -v * timestep
        dP = v * timestep
        return {
            'internal': {
                'substrate': dS,
                'product': dP
            }
        }
```

### Store

A **Store** is a hierarchical dictionary of named state variables (the "blackboard" shared by all processes). Stores can be nested:

```python
initial_state = {
    'internal': {
        'metabolites': {
            'glucose': 10.0,   # mM
            'pyruvate': 0.1,   # mM
            'atp': 3.0         # mM
        }
    },
    'external': {
        'glucose': 100.0,  # mM (medium)
        'oxygen': 0.21     # mM
    },
    'chromosome': {
        'genes': {'dnaA': 1, 'rpoB': 1},  # copy numbers
    }
}
```

### Composite

A **Composite** assembles multiple processes and a store into a simulation:

```python
from vivarium.core.composition import Composite, simulate_composite

# Define a cell as a composition of processes
cell_composite = Composite({
    'processes': {
        'metabolism': FBAProcess(parameters={'model': 'iJO1366'}),
        'transcription': StochasticTranscription(parameters={...}),
        'translation': TranslationProcess(parameters={...}),
        'gene_regulation': BooleanGRN(parameters={...})
    },
    'topology': {
        'metabolism': {
            'internal_metabolites': ('internal', 'metabolites'),
            'external_metabolites': ('external',),
            'gene_expression': ('gene_expression',)
        },
        'transcription': {
            'metabolites': ('internal', 'metabolites'),
            'mrna_counts': ('gene_expression', 'mrna')
        },
        # ... wiring for other processes
    }
})
```

### Wiring (Topology)

The **topology** defines how each process's ports connect to the shared store. This is the key innovation: processes do not directly communicate — they only read from and write to named locations in the store. The topology is specified separately from the processes themselves, allowing the same process to be wired differently in different composites.

## Hierarchical Composition

Processes can be nested: a Composite can itself act as a Process from the perspective of a higher-level simulation. This enables **multi-scale composition**:

```python
# A cell (Composite of molecular processes) acts as a Process
# in a colony simulation (where cells are agents)

class CellProcess(Process):
    """Wraps the cell composite as a single process for colony simulation."""
    def __init__(self, parameters):
        self.internal_cell = cell_composite  # the whole-cell model
        super().__init__(parameters)
    
    def next_update(self, timestep, states):
        # Run internal cell model for timestep
        return self.internal_cell.update(timestep, states['cell_state'])

# Colony simulation: multiple cells as agents
colony = Composite({
    'processes': {
        'cell_1': CellProcess(parameters={'position': (0, 0)}),
        'cell_2': CellProcess(parameters={'position': (5, 0)}),
        'diffusion': NutrientDiffusion(parameters={'grid': 100}),
    },
    'topology': {
        'cell_1': {
            'nutrients': ('environment', 'glucose'),
            'cell_state': ('cells', 'cell_1', 'state')
        },
        # ... 
    }
})
```

## Running a Vivarium Simulation

```python
from vivarium.core.engine import Engine

# Create simulation engine
engine = Engine(
    processes=cell_composite.processes,
    topology=cell_composite.topology,
    initial_state=initial_state
)

# Run for specified duration
output = engine.run(duration=3600)  # 1 hour in seconds

# Extract trajectories
import matplotlib.pyplot as plt
t = output['time']
glucose = [state['internal']['metabolites']['glucose'] 
           for state in output['states']]
plt.plot(t, glucose)
plt.xlabel('Time (s)')
plt.ylabel('[Glucose] (mM)')
```

## Vivarium in Practice: Published Applications

The Vivarium framework has been used to build:
- **E. coli colony models**: combining FBA metabolism with stochastic gene expression and spatial diffusion of nutrients
- **Coupled metabolism-chromosome replication**: timescale-separated coupling of metabolic FBA with DNA replication kinetics
- **Biofilm models**: individual bacteria as Vivarium agents in a diffusion environment

## Advantages Over Custom Integration Code

| Feature | Custom code | Vivarium |
|---|---|---|
| Reuse of components | Difficult (tight coupling) | Easy (standardized ports) |
| Adding new processes | Requires modifying all integration code | Add process + wiring only |
| Testing individual processes | Requires full model | Each process testable independently |
| Sharing models | Non-standard, incompatible | Standardized format |
| Documentation | Project-specific | Built into port schema |

## Why This Matters

Vivarium represents a paradigm shift from monolithic whole-cell models (like Karr 2012, with its custom integration code) to modular, composable simulation. This shift enables:
- Incremental construction of complex models: start with a simple cell, add processes one by one
- Community-contributed process libraries: reuse validated process implementations
- Rapid prototyping of multiscale hypotheses: wire existing processes in new configurations
- Reproducibility: standardized format allows exact replication of published models

As biological data accumulates and model complexity grows, compositional frameworks like Vivarium become essential infrastructure — the cell biology equivalent of version control and packaging systems in software development.
