# 13.3.1 Graph ESNs: Reservoirs on Non-Euclidean Data

## Motivation: When Inputs Have Structure

Standard ESNs assume the input $\mathbf{u}_t \in \mathbb{R}^d$ is a vector of real numbers arriving at each time step. This covers many important cases: univariate and multivariate time series, sensor streams, audio signals. But a growing class of important problems has inputs that are not vectors but graphs: molecules (atoms connected by bonds), social networks (people connected by relationships), power grids, biological neural circuits, traffic networks.

A molecule like benzene ($\text{C}_6\text{H}_6$) is not well represented as a fixed-length vector. A social network at time $t$ might have a changing number of nodes. The adjacency structure — which node is connected to which — is at least as informative as the node features. Standard ESNs, which ignore adjacency, lose this structural information.

**Graph ESNs** [Gallicchio2010, Gallicchio2020] address this by building reservoirs that operate on graphs natively, propagating information along edges as part of the state update.

## Graph Notation

A graph at time $t$ is $G_t = (V_t, E_t, \mathbf{X}_t)$ where:
- $V_t = \{1, 2, \ldots, n_t\}$ is the set of nodes (possibly time-varying)
- $E_t \subseteq V_t \times V_t$ is the edge set
- $\mathbf{X}_t \in \mathbb{R}^{n_t \times d}$ is the node feature matrix: row $i$ is the feature vector $\mathbf{x}_t^{(i)} \in \mathbb{R}^d$ for node $i$

Let $A_t \in \{0,1\}^{n_t \times n_t}$ be the adjacency matrix. Let $\mathcal{N}(i) = \{j : (j,i) \in E_t\}$ denote the neighbors of node $i$.

## The Graph ESN Update Equation

In a Graph ESN, each node $i$ maintains a hidden state $\mathbf{h}_t^{(i)} \in \mathbb{R}^N$. The state update aggregates information from the node's own previous state, from its neighbors' previous states, and from the node's current features:

$$\mathbf{h}_t^{(i)} = (1-\alpha)\mathbf{h}_{t-1}^{(i)} + \alpha\,\tanh\!\left(W^{rec}\mathbf{h}_{t-1}^{(i)} + W^{in}\mathbf{x}_t^{(i)} + W^{nb}\sum_{j \in \mathcal{N}(i)} \mathbf{h}_{t-1}^{(j)}\right)$$

Here:
- $W^{rec} \in \mathbb{R}^{N \times N}$ is the standard (random, fixed) recurrent matrix at each node
- $W^{in} \in \mathbb{R}^{N \times d}$ maps node features into the hidden state
- $W^{nb} \in \mathbb{R}^{N \times N}$ is the (random, fixed) neighbor aggregation matrix
- $\alpha$ is the leaking rate

**Normalization.** For nodes with high degree, the sum $\sum_j \mathbf{h}_{t-1}^{(j)}$ can be large. A common normalization is:

$$\sum_{j \in \mathcal{N}(i)} \mathbf{h}_{t-1}^{(j)} \to \frac{1}{|\mathcal{N}(i)|}\sum_{j \in \mathcal{N}(i)} \mathbf{h}_{t-1}^{(j)}$$

or the symmetric Laplacian normalization: $\sum_j (D^{-1/2} A D^{-1/2})_{ij} \mathbf{h}_{t-1}^{(j)}$, where $D$ is the degree matrix. The Laplacian normalization is equivariant to node reordering and more closely parallels the spectral graph convolution of graph neural networks.

## Echo State Property for Graph ESNs

The ESP for Graph ESNs requires that the combined system of all nodes' state updates is a contraction. Writing the full state as $\mathbf{H}_t = \text{vec}([\mathbf{h}_t^{(1)}, \ldots, \mathbf{h}_t^{(n)}])$, the update can be written compactly as:

$$\mathbf{H}_t = (1-\alpha)\mathbf{H}_{t-1} + \alpha\,\tanh\!\left((I_n \otimes W^{rec} + A \otimes W^{nb})\mathbf{H}_{t-1} + (I_n \otimes W^{in})\mathbf{X}_t\right)$$

where $\otimes$ denotes the Kronecker product. The ESP sufficient condition becomes:

$$\rho\!\left((1-\alpha)I + \alpha(I_n \otimes W^{rec} + A \otimes W^{nb})\right) < 1$$

which, by the Kronecker product spectral theorem, requires:

$$\max_{k}\left|(1-\alpha) + \alpha(\lambda_k(W^{rec}) + \lambda_j(A)\cdot\lambda_k(W^{nb}))\right| < 1$$

where $\lambda_k(W^{rec})$ are eigenvalues of $W^{rec}$ and $\lambda_j(A)$ are eigenvalues of the adjacency matrix $A$. Since $|\lambda_j(A)| \leq \rho(A) \leq \|A\|_2$, a sufficient condition is:

$$\rho(W^{rec}) + \rho(A)\cdot\rho(W^{nb}) < \frac{1}{\alpha}(1 - (1-\alpha)) = 1$$

This has a clean interpretation: the spectral radii of the recurrent and neighbor-aggregation matrices must jointly be small enough, weighted by the spectral norm of the graph's adjacency structure.

## Graph-Level Readout

For graph-level prediction tasks (e.g., predicting a property of the entire molecule), the individual node states must be aggregated into a single graph-level representation. Common approaches:
- **Sum pooling:** $\mathbf{h}_G = \sum_i \mathbf{h}_t^{(i)}$
- **Mean pooling:** $\mathbf{h}_G = \frac{1}{n}\sum_i \mathbf{h}_t^{(i)}$
- **Hierarchical pooling:** Apply a deep ESN layer structure where each level aggregates node states to form super-node states

The graph-level readout $\mathbf{y}_G = W^{out}\mathbf{h}_G$ is then trained by standard linear regression or ridge regression.

## Deep Graph ESNs

The deep architecture extends naturally to graphs. In a deep Graph ESN with $L$ layers, layer $\ell$ updates:

$$\mathbf{h}_t^{(\ell,i)} = (1-\alpha_\ell)\mathbf{h}_{t-1}^{(\ell,i)} + \alpha_\ell\,\tanh\!\left(W_\ell^{rec}\mathbf{h}_{t-1}^{(\ell,i)} + W_\ell^{in}\mathbf{h}_t^{(\ell-1,i)} + W_\ell^{nb}\sum_{j \in \mathcal{N}(i)}\mathbf{h}_{t-1}^{(\ell,j)}\right)$$

The lower layers capture local, short-timescale structural patterns; the upper layers integrate these into long-range, long-timescale representations of graph structure. This mirrors the timescale hierarchy of the standard deep ESN, but now the "long range" is both temporal (across time steps) and spatial (across graph neighborhoods).

## Application: Molecular Property Prediction

Graph ESNs have been applied to predicting molecular properties (e.g., solubility, toxicity, binding affinity) from molecular graphs. Each atom is a node with features (atomic number, charge, hybridization), and bonds are edges with features (bond order, whether aromatic). The deep Graph ESN processes the molecular graph through multiple layers, each aggregating neighborhood information, and the final graph-level representation is passed to a linear readout.

A key advantage over trained graph neural networks for small-dataset tasks: the random, fixed reservoir matrices require no gradient-based training of the encoder. Only the linear readout is trained, making the approach computationally efficient and relatively robust to overfitting.

---

## References

- [Gallicchio2010] Gallicchio, C. & Micheli, A. (2010). Graph echo state networks. In *Proceedings of IJCNN*. IEEE.
- [Gallicchio2020] Gallicchio, C. & Micheli, A. (2020). Fast and deep graph neural networks. In *Proceedings of AAAI Conference on Artificial Intelligence*, 34(04), 3898–3905.
- [Scarselli2009] Scarselli, F., Gori, M., Tsoi, A.C., Hagenbuchner, M., & Monfardini, G. (2009). The graph neural network model. *IEEE Transactions on Neural Networks*, 20(1), 61–80.
- [Kipf2017] Kipf, T.N. & Welling, M. (2017). Semi-supervised classification with graph convolutional networks. In *Proceedings of ICLR*.
