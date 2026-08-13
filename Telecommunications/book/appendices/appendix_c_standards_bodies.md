# Appendix C — Standards Bodies and How a Standard Happens

Networking works because independently written implementations interoperate, and they
interoperate because of documents produced by organisations most engineers never think
about. This appendix covers who produces what, and — more usefully — how the process
differs between them, because the process shapes the result.

---

## C.1 Who Does What

| Body | Domain | Output | Access |
|---|---|---|---|
| **IETF** | Internet protocols: IP, TCP, DNS, HTTP, TLS, BGP | RFCs | Free; open participation |
| **IEEE** | Physical and link layers: Ethernet (802.3), Wi-Fi (802.11), VLANs (802.1Q) | 802.x standards | Paid, though many are freely available after six months via IEEE GET |
| **ITU-T** | International telecommunications: optical transport, DSL, video coding | Recommendations (G., V., H., Y. series) | Paid; some free |
| **ISO/IEC** | The OSI model, cabling standards, information security (27000 series) | ISO standards | Paid |
| **ANSI/TIA** | Structured cabling in North America (TIA-568) | TIA standards | Paid |
| **3GPP** | Cellular: LTE, 5G NR, and the core network | Technical Specifications, by Release | Free |
| **W3C** | Web technologies: HTML, CSS, web APIs | Recommendations | Free |
| **ICANN / IANA** | Names, numbers, the DNS root, protocol parameter registries | Policy, registries | Free |
| **Wi-Fi Alliance** | Interoperability certification and the "Wi-Fi 6" marketing names | Certification programmes | Membership |
| **CSA** | Zigbee, Matter | Specifications | Membership |
| **MEF** | Carrier Ethernet service definitions | Technical specifications | Membership |

The division that catches people out: **IEEE standardises Ethernet and Wi-Fi; the IETF
standardises everything above the link layer.** So the frame format of Chapter 15 is
IEEE 802.3, and the packet inside it is IETF RFC 791, and these are different
organisations with different cultures, different funding and different documents.

---

## C.2 The IETF, and Why It Is Unusual

The Internet Engineering Task Force has **no membership**. There is no application, no
fee, and no organisational representation. You participate by joining a mailing list
and contributing, and your affiliation carries no formal weight — a contribution from a
graduate student and one from a large vendor are procedurally identical.

Decisions are by **rough consensus**, judged by a working group chair, not by vote.
The absence of voting is deliberate: voting rewards whoever can recruit the most
participants, which would hand the process to whoever has the most employees.

The guiding sentiment is Dave Clark's, from a 1992 presentation:

> *"We reject: kings, presidents and voting. We believe in: rough consensus and running
> code."*

That last clause is the load-bearing one, and it is the difference Chapter 22 §22.1
identifies as decisive against OSI. The IETF strongly prefers specifications that have
been implemented and tested over specifications that have been designed. Interoperable
running code is treated as evidence that the specification is adequate; its absence is
treated as evidence that it is not.

### How a document progresses

```
  idea → mailing list discussion → Internet-Draft (draft-author-topic-NN)
       → working group adoption (draft-ietf-wg-topic-NN)
       → working group last call → IETF last call → IESG review
       → RFC publication
```

Internet-Drafts expire after six months, which is a deliberate forcing function: a
draft that nobody is working on disappears rather than lingering as an apparent
standard.

### RFC categories

**Not every RFC is a standard**, and the distinction matters when you cite one.

| Status | Meaning |
|---|---|
| **Internet Standard** | Mature, widely deployed, interoperability demonstrated |
| **Proposed Standard** | Stable and reviewed; most protocols in production stop here |
| **Best Current Practice (BCP)** | Operational guidance rather than a protocol |
| **Informational** | Published for the record; not a standard |
| **Experimental** | Under investigation |
| **Historic** | Superseded or abandoned |

Two things follow. Most of what you use daily — including a great deal of TCP — is
Proposed Standard, because advancement is laborious and nobody bothers once something
works. And the series includes the annual **1 April** RFCs: RFC 1149, *A Standard for
the Transmission of IP Datagrams on Avian Carriers*, is real, is numbered, and was
implemented in Bergen in 2001 with a measured packet loss of 55%.

Check `rfc-editor.org` for a document's current status and whether it has been obsoleted
before relying on it. Reading an obsolete RFC and quoting it confidently is a common
and avoidable embarrassment.

---

## C.3 The IEEE, and Why Its Documents Cost Money

The Institute of Electrical and Electronics Engineers works differently: formal
membership, balloting by qualified voters, and standards sold to fund the process.

The naming convention is worth being able to read. **802.3** is Ethernet; the letters
appended identify amendments — **802.3af** is Power over Ethernet, **802.3bz** is
2.5G/5GBASE-T, **802.3bt** is the 90 W PoE. Amendments are periodically rolled into a
revised base standard, at which point the letters disappear and the feature is simply
part of 802.3.

The paywall is a genuine barrier to learners, and the partial remedy is the **IEEE GET
Program**, which makes many 802 standards freely downloadable six months after
publication. Use it; reading the actual specification is a different experience from
reading summaries of it.

---

## C.4 The ITU-T

The International Telecommunication Union is a United Nations agency, and its
membership is national governments plus sector members. This gives it a very different
character: its recommendations carry the weight of international treaty obligations in
some domains, its process is slower and more formal, and its documents assume a
carrier rather than an enterprise perspective.

Series worth recognising:

| Series | Subject | Examples |
|---|---|---|
| G | Transmission systems | G.652 (fibre), G.709 (OTN), G.992 (ADSL), G.114 (delay) |
| V | Data over telephone | V.34, V.90 (modems) |
| H | Audiovisual | H.264, H.265, H.323 |
| Y | Next-generation networks | Y.1541 (QoS classes) |
| X | Data networks | X.25, X.509 (certificates) |

X.509 is worth noting: the certificate format underlying all of TLS (Chapter 58 §58.4)
comes from an ITU-T directory standard, which is why its structure is more elaborate
than a web-focused design would be.

---

## C.5 3GPP

The Third Generation Partnership Project produces cellular standards, and its structure
is unusual: it is a partnership of seven regional standards organisations, and its
output is organised into **Releases** rather than named standards.

| Release | Introduced |
|---|---|
| 8 (2008) | LTE |
| 10 (2011) | LTE-Advanced |
| 15 (2018) | 5G NR (non-standalone) |
| 16 (2020) | 5G standalone, URLLC |
| 17 (2022) | NTN (satellite), RedCap |
| 18 (2024) | "5G-Advanced" |

Specifications are free, which makes 3GPP unusually accessible for a body of its size,
and the numbering (TS 23.501, TS 38.300 and so on) is worth learning if you work near
cellular.

---

## C.6 How to Read a Standard

A practical note, because reading specifications is a learnable skill that most courses
never teach.

**Read the abstract and the introduction first**, then the terminology section, then
skip to the part you need. Standards are reference documents and are not written to be
read linearly.

**Learn RFC 2119 language.** In IETF documents, capitalised **MUST**, **MUST NOT**,
**SHOULD**, **SHOULD NOT** and **MAY** have precise defined meanings, and the difference
between MUST and SHOULD is the difference between a conformance requirement and a
recommendation. A great deal of interoperability trouble lives in the SHOULDs.

**The Security Considerations section is mandatory in every RFC** and is often the most
interesting part — it is where the authors state what their protocol does not protect
against, and it is frequently more candid than any other document you will find on the
subject.

**Check the errata.** Published RFCs accumulate corrections at `rfc-editor.org`, and
some are substantive.

**Find the deployment reality.** A standard describes what implementations should do.
What they actually do is documented in mailing list archives, vendor release notes, and
the occasional measurement paper — and the gap between the two is where a great many
operational problems live.
