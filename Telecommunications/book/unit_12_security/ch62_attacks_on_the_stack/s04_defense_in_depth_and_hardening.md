# 62.4 Defence in Depth and Hardening

**The unit's closing argument, and a checklist you can actually apply.**

## Defence in depth, stated precisely

**The phrase is used loosely to mean "several security products".** **What it actually means is
narrower and more useful:**

> **No single control failure should result in compromise.**

**Which is a statement about independence, and it is Chapter 56 §56.2's shared-fate argument
applied to security controls.**

| | |
|---|---|
| **Layered** | **several controls in series, so bypassing one is not enough** |
| **Independent** | **and they must not share a failure mode** |
| **Diverse** | **different mechanisms, so one class of flaw does not defeat all of them** |

**The shared-fate question applies exactly as it did to redundancy:**

**Three firewalls from one vendor with one vulnerability are one firewall.** **Two controls
managed by the same compromised administrator account are one control.** **Detection and
prevention that both depend on the same signature feed are one control.**

**And the honest qualification:** **diversity has real costs** — **more products, more expertise,
more integration, more to go wrong** — **and "defence in depth" is frequently used to justify
buying another product rather than to assess whether the controls you have are actually
independent.**

**The useful exercise:** **for each of your controls, ask what single event defeats it, and see
how many of the answers are the same event.**

## Assume breach

**The corollary, and it changes what you build.**

> **Every control eventually fails.** **The question is not whether an attacker gets in, but
> what they reach and how quickly you notice.**

**Which is why the highest-value controls in this unit are not preventive:**

| Control | Kind | What it does when prevention has failed |
|---|---|---|
| **Segmentation** (Chapter 60 §60.4) | **containment** | **limits reach** |
| **Least privilege** (Chapter 59 §59.3) | **containment** | limits what the credential does |
| **MFA** (Chapter 59 §59.1) | preventive | **and it is the highest-value preventive control** |
| **Logging and baselines** (Chapter 54) | **detection** | **shortens the time to notice** |
| **Offline backups** (Chapter 56 §56.4) | **recovery** | **decides the ransomware outcome** |
| **Tested recovery** (Chapter 56) | recovery | **converts a catastrophe into an outage** |

**Chapter 57 §57.1's data supports this:** **the median time to detection is weeks or months, and
a significant share of breaches are reported by a third party.** **You are not going to catch the
intrusion; you are trying to limit and detect what follows.**

## The device hardening checklist

**Applicable to switches, routers, firewalls and wireless controllers.** **Every item prevents
something specific in this unit.**

### Management plane

```
   ✓ No default credentials, anywhere               (§57.1's opportunist)
   ✓ Individual accounts; no shared 'enable'         (Ch 59 §59.3)
   ✓ AAA to RADIUS/TACACS+, with local break-glass   (Ch 59 §59.2)
   ✓ MFA on any Internet-reachable administration    (Ch 59 §59.1)
   ✓ SSH only; Telnet and HTTP disabled              (Ch 23 §23.4's plaintext legacy)
   ✓ SSH v2, modern ciphers, key-based where possible
   ✓ Management on a separate VRF or network         (Ch 60 §60.4)
   ✓ ACL restricting management to jump hosts        
   ✓ Out-of-band access, tested                      (Ch 60 §60.4)
   ✓ Idle timeouts on sessions
   ✓ Login banner (legally useful in some jurisdictions)
   ✓ SNMPv3 authPriv; no v1/v2c; no read-write       (Ch 54 §54.2)
```

### Control plane

```
   ✓ Routing protocol authentication                 (§62.2)
   ✓ Control plane policing (CoPP) / rate limits
   ✓ BGP: prefix filters, max-prefix, RPKI           (Ch 32 §32.4)
   ✓ Unused routing protocols disabled
   ✓ NTP configured, authenticated, from two sources (Ch 54 §54.3)
```

### Data plane

```
   ✓ Access ports: mode access, nonegotiate          (§62.1)
   ✓ Port security with a sensible maximum
   ✓ BPDU guard + PortFast on access ports
   ✓ Root guard towards other switches
   ✓ DHCP snooping, with trusted uplinks
   ✓ Dynamic ARP inspection
   ✓ RA Guard and ND inspection for IPv6
   ✓ Native VLAN unused and tagged; VLAN 1 unused
   ✓ Unused ports shut down and in an unused VLAN
   ✓ Storm control
   ✓ uRPF or anti-spoof ACLs at the edge             (§62.2)
   ✓ CDP/LLDP disabled towards users
```

### Lifecycle and monitoring

```
   ✓ Firmware on the vendor's suggested release      (Ch 55 §55.3)
   ✓ EOL date recorded and reported 24 months out    (Ch 53 §53.2)
   ✓ Configuration backed up automatically, in git   (Ch 55 §55.4)
   ✓ Configuration compared against a golden template
   ✓ Syslog and flow export to a central collector   (Ch 54)
   ✓ Alerting on configuration change, on AAA change,
     on port security and BPDU guard violations
```

> **Every line is one command or two.** **The reason estates are not hardened is not difficulty;
> it is that nothing breaks when these are absent** (Chapter 55 §55.1's invisibility argument),
> **and nobody is thanked for the incident that did not happen** (Chapter 55 §55.1 again).

**Which is the argument for automating it** (Chapter 70): **a hardening standard that is a
template applied by a tool is a standard that is actually deployed**, and **a hardening standard
in a wiki is a document.**

## Where to spend, in order

**The unit's practical conclusion, ordered by effect per pound.**

| | Control | Addresses |
|---|---|---|
| **1** | **MFA on all remote access and administration** | **the commonest initial access route** (Ch 61 §61.4) |
| **2** | **No default credentials; no exposed management** | **§57.1's opportunist, which is certain** |
| **3** | **Patch what is known to be exploited** | Ch 55 §55.3's CISA KEV track |
| **4** | **Offline, credential-separated backups** | **decides the ransomware outcome** |
| **5** | **Segment the management plane and the backups** | Ch 60 §60.4 |
| **6** | **Central logging with a baseline** | **you will not prevent everything** |
| **7** | **Segment servers from workstations** | **the largest lateral-movement reduction** |
| **8** | **Layer 2 hardening as above** | cheap, and it stops accidents too |
| **9** | **802.1X** | Ch 59 §59.2 — **more work, and it is dynamic segmentation** |
| **10** | **Everything else** | |

> **Items 1 to 4 are cheap, certain and unglamorous, and an organisation that has done only
> those has changed its outcomes more than one that has bought three products and skipped
> them.** **This ordering is not what the security market sells and it is what the incident data
> supports.**

## What this unit has argued

**Six chapters, and the through-line is worth stating.**

**The properties are three and the list is complete** (Chapter 57). **Everything else is a
mechanism.**

**The cryptography works and the deployments fail** (Chapter 58). **Nonces, certificate
validation, key management and implementation — never the cipher.**

**Authentication is not authorization, and the second fails more often** (Chapter 59).

**Default deny, and the control must be where the traffic actually passes** (Chapter 60).

**A tunnel grants network access, and network access is the thing being reconsidered**
(Chapter 61).

**And the attacks are old, were documented before deployment, and remain effective because the
fixes cost someone other than the beneficiary** (Chapter 62; Chapter 57 §57.4's BCP 38).

> **The uncomfortable summary: almost nothing in this unit is new.** **Bellovin enumerated the
> protocol attacks in 1989. Saltzer and Schroeder gave the design principles in 1975.**
> **The gap between what is known and what is deployed is where security work actually
> happens**, and it is an organisational and economic gap rather than a technical one.

## What breaks here

**Three products from one vendor described as defence in depth.** **One vulnerability defeats
all three.** Ask what single event defeats each control.

**A hardening standard in a wiki.** **It is a document.** Automate it.

**Controls deployed and nothing instrumented.** Chapter 57 §57.4 — **a control with no detection
is a control you are trusting.**

**An organisation with an EDR product, a NGFW and no MFA.** **Items 1 to 4 were skipped.**

**Detection that depends on the same signature feed as prevention.** **Not independent.**

**A hardened perimeter and a flat interior.** **Assume breach.** The interior is where the
compromise will be.

**Everything hardened and the backups reachable with domain credentials.** Chapter 57 §57.1
step 5, **and it is the one that decides the outcome.**

> **Network+ note.** Objective 4.3 covers hardening directly, and this section is the
> consolidated list. Over-learn: **change default credentials, disable unused ports and
> services, use secure protocols (SSH not Telnet, SNMPv3 not v2c), apply patches, implement port
> security, DHCP snooping, DAI and BPDU guard, and separate the management network.**
> **Defence in depth means layered independent controls.** This section and §57.4 together cover
> the whole objective.
