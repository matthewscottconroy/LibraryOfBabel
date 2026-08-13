# Chapter 4 — Further Reading

## Primary sources

**Shannon, C. E. (1948). "A Mathematical Theory of Communication." *BSTJ* 27.**
Part I (§§1–11) covers entropy and source coding; Part II covers the noisy channel
and capacity. §§6–9 on entropy are readable after this chapter. Theorem 17 is the
capacity result, and §25's discussion of the continuous channel is where
*C* = *B* log₂(1+SNR) appears. The 1949 book edition with Warren Weaver's
introductory essay is the usual way to read it; Weaver's essay is a competent
popularisation but Shannon's own prose is better.

**Nyquist, H. (1928). "Certain Topics in Telegraph Transmission Theory."
*Transactions of the AIEE* 47: 617–644.**
The refined statement of the bandwidth–symbol-rate result and the zero-ISI pulse
shaping criterion. Reading the 1924 and 1928 papers together shows the argument
being sharpened.

**Nyquist, H. (1928). "Thermal Agitation of Electric Charge in Conductors."
*Physical Review* 32: 110–113.** and **Johnson, J. B. (1928). "Thermal Agitation of
Electricity in Conductors." *Physical Review* 32: 97–109.**
Published back to back — the measurement and the theory. Three and seven pages
respectively, and the origin of the *kTB* that sets every noise floor in this book.

**Shannon, C. E. (1951). "Prediction and Entropy of Printed English." *BSTJ* 30:
50–64.**
The human-guessing experiment that estimated English at 0.6–1.3 bits/character.
Charming and completely accessible; the method is one anyone can reproduce.

**Berrou, C., Glavieux, A. & Thitimajshima, P. (1993). "Near Shannon Limit
Error-Correcting Coding and Decoding: Turbo-Codes." *Proc. ICC '93*.**
The paper that closed the gap. Worth reading the abstract and looking at Figure 8,
which is the plot that startled the field.

**Gallager, R. G. (1962). "Low-Density Parity-Check Codes." *IRE Transactions on
Information Theory* 8(1): 21–28.**
Thirty-five years ahead of the hardware. A useful object lesson about publishing.

## Books

**Cover, T. M. & Thomas, J. A. (2006). *Elements of Information Theory*, 2nd ed.
Wiley.**
The standard graduate text. Chapters 2 (entropy), 5 (source coding), 7 (channel
capacity) and 9 (the Gaussian channel) cover this chapter rigorously. Requires
comfort with probability; rewards it.

**MacKay, D. J. C. (2003). *Information Theory, Inference, and Learning
Algorithms.* Cambridge University Press.**
Free online, legally, from the author's page. Idiosyncratic, brilliant, and much
more fun than Cover & Thomas. Chapters 1–10 cover this material; the treatment of
LDPC codes is by the person who rediscovered them.

**Pierce, J. R. (1980). *An Introduction to Information Theory: Symbols, Signals
and Noise*, 2nd ed. Dover.**
Written by a Bell Labs colleague of Shannon's (and the man who named the
transistor). Requires almost no mathematics and conveys the ideas honestly. The
best first book on the subject, and it costs almost nothing.

**Proakis, J. G. & Salehi, M. (2007). *Digital Communications*, 5th ed.
McGraw-Hill.**
The standard engineering reference for everything in Units I and II. Chapter 4 on
optimum receivers and Chapter 6 on carrier and symbol synchronisation are where
the practical consequences of §4.2 live.

**Sklar, B. & Harris, F. J. (2020). *Digital Communications: Fundamentals and
Applications*, 3rd ed. Pearson.**
More accessible than Proakis, with unusually good treatment of link budgets and
*E_b*/*N_0* — directly useful for Chapter 42.

## Applied and practical

**Bahai, A. R. S., Saltzberg, B. R. & Ergen, M. (2004). *Multi-Carrier Digital
Communications: Theory and Applications of OFDM*, 2nd ed. Springer.**
For understanding why real OFDM systems fall short of the Nyquist and Shannon
bounds by the amounts they do. Relevant to Chapter 8 and Chapter 44.

**ITU-T Recommendation V.90 (1998), *A digital modem and analogue modem pair for
use on the PSTN at data signalling rates of up to 56 000 bit/s downstream and
33 600 bit/s upstream*.**
The specification of the trick described in §4.4. Worth skimming §6 to see the
asymmetry made explicit in a standards document.

**`shannon_capacity.py`** and **`link_budget.py`** in this book's
[tools/](../../../tools/) directory — compute capacity from bandwidth and SNR,
plot the capacity curve against SNR, and evaluate a complete link budget in dB
with a stated margin.

## For the certification-minded

None of this chapter's mathematics is on N10-009. Three of its conclusions
effectively are:

- Wider wireless channels carry more data but have shorter range and more
  interference exposure (objectives 2.3, 5.5).
- Signal strength in dBm is meaningful only against a noise floor; SNR determines
  the achievable rate (objective 5.5).
- Advertised wireless rates are not achievable throughput (a recurring distractor
  in performance scenarios).

Understanding *why* each is true makes them impossible to forget, which is the
entire argument for including the chapter.
