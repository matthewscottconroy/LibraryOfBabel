# How CELLO Works: Algorithm, Assignment, and Circuit Synthesis

The gap between "CELLO takes a Boolean specification and outputs DNA" and "here is how CELLO actually does that" is worth closing. Behind the elegant interface is a concrete algorithm, and understanding that algorithm reveals both why CELLO works as well as it does and why it fails when it does. The core problem CELLO solves is a **combinatorial assignment problem**: given a Boolean function decomposed into logic gates and a library of biological gates, find the assignment of biological gates to logical gates that maximizes circuit performance. Each step of the algorithm borrows from a different engineering discipline — VLSI chip design, operations research, bioinformatics — adapted to meet the specific constraints of biological parts.

## Step 1: Technology Mapping — Boolean Function to Gate Network

CELLO begins by **technology mapping**: converting the user-specified Boolean function into a network of elementary gates using only gate types available in the biological gate library. Since NOR gates are functionally complete, CELLO's default approach is to express the Boolean function entirely in terms of NOR gates (plus NOT gates for single inputs).

This is analogous to technology mapping in VLSI (Very Large Scale Integration) chip design, where circuit designers convert logic functions to networks of NAND/NOR gates available in a standard cell library. CELLO uses the same algorithms from digital circuit design, adapted for biological gates.

**Example**: 2-input AND function → NOR-only implementation:
```
AND(A, B) = NOR(NOT(A), NOT(B))  [De Morgan's theorem]
```
Using NOR gates:
```
NOT(A) = NOR(A, A)  [self-NOR is NOT]
NOT(B) = NOR(B, B)
AND(A,B) = NOR(NOR(A,A), NOR(B,B))
```
This uses 3 NOR gates (or 2 NOR + 1 final NOR), which is a valid CELLO-compatible implementation.

## Step 2: Gate Assignment — Biological Gates to Abstract Gates

With a network of abstract logic gates, CELLO must assign a specific biological gate from the library to each abstract gate. The constraints:

**Orthogonality constraint**: each biological gate uses a unique TF-promoter pair. No two gates in the same circuit can use the same TF (which would create cross-talk) or the same promoter (which would create combinatorial interactions not modeled as single-gate logic).

**Signal compatibility constraint**: the output signal range of gate $i$ must overlap with the input sensitivity range of any gate $j$ that gate $i$ drives. Specifically:
$$\text{Output}_{ON}(i) > K_{1/2}(j) > \text{Output}_{OFF}(i)$$

This ensures that when gate $i$ is in the ON state, it drives gate $j$ into the ON state; when gate $i$ is OFF, gate $j$ stays OFF.

**Fanout constraint**: a gate's output promoter can drive multiple downstream promoters, but each downstream gene is driven by only one output (no shared promoters in the default design).

CELLO performs this assignment using **simulated annealing** or other combinatorial optimization algorithms:
1. Start with a random assignment of biological gates to abstract gates
2. Evaluate circuit score (see section 8.3)
3. Propose a random swap (exchange two gate assignments)
4. Accept the swap if it improves the score, or occasionally accept it if it worsens the score (to escape local optima)
5. Repeat until convergence

For a 3-input circuit with 8 abstract gates and a 12-gate library, the search space is $12!/(12-8)! = 19,958,400$ possible assignments. Simulated annealing finds near-optimal solutions in this space in seconds.

## Step 3: Circuit Simulation — Predicting Performance

After gate assignment, CELLO simulates the full circuit for all $2^n$ input combinations:

For each input combination $(I_1, I_2, \ldots, I_n) \in \{0, 1\}^n$:
1. Convert binary inputs to concentrations: "0" → minimum inducer → minimum TF concentration; "1" → maximum inducer → maximum TF concentration
2. Propagate signals through the circuit: compute output of each gate using its Hill function transfer function, starting from inputs and working toward the output
3. Record predicted output fluorescence for each input combination

**Signal propagation algorithm**:
```python
def propagate_signals(circuit, input_values, gate_library):
    signal_levels = {input_name: val for input_name, val in input_values.items()}
    
    # Topological sort: compute each gate's output in order
    for gate in topological_sort(circuit):
        input_signal = signal_levels[gate.input_wire]
        tf_params = gate_library[gate.gate_type]
        
        # Compute TF concentration from input signal level
        tf_conc = tf_params['alpha'] / (1 + (input_signal / tf_params['K'])**tf_params['n'])
        
        signal_levels[gate.output_wire] = tf_conc
    
    return signal_levels[circuit.output_wire]
```

## Step 4: DNA Assembly — From Gate Assignment to Sequence

Once the optimal gate assignment is identified, CELLO generates the physical DNA design:

**Transcriptional unit structure** for each gate:
```
[Promoter (regulated by upstream TF)] → [Insulator (RiboJ)] → [RBS] → [TF CDS] → [Double Terminator]
```

**Assembly order**: gates are arranged on a single DNA construct (or split across multiple constructs if size exceeds practical limits). The arrangement follows:
- Input sensors (inducer-responsive TF expression cassettes)
- Internal gates (TF-to-TF signal transfer cassettes)  
- Output reporter (final promoter → reporter cassette)

**Sequence output format**: CELLO generates:
- Full DNA sequence (ready for synthesis)
- Annotated GenBank file with all part boundaries
- Circuit schematic as a visual diagram

## Gate Library Expansion: UCF Files

Gate libraries in CELLO are stored in **UCF (User Constraint Files)** — JSON-formatted files containing:
- Gate transfer functions (Hill function parameters)
- Signal levels (input/output ranges in REU)
- Part sequences (promoter, RBS, CDS sequences)
- Organism-specific information (chassis, growth conditions)

Multiple UCF files enable CELLO to target different organisms:
- `Eco1C1G1T1.UCF.json`: *E. coli* K-12 (original 2016 library, 12 gates)
- `Eco2C1G2T2.UCF.json`: *E. coli* (expanded library, ~20 gates)
- Yeast UCF files under development (fewer validated gates)

A major research activity in synthetic biology is expanding these UCF libraries by characterizing new TF-promoter pairs in standardized contexts.

## Known Limitations of the Algorithm

1. **Linear signal propagation assumption**: CELLO models each gate's output as depending on only one input signal (its immediately upstream gate's output TF). In reality, cells have retroactivity, shared resource pools, and cross-talk. These are not modeled.

2. **Steady-state assumption**: CELLO predicts steady-state behavior. Dynamic behavior (response time, noise filtering) is not predicted.

3. **Gate independence assumption**: each gate's transfer function was measured in isolation. When multiple gates are in the same cell, resource competition can shift individual gate parameters.

4. **No stochastic noise modeling**: CELLO predicts mean behavior. Cell-to-cell variability in gene expression can cause individual cells to output different logic values even when the mean prediction is correct.

These limitations explain why 15/60 circuits failed in the 2016 paper. Incorporating these effects into the model is an active area of research, with variants of CELLO that include resource competition and noise models in development.

## Why This Matters

CELLO's algorithmic approach — technology mapping, combinatorial assignment, simulation, sequence generation — is directly analogous to electronic CAD (Computer-Aided Design) tools that have made silicon chip design possible at scales of billions of transistors. The biology-specific adaptations (signal compatibility constraints, orthogonality constraints, UCF-based gate libraries) demonstrate that the gap between biological and digital circuit design is narrowing. The practical impact is already visible: circuits that would have taken expert teams months to design manually can now be designed in minutes by researchers with minimal circuit design background. As the gate libraries grow and the models improve, CELLO and its successors will make complex multi-gene circuit design routine.
