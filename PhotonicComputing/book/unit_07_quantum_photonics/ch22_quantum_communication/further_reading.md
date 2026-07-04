# Chapter 22: Further Reading and References

---

## Reviews: Start Here

**Gisin, N., Ribordy, G., Tittel, W., & Zbinden, H. (2002). "Quantum cryptography." *Reviews of Modern Physics*, 74(1), 145–195.**
The classic review of QKD protocols and early implementations; still the clearest first pass through BB84, E91, and the practical issues of fibre systems.

**Pirandola, S., Andersen, U.L., Banchi, L., et al. (2020). "Advances in quantum cryptography." *Advances in Optics and Photonics*, 12(4), 1012–1236.**
The definitive modern survey — discrete- and continuous-variable QKD, MDI and twin-field protocols, device independence, and the security-proof landscape in one authoritative reference.

**Xu, F., Ma, X., Zhang, Q., Lo, H.-K., & Pan, J.-W. (2020). "Secure quantum key distribution with realistic devices." *Reviews of Modern Physics*, 92(2), 025002.**
Focused on the gap between idealized proofs and real hardware: side channels, finite-key effects, decoy states, and the attacks that motivate MDI-QKD.

**Sangouard, N., Simon, C., de Riedmatten, H., & Gisin, N. (2011). "Quantum repeaters based on atomic ensembles and linear optics." *Reviews of Modern Physics*, 83(1), 33–80.**
The comprehensive repeater-architecture review — memory requirements, rate scaling, and the entanglement-loading arithmetic used in Section 22.3.2.

**Kimble, H.J. (2008). "The quantum internet." *Nature*, 453, 1023–1030.**
The manifesto that named the vision; the source of the "entanglement as network resource" framing of Section 22.3.

**Wehner, S., Elkouss, D., & Hanson, R. (2018). "Quantum internet: A vision for the road ahead." *Science*, 362(6412), eaam9288.**
The six-stage capability roadmap that structures Section 22.3.1 — the standard taxonomy of the field.

---

## QKD Protocols

**Bennett, C.H. & Brassard, G. (1984). "Quantum cryptography: Public key distribution and coin tossing." *Proc. IEEE Int. Conf. Computers, Systems and Signal Processing*, Bangalore, 175–179.**
BB84 — the founding protocol. (Reprinted in *Theoretical Computer Science*, 560, 7–11, 2014.)

**Ekert, A.K. (1991). "Quantum cryptography based on Bell's theorem." *Physical Review Letters*, 67(6), 661–663.**
E91: security from CHSH violation, the root of device-independent QKD.

**Bennett, C.H., Brassard, G., & Mermin, N.D. (1992). "Quantum cryptography without Bell's theorem." *Physical Review Letters*, 68(5), 557–559.**
BBM92: entanglement-based QKD with an untrusted source, formally equivalent to BB84 — the protocol Micius ran over 1,120 km.

**Shor, P.W. & Preskill, J. (2000). "Simple proof of security of the BB84 quantum key distribution protocol." *Physical Review Letters*, 85(2), 441–444.**
The clean entanglement-distillation security proof behind the $r = 1 - 2h(Q)$ key fraction and the 11% threshold.

**Lo, H.-K., Ma, X., & Chen, K. (2005). "Decoy state quantum key distribution." *Physical Review Letters*, 94(23), 230504.**
Decoy states made rigorous — the fix for the photon-number-splitting attack and the reason weak-coherent-pulse QKD is practical.

**Lo, H.-K., Curty, M., & Qi, B. (2012). "Measurement-device-independent quantum key distribution." *Physical Review Letters*, 108(13), 130503.**
MDI-QKD: an untrusted Bell-measurement relay eliminates all detector side channels.

**Lucamarini, M., Yuan, Z.L., Dynes, J.F., & Shields, A.J. (2018). "Overcoming the rate–distance limit of quantum key distribution without quantum repeaters." *Nature*, 557, 400–403.**
Twin-field QKD: $\sqrt{\eta}$ scaling that breaks the PLOB bound with a single untrusted interference node.

**Grosshans, F. & Grangier, P. (2002). "Continuous variable quantum cryptography using coherent states." *Physical Review Letters*, 88(5), 057902.**
GG02 — CV-QKD from Gaussian-modulated coherent states read by homodyne detection.

---

## Implementations and Records

**Bennett, C.H., Bessette, F., Brassard, G., Salvail, L., & Smolin, J. (1992). "Experimental quantum cryptography." *Journal of Cryptology*, 5(1), 3–28.**
The first working QKD system — 32 cm of free space — turning the 1984 proposal into hardware.

**Pirandola, S., Laurenza, R., Ottaviani, C., & Banchi, L. (2017). "Fundamental limits of repeaterless quantum communications." *Nature Communications*, 8, 15043.**
The PLOB bound: $K \le -\log_2(1-\eta)$, the protocol-independent ceiling every long-distance scheme is measured against.

**Boaron, A., et al. (2018). "Secure quantum key distribution over 421 km of optical fiber." *Physical Review Letters*, 121(19), 190502.**
The decoy-BB84 fibre-distance record, enabled by ultra-low-loss fibre and low-dark-count SNSPDs (Chapter 19).

**Liu, Y., et al. (2023). "Experimental twin-field quantum key distribution over 1000 km fiber distance." *Physical Review Letters*, 130(21), 210801.**
Twin-field QKD across 1,002 km — the first key exchange over 1,000 km of fibre.

---

## Repeaters and Networks

**Briegel, H.-J., Dür, W., Cirac, J.I., & Zoller, P. (1998). "Quantum repeaters: The role of imperfect local operations in quantum communication." *Physical Review Letters*, 81(26), 5932–5935.**
The founding repeater proposal — nested swapping and purification that turn exponential loss into polynomial overhead.

**Duan, L.-M., Lukin, M.D., Cirac, J.I., & Zoller, P. (2001). "Long-distance quantum communication with atomic ensembles and linear optics." *Nature*, 414, 413–418.**
The DLCZ protocol: atomic ensembles as combined source, herald, and memory — the template for ensemble-based repeaters.

**Afzelius, M., Simon, C., de Riedmatten, H., & Gisin, N. (2009). "Multimode quantum memory based on atomic frequency combs." *Physical Review A*, 79(5), 052329.**
The AFC memory — massive temporal multimode storage in rare-earth crystals (Section 22.2.2).

**Hensen, B., et al. (2015). "Loophole-free Bell inequality violation using electron spins separated by 1.3 kilometres." *Nature*, 526, 682–686.**
Heralded entanglement between distant NV nodes closing all loopholes — the matter-qubit link at the heart of the Delft network.

**Dahlberg, A., Skrzypczyk, M., Coopmans, T., et al. (2019). "A link layer protocol for quantum networks." *Proc. ACM SIGCOMM 2019*, 159–173.**
The first link-layer protocol turning probabilistic entanglement generation into a robust on-request service (Section 22.3.2).

**Pompili, M., Hermans, S.L.N., Baier, S., et al. (2021). "Realization of a multinode quantum network of remote solid-state qubits." *Science*, 372(6539), 259–264.**
The Delft three-node NV network — the first integrated multi-node quantum network.

**Hermans, S.L.N., Pompili, M., Beukers, H.K.C., Baier, S., Borregaard, J., & Hanson, R. (2022). "Qubit teleportation between non-neighbouring nodes in a quantum network." *Nature*, 605, 663–668.**
Teleportation across an intermediate swap — the first end-to-end Stage-4 network primitive.

---

## Satellite Quantum Communication

**Liao, S.-K., Cai, W.-Q., Liu, W.-Y., et al. (2017). "Satellite-to-ground quantum key distribution." *Nature*, 549, 43–47.**
Micius delivers decoy-state keys over 645–1,200 km — free-space diffraction beating fibre absorption by $\sim 20$ orders of magnitude.

**Yin, J., Cao, Y., Li, Y.-H., et al. (2017). "Satellite-based entanglement distribution over 1200 kilometres." *Science*, 356(6343), 1140–1144.**
Entangled pairs split between ground stations 1,203 km apart — the longest-distance Bell test performed.

**Ren, J.-G., Xu, P., Yong, H.-L., et al. (2017). "Ground-to-satellite quantum teleportation." *Nature*, 549, 70–73.**
Teleportation of single-photon states up to the satellite over uplinks to 1,400 km.

**Yin, J., Li, Y.-H., Liao, S.-K., et al. (2020). "Entanglement-based secure quantum cryptography over 1,120 kilometres." *Nature*, 582, 501–505.**
BBM92 with an untrusted satellite source — Stage-3 capability at continental scale.

**Chen, Y.-A., Zhang, Q., Chen, T.-Y., et al. (2021). "An integrated space-to-ground quantum communication network over 4,600 kilometres." *Nature*, 589, 214–219.**
Micius fused with the 2,000 km Beijing–Shanghai backbone into a 4,600 km, 150+-user network.

**Lu, C.-Y., Cao, Y., Peng, C.-Z., & Pan, J.-W. (2022). "Micius quantum experiments in space." *Reviews of Modern Physics*, 94(3), 035001.**
The comprehensive review of the entire Micius program.

---

## Textbooks

**Nielsen, M.A. & Chuang, I.L. (2010). *Quantum Computation and Quantum Information* (10th Anniversary Edition). Cambridge University Press.**
The standard reference for no-cloning, teleportation, entanglement measures, and the information theory behind QKD security.

**Van Meter, R. (2014). *Quantum Networking*. Wiley-ISTE.**
The book-length treatment of quantum network architecture, entanglement routing, and repeater engineering underlying Section 22.3.2.
