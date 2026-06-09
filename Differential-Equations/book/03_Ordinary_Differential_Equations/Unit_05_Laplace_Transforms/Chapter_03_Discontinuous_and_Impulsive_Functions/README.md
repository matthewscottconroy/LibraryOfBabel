# Chapter 3: Discontinuous and Impulsive Forcing Functions

One of the greatest advantages of the Laplace transform over classical methods is its ability to handle forcing functions that are discontinuous, impulsive, or piecewise-defined. The Heaviside step function, the Dirac delta distribution, and periodic functions all have clean Laplace transforms, and the algebraic machinery of the transform handles them as naturally as it handles smooth functions.

## Why This Matters

Physical systems are often driven by forces that switch on or off at specific times, deliver sudden impulses (like a hammer blow or an electrical spike), or repeat periodically. Modeling these with elementary functions requires piecewise definitions that are awkward to differentiate conventionally. The Laplace transform converts these piecewise definitions into algebraic expressions via the Heaviside function and the second shifting theorem.

## Chapter Contents

The first section introduces the **Heaviside unit step function** $u(t-a)$ and computes its transform. The second develops the **second shifting theorem** (time shifting): $\mathcal{L}\{u(t-a)f(t-a)\} = e^{-as}F(s)$. This allows piecewise-defined forcing functions to be transformed cleanly.

The third section treats the **Dirac delta** $\delta(t-a)$ as the idealization of an impulsive force. The transform $\mathcal{L}\{\delta(t-a)\} = e^{-as}$ (for $a \geq 0$) is simple, and the delta function simplifies the analysis of impulsively forced systems dramatically.

The fourth section handles **periodic forcing functions** via the formula $\mathcal{L}\{f_T\} = \frac{1}{1-e^{-Ts}}\int_0^T e^{-st}f(t)\,dt$, where $f_T$ is periodic with period $T$.

## The Unifying Theme

All these special functions are handled by the same algebraic framework once their transforms are known. The Laplace method makes no distinction between smooth and discontinuous forcing; the transform converts everything into rational (or nearly rational) functions of $s$, and the same partial-fraction inversion technique applies.
