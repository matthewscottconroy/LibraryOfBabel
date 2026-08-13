# Chapter 59 — Exercises

## A. Recall

**A1.** Define authentication, authorization and accounting, and state what each answers.

**A2.** Describe the 2013 Target breach in terms of the three A's, identifying which failed.

**A3.** Name the three classical factors, and give the two additional ones used in practice.

**A4.** Why is a biometric a poor primary credential? State the property precisely.

**A5.** Why is a password plus a security question not multi-factor?

**A6.** Explain why FIDO2 resists phishing structurally, and why TOTP does not.

**A7.** State two pieces of current password guidance that reverse older advice, with the
reason for each reversal.

**A8.** Name the three 802.1X roles and say which one makes the decision.

**A9.** What does an 802.1X port forward before authentication succeeds?

**A10.** What is the most common 802.1X misconfiguration, and what does it enable?

**A11.** Give three differences between RADIUS and TACACS+, and say which is used for what.

**A12.** What is MAB, and in what sense is it not authentication?

**A13.** What does a RADIUS Access-Accept return besides "yes"?

**A14.** Name the four authorization models and state RBAC's characteristic failure mode.

**A15.** Why do the failure modes of least privilege push consistently towards over-granting?

**A16.** State the zero trust claim in one sentence.

**A17.** What distinguishes zero trust access from a VPN? Give three differences.

## B. Apply

**B1.** For each pair, state whether it is multi-factor and why:

(a) Password + SMS code
(b) Password + security question
(c) Fingerprint + phone possession
(d) Smartcard + PIN
(e) Password + "remember this device" cookie
(f) Certificate on a laptop + password

**B2.** Rank these second factors from strongest to weakest, and for each state the specific
attack that defeats it:

SMS code, TOTP app, push approval, number-matching push, FIDO2 hardware key, email code.

**B3.** An organisation's password policy requires 8 characters with one uppercase, one digit
and one symbol, expiring every 60 days.

(a) Predict what passwords this produces.
(b) State what the policy achieves and what it does not.
(c) Write the replacement policy and justify each change.

**B4.** Choose an EAP method for each and justify:

(a) 4,000 corporate laptops with a managed PKI
(b) 300 contractor devices with no management
(c) 40 IP cameras with no supplicant
(d) A guest Wi-Fi network
(e) 200 medical devices with an 802.1X implementation known to be unreliable

**B5.** Design the RADIUS authorisation attributes returned for each of these authentications on
the same physical switch port:

(a) A domain-joined finance workstation
(b) A contractor's laptop with a valid guest credential
(c) A printer, via MAB
(d) An unknown device that fails authentication

State the VLAN, the ACL in words, and any timers.

**B6.** An organisation has 620 employees and 480 defined roles.

(a) Diagnose.
(b) Propose an approach that reduces the count, and estimate what it would reduce to.
(c) State what you would lose and how you would compensate.

**B7.** Design the privilege model for a network team of nine: two senior engineers, four
engineers, two operators and one security specialist. Specify roles, what each may do, how
elevation works, and what is separated from what.

**B8.** For each, state whether it is a network control, an identity control, or both, and what
happens if it is the only one deployed:

(a) Microsegmentation
(b) Per-application access brokering
(c) 802.1X with dynamic VLAN assignment
(d) Device posture assessment
(e) East–west traffic monitoring

## C. Analyse

**C1.** The chapter argues authentication and authorization fail differently and that the second
is the more common failure. Analyse why: is it harder, less visible, less rewarded, or something
else?

**C2.** Analyse the honest case for SMS as a second factor. Given that it is defeated by SIM
swapping, under what circumstances is deploying it the correct decision, and what does the
"SMS is worthless" message cost?

**C3.** FIDO2 defeats phishing because the user is not the one making the decision. Analyse this
as a general design principle, and identify two other controls in this book that work by
removing a human judgement.

**C4.** Analyse why forced password expiry produces worse passwords. Then analyse why the
guidance took twenty years to change despite the evidence being available throughout.

**C5.** PEAP protects a password inside TLS only if the client validates the server certificate,
and it usually does not. Analyse this as a failure of specification, of implementation, of
default configuration, or of deployment practice — and say which and why.

**C6.** Analyse the 802.1X failure-mode decision. For a hospital, a school, a trading floor and a
factory, state what should happen when RADIUS is unreachable, and justify each differently.

**C7.** Just-in-time elevation is described as the single highest-value change available.
Analyse the claim: what does it actually prevent, what does it cost operationally, and under
what circumstances would you not do it?

**C8.** Zero trust is described as replacing one trusted thing with another. Analyse whether the
identity system is genuinely a better thing to trust than the network, and state the conditions
under which it is not.

**C9.** Analyse the residual perimeter problem: an organisation with fifteen legacy applications
that cannot participate in a zero trust model. What is the correct architecture, and what should
be said about it honestly in a design document?

## D. Design

**D1.** Design the authentication architecture for a 2,000-person organisation: identity
provider, factors by user population and by resource sensitivity, the treatment of service
accounts and machine credentials, and the break-glass path. Address what happens when the
identity provider is unavailable.

**D2.** Design an 802.1X deployment for a campus of 4,000 wired ports and 600 access points.
Cover: EAP method per device class, the fallback for devices that cannot authenticate,
authorisation attributes per class, failure modes, the RADIUS architecture and its redundancy,
and the phased rollout including monitor mode.

**D3.** Design the device administration AAA for a 300-device estate: protocol, roles, per-command
authorisation, accounting, what is separated from what, the break-glass arrangement, and how the
shared `enable` password is eliminated.

**D4.** Design an access review process for an organisation of 800 people: what is reviewed, by
whom, how often, what the default outcome is, and how you would prevent it becoming a rubber
stamp. State how you would measure whether it is working.

**D5.** Write a two-page assessment of an organisation's readiness for zero trust: what it would
have to have in place first, what its legacy applications would require, what the identity
provider's availability target would become, and a realistic three-year sequence. Be explicit
about what you would not attempt.

## E. Troubleshoot

**E1.** A laptop has link and no network on a wired port. Describe your diagnosis in order.

**E2.** Credentials for domain users are being harvested by a rogue access point in a car park.
Explain exactly what is misconfigured and give the specific client settings that fix it.

**E3.** A RADIUS server is rebooted for patching and every wireless client across the estate
disconnects. Explain why and give two configuration changes.

**E4.** A printer authenticates successfully and reaches the finance servers. Explain what is
wrong and where.

**E5.** After an 802.1X rollout, thirty devices in a building stop working and nobody knows what
they are. Explain what step was omitted and what you do now.

**E6.** An engineer changed the AAA configuration on a router and the change log shows only
"admin". Analyse the two failures.

**E7.** A user's access review was completed with all 47 grants approved in under a minute.
Assess whether the review achieved anything, and state the design change.

**E8.** A zero trust deployment is complete and an attacker who compromised one laptop
successfully scans and exploits three internal servers. Explain how this is possible.

**E9.** Users are prompted to re-authenticate every 15 minutes and are writing their passwords
on notes. Diagnose in terms of Chapter 57's principles and give the fix.

## F. Extend

**F1.** Configure 802.1X in a lab: `hostapd` or a switch, `freeRADIUS`, and a client. Get
EAP-TLS working with certificates you issue yourself. Then deliberately misconfigure the client
to accept any server certificate, stand up a second RADIUS server with a different certificate,
and demonstrate the credential capture with PEAP.

**F2.** Capture a RADIUS exchange with Wireshark and identify what is and is not encrypted.
Report exactly what an observer on the path learns.

**F3.** Register a FIDO2 key or a passkey with a service, then attempt to authenticate through a
proxy or a lookalike hostname you control. Document what happens and explain the mechanism.

**F4.** Audit the SSH `authorized_keys` files across a set of systems you administer. For each
key, determine whose it is and when it was added. Report the proportion you could not attribute.

**F5.** Take a role definition from a system you have access to and enumerate every permission it
actually grants. Compare with what a person in that role does in a week. Report the gap.

**F6.** Read NIST SP 800-63B sections 5 and 10. Compare its guidance with the password policy of
an organisation or service you use, and list every divergence with the likely reason.

**F7.** Read NIST SP 800-207 and produce a one-page assessment of a network you know against its
seven tenets, with evidence for each rating.

**F8.** Configure TACACS+ per-command authorisation in a lab and demonstrate a role that may run
`show` commands but not `configure terminal`. Capture the accounting records and report what
they contain.
