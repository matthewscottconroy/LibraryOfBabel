"""
Molecular Dynamics: 2D Lennard-Jones Particles
================================================
Tier 4.3 — Computational Tools: Molecular Dynamics

Concepts demonstrated:
  - Lennard-Jones potential: V(r) = 4ε[(σ/r)¹² - (σ/r)⁶]
  - Forces from the potential gradient
  - Velocity Verlet integrator (time-reversible, symplectic)
  - Energy conservation: kinetic + potential ≈ constant
  - Thermostat: velocity rescaling (Berendsen proxy)
  - Radial distribution function g(r) (structural order)
  - Phases: gas vs. liquid vs. solid (crystal) by density/temperature
  - Periodic boundary conditions

Usage:
    python 21_molecular_dynamics_toy.py [--phase PHASE] [--n N] [--steps STEPS]

Phases:
    gas     — Low density, high temperature
    liquid  — Medium density, medium temperature
    solid   — High density, low temperature

Key insight:
    MD integrates Newton's equations for all particles simultaneously.
    The Lennard-Jones potential captures the essential physics: short-range
    repulsion (steric clash, r⁻¹²) and long-range attraction (London
    dispersion, r⁻⁶). Real force fields add electrostatics, bonds, angles,
    and dihedrals — the same principles but more interaction types.
"""

import argparse
import numpy as np
import matplotlib.pyplot as plt
import matplotlib.gridspec as gridspec
import matplotlib.animation as animation
from matplotlib.collections import CircleCollection


# ── LJ Potential and Forces ───────────────────────────────────────────────────

def lj_potential_force(r2, eps=1.0, sigma=1.0, cutoff=2.5):
    """
    Returns (V, F_magnitude / r) for use as F = f * dr_vec
    r2 = squared distance between particles.
    cutoff: in units of sigma.
    """
    sigma2 = sigma**2
    rc2 = (cutoff * sigma)**2
    if r2 > rc2 or r2 < 1e-20:
        return 0.0, 0.0

    r2_inv = sigma2 / r2
    r6_inv = r2_inv**3
    r12_inv = r6_inv**2

    V = 4 * eps * (r12_inv - r6_inv)
    # dV/dr = 4ε(-12r^-13 + 6r^-7) * sigma^(12 or 6)
    # F_scalar = -dV/dr * (1/r) = factor for F = factor * r_vec
    dV_dr2 = 4 * eps * (-12 * r12_inv + 6 * r6_inv) / r2
    return V, -dV_dr2   # F = -∇V, factor multiplied by r_vec


# ── Simulation ────────────────────────────────────────────────────────────────

class LJSimulation:
    def __init__(self, n_particles=36, box_length=8.0, temperature=1.0, seed=42):
        self.N = n_particles
        self.L = box_length
        self.T_target = temperature
        self.eps = 1.0
        self.sigma = 1.0
        self.cutoff = 2.5
        self.m = 1.0
        self.dt = 0.005

        rng = np.random.default_rng(seed)

        # Initialize on a grid
        n_side = int(np.ceil(np.sqrt(n_particles)))
        spacing = box_length / (n_side + 1)
        xs = np.linspace(spacing, box_length - spacing, n_side)
        pos_list = [(x, y) for x in xs for y in xs][:n_particles]
        self.pos = np.array(pos_list[:n_particles], dtype=float)

        # Random velocities at target temperature
        vel = rng.standard_normal((n_particles, 2))
        vel -= vel.mean(axis=0)  # zero center of mass velocity
        ke = 0.5 * self.m * (vel**2).sum()
        # KE = n_dof/2 * kT, n_dof = 2N for 2D
        ke_target = n_particles * temperature   # kT per particle in 2D
        scale = np.sqrt(ke_target / ke)
        self.vel = vel * scale

        self.force = np.zeros_like(self.pos)
        self._compute_forces()

        # Trajectory storage
        self.t_hist = [0.0]
        self.ke_hist = [self._kinetic_energy()]
        self.pe_hist = [self._potential_energy()]
        self.T_hist  = [self._temperature()]
        self.pos_snapshots = [self.pos.copy()]

    def _pbc(self, dr):
        """Minimum image convention."""
        dr -= np.round(dr / self.L) * self.L
        return dr

    def _compute_forces(self):
        self.force[:] = 0.0
        self._pe = 0.0
        for i in range(self.N):
            for j in range(i + 1, self.N):
                dr = self._pbc(self.pos[i] - self.pos[j])
                r2 = (dr**2).sum()
                V, f = lj_potential_force(r2, self.eps, self.sigma, self.cutoff)
                self._pe += V
                fvec = f * dr
                self.force[i] += fvec
                self.force[j] -= fvec

    def _kinetic_energy(self):
        return 0.5 * self.m * (self.vel**2).sum()

    def _potential_energy(self):
        return self._pe

    def _temperature(self):
        ke = self._kinetic_energy()
        return ke / self.N   # T = KE/(N) in 2D (kB=1, kT per particle)

    def _velocity_rescale(self):
        """Berendsen-like thermostat: rescale velocities toward target T."""
        T_curr = self._temperature()
        if T_curr > 0:
            scale = np.sqrt(self.T_target / T_curr)
            self.vel *= scale

    def step(self, thermostat=True, thermostat_freq=10):
        """Velocity Verlet integration."""
        # Half-step velocity
        self.vel += 0.5 * self.dt / self.m * self.force
        # Full-step position (with PBC)
        self.pos += self.dt * self.vel
        self.pos %= self.L   # periodic boundary conditions
        # Recompute forces
        self._compute_forces()
        # Full-step velocity
        self.vel += 0.5 * self.dt / self.m * self.force

        if thermostat and (len(self.t_hist) % thermostat_freq == 0):
            self._velocity_rescale()

    def run(self, n_steps=2000, save_every=50, thermostat=True):
        for step_i in range(n_steps):
            self.step(thermostat=thermostat)
            if (step_i + 1) % save_every == 0:
                t = (step_i + 1) * self.dt
                self.t_hist.append(t)
                self.ke_hist.append(self._kinetic_energy())
                self.pe_hist.append(self._potential_energy())
                self.T_hist.append(self._temperature())
                self.pos_snapshots.append(self.pos.copy())

    def radial_distribution(self, n_bins=50, r_max=None):
        """Compute g(r) from last 30% of trajectory."""
        if r_max is None:
            r_max = self.L / 2
        bins = np.linspace(0, r_max, n_bins + 1)
        dr = bins[1] - bins[0]
        hist = np.zeros(n_bins)

        snapshots = self.pos_snapshots[int(0.7 * len(self.pos_snapshots)):]
        for snap in snapshots:
            for i in range(self.N):
                for j in range(i + 1, self.N):
                    dr_vec = self._pbc(snap[i] - snap[j])
                    r = np.sqrt((dr_vec**2).sum())
                    if r < r_max:
                        k = int(r / dr)
                        if k < n_bins:
                            hist[k] += 2   # count both i→j and j→i

        r_vals = (bins[:-1] + bins[1:]) / 2
        # Normalize: ideal gas density
        n_pairs = self.N * (self.N - 1) / 2
        n_snaps = len(snapshots) if snapshots else 1
        rho = self.N / self.L**2   # 2D number density
        # 2D: area of annulus = 2πr dr
        area = 2 * np.pi * r_vals * dr
        g_r = hist / (n_pairs * n_snaps * rho * area + 1e-12) * self.N / 2
        return r_vals, g_r


# ── Phase Presets ─────────────────────────────────────────────────────────────

PHASES = {
    'gas':    {'n': 25, 'box': 12.0, 'T': 2.0, 'steps': 1000},
    'liquid': {'n': 25, 'box': 7.0,  'T': 1.0, 'steps': 2000},
    'solid':  {'n': 25, 'box': 5.5,  'T': 0.3, 'steps': 2000},
}


# ── Main Visualisation ────────────────────────────────────────────────────────

def run(phase='liquid', n=None, steps=None):
    preset = PHASES[phase]
    N = n or preset['n']
    n_steps = steps or preset['steps']
    T = preset['T']
    L = preset['box']

    print(f"Running LJ MD: phase={phase}, N={N}, T={T}, L={L}, steps={n_steps}")
    sim = LJSimulation(n_particles=N, box_length=L, temperature=T, seed=42)
    sim.run(n_steps=n_steps, save_every=20, thermostat=True)

    r_vals, g_r = sim.radial_distribution()

    fig = plt.figure(figsize=(18, 10))
    fig.patch.set_facecolor('#1a1a2e')
    fig.suptitle(f'Molecular Dynamics — LJ Particles  '
                 f'(phase: {phase}, N={N}, T={T:.2f}, ρ={N/L**2:.3f})',
                 color='white', fontsize=12, fontweight='bold')

    gs = gridspec.GridSpec(2, 3, figure=fig,
                           left=0.05, right=0.97, top=0.88, bottom=0.07,
                           hspace=0.50, wspace=0.35)

    ax_snap = fig.add_subplot(gs[:, 0])
    ax_e    = fig.add_subplot(gs[0, 1])
    ax_temp = fig.add_subplot(gs[1, 1])
    ax_gr   = fig.add_subplot(gs[0, 2])
    ax_info = fig.add_subplot(gs[1, 2])

    for ax in [ax_snap, ax_e, ax_temp, ax_gr, ax_info]:
        ax.set_facecolor('#0f3460')

    # ── Final snapshot ────────────────────────────────────────────────────────
    final_pos = sim.pos_snapshots[-1]
    speeds = np.sqrt((sim.vel**2).sum(axis=1))
    ax_snap.scatter(final_pos[:, 0], final_pos[:, 1],
                    c=speeds, cmap='plasma', s=250, alpha=0.9,
                    edgecolors='white', linewidths=0.5, zorder=4)
    ax_snap.set_xlim(0, L)
    ax_snap.set_ylim(0, L)
    ax_snap.set_aspect('equal')
    ax_snap.set_xlabel('x (σ)', color='white')
    ax_snap.set_ylabel('y (σ)', color='white')
    ax_snap.set_title(f'Final Configuration\n(color = speed)', color='white', fontsize=9)
    ax_snap.tick_params(colors='white')
    # Draw LJ radius reference
    for px, py in final_pos[:5]:
        circle = plt.Circle((px, py), 0.5, color='white', fill=False,
                             alpha=0.15, lw=0.5)
        ax_snap.add_patch(circle)
    for sp in ax_snap.spines.values():
        sp.set_edgecolor('#888')

    # ── Energy time series ────────────────────────────────────────────────────
    t = np.array(sim.t_hist)
    ke = np.array(sim.ke_hist)
    pe = np.array(sim.pe_hist)
    te = ke + pe
    ax_e.plot(t, ke, color='#50fa7b', lw=1.5, label='KE')
    ax_e.plot(t, pe, color='#ff79c6', lw=1.5, label='PE')
    ax_e.plot(t, te, color='#f5a623', lw=2.0, label='Total E', zorder=5)
    ax_e.set_xlabel('Time (τ)', color='white')
    ax_e.set_ylabel('Energy (ε)', color='white')
    ax_e.set_title('Energy Conservation', color='white', fontsize=9)
    ax_e.tick_params(colors='white')
    ax_e.legend(fontsize=8, facecolor='#16213e', labelcolor='white')
    for sp in ax_e.spines.values():
        sp.set_edgecolor('#888')

    # ── Temperature trajectory ────────────────────────────────────────────────
    ax_temp.plot(t, np.array(sim.T_hist), color='#8be9fd', lw=1.5, label='T(t)')
    ax_temp.axhline(T, color='#f5a623', lw=1.5, ls='--', label=f'T_target={T}')
    ax_temp.set_xlabel('Time (τ)', color='white')
    ax_temp.set_ylabel('Temperature (kBT/ε)', color='white')
    ax_temp.set_title('Temperature (thermostat active)', color='white', fontsize=9)
    ax_temp.tick_params(colors='white')
    ax_temp.legend(fontsize=8, facecolor='#16213e', labelcolor='white')
    for sp in ax_temp.spines.values():
        sp.set_edgecolor('#888')

    # ── Radial distribution function ──────────────────────────────────────────
    ax_gr.plot(r_vals / sim.sigma, g_r, color='#ff79c6', lw=2.0)
    ax_gr.axhline(1.0, color='#888', lw=0.8, ls='--', label='Ideal gas g(r)=1')
    ax_gr.axvline(2**(1/6), color='#f5a623', lw=1, ls=':', alpha=0.7,
                  label=f'LJ min: r*={2**(1/6):.2f}σ')
    ax_gr.set_xlabel('r/σ', color='white')
    ax_gr.set_ylabel('g(r)', color='white')
    ax_gr.set_title('Radial Distribution Function', color='white', fontsize=9)
    ax_gr.set_xlim(0, min(L / 2, 5))
    ax_gr.tick_params(colors='white')
    ax_gr.legend(fontsize=7, facecolor='#16213e', labelcolor='white')
    for sp in ax_gr.spines.values():
        sp.set_edgecolor('#888')

    # ── Info panel ────────────────────────────────────────────────────────────
    ax_info.axis('off')
    e_drift = (te[-1] - te[0]) / abs(te[0]) if len(te) > 1 and te[0] != 0 else 0
    peak_gr = g_r.max()
    peak_r  = r_vals[np.argmax(g_r)] / sim.sigma

    info = (
        f"Phase: {phase}\n"
        f"N = {N}\n"
        f"Box = {L}σ × {L}σ\n"
        f"ρ = {N/L**2:.4f} σ⁻²\n"
        f"T_target = {T} ε/kB\n"
        f"dt = {sim.dt} τ\n"
        f"Steps: {n_steps}\n\n"
        f"Energy:\n"
        f"  KE final = {ke[-1]:.3f} ε\n"
        f"  PE final = {pe[-1]:.3f} ε\n"
        f"  E drift  = {e_drift:.2%}\n\n"
        f"g(r) peak: {peak_gr:.2f} at r={peak_r:.2f}σ\n\n"
        f"LJ potential:\n"
        f"  V(r) = 4ε[(σ/r)¹² - (σ/r)⁶]\n"
        f"  Min at r* = 2^(1/6)σ ≈ 1.12σ\n"
        f"  ε = well depth\n\n"
        f"Phase guide:\n"
        f"  Gas: g(r)≈1 everywhere\n"
        f"  Liquid: first peak, then ≈1\n"
        f"  Solid: sharp multiple peaks"
    )
    ax_info.text(0.04, 0.97, info, transform=ax_info.transAxes,
                 va='top', fontsize=7.5, color='white', fontfamily='monospace',
                 linespacing=1.4)

    plt.show()

    print(f"\nPhase: {phase}")
    print(f"N={N}, T={T}, L={L}, density={N/L**2:.4f}")
    print(f"Energy drift: {e_drift:.3%}")
    print(f"g(r) first peak: {peak_gr:.2f} at r={peak_r:.2f}σ")


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='2D Lennard-Jones MD')
    parser.add_argument('--phase', default='liquid',
                        choices=['gas', 'liquid', 'solid'])
    parser.add_argument('--n', type=int, default=None)
    parser.add_argument('--steps', type=int, default=None)
    args = parser.parse_args()

    print("Molecular Dynamics: 2D Lennard-Jones Particles")
    print("=" * 55)
    run(phase=args.phase, n=args.n, steps=args.steps)
