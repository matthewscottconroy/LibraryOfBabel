# Section 1.2: Sequences, Signals, and the Need for State

---

## Section Introduction

The previous section established what feedforward networks cannot do. This section examines what temporal processing actually requires — and why **state** is the fundamental answer.

A state is a finite-dimensional summary of the past that is sufficient (in some approximate sense) to predict the future. The idea appears independently across many fields: in control theory (the state vector), in probability (the sufficient statistic), in physics (the phase space point), in cognitive science (the mental representation). The commonality is always the same: rather than explicitly storing the entire past, we maintain a compressed representation that captures what is relevant.

This section develops the precise mathematical language for sequences and signals, introduces the concept of state formally, and motivates the Volterra series as the mathematician's canonical approach to systems with memory — before explaining why, in practice, the Volterra series breaks down and a different approach is needed.
