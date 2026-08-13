# Lab 12 — Segmentation, Firewalls, and TLS

**Corresponds to:** Chapters 58, 60, 62
**Week:** 12
**Time:** 120 minutes

---

## Objectives

- Write, order and debug an access control list, including a shadowed rule.
- Demonstrate the difference between stateless filtering and stateful inspection.
- Reproduce the asymmetric-routing failure that breaks stateful firewalls.
- Segment a network and measure the reduction in lateral reach.
- Read a TLS handshake, examine a certificate chain, and diagnose validation
  failures.
- Perform and detect the layer-two attacks from Chapter 62, on an isolated
  network.

---

## You will need

- The routed lab network from week 7.
- A Linux host acting as a firewall (`iptables`/`nftables`) or a firewall
  appliance.
- `openssl`, `curl`, Wireshark.
- **An isolated segment** for Part 5. Nothing in Part 5 goes near a production
  network.

---

## Procedure

### Part 1 — The access control list

**1.** On your firewall host, set a default-deny policy for forwarded traffic and
permit nothing:

```bash
sudo iptables -P FORWARD DROP
sudo iptables -F FORWARD
```

**2.** Verify that everything is now blocked. Record the failure mode — is it a
timeout or a refusal? Explain the difference and which one a `DROP` produces.

**3.** Change the last rule to `REJECT` instead and retest. Record the difference
in observed behaviour and in the time to fail.

**4.** Now permit web traffic from one subnet to one server, statelessly:

```bash
sudo iptables -A FORWARD -s 10.1.1.0/24 -d 10.3.3.10 -p tcp --dport 443 -j ACCEPT
```

**5.** Test. It fails. Explain why before adding anything, then add the return
rule a stateless filter requires:

```bash
sudo iptables -A FORWARD -s 10.3.3.10 -d 10.1.1.0/24 -p tcp --sport 443 -j ACCEPT
```

**6.** Now demonstrate the weakness you just created. From the server side, source
traffic **from port 443** to an arbitrary high port on a client and confirm it
passes:

```bash
sudo nc -p 443 <client-ip> 4444
```

**7.** Replace both rules with stateful inspection:

```bash
sudo iptables -F FORWARD
sudo iptables -A FORWARD -m conntrack --ctstate ESTABLISHED,RELATED -j ACCEPT
sudo iptables -A FORWARD -s 10.1.1.0/24 -d 10.3.3.10 -p tcp --dport 443 -j ACCEPT
```

**8.** Repeat step 6 and confirm it now fails. Then examine the state table:

```bash
sudo conntrack -L
```

---

### Part 2 — Shadowed rules and ordering

**9.** Build this rule set deliberately:

```bash
sudo iptables -F FORWARD
sudo iptables -A FORWARD -m conntrack --ctstate ESTABLISHED,RELATED -j ACCEPT
sudo iptables -A FORWARD -s 10.1.1.0/24 -j DROP
sudo iptables -A FORWARD -s 10.1.1.0/24 -d 10.3.3.10 -p tcp --dport 443 -j ACCEPT
```

**10.** Test from `10.1.1.0/24` to the server on 443. Record the result.

**11.** Now read the hit counters:

```bash
sudo iptables -L FORWARD -v -n --line-numbers
```

Record the packet count on the permit rule. Explain what it tells you.

**12.** Fix it by reordering, and confirm.

**13.** Write down the general diagnostic rule you just derived, in one sentence.

---

### Part 3 — Asymmetric routing breaks state

**14.** Build a second path between the client subnet and the server subnet that
bypasses the firewall in **one direction only** — a route on the server's router
sending return traffic via a different next hop.

**15.** Attempt a TCP connection through the firewall. Record what happens.

**16.** Capture on both paths simultaneously and confirm the SYN goes one way and
the SYN-ACK returns the other.

**17.** Examine the firewall's state table during the attempt. Record what it
contains and what it never sees.

**18.** Explain why everything is individually correctly configured and the
connection still fails.

---

### Part 4 — Segmentation and blast radius

**19.** Before segmenting, measure lateral reach. From one host, scan the whole
lab range — **your own lab range only**:

```bash
sudo nmap -sn 10.0.0.0/8 --exclude <anything not yours>
sudo nmap -p 22,80,139,443,445,3389 <your lab subnets>
```

Record: how many hosts are reachable, and how many open ports in total.

**20.** Now segment. Place hosts into zones — workstations, servers, cameras,
guest — using VLANs from week 5, and write a policy permitting only:

- workstations → servers on 443 and 445
- servers → servers on any
- cameras → the NVR host on its port only
- guest → the Internet only, nothing internal
- everything → DNS and DHCP

**21.** Repeat the scan from a workstation. Record the new figures.

**22.** Repeat from the **guest** zone. Record.

**23.** Compute the reduction in reachable services from each zone, and express it
as a percentage.

**24.** Now simulate a compromise: from the camera VLAN, attempt to reach the
server zone. Record. This is the lateral movement step that Chapter 57 §57.4
identifies as where a foothold becomes a catastrophe.

---

### Part 5 — TLS and certificates

**25.** Capture a TLS 1.3 handshake:

```bash
sudo tcpdump -i any -n -w /tmp/tls.pcap 'tcp port 443' &
curl -s https://example.com > /dev/null
```

**26.** In Wireshark, find the Client Hello. Record: the TLS version offered, the
cipher suites offered, and the **SNI** — note that the server name is visible in
plaintext.

**27.** Count the round trips from the first TCP SYN to the first byte of
application data.

**28.** Examine the certificate chain:

```bash
openssl s_client -connect example.com:443 -showcerts < /dev/null
```

Record: the subject, the issuer, the validity dates, and how many certificates are
in the chain up to the root.

**29.** Check the negotiated parameters:

```bash
openssl s_client -connect example.com:443 < /dev/null 2>&1 | grep -E 'Protocol|Cipher|Server public key'
```

**30.** Now produce each of the four common validation failures and record the
exact error message for each:

```bash
# (a) hostname mismatch
curl https://<ip-address-of-a-tls-server>/

# (b) expired certificate
curl https://expired.badssl.com/

# (c) self-signed / unknown issuer
curl https://self-signed.badssl.com/

# (d) wrong host entirely
curl https://wrong.host.badssl.com/
```

**31.** For each, record what `-k` (ignore verification) changes, and state what
security property you gave up by using it.

---

### Part 6 — Layer-two attacks (isolated network only)

With instructor supervision, on a segment connected to nothing else.

**32.** **MAC flooding.** From one host, generate frames with many fabricated
source addresses:

```bash
sudo macof -i <iface>          # from dsniff
```

Watch the switch's MAC table fill:

```
show mac address-table count
```

Then capture on a third host and record whether it begins seeing traffic it
should not.

**33.** Enable port security and repeat:

```
interface <port>
 switchport port-security
 switchport port-security maximum 2
 switchport port-security violation restrict
```

Record what happens to the attacking port.

**34.** **Rogue DHCP.** Run a second DHCP server (from Lab 08 Break-It D). Then
enable DHCP snooping and repeat:

```
ip dhcp snooping
ip dhcp snooping vlan 10
interface <uplink-to-real-server>
 ip dhcp snooping trust
```

**35.** **ARP spoofing with DAI.** Repeat Lab 04's spoof, then enable Dynamic ARP
Inspection and repeat:

```
ip arp inspection vlan 10
interface <trusted-uplink>
 ip arp inspection trust
```

**36.** **BPDU Guard.** Send a BPDU claiming a low bridge priority from a host
port, then enable BPDU Guard and repeat:

```
interface <access-port>
 spanning-tree portfast
 spanning-tree bpduguard enable
```

---

## Expected observations

- **Step 2 versus 3: `DROP` produces a timeout; `REJECT` produces an immediate
  refusal.** The difference is diagnostically important and is a deliberate policy
  choice — silence is stealthier, refusal is kinder to legitimate users.
- **Step 6: the stateless rule set is walked straight through** by traffic sourced
  from port 443. This is why stateful inspection exists.
- **Step 10: the connection fails despite a correct permit rule**, and step 11's
  hit counter reads **zero** on that rule — the definitive evidence that it is
  shadowed rather than wrong.
- **Step 15: the connection fails with everything correctly configured.** The
  firewall sees a SYN, never sees the SYN-ACK, and drops the client's subsequent
  ACK as out of state.
- **Steps 21–23: reachable services fall by a large factor** from the workstation
  zone and to nearly nothing from guest.
- **Step 26: SNI is in plaintext.** The server name is visible to anyone on the
  path, which is why Encrypted Client Hello exists and is contested.
- **Step 27: three or four round trips** from SYN to first byte, and TLS 1.3
  accounts for one of them where TLS 1.2 needed two.
- **Steps 32–36: every attack succeeds with default configuration and fails once
  the corresponding feature is enabled.** Each feature is one or two lines.

---

## Debrief

**1.** Explain the difference between `DROP` and `REJECT` in terms of what the
client observes, and state which you would choose for an Internet-facing rule and
which for an internal one, with reasons.

**2.** In step 6 you walked through a stateless rule set. Explain the mechanism,
then explain precisely what the stateful version checks that the stateless one
cannot. Then name the three costs statefulness introduces.

**3.** Your shadowed rule had zero hits. State the general diagnostic rule in one
sentence, and explain why a rule with zero hits after a week is *either*
unnecessary *or* shadowed — and how you would tell which.

**4.** In Part 3 everything was correctly configured and the connection failed.
Draw the packet flow, mark what the firewall saw, and explain the failure. Then
state two ways to fix it and which you would choose in a design.

**5.** Present your before-and-after reachability figures for each zone. Express
the reduction as a percentage. Then explain, in terms of a ransomware operator's
workflow, exactly which step of theirs you made expensive — and what you did
*not* prevent.

**6.** Tabulate your four TLS validation failures: the trigger, the exact error
message, and what a user typically does when they see it. Then explain what `-k`
gives up, and why the fact that users routinely click through these warnings is a
design problem rather than a user problem.

**7.** For each of the four layer-two attacks, state: the mechanism, the mitigation
you enabled, and the number of configuration lines it took. Then answer the
question the chapter poses — if these are free and effective, why are they absent
from so many production networks?

---

## Feeds the project

Deliverable 5 is due this week. Part 4's zone policy is directly reusable as the
basis for Meridian's segmentation, and Part 6's results give you the evidence for
the access-layer hardening checklist the deliverable requires — each item tied to
the attack you watched it prevent.
