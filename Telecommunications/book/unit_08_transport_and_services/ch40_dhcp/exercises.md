# Chapter 40 — Exercises

## A. Recall

**A1.** Name the four values a host needs before it can use a network, and the failure each
produces when wrong.

**A2.** What did RARP provide, and what were its two limitations?

**A3.** What did BOOTP add over RARP, and what did DHCP add over BOOTP?

**A4.** Give the four DORA messages and what each accomplishes.

**A5.** Why does the client use source `0.0.0.0` and destination `255.255.255.255`?

**A6.** Why is the REQUEST broadcast rather than unicast to the chosen server?

**A7.** At what fractions of the lease do T1 and T2 fire, and what does the client do at
each?

**A8.** What is `giaddr`, who sets it, and what does the server use it for?

**A9.** Give the option numbers for: subnet mask, gateway, DNS servers, lease time, message
type, server identifier, TFTP server.

## B. Apply

**B1.** Draw the complete DORA exchange for a client on VLAN 20 whose DHCP server is on a
different subnet, showing every source and destination address and the `giaddr` value.

**B2.** A client obtains a 24-hour lease at 08:00 Monday.

(a) When does it first attempt renewal, and to whom?
(b) When does it broadcast for any server?
(c) The DHCP server fails at 14:00 Monday and is repaired at 02:00 Tuesday. Does the client
notice? Justify.
(d) The server fails at 08:00 Monday instead. When does the client lose its address?

**B3.** Design the scope for `10.2.7.0/24`: 90 workstations, 6 printers needing stable
addresses, a gateway, two local servers, and room to grow. Give ranges, exclusions,
reservations and a justified lease time.

**B4.** A guest wireless network has 400 addresses and serves visitors who stay about
90 minutes. Compute the maximum sustainable arrival rate for lease times of 8 days,
1 day, and 2 hours. State which you would choose.

**B5.** For each symptom, state the most likely cause and the first command:

(a) One host has `169.254.x.x`; its neighbours are fine
(b) A whole floor has `169.254.x.x`
(c) Every host on every subnet has `169.254.x.x`
(d) A host fails DHCP at boot and succeeds on manual renew
(e) Hosts get addresses in `192.168.0.0/24` on a `10.0.0.0/8` network
(f) New clients fail while existing ones work

**B6.** Write the relay configuration for a router with SVIs on VLANs 10, 20 and 30, with
two DHCP servers at `10.1.1.53` and `10.1.1.54`.

**B7.** A network has 60 VLANs. Compare, in terms of servers, configurations and failure
points: (a) one DHCP server per VLAN, (b) one central server with relays.

## C. Analyse

**C1.** Explain why the *rate of change* rather than the host count is what makes manual
configuration untenable.

**C2.** Explain what BOOTP automated and what it did not, and why the lease is the
distinguishing idea.

**C3.** "The lease time is both a propagation window and a resilience window." Explain both
and why they pull in opposite directions.

**C4.** Explain why the server broadcasts the OFFER rather than unicasting to the address
it is offering.

**C5.** Explain precisely what would go wrong if the REQUEST were unicast.

**C6.** Explain why a DHCP outage is typically discovered hours later, and what this
implies about monitoring.

**C7.** Explain why `giaddr` is necessary, and what happens when a server has no scope
matching it.

**C8.** Explain the rogue DHCP server attack, why it is a man-in-the-middle, and why the
symptoms vary between hosts.

**C9.** Explain why Dynamic ARP Inspection requires DHCP snooping, and what happens if it
is enabled without it.

**C10.** A reservation and a static configuration produce the same address. Give four
reasons to prefer the reservation.

**C11.** Explain why option 121 requires the default route to be listed explicitly, and what
happens if it is not.

## D. Design

**D1.** For the semester project's network, produce the complete DHCP design: every scope,
its ranges, exclusions, options, reservations and lease time, plus the relay configuration
and the redundancy approach.

**D2.** Design DHCP redundancy for a 5,000-client network. Compare split scopes with
failover, choose one, and justify.

**D3.** Write the DHCP security configuration for an access switch: snooping, trusted
ports, rate limits, and their interaction with DAI and port security. Justify each line
against a specific attack.

**D4.** An organisation is deploying 400 IP phones and 200 access points. Design the DHCP
configuration that lets both self-provision, including the options each needs.

**D5.** Design the monitoring and alerting for DHCP: what is measured, at what thresholds,
and what each alert would mean.

## E. Troubleshoot

**E1.** A user reports no network. `ipconfig` shows `169.254.88.4`. Give your first three
checks in order and what each would rule out.

**E2.** After a switch replacement, one VLAN's clients cannot obtain addresses. Others are
fine. Diagnose.

**E3.** Clients on a new subnet receive no addresses. The relay is configured and the
server is reachable from the router. The server's log shows the requests arriving.
Diagnose.

**E4.** A conference network exhausts its pool every afternoon and recovers overnight.
Explain and give the immediate fix.

**E5.** Some laptops receive `192.168.1.x` addresses on a corporate `10.20.0.0/16`
network, apparently at random. Diagnose, and give the switch configuration that prevents
it.

**E6.** A printer with a DHCP reservation started getting a different address after a
firmware update. Explain.

**E7.** After enabling Dynamic ARP Inspection, all client traffic stopped. Explain.

**E8.** A new access point will not join the wireless controller. It has an IP address.
Give the two most likely causes.

**E9.** Duplicate address warnings appear on a subnet six months after it was built.
Explain why now.

**E10.** After adding a static route via DHCP, clients lost internet access entirely.
Explain.

## F. Extend

**F1.** Capture a complete DORA exchange. Identify option 53 in each message, the
transaction ID, the parameter request list, and every option the server returned.

**F2.** Compare the options a Windows client, a Linux client and a phone request (option
55). Explain the differences.

**F3.** Set up a DHCP server (ISC DHCP, Kea, or dnsmasq) with two scopes and a relay
between them. Verify `giaddr` in a capture and confirm the correct scope is used.

**F4.** Run a rogue DHCP server on an isolated lab segment and observe which clients it
captures and why. Then enable DHCP snooping and repeat.

**F5.** Measure lease behaviour: set a 4-minute lease, capture for 10 minutes, and confirm
T1 and T2 fire where predicted.

**F6.** Configure option 43 for a device type you have available, and verify with a capture
that the client requested it and the server returned it.
