# Chapter 15 — The People

**W. Wesley Peterson (1924–2009).** American mathematician and computer scientist
who invented the cyclic redundancy check, publishing *Cyclic Codes for Error
Detection* with Daniel Brown in 1961. The insight was that treating a message as a
polynomial over GF(2) and dividing by a carefully chosen generator produces a
remainder whose properties can be *proved* rather than merely tested — which is why
the guarantees in §15.4 are guarantees rather than statistics.

He spent most of his career at the University of Hawaii, where he was also a
distinguished scholar of the Japanese language and wrote a Japanese-English character
dictionary. His *Error-Correcting Codes* (1961, with E. J. Weldon) was the standard
text for a generation.

**Daniel T. Brown.** Peterson's co-author on the 1961 CRC paper, and generally
uncredited in the shorthand "Peterson's CRC".

**Richard Hamming (1915–1998).** Detection's counterpart. Frustrated by weekend
batch jobs failing on a single-bit error with no way to recover, he produced the
first practical error-**correcting** codes in 1950 — the first constructive step
toward Shannon's promise, and the other half of §15.4's tradeoff. See Chapter 4.

**Robert Metcalfe (b. 1946) and David Boggs (1950–2022).** The Ethernet frame format
is theirs, from the 1976 CACM paper *Ethernet: Distributed Packet Switching for
Local Computer Networks*. Boggs did much of the implementation work and is
consistently under-credited relative to Metcalfe; Metcalfe himself has been
scrupulous about correcting this.

The frame they defined in 1976, standardised as Ethernet II in 1982, is
**structurally unchanged today**. The medium, the coding, the topology, the
arbitration, the duplex model and the speed have all been replaced — four times over
— and the header layout has not. Chapter 16 §16.3 draws the general lesson about
stable interfaces; this is its cleanest instance.

**The DIX consortium (Digital, Intel, Xerox).** The three companies that published
the Ethernet II specification in 1980 and revised it in 1982, ahead of and in some
tension with the IEEE 802.3 process. The EtherType-versus-length ambiguity in §15.3
is the residue of that tension: DIX used the field as a type, IEEE used it as a
length, and the ≤1500/≥1536 rule is the compromise that let both coexist. DIX won in
practice.

**The IEEE 802 committee.** Responsible for the length-field variant, for the 802.2
LLC header that was supposed to identify protocols instead, and — more durably — for
the OUI registry that makes MAC addresses globally unique without central
allocation of individual addresses. Delegating 24 bits to manufacturers and letting
them manage the rest is a small and effective piece of administrative design.

**Jonathan Stone and Craig Partridge.** Their 2000 SIGCOMM paper *When the CRC and
TCP Checksum Disagree* measured real Internet traffic and found end-to-end checksum
failures at rates far above what link-layer error rates predicted — roughly one in
1,100 to one in 32,000 packets in some traces. They traced a substantial fraction to
**middleboxes, host software and memory errors** rather than to the wire.

The paper is the empirical basis for §15.4's warning that a passing FCS is not
end-to-end integrity, and it is one of the strongest measured arguments for the
end-to-end principle in the literature. Partridge also did foundational work on
multicast, on high-speed routing, and on the Internet's mail architecture.

**Vint Cerf and Jon Postel**, for the decision that IP would carry its own **Total
Length** field. §15.3 notes that Ethernet II has no length field and pads short
frames, so the upper layer must supply the real length. That IP does is why a
20-byte payload in a padded 64-byte frame is unambiguous, and it is a small example
of a layer being designed to not depend on a property the layer below happens to
have.

**Bram Cohen, and more relevantly the authors of the 802.11 privacy work.** MAC
randomisation as described in §15.2 emerged from research demonstrating that
retail analytics firms were tracking individuals through public spaces by passively
logging probe requests. The IEEE 802.11aq and later work, and Apple's and Google's
unilateral implementations, are a rare case of platform vendors deploying a privacy
protection that broke a widely used commercial practice, over that industry's
objections.
