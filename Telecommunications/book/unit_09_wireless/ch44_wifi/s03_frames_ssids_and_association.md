# 44.3 Frames, SSIDs and Association

Getting onto a wireless network is a multi-stage process, and **each stage fails
differently** — which makes knowing the stages the fastest route to a diagnosis.

## The three frame types

**802.11 has more frame types than Ethernet**, because it must manage the association that
Ethernet gets from a cable.

| Type | Purpose | Examples |
|---|---|---|
| **Management** | **join, leave, discover** | Beacon, Probe, Authentication, Association, Deauthentication |
| **Control** | **medium access** | RTS, CTS, ACK, Block ACK |
| **Data** | **the payload** | your actual traffic |

**Management frames are what a wireless capture is mostly made of**, and they are the ones
that carry the diagnosis.

**And historically they were unprotected** — sent in the clear, unauthenticated, by anyone —
which is the basis of the deauthentication attack below and why **802.11w Protected
Management Frames** exists.

## Beacons

**The access point announces itself, typically ten times a second.**

**Every beacon carries:**

| Field | Contents |
|---|---|
| **SSID** | the network name |
| **BSSID** | **the radio's MAC address** |
| Supported rates | which MCS values are available |
| **Channel** | |
| Capability information | security, QoS, standards supported |
| **TIM** | **which sleeping clients have buffered traffic** |
| Country, power constraints | regulatory information |
| **RSN** | the security configuration (WPA2/WPA3) |

**The beacon interval is normally 102.4 ms** — so about **ten beacons per second per SSID per
radio.**

> **Which is the argument against many SSIDs.** Four SSIDs on an access point with two radios
> means **eighty beacons per second**, sent at the lowest basic rate so every client can hear
> them — **and each one occupies the medium.**

**A network with six SSIDs can spend a substantial fraction of its airtime announcing
itself.** Chapter 45 §45.3 covers the design consequence; the rule is **three SSIDs maximum,
and two is better.**

**The TIM** is how power saving works: a sleeping client wakes periodically, **reads the TIM
to learn whether anything is waiting for it**, and sleeps again if not.

## SSID, BSSID and ESSID

**Three terms, frequently confused, and the distinction matters for roaming.**

| Term | What it is |
|---|---|
| **SSID** | **the network name** — a human-readable string, up to 32 bytes |
| **BSSID** | **one radio's MAC address** — a specific access point's specific band |
| **BSS** | one access point's cell |
| **ESS** | several access points sharing an SSID |

```
   ESS "CorpWiFi"
     ├── AP1, 2.4 GHz  BSSID aa:bb:cc:00:00:01
     ├── AP1, 5 GHz    BSSID aa:bb:cc:00:00:02   ← same AP, different BSSID
     ├── AP2, 2.4 GHz  BSSID aa:bb:cc:00:01:01
     └── AP2, 5 GHz    BSSID aa:bb:cc:00:01:02
```

> **A client associates with a BSSID, not an SSID.** Roaming (Chapter 45 §45.2) is moving
> from one BSSID to another within the same ESS — **and the client decides when, not the
> network.**

**And one access point has several BSSIDs** — one per radio, and one per SSID per radio. **An
access point with three SSIDs on two bands presents six BSSIDs**, which is why a scan shows
many more entries than there are physical devices.

## Hidden SSIDs

**A beacon with the SSID field blank.** Frequently recommended as a security measure, and
**it is not one.**

**Why it does not work:**

**The SSID still appears in every association**, in probe requests and responses, and in
reassociation frames. **Any passive capture during a single client joining reveals it**, and
tools do this automatically.

**And it makes things worse in three ways:**

**Clients must probe actively** for the hidden network, **broadcasting the name they are
looking for** — so a laptop configured for a hidden corporate network **announces that name
everywhere it goes**, including in cafés and airports. **The network is now discoverable by
following the client rather than the access point.**

**It breaks some clients**, particularly older ones and some IoT devices.

**And it makes roaming slower**, because the client cannot discover the network passively.

> **Hiding an SSID provides no security, leaks the name from every client, and degrades
> behaviour.** Chapter 59 makes the general argument about security through obscurity; this
> is the clearest wireless instance.

## The association sequence

**Four stages, and they must complete in order.**

```
   1. DISCOVERY
      Passive:  client listens for beacons
      Active:   client sends Probe Request → AP sends Probe Response

   2. AUTHENTICATION  (802.11-level — largely vestigial)
      Client → AP:  Authentication Request
      AP → Client:  Authentication Response

   3. ASSOCIATION
      Client → AP:  Association Request  (capabilities, rates, security)
      AP → Client:  Association Response (accepted, + Association ID)

   4. SECURITY  (WPA2/WPA3 — the real authentication)
      802.1X / EAP if enterprise
      then the 4-way handshake

   → now data may flow
```

**Stage 2 is a historical artefact.** In **Open System** authentication — which is what
everything uses — **the access point accepts unconditionally.** The original alternative,
Shared Key authentication with WEP, was **worse than useless** — it leaked keystream and made
the WEP key easier to recover — and is long deprecated.

**The real authentication is stage 4**, and Chapter 59 covers it.

## The four-way handshake

**How WPA2 and WPA3 establish keys**, and it is worth knowing because its failure is a common
and specific diagnosis.

**The premise:** both sides already share the **PMK** — the Pairwise Master Key — derived from
the passphrase (WPA2-Personal) or from 802.1X (WPA2-Enterprise). **The handshake never
transmits it.**

```
   AP → Client:  ANonce                         (a random number)
                 ↓
                 Client now has PMK, ANonce, SNonce, both MACs
                 → derives the PTK (Pairwise Transient Key)
                 ↓
   Client → AP:  SNonce + MIC                   (proves it has the PMK)
                 ↓
                 AP derives the same PTK, verifies the MIC
                 ↓
   AP → Client:  GTK + MIC                      (the group key, encrypted)
   Client → AP:  ACK
                 ↓
                 Both have keys; data may flow
```

**Two properties worth noting:**

**The passphrase is never sent.** Both sides prove they know it by producing a MIC over data
derived from it — **and a wrong passphrase fails at message 2**, because the MIC does not
verify.

**The nonces make each session's keys unique**, so capturing one session does not help with
another.

**And the vulnerability:** **capturing the four-way handshake allows an offline dictionary
attack** against the passphrase in WPA2-Personal. **The attacker does not need to interact
further** — they can test billions of candidate passphrases against the captured handshake.

**Which is why WPA3's SAE (Simultaneous Authentication of Equals) matters**: it is resistant
to offline dictionary attack by construction, and Chapter 59 §59.3 covers it.

## Deauthentication — the attack that persists

**Management frames were unauthenticated**, so **anyone can forge a deauthentication frame**
claiming to be from the access point:

```
   Attacker → Client:  Deauthentication ("you are disconnected"), spoofing the AP's BSSID
   → the client disconnects and must reassociate
```

**Uses:**

**Denial of service** — repeat continuously, and the client can never stay connected.

**Forcing a handshake capture** — deauthenticate a client, watch it reconnect, **capture the
four-way handshake** for the offline attack above. **This is the standard first step in
attacking a WPA2-Personal network.**

**Evil twin** — deauthenticate clients from the real access point and offer a
identically-named one.

**The defence is 802.11w — Protected Management Frames**, which authenticates management
frames so forgeries are rejected.

> **PMF is mandatory in WPA3 and optional in WPA2.** **Enable it.** It closes a
> twenty-year-old attack that requires no privileged access and freely available tools.

**The caveat:** some older clients do not support it, and **requiring** PMF excludes them.
The usual deployment is **PMF optional on WPA2 networks and required on WPA3.**

## Power saving

**How a battery-powered client stays connected without staying awake.**

**Legacy power save:**

1. The client tells the access point it is entering power save
2. **The access point buffers frames for it**
3. The client sleeps, **waking to read the TIM in beacons**
4. If the TIM shows buffered traffic, the client sends a **PS-Poll** and receives it

**802.11e's U-APSD** improved it for voice; **802.11ax's TWT** (§44.1) improved it far more:
**the client negotiates specific wake times** and can sleep for seconds or minutes between
them, rather than waking every beacon interval.

**The diagnostic consequence:** **a sleeping client appears unresponsive.** A ping to an
idle phone may take hundreds of milliseconds for the first packet **and then be fast** — the
first packet woke it. **This is normal and is frequently mistaken for a network problem.**

## Reading a wireless capture

**Wireless capture requires monitor mode** — an adapter that reports all frames rather than
only those addressed to it, **and not all adapters or drivers support it.** macOS supports it
natively; Linux does with the right chipset; Windows generally requires special hardware.

```bash
# Linux
sudo airmon-ng start wlan0
sudo tcpdump -i wlan0mon -nn

# macOS — built in
sudo /System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport \
     en0 sniff 36
```

**Wireshark filters worth knowing:**

```
wlan.fc.type == 0                    # management
wlan.fc.type == 1                    # control
wlan.fc.type == 2                    # data
wlan.fc.type_subtype == 8            # beacons
wlan.fc.type_subtype == 4            # probe requests
wlan.fc.type_subtype == 12           # DEAUTHENTICATION  ← attack, or a fault
wlan.fc.retry == 1                   # retransmissions
eapol                                # the four-way handshake
```

**`wlan.fc.type_subtype == 12` is the one to remember.** A flood of deauthentication frames
is either an attack or a badly-behaved controller, **and either way it explains "clients keep
dropping".**

**And `wlan.fc.retry == 1`** gives the retry rate of Chapter 43 §43.4 directly.

## Where association fails

**The stages, and what each failure means:**

| Fails at | Symptom | Cause |
|---|---|---|
| **Discovery** | network not visible | out of range; wrong band; hidden SSID; radio disabled |
| **Authentication** | rarely fails | MAC filtering; AP at client limit |
| **Association** | "cannot connect" | incompatible rates or capabilities; AP full |
| **4-way handshake** | **"incorrect password"** | **wrong passphrase**; RADIUS failure; certificate problem |
| **After association** | connects then nothing | **DHCP** (Chapter 40 §40.4); VLAN; captive portal |

> **The last row is the most common and the most misattributed.** A client that associates
> successfully and gets no address has a **DHCP** problem, not a wireless one — and
> `169.254.x.x` says so.

**And the fourth row's distinction matters:** WPA2-Personal failing means the passphrase;
**WPA2-Enterprise failing means RADIUS, or the certificate, or the user's credentials** — and
the client's error message says "incorrect password" for all of them.

## What breaks here

**Clients disconnecting repeatedly.** Look for deauthentication frames. An attack, a
controller misbehaving, or aggressive roaming settings.

**A network with six SSIDs performing badly.** Beacon overhead. Reduce to two or three.

**A hidden SSID that everyone knows.** It was never hidden. Remove the setting.

**Clients associating and getting no address.** DHCP, not wireless.

**"Incorrect password" on an enterprise network.** RADIUS, certificate, or credentials — not
the passphrase.

**A phone that seems slow to respond and then is fast.** Power saving. Normal.

**An IoT device that cannot join a network everything else can.** Often PMF required, or a
rate the device cannot use, or 5 GHz-only.

> **Network+ note.** Objective 2.4 expects SSIDs and association; objective 4.2 expects
> deauthentication and evil twin attacks. Over-learn: **SSID is the network name and BSSID is
> the access point radio's MAC address**; **the association sequence is discovery,
> authentication, association, then security**; **hiding an SSID is not security**; and
> **deauthentication attacks exploit unprotected management frames, mitigated by 802.11w
> PMF.**
