# Training Deep Reservoirs

## Standard Approach: Fixed Layers and Ridge Regression

The simplest and most common training procedure for a DeepESN is identical in structure to single-layer ESN training: fix all recurrent weights $\{\mathbf{W}^{\text{rec},\ell}\}_{\ell=1}^L$ and inter-layer weights $\{\mathbf{W}^{\text{in},\ell}\}_{\ell=1}^L$, collect the concatenated state matrix

$$\mathbf{S} = [\mathbf{X}^{(1)} \mid \mathbf{X}^{(2)} \mid \cdots \mid \mathbf{X}^{(L)}] \in \mathbb{R}^{T \times \sum_\ell N_\ell},$$

where $\mathbf{X}^{(\ell)} \in \mathbb{R}^{T \times N_\ell}$ is the state matrix for layer $\ell$, and solve the ridge regression problem:

$$\hat{\mathbf{W}}^{\text{out}} = \mathbf{Y}^* \mathbf{S}^\top (\mathbf{S} \mathbf{S}^\top + \lambda \mathbf{I})^{-1},$$

or equivalently in the transposed form

$$\hat{\mathbf{W}}^{\text{out}} = (\mathbf{S}^\top \mathbf{S} + \lambda \mathbf{I})^{-1} \mathbf{S}^\top \mathbf{Y}^*.$$

The second form is preferred when $\sum_\ell N_\ell < T$, giving cost $O((\sum_\ell N_\ell)^3 + T(\sum_\ell N_\ell)^2)$ [Gallicchio & Micheli 2017].

## Computational Cost

The state collection phase requires running the forward pass of all $L$ layers over $T$ time steps. Each layer $\ell$ requires $O(N_\ell^2 + N_\ell N_{\ell-1})$ operations per time step (recurrent and inter-layer products). For equal-size layers ($N_\ell = N$), the total state collection cost is:

$$O\!\left(L N^2 T\right).$$

The ridge regression solve on the concatenated state has cost:

$$O\!\left((LN)^3 + T(LN)^2\right) = O\!\left(L^3 N^3 + T L^2 N^2\right).$$

For $L = 5$, $N = 200$, $T = 10{,}000$: state collection costs $\sim 5 \times 200^2 \times 10^4 = 2 \times 10^9$ operations; ridge regression costs $\sim 5^3 \times 200^3 = 10^9$ operations. Both are tractable on modern hardware without GPU acceleration [Gallicchio & Micheli 2017].

## Layerwise Pretraining

Inspired by the greedy layerwise pretraining of deep belief networks [Hinton et al. 2006], one can train a DeepESN layer by layer. Train a readout $\mathbf{W}_1^{\text{out}}$ on layer-1 states, compute the residual error, use the residual as a "target" for layer 2, and so on. More concretely:

1. Train $\hat{\mathbf{W}}_1^{\text{out}}$ by ridge regression on $\mathbf{X}^{(1)}$ vs. $\mathbf{Y}^*$; compute residual $\mathbf{E}_1 = \mathbf{Y}^* - \hat{\mathbf{W}}_1^{\text{out}} \mathbf{X}^{(1)}$.
2. Train $\hat{\mathbf{W}}_2^{\text{out}}$ on $\mathbf{X}^{(2)}$ vs. $\mathbf{E}_1$; compute residual $\mathbf{E}_2 = \mathbf{E}_1 - \hat{\mathbf{W}}_2^{\text{out}} \mathbf{X}^{(2)}$.
3. Repeat for layers $3, \ldots, L$.

The combined prediction is $\hat{\mathbf{y}} = \sum_\ell \hat{\mathbf{W}}_\ell^{\text{out}} \mathbf{x}^{(\ell)}$, which is equivalent to fitting the concatenated readout on the layerwise residuals. This is related to boosting and gradient boosting: each layer corrects the residual error of all previous layers [Hinton et al. 2006].

## Fine-Tuning with Gradient Descent

After fixing the reservoir and training the readout, one can optionally fine-tune the inter-layer weights $\mathbf{W}^{\text{in},\ell}$ using gradient descent. Because the reservoir is fixed, the gradient of the readout loss with respect to $\mathbf{W}^{\text{in},\ell}$ can be computed by backpropagation through the inter-layer connections (but not through the recurrent matrices). The update is:

$$\mathbf{W}^{\text{in},\ell} \leftarrow \mathbf{W}^{\text{in},\ell} - \eta \nabla_{\mathbf{W}^{\text{in},\ell}} \mathcal{L},$$

where $\mathcal{L} = \|\mathbf{Y}^* - \mathbf{W}^{\text{out}} \mathbf{S}\|_F^2 + \lambda \|\mathbf{W}^{\text{out}}\|_F^2$. This fine-tuning step adapts the inter-layer projections to the task, improving performance modestly while maintaining the fixed-reservoir guarantee. The recurrent matrices $\mathbf{W}^{\text{rec},\ell}$ remain fixed throughout.

## Hybrid: Fix Early Layers, Fine-Tune Later Layers

An alternative is to fix the lower layers (which track fast, input-close dynamics) and allow the upper layers to be partially trained. Since upper layers integrate over longer timescales, their dynamics are smoother and gradient-based training is more stable (smaller effective spectral radius). This hybrid approach captures the best of both worlds: lower layers provide stable, well-characterized fast features; upper layers are adapted to the task's slow structure [Gallicchio & Micheli 2017].

## Batch vs. Online Training

For large datasets or streaming data, batch ridge regression is impractical. Online recursive least squares (RLS) on the concatenated state extends directly to DeepESNs. The update equations are the same as for single-layer ESN (Chapter 5), applied to $\mathbf{s}_t = [\mathbf{x}_t^{(1)}; \ldots; \mathbf{x}_t^{(L)}]$. The cost per time step is $O((LN)^2)$ — higher than single-layer RLS by a factor of $L^2$, but still practical for moderate $L$.

---

## References

- Gallicchio, C., & Micheli, A. (2017). Echo state property of deep reservoir computing networks. *Cognitive Computation*, 9(3), 337–350.
- Hinton, G. E., Osindero, S., & Teh, Y. W. (2006). A fast learning algorithm for deep belief nets. *Neural Computation*, 18(7), 1527–1554.
