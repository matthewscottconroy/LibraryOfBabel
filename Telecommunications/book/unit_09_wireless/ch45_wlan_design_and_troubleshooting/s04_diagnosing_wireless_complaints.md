# 45.4 Diagnosing Wireless Complaints

Users make four complaints about wireless, and each maps to a small set of causes that
this unit has established. This section is the procedure.

## The four complaints

| Complaint | Usually means |
|---|---|
| **"I can't connect"** | association, authentication, or **DHCP** |
| **"It's slow"** | **coverage, capacity, interference, or not wireless at all** |
| **"It keeps dropping"** | roaming, power saving, or deauthentication |
| **"It works here and not there"** | **coverage** — the most tractable |

## The first question

**Before any measurement:**

> **How many people, and where?**

| Scope | Points at |
|---|---|
| **One user, one device** | **the client** — driver, configuration, hardware |
| One user, all their devices | their **location** |
| **Everyone in one area** | **coverage or interference there** |
| **Everyone, everywhere** | **infrastructure** — controller, RADIUS, DHCP, uplink |
| Everyone, intermittently | interference, or a scheduled event |

This is Chapter 40 §40.4's question applied to wireless, and it eliminates most of the
search space before you have touched a tool.

## "I can't connect"

Work the association stages (Chapter 44 §44.3), because each fails distinctly.

```
   1. Is the SSID visible?
        No  → range, band, radio disabled, hidden SSID, wrong regulatory domain
        Yes ↓
   2. Does it associate?
        No  → capabilities mismatch, MAC filter, AP at client limit
        Yes ↓
   3. Does authentication succeed?
        No  → passphrase (personal); RADIUS / certificate / credentials (enterprise)
        Yes ↓
   4. Does it get an IP address?
        No  → DHCP  ← MOST COMMON, and not a wireless problem
        Yes ↓
   5. Does traffic pass?
        No  → VLAN, firewall, captive portal
```

Step 4 is the one to check early. `169.254.x.x` (Chapter 27 §27.2) says the client
associated fine and DHCP did not answer — and the causes are Chapter 40 §40.4's, not
wireless ones.

And step 3's enterprise case is worth separating: the client says "incorrect password" for
a wrong passphrase, a RADIUS timeout, an expired certificate, a wrong username, and a
certificate the client does not trust. The client cannot distinguish them and the RADIUS
log can, which is where to look.

**The specific cases worth knowing:**

An IoT device that cannot join a network laptops use. Usually: it is 2.4 GHz only and
the SSID is 5 GHz; or **PMF is required** and it does not support 802.11w; or the minimum
data rate excludes it (Chapter 44 §44.2); or it cannot do WPA3.

A device that connects at home and not at work. Enterprise authentication, or a captive
portal it cannot present.

**A client that connects and immediately disconnects.** Often authentication succeeding and
authorisation failing — check the RADIUS accounting records, which will show the
disconnect reason.

## "It's slow"

The complaint with the most possible causes, and the one where measurement matters most.

**Measure four things, in this order:**

```
   1. RSSI       — is the signal adequate?         (target −67 dBm)
   2. SNR        — is it usable?                   (target > 25 dB)
   3. Retry rate — is it being corrupted?          (< 10%)
   4. Channel utilisation — is it busy?            (< 70%)
```

**And the combination diagnoses it:**

| RSSI | SNR | Retries | Utilisation | Diagnosis |
|---|---|---|---|---|
| **poor** | poor | high | low | **coverage** — add APs, or the client is at the edge |
| **good** | **poor** | high | low | **interference** — the noise floor is raised (Ch 43 §43.4) |
| good | good | **high** | low | **hidden nodes** (Ch 44 §44.2), or a marginal client |
| good | good | low | **high** | **capacity** — too many clients (§45.3) |
| **good** | **good** | **low** | **low** | **not the wireless** ← |

> **The last row is the important one.** Signal good, noise low, no retries, medium quiet —
> **and the user says it is slow.** **The problem is upstream**: the WAN link, DNS
> (Chapter 39), the server, or the application. Wireless is where complaints arrive, not
> where they originate.

And this is worth saying to users and to management, because wireless is blamed for a
great deal it did not cause.

Two further checks when the four numbers are ambiguous:

**What rate is the client actually using?** (Chapter 44 §44.1's MCS.) A client at MCS 2 with
good RSSI has an SNR problem the RSSI is hiding.

**Is one client consuming the airtime?** (Chapter 44 §44.2.) `iw dev wlan0 station dump` or
the controller's per-client rate view. One device at 6 Mb/s explains a whole cell.

## "It keeps dropping"

**Four causes, and they are distinguishable.**

**Roaming** (§45.2) — the drops correlate with movement. Check whether the client is sticky
or whether roams are slow, and whether the VLAN changes at that point.

**Deauthentication frames** (Chapter 44 §44.3) — capture and filter
`wlan.fc.type_subtype == 12`. An attack, a misbehaving controller, or aggressive client
load-balancing.

**Power saving** (Chapter 44 §44.3) — the device appears to drop and is asleep. Slow first
packet then fast is normal.

**DFS radar events** (Chapter 43 §43.1) — everyone on one channel drops simultaneously,
and the access point logs a channel change. Distinctive, and frequently mystifying until you
know about it.

| Symptom | Cause |
|---|---|
| Drops while walking | roaming |
| **Everyone on one AP at once** | **DFS**, or an AP reboot |
| One client, repeatedly, stationary | deauthentication, or a driver |
| Appears dropped, works when used | power saving |
| Drops at a specific doorway | subnet change (§45.2) |

## "It works here and not there"

The most tractable complaint, because it is a coverage question and coverage is
measurable.

Walk it with a survey tool and find the boundary. Then:

| Finding | Remedy |
|---|---|
| RSSI falls below −70 dBm | **an access point is needed**, or the cell is too small |
| RSSI fine, SNR poor | **interference** at that location |
| RSSI fine at 2.4 and poor at 5 | expected (Ch 43 §43.3); design for 5 |
| A dead spot under an AP | **the antenna null** (Ch 42 §42.2) |
| Poor in a specific room only | construction — check the walls, and check for foil insulation or low-E glass |

And the specific case of a room that is worse than its neighbours is nearly always
**construction**: a plant room with metal walls, a lift lobby, a room with foil-backed
insulation, or a modern glazed meeting room (Chapter 42 §42.1).

## The tools

```bash
# Client side
iw dev wlan0 link                 # rate, MCS, signal
iw dev wlan0 station dump         # on an AP: per-client rates, retries, inactivity
iw dev wlan0 survey dump          # channel utilisation and noise

# macOS
sudo wdutil info
# Option-click the Wi-Fi menu for live rate and RSSI

# Windows
netsh wlan show interfaces
netsh wlan show wlanreport        # a very good HTML report of recent connections
```

`netsh wlan show wlanreport` is underused — it produces a graphical timeline of every
connection, disconnection and its reason over the last three days, which answers "it keeps
dropping" without any capture.

And on the infrastructure side, the controller holds what you need: per-client RSSI and
SNR history, per-AP client counts and utilisation, retry rates, roaming events, and RRM
decisions. Look there before capturing.

## The complaint that is not wireless

Worth a section of its own, because it is common.

When the four measurements are all good and users still complain:

| Check | For |
|---|---|
| **DNS** | Chapter 39 §39.4 — the commonest cause of "the network is slow" |
| **The WAN link** | utilisation, and whether it is saturated |
| **The application** | is it slow on wired too? **The definitive test.** |
| **The client** | CPU, disk, other software |
| **A captive portal** | expired session |

> The single most useful diagnostic is: does it happen on the wired network too? If yes,
> wireless is exonerated in one test, and the search moves to where the problem actually is.

## What breaks here

Assuming a slow complaint is a coverage problem. Measure before adding access points.

Adding access points for a capacity problem without reducing power. §45.3.

**Chasing a wireless fault that is DNS.** Test on wired.

**Treating `169.254.x.x` as a wireless problem.** It is DHCP.

**Blaming the network for a client's driver.** One user, one device, everywhere they go.

Not asking "how many people, and where" first. It eliminates most causes for free.

> **Network+ note.** Objective 5.4 is wireless troubleshooting and **this section is directly
> examined.** Over-learn: the four measurements — RSSI, SNR, retries, utilisation — and
> what each combination indicates; **good signal with poor performance means interference**;
> a client that associates and gets no address has a DHCP problem; and DFS events
> disconnect everyone on a channel at once.
