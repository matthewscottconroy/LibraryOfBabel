# 20.3.1 Cluster States

## A Qubit per Vertex, an Entangling Gate per Edge

A **graph state** is a recipe for entanglement written as a graph. Take a graph $G = (V, E)$, place one qubit on each vertex, initialize every qubit in $|+\rangle = (|0\rangle + |1\rangle)/\sqrt{2}$, and apply a controlled-$Z$ gate across every edge:

$$|G\rangle = \prod_{(u,v)\in E} CZ_{uv}\,|+\rangle^{\otimes |V|}.$$

The order of the $CZ$s does not matter — they all commute (each is diagonal in the computational basis). A **cluster state** is the special case where $G$ is a regular lattice; the 2D square-lattice cluster is the canonical universal resource (Briegel & Raussendorf, 2001). The two-qubit graph state (one edge) is $CZ|{+}{+}\rangle = (|0{+}\rangle + |1{-}\rangle)/\sqrt{2}$, which is local-unitary equivalent to a Bell state; the fully connected graph on $n$ vertices is local-unitary equivalent to an $n$-qubit GHZ state. Graph states thus interpolate smoothly between "Bell pairs everywhere" and "one big cat state," with the connectivity dialing the entanglement structure.

## The Stabilizer Description

Cluster states are stabilizer states, and the stabilizer formalism is far more economical than the $2^{|V|}$ amplitudes. For each vertex $v$, define the operator

$$K_v = X_v \prod_{w \in N(v)} Z_w,$$

where $N(v)$ is the set of neighbours of $v$. The graph state $|G\rangle$ is the unique simultaneous $+1$ eigenstate of all $|V|$ generators $\{K_v\}$: $K_v |G\rangle = +|G\rangle$ for every $v$. These $|V|$ commuting operators specify the state completely, replacing an exponential list of amplitudes with a linear list of Pauli strings. The stabilizer picture is what makes the theory tractable: measurement outcomes, byproduct operators, and error propagation are all bookkeeping in the Pauli group, and the Gottesman-Knill theorem guarantees the whole apparatus can be tracked on a classical computer (until, of course, the adaptive non-Pauli measurements of Section 20.3.2 supply the quantum power).

A single Pauli measurement transforms one graph into another by simple graph surgery (Hein et al., 2004). The rule we will lean on most: a **$Z$-measurement on vertex $v$ deletes $v$** from the graph, together with all its edges, leaving the graph state of $G - v$ (up to a $Z$ byproduct on the former neighbours if the outcome is $-1$). This is how measurement carves a computational "wire" out of a bulk cluster.

## Building Cluster States from Light

Photons cannot be held still while a deterministic $CZ$ acts across an edge — the $CZ$ itself is the 1/16-probability gate of Section 20.2.2. The practical route is to fabricate small graph states that *can* be made (Bell pairs from SPDC, or three-photon GHZ states) and **fuse** them into larger graphs. Browne and Rudolph (2005) introduced the two workhorse operations. A **type-I fusion** measures a photon from each of two states and, on success, welds the two graphs at that point. A **type-II fusion** is a partial Bell-state measurement on two photons (one from each graph): it consumes both, and on success it creates an edge joining their neighbourhoods. With bare linear optics a type-II fusion succeeds with probability $1/2$ — the Bell-measurement bottleneck of Section 20.2.3 in a new costume — and on failure it acts as a $Z$-measurement on both photons, deleting them cleanly. Nielsen (2004) first married the KLM gates to the cluster-state picture and showed the resource cost dropped by more than an order of magnitude versus direct circuit-model KLM; fusion pushed it further still. The lesson repeats: probabilistic operations are tolerable if failure is *heralded and benign* (here, a known vertex deletion) and if the growth process is redundant enough to route around losses.

## Worked Example: Stabilizers, and Measuring a Node Away

**Linear 4-chain.** Label the vertices $1{-}2{-}3{-}4$. Reading off $K_v = X_v \prod_{w\in N(v)} Z_w$:

$$K_1 = X_1 Z_2,\quad K_2 = Z_1 X_2 Z_3,\quad K_3 = Z_2 X_3 Z_4,\quad K_4 = Z_3 X_4.$$

These four commuting generators (check: $K_1$ and $K_2$ share qubits 1 and 2, on which they act as $X_1 Z_1$ vs. $Z_1 X_2$ — the overlap is a single qubit for $K_2$, so they commute) fix the state.

**Box (4-cycle).** Add the edge $4{-}1$ to close the ring. Now every vertex has two neighbours:

$$K_1 = X_1 Z_2 Z_4,\quad K_2 = Z_1 X_2 Z_3,\quad K_3 = Z_2 X_3 Z_4,\quad K_4 = Z_1 Z_3 X_4.$$

**Effect of a $Z$-measurement.** Measure qubit 2 of the linear 4-chain in the $Z$ basis, outcome $+1$. The vertex and its edges $1{-}2$ and $2{-}3$ vanish. The surviving graph on $\{1,3,4\}$ has only the edge $3{-}4$: qubit 1 is now isolated (its state is $|+\rangle$, since a degree-zero graph vertex is unentangled), and qubits 3, 4 form a two-qubit graph state — a Bell pair. A single $Z$-measurement has **severed the wire**, splitting a length-4 chain into a spectator and a Bell pair. The complementary fact drives the one-way computer: measuring the *interior* qubits in a tilted basis, rather than in $Z$, does not merely delete them but *pushes the quantum information along the chain*, applying a gate as it goes. That is the subject of Section 20.3.2.

The same surgery explains loss tolerance. If a photon in a cluster is lost, the environment has effectively performed an unwanted measurement on it; provided the encoding lets us treat that as a $Z$-measurement (a deletion) rather than an uncontrolled error, the graph simply loses a node and — if the lattice was built with enough redundancy to stay connected — the computation routes around the hole. Section 20.5 turns this observation into a fault-tolerance strategy.
