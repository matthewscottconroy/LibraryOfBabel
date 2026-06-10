"""
Standard metrics for reservoir computing experiments.
"""

import numpy as np


def nmse(y_true: np.ndarray, y_pred: np.ndarray) -> float:
    """
    Normalised Mean Squared Error.
        NMSE = MSE(y_true, y_pred) / Var(y_true)

    A perfect predictor gives NMSE = 0. A predictor equal to the mean gives
    NMSE = 1. Values < 0.01 are considered good for most RC benchmarks.
    """
    return float(np.mean((y_true - y_pred) ** 2) / np.var(y_true))


def nrmse(y_true: np.ndarray, y_pred: np.ndarray) -> float:
    """Normalised Root Mean Squared Error = sqrt(NMSE)."""
    return float(np.sqrt(nmse(y_true, y_pred)))


def valid_prediction_time(
    y_true: np.ndarray,
    y_pred: np.ndarray,
    threshold: float = 0.05,
    dt: float = 1.0,
) -> float:
    """
    Valid Prediction Time (VPT) in time units.

    The VPT is the first time t where the normalised error exceeds `threshold`:
        error_t = ||y_true_t - y_pred_t||^2 / Var(y_true)

    Used in chaos prediction benchmarks (Jaeger & Haas 2004, Pathak et al. 2018).
    A VPT of k Lyapunov times means the reservoir predicted k times longer than
    a naive persistence model.

    Parameters
    ----------
    threshold : default 0.05  (5% of signal variance, standard choice)
    dt        : time step in physical units

    Returns
    -------
    vpt : float   in units of dt
    """
    if y_true.ndim == 1:
        y_true = y_true.reshape(-1, 1)
        y_pred = y_pred.reshape(-1, 1)

    var = np.var(y_true, axis=0).mean()
    err = np.mean((y_true - y_pred) ** 2, axis=1) / var

    idx = np.where(err > threshold)[0]
    if len(idx) == 0:
        return float(len(y_true)) * dt
    return float(idx[0]) * dt


def memory_capacity(
    esn,
    T_train: int = 3_000,
    T_test: int = 1_000,
    max_delay: int | None = None,
    seed: int = 42,
) -> tuple[float, np.ndarray]:
    """
    Linear Memory Capacity (MC) as defined by Jaeger (2002).

    MC = sum_{k=1}^{inf} MC_k
    where MC_k = Cov(y_k, u)^2 / (Var(u) Var(y_k))

    and y_k is the reservoir's output when trained to recall u_{t-k}.

    The total MC is bounded above by N (the reservoir size).

    Parameters
    ----------
    esn     : a fitted or unfitted EchoStateNetwork
    T_train : training length for each delay
    T_test  : test length for each delay
    max_delay : maximum delay to test (default: 2 * N)

    Returns
    -------
    total_mc : float          sum of all MC_k values
    mc_per_delay : (max_delay,) array of MC_k values
    """
    from numpy.random import default_rng
    from .esn import EchoStateNetwork
    import copy

    if max_delay is None:
        max_delay = 2 * esn.n_reservoir

    rng = default_rng(seed)
    T = T_train + T_test + max_delay
    u = rng.uniform(-0.8, 0.8, (T, 1))

    mc_per_delay = np.zeros(max_delay)

    for k in range(1, max_delay + 1):
        target_full = np.roll(u[:, 0], k)  # u_{t-k}

        u_train = u[:T_train]
        y_train = target_full[:T_train]

        u_test = u[T_train : T_train + T_test]
        y_test = target_full[T_train : T_train + T_test]

        esn_k = copy.deepcopy(esn)
        esn_k.ridge_alpha = 1e-8
        esn_k.washout = min(100, T_train // 10)

        try:
            esn_k.fit(u_train, y_train)
            y_pred = esn_k.predict(u_test)

            cov = np.cov(y_test, y_pred)[0, 1]
            mc_k = cov**2 / (np.var(y_test) * np.var(y_pred) + 1e-15)
            mc_per_delay[k - 1] = max(0.0, mc_k)
        except Exception:
            mc_per_delay[k - 1] = 0.0

    return float(mc_per_delay.sum()), mc_per_delay


def information_processing_capacity(
    esn,
    T_train: int = 3_000,
    T_test: int = 1_000,
    max_delay: int | None = None,
    seed: int = 42,
) -> dict:
    """
    Total Information Processing Capacity (Dambre et al. 2012).

    Decomposes total capacity into linear MC (k=1 terms) and nonlinear
    contributions (k≥2 Legendre polynomial targets).

    Returns a dict with keys: 'total', 'linear', 'nonlinear', 'per_delay'.
    """
    from numpy.polynomial import legendre as L
    import copy

    if max_delay is None:
        max_delay = esn.n_reservoir

    rng = np.random.default_rng(seed)
    T = T_train + T_test + max_delay + 10
    u_raw = rng.uniform(-1.0, 1.0, T)

    total_cap = 0.0
    linear_cap = 0.0
    nonlinear_cap = 0.0
    results = {}

    for k in range(1, max_delay + 1):
        for degree in (1, 2, 3):
            # Target: Legendre polynomial of u_{t-k}
            u_delayed = np.roll(u_raw, k)
            target = L.legval(u_delayed, [0] * degree + [1])

            u_in = u_raw[max_delay:max_delay + T_train + T_test].reshape(-1, 1)
            t_in = target[max_delay:max_delay + T_train + T_test]

            esn_c = copy.deepcopy(esn)
            esn_c.ridge_alpha = 1e-8
            esn_c.washout = min(100, T_train // 10)
            try:
                esn_c.fit(u_in[:T_train], t_in[:T_train])
                y_pred = esn_c.predict(u_in[T_train:])
                y_true = t_in[T_train:]
                r2 = 1.0 - nmse(y_true, y_pred)
                c = max(0.0, r2)
            except Exception:
                c = 0.0

            total_cap += c
            if degree == 1:
                linear_cap += c
            else:
                nonlinear_cap += c

            results[(k, degree)] = c

    return {
        "total": total_cap,
        "linear": linear_cap,
        "nonlinear": nonlinear_cap,
        "per_degree_delay": results,
    }
