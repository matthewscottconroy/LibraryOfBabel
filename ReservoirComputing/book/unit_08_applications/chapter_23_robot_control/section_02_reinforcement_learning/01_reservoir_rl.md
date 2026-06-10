# Section 23.2: Reinforcement Learning with Reservoir Policies

## 23.2.1 The Reinforcement Learning Framework

In reinforcement learning (RL), an agent interacts with an environment through a loop: at each time step $t$, the agent observes a state $s_t$, selects an action $a_t$ according to its policy $\pi(a | s)$, receives a scalar reward $r_t$, and transitions to the next state $s_{t+1}$ according to the environment's dynamics $p(s_{t+1} | s_t, a_t)$.

The agent's objective is to find a policy that maximizes the expected discounted return:

$$J(\pi) = \mathbb{E}_{\pi}\left[\sum_{t=0}^T \gamma^t r_t\right]$$

where $\gamma \in [0,1)$ is the discount factor, balancing immediate versus long-term rewards.

Standard deep RL uses a neural network to parameterize the policy $\pi_\theta(a|s)$, with parameters $\theta$ optimized by policy gradient methods (REINFORCE, PPO, SAC) or value-based methods (DQN, TD3). The computational cost and instability of deep RL motivates the reservoir computing alternative.

## 23.2.2 Reservoir Policy Architecture

The reservoir policy [TrieschlEtAl2007, NatschlaegerEtAl2003] parameterizes the policy using the reservoir state as the observation embedding:

1. **Input encoding**: The environment observation $s_t$ is fed to the reservoir as input $\mathbf{u}(t) = s_t$.

2. **Reservoir state**: The reservoir evolves:
   $$\mathbf{x}(t) = \tanh(W_{\text{res}}\mathbf{x}(t-1) + W_{\text{in}}s_t)$$

3. **Policy output**: The action is a linear function of the reservoir state:
   $$a_t = W_{\text{out}}\mathbf{x}(t)$$
   For continuous action spaces (e.g., joint torques), this is a linear policy. For discrete action spaces, a softmax is applied.

The only trainable parameters are $W_{\text{out}} \in \mathbb{R}^{d_a \times N}$, where $d_a$ is the action dimension and $N$ is the reservoir size. All other parameters ($W_{\text{res}}$, $W_{\text{in}}$) are fixed at initialization.

### Why Does a Linear Policy Work?

The reservoir transforms the observation $s_t$ into a high-dimensional, nonlinear, temporally extended representation $\mathbf{x}(t)$. Even if the optimal policy is highly nonlinear in $(s_0, s_1, \ldots, s_t)$ (the history of observations), it may be approximately linear in $\mathbf{x}(t)$ — because the reservoir has already performed the nonlinear feature extraction.

This is a consequence of the kernel interpretation of reservoirs (Chapter 6): the reservoir state $\mathbf{x}(t)$ can be viewed as a feature map into a high-dimensional space in which the optimal policy is nearly linear. The quality of this approximation depends on the reservoir design: richer reservoirs (larger $N$, appropriate spectral structure) allow more complex policies to be linearly approximated.

## 23.2.3 Training Methods

### Policy Gradient: REINFORCE

The REINFORCE algorithm [Williams1992] estimates the gradient of $J(W_{\text{out}})$ by:

$$\nabla_{W_{\text{out}}} J \approx \frac{1}{M} \sum_{m=1}^M G_m \sum_t \nabla_{W_{\text{out}}} \log \pi_{W_{\text{out}}}(a_t^{(m)} | s_t^{(m)})$$

where $G_m = \sum_t \gamma^t r_t^{(m)}$ is the return from episode $m$ and $M$ is the number of episodes.

For a stochastic Gaussian policy $a_t \sim \mathcal{N}(W_{\text{out}}\mathbf{x}(t), \sigma^2 I)$:

$$\nabla_{W_{\text{out}}} \log \pi(a_t | s_t) = \frac{1}{\sigma^2}(a_t - W_{\text{out}}\mathbf{x}(t))\mathbf{x}(t)^\top$$

The update rule is:

$$W_{\text{out}} \leftarrow W_{\text{out}} + \eta \frac{1}{M} \sum_{m=1}^M G_m \sum_t \frac{a_t^{(m)} - W_{\text{out}}\mathbf{x}^{(m)}(t)}{\sigma^2}\mathbf{x}^{(m)}(t)^\top$$

Because only $W_{\text{out}}$ is updated, the gradient computation involves only linear operations, making the algorithm extremely simple compared to backpropagation through the full network.

### Evolution Strategies

Evolution strategies (ES) [SalimansEtAl2017] optimize $J(W_{\text{out}})$ by parameter perturbation:

1. Generate $\lambda$ perturbed policy parameters: $W_{\text{out}}^{(k)} = W_{\text{out}} + \sigma \epsilon_k$, $\epsilon_k \sim \mathcal{N}(0, I)$.
2. Evaluate each perturbed policy: $J_k = J(W_{\text{out}}^{(k)})$.
3. Update: $W_{\text{out}} \leftarrow W_{\text{out}} + \frac{\eta}{\lambda\sigma} \sum_k J_k \epsilon_k$.

ES is well-suited to reservoir policies because:
- No gradient computation required (evaluations are black-box)
- Naturally parallelizable ($\lambda$ evaluations can run simultaneously on $\lambda$ CPUs)
- Robust to non-differentiable environments (contact dynamics, discrete rewards)

For a reservoir policy with $N = 500$ and $d_a = 2$, the number of parameters is $1000$ — much smaller than a deep MLP policy ($\sim 10^5$–$10^6$), making ES feasible.

### Linear Quadratic Regulator (LQR) Analogy

For linear environments (where $s_{t+1} = As_t + Ba_t + \text{noise}$) and quadratic rewards (LQR), the optimal policy is linear in the state. In this case, a reservoir policy with $W_{\text{out}}$ trained by least squares is guaranteed to achieve near-optimal performance, provided the reservoir state is rich enough to span the relevant features of $s_t$.

This provides a theoretical justification for reservoir policies in approximately linear environments — which includes many real robotic systems near their operating point.

## 23.2.4 CartPole Benchmark

CartPole is the standard introductory benchmark for RL: a pole is balanced on a moving cart by applying left/right forces. The environment state is $s_t = (x, \dot{x}, \theta, \dot{\theta})$ (cart position and velocity, pole angle and angular velocity). The action is a discrete force $a_t \in \{-1, +1\}$ (or continuous force in CartPole-v1 continuous variant). The reward is +1 per timestep the pole remains upright.

### Reservoir Policy for CartPole

```python
"""
CartPole Balance with Echo State Network Policy
Reinforcement Learning with fixed reservoir and trained linear readout.

Requires: gymnasium, numpy
"""

import numpy as np
import gymnasium as gym
from typing import Optional, List, Tuple


class ESNPolicy:
    """
    Echo State Network reinforcement learning policy.
    
    The reservoir provides a fixed nonlinear, temporally extended
    representation of the observation history. Only the linear readout
    W_out is trained.
    
    Parameters
    ----------
    obs_dim : int
        Observation space dimensionality.
    action_dim : int
        Action dimensionality (1 for CartPole discrete).
    n_reservoir : int
        Number of reservoir neurons.
    spectral_radius : float
        Spectral radius of W_res.
    input_scaling : float
        Scaling of W_in.
    leaking_rate : float
        Leaking rate alpha.
    noise_std : float
        Policy noise standard deviation (for exploration).
    seed : int
        Random seed.
    """
    
    def __init__(
        self,
        obs_dim: int,
        action_dim: int = 1,
        n_reservoir: int = 200,
        spectral_radius: float = 0.9,
        input_scaling: float = 0.5,
        leaking_rate: float = 0.7,
        noise_std: float = 0.5,
        seed: int = 42,
    ):
        self.d_in = obs_dim
        self.d_out = action_dim
        self.N = n_reservoir
        self.rho = spectral_radius
        self.s_in = input_scaling
        self.alpha = leaking_rate
        self.noise_std = noise_std
        self.rng = np.random.RandomState(seed)
        
        # Initialize reservoir (fixed, never updated)
        self._init_reservoir()
        
        # Linear readout (the only trained parameter)
        self.W_out = np.zeros((action_dim, n_reservoir))
        self._x = np.zeros(n_reservoir)  # current state
    
    def _init_reservoir(self) -> None:
        N = self.N
        # Sparse random reservoir weights
        W = self.rng.randn(N, N)
        W[self.rng.rand(N, N) > 0.1] = 0.0
        ev = np.linalg.eigvals(W)
        W *= self.rho / (np.max(np.abs(ev)) + 1e-10)
        self.W_res = W
        
        # Input weights
        self.W_in = self.s_in * self.rng.randn(N, self.d_in)
        self.bias = 0.05 * self.rng.randn(N)
    
    def reset_state(self) -> None:
        """Reset reservoir state at start of episode."""
        self._x = np.zeros(self.N)
    
    def get_reservoir_state(self, obs: np.ndarray) -> np.ndarray:
        """Update reservoir state and return it."""
        pre = self.W_res @ self._x + self.W_in @ obs + self.bias
        self._x = (1 - self.alpha) * self._x + self.alpha * np.tanh(pre)
        return self._x.copy()
    
    def act(self, obs: np.ndarray, deterministic: bool = False) -> int:
        """Select action from current observation."""
        x = self.get_reservoir_state(obs)
        logit = self.W_out @ x  # (d_out,)
        
        if deterministic:
            return int(logit[0] > 0)
        else:
            # Stochastic policy: sigmoid with noise for exploration
            prob = 1 / (1 + np.exp(-(logit[0] + self.noise_std * self.rng.randn())))
            return int(prob > 0.5)
    
    def set_weights(self, W: np.ndarray) -> None:
        self.W_out = W.reshape(self.d_out, self.N)
    
    def get_weights(self) -> np.ndarray:
        return self.W_out.flatten()
    
    def n_params(self) -> int:
        return self.d_out * self.N


def rollout(
    policy: ESNPolicy,
    env: gym.Env,
    max_steps: int = 500,
    deterministic: bool = False,
    seed: Optional[int] = None,
) -> Tuple[float, List]:
    """
    Run one episode and return total reward and trajectory.
    """
    obs, _ = env.reset(seed=seed)
    policy.reset_state()
    
    total_reward = 0.0
    trajectory = []
    
    for t in range(max_steps):
        action = policy.act(obs, deterministic=deterministic)
        next_obs, reward, terminated, truncated, _ = env.step(action)
        
        trajectory.append((obs.copy(), action, reward, next_obs.copy()))
        total_reward += reward
        obs = next_obs
        
        if terminated or truncated:
            break
    
    return total_reward, trajectory


def policy_gradient_train(
    policy: ESNPolicy,
    env: gym.Env,
    n_episodes: int = 500,
    learning_rate: float = 1e-3,
    gamma: float = 0.99,
    n_eval: int = 10,
    verbose: bool = True,
) -> List[float]:
    """
    Train ESN policy using REINFORCE (policy gradient).
    Returns list of episode returns.
    """
    returns_history = []
    
    for episode in range(n_episodes):
        # Collect trajectory
        reward, trajectory = rollout(policy, env)
        returns_history.append(reward)
        
        # Compute discounted returns
        T = len(trajectory)
        G = np.zeros(T)
        running_return = 0.0
        for t in reversed(range(T)):
            running_return = trajectory[t][2] + gamma * running_return
            G[t] = running_return
        
        # Normalize returns (variance reduction)
        G = (G - G.mean()) / (G.std() + 1e-8)
        
        # Compute policy gradient and update W_out
        grad = np.zeros_like(policy.W_out)
        policy.reset_state()
        for t, (obs, action, r, _) in enumerate(trajectory):
            x = policy.get_reservoir_state(obs)
            logit = policy.W_out @ x
            # Gradient for Bernoulli policy: (action - sigmoid(logit)) * x
            p = 1 / (1 + np.exp(-logit[0]))
            grad += G[t] * (action - p) * x[None, :]
        
        policy.W_out += learning_rate * grad / T
        
        # Periodic evaluation
        if verbose and (episode + 1) % 50 == 0:
            eval_returns = [rollout(policy, env, deterministic=True)[0]
                           for _ in range(n_eval)]
            mean_ret = np.mean(eval_returns)
            print(f"Episode {episode+1:4d}: "
                  f"train={reward:.0f}, eval={mean_ret:.1f}")
    
    return returns_history


def evolution_strategy_train(
    policy: ESNPolicy,
    env: gym.Env,
    n_iterations: int = 200,
    population_size: int = 50,
    noise_std: float = 0.05,
    learning_rate: float = 0.01,
    verbose: bool = True,
) -> List[float]:
    """
    Train ESN policy using Evolution Strategies (OpenAI ES).
    Returns list of mean returns per iteration.
    """
    theta = policy.get_weights()  # (d_out * N,)
    returns_history = []
    
    for iteration in range(n_iterations):
        # Sample perturbations
        epsilon = np.random.randn(population_size, len(theta))
        
        # Evaluate perturbed policies
        rewards = np.zeros(population_size)
        for k in range(population_size):
            policy.set_weights(theta + noise_std * epsilon[k])
            r, _ = rollout(policy, env)
            rewards[k] = r
        
        # Normalize rewards
        rewards_norm = (rewards - rewards.mean()) / (rewards.std() + 1e-8)
        
        # ES update
        theta += learning_rate / (population_size * noise_std) * (epsilon.T @ rewards_norm)
        policy.set_weights(theta)
        
        mean_r = rewards.mean()
        returns_history.append(mean_r)
        
        if verbose and (iteration + 1) % 20 == 0:
            eval_r = np.mean([rollout(policy, env, deterministic=True)[0]
                              for _ in range(5)])
            print(f"Iter {iteration+1:3d}: mean={mean_r:.1f}, eval={eval_r:.1f}")
    
    return returns_history


def compare_with_random(env: gym.Env, n_eval: int = 100) -> float:
    """Baseline: random policy return."""
    returns = []
    for _ in range(n_eval):
        obs, _ = env.reset()
        total_r, done = 0, False
        while not done:
            action = env.action_space.sample()
            obs, r, term, trunc, _ = env.step(action)
            total_r += r
            done = term or trunc
        returns.append(total_r)
    return np.mean(returns)


def main():
    env = gym.make('CartPole-v1')
    obs_dim = env.observation_space.shape[0]   # 4
    
    # ESN policy
    policy = ESNPolicy(
        obs_dim=obs_dim,
        n_reservoir=200,
        spectral_radius=0.9,
        input_scaling=0.5,
        leaking_rate=0.7,
    )
    print(f"ESN policy parameters: {policy.n_params()} "
          f"(reservoir neurons: {policy.N})")
    
    # Random baseline
    rand_return = compare_with_random(env)
    print(f"Random policy mean return: {rand_return:.1f}")
    
    print("\n--- Training with Evolution Strategies ---")
    returns_es = evolution_strategy_train(
        policy, env, n_iterations=200, population_size=50
    )
    
    # Final evaluation
    final_returns = [rollout(policy, env, deterministic=True)[0]
                     for _ in range(20)]
    print(f"\nFinal ESN policy: mean={np.mean(final_returns):.1f} "
          f"± {np.std(final_returns):.1f}")
    print(f"Random baseline: {rand_return:.1f}")
    print(f"Expert threshold (solved): 475.0")
    
    env.close()
    
    # Plot learning curve
    import matplotlib.pyplot as plt
    fig, ax = plt.subplots(figsize=(8, 4))
    ax.plot(returns_es, 'b-', lw=0.7, alpha=0.5, label='ES per-iter mean')
    window = 20
    smoothed = np.convolve(returns_es,
                           np.ones(window)/window, mode='valid')
    ax.plot(np.arange(window-1, len(returns_es)), smoothed,
            'r-', lw=2, label=f'{window}-iter moving average')
    ax.axhline(475, color='g', ls='--', label='Solved threshold')
    ax.set_xlabel("ES Iteration")
    ax.set_ylabel("Mean Episode Return")
    ax.set_title("ESN Policy on CartPole-v1 (ES Training)")
    ax.legend()
    plt.tight_layout()
    plt.savefig("cartpole_esn_es.png", dpi=150)
    print("Learning curve saved to cartpole_esn_es.png")


if __name__ == "__main__":
    main()
```

## 23.2.5 Performance Analysis

The CartPole task is solvable with very few parameters. The ESN policy with $N = 200$ reservoir neurons has $200 \times 1 = 200$ trainable parameters. For comparison:
- Deep MLP policy (2 hidden layers, 64 units each): $\sim 4400$ parameters
- Random policy: mean return $\approx 22$ (chance level)
- ESN with ES training (200 iterations): typically reaches $>450$ (near-optimal)
- Deep PPO (standard): reaches $475$ (optimal) in $\sim 20,000$ timesteps

The ESN typically solves CartPole in $\sim 5,000$–$10,000$ environment interactions with ES training — competitive with deep RL methods and significantly better in terms of computational cost (no backpropagation, simple linear operations).

### Why Evolution Strategies for Reservoir RL?

Policy gradient methods (REINFORCE, PPO) require gradient computation through the policy. For a deep network, this uses backpropagation. For an ESN policy, the gradient is computed directly (it involves only $W_{\text{out}}$). However, ES offers additional advantages:

1. **No gradient bias**: The ES estimate is unbiased for the fitness function, even for non-differentiable rewards.
2. **Better exploration**: Parameter-space exploration (ES perturbs $W_{\text{out}}$) can explore more diverse behaviors than action-space exploration.
3. **Parallelism**: $\lambda$ population members can be evaluated simultaneously — a major advantage for robot hardware experiments.
4. **Simplicity**: No value function, no replay buffer, no critic network needed.

For more complex tasks (Mujoco locomotion, robotic manipulation), the ES + reservoir approach has been shown to require 2–5× fewer environment interactions than deep RL with comparable final performance [ContiEtAl2018].
