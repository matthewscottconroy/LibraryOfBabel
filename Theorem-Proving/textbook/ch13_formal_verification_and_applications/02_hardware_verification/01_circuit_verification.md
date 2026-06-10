# Hardware Circuit Verification

## Why Hardware Verification Matters

Software can be patched. Hardware cannot. A bug in a processor or ASIC affects every unit shipped and can only be fixed by a hardware recall — enormously expensive (the Pentium FDIV recall cost $475M) or by abandoning faulty products.

Hardware verification uses formal methods to prove that a circuit implementation matches its specification *before* manufacturing.

## Levels of Abstraction

**Register Transfer Level (RTL)**: The standard level for hardware design. Describes circuits as networks of registers (state), combinational logic, and control. VHDL and Verilog are RTL languages.

**Gate level**: Networks of AND, OR, NOT, NAND, NOR gates. RTL compiles to gate level.

**Specification**: A mathematical or temporal logic description of intended behavior.

## Formal Verification Techniques

**Equivalence checking**: Prove that two circuit descriptions (e.g., before and after optimization) are functionally equivalent. Uses BDDs or SAT solvers. Industry standard for synthesis.

**Model checking**: Prove that a circuit satisfies temporal logic properties (safety, liveness). Efficient for control-dominated circuits; harder for large datapaths.

**Theorem proving**: Use Coq, Isabelle, or HOL to prove deep properties of circuit behavior. More expressive but requires expert knowledge.

## The Intel FPU Experience

After the Pentium FDIV bug (1994), Intel and AMD invested heavily in formal verification of arithmetic units:
- Every new FPU is now verified against IEEE 754 specification using formal methods
- Model checking tools verify cache coherence protocols
- Property checking verifies absence of specific error conditions

## SAT-Based Combinational Equivalence

For combinational circuits (no state, pure Boolean functions):

Given circuits $C_1$ and $C_2$, both computing Boolean functions $f_1, f_2 : \{0,1\}^n \to \{0,1\}$, equivalence checking asks: $f_1 = f_2$?

Encode as SAT: is there an input $x$ with $f_1(x) \neq f_2(x)$?

If SAT: found a counterexample (witness). If UNSAT: $f_1 = f_2$ — the circuits are equivalent.

Modern SAT solvers handle circuits with millions of gates efficiently via CDCL and circuit-specific preprocessing.

## Exercises
See [problems/ch13_applications/](../../../problems/ch13_applications/)
