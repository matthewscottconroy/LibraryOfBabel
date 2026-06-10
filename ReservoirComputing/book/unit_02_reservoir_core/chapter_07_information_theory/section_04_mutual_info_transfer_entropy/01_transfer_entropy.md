# Section 7.4: Mutual Information and Transfer Entropy

## 7.4.1 Beyond Correlation

Linear correlation between two time series measures only their first-order statistical relationship. Many aspects of reservoir dynamics — the nonlinear mixing of inputs, the directed flow of information from input to state variables, and the causal structure within the recurrent network — require higher-order measures. Mutual information and its extension to time series, transfer entropy, provide the appropriate framework [CoverThomas2006].

## 7.4.2 Mutual Information

Let $X$ and $Y$ be two random variables with joint distribution $p(x, y)$ and marginals $p_X(x)$, $p_Y(y)$. The *mutual information* is

$$I(X; Y) = H(X) + H(Y) - H(X, Y),$$

where $H(X) = -\mathbb{E}[\log p(X)]$ is the Shannon entropy and $H(X, Y) = -\mathbb{E}[\log p(X, Y)]$ is the joint entropy. Equivalently,

$$I(X; Y) = D_{KL}(p(x,y) \| p_X(x) p_Y(y)) = \sum_{x,y} p(x,y) \log \frac{p(x,y)}{p_X(x) p_Y(y)},$$

the KL divergence between the joint distribution and the product of the marginals. $I(X; Y) \geq 0$ with equality if and only if $X$ and $Y$ are statistically independent.

Mutual information is symmetric ($I(X;Y) = I(Y;X)$) and invariant to invertible transformations ($I(X;Y) = I(f(X); g(Y))$ for bijections $f, g$). It captures all statistical dependencies, not just linear ones: a nonlinear relationship with zero correlation still has positive mutual information.

**Application to reservoir computing.** The mutual information $I(\mathbf{u}(t); \mathbf{x}(t))$ between the current input and the current reservoir state measures how much information about the input is encoded in the state. More relevant is $I(\mathbf{u}_{1:t}; \mathbf{x}(t))$ — how much of the input *history* is encoded — which is bounded by $N \log_2(1 + \text{SNR})$ nats for a Gaussian reservoir (Shannon's channel capacity formula).

## 7.4.3 Transfer Entropy

Mutual information is symmetric and does not distinguish cause from effect. Schreiber [Schreiber2000] introduced *transfer entropy* as a measure of directed information flow:

**Definition 7.4.1 (Transfer Entropy).** The *transfer entropy* from process $X$ to process $Y$, measuring the influence of $X$'s past on $Y$'s future given $Y$'s own past, is

$$TE(X \to Y) = I(Y_t;\ X_{t-1}^{(k)}\ |\ Y_{t-1}^{(l)}),$$

where $X_{t-1}^{(k)} = (X_{t-1}, X_{t-2}, \ldots, X_{t-k})$ is the $k$-step history of $X$, and $Y_{t-1}^{(l)} = (Y_{t-1}, \ldots, Y_{t-l})$ is the $l$-step history of $Y$. Expanding using the definition of conditional mutual information:

$$TE(X \to Y) = \sum_{y_t, x_{t-1}^{(k)}, y_{t-1}^{(l)}} p(y_t, x_{t-1}^{(k)}, y_{t-1}^{(l)}) \log \frac{p(y_t | x_{t-1}^{(k)}, y_{t-1}^{(l)})}{p(y_t | y_{t-1}^{(l)})}.$$

Transfer entropy measures how much the past $k$ steps of $X$ improve prediction of $Y_t$ *above and beyond* what $Y$'s own past already provides. It is zero when $X$ contributes no additional predictive power, which occurs when $X$ and $Y$ are conditionally independent given $Y$'s past — the Granger non-causality condition. Transfer entropy thus operationalizes Granger causality in an information-theoretic, model-free way.

**Transfer entropy is asymmetric:** $TE(X \to Y) \neq TE(Y \to X)$ in general. This asymmetry is essential for identifying directed information flow.

## 7.4.4 Information Flow from Input to Reservoir

The transfer entropy $TE(\mathbf{u} \to x_i)$ from the input stream to neuron $i$ measures how much the input drives neuron $i$'s dynamics beyond its own recurrent history. Neurons receiving strong direct input connections ($|W^{in}_i|$ large) and neurons connected to many input-receiving neurons will have high $TE(\mathbf{u} \to x_i)$.

A reservoir with high total input transfer entropy $\sum_i TE(\mathbf{u} \to x_i)$ encodes the input efficiently into the state space. Reservoirs near the edge of chaos have been found to maximize this quantity: in the chaotic regime, information is stored in attractors that are hard to access from the input; in the stable contractive regime, information decays too fast. At the edge, each neuron acts as an effective sensor for input changes [Langton1990].

The conditional mutual information $I(\mathbf{x}(t);\ \mathbf{u}(t-\tau)\ |\ \mathbf{u}(t-\tau+1), \ldots, \mathbf{u}(t))$ measures how much unique information about $\mathbf{u}(t-\tau)$ is retained in the state after conditioning on more recent inputs — a direct measure of the reservoir's temporal memory at lag $\tau$.

## 7.4.5 Information Flow Within the Reservoir

Transfer entropy between pairs of reservoir neurons, $TE(x_j \to x_i)$, reveals the directed information structure of the reservoir's internal dynamics. This can be used diagnostically:

- **Hubs:** Neurons with high total outgoing transfer entropy $\sum_i TE(x_j \to x_i)$ are information sources; neurons with high incoming transfer entropy are sinks.
- **Bottlenecks:** If information must flow through a small set of neurons to propagate through the network, those neurons are information bottlenecks. Removing or saturating them would disconnect the information flow.
- **Recurrent loops:** Cycles in the transfer entropy graph (where $TE(x_j \to x_i) > 0$ and $TE(x_i \to x_j) > 0$) identify recurrent memory loops within the reservoir.

Computing transfer entropy for all pairs of $N$ neurons requires estimating $O(N^2)$ conditional distributions — expensive but feasible with the JIDT toolkit [Lizier2014] for moderate $N$.

## 7.4.6 Active Information Storage

A third measure, introduced by Lizier et al. [Lizier2012], is the *active information storage* (AIS):

$$AIS(Y_t) = I(Y_t;\ Y_{t-1}^{(k)}) = H(Y_t) - H(Y_t | Y_{t-1}^{(k)}).$$

AIS measures how much of the current value of a process is predictable from its own past — a direct quantification of *self-memory* or autocorrelation in information-theoretic terms.

For reservoir neurons, AIS$(x_i(t))$ measures how much the current state of neuron $i$ is determined by its own history, independent of other neurons or the input. High AIS indicates strong self-recurrence; low AIS indicates that the neuron is primarily driven by its inputs (other neurons or external signal) rather than its own dynamics.

The *information storage* and *transfer* of a reservoir can be decomposed as:

$$H(x_i(t)) = AIS(x_i) + \sum_j TE(x_j \to x_i) + \text{noise entropy},$$

providing a complete information budget for each neuron [Lizier2012].

## 7.4.7 Gaussian Approximations

For reservoirs with tanh or linear activation functions, driven by Gaussian inputs, the distribution of states is approximately Gaussian (a consequence of the central limit theorem for the sum of many weighted inputs). In the Gaussian case, all information-theoretic quantities reduce to functions of the covariance matrix:

$$I(X; Y) = \frac{1}{2} \log \frac{\det(\Sigma_X) \det(\Sigma_Y)}{\det(\Sigma_{XY})},$$

where $\Sigma_{XY}$ is the joint covariance. This is the *Gaussian mutual information*, and it equals the capacity of the Gaussian channel with the given covariance structure.

Under the Gaussian approximation, transfer entropy reduces to the *Granger causality* measure [Granger1969], which is the log-ratio of prediction variances:

$$TE_{Gauss}(X \to Y) = \frac{1}{2} \log \frac{\text{Var}(Y_t | Y_{t-1}^{(l)})}{\text{Var}(Y_t | Y_{t-1}^{(l)}, X_{t-1}^{(k)})}.$$

This connection means that for Gaussian reservoirs, standard linear VAR model analysis (Granger causality testing) is sufficient for characterizing directed information flow — a significant computational saving [CoverThomas2006].

For nonlinear reservoirs with non-Gaussian states, non-parametric estimators (kernel density estimation, $k$-nearest-neighbor entropy estimation, or bin-based estimators as implemented in JIDT) are required.

## 7.4.8 Practical Computation with JIDT

The Java Information Dynamics Toolkit (JIDT) [Lizier2014] provides open-source implementations of MI, TE, and AIS estimators in multiple forms (discrete, Gaussian approximation, kernel density, $k$-NN). For reservoir analysis:

1. Run the reservoir on a long test sequence (length $T \geq 10^4$) to ensure reliable estimation.
2. Extract the state time series $\{x_i(t)\}_{t=1}^T$ for each neuron $i$.
3. Estimate $TE(\mathbf{u} \to x_i)$ for each neuron using the Gaussian estimator (sufficient for tanh reservoirs near the linear regime) or the $k$-NN estimator for strongly nonlinear reservoirs.
4. Estimate $AIS(x_i)$ for each neuron to identify memory vs. driven neurons.

The resulting information-flow graph provides a diagnostic portrait of the reservoir's computational substrate, identifying which neurons perform memory storage, which perform input integration, and which perform nonlinear transformation.

---

## References

- **[CoverThomas2006]** T. M. Cover and J. A. Thomas. *Elements of Information Theory*, 2nd ed. Wiley, 2006.
- **[Granger1969]** C. W. J. Granger. "Investigating causal relations by econometric models and cross-spectral methods." *Econometrica*, 37(3):424-438, 1969.
- **[Langton1990]** C. G. Langton. "Computation at the edge of chaos: Phase transitions and emergent computation." *Physica D*, 42(1-3):12-37, 1990.
- **[Lizier2012]** J. T. Lizier, M. Prokopenko, and A. Y. Zomaya. "Local measures of information storage in complex distributed computation." *Information Sciences*, 208:39-54, 2012.
- **[Lizier2014]** J. T. Lizier. "JIDT: An information-theoretic toolkit for studying the dynamics of complex systems." *Frontiers in Robotics and AI*, 1:11, 2014.
- **[Schreiber2000]** T. Schreiber. "Measuring information transfer." *Physical Review Letters*, 85(2):461-464, 2000.
