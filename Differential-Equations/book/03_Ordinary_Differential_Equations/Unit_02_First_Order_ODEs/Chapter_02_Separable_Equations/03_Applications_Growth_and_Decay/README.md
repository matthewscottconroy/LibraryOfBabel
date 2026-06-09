# Applications: Growth and Decay

The simplest differential equation, $y' = ky$, encodes a principle that appears throughout the natural sciences: whenever the rate of change of a quantity is proportional to the quantity itself, exponential behavior results. The proportionality constant $k$ determines whether the quantity grows ($k > 0$) or decays ($k < 0$), and the magnitude of $k$ determines how fast.

## Exponential Growth

The equation $y' = ky$ with $k > 0$ and $y(0) = y_0 > 0$ has the solution $y(t) = y_0 e^{kt}$. The quantity $y$ grows exponentially, doubling every $T_{1/2} = (\ln 2)/k$ units of time. This **doubling time** is independent of the initial value $y_0$: whether the population starts at 100 or 1000, it takes the same time to double.

**Unrestricted population growth.** In the simplest model, a population $P(t)$ of bacteria grows proportionally to its size: $dP/dt = rP$, where $r > 0$ is the per capita growth rate (births minus deaths per individual per unit time). The solution $P(t) = P_0 e^{rt}$ predicts unbounded growth, which obviously cannot hold forever. The logistic correction (next section) addresses this. Nevertheless, exponential growth is an excellent approximation during the early phase when resources are abundant.

**Compound interest.** A bank account with balance $B(t)$ earning a continuously compounded interest rate $r$ satisfies $dB/dt = rB$, giving $B(t) = B_0 e^{rt}$. After time $T = (\ln 2)/r$, the balance has doubled. At an annual rate of $r = 0.07$, the doubling time is approximately 10 years (the "rule of 70").

## Exponential Decay

With $k < 0$, the solution $y(t) = y_0 e^{kt}$ decays toward zero. Writing $k = -\lambda$ with $\lambda > 0$, the equation is $y' = -\lambda y$ and the solution is $y(t) = y_0 e^{-\lambda t}$. The **half-life** is $t_{1/2} = (\ln 2)/\lambda$: the time for the quantity to decrease to half its current value, regardless of the current value.

**Radioactive decay.** A radioactive isotope decays because each nucleus has a fixed probability per unit time $\lambda$ of undergoing fission. If $N(t)$ is the number of nuclei, $dN/dt = -\lambda N$, so $N(t) = N_0 e^{-\lambda t}$. The half-life of carbon-14 is approximately 5730 years ($\lambda \approx 1.21 \times 10^{-4}$ yr$^{-1}$), making it useful for radiocarbon dating over a range of about 50,000 years.

**Carbon-14 dating example.** An archaeological sample contains $35\%$ of the carbon-14 found in a living organism of the same type. How old is the sample?

We need $t$ such that $e^{-\lambda t} = 0.35$, so $t = -\ln(0.35)/\lambda = \ln(1/0.35)/(1.21 \times 10^{-4})$. Computing: $\ln(1/0.35) = \ln(2.857) \approx 1.050$. Then $t \approx 1.050/(1.21 \times 10^{-4}) \approx 8678$ years.

## Newton's Law of Cooling

Newton's law of cooling states that the rate of heat loss from a body is proportional to the temperature difference between the body and its surrounding medium. If $T(t)$ is the temperature of the body and $T_{\text{env}}$ is the (constant) ambient temperature:

$$\frac{dT}{dt} = -k(T - T_{\text{env}}), \qquad k > 0.$$

This is separable (and linear). Let $u = T - T_{\text{env}}$; then $u' = -ku$, giving $u(t) = u_0 e^{-kt}$, so

$$T(t) = T_{\text{env}} + (T_0 - T_{\text{env}})e^{-kt}.$$

The body's temperature exponentially approaches the ambient temperature, with time constant $\tau = 1/k$.

**Example.** A cup of coffee at $95^\circ$C is placed in a room at $20^\circ$C. After 5 minutes it has cooled to $80^\circ$C. What will its temperature be after 15 minutes?

From $T(5) = 80$: $80 = 20 + (95 - 20)e^{-5k}$, so $60 = 75e^{-5k}$, giving $e^{-5k} = 4/5$, so $k = -\frac{1}{5}\ln(4/5) = \frac{1}{5}\ln(5/4)$.

Then $e^{-kt} = (4/5)^{t/5}$. At $t = 15$: $T(15) = 20 + 75(4/5)^3 = 20 + 75(64/125) = 20 + 38.4 = 58.4^\circ$C.

## Mixing Problems

A tank contains $V$ liters of salt solution. Pure water enters at rate $r$ liters/min, and well-mixed solution leaves at the same rate. The amount of salt $Q(t)$ (grams) satisfies

$$\frac{dQ}{dt} = -\frac{r}{V}Q,$$

since salt enters at rate 0 (pure water) and leaves at concentration $Q/V$ times flow rate $r$. The solution is $Q(t) = Q_0 e^{-rt/V}$, with time constant $\tau = V/r$. After time $V/r$, about $63\%$ of the initial salt has been flushed out.

A more general mixing problem has solution entering at rate $r_{\text{in}}$ with concentration $c_{\text{in}}$ and leaving at rate $r_{\text{out}} = r_{\text{in}} = r$:

$$\frac{dQ}{dt} = r\,c_{\text{in}} - \frac{r}{V}Q.$$

This is first-order linear (treated in Chapter 3), with steady-state salt content $Q^* = V\,c_{\text{in}}$.

## Continuously Compounded Problems and More General Growth Laws

All the examples above share the same mathematical structure: a linear autonomous ODE $y' = ay + b$ (possibly $b = 0$). The general solution is

$$y(t) = \frac{b}{-a} + \left(y_0 - \frac{b}{-a}\right)e^{at} = -\frac{b}{a} + \left(y_0 + \frac{b}{a}\right)e^{at}.$$

The equilibrium $y^* = -b/a$ attracts all solutions when $a < 0$ and repels them when $a > 0$. This structure underlies the mixing problem (where $a = -r/V < 0$ and $b = rc_{\text{in}}$), Newton's cooling (where $a = -k < 0$ and $b = kT_{\text{env}}$), and compound interest with a savings deposit (where $a = r > 0$ and $b$ is a deposit rate).
