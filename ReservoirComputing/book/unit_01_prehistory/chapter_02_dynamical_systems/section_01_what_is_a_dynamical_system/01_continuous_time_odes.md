# Section 1.1: Continuous-Time Dynamical Systems

## Definition

A **continuous-time dynamical system** on a state space $X \subseteq \mathbb{R}^n$ is a differential equation of the form

$$\dot{\mathbf{x}} = f(\mathbf{x}), \qquad \mathbf{x}(0) = \mathbf{x}_0 \tag{1.1}$$

where $f: X \to \mathbb{R}^n$ is a smooth (or at least Lipschitz continuous) vector field. The state $\mathbf{x}(t) \in X$ describes the system at time $t$. The function $f$ is the *law of motion* — it assigns to each point in state space a velocity vector telling you how fast and in what direction the state is moving.

This equation is called *autonomous* because $f$ does not depend explicitly on $t$. The vector field $f$ is fixed — it does not care what time it is, only where the system currently is. (Non-autonomous systems of the form $\dot{\mathbf{x}} = f(\mathbf{x}, t)$ appear in Section 7 when we study driven reservoirs.)

The solution $\mathbf{x}(t)$ for $t \geq 0$ is called a **trajectory** or **orbit** of the system. The collection of all trajectories, drawn as curves in $X$, is the **phase portrait**. The phase portrait is a geometric picture of the entire behavior of the system, independent of any particular initial condition.

---

## Existence and Uniqueness: The Picard-Lindelöf Theorem

Before working with a differential equation, you should know whether it has solutions and whether those solutions are unique. Fortunately, for most physically reasonable systems, both answers are yes, and the reason is a classical theorem.

**Theorem (Picard-Lindelöf).** *Let $f: U \to \mathbb{R}^n$ be Lipschitz continuous on an open set $U \subseteq \mathbb{R}^n$: that is, there exists a constant $L > 0$ such that*

$$\|f(\mathbf{x}) - f(\mathbf{y})\| \leq L \|\mathbf{x} - \mathbf{y}\| \quad \text{for all } \mathbf{x}, \mathbf{y} \in U$$

*Then for any initial condition $\mathbf{x}_0 \in U$, there exists a unique solution $\mathbf{x}(t)$ of $\dot{\mathbf{x}} = f(\mathbf{x})$ defined on some interval $(-\varepsilon, \varepsilon)$ around $t = 0$.*

We do not prove this theorem here (the proof constructs the solution as the fixed point of a contraction map on a space of continuous functions — a beautiful argument that repays study [Teschl2012]), but let's understand what it says geometrically.

The Lipschitz condition says that $f$ cannot change too fast: if you move a small distance in $\mathbf{x}$, the velocity vector $f(\mathbf{x})$ changes by at most a proportional small amount. This prevents two things from going wrong. First, it prevents the velocity from blowing up so fast that the solution escapes to infinity in finite time (which can happen with equations like $\dot{x} = x^2$, which has the solution $x(t) = 1/(1-t)$ — finite-time blowup at $t=1$). Second, it prevents two trajectories from crossing. In a Lipschitz vector field, if two trajectories meet at any point, they must be the same trajectory. The phase portrait is foliated by non-intersecting curves.

This non-crossing property is geometrically powerful. It means that once you sketch the phase portrait of a 2D system, you know that the dynamics must follow those curves without jumping or crossing. Trajectories in 2D can therefore do only a limited number of things: flow toward a fixed point, flow away from one, approach a limit cycle, or escape to infinity. This is the content of the Poincaré-Bendixson theorem, which we encounter in Section 3.

---

## Example 1: The Damped Pendulum

The angular position $\theta$ of a pendulum of length $\ell$ and mass $m$, subject to gravitational restoring force and linear damping, satisfies:

$$m\ell^2 \ddot{\theta} = -mg\ell \sin\theta - b\dot{\theta}$$

Dividing by $m\ell^2$:

$$\ddot{\theta} = -\frac{g}{\ell} \sin\theta - \frac{b}{m\ell^2} \dot{\theta}$$

Let us define $\omega = \dot{\theta}$ (the angular velocity) and set $\alpha = g/\ell$, $\gamma = b/(m\ell^2)$ for brevity. Then we have a *system* of two first-order ODEs:

$$\dot{\theta} = \omega \tag{1.2a}$$
$$\dot{\omega} = -\alpha \sin\theta - \gamma \omega \tag{1.2b}$$

The state is $\mathbf{x} = (\theta, \omega) \in \mathbb{R} \times \mathbb{R}$, and the vector field is $f(\theta, \omega) = (\omega, -\alpha\sin\theta - \gamma\omega)$.

Any $n$-th order ODE can be converted to a first-order system in exactly this way: introduce new variables for each derivative up to order $n-1$. This is why we focus on first-order systems — they are completely general.

The phase portrait of the damped pendulum has a rich structure. When $\gamma > 0$, trajectories spiral inward toward fixed points at $(\theta^*, \omega^*) = (2\pi k, 0)$ for integer $k$ — these correspond to the pendulum hanging straight down. There are also unstable fixed points at $(\theta^*, \omega^*) = ((2k+1)\pi, 0)$, corresponding to the pendulum balanced upright. Trajectories starting with large enough initial angular velocity $\omega$ wrap around the cylinder $S^1 \times \mathbb{R}$ (since $\theta$ is an angle, physically identified modulo $2\pi$) before settling into a downward equilibrium.

This is already a rich picture from a simple 2D system with a clear physical interpretation.

---

## Example 2: The RC Circuit

An RC circuit — a resistor $R$ and capacitor $C$ in series with a voltage source $V_{in}$ — obeys Kirchhoff's voltage law:

$$V_{in}(t) = R \cdot I(t) + V_C(t)$$

Since $I = C \dot{V}_C$, we get

$$\dot{V}_C = \frac{1}{RC} \left( V_{in}(t) - V_C \right) \tag{1.3}$$

In the autonomous case ($V_{in} = V_0$ constant), this is a 1D linear ODE with solution

$$V_C(t) = V_0 + (V_C(0) - V_0) e^{-t/RC}$$

The voltage on the capacitor decays exponentially toward $V_0$, with time constant $\tau = RC$. The single fixed point is $V_C^* = V_0$.

The RC circuit is trivially simple — just a single state variable with a linear equation — but it illustrates something important: the time constant $\tau$ controls how fast the system forgets its initial condition. A circuit with large $\tau$ remembers its initial state for a long time; one with small $\tau$ forgets quickly. This is an embryonic form of the *memory* that reservoirs exploit. In a network of RC circuits — or neurons, which behave similarly — different time constants allow different layers of temporal integration, and the ensemble of states becomes a record of the input's history.

---

## Example 3: The Lotka-Volterra System

Perhaps the most famous model in mathematical biology, the Lotka-Volterra predator-prey equations describe the interaction between a prey population $x$ (say, rabbits) and a predator population $y$ (say, foxes) [Volterra1926]:

$$\dot{x} = \alpha x - \beta x y \tag{1.4a}$$
$$\dot{y} = \delta x y - \gamma y \tag{1.4b}$$

Here $\alpha$ is the rabbit birth rate, $\beta$ is the predation rate, $\delta$ is the efficiency with which eaten rabbits become new foxes, and $\gamma$ is the fox death rate. All parameters are positive.

This system has two fixed points: the trivial equilibrium $(0, 0)$ (extinction of both species) and the coexistence equilibrium

$$\mathbf{x}^* = \left(\frac{\gamma}{\delta},\ \frac{\alpha}{\beta}\right)$$

The phase portrait around $\mathbf{x}^*$ consists of closed orbits — periodic oscillations in both populations. Rabbits and foxes cycle out of phase: the rabbit population peaks first, which sustains a growing fox population, which then eats down the rabbits, causing the fox population to fall, which allows rabbits to recover. The system is a conservative oscillator: it has a conserved quantity (the Volterra invariant $H = \delta x - \gamma \ln x + \beta y - \alpha \ln y$), and trajectories live on level sets of $H$.

The Lotka-Volterra system is a touchstone because it demonstrates that *qualitative, robust oscillatory behavior* can emerge from simple nonlinear rules. No one "built in" the oscillation; it arises from the geometry of the vector field.

---

## The Phase Portrait as the Right Object

A plot of a single trajectory $\mathbf{x}(t)$ against $t$ tells you what the system does for one initial condition. The phase portrait tells you what the system *can* do, for all initial conditions simultaneously. It is the geometry of the dynamics.

For a 2D system $(\dot{x}, \dot{y}) = (f_1(x,y), f_2(x,y))$, the phase portrait is drawn by:

1. Sketching the **nullclines**: the curves where $f_1 = 0$ (so $\dot{x} = 0$) and $f_2 = 0$ (so $\dot{y} = 0$). At their intersections lie the fixed points.
2. Drawing the **direction field**: at a grid of points $(x, y)$, drawing short arrows in the direction of $f(x, y)$.
3. Sketching representative **trajectories** that follow the arrows.

The phase portrait partitions the state space into regions of qualitatively similar behavior. Between these regions are **separatrices** — special trajectories that divide basins of attraction. For the pendulum, for instance, the separatrices are the trajectories that asymptotically approach the upright equilibrium: trajectories starting just inside the separatrix spiral down to the resting equilibrium; trajectories starting outside wrap all the way around.

For reservoir computing, the phase portrait of the reservoir in the absence of driving is a starting point for understanding what the reservoir does when driven. A reservoir with a phase portrait full of complex transient dynamics — many directions of approach to attractors, a rich geometry of separatrices — has more potential to encode complex input histories. A reservoir whose phase portrait collapses quickly to a single fixed point will forget its initial state, and hence its input history, too quickly.

---

## The Flow Map

Given the solution $\mathbf{x}(t)$ of equation (1.1) for initial condition $\mathbf{x}_0$, we can define the **flow map**:

$$\Phi^t: X \to X, \qquad \Phi^t(\mathbf{x}_0) = \mathbf{x}(t)$$

This is the map that takes an initial condition to the state $t$ time units later. The flow map satisfies:

- $\Phi^0 = \text{id}$ (identity): at time zero, every point maps to itself.
- $\Phi^{t+s} = \Phi^t \circ \Phi^s$ (group property): flowing for time $s$ then for time $t$ is the same as flowing for time $t+s$.
- $t \mapsto \Phi^t$ is smooth in $t$ when $f$ is smooth.

The group property is the mathematical expression of the fact that the system has no memory of how it got to its current state: only the current state matters for future evolution. This is the Markov property, and it is built into the very definition of a dynamical system.

The flow map will appear again in Section 2 when we analyze stability: stability of a fixed point $\mathbf{x}^*$ is a statement about the behavior of $D\Phi^t$ near $\mathbf{x}^*$ as $t \to \infty$.

---

## Summary

A continuous-time dynamical system is a vector field on a state space. The Picard-Lindelöf theorem guarantees that trajectories exist and are unique for smooth enough vector fields. Phase portraits reveal the qualitative geometry of all trajectories simultaneously. Three examples — the damped pendulum, the RC circuit, and the Lotka-Volterra system — illustrate how different physical mechanisms give rise to different phase portrait geometries: spiraling to fixed points, exponential decay, and persistent oscillation.

In the next section, we turn to discrete-time systems, where $t$ takes integer values and the map $x_{t+1} = f(x_t)$ replaces the differential equation.
