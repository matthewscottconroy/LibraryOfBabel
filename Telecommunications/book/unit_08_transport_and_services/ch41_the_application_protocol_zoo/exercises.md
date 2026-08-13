# Chapter 41 — Exercises

## A. Recall

**A1.** Give the HTTP status code classes and one example from each.

**A2.** What distinguishes a safe method from an idempotent one? Give a method that is
neither.

**A3.** What problem did each HTTP version solve, from 0.9 through 3?

**A4.** What three properties does TLS provide, and which is most often underestimated?

**A5.** Give the port for: SSH, Telnet, FTP control, FTP data, TFTP, SMTP submission, POP3S,
IMAPS, LDAP, LDAPS, NTP, SIP, SNMP, syslog, RDP.

**A6.** Distinguish SFTP from FTPS.

**A7.** Distinguish SMTP from POP3 and IMAP by direction and purpose.

**A8.** Name the three mail anti-spoofing mechanisms, what each checks, and which survives
forwarding.

**A9.** Give the syslog severity levels 0–7 in order.

**A10.** Distinguish SIP from RTP.

## B. Apply

**B1.** Write, by hand, the HTTP/1.1 request a browser sends for
`https://example.com/about`, with the minimum required headers. Then write a plausible
200 response.

**B2.** For each, give the status code you would expect and say whether the fault is the
client's or the server's:

(a) a page that has moved permanently  (b) an unauthenticated request to a protected
resource  (c) the origin server did not respond to the proxy in time  (d) the resource has
not changed since the client last fetched it  (e) too many requests  (f) a bug in the
application code

**B3.** Compute the round trips to first application byte for: HTTP over TCP, HTTPS over
TCP+TLS 1.2, HTTPS over TCP+TLS 1.3, HTTP/3 first connection, HTTP/3 resumed. Then give
the wall-clock time for each on a 120 ms path.

**B4.** A VoIP deployment uses G.711 (160-byte payload every 20 ms). Compute:

(a) the on-the-wire bandwidth per call over Ethernet
(b) the bandwidth for 200 concurrent calls
(c) the same for G.729 (20-byte payload)
(d) the ratio of on-the-wire to codec rate for each

**B5.** Decode syslog PRI values 0, 30, 134, and 191. Give facility and severity for each.

**B6.** For each scenario choose the protocol and justify:

(a) copying a 4 GB file to a server you administer
(b) loading firmware onto a switch that has no OS running
(c) a user reading mail from a phone and a laptop
(d) a monitoring system polling 3,000 devices every minute
(e) an application authenticating users against a central directory
(f) live audio between two people
(g) a recorded lecture watched by 5,000 students

**B7.** Write the DNS records for a domain implementing SPF, DKIM and DMARC in monitoring
mode.

## C. Analyse

**C1.** Explain why HTTP/1.0's one-request-per-connection model made the transport cost
dominate, using Chapter 37 and Chapter 38's mechanisms.

**C2.** Explain why the `Host` header was necessary and what it enabled.

**C3.** Explain why HTTP/2 was measurably worse than HTTP/1.1 on lossy paths, and why
HTTP/3 fixes it.

**C4.** "Encryption without authentication protects you from a passive observer and not
from an active one." Explain, and state what the certificate contributes.

**C5.** Explain why the CA trust model is a logical OR, what that means for its security,
and how Certificate Transparency and CAA each mitigate it.

**C6.** Explain SSH's TOFU model: what it protects against, what it does not, and two ways
to close the gap.

**C7.** Explain why SFTP is preferable to FTPS across NAT, referring to Chapter 33 §33.3.

**C8.** Explain the envelope/header distinction in SMTP and why it is the basis of mail
spoofing. Then explain what DMARC alignment adds.

**C9.** Explain why SPF breaks on forwarding and DKIM does not.

**C10.** Explain why NTP's offset calculation is wrong on an asymmetric path, and by how
much.

**C11.** "Jitter sets the latency." Explain, using the jitter buffer.

**C12.** Explain why streaming video uses TCP while conferencing uses UDP, referring to
Chapter 36 §36.2's criteria.

**C13.** Explain why an SNMP trap is least likely to arrive exactly when it is most needed.

## D. Design

**D1.** For the semester project's network, list every application protocol in use, its
port, its transport, and whether a more secure alternative should replace it.

**D2.** Write the protocol migration plan for an organisation still running Telnet, FTP,
SNMPv2c and plaintext LDAP. Order the work by risk and state what breaks at each step.

**D3.** Design the QoS marking and queueing policy for a site carrying voice, video
conferencing, streaming and bulk data. Justify each class against §41.4's requirements.

**D4.** Design the logging architecture: what is logged where, over what transport, with
what retention, and how clocks are kept consistent.

**D5.** Design the mail authentication deployment for a domain that sends from its own
servers, a marketing platform and a ticketing system. Give the records and the rollout
order.

## E. Troubleshoot

**E1.** A website returns 502 intermittently. What does that tell you about where the fault
is, and what do you check?

**E2.** HTTPS works in a browser and a script reports a certificate error. Give two causes.

**E3.** A whole fleet reports certificate errors at the same moment. Certificates were
renewed last week. Diagnose.

**E4.** `ssh` reports "REMOTE HOST IDENTIFICATION HAS CHANGED". State the two possible
causes and how you would distinguish them.

**E5.** FTP works from the office and fails from home. Diagnose, and give the mode that
would work.

**E6.** Outbound mail from an application fails; the same account works in a mail client.
Diagnose.

**E7.** Legitimate mail from a domain started going to spam after a new marketing platform
was added. Diagnose.

**E8.** Nobody in the domain can authenticate. Network connectivity is fine and the domain
controllers are up. Give the first thing to check.

**E9.** A VoIP call connects and has audio in one direction only. Give three candidate
causes and how to distinguish them.

**E10.** Voice quality is poor during business hours and fine at night. Jitter measures
under 30 ms on average. Diagnose.

**E11.** A monitoring system shows a switch as up while its logs show it rebooted twice.
Explain both observations.

**E12.** Log entries from two devices cannot be correlated because the timestamps
disagree by four minutes. Give the fix and one other thing that is probably also broken.

## F. Extend

**F1.** Perform an HTTP request by hand with `nc`, then the same over TLS with
`openssl s_client`. Compare what you had to do.

**F2.** Use `openssl s_client -connect host:443` against three sites. Report the TLS
version, cipher, certificate issuer and expiry for each, and check them against Certificate
Transparency logs.

**F3.** Capture a full SIP call setup and the RTP that follows. Identify the SDP body, the
negotiated codec, and the addresses and ports the media used. Confirm whether the media
took the same path as the signalling.

**F4.** Configure SPF, DKIM and DMARC for a test domain and use a mail-testing service to
verify each. Then deliberately break one and observe the report.

**F5.** Set up a syslog collector, send messages at several severities, and verify which
arrive. Then flood it over UDP and quantify the loss. Repeat over TCP.

**F6.** Compare `chronyc sources` output on a machine with three upstream servers before
and after blocking one. Explain the selection changes.

**F7.** Measure the on-the-wire bandwidth of a real VoIP call with a capture and compare
with your answer to B4.
