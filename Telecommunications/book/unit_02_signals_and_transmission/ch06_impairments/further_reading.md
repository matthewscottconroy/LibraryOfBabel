# Chapter 6 — Further Reading

## Primary sources

**Thomson, W. (1855). "On the Theory of the Electric Telegraph." *Proceedings of
the Royal Society of London* 7: 382–399.**
The law of squares. Heavy going, and the moment cable engineering became a
quantitative science rather than a craft.

**Heaviside, O. (1887). "Electromagnetic Induction and its Propagation."
*The Electrician* (series).**
The distortionless-line condition and the case for loading. Heaviside's prose is
famously idiosyncratic and worth sampling for that alone; his mathematics was
rejected by referees who could not follow it and was later found correct.

**Johnson, J. B. (1928). "Thermal Agitation of Electricity in Conductors."
*Physical Review* 32: 97–109**, and **Nyquist, H. (1928). "Thermal Agitation of
Electric Charge in Conductors." *Physical Review* 32: 110–113.**
Seven pages and three pages, published consecutively: the measurement and the
theory. The origin of *kTB*.

**Friis, H. T. (1944). "Noise Figures of Radio Receivers." *Proceedings of the
IRE* 32(7): 419–422.**
Four pages, and the cascade formula that determines where the amplifier goes.

**Lucky, R. W. (1965). "Automatic Equalization for Digital Communication."
*Bell System Technical Journal* 44(4): 547–588.**
The adaptive equaliser. Worth reading §I for the problem statement, which is
essentially §6.3's, and for the observation that a modem cannot know in advance
what line it will be plugged into.

## Books

**Horowitz, P. & Hill, W. (2015). *The Art of Electronics*, 3rd ed. Cambridge.**
Chapter 8 on noise, and the appendices on transmission lines. The best available
treatment of noise for someone who wants to build things rather than prove
theorems, and unusually honest about which effects matter in practice and which
are textbook decoration.

**Ott, H. W. (2009). *Electromagnetic Compatibility Engineering.* Wiley.**
The standard reference on interference, shielding, grounding and the ground-loop
problem that §6.4 warns about. Chapter 3 on cabling and chapter 5 on shielding
explain the earthing decision properly, and it is the book to consult before
specifying shielded cable in an industrial installation.

**Johnson, H. & Graham, M. (1993). *High-Speed Digital Design: A Handbook of Black
Magic.* Prentice Hall.**
The classic on why fast digital signals behave like analog ones. Chapters 1–4
cover rise time, transmission line effects, and why "it's just a wire" stops being
true. The title is accurate about how the subject feels before you read it and
inaccurate afterwards, which is the point.

**Agrawal, G. P. (2012). *Fiber-Optic Communication Systems*, 4th ed. Wiley.**
The standard reference for the optical material in §6.1 and §6.3. Chapter 2 on
fibres covers attenuation and all three dispersion mechanisms rigorously.
Demanding, and the place to go when the summary here is insufficient.

**Ramo, S., Whinnery, J. R. & Van Duzer, T. (1994). *Fields and Waves in
Communication Electronics*, 3rd ed. Wiley.**
Where the skin effect comes from, derived. For readers who want §6.1's √*f* to be
a consequence rather than an assertion.

## Standards

**TIA-568.2-D / ISO-IEC 11801, cabling standards.**
The source of every NEXT, FEXT, PSNEXT, ACR and alien-crosstalk specification in
§6.4, and of the category bandwidth ratings. Read the parameter list once — it is
instructive to see how many distinct measurements a cable must pass, and it makes
clear why a continuity tester is not a certifier.

**ITU-T Recommendation G.652 (single-mode fibre) and G.651.1 (multimode).**
Where the attenuation and dispersion figures in §6.1 and §6.3 are specified.
G.652.D is the low-water-peak variant that opened the band around 1,383 nm.

**IEEE 802.3, Clause 40 (1000BASE-T) and Clause 55 (10GBASE-T).**
The link budgets and channel models that the 100 m limit falls out of. Clause 55's
treatment of alien crosstalk is the reason Cat6a exists.

## Applied

**Fluke Networks and Viavi application notes on cable certification.**
Freely available, vendor-written, and genuinely good on what each measurement
means and what a failure of each indicates. The material on split pairs and on
diagnosing NEXT failures is directly useful and better than most textbook
treatments.

**The Fiber Optic Association (foa.org) reference guides.**
Free, practical, and correct on loss budgets, connector cleaning and OTDR
interpretation. The cleaning material is worth reading precisely because dirty
connectors are the most common fibre fault and the least glamorous topic.

## Tools

**`perfcalc.py noise`** and **`perfcalc.py linkbudget`** in this book's
[tools/](../../../tools/) directory — compute a noise floor with a stated noise
figure, and work a complete budget with margin.

**An eye diagram, if you can get near a fast oscilloscope.** Twenty minutes
watching an eye close as you add cable length does more for §6.3 than any amount
of reading. Many university labs have a scope capable of it and never use it for
this.

## For the certification-minded

Objective 5.2 covers cable connectivity issues and names attenuation, interference,
crosstalk and EMI directly. Objective 1.5 covers media characteristics and the
shielded/unshielded decision. This chapter supplies the mechanisms behind both.

Two things worth over-learning for the exam and for practice alike: **the interface
counters** and what each indicates (§6.1's and §6.4's "what breaks here" sections,
consolidated in Chapter 65 §65.1), and **the difference between a continuity tester
and a certifier**, which appears as an exam distractor and as a real diagnostic
decision.
