"""
================================================================================
Lab 13 — Deep Echo State Network: Hierarchical Timescale Processing
================================================================================

Textbook connection
-------------------
Unit 4, Chapter 13: "Deep Reservoir Computing."
Follows Gallicchio, Micheli & Pedrelli (2017, "Deep Echo State Networks",
arXiv:1708.01525) and the analysis in Gallicchio & Micheli (2020).

Learning objectives
-------------------
  * Implement a hierarchical DeepESN by stacking ESN layers, feeding the state
    matrix of layer ℓ as input to layer ℓ+1.
  * Understand how depth creates a timescale hierarchy: fast layers at the
    bottom, slow layers at the top.
  * Verify this by inspecting the power spectrum of each layer's mean state.
  * Compare a 3-layer DeepESN (3×100 neurons) against a single ESN (N=300) on
    a multi-timescale prediction task.

Architecture:
    Layer 1 (α=1.0, ρ=0.9):  fast dynamics → captures rapid changes
    Layer 2 (α=0.3, ρ=0.95): medium timescale
    Layer 3 (α=0.1, ρ=0.99): slow dynamics → captures long-term context

Readout:
    Concatenate states from all 3 layers → ridge regression.

Multi-timescale task:
    signal(t) = sin(2π · 50 Hz · t) · (1 + sin(2π · 5 Hz · t)) / 2
    Target:  the slow modulation envelope  (5 Hz component)
    This requires short-term memory (track fast oscillation) AND
    long-term memory (identify the slow envelope phase).

What to observe
---------------
  * DeepESN power spectra: Layer 1 peak at fast frequency; Layer 3 peak near
    slow frequency. This is the "timescale hierarchy" in action.
  * The DeepESN outperforms the single-layer ESN on envelope prediction
    because its hierarchical memory naturally separates the two timescales.
================================================================================
"""

import os
import sys
from pathlib import Path

import numpy as np
import matplotlib.pyplot as plt

sys.path.insert(0, str(Path(__file__).parent.parent))
from utils import EchoStateNetwork, nmse


# ── Configuration ─────────────────────────────────────────────────────────────

SEED         = 42
FS           = 1000       # sample rate (Hz)
T_TOTAL      = 6000       # total samples
T_TRAIN      = 4000
T_TEST       = 2000
WASHOUT      = 200

# Signal parameters
F_FAST       = 50.0       # fast oscillation (Hz)
F_SLOW       = 5.0        # slow modulation (Hz)

# Deep ESN layers
LAYER_SIZES  = [100, 100, 100]
LEAK_RATES   = [1.0,  0.3,  0.1]
SPEC_RADII   = [0.9,  0.95, 0.99]
RIDGE_ALPHA  = 1e-6

# Single ESN baseline
N_SINGLE     = 300
RHO_SINGLE   = 0.95
ALPHA_SINGLE = 0.5   # intermediate leak rate


# ── Signal generation ─────────────────────────────────────────────────────────

def make_signal(T, fs=FS, f_fast=F_FAST, f_slow=F_SLOW, seed=SEED):
    """
    Amplitude-modulated signal:
        s(t) = sin(2π f_fast t) · (1 + sin(2π f_slow t)) / 2
    Normalised to zero mean, unit std.

    Returns:
        signal   : (T,) raw signal (reservoir input)
        envelope : (T,) slow modulation envelope (prediction target)
        t_sec    : (T,) time in seconds
    """
    t_sec    = np.arange(T) / fs
    fast_osc = np.sin(2 * np.pi * f_fast * t_sec)
    envelope = (1.0 + np.sin(2 * np.pi * f_slow * t_sec)) / 2.0
    signal   = fast_osc * envelope

    # Normalise
    signal   = (signal - signal.mean()) / (signal.std() + 1e-8)
    envelope = (envelope - envelope.mean()) / (envelope.std() + 1e-8)
    return signal, envelope, t_sec


# ── DeepESN implementation ────────────────────────────────────────────────────

class DeepESN:
    """
    Deep Echo State Network: stacked ESN layers where the state of layer ℓ
    feeds as input to layer ℓ+1.

    Parameters
    ----------
    layer_sizes    : list of ints    — reservoir sizes per layer
    spectral_radii : list of floats
    leak_rates     : list of floats
    ridge_alpha    : float
    washout        : int
    seed           : int
    """

    def __init__(self, layer_sizes, spectral_radii, leak_rates,
                 ridge_alpha=1e-6, washout=WASHOUT, seed=SEED):
        self.layer_sizes    = layer_sizes
        self.n_layers       = len(layer_sizes)
        self.ridge_alpha    = ridge_alpha
        self.washout        = washout

        # Build one EchoStateNetwork per layer
        self.layers = []
        for i, (N, rho, alpha) in enumerate(zip(layer_sizes, spectral_radii, leak_rates)):
            esn = EchoStateNetwork(
                n_reservoir=N,
                spectral_radius=rho,
                input_scaling=1.0,
                leak_rate=alpha,
                connectivity=0.1,
                bias_scaling=0.1,
                ridge_alpha=ridge_alpha,
                washout=0,                # we handle washout globally
                extend_with_input=False,  # don't extend; we concatenate all layers
                seed=seed + i * 100,
            )
            self.layers.append(esn)

        self.W_out = None
        self._n_outputs = None

    def _run_all_layers(self, inputs):
        """
        Drive all layers sequentially.
        inputs : (T, n_in) array
        Returns: (T, sum(layer_sizes)) concatenated states
        """
        x = inputs
        all_states = []
        for layer in self.layers:
            # Build weights on first call if needed
            if layer.W_rec is None:
                layer._build_reservoir(x.shape[1])
            states, _ = layer.run(x)     # states : (T, N_layer)
            all_states.append(states)
            x = states                   # feed states to next layer
        return np.concatenate(all_states, axis=1)  # (T, total_N)

    def fit(self, inputs, targets):
        """Train readout via ridge regression on concatenated states."""
        from scipy import linalg

        if targets.ndim == 1:
            targets = targets.reshape(-1, 1)
        self._n_outputs = targets.shape[1]

        X = self._run_all_layers(inputs)[self.washout:]   # (T-washout, total_N)
        Y = targets[self.washout:]

        A = X.T @ X + self.ridge_alpha * np.eye(X.shape[1])
        B = X.T @ Y
        self.W_out = linalg.solve(A, B, assume_a="pos").T   # (n_out, total_N)
        return self

    def predict(self, inputs):
        X = self._run_all_layers(inputs)
        preds = (self.W_out @ X.T).T
        if self._n_outputs == 1:
            return preds.ravel()
        return preds

    def get_layer_states(self, inputs):
        """Return list of (T, N_layer) state arrays, one per layer."""
        x = inputs
        layer_states = []
        for layer in self.layers:
            if layer.W_rec is None:
                layer._build_reservoir(x.shape[1])
            states, _ = layer.run(x)
            layer_states.append(states)
            x = states
        return layer_states


# ── Power spectrum helper ─────────────────────────────────────────────────────

def mean_power_spectrum(states, fs=FS):
    """
    Compute the mean power spectrum across all neurons in a state matrix.
    Returns (freqs, power).
    """
    T, N = states.shape
    # FFT along time axis for each neuron; average power
    F = np.fft.rfft(states - states.mean(axis=0), axis=0)
    power = (np.abs(F) ** 2).mean(axis=1)
    freqs = np.fft.rfftfreq(T, d=1.0 / fs)
    return freqs, power


# ── Main ──────────────────────────────────────────────────────────────────────

def main():
    os.makedirs("output", exist_ok=True)

    # ── Generate multi-timescale signal ───────────────────────────────────────
    print("Generating multi-timescale signal ...")
    signal, envelope, t_sec = make_signal(T_TOTAL)

    u_train = signal[:T_TRAIN].reshape(-1, 1)
    y_train = envelope[:T_TRAIN]
    u_test  = signal[T_TRAIN:].reshape(-1, 1)
    y_test  = envelope[T_TRAIN:]

    # ── Train DeepESN ─────────────────────────────────────────────────────────
    print("=" * 60)
    print("Deep ESN (3 layers × 100 neurons)")
    print("=" * 60)

    deep = DeepESN(
        layer_sizes=LAYER_SIZES,
        spectral_radii=SPEC_RADII,
        leak_rates=LEAK_RATES,
        ridge_alpha=RIDGE_ALPHA,
        washout=WASHOUT,
        seed=SEED,
    )
    deep.fit(u_train, y_train)
    y_pred_deep = deep.predict(u_test)
    nmse_deep   = nmse(y_test, y_pred_deep)
    print(f"  Deep ESN NMSE: {nmse_deep:.4f}")

    # ── Train single ESN ──────────────────────────────────────────────────────
    print("\n" + "=" * 60)
    print(f"Single ESN (N={N_SINGLE}, ρ={RHO_SINGLE}, α={ALPHA_SINGLE})")
    print("=" * 60)

    single = EchoStateNetwork(
        n_reservoir=N_SINGLE,
        spectral_radius=RHO_SINGLE,
        input_scaling=1.0,
        leak_rate=ALPHA_SINGLE,
        ridge_alpha=RIDGE_ALPHA,
        washout=WASHOUT,
        seed=SEED,
    )
    single.fit(u_train, y_train)
    y_pred_single = single.predict(u_test)
    nmse_single   = nmse(y_test, y_pred_single)
    print(f"  Single ESN NMSE: {nmse_single:.4f}")

    print(f"\nDeep ESN improvement: {(nmse_single - nmse_deep) / nmse_single * 100:.1f}%")

    # ── Figure 1: Predictions ─────────────────────────────────────────────────
    t_plot = t_sec[T_TRAIN : T_TRAIN + 500]
    t_slice = slice(0, 500)

    fig, axes = plt.subplots(3, 1, figsize=(13, 9), sharex=True)

    axes[0].plot(t_plot, signal[T_TRAIN : T_TRAIN + 500], color="gray", lw=0.8, alpha=0.6,
                 label="Input signal")
    axes[0].plot(t_plot, y_test[t_slice], color="black", lw=2, label="Envelope (target)")
    axes[0].set_ylabel("Amplitude", fontsize=10)
    axes[0].set_title("(a) Input Signal and Target Envelope (first 500 test steps)", fontsize=10)
    axes[0].legend(fontsize=9)

    axes[1].plot(t_plot, y_test[t_slice], color="black", lw=2, label="Target")
    axes[1].plot(t_plot, y_pred_deep[t_slice], color="steelblue", lw=2, ls="--",
                 label=f"Deep ESN (NMSE={nmse_deep:.3f})")
    axes[1].set_ylabel("Envelope", fontsize=10)
    axes[1].set_title("(b) Deep ESN Prediction", fontsize=10)
    axes[1].legend(fontsize=9)

    axes[2].plot(t_plot, y_test[t_slice], color="black", lw=2, label="Target")
    axes[2].plot(t_plot, y_pred_single[t_slice], color="tomato", lw=2, ls="--",
                 label=f"Single ESN (NMSE={nmse_single:.3f})")
    axes[2].set_ylabel("Envelope", fontsize=10)
    axes[2].set_xlabel("Time (s)", fontsize=11)
    axes[2].set_title("(c) Single ESN Prediction", fontsize=10)
    axes[2].legend(fontsize=9)

    fig.suptitle(
        f"Multi-Timescale Task: Slow Envelope Prediction\n"
        f"f_fast={F_FAST:.0f} Hz · f_slow={F_SLOW:.0f} Hz · fs={FS} Hz",
        fontsize=12
    )
    plt.tight_layout()
    plt.savefig("output/13_deep_esn_predictions.png", dpi=150)
    plt.close()
    print("\nFigure saved: output/13_deep_esn_predictions.png")

    # ── Figure 2: Power spectrum of each layer's states ────────────────────────
    print("\nComputing layer power spectra ...")
    layer_states = deep.get_layer_states(u_train)

    fig, ax = plt.subplots(figsize=(10, 5))
    layer_colors = ["steelblue", "darkorange", "tomato"]
    layer_alphas = [1.0, 0.3, 0.1]

    # Also plot input spectrum for reference
    input_freqs = np.fft.rfftfreq(len(signal[:T_TRAIN]), d=1.0 / FS)
    input_fft   = np.abs(np.fft.rfft(signal[:T_TRAIN])) ** 2
    ax.semilogy(input_freqs, input_fft / input_fft.max() + 1e-10,
                color="gray", lw=1, alpha=0.4, label="Input signal")

    for li, (states, color, lr) in enumerate(zip(layer_states, layer_colors, LEAK_RATES)):
        freqs, power = mean_power_spectrum(states, fs=FS)
        power_norm   = power / (power.max() + 1e-10)
        ax.semilogy(freqs, power_norm + 1e-10, color=color, lw=2,
                    label=f"Layer {li+1}  (α={lr}, ρ={SPEC_RADII[li]})")

    ax.axvline(F_FAST, color="black", ls="--", lw=1.5, label=f"f_fast = {F_FAST:.0f} Hz")
    ax.axvline(F_SLOW, color="purple", ls=":", lw=1.5, label=f"f_slow = {F_SLOW:.0f} Hz")
    ax.set_xlabel("Frequency (Hz)", fontsize=11)
    ax.set_ylabel("Normalised power (log)", fontsize=11)
    ax.set_title("Power Spectrum of Mean Reservoir States per Layer\n"
                 "Layer 1 → fast; Layer 3 → slow (timescale hierarchy)", fontsize=11)
    ax.set_xlim(0, FS // 2)
    ax.legend(fontsize=9)
    plt.tight_layout()
    plt.savefig("output/13_layer_power_spectra.png", dpi=150)
    plt.close()
    print("Figure saved: output/13_layer_power_spectra.png")

    # ── Peak frequency per layer ───────────────────────────────────────────────
    print("\n" + "=" * 60)
    print("Layer Power Spectrum Analysis")
    print("=" * 60)
    for li, (states, lr) in enumerate(zip(layer_states, LEAK_RATES)):
        freqs, power = mean_power_spectrum(states, fs=FS)
        # Find dominant frequency above 1 Hz to avoid DC
        idx_above = freqs > 1.0
        peak_freq = freqs[idx_above][np.argmax(power[idx_above])]
        print(f"  Layer {li+1} (α={lr}):  dominant frequency ≈ {peak_freq:.1f} Hz")

    print(f"\n  Target frequencies: f_fast={F_FAST:.0f} Hz, f_slow={F_SLOW:.0f} Hz")
    print(
        "\nObservation: The timescale hierarchy is reflected in the frequency content.\n"
        "Layer 1 (fast) captures the oscillation at f_fast.\n"
        "Layer 3 (slow) preserves only the low-frequency modulation.\n"
        "Concatenating all layers gives the readout access to both timescales."
    )


if __name__ == "__main__":
    main()
