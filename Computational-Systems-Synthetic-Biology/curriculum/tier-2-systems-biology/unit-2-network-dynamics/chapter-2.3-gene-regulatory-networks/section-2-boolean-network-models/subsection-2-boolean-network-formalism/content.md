# Boolean Network Formalism

## State Space and Update Rules

When Stuart Kauffman first wrote down Boolean networks in 1969, he was asking a simple but audacious question: if you model every gene as either ON or OFF and specify the logical rules by which genes influence each other, what kinds of dynamics emerge? The answer — that such networks spontaneously organize into a small number of stable patterns — laid the theoretical groundwork for understanding cell type diversity. Before we get to that result, we need to understand the machinery.

A Boolean network consists of $n$ binary variables (genes), each taking a value in $\{0, 1\}$:

$$\mathbf{g}(t) = (g_1(t), g_2(t), \ldots, g_n(t)) \in \{0, 1\}^n$$

The total state space has $2^n$ states. For a network of 10 genes, this is $1024$ states; for 50 genes, $2^{50} \approx 10^{15}$ — computationally intractable for exhaustive enumeration without special algorithms.

Each gene's state at time $t+1$ is determined by a Boolean **update function** $F_i$:

$$g_i(t+1) = F_i(g_{j_1}(t), g_{j_2}(t), \ldots, g_{j_k}(t))$$

where $\{j_1, \ldots, j_k\}$ are the regulators of gene $i$. The function $F_i$ is any Boolean function of its inputs — AND, OR, NOT, and their combinations.

## Constructing Update Rules from Biology

The power of Boolean networks comes from the directness of the translation from biology to mathematics. Experimental evidence about regulatory interactions maps almost immediately onto logical rules:

Update rules are derived from experimental evidence:
- If TF A activates gene B and TF C also activates gene B independently: $F_B = A \text{ OR } C$
- If gene B requires both A and C simultaneously: $F_B = A \text{ AND } C$
- If gene B is repressed by A: $F_B = \text{NOT } A$
- If gene B is activated by A but repressed by C: $F_B = A \text{ AND NOT } C$

**Example — T cell activation (simplified):**

```
TCR signal (S): external input (1 if present)
CD28: co-stimulatory signal (external input)
ZAP70: ZAP70(t+1) = S AND NOT LCK_inhibitor
NFAT: NFAT(t+1) = Calcineurin AND ZAP70
AP1: AP1(t+1) = ZAP70 AND RAS
IL2: IL2(t+1) = NFAT AND AP1 AND NOT CTLA4
```

Each rule encodes a logical gate derived from signaling pathway knowledge. If you know the signaling pathway, you can write the Boolean rules. No kinetic constants required.

## Synchronous vs. Asynchronous Update

This is not a technical detail to skim past — the choice of update scheme fundamentally determines which attractors you find, and therefore which biological conclusions you draw.

**Synchronous update**: all genes update simultaneously at each discrete time step. Simple to implement; may produce artificial oscillatory attractors due to synchrony artifacts.

$$\mathbf{g}(t+1) = \mathbf{F}(\mathbf{g}(t)) = (F_1(\mathbf{g}(t)), \ldots, F_n(\mathbf{g}(t)))$$

**Asynchronous update**: one randomly chosen gene updates at each time step. Better biological fidelity — genes in cells do not coordinate their updates. Produces a set of possible trajectories rather than a single deterministic path.

**Priority class (general asynchronous)**: genes are assigned to priority classes and update according to a specified ordering. Allows modeling of known temporal hierarchies (e.g., fast signaling → slow transcription).

```python
import itertools
import random

def synchronous_step(state, update_funcs):
    """One synchronous update step."""
    return tuple(f(state) for f in update_funcs)

def async_step(state, update_funcs):
    """One asynchronous update step (random gene)."""
    state = list(state)
    i = random.randint(0, len(state)-1)
    state[i] = update_funcs[i](tuple(state))
    return tuple(state)

# Example: toggle switch (two mutually repressing genes)
def F_A(state):
    A, B = state
    return int(not B)  # A = NOT B

def F_B(state):
    A, B = state
    return int(not A)  # B = NOT A

update_funcs = [F_A, F_B]
# Starting from A=1, B=1 (unstable): synchronous leads to oscillation
state = (1, 1)
trajectory = [state]
for _ in range(10):
    state = synchronous_step(state, update_funcs)
    trajectory.append(state)
print("Synchronous:", trajectory)
# Oscillates: (1,1)→(0,0)→(1,1)→... (synchrony artifact)

# Asynchronous avoids this artifact
state = (1, 1)
for _ in range(10):
    state = async_step(state, update_funcs)
print("Final async state:", state)
# Settles to either (1,0) or (0,1) — the true attractors
```

The (1,1) → (0,0) → (1,1) oscillation under synchronous update is a mathematical artifact: in biology, both genes of a toggle switch do not flip simultaneously. Under asynchronous update, one gene flips first, breaking the symmetry and allowing the system to settle into one of the two genuine stable states.

## State Transition Graphs

For a small network, the **state transition graph (STG)** provides a complete picture of the dynamics. Each node is a network state; each directed edge shows which state follows. The STG has $2^n$ nodes and one outgoing edge per node (for synchronous update).

For $n=3$ (8 states), the STG can be drawn and inspected manually. For $n \leq 20$, computational tools can enumerate all attractors in the STG. For $n > 20$, specialized algorithms (SAT solvers, symbolic model checking) are needed.

## Basins of Attraction

Every state eventually leads to an attractor. The **basin of attraction** of an attractor is the set of all states that eventually converge to it. Basin sizes reveal the robustness of each cell state:

- Large basin → attractor is easily reached; the corresponding cell state is default
- Small basin → attractor requires specific initial conditions; rare cell state

```python
def find_attractor(initial_state, update_funcs, max_steps=1000):
    """Follow synchronous trajectory to find attractor."""
    seen = {}
    state = initial_state
    for step in range(max_steps):
        if state in seen:
            # Found cycle
            cycle_start = seen[state]
            return list(seen.keys())[cycle_start:]
        seen[state] = step
        state = synchronous_step(state, update_funcs)
    return None  # no attractor found within max_steps

# Enumerate all states and their attractors
n = 3
all_states = list(itertools.product([0,1], repeat=n))
attractor_map = {}
for s in all_states:
    attr = find_attractor(s, update_funcs)
    attractor_map[s] = tuple(attr[0]) if attr else None
```

## Perturbation Analysis

One of the most valuable uses of Boolean models is systematic **perturbation analysis**: simulating gene knockouts (fix $g_i = 0$) or constitutive activation (fix $g_i = 1$) and observing how attractors change.

If knocking out gene A eliminates the erythroid attractor but not the myeloid attractor, this predicts that A is essential for erythropoiesis — a testable experimental prediction. This kind of in silico genetic screen can guide experimental prioritization: instead of testing 50 candidate TFs in the lab, test the 5 that the Boolean model predicts to be essential.

## Why This Matters

The Boolean network formalism provides a concrete, computationally tractable language for encoding regulatory knowledge and extracting dynamical predictions. The distinction between synchronous and asynchronous update is not a technical detail — it fundamentally affects which attractors are found. Understanding this formalism, including its limitations, is prerequisite to using Boolean models responsibly for cell fate analysis and synthetic circuit design.
