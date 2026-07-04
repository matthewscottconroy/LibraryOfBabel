# 20.3.2 The One-Way Quantum Computer

## Computation by Measurement Alone

Raussendorf and Briegel (2001) proposed a model of computation with a startling economy of ingredients: prepare a cluster state once, then compute by measuring its qubits one at a time, each in a single-qubit basis chosen by the programmer. There are no unitary gates during the computation and no way to run it backwards — hence *one-way* quantum computer. The cluster is consumed as it is measured; the answer is read from the last unmeasured qubits. Everything hard has been front-loaded into making the cluster; the "program" is simply the list of measurement bases.

The measurements that do computational work lie in the equatorial ($X$–$Y$) plane of the Bloch sphere. Measuring qubit $j$ in the basis

$$|{\pm}\theta\rangle_j = \frac{1}{\sqrt{2}}\left(|0\rangle \pm e^{i\theta}|1\rangle\right)$$

is equivalent to measuring the observable $M_j(\theta) = \cos\theta\, X_j + \sin\theta\, Y_j$. The angle $\theta$ is the knob: $\theta = 0$ is an $X$ measurement, $\theta = \pi/2$ is a $Y$ measurement, and intermediate angles implement continuous rotations. Computational-basis ($Z$) measurements, by contrast, do no logic — they merely delete a qubit (Section 20.3.1), and are used to etch the connectivity of the desired circuit into a bulk cluster before the real computation begins.

## The Rotation-Plus-Teleportation Gadget

The atom of one-way computing is a two-qubit cluster: an input qubit carrying a state $|\psi\rangle = \alpha|0\rangle + \beta|1\rangle$, linked by a $CZ$ to a second qubit prepared in $|+\rangle$. Measure the input in the $|{\pm}\theta\rangle$ basis and the state is *teleported onto the second qubit with a gate applied en route*. The one-way computer is a lattice of these gadgets, chained so the output of each is the input of the next.

Universality follows from stringing gadgets together. A horizontal chain of equatorial measurements realizes an arbitrary single-qubit rotation, because the product $\cdots H R_z(\theta_3)\, H R_z(\theta_2)\, H R_z(\theta_1)$ generates all of SU(2) (an Euler decomposition, with the Hadamards supplied for free by the teleportation step). Genuine two-qubit gates require the second spatial dimension: a vertical edge bridging two horizontal chains carries a $CZ$ between the logical qubits riding them. Hence the 2D square-lattice cluster is universal, while a 1D chain gives only single-qubit unitaries. Information flows across the lattice in the direction of measurement, each column of measurements advancing every logical qubit one step through its circuit.

## Byproducts and Feed-Forward

The catch is that each measurement has two outcomes, and the "wrong" one applies an extra Pauli. The output of a gadget is the desired gate **times a known Pauli byproduct** $X^{s}Z^{t}$, with $s, t$ read from the measurement records. Byproducts are harmless in isolation — they are tracked classically in software (the Pauli frame) and undone at the very end. But they are *not* harmless for the measurements still to come. Pushing a byproduct $X^s$ through the next rotation flips the sign of its angle, because $R_z(\theta)\,X = X\,R_z(-\theta)$. Therefore the basis angle actually dialed into qubit $j$ must be corrected in real time,

$$\theta_j \;\longrightarrow\; (-1)^{s_j}\,\theta_j + t_j\,\pi,$$

using outcomes $s_j, t_j$ accumulated from qubits measured earlier. This **feed-forward** — earlier outcomes steering later measurement settings — is the source of the model's only temporal ordering: equatorial ($X$–$Y$) measurements must respect the partial order of information flow, while Pauli measurements commute with the byproducts and may be performed in any order, even all at once. Feed-forward is exactly the nanosecond-scale conditional switching demonstrated for KLM (Section 20.2.3); the one-way computer simply makes it the whole control system.

## Reading a Circuit off the Lattice

A quantum circuit is compiled into a measurement pattern by laying its qubit worldlines along horizontal rows of the cluster and its two-qubit gates along the vertical bonds that bridge them. Each logical time-step becomes a column of measurements, and a depth-$D$, width-$w$ circuit maps to a cluster of roughly $w$ rows and $D$ columns, consumed column by column. Computational-basis ($Z$) measurements etch away the unused vertices to leave the desired connectivity; because they commute with the byproduct bookkeeping, they may be performed first and in any order, while only the equatorial measurements that carry rotations must wait on feed-forward. The upshot is a strict resource accounting: the number of physical cluster qubits scales with the circuit's *space-time volume*, $\sim wD$ — the currency that fusion-based architectures (Section 20.3.3) and the error-corrected machines of Section 20.5 must ultimately pay in photons.

## Worked Example: A Rotation Teleported Down One Bond

Take the elementary gadget: input $|\psi\rangle = \alpha|0\rangle + \beta|1\rangle$ on qubit 1, ancilla $|+\rangle$ on qubit 2, joined by $CZ_{12}$. The joint state is

$$CZ_{12}\,\big(|\psi\rangle_1 \otimes |+\rangle_2\big) = \tfrac{1}{\sqrt2}\big[\,|0\rangle_1(\alpha|0\rangle + \alpha|1\rangle)_2 + |1\rangle_1(\beta|0\rangle - \beta|1\rangle)_2\,\big].$$

Measure qubit 1 in the $|{\pm}\theta\rangle$ basis. For outcome $|{+}\theta\rangle$ (call it $s=0$), project with $\langle{+}\theta|_1 = \tfrac{1}{\sqrt2}(\langle 0| + e^{-i\theta}\langle 1|)$. The (unnormalized) state left on qubit 2 is

$$\tfrac{1}{2}\big[(\alpha + e^{-i\theta}\beta)\,|0\rangle + (\alpha - e^{-i\theta}\beta)\,|1\rangle\big] \;\propto\; H\,R_z(-\theta)\,|\psi\rangle,$$

using $R_z(-\theta)|\psi\rangle = \alpha|0\rangle + e^{-i\theta}\beta|1\rangle$ and $H(a|0\rangle+b|1\rangle) = \tfrac{1}{\sqrt2}[(a{+}b)|0\rangle + (a{-}b)|1\rangle]$. For outcome $|{-}\theta\rangle$ ($s=1$), the same computation gives $X\,H\,R_z(-\theta)\,|\psi\rangle$. Combining,

$$|\psi\rangle \;\longmapsto\; X^{s}\,H\,R_z(-\theta)\,|\psi\rangle \quad\text{on qubit 2}.$$

The input has been teleported one bond over, rotated by $R_z(-\theta)$ and Hadamard-transformed, up to the byproduct $X^{s}$. To realize a target $R_z(\phi)$ one sets $\theta = -\phi$; to cancel the accompanying $H$ one measures a second qubit at angle $0$. Chaining, say, four such gadgets with angles $(\theta_1,\theta_2,\theta_3,\theta_4)$ implements $H R_z(-\theta_4)\cdots H R_z(-\theta_1)$ on the state, an arbitrary single-qubit unitary — and the adaptive rule $\theta_j \to (-1)^{s_{j-1}}\theta_j$ keeps the byproducts from corrupting it. This single bond, iterated across a 2D lattice and cross-linked for $CZ$s, is a universal quantum computer whose only moving part is a photodetector.
