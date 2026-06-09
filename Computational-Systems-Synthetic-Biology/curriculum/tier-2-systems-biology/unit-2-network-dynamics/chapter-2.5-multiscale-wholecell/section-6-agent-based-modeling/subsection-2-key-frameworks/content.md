# Key Agent-Based Modeling Frameworks

## The Landscape of ABM Tools

You have decided an agent-based model is the right tool for your question. Now: which framework? The choice matters more than it might seem. Different frameworks embed different assumptions about how space works, how agents move, how they interact with their environment, and how their internal biology is represented. Picking the wrong one can mean spending months fighting the tool instead of doing biology.

The choice of ABM framework depends on the spatial model (discrete grid vs. continuous space), the complexity of agent behavior, available programming expertise, and the need for integration with ODE/PDE or metabolic models. The landscape of biological ABM tools spans from simple educational platforms to research-grade frameworks capable of multiscale integration.

## NetLogo

**NetLogo** (Wilensky 1999) is a patch-based, agent-based modeling environment designed for accessibility. It uses its own simple scripting language:

```netlogo
; Minimal bacterial growth model
turtles-own [energy]

to setup
  clear-all
  create-turtles 100 [
    setxy random-xcor random-ycor
    set energy 5
  ]
  reset-ticks
end

to go
  ask turtles [
    ; Move randomly
    right random 360
    forward 1
    ; Consume energy and eat local nutrient
    set energy energy - 0.1
    if pcolor = green [
      set energy energy + 2
      set pcolor black
    ]
    ; Divide if enough energy
    if energy > 10 [
      hatch 1 [set energy 5]
      set energy 5
    ]
    ; Die if no energy
    if energy <= 0 [die]
  ]
  tick
end
```

**Strengths**: extremely low barrier to entry; built-in visualization; massive model library (NetLogo Models Library with 500+ examples); excellent for teaching.

**Weaknesses**: scripting language not suitable for complex internal models; limited scalability (thousands of agents, not millions); single-threaded; poor integration with scientific Python stack.

**Best uses**: ecological models, epidemiological simulations, educational demonstrations, rapid prototyping of simple agent behaviors.

## Mesa (Python)

**Mesa** (Masad & Kazil 2015) is a Python framework for agent-based models:

```python
from mesa import Agent, Model
from mesa.space import MultiGrid
from mesa.time import RandomActivation
from mesa.datacollection import DataCollector
import numpy as np

class BacteriumAgent(Agent):
    def __init__(self, unique_id, model):
        super().__init__(unique_id, model)
        self.energy = 5.0
        self.growth_rate = 0.1  # h^-1
    
    def step(self):
        # Move to random adjacent cell
        possible_steps = self.model.grid.get_neighborhood(
            self.pos, moore=True, include_center=False)
        new_position = self.random.choice(possible_steps)
        self.model.grid.move_agent(self, new_position)
        
        # Consume nutrients
        nutrients = self.model.grid.get_cell_list_contents([self.pos])
        for nutrient in nutrients:
            if isinstance(nutrient, Nutrient):
                self.energy += nutrient.value
                self.model.grid.remove_agent(nutrient)
        
        # Metabolic cost
        self.energy -= 0.05
        
        # Divide
        if self.energy > 10:
            offspring = BacteriumAgent(self.model.next_id(), self.model)
            self.model.grid.place_agent(offspring, self.pos)
            self.model.schedule.add(offspring)
            self.energy /= 2
        
        # Die
        if self.energy <= 0:
            self.model.grid.remove_agent(self)
            self.model.schedule.remove(self)

class BacterialColonyModel(Model):
    def __init__(self, N, width, height):
        self.grid = MultiGrid(width, height, True)  # torus
        self.schedule = RandomActivation(self)
        
        # Initialize agents
        for i in range(N):
            agent = BacteriumAgent(i, self)
            x, y = self.random.randint(0, width-1), self.random.randint(0, height-1)
            self.grid.place_agent(agent, (x, y))
            self.schedule.add(agent)
        
        self.datacollector = DataCollector(
            model_reporters={'Population': lambda m: m.schedule.get_agent_count()},
            agent_reporters={'Energy': 'energy'}
        )
    
    def step(self):
        self.datacollector.collect(self)
        self.schedule.step()

# Run model
model = BacterialColonyModel(N=100, width=50, height=50)
for i in range(200):
    model.step()

data = model.datacollector.get_model_vars_dataframe()
print(data.tail())
```

**Strengths**: pure Python; integrates with NumPy, SciPy, pandas; supports grid and continuous space; good documentation; Solara-based visualization.

**Weaknesses**: not optimized for performance (slower than compiled languages); limited built-in support for PDE diffusion fields.

**Best uses**: custom biological models requiring integration with scientific Python; research-grade models that need flexible agent behavior.

## CompuCell3D

**CompuCell3D** (Swat et al. 2012) is the premier framework for **multiscale cellular models** that include cell shape dynamics (via the Cellular Potts Model, CPM), intracellular ODE/FBA models, and extracellular diffusion fields:

```python
# CompuCell3D simulation script (Python scripting interface)
from cc3d.core.PySteppables import *
from cc3d.CompuCellSetup import *

class BacteriumSteppable(SteppableBasePy):
    def __init__(self, frequency=1):
        SteppableBasePy.__init__(self, frequency)
    
    def start(self):
        # Access secretion and chemotaxis plugins
        self.secretor = self.get_field_secretor("Nutrient")
        
        for cell in self.cell_list:
            cell.targetVolume = 25
            cell.lambdaVolume = 2.0
            # Each cell runs its own ODE model
            cell.dict['metabolism'] = MetabolicModel()
    
    def step(self, mcs):
        for cell in self.cell_list:
            # Run intracellular metabolism model
            nutrients = self.secretor.amountSeenByCell(cell)
            growth = cell.dict['metabolism'].update(nutrients, dt=1)
            
            # Update cell volume target (growth)
            cell.targetVolume += growth * 0.1
            
            # Divide when large enough
            if cell.volume > 50:
                self.divide_cell_random_orientation(cell)
            
            # Consume nutrients
            self.secretor.secreteInsideCell(cell, -0.1 * cell.volume)
```

**Strengths**: physically realistic cell shape dynamics (CPM); supports ODE internal models per cell; PDE diffusion fields for extracellular signals; widely used for morphogenesis and cancer modeling.

**Weaknesses**: steep learning curve; GUI is sometimes unstable; Python scripting has some limitations.

**Best uses**: morphogenesis, cell sorting, tumor invasion, organoid formation models where cell shape and contact interactions matter.

## iDynoMiCS and BacSim

**iDynoMiCS** (Individual-based Dynamics of Microbial Communities in a Simulator; Lardon et al. 2011) is designed specifically for **biofilm and microbial community modeling**:

- Each bacterium is a sphere with explicit 3D position
- Biomass determines radius; bacteria grow, divide, and push neighbors
- Nutrient gradients computed as reaction-diffusion PDEs on a finite-difference grid
- Bacteria can have multiple "species" types with different metabolic strategies
- Supports aerobic, anaerobic, and microaerobic zones within the same biofilm

**Example application**: predicting the spatial distribution of nitrifiers and denitrifiers in a wastewater treatment biofilm, where oxygen penetration determines which metabolic strategy is locally advantageous.

## Choosing a Framework

| Need | Best framework |
|---|---|
| Teaching, rapid prototyping | NetLogo |
| Custom Python model with flexible behavior | Mesa |
| Cell shape, morphogenesis, contact-dependent signaling | CompuCell3D |
| Microbial biofilm, 3D community structure | iDynoMiCS |
| High-performance large-scale simulations | PhysiCell, FLAME GPU |
| Integration with FBA/metabolic models | CompuCell3D + Vivarium |

## Why This Matters

The diversity of ABM frameworks reflects the diversity of biological questions that require agent-based approaches. Choosing the right framework is the first step in any ABM project — the wrong choice can make implementation prohibitively difficult or produce qualitatively incorrect spatial dynamics. Understanding the design principles of each framework (grid vs. continuous space, CPM vs. force-based mechanics, built-in vs. custom diffusion) allows you to match the modeling tool to the biological question and the available computational resources.
