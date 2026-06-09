# Building the Stoichiometric Matrix

## From Reactions to Matrix

There is something quietly remarkable about the fact that the entire metabolic connectivity of a cell — thousands of reactions, hundreds of metabolites, cofactors shuttling between pathways — can be captured in a single matrix. That matrix is $\mathbf{S}$, the **stoichiometric matrix**, and it is the central data structure of all constraint-based metabolic modeling. Before you optimize, simulate, or engineer, you build $\mathbf{S}$. Understanding what it encodes and why it is constructed the way it is will pay dividends through every subsequent analysis.

Metabolism is a network of chemical reactions, and $\mathbf{S}$ is the compact mathematical representation of that network's connectivity. It encodes which metabolites participate in which reactions and in what quantities.

For a network with $m$ metabolites and $n$ reactions, $\mathbf{S}$ is an $m \times n$ matrix where:

$$S_{ij} = \begin{cases} +|\nu_{ij}| & \text{if metabolite } i \text{ is produced by reaction } j \\ -|\nu_{ij}| & \text{if metabolite } i \text{ is consumed by reaction } j \\ 0 & \text{if metabolite } i \text{ does not participate in reaction } j \end{cases}$$

The sign convention follows the chemist's standard: products are positive, reactants are negative.

## Worked Example: A Three-Reaction Network

Consider a small metabolic network with five metabolites (A, B, C, D, E) and four reactions:

| Reaction | Equation | Interpretation |
|---|---|---|
| $r_1$ | $A \rightarrow B$ | Isomerization |
| $r_2$ | $B + C \rightarrow D$ | Condensation |
| $r_3$ | $D \rightarrow E + C$ | Lyase (C recycled) |
| $r_4$ | $A \rightarrow \emptyset$ | Exchange (uptake of A) |
| $r_5$ | $E \rightarrow \emptyset$ | Exchange (secretion of E) |

The stoichiometric matrix (rows = metabolites, columns = reactions):

$$\mathbf{S} = \begin{array}{c} A \\ B \\ C \\ D \\ E \end{array} \begin{pmatrix} -1 & 0 & 0 & -1 & 0 \\ +1 & -1 & 0 & 0 & 0 \\ 0 & -1 & +1 & 0 & 0 \\ 0 & +1 & -1 & 0 & 0 \\ 0 & 0 & +1 & 0 & -1 \end{pmatrix} \begin{array}{c} \leftarrow r_1 \\ \leftarrow r_2 \\ \leftarrow r_3 \\ \leftarrow r_4 \\ \leftarrow r_5 \end{array}$$

Check the C row: $-1$ from $r_2$ (consumed), $+1$ from $r_3$ (produced). Carbon C is recycled within the network — it appears as neither a net input nor a net output. This is exactly what the matrix reveals: C is an internal cofactor-like species, perpetually consumed and regenerated. You can read the metabolic role of any species directly from its row.

## Building $\mathbf{S}$ in Python

```python
import numpy as np
import cobra

# Manual construction for small networks
# Rows: A, B, C, D, E
# Cols: r1, r2, r3, r4 (exchange A), r5 (exchange E)

S = np.array([
    [-1,  0,  0, -1,  0],   # A
    [+1, -1,  0,  0,  0],   # B
    [ 0, -1, +1,  0,  0],   # C
    [ 0, +1, -1,  0,  0],   # D
    [ 0,  0, +1,  0, -1],   # E
], dtype=float)

print("S matrix shape:", S.shape)
print("Rank of S:", np.linalg.matrix_rank(S))

# Atom/charge balance check: mass × S should equal zero per column
# (requires molecular weights for each metabolite)
mol_weights = np.array([180, 180, 60, 240, 120])  # example g/mol
mass_balance = mol_weights @ S
print("Mass balance check (should be ~0 for balanced reactions):")
for j, mb in enumerate(mass_balance):
    print(f"  r{j+1}: {mb:.1f}")
```

## Compartmentalization

Real cells are not homogeneous bags of enzymes. A mammalian cell runs glycolysis in the cytoplasm, oxidizes pyruvate in the mitochondrial matrix, and performs fatty acid synthesis back in the cytoplasm — the same chemical species inhabiting different compartments, participating in different reactions, connected by transporters. This spatial organization matters enormously for metabolism.

Real metabolic networks include multiple compartments (cytoplasm, mitochondrial matrix, ER, peroxisome). Metabolites are distinguished by compartment label, and **transport reactions** carry them across membranes:

$$\text{ATP}_\text{cytosol} \rightarrow \text{ATP}_\text{mitochondria}$$

In a GEM for a mammalian cell, the same chemical entity (e.g., ATP) appears as separate rows for each compartment in which it occurs. The stoichiometric matrix becomes larger but retains the same structure.

Exchange reactions ($A \rightarrow \emptyset$ or $\emptyset \rightarrow A$) represent the boundary between the cell and its environment. They are included in $\mathbf{S}$ with a $-1$ or $+1$ in the metabolite row, allowing boundary fluxes to be constrained.

## Checking Stoichiometric Balance

A fundamental requirement: every reaction must be **mass-balanced** (atoms conserved) and **charge-balanced** (electrons conserved). Unbalanced reactions indicate:
- Missing cofactors (e.g., ATP, NADH that are produced/consumed but not shown)
- Implicit assumptions about proton stoichiometry
- Errors in the network reconstruction

In practice, checking balance requires knowing the molecular formula and charge of each metabolite — stored in the GEM as metadata. COBRApy provides:

```python
import cobra

model = cobra.io.read_sbml_model('iJO1366.xml')

# Check all reactions for balance
unbalanced = []
for rxn in model.reactions:
    if not rxn.check_mass_balance():
        unbalanced.append(rxn.id)

print(f"{len(unbalanced)} unbalanced reactions found")
# In well-curated GEMs, this should be 0 for internal reactions
```

## GPR Associations

The stoichiometric matrix tells you what reactions occur. But which genes make those reactions possible? In genome-scale models, each reaction $j$ is associated with the gene(s) encoding the enzyme(s) that catalyze it via **Gene-Protein-Reaction (GPR) associations**:

- **Single enzyme**: reaction requires gene $g_1$ (AND logic)
- **Isozymes**: reaction can be catalyzed by enzyme A OR enzyme B (OR logic)
- **Multi-subunit complex**: reaction requires subunit $g_1$ AND subunit $g_2$

These Boolean expressions allow simulation of gene knockouts: if a gene is knocked out, all reactions whose GPR cannot be satisfied by remaining genes are set to zero flux. This is the bridge between genotype and phenotype in constraint-based modeling.

## Why This Matters

The stoichiometric matrix is the foundational data structure of constraint-based metabolic modeling. Every analysis in this chapter — flux balance analysis, flux variability, $^{13}$C metabolic flux analysis — operates on $\mathbf{S}$. Its construction requires careful biological knowledge: which reactions occur in which compartments, what the cofactor stoichiometries are, which genes encode which enzymes. Getting this right is the critical first step; all subsequent analysis is only as valid as the accuracy of $\mathbf{S}$.
