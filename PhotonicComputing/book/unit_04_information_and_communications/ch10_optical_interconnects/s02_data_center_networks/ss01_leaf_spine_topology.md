# Subsection 10.2.1: Leaf-Spine Topology and AI Cluster Networks

## Orientation

The topology of a data center network — how servers are connected to switches, how switches are connected to other switches — determines almost everything about the network's performance characteristics: its bandwidth, its latency, its resilience to failures, its cost, and its suitability for different workloads. For the past decade, hyperscale data centers have converged on a family of topologies called *fat-tree* or *Clos* networks, implemented as *leaf-spine* architectures. Understanding these topologies is essential for understanding where optical interconnects fit and what they enable.

This subsection develops the topology from first principles, derives the bandwidth requirements for AI training clusters, and explains why the communication patterns of large-scale machine learning are creating a genuine crisis in data center networking.

---

## 10.2.1.1 The Leaf-Spine Architecture

### From Trees to Fat Trees

The naive approach to building a network of $N$ servers is a tree: connect groups of servers to access switches, connect access switches to aggregation switches, connect aggregation switches to a core switch. This is simple but problematic. The core switch at the top of the tree is a bottleneck: every packet that needs to cross from one part of the network to another must pass through it. More fundamentally, the total bandwidth available decreases as you travel up the tree — the core switch has far less aggregate bandwidth than the sum of all the edge links below it.

The ratio of total edge bandwidth to total backbone bandwidth is called the *oversubscription ratio*. A traditional enterprise network might have 20:1 oversubscription (the backbone handles 1/20th of the theoretical edge capacity), which is acceptable for office applications with bursty, non-concurrent traffic patterns. For high-performance computing or AI training, it is catastrophic.

The fat-tree topology, introduced to the networking community by Al-Fares, Loukissas, and Vahdat in 2008 [1], solves this by using many parallel paths between any two servers and building the network from identical commodity switches. The key insight: if you build a $k$-ary fat-tree using switches with $k$ ports each, you can achieve *non-blocking* connectivity — full bandwidth between any pair of servers — for a network of $k^3/4$ servers.

### The $k$-ary Fat-Tree

A $k$-ary fat-tree has three layers:
- **Edge layer (access switches)**: $(k^2/4)$ switches, each connecting $k/2$ servers and $k/2$ uplinks
- **Aggregation layer**: $(k^2/4)$ switches, each connecting $k/2$ downlinks (to edge) and $k/2$ uplinks (to core)
- **Core layer**: $(k/2)^2$ switches, each connecting $k$ downlinks (one to each pod)

Each group of one edge switch + one aggregation switch + their connected servers forms a *pod*; there are $k$ pods.

**Total servers**: Each edge switch connects $k/2$ servers, there are $k/2$ edge switches per pod, and $k$ pods:

$$N_{\text{servers}} = \frac{k}{2} \cdot \frac{k}{2} \cdot k = \frac{k^3}{4}$$

**Total bandwidth analysis**: Consider the bisection bandwidth — the minimum bandwidth across any cut that divides the network in half. For the $k$-ary fat-tree with 10 Gbps links:

$$B_{\text{bisection}} = \left(\frac{k}{2}\right)^2 \cdot B_{\text{link}}$$

where $(k/2)^2$ is the number of core switches, each carrying one link across the bisection. For $k = 48$ (a common modern scale) with 400 Gbps links:

$$B_{\text{bisection}} = 24^2 \cdot 400 \text{ Gbps} = 576 \times 400 \text{ Gbps} = 230 \text{ Tbps}$$

This serves $48^3/4 = 27{,}648$ servers, each with $48/2 = 24$ server-facing ports of 400 Gbps = 9.6 Tbps per server, for a total edge bandwidth of $27{,}648 \times 9.6 \text{ Tbps} \approx 265 \text{ Pbps}$. The ratio is not 1:1, because real fat-trees use oversubscription at the edge — in the formula above, each edge switch provides 24 uplinks for 24 servers, which is 1:1, but in practice oversubscription is often 2:1 or 4:1 to reduce cost.

---

## 10.2.1.2 The Leaf-Spine Simplification

In practice, hyperscale data centers use a two-layer variant called leaf-spine that trades the mathematical elegance of the full fat-tree for operational simplicity:

- **Leaf switches**: connect directly to servers (typically 48 × 25 GbE server ports + 8 × 100 GbE uplinks per switch)
- **Spine switches**: connect only to leaf switches (typically 32 × 100 GbE or 64 × 400 GbE)

The oversubscription ratio for a leaf-spine network is:

$$\text{Oversubscription} = \frac{N_{\text{server ports}} \times B_{\text{server link}}}{N_{\text{uplinks}} \times B_{\text{uplink}}}$$

For a 48-server leaf with 25 GbE server ports and 8 × 100 GbE uplinks:

$$\text{Oversubscription} = \frac{48 \times 25}{8 \times 100} = \frac{1200}{800} = 1.5:1$$

This is considered excellent for most workloads. The total number of servers in a leaf-spine pod is $N_{\text{leaf}} \times N_{\text{server-ports}} = N_{\text{leaf}} \times 48$. If there are $N_{\text{spine}} = 8$ spine switches and each leaf has 8 uplinks, the bisection bandwidth per server is:

$$B_{\text{per-server, bisection}} = \frac{N_{\text{uplinks}} \times B_{\text{uplink}}}{N_{\text{server-ports}}} = \frac{8 \times 100}{48} \approx 16.7 \text{ Gbps}$$

---

## 10.2.1.3 AI Training and the All-Reduce Problem

### Why AI Training Is Different

For most workloads — web requests, database queries, video streaming — data center traffic is *east-west* (server to server) but relatively sparse: each server communicates with a handful of other servers at any given moment. The fat-tree topology, with its many parallel paths, handles this well.

Large-scale AI training is fundamentally different. Distributed training of large language models (LLMs) like GPT-4, Llama, or Gemini uses techniques called *data parallelism* and *tensor parallelism* that require collective communication operations among all participating GPUs. The most important of these is *all-reduce*.

**Data parallelism**: $N$ copies of the model train on different batches simultaneously; after each gradient step, all $N$ copies must average their gradients — this is the all-reduce operation.

**Tensor parallelism**: The model itself is split across $N$ accelerators, which must exchange activation tensors at each layer boundary — another form of collective communication.

**Pipeline parallelism**: The model is split into stages running on different accelerators, which must pass micro-batches between stages — more communication.

For a model with $P$ parameters (e.g., $P = 70 \times 10^9$ for Llama-2-70B, stored in float16 = 2 bytes):

$$\text{Data volume per all-reduce} = 2P \cdot (\text{bytes/parameter}) = 2 \times 70 \times 10^9 \times 2 = 280 \text{ GB}$$

(The factor of 2 comes from the send + receive in ring-allreduce.) For a ring-allreduce across $N$ nodes, each node sends $2(N-1)/N$ times its local data, approaching $2P$ bytes total per global synchronization.

**Training throughput requirement**: A typical LLM training run achieves roughly:
- $T_{\text{compute}} \approx 1{,}000$ tokens/second on an 8-GPU node
- Sequence length $L = 2048$ tokens, batch size $B_{\text{global}} = 4096$ sequences
- Gradient synchronization period: one full forward+backward pass, $\approx 10$ seconds at this rate

The required network bandwidth to not be communication-bottlenecked:

$$B_{\text{required}} = \frac{\text{Data per all-reduce}}{T_{\text{compute/allreduce}}} = \frac{280 \text{ GB}}{10 \text{ s}} = 28 \text{ GB/s} = 224 \text{ Gbps per node}$$

For a cluster of $N = 1024$ nodes, the *total* all-reduce traffic is $N \times 224 \text{ Gbps} / 2 = 115 \text{ Tbps}$ (divided by 2 because each bit is sent once and received once). This must traverse the spine layer.

### The Bandwidth Gap

Modern GPU training nodes (NVIDIA DGX H100) have:
- 8 × H100 GPUs per node
- NVLink 4.0 connecting GPUs within the node: 900 GB/s bidirectional per GPU
- Network: 8 × 400 GbE NICs (InfiniBand HDR200 or 400 GbE) = 8 × 400 Gbps = 3.2 Tbps per node

The NVLink bandwidth within a node is orders of magnitude higher than the network bandwidth leaving the node. This means:
1. Intra-node communication (tensor parallel) should be maximized
2. Inter-node communication (data/pipeline parallel) is the bottleneck
3. The network fabric must provide near-full bandwidth to every node simultaneously

For a spine-layer switch handling 128 nodes, each with 400 Gbps uplinks:
$$B_{\text{spine}} = 128 \times 400 \text{ Gbps} = 51.2 \text{ Tbps}$$

This is exactly the capacity of a single Broadcom Tomahawk 5 / Cisco Silicon One G200 switch (2023). A fabric connecting 1024 nodes requires 8 spine switches, each providing this bandwidth.

---

## 10.2.1.4 The Optical Infrastructure

Every link in a modern hyperscale spine layer is optical. The distances involved range from meters (within a pod) to hundreds of meters (cross-pod), with a small fraction at 1–10 km (inter-building). The optical infrastructure is:

**Short reach (< 100 m)**: Direct Attach Copper (DAC) cables — passive copper, no optics.

**Medium reach (100 m – 2 km)**: Active optical cables (AOC) or pluggable transceivers (QSFP-DD, OSFP) using 850 nm VCSEL arrays (SR optics). 8 lanes × 50 Gbps = 400G per fiber bundle.

**Long reach (2–80 km)**: 1310 nm or 1550 nm single-mode fiber with DFB lasers (LR4, ER4). Individual 100 Gbps NRZ channels; 4 × 100 Gbps = 400G.

**Very long reach (80–120 km, inter-data-center)**: Coherent transceivers using DP-QPSK or DP-16QAM with DSP. 400G or 800G per single fiber pair.

The quantities involved are staggering: Google's data center network reportedly uses more than 10 million optical transceiver modules [2]. At $100 per transceiver, this represents $1B in optical components for a single hyperscaler's network — before counting the fiber, the switches, or the installation.

**The co-packaged optics transition** (covered in Section 10.1.2) addresses the energy cost of the electrical-to-optical conversion at the switch ASIC boundary: the pluggable transceiver is moved from a faceplate connector to the same package as the switch chip, cutting the SerDes power by ~4×. As discussed there, the target is ~100 fJ/bit for the optical transceiver itself.

---

## 10.2.1.5 Bandwidth Scaling and the Future

Moore's Law, as applied to network bandwidth, has continued longer than Moore's Law for transistor counts — but it is running into limits. The optical transceiver roadmap:

| Year | Per-fiber rate | Per-module rate | Power |
|------|----------------|-----------------|-------|
| 2020 | 100 Gbps (NRZ) | 400G (4 fibers) | 3.5 W |
| 2023 | 200 Gbps (PAM4) | 800G (4 fibers) | 5.5 W |
| 2025 | 400 Gbps (PAM4) | 1.6T (4 fibers) | 7–8 W |
| 2028 | 800 Gbps (coherent) | 3.2T | 10–15 W |

*Sources: IEEE 802.3 roadmap; Optical Internetworking Forum (OIF) [3]*

Each doubling of per-module rate is achieved through a combination of baud rate scaling (more GBd/s), higher-order modulation (2→4→8 bits/symbol), and spatial multiplexing (more fibers). Each step forward requires more DSP complexity, better forward error correction, higher-precision DACs and ADCs, and more power.

This is the physics of Shannon's channel capacity applied to the practical constraints of data center economics: you can always carry more information per fiber if you are willing to spend more on DSP and optics. The question is whether the power budget is available, and increasingly it is not — the total power of a data center is a fixed resource, and optical networking already consumes 10–15% of it.

---

## References

[1] Al-Fares, M., Loukissas, A., & Vahdat, A. (2008). "A scalable, commodity data center network architecture." *Proceedings of ACM SIGCOMM 2008*, 63–74. [The fat-tree paper; defines the architecture now used in virtually every hyperscale data center.]

[2] Vahdat, A. (2022). "Optical networking at Google." Optical Fiber Communication Conference (OFC 2022), Keynote. [Vahdat is VP of Network Engineering at Google; his keynotes provide the most reliable public data on hyperscale optical infrastructure.]

[3] Optical Internetworking Forum. (2023). *400ZR and 800G Coherent DWDM MSA*. OIF Technical Specifications. [Defines the transceiver specifications for coherent data center interconnects.]

[4] Narayanan, D., et al. (2021). "Efficient large-scale language model training on GPU clusters using Megatron-LM." *SC '21: Proceedings of the International Conference for High Performance Computing, Networking, Storage and Analysis*. [The paper that codified tensor/pipeline/data parallelism for LLM training and established the bandwidth requirements now common at hyperscale.]

[5] Lepikhin, D., et al. (2021). "GShard: Scaling giant models with conditional computation and automatic sharding." *ICLR 2021*. [Early large-scale MoE training demonstrating the all-reduce bandwidth bottleneck.]
