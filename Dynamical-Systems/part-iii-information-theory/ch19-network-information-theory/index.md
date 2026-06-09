# Chapter 19 — Network Information Theory

> *Shannon's theory covers one sender and one receiver. The real world has networks. Network information theory asks: what are the capacity limits of multi-user communication systems?*

**Prerequisites:** Chapter 16 (classical information theory, channel capacity).

---

Shannon's 1948 theory is, at its core, a point-to-point theory. One source, one channel, one receiver. This is already rich enough to fill a career, and the mathematical framework it provides — entropy, mutual information, typical sequences — is indispensable.

But the real world does not communicate point-to-point. The internet is a network with billions of nodes. A cellular network has thousands of users sharing spectrum. A satellite downlinks to a continent's worth of receivers simultaneously. A relay station in the middle of the ocean helps a ship communicate with the mainland. Every one of these situations involves multiple senders, multiple receivers, and shared resources — and the Shannon theory of Chapter 16 is simply not equipped to handle them.

Network information theory extends Shannon's framework to multi-user systems. The goal is the same: find the exact capacity limits. But the structure is richer, the geometry is multi-dimensional (capacity *regions* instead of a single capacity number), and many problems remain open after fifty years of effort.

This chapter covers the major cases that are understood: the multiple access channel (many senders, one receiver), the broadcast channel (one sender, many receivers), distributed source coding (Slepian-Wolf), lossy coding with side information (Wyner-Ziv), relay channels, and information-theoretic security. We also note honestly where the theory runs out — the capacity of the relay channel, the general broadcast channel, and the interference channel all remain open.

**What this chapter builds:**
- Multiple access channels and their capacity regions
- Broadcast channels and superposition coding
- Slepian-Wolf distributed source coding: the surprising power of separate encoding
- Wyner-Ziv coding with decoder side information
- Relay channels and cooperative communication
- Shannon's perfect secrecy and the wiretap channel
- Secret key agreement

**Sections:**
- [19.1 Multiple Access Channels](multiple-access-channels.md)
- [19.2 Broadcast Channels](broadcast-channels.md)
- [19.3 Distributed Source Coding — Slepian-Wolf](slepian-wolf.md)
- [19.4 Wyner-Ziv — Lossy Coding with Side Information](wyner-ziv.md)
- [19.5 Relay Channels](relay-channels.md)
- [19.6 Information-Theoretic Security](information-theoretic-security.md)
- [19.7 Secret Key Agreement](secret-key-agreement.md)
- [Exercises](exercises.md)
- [Notes](notes.md)
