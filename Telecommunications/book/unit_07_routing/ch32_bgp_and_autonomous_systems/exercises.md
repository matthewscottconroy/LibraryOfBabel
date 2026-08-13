# Chapter 32 — Exercises

## A. Recall

**A1.** Define an autonomous system in one sentence. What kind of boundary is it?

**A2.** Give the AS number sizes, the private ranges, and the transition mechanism that
made the 32-bit extension work.

**A3.** Give BGP's transport, port, administrative distances, keepalive and hold timers.

**A4.** State BGP's loop-detection rule.

**A5.** List the BGP path-selection steps in order, at least through step 8.

**A6.** State the valley-free rule as a three-row table.

**A7.** Distinguish a route leak from a route hijack.

**A8.** What does a ROA attest, and what does RPKI **not** protect against?

## B. Apply

**B1.** AS 100 originates `198.51.100.0/24`. Trace the AS_PATH as it passes through AS
200, AS 300 and AS 400. Then state what AS 200 does if it receives that route back from
AS 400, and why.

**B2.** For each pair of paths to the same prefix, state which BGP selects and at which
step:

(a) LOCAL_PREF 100, AS_PATH (200 300) versus LOCAL_PREF 150, AS_PATH (400 500 600 700)
(b) LOCAL_PREF 100, AS_PATH (200 300) versus LOCAL_PREF 100, AS_PATH (400)
(c) Both LOCAL_PREF 100, AS_PATH length 3, one eBGP and one iBGP
(d) Identical in every respect except IGP metric to next hop: 10 versus 30
(e) LOCAL_PREF 200 with MED 500 versus LOCAL_PREF 200 with MED 10, same neighbour AS

**B3.** Compute the iBGP full-mesh session count for 4, 8, 16 and 40 routers. Then give
the count with a single route reflector.

**B4.** An AS has these neighbours: Provider P, Peer Q, Customer C. For each route source,
state which neighbours it may be advertised to:

(a) a route learned from C  (b) a route learned from Q  (c) a route learned from P
(d) the AS's own prefixes

**B5.** Your AS holds `203.0.113.0/24`. An attacker announces `203.0.113.0/25` and
`203.0.113.128/25`. Explain what happens to your traffic and why, then give two emergency
responses.

**B6.** Write the BGP configuration for an AS multihoming to two providers, preferring
Provider A for outbound, with all mandatory filters present.

## C. Analyse

**C1.** Give four reasons an interior gateway protocol cannot be used between
organisations. Rank them by how fundamental each is.

**C2.** Explain how carrying the AS path solves the count-to-infinity problem of Chapter
31 §31.2 completely, where four heuristics could not.

**C3.** The BGP selection algorithm contains no bandwidth, latency or loss. Explain why,
and explain what this means for anyone expecting BGP to find a fast path.

**C4.** Explain hot-potato routing and prove that it produces asymmetric paths.

**C5.** Derive the valley-free rule from the money, treating each of the three cases
separately.

**C6.** "Almost every large BGP incident in history is a valley." Test this against the AS
7007, Telekom Malaysia, and 2019 Verizon incidents.

**C7.** Explain why outbound traffic engineering is easy and inbound is hard. Is this a
deficiency in the tools or something structural?

**C8.** Explain the 2008 Pakistan Telecom incident completely: the intent, each failure,
and the three separate defences that would each have prevented it.

**C9.** RPKI validates origin but not path. Construct an attack that RPKI does not stop,
and name the proposed mechanism that would.

**C10.** The 32-bit AS number transition succeeded in a few years; IPv6's has taken thirty.
Identify three differences that account for this.

## D. Design

**D1.** Design the BGP policy for an enterprise multihoming to two ISPs: which provider is
preferred for outbound, how inbound is influenced, what is filtered in both directions,
and what fails if each link dies.

**D2.** Design the iBGP topology for a network of 24 routers in four sites. Specify route
reflectors, redundancy, and what peers with what.

**D3.** Write the complete BGP security posture for a mid-sized ISP: filtering, RPKI,
monitoring, and the response procedure for a hijack of a customer's prefix.

**D4.** An organisation's two 10 Gb/s transit links carry 9 Gb/s and 1 Gb/s. Design a plan
to balance them, in order of what you would try, with the expected effect of each.

**D5.** For the semester project's network, determine whether BGP is warranted. Justify
either way against the criterion in §32.1.

## E. Troubleshoot

**E1.** Routes appear in `show ip bgp` and not in `show ip route`. Give the most likely
cause and the fix.

**E2.** An iBGP peer has routes that its iBGP peers do not. Explain and give two
solutions.

**E3.** You prepend your AS three times and traffic distribution does not change. Give two
reasons.

**E4.** A BGP session flaps every few minutes and each flap causes a large outage. Explain
why the outage is disproportionate, and give three causes of the flap.

**E5.** After a change, your provider's session drops with a `maximum-prefix` message.
What happened, and what do you check before raising the limit?

**E6.** Your prefixes are being announced by an AS you have never heard of. Give the
response, in order.

**E7.** Transit costs rose sharply with no increase in your own traffic. Give a
routing-related explanation.

**E8.** Some of your routes are being rejected as RPKI Invalid by several networks, and
you are the legitimate holder. Diagnose.

**E9.** Two offices in the same city communicate over a path that crosses an ocean.
Explain.

## F. Extend

**F1.** Look up your own network's AS and prefixes on `bgp.he.net` or RIPEstat. Identify
its upstreams, its peers, and whether its prefixes have ROAs.

**F2.** Use RIPE RIS or RouteViews to examine the AS paths to your prefixes from several
vantage points. Explain any differences.

**F3.** Build a three-AS BGP lab with FRRouting. Implement the valley-free rule with
communities and prove that a leak is prevented.

**F4.** Read Cloudflare's post-mortem of the June 2019 leak. Identify every party's
failure and what each should have done.

**F5.** Check the current RPKI coverage of the global routing table and its growth over
five years. Compare with IPv6 adoption and comment on the difference.

**F6.** Configure BGPalerter for a prefix you control, or for a public one, and document
what it reports over a week.
