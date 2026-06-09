# Unit 1: Introduction and Classification

Before developing solution techniques for differential equations, it is essential to have a clear conceptual framework: precise definitions, a reliable classification scheme, and a foundational theorem that tells us when problems are well-posed. This unit builds that framework.

## What This Unit Covers

The unit consists of a single chapter addressing the fundamental concepts that underlie the entire subject. It begins with the definition of an ordinary differential equation and immediately engages the reader with concrete examples drawn from physics and biology. It then introduces the classification scheme based on order, degree, and linearity, which determines which solution methods apply to a given equation. The notion of a solution is made precise, and the important distinction between general, particular, and singular solutions is drawn carefully. The unit culminates with the existence and uniqueness theory for initial value problems, including a treatment of Picard's theorem and a discussion of how long a solution can be guaranteed to persist.

## Why Classification Matters

Differential equations do not form a single homogeneous class of mathematical objects. An equation like $y' = ky$ differs from $y'' + y = 0$ not merely in complexity but in structure, and the structure determines everything about how the equation behaves and how it can be solved. A first-order linear equation admits a universal solution formula; a first-order nonlinear equation may require a completely different approach or may resist closed-form solution entirely. Recognizing these structural features before attempting a solution is therefore not preliminary bookkeeping but genuine mathematical work.

The linearity distinction is especially significant. Linear equations obey a superposition principle that makes their solution sets vector spaces. This algebraic structure is the foundation for the entire theory of second-order linear equations and systems, for the Laplace transform method, for Sturm-Liouville theory, and ultimately for the spectral analysis of partial differential equations. Nonlinear equations lack this structure and must generally be treated by more specialized methods or by qualitative and numerical analysis.

## Existence and Uniqueness as a Foundation

A central achievement of this unit is the statement and discussion of Picard's existence and uniqueness theorem. This result guarantees that, under reasonable smoothness conditions on the right-hand side of $y' = f(x, y)$, there is exactly one solution passing through any prescribed initial point. The theorem is not merely a reassurance that problems have answers; it is a structural result whose proof introduces the method of successive approximations, a construction that is simultaneously an existence proof, a numerical method, and a precursor to fixed-point theorems in analysis.

Understanding what the theorem requires, and what happens when its hypotheses fail, is equally important. Examples where existence fails, where uniqueness fails, or where a solution exists only on a bounded interval prepare the student for the subtleties that arise in more advanced work.
