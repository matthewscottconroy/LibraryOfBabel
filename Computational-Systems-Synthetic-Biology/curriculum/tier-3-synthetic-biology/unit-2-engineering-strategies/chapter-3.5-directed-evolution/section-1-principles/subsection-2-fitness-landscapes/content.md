# Fitness Landscapes

Here is a thought experiment. You are trying to improve an enzyme, and you have just found a mutation — call it A — that improves activity by 50%. Should you now search for a second mutation that stacks on top of A? The intuitive answer is yes. But what if another mutation B, measured in isolation, also improves activity by 50%, yet when you combine A and B the enzyme is actually worse than either alone? This is not a hypothetical curiosity — it happens regularly in directed evolution, and it is the reason why protein engineering cannot simply be reduced to finding the best individual mutations and piling them together. The fitness landscape framework gives you the conceptual vocabulary to think about exactly these situations, and it turns out to be indispensable for choosing the right experimental strategy.

The **fitness landscape** is the central conceptual framework for understanding protein evolution. It maps the relationship between protein sequence (genotype) and protein function (phenotype, or fitness). Understanding the topography of fitness landscapes — how rugged they are, how many peaks they have, how accessible different regions are — directly informs the design of directed evolution experiments.

## The Concept

**Sewall Wright (1932)** introduced the fitness landscape metaphor for organismal populations. Applied to proteins, it works as follows:

- **Sequence space**: the set of all possible amino acid sequences of length $L$ is a hypercube with $20^L$ vertices. For a 100 aa protein, this is $20^{100} \approx 10^{130}$ sequences.
- **Fitness**: a scalar value assigned to each sequence (kcat, Tm, binding affinity, activity in a specific assay, or any measurable property)
- **Landscape**: the mapping of fitness values onto sequence space

Two sequences are **neighbors** in sequence space if they differ by exactly one amino acid substitution. The fitness landscape defines a "topography" where adjacent sequences may have similar or very different fitness values.

## Landscape Properties Relevant to Directed Evolution

### Ruggedness and Local Optima

A **rugged** landscape has many peaks (local optima) separated by valleys. A sequence at a local optimum is surrounded by neighbors with lower fitness — sequential mutation cannot move the sequence to higher fitness without crossing a valley.

**NK model** (Kauffman 1989): parameterizes landscape ruggedness by $K$, the number of epistatic interactions per position:
- $K = 0$: all positions contribute independently → smooth landscape (single peak)
- $K = N-1$: every position interacts with every other → maximally rugged (exponential number of local optima)
- Empirical protein landscapes: intermediate $K$; ruggedness depends on the property being evolved

**Implication for directed evolution**: in a rugged landscape, random walks (epPCR rounds) often find different local optima depending on the starting sequence and mutation order. Two experiments with the same starting protein may reach very different final sequences — both are local optima but neither is the global optimum.

### Peaks in Sequence Space

Empirical landscape measurements (described below) show that:
- Most proteins occupy sequences in a narrow region of sequence space (proteins with similar function are clustered)
- Multiple peaks exist within accessible sequence space
- Global peak may be far from any known sequence

### Neutral Networks and Neutral Drift

Not all sequence changes affect fitness significantly. A **neutral network** is the set of sequences with approximately equal fitness connected by single mutations. Neutral evolution — moving along neutral networks without selection — can access previously inaccessible regions of sequence space where beneficial mutations are available.

In directed evolution: **neutral drift** (evolution at low selection stringency) can serve as a pre-adaptation strategy to explore sequence space before applying stringent selection.

## Empirically Measured Fitness Landscapes

Several landmark studies have directly measured large fractions of sequence space for specific proteins:

### GB1 Protein: 4-Point Combinatorial Landscape (Wu et al. 2016)

Wu, Bhaskara, Bhaskara, and Bhaskara (2016) measured the fitness of all $20^4 = 160,000$ possible amino acid combinations at four critical positions in the GB1 immunoglobulin-binding domain.

**Key findings**:
- Only ~16% of all 160,000 combinations had fitness above the wild-type
- Epistasis was widespread: most beneficial mutations were not independently beneficial (sign epistasis)
- The fitness landscape was highly rugged: sequential mutation rarely found the global optimum
- Recombination between high-fitness sequences was more effective than sequential mutation for traversing the landscape

This is the benchmark dataset used by virtually all machine learning-guided directed evolution papers.

### GFP Sequential Evolution (Sarkisyan et al. 2016)

Sarkisyan et al. measured fluorescence for ~51,000 GFP variants along all single mutant paths from a low-activity ancestor to a high-activity variant. They found:

- Many mutational trajectories lead to non-fluorescent local optima
- Epistasis between mutations was common: mutation A was beneficial only after B had been introduced
- The actual evolutionary path taken depended strongly on the order mutations were introduced

This directly demonstrated how order-of-mutation effects (historical contingency) shape evolutionary outcomes.

## Epistasis: The Key Driver of Landscape Ruggedness

**Epistasis** is the deviation of a multi-mutation combination's fitness from the product of the individual mutation effects:

$$\text{Multiplicative expectation: } w_{AB} = w_A \times w_B$$
$$\text{Epistasis: } \epsilon = w_{AB} - w_A \times w_B$$

Types:
- **Magnitude epistasis** ($\epsilon \neq 0$ but same sign): mutations interact but both are beneficial or both deleterious in combination
- **Sign epistasis** ($\epsilon$ is large enough that one mutation is beneficial alone but deleterious in combination with another)
- **Reciprocal sign epistasis**: A is beneficial, B is beneficial, but AB is deleterious; or A is neutral, B is neutral, but AB is beneficial

Reciprocal sign epistasis creates fitness valleys between peaks — the landscape is rugged enough that sequential mutation cannot easily traverse from one peak to another.

## Implications for Directed Evolution Strategy

**Low landscape ruggedness** (few epistatic interactions):
- Random mutagenesis (epPCR) + selection is sufficient
- Each round finds additively beneficial mutations
- Landscape topology allows sequential optimization

**High landscape ruggedness** (many epistatic interactions):
- Sequential mutagenesis (epPCR) gets trapped at local optima
- Need recombination to jump between peaks (DNA shuffling)
- Machine learning can model epistatic interactions and propose sequences at distant peaks

**Unknown ruggedness** (typical starting situation):
- Start with epPCR for 3–5 rounds → if progress stalls, switch to recombination or ML-guided strategies

## Visualizing the Landscape

Since sequence space is $10^{130}$-dimensional, direct visualization is impossible. Several dimensionality reduction approaches are used:

**Principal component analysis (PCA)**: project sequence-fitness data onto 2D plane using first two principal components of variation. Shows structure of the explored sequence space.

**t-SNE / UMAP**: non-linear dimensionality reduction; better at preserving local neighborhood structure. Used to visualize ML-learned embeddings of protein sequence fitness landscapes.

**Correlation of fitness**: plot fitness of sequences vs. their Hamming distance from a reference sequence. A high correlation at short distances indicates a smooth local landscape; rapid decorrelation indicates ruggedness.

## Why This Matters

The fitness landscape framework explains both the successes and failures of directed evolution experiments. Why does sequential mutagenesis plateau after 3–5 rounds? Because the evolving sequence has reached a local optimum on a rugged landscape. Why does recombination between two moderately improved variants sometimes produce a dramatically better variant? Because recombination can combine beneficial mutations that are in different sequence regions and that could not be accumulated one at a time due to sign epistasis. Why does machine learning improve on random mutagenesis? Because ML models can learn epistatic interactions from training data and propose sequences that are at peaks of the landscape not accessible by local search. The fitness landscape is not just a metaphor — it is a quantitative description of the optimization problem that directed evolution is solving, and understanding its properties is essential for choosing the right experimental strategy.
