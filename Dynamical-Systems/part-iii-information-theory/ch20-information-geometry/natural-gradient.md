# 20.6 Natural Gradient

Gradient descent in parameter space has a well-known pathology: its performance depends heavily on the parametrization. For a neural network, different layers have very different scales; for a mixture model, the entropy and the means are on very different scales. Ordinary gradient descent moves in Euclidean parameter space, which ignores this geometry and leads to slow convergence.

The *natural gradient*, introduced by Amari in 1998, fixes this by replacing the Euclidean gradient with the gradient in the Fisher-Rao metric.

**Definition 20.6.1 (Natural Gradient).** For a loss function $L(\theta)$ on a statistical manifold, the *natural gradient* is:
$$\tilde\nabla L(\theta) = g(\theta)^{-1} \nabla L(\theta),$$
where $g(\theta)$ is the Fisher information matrix.

The ordinary gradient $\nabla L(\theta)$ points in the direction of steepest descent in Euclidean parameter space. The natural gradient $\tilde\nabla L(\theta)$ points in the direction of steepest descent in the Fisher-Rao metric — the "information-geometric" steepest descent.

**Motivation:** Steepest descent in Euclidean parameter space depends on how you choose to parametrize the model. If you reparametrize $\phi = f(\theta)$, the gradient changes, and so does the direction of steepest descent. This is undesirable: the best direction to move should not depend on our arbitrary choice of coordinates.

The Fisher metric provides the canonical Riemannian metric on the space of distributions, which is invariant under reparametrization (by Chentsov's theorem). Steepest descent in the Fisher metric — natural gradient — is therefore invariant: it gives the same direction of improvement regardless of how you parametrize the model. This is the information-geometric notion of "the right direction to move."

**Convergence:** Natural gradient descent achieves Fisher efficiency: it converges as fast as any second-order method (like Newton's method), without computing the Hessian explicitly. In the vicinity of a minimum, natural gradient descent is equivalent to Newton's method in the information-geometric metric, giving quadratic convergence.

**Application — Neural Networks:** Amari showed that natural gradient descent for neural networks — using the Fisher information of the network output distribution — can significantly accelerate learning and avoids the ill-conditioning that arises from the layered parametrization of deep networks. The practical bottleneck is computing $g(\theta)^{-1}$, which is a large matrix for neural networks. Various approximations (Kronecker factorizations like K-FAC, diagonal approximations) make natural gradient practical for large models.

Natural gradient connects directly to the Pythagorean theorem: natural gradient descent is approximately equivalent to repeatedly computing the $e$-projection of the current distribution onto the direction of steepest KL divergence decrease. The information geometry makes explicit what "the right direction" means, independent of parametrization.
