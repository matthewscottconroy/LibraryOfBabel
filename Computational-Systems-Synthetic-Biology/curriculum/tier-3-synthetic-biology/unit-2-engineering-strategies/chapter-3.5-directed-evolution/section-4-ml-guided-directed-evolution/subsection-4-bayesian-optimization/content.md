# Bayesian Optimization for Directed Evolution

Suppose your surrogate model has been trained on 200 measured variants and is doing reasonably well — Spearman correlation around 0.65. You now have a budget for 96 more experiments this round. How do you decide which 96 sequences to measure? You could simply take the top 96 sequences by predicted fitness — but that has a problem. If your model is slightly wrong in its confident regions, you'll spend the whole round measuring sequences that cluster together, all predicted high but all failing in the same direction. You've exploited the model's knowledge without questioning it. Alternatively, you could sample broadly from uncertain regions — but if the model is actually good, most of those experiments tell you little about where the optimum is. What you want is a principled rule for balancing confidence and uncertainty, exploitation and exploration, in a way that gets you to the optimum in the fewest experiments possible. That rule is called an acquisition function, and Bayesian optimization is the framework that derives it rigorously.

Bayesian optimization (BO) is the principled framework for using a probabilistic surrogate model to efficiently find the optimum of an expensive-to-evaluate function. In the context of directed evolution, the "function" is the fitness of a protein sequence, which is expensive because measuring it requires synthesis, expression, and biochemical assay. BO provides the acquisition function — the rule for deciding which sequences to measure next — that balances exploring uncharted sequence space with exploiting the model's current best predictions.

## The Bayesian Optimization Setup

**Problem**: find the sequence $x^*$ that maximizes fitness $f(x)$, given:
- A surrogate model $\hat{f}(x)$ trained on $N$ previously measured sequences
- The ability to measure fitness $f(x)$ for a batch of new sequences at each iteration
- A total experimental budget of $T$ measurements

**BO algorithm**:
1. Fit surrogate model on current training data
2. Apply acquisition function $\alpha(x)$ to rank all candidate sequences
3. Measure fitness of top-$k$ sequences by $\alpha(x)$
4. Add new measurements to training data
5. Retrain surrogate model
6. Repeat until budget exhausted or convergence

## The Exploration-Exploitation Trade-off

The central tension in BO is between:

**Exploitation**: measure sequences where the model predicts high fitness. If the model is accurate, this efficiently finds high-fitness variants. If the model is wrong in specific regions, exploitation blindly follows bad predictions.

**Exploration**: measure sequences where the model is uncertain. This improves model accuracy in uncertain regions, potentially revealing high-fitness areas the model didn't know about. But if the model is already accurate, exploration wastes measurements.

**Ideal strategy**: early rounds → explore (less data, model is inaccurate everywhere); late rounds → exploit (more data, model is confident, primarily finding the optimum).

## Acquisition Functions

An acquisition function $\alpha(x)$ scores each candidate sequence based on the surrogate model's predictions ($\mu(x)$, the predicted mean fitness) and uncertainty ($\sigma(x)$, the predicted standard deviation).

### Upper Confidence Bound (UCB)

$$\alpha_{UCB}(x) = \mu(x) + \kappa \sigma(x)$$

- $\mu(x)$: exploitation term — prefer high-predicted-fitness sequences
- $\kappa \sigma(x)$: exploration term — prefer sequences where model is uncertain
- $\kappa$: trade-off parameter. $\kappa = 0$ → pure exploitation; $\kappa = 2$: standard; $\kappa = 5$: high exploration

**Properties**: UCB is simple to implement and intuitive. The $\kappa$ parameter must be tuned (or annealed from high to low over rounds).

**Selecting sequences to measure**: compute $\alpha_{UCB}(x)$ for all candidates; measure the top-$k$ sequences.

### Expected Improvement (EI)

EI measures the expected gain over the current best observation $f(x^+) = \max_i y_i$:

$$\alpha_{EI}(x) = \mathbb{E}[\max(f(x) - f(x^+), 0)]$$

For a Gaussian surrogate model:
$$\alpha_{EI}(x) = (\mu(x) - f(x^+)) \Phi(Z) + \sigma(x) \phi(Z)$$

Where $Z = (\mu(x) - f(x^+)) / \sigma(x)$, $\Phi$ is the standard normal CDF, and $\phi$ is the standard normal PDF.

**Interpretation**: the first term $(\mu(x) - f(x^+))\Phi(Z)$ penalizes sequences that are below the current best (no expected improvement); the second term $\sigma(x)\phi(Z)$ rewards uncertainty — even sequences with mediocre mean predictions may have high EI if $\sigma(x)$ is large.

**Advantage**: EI automatically balances exploration and exploitation without a tunable $\kappa$ parameter. It is the most commonly used acquisition function in standard BO literature.

### Thompson Sampling

Thompson sampling draws one sample from the surrogate posterior, then chooses the sequence that maximizes the sampled function value:

1. Draw $\tilde{f}$ from the surrogate posterior: $\tilde{f} \sim p(f | data)$
2. Select $x^* = \arg\max_x \tilde{f}(x)$

Repeat for each sequence to be measured (draw independent samples for each).

**Why it works**: by sampling from the posterior, Thompson sampling naturally mixes exploration and exploitation — high-mean sequences are often selected (posterior mean is high → sample likely high), but uncertain regions occasionally produce high samples too (exploration).

**Advantage**: naturally parallelizable — drawing $k$ independent samples gives $k$ diverse candidates for batch measurement.

### Probability of Improvement (PI)

$$\alpha_{PI}(x) = P(f(x) > f(x^+)) = \Phi\left(\frac{\mu(x) - f(x^+)}{\sigma(x)}\right)$$

Simpler than EI; only rewards sequences predicted to exceed the current best with high probability. More exploitative than EI; rarely used in practice because it ignores the magnitude of improvement.

## Batch Bayesian Optimization

In directed evolution, it is far more efficient to measure $k$ sequences simultaneously (one synthesis/assay batch) than to run $k$ sequential single-measurement rounds. **Batch BO** methods select $k$ diverse, high-quality candidates per round:

**Hallucination/fantasizing** (qEI, qUCB): extend the acquisition function to consider $k$ sequences jointly. The joint expected improvement of measuring $k$ sequences simultaneously accounts for information redundancy — measuring two nearly identical sequences is wasteful.

**Greedy batch construction**: iteratively select the sequence with highest $\alpha(x)$, update the surrogate model as if it had been measured (using the predicted value), then select the next candidate. This avoids redundant selection at the cost of potentially suboptimal batches.

## Worked Example: BO for Thermostability Engineering

**Setup**: evolve thermostable variant of mesophilic xylanase (Tm = 58°C target → Tm > 75°C)

**Initial data**: measure Tm for 96 variants from saturation mutagenesis of 5 predicted hotspot positions.

**Model**: GP with RBF kernel on ESM-2 embeddings.

**Round 1 BO** (UCB, $\kappa = 2$):
- Compute $\mu(x)$ and $\sigma(x)$ for all 20⁵ = 3.2M candidate sequences
- Top $\alpha_{UCB}$ sequences cluster near positions 3 and 5 (high predicted benefit + high uncertainty)
- Synthesize and measure top 96 by UCB

**Round 2 BO**: model now has 192 training points; $\sigma(x)$ reduced near positions 3 and 5 → UCB now emphasizes positions 1, 2, 4 where uncertainty remains high.

**After 3 rounds** (288 total measurements): model converges; top-10 predictions all have Tm > 73°C; best experimentally confirmed: Tm = 79°C.

**Comparison to random screening**: to find a Tm = 79°C variant by random screening at 3.2M candidate sequences would require testing ~3.2M × (fraction with Tm > 79°C). If that fraction is 10⁻³ (1 in 1000), random screening would require ~3,200 measurements vs. 288 for BO — 11-fold improvement in efficiency.

## Practical Implementation

```python
from gpytorch import ExactGP
from botorch import fit_gpytorch_model
from botorch.acquisition import ExpectedImprovement
from botorch.optim import optimize_acqf

# 1. Fit GP surrogate model
gp = ExactGP(train_X, train_Y, likelihood)
fit_gpytorch_model(gp)

# 2. Define acquisition function
best_f = train_Y.max()
EI = ExpectedImprovement(model=gp, best_f=best_f, maximize=True)

# 3. Optimize acquisition function over candidate sequences
candidate_embeddings = esm_embed(candidate_sequences)
EI_scores = EI(candidate_embeddings.unsqueeze(1))

# 4. Select top-k candidates
top_k_idx = EI_scores.argsort(descending=True)[:k]
candidates = [candidate_sequences[i] for i in top_k_idx]
```

## Why This Matters

Bayesian optimization provides the mathematical framework that converts a surrogate model's predictions and uncertainties into an actionable experimental plan. Without BO, an ML model could predict fitness for all candidates, but selecting which ones to measure from among millions of predictions requires principled criteria. Acquisition functions like UCB and EI embed decades of decision theory and optimal control theory into an algorithm that any directed evolution practitioner can implement. The result is that MLDE with BO consistently outperforms both random screening and greedy exploitation (always measuring the predicted best) in the data-limited regime of directed evolution — finding the fitness optimum in fewer experimental rounds.
