# Logic in Hardware Design

## Boolean Algebra is Hardware

The most direct connection between logic and physical reality: **digital hardware is Boolean algebra made physical**. Claude Shannon's 1937 master's thesis showed that Boolean algebra perfectly describes the behavior of electrical circuits with two states (on/off). Every AND gate, OR gate, NOT gate is a direct hardware implementation of a logical connective.

**Logic gate** → **Boolean operation** → **Logical connective**:
- AND gate: $A \cdot B$ → conjunction $A \wedge B$
- OR gate: $A + B$ → disjunction $A \vee B$
- NOT gate: $\bar{A}$ → negation $\neg A$
- NAND gate: $\overline{A \cdot B}$ → Sheffer stroke (functionally complete alone)
- XOR gate: $A \oplus B$ → exclusive or

**Boolean algebra** describes the algebra of logic circuits. Every digital circuit computes a Boolean function, and Boolean algebra provides the mathematics for simplifying, optimizing, and reasoning about these functions.

## Hardware Description Languages

Modern hardware is designed in **Hardware Description Languages (HDL)** like VHDL and Verilog, which describe circuits at the Register Transfer Level (RTL):

```verilog
// A simple 4-bit adder in Verilog
module adder4(
  input  [3:0] a, b,
  input        cin,
  output [3:0] sum,
  output       cout
);
  assign {cout, sum} = a + b + cin;
endmodule
```

This is *synthesized* by tools that convert the HDL to actual gates, satisfying timing, power, and area constraints.

## Formal Verification of Hardware

**Equivalence checking**: After optimization or synthesis, verify the optimized netlist is equivalent to the original specification. Uses SAT/BDD-based methods.

**Property checking (model checking)**: Verify temporal logic properties like:
- "After a reset, the counter is 0" — $AG(\text{reset} \to AX(\text{count} = 0))$
- "The pipeline never reads from an uninitialized register" — $AG\, \neg\text{uninit\_read}$

**Theorem proving**: Prove deep properties using Isabelle, HOL4, or Coq. Used for arithmetic circuits (proving multiplication is correct), cryptographic hardware, and protocol implementations.

## Boolean Satisfiability in Hardware

Modern hardware design relies heavily on **SAT solvers**:
- **Equivalence checking**: Is circuit $C_1$ equivalent to $C_2$? Encode as "is there an input where they differ?" — a SAT query.
- **Timing analysis**: Can all paths be satisfied simultaneously? SAT/SMT.
- **Clock domain crossing**: Verify no metastability across clock domains.
- **Automatic Test Pattern Generation (ATPG)**: Find tests that detect hardware faults — a SAT problem (find an input distinguishing the faulty and correct circuits).

Modern **electronic design automation (EDA)** tools use SAT solvers with millions of variables routinely — a direct industrial application of computational logic.

## Propositional Resolution in Gate-Level Verification

```python
# Verifying a simple combinational circuit using Z3
from z3 import *

# Declare gate inputs and outputs as Booleans
a, b, cin = Bools('a b cin')

# Half adder: sum = a XOR b, carry = a AND b
sum_ab = Xor(a, b)
carry_ab = And(a, b)

# Full adder: use two half adders
sum_full = Xor(sum_ab, cin)
carry_mid = And(sum_ab, cin)
cout = Or(carry_ab, carry_mid)

# Verify: the full adder output matches expected arithmetic
# sum = (a + b + cin) mod 2, cout = (a + b + cin) >= 2
s = Solver()

# Express the specification mathematically (using arithmetic)
a_int = If(a, BitVecVal(1, 2), BitVecVal(0, 2))
b_int = If(b, BitVecVal(1, 2), BitVecVal(0, 2))
cin_int = If(cin, BitVecVal(1, 2), BitVecVal(0, 2))
total = a_int + b_int + cin_int

expected_sum = (total & BitVecVal(1, 2)) != BitVecVal(0, 2)
expected_cout = (total & BitVecVal(2, 2)) != BitVecVal(0, 2)

# Check that our circuit matches the specification
s.add(Or(
    sum_full != expected_sum,
    cout != expected_cout
))
result = s.check()
print("Circuit verified correct:", result == unsat)  # True
```

## Exercises
See [problems/ch13_applications/](../../../problems/ch13_applications/)
