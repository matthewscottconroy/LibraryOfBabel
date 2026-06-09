# Chapter 34: Lessons from Biological Intelligence for AI

## A Robot That Could Not Climb Stairs

In the late 1980s, the dominant paradigm in robotics was what Rodney Brooks called "sense-model-plan-act": build a detailed internal model of the world, use it to plan a sequence of actions, then execute the plan. The approach had produced impressive demonstrations in carefully controlled laboratory environments. But it failed, consistently and sometimes spectacularly, when robots were asked to do something as simple as walk across an uneven floor or pick up an object that wasn't exactly where the map said it would be. The world, it turned out, did not behave like a laboratory.

Brooks's response was to scrap the model. His robots would not maintain rich internal representations of the environment. Instead, they would couple directly to the world through a layered stack of behaviors, each responding to real sensory input in real time. The approach — subsumption architecture — produced robots that were, in some respects, more capable than their more elaborate predecessors. They could navigate cluttered rooms, track moving targets, and adapt to unexpected obstacles. They were doing something closer to what a cockroach does than what a chess-playing algorithm does.

That contrast — chess-playing algorithm versus cockroach — is the central provocation of this chapter.

## The Central Question

The dominant paradigm in contemporary AI — deep learning applied to large datasets — has achieved extraordinary things: superhuman performance on image recognition, language translation, protein structure prediction, and strategic board games. These are genuine achievements, and they should not be understated. But they raise a pointed question that this book's preceding chapters have prepared you to ask with some precision: **what kinds of intelligence are deep learning systems missing, and why?**

The answer, we will argue, has everything to do with embodiment, real-time coupling with a physical environment, energy efficiency, and the distributed, substrate-independent logic of biological cognition. None of these are exotic or mystical properties. They are features of the kind of intelligence that has been continuously operative on this planet for nearly four billion years, and they remain largely absent from the AI systems we build today.

The organisms that appear throughout this book — bacteria, slime molds, plants, mycelia — are not interesting primarily as curiosities. They are existence proofs. They demonstrate that information integration, memory, anticipation, and adaptive behavior can be implemented without centralized control, without explicit symbolic representation, without vast training datasets, and with energy budgets that would make an iPhone blush. Understanding *how* they do this is, we will argue, one of the most productive directions for the next generation of AI research.

## Chapter Roadmap

**Section 34.1 — The Failure of Brain-Centric AI** takes stock of what deep learning can and cannot do, and traces the failure modes back to a fundamental architectural assumption: that intelligence is primarily about pattern matching over large corpora, decoupled from sensorimotor engagement with the world. We examine the embodiment gap and ask what slime molds and bacteria do that large language models cannot.

**Section 34.2 — Stigmergic and Swarm AI** explores how the logic of collective, decentralized intelligence — the logic of ant colonies and mycelial networks — has been formalized in algorithms and multi-agent systems. Marco Dorigo's ant colony optimization and related approaches are examined not just as practical tools but as instances of a deeper principle: that intelligent behavior can emerge from simple local rules without any agent knowing the global solution.

**Section 34.3 — Embodied Robotics** returns to Brooks's insight and traces its development into soft robotics, evolutionary robotics, and Michael Levin's target morphology framework. The common thread is that intelligent systems need bodies that do cognitive work — that offload computation into physics rather than reserving it all for central processing.

**Section 34.4 — Neuromorphic Computing** examines the frontier of hardware designed to implement cognition the way cells do: with spiking dynamics, local learning rules, in-memory computation, and energy costs measured in femtojoules rather than watts. The gap between what biology has built and what silicon has achieved remains large, but it is beginning to narrow.

---

*Sections in this chapter:*
1. The Failure of Brain-Centric AI
2. Stigmergic and Swarm AI
3. Embodied Robotics
4. Neuromorphic Computing
