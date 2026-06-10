"""
Echo State Network implementation following Jaeger (2001) and
Lukoševičius (2012, "A Practical Guide to Applying Echo State Networks").

The leaky integrator ESN:
    x_t = (1 - α) x_{t-1} + α · tanh(W_rec x_{t-1} + W_in u_t + b)
    y_t = W_out · [x_t; u_t]        (extended state, if extend=True)

Training: ridge regression
    W_out = Y_target · X^T · (X · X^T + λ I)^{-1}

where X is the state matrix (columns = time steps, rows = neurons + inputs).
"""

import numpy as np
from numpy.random import default_rng
from scipy import linalg


class EchoStateNetwork:
    """
    Leaky integrator Echo State Network with ridge regression readout.

    Parameters
    ----------
    n_reservoir : int
        Number of reservoir neurons N.
    spectral_radius : float
        Target spectral radius ρ of W_rec. Controls memory/stability.
        Rule of thumb: ρ ≈ 0.9 for most tasks; ρ < 1 is sufficient for ESP.
    input_scaling : float
        Scales W_in. Larger values push neurons into the nonlinear regime.
    leak_rate : float
        α ∈ (0, 1]. Leak rate of the leaky integrator.
        α = 1: standard discrete-time ESN (no leaking).
        α → 0: slow, low-pass filtered dynamics.
    connectivity : float
        Fraction of non-zero entries in W_rec.
    input_connectivity : float
        Fraction of reservoir neurons that receive each input dimension.
    bias_scaling : float
        Scale of the bias vector b.
    ridge_alpha : float
        Ridge regression regularisation λ. Try values in [1e-8, 1.0].
    washout : int
        Number of initial time steps to discard (initial condition forgetting).
    extend_with_input : bool
        If True, append the input u_t to the state vector before readout.
        This adds direct input-output connections, which often helps.
    seed : int or None
        Random seed for reproducibility.
    """

    def __init__(
        self,
        n_reservoir: int = 100,
        spectral_radius: float = 0.95,
        input_scaling: float = 1.0,
        leak_rate: float = 1.0,
        connectivity: float = 0.1,
        input_connectivity: float = 1.0,
        bias_scaling: float = 0.1,
        ridge_alpha: float = 1e-6,
        washout: int = 100,
        extend_with_input: bool = True,
        seed: int = 42,
    ):
        self.n_reservoir = n_reservoir
        self.spectral_radius = spectral_radius
        self.input_scaling = input_scaling
        self.leak_rate = leak_rate
        self.connectivity = connectivity
        self.input_connectivity = input_connectivity
        self.bias_scaling = bias_scaling
        self.ridge_alpha = ridge_alpha
        self.washout = washout
        self.extend_with_input = extend_with_input
        self.rng = default_rng(seed)

        self.W_rec = None   # N × N
        self.W_in = None    # N × n_inputs
        self.b = None       # N
        self.W_out = None   # n_outputs × (N [+ n_inputs])

        self._n_inputs = None
        self._n_outputs = None

    # ------------------------------------------------------------------
    # Reservoir construction
    # ------------------------------------------------------------------

    def _build_reservoir(self, n_inputs: int):
        N = self.n_reservoir

        # Recurrent weight matrix: sparse Gaussian, then rescale to ρ
        mask = self.rng.random((N, N)) < self.connectivity
        W = self.rng.standard_normal((N, N)) * mask
        rho_W = np.max(np.abs(np.linalg.eigvals(W)))
        if rho_W < 1e-10:
            raise RuntimeError(
                "Reservoir weight matrix has near-zero spectral radius. "
                "Increase connectivity or n_reservoir."
            )
        self.W_rec = (self.spectral_radius / rho_W) * W

        # Input weights: each input dimension connects to a random subset
        mask_in = self.rng.random((N, n_inputs)) < self.input_connectivity
        self.W_in = (
            self.rng.uniform(-1, 1, (N, n_inputs)) * mask_in * self.input_scaling
        )

        # Bias
        self.b = self.rng.uniform(-1, 1, N) * self.bias_scaling

        self._n_inputs = n_inputs

    # ------------------------------------------------------------------
    # Forward pass
    # ------------------------------------------------------------------

    def _step(self, x: np.ndarray, u: np.ndarray) -> np.ndarray:
        """One reservoir update step. Returns new state x_t."""
        pre = self.W_rec @ x + self.W_in @ u + self.b
        return (1.0 - self.leak_rate) * x + self.leak_rate * np.tanh(pre)

    def run(
        self,
        inputs: np.ndarray,
        initial_state: np.ndarray | None = None,
    ) -> tuple[np.ndarray, np.ndarray]:
        """
        Drive the reservoir with an input sequence.

        Parameters
        ----------
        inputs : (T, n_inputs) array
        initial_state : (N,) array or None

        Returns
        -------
        states : (T, N) array — reservoir states at each time step
        extended : (T, N + n_inputs) array — states concatenated with inputs
        """
        T, n_inputs = inputs.shape
        if self.W_rec is None or n_inputs != self._n_inputs:
            self._build_reservoir(n_inputs)

        x = np.zeros(self.n_reservoir) if initial_state is None else initial_state.copy()
        states = np.empty((T, self.n_reservoir))

        for t in range(T):
            x = self._step(x, inputs[t])
            states[t] = x

        if self.extend_with_input:
            extended = np.concatenate([states, inputs], axis=1)
        else:
            extended = states

        return states, extended

    # ------------------------------------------------------------------
    # Training
    # ------------------------------------------------------------------

    def fit(self, inputs: np.ndarray, targets: np.ndarray) -> "EchoStateNetwork":
        """
        Train the readout weights W_out via ridge regression.

        Parameters
        ----------
        inputs  : (T, n_inputs)  array
        targets : (T, n_outputs) array or (T,) for single output

        Returns
        -------
        self
        """
        if targets.ndim == 1:
            targets = targets.reshape(-1, 1)
        self._n_outputs = targets.shape[1]

        _, X = self.run(inputs)          # X : (T, N + n_inputs)
        X = X[self.washout:]             # discard washout
        Y = targets[self.washout:]

        # Ridge regression: W_out = Y^T X (X^T X + λI)^{-1}
        # Equivalently solve: (X^T X + λI) W_out^T = X^T Y
        A = X.T @ X + self.ridge_alpha * np.eye(X.shape[1])
        B = X.T @ Y
        self.W_out = linalg.solve(A, B, assume_a="pos").T  # (n_outputs, features)

        return self

    # ------------------------------------------------------------------
    # Prediction
    # ------------------------------------------------------------------

    def predict(
        self,
        inputs: np.ndarray,
        initial_state: np.ndarray | None = None,
    ) -> np.ndarray:
        """
        Generate predictions for an input sequence.

        Parameters
        ----------
        inputs : (T, n_inputs) array
        initial_state : (N,) or None

        Returns
        -------
        predictions : (T, n_outputs) array
        """
        if self.W_out is None:
            raise RuntimeError("Call fit() before predict().")
        _, X = self.run(inputs, initial_state)
        predictions = (self.W_out @ X.T).T  # (T, n_outputs)
        if self._n_outputs == 1:
            return predictions.ravel()
        return predictions

    def fit_predict(
        self,
        train_inputs: np.ndarray,
        train_targets: np.ndarray,
        test_inputs: np.ndarray,
    ) -> np.ndarray:
        """Convenience: fit on train, return predictions on test."""
        self.fit(train_inputs, train_targets)
        return self.predict(test_inputs)

    # ------------------------------------------------------------------
    # Utilities
    # ------------------------------------------------------------------

    @property
    def actual_spectral_radius(self) -> float:
        """Measured ρ(W_rec) after construction."""
        if self.W_rec is None:
            return float("nan")
        return float(np.max(np.abs(np.linalg.eigvals(self.W_rec))))

    def echo_state_property_test(
        self,
        inputs: np.ndarray,
        n_trials: int = 5,
    ) -> tuple[bool, float]:
        """
        Empirically test the Echo State Property.

        Drive the reservoir with `inputs` starting from `n_trials` different
        random initial states. Measure the maximum final-state distance.
        If all trajectories converge (distance → 0), ESP holds.

        Returns
        -------
        esp_holds : bool   — True if max distance < 1e-4
        max_distance : float
        """
        T = inputs.shape[0]
        if self.W_rec is None:
            self._build_reservoir(inputs.shape[1])

        final_states = []
        for _ in range(n_trials):
            x0 = self.rng.standard_normal(self.n_reservoir) * 0.5
            states, _ = self.run(inputs, initial_state=x0)
            final_states.append(states[-1])

        dists = [
            np.linalg.norm(final_states[0] - final_states[i])
            for i in range(1, n_trials)
        ]
        max_dist = max(dists)
        return max_dist < 1e-4, max_dist

    def __repr__(self) -> str:
        return (
            f"EchoStateNetwork(N={self.n_reservoir}, "
            f"ρ={self.spectral_radius}, α={self.leak_rate}, "
            f"λ={self.ridge_alpha})"
        )


# ------------------------------------------------------------------
# Online / RLS readout trainer
# ------------------------------------------------------------------

class RLSReadout:
    """
    Online readout training via Recursive Least Squares (RLS).

    The RLS update (Sherman-Morrison rank-1 update):
        k_t     = P_{t-1} x_t / (1 + x_t^T P_{t-1} x_t)
        w_t     = w_{t-1} + k_t (y*_t - x_t^T w_{t-1})
        P_t     = P_{t-1} - k_t x_t^T P_{t-1}

    This is O(N^2) per step and converges to the ridge regression solution.

    Reference: Williams & Zipser (1989); Haykin (2002, "Adaptive Filter Theory").
    """

    def __init__(self, n_features: int, n_outputs: int, delta: float = 1.0):
        """
        Parameters
        ----------
        n_features : int     dimension of the state vector
        n_outputs  : int     number of output dimensions
        delta      : float   initial value of diagonal of P (= 1/λ_ridge initially)
        """
        self.w = np.zeros((n_features, n_outputs))
        self.P = np.eye(n_features) * delta  # covariance estimate

    def step(self, x: np.ndarray, y_target: np.ndarray) -> np.ndarray:
        """
        One RLS update. Returns current prediction before the update.

        Parameters
        ----------
        x        : (n_features,) state vector
        y_target : (n_outputs,) or scalar

        Returns
        -------
        y_pred : (n_outputs,) prediction
        """
        y_pred = self.w.T @ x

        # Kalman gain
        Px = self.P @ x
        k = Px / (1.0 + x @ Px)

        # Weight and covariance update
        error = np.atleast_1d(y_target) - y_pred
        self.w += np.outer(k, error)
        self.P -= np.outer(k, Px)

        return y_pred
