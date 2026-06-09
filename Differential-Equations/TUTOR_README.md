# Math Tutor — Adaptive Problem Trainer

## Quick start

```bash
# First time only — activate the venv or install deps:
.venv/bin/python tutor.py

# Or install system-wide and run directly:
pip install sympy rich
python tutor.py
```

## Commands

| Command | Action |
|---|---|
| `python tutor.py` | Start a 20-problem session |
| `python tutor.py --stats` | Show full mastery dashboard |
| `python tutor.py --reset` | Wipe progress and start over |

## During a session

| Input | Action |
|---|---|
| Your answer | Submit (symbolic, multiple choice, or T/F) |
| `hint` | Show a hint (counted against ELO gain) |
| `skip` | Skip the problem (ELO penalty) |
| `quit` | End session early |

## Answer syntax (symbolic problems)

| Math | Type this |
|---|---|
| x² | `x**2` |
| e^(3x) | `exp(3*x)` |
| ln(x) | `log(x)` |
| √x | `sqrt(x)` |
| π | `pi` |

Implicit multiplication works: `3x` → `3*x`, caret works: `x^2` → `x**2`.

## How the adaptive algorithm works

Progress is stored in `~/.math_tutor/progress.db` (SQLite).

Each topic has an **ELO rating** (starts at 450):
- Correct answer → ELO increases (more if you beat the expected score)
- Wrong answer → ELO decreases
- Higher ELO → harder problems selected

**Topic unlock chain** (prerequisites must reach ELO ≥ 620):
```
Real Analysis ──┐
                ├──▶ Multivariable Calculus ──▶ Vector Calculus ──┐
Linear Algebra ─┘                                                  │
                                                                   ▼
Real Analysis ──┐                                         Partial Diff. Eqs.
                ├──▶ ODEs ──▶ Fourier Analysis ──────────────────▶ (also needs Fourier)
Linear Algebra ─┘

Multivariable Calculus ──▶ Complex Analysis
```

## Topic coverage

| Topic | Subtopics |
|---|---|
| Real Analysis | Limits, derivatives, series convergence, continuity, MVT |
| Linear Algebra | Determinants, eigenvalues, linear systems, rank, null space |
| Multivariable Calculus | Partial derivatives, gradient, optimization, double integrals |
| Vector Calculus | Divergence, curl, line integrals, Green's/Stokes'/Divergence theorems |
| ODEs | Separable, linear, 2nd-order, undetermined coefficients, systems, Laplace |
| Fourier Analysis | Series, coefficients, convergence, transforms, Parseval |
| PDEs | Classification, heat/wave/Laplace equations, characteristics, Green's functions |
| Complex Analysis | C-R equations, Cauchy's theorem, residues, conformal maps |
