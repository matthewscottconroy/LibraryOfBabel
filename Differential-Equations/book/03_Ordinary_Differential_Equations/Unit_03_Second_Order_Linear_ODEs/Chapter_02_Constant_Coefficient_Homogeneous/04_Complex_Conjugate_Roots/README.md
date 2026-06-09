# Complex Conjugate Roots

When the discriminant $\Delta = b^2 - 4ac < 0$, the characteristic equation has two complex conjugate roots $r = \alpha \pm \beta i$ where $\alpha = -b/(2a)$ and $\beta = \sqrt{4ac - b^2}/(2a) > 0$. The corresponding complex exponential solutions are related by Euler's formula to real oscillatory functions.

## From Complex to Real Solutions

The complex solutions $e^{(\alpha + \beta i)x}$ and $e^{(\alpha - \beta i)x}$ are valid solutions, but for equations with real coefficients one generally seeks real-valued solutions. By Euler's formula $e^{i\theta} = \cos\theta + i\sin\theta$:

$$e^{(\alpha + \beta i)x} = e^{\alpha x}(\cos\beta x + i\sin\beta x).$$

The real and imaginary parts of a complex solution to a real ODE are themselves real solutions. Therefore:

$$y_1 = e^{\alpha x}\cos(\beta x), \qquad y_2 = e^{\alpha x}\sin(\beta x)$$

are both real solutions. Their Wronskian:

$$W = e^{\alpha x}\cos(\beta x)\cdot(\alpha e^{\alpha x}\sin(\beta x) + \beta e^{\alpha x}\cos(\beta x)) - (-\beta e^{\alpha x}\sin(\beta x) + \alpha e^{\alpha x}\cos(\beta x))\cdot e^{\alpha x}\sin(\beta x) = \beta e^{2\alpha x} \neq 0.$$

They are linearly independent (since $\beta > 0$), forming a fundamental set. The general solution is

$$y = e^{\alpha x}(c_1\cos\beta x + c_2\sin\beta x).$$

## Amplitude-Phase Form

The general solution can be rewritten in amplitude-phase form. Note that $c_1\cos\beta x + c_2\sin\beta x = A\cos(\beta x - \phi)$ where $A = \sqrt{c_1^2 + c_2^2}$ and $\tan\phi = c_2/c_1$. Therefore

$$y = Ae^{\alpha x}\cos(\beta x - \phi).$$

This form explicitly shows the **envelope** $\pm Ae^{\alpha x}$ and the **oscillation** $\cos(\beta x - \phi)$:
- If $\alpha < 0$: the envelope decays to zero; solutions are **underdamped oscillations** that approach zero while oscillating with angular frequency $\beta$.
- If $\alpha = 0$: the envelope is constant; solutions are **undamped oscillations** $A\cos(\beta x - \phi)$ with fixed amplitude.
- If $\alpha > 0$: the envelope grows; solutions exhibit growing oscillations.

## Worked Example

Solve $y'' - 4y' + 13y = 0$, $y(0) = 1$, $y'(0) = 2$.

Characteristic equation: $r^2 - 4r + 13 = 0$. Roots: $r = (4 \pm \sqrt{16 - 52})/2 = (4 \pm \sqrt{-36})/2 = 2 \pm 3i$.

So $\alpha = 2$, $\beta = 3$. General solution:

$$y = e^{2x}(c_1\cos 3x + c_2\sin 3x).$$

$y' = 2e^{2x}(c_1\cos 3x + c_2\sin 3x) + e^{2x}(-3c_1\sin 3x + 3c_2\cos 3x)$.

Imposing $y(0) = 1$: $c_1 = 1$. Imposing $y'(0) = 2$: $2c_1 + 3c_2 = 2$, so $c_2 = 0$. Solution:

$$y = e^{2x}\cos 3x.$$

In amplitude-phase form: $y = e^{2x}\cos(3x - 0)$. The amplitude $e^{2x}$ grows because $\alpha = 2 > 0$.

## Physical Significance: Underdamped Oscillator

For the underdamped spring-mass system $y'' + 2\delta y' + \omega_0^2 y = 0$ (with $0 < \delta < \omega_0$), the characteristic roots are $r = -\delta \pm \sqrt{\delta^2 - \omega_0^2} = -\delta \pm i\omega_d$, where $\omega_d = \sqrt{\omega_0^2 - \delta^2}$ is the **damped natural frequency**. The solution is

$$y = Ae^{-\delta t}\cos(\omega_d t - \phi).$$

The oscillations decay with time constant $1/\delta$; the frequency of oscillation is $\omega_d$, slightly lower than the natural frequency $\omega_0$ of the undamped system. As $\delta \to 0$, $\omega_d \to \omega_0$ and the amplitude stabilizes at $A$: undamped oscillation. As $\delta \to \omega_0^-$, $\omega_d \to 0$ and oscillations slow to extinction: approach to critical damping.

## Euler's Formula as the Bridge

The deep mathematical content of this section is Euler's formula $e^{i\theta} = \cos\theta + i\sin\theta$, which links exponential functions (algebraically natural solutions of linear ODEs) to trigonometric functions (physically meaningful oscillatory behavior). Complex exponentials provide the unified framework; real sine and cosine functions provide the physically interpretable form. This bridge between algebra (roots of polynomials), analysis (exponential functions), and geometry (trigonometric functions on the unit circle) is one of the central connections of undergraduate mathematics.
