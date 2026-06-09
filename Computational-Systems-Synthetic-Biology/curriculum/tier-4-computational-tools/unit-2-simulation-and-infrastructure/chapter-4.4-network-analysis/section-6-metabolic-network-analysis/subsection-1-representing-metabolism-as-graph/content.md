# Representing Metabolism as a Graph

Cellular metabolism is, at one level, just chemistry: enzymes bind substrates, catalyze reactions, release products. But when you step back and look at all those reactions simultaneously — the full 10,000-reaction reconstruction of human metabolism — what you see is not just chemistry. You see structure. Glycolysis connects to the TCA cycle connects to the electron transport chain; branching pathways feed into and out of this central trunk. Some metabolites appear in hundreds of reactions; others in just one. Removing glucose from the medium does not affect histidine biosynthesis, but removing it does affect everything that requires NADH. Which reactions are central? Which metabolites link otherwise separate subsystems? These are topological questions, and answering them requires representing metabolism as a graph and applying the tools of network analysis — carefully.

Cellular metabolism is an interconnected web of enzymatic reactions transforming metabolites. Representing this system as a graph enables the application of network analysis tools — but the choice of representation fundamentally determines what can be analyzed. Metabolic networks have unique structural properties (bipartite reaction-metabolite structure, currency metabolites, stoichiometric constraints) that require careful treatment.

## The Stoichiometric Matrix

The foundational representation of a metabolic network is the **stoichiometric matrix** $S \in \mathbb{R}^{m \times n}$, where $m$ is the number of metabolites and $n$ is the number of reactions:

$$S_{ij} = \begin{cases} -\nu & \text{if metabolite } i \text{ is consumed by reaction } j \text{ with stoichiometry } \nu \\ +\nu & \text{if metabolite } i \text{ is produced by reaction } j \text{ with stoichiometry } \nu \\ 0 & \text{otherwise} \end{cases}$$

At steady state, the net rate of change of each metabolite is zero:

$$S \mathbf{v} = \mathbf{0}$$

where $\mathbf{v} \in \mathbb{R}^n$ is the vector of reaction fluxes. This constraint defines the **feasible flux space** — a convex polytope explored by FBA and flux sampling.

```python
import numpy as np
import cobra
import networkx as nx

# Load a metabolic model (SBML format)
model = cobra.io.read_sbml_model("iJO1366.xml")  # E. coli K-12

print(f"Metabolic model: {model.id}")
print(f"  Metabolites: {len(model.metabolites)}")
print(f"  Reactions:   {len(model.reactions)}")
print(f"  Genes:       {len(model.genes)}")

# Access the stoichiometric matrix
S = cobra.util.create_stoichiometric_matrix(model, array_type="dense")
print(f"\nStoichiometric matrix S: shape {S.shape}")
print(f"  Sparsity: {(S == 0).sum() / S.size * 100:.1f}% zeros")

# Example: inspect the glycolysis reaction PFK (phosphofructokinase)
pfk = model.reactions.get_by_id("PFK")
print(f"\nReaction: {pfk.id} — {pfk.name}")
print(f"  Equation: {pfk.reaction}")
print(f"  Metabolites and stoichiometries:")
for metabolite, coeff in pfk.metabolites.items():
    print(f"    {metabolite.id}: {coeff:+.1f}")
```

## Bipartite Graph Representation

The most structurally faithful graph representation of a metabolic network is a **bipartite graph** with two node types:
- **Metabolite nodes**: represent chemical species
- **Reaction nodes**: represent enzymatic transformations

Edges connect metabolites to reactions (substrate → reaction) and reactions to metabolites (reaction → product). This preserves stoichiometric information and avoids false edges.

```python
def build_bipartite_metabolic_graph(model):
    """
    Build a bipartite metabolite-reaction graph from a COBRA model.
    Metabolite nodes: prefix 'met_'
    Reaction nodes: prefix 'rxn_'
    """
    G = nx.DiGraph()

    # Add metabolite nodes
    for met in model.metabolites:
        G.add_node(f"met_{met.id}", type="metabolite",
                   name=met.name, formula=met.formula)

    # Add reaction nodes and edges
    for rxn in model.reactions:
        rxn_node = f"rxn_{rxn.id}"
        G.add_node(rxn_node, type="reaction", name=rxn.name,
                   subsystem=rxn.subsystem)

        for metabolite, coeff in rxn.metabolites.items():
            met_node = f"met_{metabolite.id}"
            if coeff < 0:  # substrate: metabolite → reaction
                G.add_edge(met_node, rxn_node, stoichiometry=abs(coeff))
            else:          # product: reaction → metabolite
                G.add_edge(rxn_node, met_node, stoichiometry=coeff)

    print(f"Bipartite metabolic graph:")
    print(f"  Total nodes: {G.number_of_nodes()}")
    print(f"  Metabolite nodes: {sum(1 for _, d in G.nodes(data=True) if d.get('type') == 'metabolite')}")
    print(f"  Reaction nodes: {sum(1 for _, d in G.nodes(data=True) if d.get('type') == 'reaction')}")
    print(f"  Edges: {G.number_of_edges()}")
    return G

G_bipartite = build_bipartite_metabolic_graph(model)
```

## Metabolite-Metabolite Graph and the Currency Problem

A simpler (but more problematic) representation connects two metabolites if they share a reaction. This allows standard graph algorithms to be applied directly but introduces a severe artifact: **currency metabolites**.

**Currency metabolites** (ATP, ADP, NADH, NAD+, H₂O, H+, CoA, Pi) participate in hundreds of reactions. In a metabolite-metabolite graph, they become super-hubs connecting all of metabolism into a single giant neighborhood — making path length analyses meaningless and centrality rankings dominated by artifact.

```python
# Known currency metabolites to exclude
CURRENCY_METABOLITES = {
    "atp_c", "adp_c", "amp_c", "pi_c", "ppi_c",
    "nadh_c", "nad_c", "nadph_c", "nadp_c",
    "h2o_c", "h_c", "co2_c",
    "coa_c", "accoa_c",
    "gtp_c", "gdp_c",
    "fad_c", "fadh2_c",
    "atp_m", "adp_m"  # also mitochondrial compartment versions
}

def build_metabolite_metabolite_graph(model, exclude_currency=True):
    """
    Build metabolite-metabolite graph (two metabolites connected if they share a reaction).
    Excluding currency metabolites is essential for meaningful topology.
    """
    G = nx.Graph()
    excluded = 0

    for rxn in model.reactions:
        substrates = [m for m, c in rxn.metabolites.items() if c < 0]
        products   = [m for m, c in rxn.metabolites.items() if c > 0]

        # Connect substrates to products
        all_mets = substrates + products
        if exclude_currency:
            all_mets = [m for m in all_mets if m.id not in CURRENCY_METABOLITES]
            excluded += len(substrates + products) - len(all_mets)

        for i, m1 in enumerate(all_mets):
            for m2 in all_mets[i+1:]:
                if G.has_edge(m1.id, m2.id):
                    G[m1.id][m2.id]["reactions"].add(rxn.id)
                else:
                    G.add_edge(m1.id, m2.id, reactions={rxn.id})

    print(f"Metabolite-metabolite graph:")
    print(f"  Nodes: {G.number_of_nodes()} (excluded {excluded} currency metabolite instances)")
    print(f"  Edges: {G.number_of_edges()}")

    # Top degree metabolites (potential remaining issues)
    top_degree = sorted(G.degree(), key=lambda x: -x[1])[:10]
    print(f"  Top 10 by degree:")
    for met_id, deg in top_degree[:5]:
        print(f"    {met_id}: {deg}")
    return G

G_met = build_metabolite_metabolite_graph(model)
```

## Pathway Subsystem Analysis

Metabolic reactions are organized into subsystems (glycolysis, TCA cycle, etc.) in COBRA models. Analyzing the network within and between subsystems reveals metabolic highway structure:

```python
def pathway_connectivity_analysis(model):
    """
    Analyze which metabolic subsystems are most interconnected.
    Returns a subsystem-level network.
    """
    from collections import Counter

    subsystem_counts = Counter(rxn.subsystem for rxn in model.reactions)
    print(f"Metabolic subsystems: {len(subsystem_counts)}")
    print("Top 10 by number of reactions:")
    for subsystem, count in subsystem_counts.most_common(10):
        print(f"  {subsystem}: {count} reactions")

    # Build subsystem-subsystem graph via shared metabolites
    G_subsys = nx.Graph()
    for subsystem in subsystem_counts:
        G_subsys.add_node(subsystem, n_reactions=subsystem_counts[subsystem])

    # Find metabolites shared between subsystems
    met_to_subsystems = {}
    for rxn in model.reactions:
        sub = rxn.subsystem
        if not sub:
            continue
        for met in rxn.metabolites:
            if met.id not in CURRENCY_METABOLITES:
                met_to_subsystems.setdefault(met.id, set()).add(sub)

    for met_id, subsystems in met_to_subsystems.items():
        subsystems = list(subsystems)
        for i, s1 in enumerate(subsystems):
            for s2 in subsystems[i+1:]:
                if G_subsys.has_edge(s1, s2):
                    G_subsys[s1][s2]["shared_mets"] += 1
                else:
                    G_subsys.add_edge(s1, s2, shared_mets=1)

    print(f"\nSubsystem-level network: {G_subsys.number_of_nodes()} nodes, "
          f"{G_subsys.number_of_edges()} edges")
    return G_subsys

G_subsys = pathway_connectivity_analysis(model)
```

## Why This Matters

The representation choice in metabolic network analysis is not a technicality — it determines whether the analysis is biologically interpretable. A metabolite-metabolite graph with currency metabolites included will always show that every metabolite is 2–3 steps from every other (because ATP connects everything), producing trivially short path lengths with no biological meaning. Excluding currency metabolites reveals the true carbon skeleton topology of metabolism: glycolysis, TCA cycle, and pentose phosphate pathway form a connected core, while fatty acid synthesis and amino acid biosynthesis are peripheral branches. This structure is directly visible in the graph and predicts which metabolic perturbations will have widespread effects (disrupting the core) vs. specific effects (disrupting a peripheral pathway). The bipartite representation is the mathematically correct foundation for constraint-based analyses like FBA and FEM.
