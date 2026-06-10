# Effective Computability

## What Does "Computable" Mean?

Before computers existed, mathematicians wondered: what problems can be solved by a purely mechanical, step-by-step process? The question was urgent because of Hilbert's *Entscheidungsproblem* (1928): "Is there a mechanical procedure that can determine whether any given mathematical statement is provable?"

In the 1930s, several independent answers emerged, all defining the same class of functions:
- **Turing machines** (Alan Turing, 1936)
- **Lambda calculus** (Alonzo Church, 1936)
- **General recursive functions** (Gödel and Kleene, 1936)
- **Post correspondence systems** (Emil Post, 1946)

The agreement of these wildly different models on the same class of functions is deeply significant: it suggests we have captured the correct notion of "mechanical computation."

## Primitive Recursive Functions

**Primitive recursive functions** are defined from:
1. **Base functions**: constant functions, the successor $S(n) = n+1$, projections $\pi_i^n(x_1, \ldots, x_n) = x_i$
2. **Composition**: $h(\vec{x}) = f(g_1(\vec{x}), \ldots, g_k(\vec{x}))$
3. **Primitive recursion**: $f(0, \vec{x}) = g(\vec{x})$ and $f(n+1, \vec{x}) = h(n, f(n, \vec{x}), \vec{x})$

This class includes factorial, addition, multiplication, exponentiation, GCD, primality testing, and essentially all "everyday" number-theoretic functions.

But it is **not all computable functions**: the Ackermann function (a classic fast-growing function) is computable but not primitive recursive.

## Total and Partial Computable Functions

**Total computable** (recursive) functions: defined for all inputs, terminate on all inputs.

**Partial computable** (partial recursive) functions: may be undefined (non-terminating) for some inputs. These are the functions Turing machines compute — they halt and produce output when defined, and run forever otherwise.

The **halting problem** (section 02) shows there are total functions that are *not* total computable — the function "does Turing machine $M$ halt on input $x$?" is total (either yes or no) but not computable.

## Exercises
See [problems/ch10_computability/](../../../problems/ch10_computability/)
