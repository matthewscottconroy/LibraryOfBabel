# Chapter 16 — The People

**Norman Abramson (1932–2020).** American engineer and professor at the University
of Hawaii, where he had gone partly for the surfing and where the geography handed
him a research problem. ALOHAnet went live in June 1971 and answered a question that
had not previously arisen: how to share a channel among stations that cannot hear
each other.

The 18.4% figure is in his paper. He knew the protocol was inefficient and built it
anyway, because the alternative was leased inter-island telephone lines that the
budget could not support. He later described the design as the obvious thing to do
when coordination costs more than collisions, which is a fair summary of the whole
random-access family. He received the IEEE Alexander Graham Bell Medal in 2007.

**Franklin Kuo (b. 1934).** Abramson's principal collaborator on ALOHAnet and
co-author of the key papers. Consistently under-credited relative to Abramson.

**Lawrence Roberts (1937–2018).** Proposed **slotted ALOHA** in 1972, doubling the
ceiling to 36.8% by requiring transmissions to begin at slot boundaries. See
Chapter 13 for his ARPANET work.

**Robert Metcalfe (b. 1946).** Read Abramson's paper as a Harvard graduate student.
His doctoral thesis was rejected as insufficiently theoretical; he rewrote it around
an improved analysis of ALOHA's performance and it passed — a rejection that turned
out to matter, since it put him in possession of exactly the prior art PARC needed.

The 22 May 1973 memo describes Ethernet, and the name is a deliberate joke about the
**luminiferous aether**: a passive medium that carries waves and does not exist. He
left Xerox in 1979, founded 3Com, and spent the following decade selling Ethernet
against Token Ring — a commercial fight he won on cost. He received the Turing Award
in 2022, nearly fifty years after the memo. He has also been scrupulous about
correcting the record on Boggs's contribution.

**David Boggs (1950–2022).** Metcalfe's co-author and the person who did much of the
implementation — the transceivers, the interface hardware, and a great deal of the
measurement work that showed the design behaved as predicted. He is consistently
under-credited in popular accounts, and Metcalfe has repeatedly said so. He later
worked on high-speed networking at DEC and founded LAN Media Corporation.

**Butler Lampson (b. 1943) and Charles Thacker (1943–2017).** Named on the Ethernet
patent alongside Metcalfe and Boggs. Thacker designed the Alto — the machine
Ethernet was built to connect — and later much of PARC's hardware; Lampson
contributed across PARC's systems work. Both received Turing Awards, Thacker in 2009
and Lampson in 1992.

**Ronald Crane.** Designed much of the transceiver electronics for the DIX Ethernet
specification, and is credited within the field with a great deal of the practical
engineering that made 10 Mb/s work reliably over coax. Almost never mentioned outside
it.

**Olof Söderblom (b. 1940).** Patented token ring in 1967 and 1969, before IBM's
development. IBM licensed his patents, as did others after litigation, and he
collected royalties on a technology he did not commercialise. Token Ring's
determinism — the property §16.3 identifies as genuinely superior and ultimately
irrelevant — is his mechanism.

**Rich Seifert.** Chaired or contributed to several IEEE 802.3 working groups
including 100BASE-T and full-duplex operation, and wrote *The Switch Book*, which is
the standard reference for the mechanisms in Chapter 17. He is one of the small
number of people who can explain both what the standard says and why the committee
decided it.

**Geoff Thompson.** Long-serving 802.3 participant and chair, involved from the
early standardisation through the gigabit and 10-gigabit work. Much of §16.3's
"interface stayed stable while the implementation was replaced" is a description of
decisions taken repeatedly by committees he sat on, and the consistency of that
choice over four decades is not accidental.

**The 802.3bz task force (2014–2016).** Produced 2.5GBASE-T and 5GBASE-T
specifically to extract multi-gigabit rates from the installed Cat5e and Cat6 base,
having measured what that cable could actually support rather than what its
specification promised. A clear case of a standards body responding to an economic
constraint — the impossibility of recabling the world — with a technical answer.
