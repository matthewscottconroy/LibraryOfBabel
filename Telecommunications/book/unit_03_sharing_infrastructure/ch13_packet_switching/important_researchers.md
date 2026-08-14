# Chapter 13 — The People

**Paul Baran (1926–2011).** Polish-born, raised in Philadelphia, and working at the
RAND Corporation when he produced *On Distributed Communications* between 1960 and
1964 — eleven volumes describing packet switching, distributed routing, digital
transmission and end-to-end encryption, none of which existed.

He was not a communications engineer by training and has said this helped: he was
not encumbered by knowing how telephony worked. The design was shown to AT&T and
rejected; the Air Force funded a demonstration; the programme was transferred to an
agency Baran judged incapable of building it well, and he **recommended it be
cancelled rather than built badly and discredited** — a decision worth noting, since
it is not what most inventors do.

He later founded seven companies, including one that produced an early
metropolitan-area wireless system and another whose technology became a substantial
part of DSL. Asked repeatedly about credit for the Internet, he consistently deflected
toward Davies, Roberts, Kleinrock and Cerf, and described the invention as something
that "the technology of the time was going to produce anyway".

**Donald Davies (1924–2000).** Welsh physicist and computer scientist at the National
Physical Laboratory, who had worked with Turing on the Pilot ACE. He arrived at
packet switching in 1965 from the time-sharing problem, learned of Baran's work in
1966, and found the designs essentially identical.

He gave the field its word. Wanting something short, ordinary and translatable, he
consulted a linguist and chose **packet** because it existed with the same sense in
French and German. He also built one — the NPL network ran from 1969 for a decade —
making it arguably the first operational packet-switched network, though at the scale
of a single building. British funding for a national network was not forthcoming,
which is a recurring theme in British computing history.

**Leonard Kleinrock (b. 1934).** His 1962 MIT thesis applied queueing theory to
message-switched networks, producing the mathematics of §13.3. The ARPANET's first
node was installed in his UCLA laboratory in September 1969, and he was present for
the first message — an attempt to type LOGIN that crashed after LO.

The extent to which his thesis constitutes "the invention of packet switching" has
been publicly and sometimes sharply disputed, notably by Baran's and Davies's
supporters, who point out that the thesis concerns message switching and does not
describe fixed-size packets or distributed adaptive routing. Kleinrock has defended
his claim; others have not accepted it. What is undisputed is that he supplied the
delay mathematics, and this book credits him for that rather than adjudicating the
rest.

**Lawrence Roberts (1937–2018).** Programme manager at ARPA who designed the ARPANET,
having read Baran's reports and heard Davies's work presented. He is the person who
took two independent theoretical designs and built a network, which is a distinct
and undervalued contribution. He later ran Telenet, the first commercial
packet-switched network.

**Bob Taylor (1932–2017).** The ARPA director who funded the ARPANET, reportedly
after becoming frustrated with having three separate terminals in his office for
three separate computers. His account of the motivation is consistently
**resource sharing**, not survivability, and he was direct about this whenever the
nuclear-war story was repeated to him. He later ran Xerox PARC's computer science
laboratory during the period that produced Ethernet, the Alto and the laser printer.

**Roger Scantlebury.** Davies's colleague at NPL, who presented the NPL work at the
1967 Gatlinburg symposium where Roberts was present — the meeting at which the
British and American lines of work connected, and at which Roberts learned of
Baran's reports. He also argued, successfully, for higher line speeds in the ARPANET
design than had been planned.

**Will Leland, Murad Taqqu, Walter Willinger and Daniel Wilson.** Their 1993 paper
*On the Self-Similar Nature of Ethernet Traffic* demonstrated from Bellcore's
Ethernet traces that real network traffic is bursty at **every timescale** — that
zooming in or out produces statistically similar pictures — and that Poisson models
therefore systematically understate queue lengths. It is one of the most cited
measurement papers in networking and it changed how capacity planning is done.

**Sally Floyd and Kathleen Nichols.** Floyd's work on Random Early Detection
(1993, with Van Jacobson) was the first serious attempt at active queue management,
and Nichols's later work with Jacobson produced **CoDel** (2012), which finally
made AQM deployable by removing the need to tune parameters. Floyd's broader contributions to
congestion control and to the culture of network measurement were substantial; she
died in 2019.

**Jim Gettys.** Coined **bufferbloat** in 2010 after investigating why his
home connection behaved so badly under load, and then spent years demonstrating that
the problem was endemic across the industry — in routers, in operating systems, in
device drivers, and in cable modems. The campaign that followed produced FQ-CoDel and
CAKE and changed the default behaviour of a great deal of consumer equipment. A good
example of one person noticing something everyone had accepted and refusing to accept
it.
