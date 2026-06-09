# Chapter 13: Slime Molds — Distributed Computing in the Forest Floor

## Opening Scene

The year 2000. A paper appears in *Nature* — the most prestigious scientific journal in the world — and its title stops readers cold: "Maze-solving by an amoeboid organism." The paper describes how Physarum polycephalum, a plasmodial slime mold, was placed at the entrance of a maze with food at the exit. The organism sent exploring tendrils through all available paths. And then — over a period of hours — it withdrew from the dead ends, retaining only the shortest path connecting entrance to exit.

A slime mold. Solving a maze.

The paper (Nakagaki, Yamada, & Tóth, 2000) was not arguing that Physarum was conscious, or even that it was intelligent in any rich sense. But the demonstration was viscerally provocative in a way that went beyond its technical content. Here was an organism with no brain, no neurons, no centralized information processing of any kind — just a distributed network of protoplasm flowing through tubes — and it was solving a classic computational problem that, in artificial systems, requires explicit algorithmic search.

The slime mold had no algorithm. It had flow dynamics. And somehow, flow dynamics was enough.

## The Central Question

Can a system with no neurons, no centralized processing, and no capacity for representation compute optimal solutions to spatial problems?

The question sounds like it should be easy to answer "no." We think of computation as something that requires a computer — something with explicit state, specified operations, a program. Physarum has none of these things. What it has is a network of protoplasmic tubes through which cytoplasm flows, driven by rhythmic contractions of actomyosin in the tube walls, and a feedback mechanism whereby tubes that carry more flow grow thicker (reducing resistance) while tubes that carry less flow thin and eventually disappear.

This is not computation in the Turing sense. But it produces results that look, from the outside, like computation: the organism finds shortest paths, builds efficient networks, and — as we will see — can even anticipate periodic events in a way that implies something functionally equivalent to memory. Whether these accomplishments require us to revise our understanding of what computation is, or merely to appreciate the power of physical dynamics in biological systems, is a question this chapter engages with the seriousness it deserves.

## What You Will Learn

By the end of this chapter, you should be able to:

**Describe the biology of Physarum polycephalum** — its life cycle, the structure of the plasmodial stage, the mechanics of cytoplasmic streaming, and how tube networks form and are remodeled.

**Explain the mechanism of maze-solving and network optimization** in Physarum — specifically, how the flow-reinforcement feedback between tube diameter and flow rate produces shortest-path behavior, and how this mechanism generalizes to network design problems including the Tokyo rail network analogy.

**Analyze the cold-shock anticipation experiment** (Saigusa et al., 2008) and its implications, while maintaining appropriate philosophical caution about what "memory" and "anticipation" mean for an organism without neurons.

**Evaluate Andrew Adamatzky's program of Physarum computing** — understanding what kinds of computational problems Physarum can genuinely solve, what the limitations are, and what the broader implications for unconventional computing might be.

The slime mold is this chapter's protagonist because it makes the challenge to brain-centric views of intelligence as stark as possible. Whatever Physarum is doing, it is doing it without anything we would recognize as a mind. Understanding how is one of the genuinely exciting scientific problems of our time.

---

*Proceed to Section 1: Physarum Biology*
