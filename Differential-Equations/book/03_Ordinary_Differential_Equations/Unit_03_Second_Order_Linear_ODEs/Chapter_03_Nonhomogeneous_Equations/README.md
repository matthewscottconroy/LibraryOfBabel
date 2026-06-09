# Chapter 3: Nonhomogeneous Equations

When the second-order linear equation is nonhomogeneous, $ay'' + by' + cy = g(x)$ with $g(x) \not\equiv 0$, the solution must account for both the internal dynamics of the equation and the external forcing $g(x)$. The general solution has the structure $y = y_h + y_p$, where $y_h$ is the general homogeneous solution and $y_p$ is any one particular solution.

## Structure of the General Solution

The first section of this chapter establishes this decomposition theorem rigorously and discusses its implications. The homogeneous solution $y_h = c_1 y_1 + c_2 y_2$ represents the transient: the natural response of the system to initial conditions, which typically decays over time for stable systems. The particular solution $y_p$ represents the steady state: the long-term response driven by the forcing function $g$.

## Two Methods for Finding Particular Solutions

The second section develops the **method of undetermined coefficients**, which works when $g(x)$ is a combination of polynomials, exponentials, sines, and cosines. The form of $y_p$ is guessed based on the form of $g$, and the undetermined coefficients in the guess are found by substitution. The method is algebraically efficient when it applies.

The third section develops **variation of parameters**, a general method that works for any continuous $g(x)$. It extends the idea from Chapter 5 of Unit 2: the constants $c_1, c_2$ in the homogeneous solution are replaced by functions $v_1(x), v_2(x)$, and a system of equations is imposed to determine $v_1'$ and $v_2'$.

The fourth section discusses the superposition principle for forcing: if $g = g_1 + g_2 + \cdots$, one finds $y_{p,k}$ for each $g_k$ separately and adds them. This allows complex forcing to be decomposed into tractable components.

## The Physical Picture

For a damped oscillator driven by a periodic force, the homogeneous solution decays to zero while the particular solution approaches a steady periodic oscillation at the driving frequency. The interplay between these two components, and the dramatic behavior near resonance when the driving frequency matches the natural frequency, is the subject of Chapter 4.
