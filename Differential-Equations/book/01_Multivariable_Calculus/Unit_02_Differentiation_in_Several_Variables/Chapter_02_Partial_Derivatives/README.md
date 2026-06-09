# Chapter 02 Partial Derivatives

Given a function $f(x, y)$ of two variables, the most immediate attempt to differentiate it is to hold one variable fixed and differentiate with respect to the other. The result is the partial derivative: $\partial f/\partial x$ treats $y$ as a constant and differentiates with respect to $x$; $\partial f/\partial y$ treats $x$ as constant and differentiates with respect to $y$. These are not two separate functions but two aspects of the derivative of $f$, each capturing its rate of change in one coordinate direction.

## What This Chapter Covers

**Section 1 (Definition and Computation)** gives the limit definition of the partial derivative: $\frac{\partial f}{\partial x}(a,b) = \lim_{h\to 0}\frac{f(a+h,b)-f(a,b)}{h}$, and shows that computation reduces to ordinary single-variable differentiation (hold the other variables fixed and differentiate as usual). The section also introduces various notations ($f_x$, $f_{x_i}$, $D_i f$, $\partial f/\partial x_i$) and treats functions of $n$ variables.

**Section 2 (Higher-Order Partial Derivatives)** differentiates the partial derivatives again to obtain second-order and higher partial derivatives. A function of two variables has four second-order partial derivatives: $f_{xx}$, $f_{xy}$, $f_{yx}$, and $f_{yy}$. The mixed partials $f_{xy}$ and $f_{yx}$ involve differentiating first with respect to one variable, then the other, in opposite orders.

**Section 3 (Clairaut's Symmetry Theorem)** establishes that, under mild conditions, the order of differentiation does not matter: $f_{xy} = f_{yx}$ whenever both mixed partials are continuous. This is a nontrivial result — without continuity, mixed partials can differ — and it has far-reaching consequences for simplifying computations and for the theory of differential equations.

## How the Sections Build on Each Other

Section 1 provides the basic objects (partial derivatives) that Sections 2 and 3 analyze. The higher-order derivatives of Section 2 are computed by repeated application of the definition in Section 1. Clairaut's theorem in Section 3 is a theorem about the higher-order derivatives of Section 2, and its proof uses the single-variable mean value theorem applied twice.

## How This Chapter Fits into the Unit

Partial derivatives are the most elementary differentiation operation in several variables, and they feed into every subsequent chapter. The Jacobian matrix (Chapter 3) has partial derivatives as entries. The gradient (Chapter 4) is the vector of first-order partial derivatives. The Hessian matrix (Chapter 5) has second-order partial derivatives as entries, and Clairaut's theorem ensures the Hessian is symmetric. The critical point conditions of Chapter 6 set the gradient (all first partials) to zero, and the second derivative test uses the Hessian (second partials). Every partial differential equation is, by definition, an equation relating partial derivatives of various orders.

A central warning: the existence of partial derivatives does not imply differentiability (continuity, even). This surprise — thoroughly explained in Chapter 3 — should motivate students to understand the distinction between partial derivatives and the total derivative. A function can have well-defined partial derivatives at a point while failing to be continuous there, let alone differentiable. The chapters of this unit form a logical progression from the coarse tool (partial derivatives) to the correct notion (total derivative / differentiability).
