# Chapter 8: Bacterial Decision-Making

## Opening Scene

In a 1972 paper in *Nature*, Howard Berg and Douglas Brown published data that changed biology. Using a three-dimensional tracking microscope of Berg's own invention — a device that followed individual bacteria through space with a motorized stage — they had mapped, for the first time, the actual three-dimensional trajectories of *Escherichia coli* cells swimming through liquid. What they found was not what simple intuition would suggest. The bacteria did not swim in arcs or spirals or straight lines. They swam in connected sequences of roughly straight segments — runs — punctuated by brief, stationary reorientations — tumbles — after which each new run proceeded in a direction nearly random relative to the previous one (Berg & Brown, 1972).

This run-tumble behavior, Berg and Brown showed, was the bacterium's solution to a fundamental navigation problem: how do you find food in a world where the food gradient is too shallow to be detected by comparing concentration between your front and back ends, and where Brownian motion would randomize your orientation over the timescales of interest? The answer: bias the random walk. Extend your runs when you're moving in a good direction, tumble to reorient when you're not. The bacterium doesn't know where the food is — it can only compare where it is now to where it was a second ago. But by comparing present to past, and using that comparison to bias the random walk, it navigates gradient landscapes that would defeat any system limited to spatial averaging.

That a bacterium can do this — without a brain, without neurons, without any specialized sensory organ beyond the receptor proteins in its membrane — is one of the most intellectually bracing facts in biology. It is the empirical launching point for this chapter.

## The Central Question

This chapter asks: **in what senses, and to what degree, can bacteria be described as decision-making systems?** 

Decision-making implies alternatives — a set of possible states or actions — and a process that selects among them based on information about the current state. By this definition, bacteria make decisions constantly: to run or tumble, to activate or repress a gene, to divide or enter stationary phase, to resist or succumb to stress. The question is not whether these selections occur but whether the process of selection is sophisticated enough to merit the cognitive vocabulary of "decision-making" rather than the reductive vocabulary of "chemistry."

We argue that the answer depends on how richly we unpack the molecular mechanisms — and that when we do, the mechanisms that support bacterial behavioral flexibility are genuinely interesting, informationally sophisticated, and deserving of serious cognitive analysis.

## What You Will Learn

By the end of this chapter, you should be able to:

- Describe the molecular components of the *E. coli* chemotaxis system and explain how they implement the run-tumble algorithm
- Explain how the methylation-based adaptation system implements short-term memory
- Describe the major classes of bacterial signal transduction systems (two-component systems, second messenger cascades, transcription factor networks)
- Distinguish short-term behavioral adaptation from longer-term transcriptional memory, and discuss whether bacteria can be said to learn
- Explain the bacterial stress response systems (SOS, stringent response, persister cells) as examples of information-driven behavioral switching

---

*Sections in this chapter:*
1. The Chemotaxis System
2. Signal Transduction Networks
3. Bacterial Learning
4. Stress Responses and Survival Decisions
