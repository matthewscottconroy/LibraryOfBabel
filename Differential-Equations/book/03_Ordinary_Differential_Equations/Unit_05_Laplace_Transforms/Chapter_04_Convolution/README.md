# Chapter 4: Convolution

Convolution is the operation that corresponds, in the time domain, to multiplication of transforms in the $s$-domain. It expresses the response of a linear system to an arbitrary input in terms of the system's impulse response, providing the time-domain counterpart of the transfer function concept.

## The Convolution Integral

The **convolution** of two functions $f$ and $g$ (for $t \geq 0$) is

$$(f * g)(t) = \int_0^t f(t-\tau)g(\tau)\,d\tau.$$

The **convolution theorem** states: $\mathcal{L}\{f * g\} = F(s)G(s)$, so $\mathcal{L}^{-1}\{F(s)G(s)\} = (f*g)(t)$.

This theorem is immensely practical: when a Laplace transform $Y(s)$ factors as $H(s)G(s)$, the solution $y(t)$ is the convolution of $h(t) = \mathcal{L}^{-1}\{H\}$ and $g(t) = \mathcal{L}^{-1}\{G\}$.

## Applications

The chapter develops two main applications. First, the convolution theorem gives the general solution of $y'' + py' + qy = g(t)$ via the formula $y(t) = y_h(t) + \int_0^t h(t-\tau)g(\tau)\,d\tau$, where $h(t) = \mathcal{L}^{-1}\{1/(s^2+ps+q)\}$ is the impulse response. This is the time-domain expression of the Green's function solution.

Second, convolution provides the tool for solving Volterra integral equations of the convolution type: $y(t) + \int_0^t k(t-\tau)y(\tau)\,d\tau = f(t)$ transforms to $(1 + K(s))Y(s) = F(s)$, giving $Y(s) = F(s)/(1+K(s))$ and $y = \mathcal{L}^{-1}\{F/(1+K)\}$.

## Algebraic Properties

Convolution is commutative ($f*g = g*f$), associative ($f*(g*h) = (f*g)*h$), and distributes over addition. With these properties and $f * \delta = f$, the set of suitable functions forms a commutative algebra under convolution. The Laplace transform is an algebra isomorphism from this convolution algebra to the algebra of functions under pointwise multiplication.
