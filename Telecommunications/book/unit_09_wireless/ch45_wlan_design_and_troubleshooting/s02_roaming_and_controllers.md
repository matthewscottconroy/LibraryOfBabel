# 45.2 Roaming and Controllers

**The client decides when to roam, and the network cannot make it.** That single fact
explains most roaming complaints, and it is where this section starts.

## The client decides

**There is no mechanism in 802.11 by which an access point can move a client.**

A client monitors its current access point's signal, **decides by its own algorithm** that it
should look elsewhere, scans, chooses a new BSSID, and reassociates. **The network is
informed after the fact.**

**And every vendor's algorithm differs**, tuned for battery life as much as performance:

| Client | Typical behaviour |
|---|---|
| **Apple iOS** | roams around −70 to −75 dBm; **relatively eager** |
| Android | varies enormously by manufacturer |
| **Windows laptops** | often **sticky** — hold on to −80 dBm or worse |
| **Handheld scanners** | usually configurable, and often badly configured |
| IoT devices | frequently do not roam at all |

> **"Sticky client" is the commonest roaming complaint**: a laptop walks past three access
> points holding onto the first at −85 dBm, running at a low MCS, **consuming airtime**
> (Chapter 44 §44.2) and performing badly — **while three better options are audible.**

**What the network can do about it is limited and indirect:**

**Make the current access point less attractive** — reduce power, or raise the minimum data
rate so the client falls below the threshold and is disassociated.

**802.11v BSS Transition Management** — *suggest* to the client that it move. **The client
may decline.**

**Deauthenticate it** — blunt, disruptive, and some controllers do it as a last resort.

**And the real fix is design:** correct cell sizes and overlap (§45.1), so that the signal
falls away sharply enough that the client's own algorithm acts.

## The roaming sequence, and where the time goes

**A full reassociation:**

```
   1. Scan for candidates            10–500 ms   ← the largest and most variable
   2. 802.11 authentication            ~2 ms
   3. Reassociation                    ~2 ms
   4. 802.1X / EAP  (enterprise)    100–1000 ms  ← the expensive one
   5. Four-way handshake              10–50 ms
   6. DHCP (if the VLAN changed)     50–500 ms   ← should not happen
   ────────────────────────────────────────────
   Total, unoptimised:               200–2000 ms
```

**And the tolerance:**

| Application | Tolerates |
|---|---|
| **Voice** | **< 150 ms**, ideally < 50 |
| Video conferencing | < 200 ms |
| Interactive data | < 500 ms |
| Bulk transfer | seconds |

> **An unoptimised enterprise roam takes long enough to drop a call.** Steps 1 and 4 are
> where the time is, and the amendments below address exactly those.

## The three amendments

**They are frequently confused, and each does something distinct.**

### 802.11k — neighbour reports

**"Here are the access points near you, and their channels."**

**Without it, a client scans all channels** — **up to 25 in 5 GHz** — which takes 100–500 ms
**and interrupts traffic**, because a client scanning another channel is not on its own.

**With it, the access point provides a neighbour list**, so the client scans **only the
channels its neighbours are on** — typically three or four.

**Effect: scanning drops from hundreds of milliseconds to tens.**

### 802.11v — BSS transition management

**"You would be better off on that access point."**

**A suggestion, not an instruction.** The access point can recommend a target, and a
cooperative client will act on it.

**Uses:** moving sticky clients, load balancing, band steering (Chapter 43 §43.3), and
gracefully emptying an access point before maintenance.

**Its limitation is client compliance** — a client may ignore it entirely, and many do.

### 802.11r — fast BSS transition

**The one that matters for voice**, and it addresses step 4.

**Without it, an enterprise roam repeats the full 802.1X/EAP exchange** with the RADIUS
server — several round trips, **100 ms to over a second.**

**With it, the key hierarchy is established once** and distributed among the access points,
so a roam needs **only the four-way handshake** using pre-established keys.

**Effect: step 4 disappears. Roams complete in under 50 ms.**

**Two mechanisms:**

| | How the client gets the new key |
|---|---|
| **Over-the-air** | directly with the target access point |
| **Over-the-DS** | via the current access point, through the distribution system |

**Over-the-DS is usually preferred** — the client does not need to leave its current channel
to set up the new association.

**And 802.11r has a compatibility history worth knowing:** **some older clients fail to
associate at all to an SSID advertising it.** The standard workaround was a separate SSID for
voice devices; **modern clients are largely fine**, and it is worth testing rather than
assuming.

### Together

| Amendment | Addresses | Saving |
|---|---|---|
| **802.11k** | **scanning** (step 1) | 100–500 ms → tens |
| **802.11v** | **when to roam** | avoids sticky clients |
| **802.11r** | **authentication** (step 4) | 100–1000 ms → ~0 |

**Enable all three**, and test with your actual voice clients.

## Layer 3 roaming

**Step 6 above should not happen, and when it does the roam is a reconnection.**

**If a client moves between access points in different subnets, its IP address is no longer
valid** — so it must obtain a new one, and **every existing connection breaks.**

**The remedies, in order of preference:**

**Keep the wireless VLAN consistent across the roaming domain.** **The simplest and best
answer** — one VLAN, one subnet, everywhere a client might roam. Chapter 20's segmentation
still applies; it is the *wireless* VLAN that must be continuous.

**Tunnel to the controller.** Traffic is encapsulated from the access point to a central
controller, which places it on the same VLAN regardless of where the client is. **This is what
centralised controllers do**, and it works at the cost of a traffic detour.

**Mobility groups** — controllers coordinate so a client roaming between them keeps its
anchor.

> **A wireless design where a client changes subnet while walking is a design error**, and
> the symptom — calls dropping and sessions resetting at a particular point in a building —
> is distinctive.

## Controller architectures

**Four models, and the industry has moved through them:**

| Architecture | Data path | Character |
|---|---|---|
| **Autonomous** | direct | each AP configured individually; **no roaming coordination** |
| **Centralised controller** | **tunnelled to controller** | full coordination; **controller is a bottleneck and a single point of failure** |
| **Local switching** | **direct from AP** | controller manages, data stays local |
| **Cloud-managed** | **direct** | management in the cloud, **control plane survives WAN loss** |

**The trajectory is clear:** **from configuring each device, to centralising everything, to
centralising only the control plane.**

**Chapter 68's SDN argument applies exactly** — separating the control plane from the data
plane, and putting the control plane where it can see everything. **Wireless got there before
wired did**, because the coordination problem was more acute: RF is shared, so channel and
power decisions must be made globally.

**What a controller does that autonomous access points cannot:**

- **RRM — radio resource management**: automatic channel and power assignment, responding to
  interference and to access-point failure
- **Roaming coordination**: key distribution for 802.11r, neighbour lists for 802.11k
- **Central policy**: one configuration, applied everywhere
- **Client load balancing** across access points
- **RF neighbour discovery**: which access points can hear which

**And the important design question is the data path.** **Tunnelling everything to a
controller** means a client's traffic to a server in the same building may cross the network
twice — **and it means the controller's capacity is the network's capacity.**

**Local switching** avoids both, at the cost of needing the VLAN present at every access point
(above).

**Cloud management's advantage is the failure mode:** the access points continue serving
clients when the WAN link fails, because **only management is in the cloud.** A centralised
controller reachable only over a WAN link is a much worse dependency.

## RRM and its surprises

**Automatic channel and power assignment**, and it does something useful and occasionally
something disruptive.

**What it does well:** responds to a failed access point by raising neighbours' power to fill
the hole; finds interference and moves off it; assigns channels without a human channel plan.

**Where it surprises people:**

**It changes things.** A channel change disconnects clients briefly. **RRM running frequently
in a high-density area is itself disruptive**, which is why static assignment is sometimes
right for lecture theatres (Chapter 43 §43.2).

**It optimises what it can measure.** It sees other access points and the noise floor; it
does not see **your users' experience**, and it will happily produce a plan that is optimal by
its metric and poor in practice.

**It can oscillate.** Two controllers, or a controller and a neighbour's, can chase each
other's changes.

> **Verify what RRM chose.** It is usually adequate and it is not authoritative, and the
> commonest finding is power set higher than a human would choose — because more coverage
> looks better to an algorithm that does not measure contention.

## What breaks here

**Calls dropping when walking between areas.** Roaming too slow. Check 802.11r, and check
whether the VLAN changes.

**A laptop holding a distant access point at −85 dBm.** Sticky client. Reduce cell size,
raise the minimum rate, enable 802.11v.

**Roaming works for laptops and not for handheld scanners.** Client roaming thresholds. Many
scanners are configurable and shipped badly configured.

**Sessions resetting at a particular doorway.** A subnet boundary. Fix the VLAN design.

**Some clients cannot join after enabling 802.11r.** Older client incompatibility. Test, and
consider a separate SSID if genuinely needed.

**Everything slower after RRM raised the power.** Larger cells, more overlap, more contention.

**The whole site down when the WAN failed.** A centralised controller across a WAN. Local
switching or cloud management.

> **Network+ note.** Objective 2.4 expects roaming and controller architectures. Over-learn:
> **the client decides when to roam**; **802.11k gives neighbour lists, 802.11v suggests
> transitions, 802.11r speeds up authentication**; **a roam that changes subnet breaks
> connections**; and **controller-based versus cloud-managed versus autonomous** as
> architectures. The k/v/r distinction is examined and commonly confused.
