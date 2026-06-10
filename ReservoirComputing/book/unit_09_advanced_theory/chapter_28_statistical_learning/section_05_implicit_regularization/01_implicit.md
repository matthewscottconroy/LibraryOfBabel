# Implicit Regularization in Reservoir Readout Training

## 28.5.1 The Regularization Spectrum

Reservoir readout training involves a choice of optimization algorithm and, optionally, an explicit penalty term. This choice has a profound effect on *which* solution is selected when the problem is underdetermined ($N > T$), and on the generalization properties of that solution. This section develops the theory of **implicit regularization**: the phenomenon whereby the optimization algorithm itself acts as a regularizer, even in the absence of an explicit penalty.

**Explicit regularization.** Ridge regression minimizes

$$
\mathcal{L}_\lambda(\mathbf{w}) = \frac{1}{T}\sum_{t=1}^T \left(y_t - \mathbf{w}^T \mathbf{x}(t)\right)^2 + \lambda\|\mathbf{w}\|_2^2.
$$

The unique solution is $\hat{\mathbf{w}}_\lambda = (\mathbf{X}^T\mathbf{X} + \lambda T\mathbf{I})^{-1}\mathbf{X}^T\mathbf{y}$. For any $\lambda > 0$, this is well-defined even when $N > T$. As $\lambda \to 0$, $\hat{\mathbf{w}}_\lambda \to \hat{\mathbf{w}}_{\min}$, the minimum-norm least squares solution.

**Implicit regularization.** What happens when we run gradient descent (GD) on the unregularized squared loss ($\lambda = 0$) in the overparameterized regime? [Neyshabur et al. 2015] showed empirically that GD converges to a solution with small norm, even without explicit regularization. [Zhang et al. 2017] demonstrated that deep networks can fit random labels but still generalize on real labels — the optimizer was selecting functions with structural properties beyond mere interpolation.

## 28.5.2 Gradient Descent Selects Minimum-Norm Solutions

**Theorem 28.10 (Implicit Regularization of Gradient Descent).** Consider the unregularized squared loss $\mathcal{L}(\mathbf{w}) = \frac{1}{T}\|\mathbf{X}\mathbf{w} - \mathbf{y}\|^2$ with $\mathbf{X} \in \mathbb{R}^{T \times N}$, $N > T$. Starting from $\mathbf{w}(0) = \mathbf{0}$ and running gradient descent with step size $\eta < 2/\sigma_1(\mathbf{X})^2$:

$$
\mathbf{w}(k) = \eta\mathbf{X}^T\sum_{j=0}^{k-1}(\mathbf{I} - \eta\mathbf{X}\mathbf{X}^T/T)^j(\mathbf{y} - \mathbf{X}\mathbf{w}(j)).
$$

At convergence ($k \to \infty$), gradient descent converges to $\hat{\mathbf{w}}_{\min} = \mathbf{X}^+\mathbf{y}$, the minimum-$\ell^2$-norm least squares solution.

*Proof.* GD initialized at $\mathbf{0}$ maintains $\mathbf{w}(k) \in \mathrm{rowspace}(\mathbf{X})$ for all $k$ (since each update $-\nabla\mathcal{L} = \frac{2}{T}\mathbf{X}^T(\mathbf{y} - \mathbf{X}\mathbf{w}) \in \mathrm{rowspace}(\mathbf{X})$). The minimum-norm solution is the unique least squares solution in the rowspace of $\mathbf{X}$. Since GD converges to a least squares solution and stays in the rowspace, it must converge to $\hat{\mathbf{w}}_{\min}$. $\square$

This result means that in the overparameterized reservoir regime, running gradient descent on the readout (without regularization) automatically selects the minimum-norm readout — exactly the solution analyzed by the benign overfitting theorem (Section 28.4).

## 28.5.3 Early Stopping as Implicit Regularization

Gradient descent need not run to convergence. **Early stopping** — terminating the optimization before full convergence — is one of the most practically effective regularization strategies for reservoir readouts, despite being rarely analyzed theoretically.

The GD iterate at step $k$ can be written as:

$$
\mathbf{w}(k) = \sum_{i=1}^{\min(N,T)} \left(1 - (1 - \eta\sigma_i^2/T)^k\right) \frac{\langle \mathbf{u}_i, \mathbf{y}\rangle}{\sigma_i} \mathbf{v}_i,
$$

where $\mathbf{X} = \mathbf{U}\boldsymbol{\Sigma}\mathbf{V}^T$. At step $k$, the coefficient of the $i$-th component is a monotonically increasing function of $k$ that saturates at $1$. Small singular values $\sigma_i$ saturate last — they are learned slowly. Early stopping therefore effectively *ignores small singular values*, acting as a form of spectral regularization.

**Equivalence with ridge regression.** The GD iterate at step $k$ is equivalent to the ridge regression solution with

$$
\lambda_k = \frac{1}{\eta k} \cdot \frac{1}{1 - (1-\eta\sigma_i^2/T)^k/(1 - \eta\sigma_i^2/T)},
$$

which is $\lambda_k \approx 1/(\eta k)$ for small $\eta\sigma_i^2$. Thus early stopping at step $k$ approximately corresponds to ridge regression with $\lambda \approx 1/(\eta k)$: larger $k$ corresponds to smaller effective $\lambda$ and more complex readout [Yao et al. 2007].

## 28.5.4 The Neural Tangent Kernel

For large overparameterized networks (not reservoir readouts per se), the **neural tangent kernel** (NTK) [Jacot et al. 2018] provides the correct framework for understanding what function gradient descent selects.

**Definition 28.9 (Neural Tangent Kernel).** For a parameterized function $f_\theta: \mathcal{X} \to \mathbb{R}$, the NTK is

$$
K_{\mathrm{NTK}}(x, x') = \left\langle \frac{\partial f_\theta(x)}{\partial \theta}, \frac{\partial f_\theta(x')}{\partial \theta}\right\rangle.
$$

**Theorem 28.11 (NTK in the Infinite-Width Limit [Jacot et al. 2018]).** For an infinitely wide neural network, $K_{\mathrm{NTK}}$ remains constant during training, and gradient flow on the squared loss is equivalent to kernel regression with kernel $K_{\mathrm{NTK}}$.

**Reservoir NTK.** For a reservoir computing system with *fixed reservoir* and trainable linear readout $f(\mathbf{x}) = \mathbf{w}^T\mathbf{x}$, the NTK simplifies dramatically:

$$
K_{\mathrm{RC}}(\mathbf{x}, \mathbf{x}') = \frac{\partial f(\mathbf{x})}{\partial \mathbf{w}} \cdot \frac{\partial f(\mathbf{x}')}{\partial \mathbf{w}} = \mathbf{x}^T\mathbf{x}'.
$$

This is simply the inner product kernel (linear kernel) on the reservoir state space. Gradient descent on the reservoir readout is therefore equivalent to kernel regression with the linear kernel $k(\mathbf{x}, \mathbf{x}') = \mathbf{x}^T\mathbf{x}'$.

The implicit regularizer in the NTK picture is the **RKHS norm** associated with the linear kernel, which equals the Euclidean norm $\|\mathbf{w}\|_2$. This confirms that gradient descent selects the minimum-$\|\mathbf{w}\|_2$ solution.

## 28.5.5 Implications for Reservoir Design

The implicit regularization results have several design implications:

**1. Choice of optimizer matters.** Different optimizers select different implicit solutions. SGD selects small-norm solutions (similar to GD); coordinate descent may select sparse solutions (analogous to LASSO). [Gunasekar et al. 2018] characterized implicit regularization of matrix factorization as nuclear norm minimization.

**2. Batch size affects implicit regularization.** Large-batch SGD converges to solutions with larger Hessian spectral gap (sharper minima); small-batch SGD converges to flatter minima [Keskar et al. 2017]. For reservoir readouts, this suggests that online (sample-at-a-time) RLS learning may find different solutions than batch ridge regression, even with the same nominal regularization.

**3. The reservoir changes the NTK.** If the reservoir weights are partially trained (via, e.g., FORCE learning or intrinsic plasticity), the NTK changes, and the implicit regularization changes with it. The analysis of implicit regularization for partially-trained reservoirs is an active research area.

**4. Leaky integration modifies the kernel.** For a leaky ESN with leak rate $\alpha$, the effective kernel is

$$
K_{\mathrm{RC}}^\alpha(\mathbf{x}(t), \mathbf{x}(s)) = \mathbf{x}(t)^T\mathbf{x}(s),
$$

where the states $\mathbf{x}(t)$ are already time-smoothed by the leak. The kernel measures temporal similarity in the smoothed state space, providing automatic smoothing of the readout function.

## References

- Gunasekar, S., Lee, J., Soudry, D., and Srebro, N. (2018). Characterizing implicit bias in terms of optimization geometry. In *Proceedings of the 35th International Conference on Machine Learning*, 1832–1841.
- Jacot, A., Gabriel, F., and Hongler, C. (2018). Neural tangent kernel: Convergence and generalization in neural networks. In *Advances in Neural Information Processing Systems*, 31.
- Keskar, N. S., Mudigere, D., Nocedal, J., Smelyanskiy, M., and Tang, P. T. P. (2017). On large-batch training for deep learning: Generalization gap and sharp minima. In *International Conference on Learning Representations*.
- Neyshabur, B., Tomioka, R., and Srebro, N. (2015). In search of the real inductive bias: On the role of implicit regularization in deep learning. In *ICLR Workshop Track*.
- Yao, Y., Rosasco, L., and Caponnetto, A. (2007). On early stopping in gradient descent learning. *Constructive Approximation*, 26(2), 289–315.
- Zhang, C., Bengio, S., Hardt, M., Recht, B., and Vinyals, O. (2017). Understanding deep learning requires rethinking generalization. In *International Conference on Learning Representations*.
