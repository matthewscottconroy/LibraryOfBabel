# Chapter 1: Definition and Basic Properties of the Laplace Transform

This chapter establishes the Laplace transform rigorously: its definition as an improper integral, the conditions under which it converges, the linearity that makes it an algebraic tool, and the transforms of the elementary functions that form the basic table.

## The Definition

For $f: [0, \infty) \to \mathbb{R}$, the **Laplace transform** is

$$F(s) = \mathcal{L}\{f(t)\}(s) = \int_0^\infty e^{-st}f(t)\,dt,$$

defined for all $s$ for which the improper integral converges. The function $F(s)$ is the transform, sometimes called the image, of $f(t)$.

## Existence Conditions

The integral converges when $f$ does not grow too rapidly. A function $f$ is of **exponential order $c$** if there exist $M, T > 0$ such that $|f(t)| \leq Me^{ct}$ for $t > T$. If $f$ is piecewise continuous on $[0, \infty)$ and of exponential order $c$, then $\mathcal{L}\{f\}$ exists for all $\text{Re}(s) > c$.

## Linearity

$\mathcal{L}\{\alpha f + \beta g\} = \alpha\mathcal{L}\{f\} + \beta\mathcal{L}\{g\}$. This is the fundamental algebraic property: the Laplace transform is a linear operator from functions to functions.

## Basic Table

The chapter derives the transforms of elementary functions: $\mathcal{L}\{1\} = 1/s$, $\mathcal{L}\{e^{at}\} = 1/(s-a)$, $\mathcal{L}\{t^n\} = n!/s^{n+1}$, $\mathcal{L}\{\sin bt\} = b/(s^2+b^2)$, $\mathcal{L}\{\cos bt\} = s/(s^2+b^2)$, and combinations thereof. These entries, combined with the operational properties of Chapter 2, allow a wide range of functions to be transformed without evaluating the integral directly.

## Key Theorem

The Laplace transform of a function in the exponential order class is analytic (complex differentiable) for $\text{Re}(s) > c$. Moreover, the transform is unique: if $\mathcal{L}\{f\} = \mathcal{L}\{g\}$ and both are continuous, then $f = g$. This injectivity is the basis for the inverse transform.
