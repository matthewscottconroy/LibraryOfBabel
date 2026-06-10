# Tarski's World: Introduction

## Overview
Tarski's World is a software tool (part of the *Language, Proof and Logic* package) that
makes FOL semantics visual and interactive. Blocks of various shapes and sizes sit on a
grid; you write first-order sentences and immediately see whether they are true or false
in the world. This section explains how to use it and what it teaches.

## Learning Objectives
- Set up and navigate Tarski's World (or Carnap as an alternative)
- Read a first-order sentence and evaluate it in a given world
- Build worlds that satisfy or falsify given sentences

## Getting the Software
**Option 1 — Language Proof and Logic (LPL)**:
The textbook by Barwise, Etchemendy, et al. comes with LPL software (Windows/Mac).
Available from CSLI Publications.

**Option 2 — Carnap (free, web-based)**:
carnap.io offers FOL exercises directly in the browser, including blocks-world problems.
No installation required. Open source.

**Option 3 — DIY**:
The Python file `textbook/ch03_first_order_logic/05_models_and_interpretations/03_models_in_python.py`
implements a simple blocks-world model checker.

## The Interface
- **World pane**: shows a grid with colored blocks (cubes, tetrahedra, dodecahedra)
- **Sentence bar**: enter a first-order sentence
- **Evaluate**: checks truth/falsity in the current world
- **Game mode**: sentence verification by logical game (Ehrenfeucht-Fraïssé)

## Key Pedagogical Points
1. Truth is always *relative to a world* (interpretation)
2. The same sentence can be true in one world and false in another
3. Building a counterexample world refutes a proposed logical law

## Exercises
See `problems/ch03_predicate_logic/02_tarskis_world_problems.md`
