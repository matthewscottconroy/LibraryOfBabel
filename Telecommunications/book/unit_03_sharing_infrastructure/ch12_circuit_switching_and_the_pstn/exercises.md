# Chapter 12 — Exercises

## A. Recall

**A1.** State the three phases of a circuit-switched call and what is guaranteed
between the second and third.

**A2.** Why did Almon Strowger build an automatic exchange?

**A3.** Derive the DS0's 64 kb/s from the sampling and quantisation decisions.

**A4.** What is in-band signalling, and what security property does it lack?

**A5.** Define the erlang. If 240 calls per hour average 150 seconds each, how many
erlangs is that?

## B. Apply

**B1.** Compute the offered traffic in erlangs for: (a) 90 calls/hour at 240 s;
(b) 1,200 calls/hour at 90 s; (c) 40 calls/hour at 30 minutes.

**B2.** Using the Erlang B recurrence in §12.4, compute the blocking probability for
12 erlangs offered to 15, 18 and 21 circuits. Show the iteration for one case.

**B3.** A site offers 22 erlangs in the busy hour and requires 1% blocking. Find
the number of circuits needed by iterating the recurrence. Then compute the
utilisation achieved.

**B4.** Two branch offices each offer 8 erlangs and are currently served by
separate trunk groups at 1% blocking. Compute the circuits required for each, the
total, and the number required if the groups were combined. State the saving and
explain it.

**B5.** A T1 carries 24 channels. What is the maximum offered traffic it can serve
at 1% blocking? At 5%? Comment on the difference.

**B6.** Verify the T1 rate arithmetic, then compute what fraction of the 1.544 Mb/s
is framing overhead. Repeat for the E1.

**B7.** A data circuit is delivered over a robbed-bit T1. Compute its usable rate
per channel and explain the mechanism. Then explain why the same circuit carries
voice with no perceptible degradation.

**B8.** A SIP trunk must carry the traffic of 12.8's site using G.711. Compute:
the concurrent call paths required; the bandwidth per call including Ethernet, IP,
UDP and RTP headers; and the total bandwidth. Compare with the naive figure of
64 kb/s per call and state the error.

## C. Analyse

**C1.** Derive the trunking efficiency effect: show why a group carrying 100
erlangs achieves higher utilisation at the same grade of service than ten groups
carrying 10 erlangs each. Your answer should reference the relationship between the
mean and the standard deviation of the number of simultaneous calls.

**C2.** Explain the 2,600 Hz vulnerability precisely: what the tone meant, what
each exchange believed after it was sent, and why the caller retained a connection
they were not billed for. Then state the general principle it illustrates and give
two examples of the same class of vulnerability from outside telephony.

**C3.** SS7 was designed for a few dozen mutually trusting carriers. Explain how
that assumption was invalidated, describe two concrete attacks it enables, and
explain why the mitigations are only partially effective. Then identify three other
protocols in this book that were designed under the same assumption.

**C4.** Compare the failure modes of a circuit-switched and a packet-switched
network under overload. Your answer must address: what each does with the excess
demand; who experiences degradation; whether the system recovers when load falls;
and which behaviour is preferable for voice, for file transfer, and for a payment
authorisation.

**C5.** SS7 moved signalling onto a packet network in 1975. Argue that the
telephone network was therefore "packet-switched" two decades before the Internet's
data plane was, and then argue against that characterisation. What distinction
determines which claim is fairer?

## D. Design

**D1.** A company is replacing its legacy PBX with a hosted VoIP service. You have:

- 340 staff, of whom 210 are telephone users.
- Busy-hour measurements from the existing PBX: 1,850 external calls, mean duration
  195 seconds. Internal calls are not carried on the trunk.
- The company requires no worse than 0.5% blocking on external calls.
- The site has a 200 Mb/s Internet circuit, currently 40% utilised at peak.
- The finance director has asked why the SIP trunk cannot simply be sized at "one
  path per user, they are not all on the phone at once".

Determine the number of SIP paths required, showing the Erlang calculation.
Determine the bandwidth those paths consume with G.711 and with G.729, including
all headers. State whether the existing circuit is adequate and what headroom
remains for data. Then write the two-paragraph answer to the finance director,
explaining both why 210 paths are unnecessary and why the number is nonetheless
more than the average.

## E. Troubleshoot

**E1.** An organisation migrated from a T1-based PBX to SIP trunking eight months
ago. Since then, staff report that calls occasionally "fail to connect" at busy
times, with a fast busy tone, and that this happens perhaps ten times a day between
09:30 and 11:00.

Evidence gathered:

- The SIP trunk is provisioned for 30 concurrent paths.
- Call detail records show a busy-hour offered traffic of 24.2 erlangs.
- The Internet circuit is 26% utilised at the times of failure.
- No packet loss or jitter is measurable during the failures.
- The failures affect outbound calls only.
- The previous T1 arrangement had two T1s — 48 channels — and no complaints.

Diagnose it quantitatively. Compute the blocking probability at 30 paths for the
measured traffic and compare with the observed failure rate. Explain why the
Internet circuit's utilisation is a red herring. State how many paths are needed for
0.5% blocking, and explain why the previous arrangement did not exhibit the problem.
Then state what should have been done at migration time that was not.
