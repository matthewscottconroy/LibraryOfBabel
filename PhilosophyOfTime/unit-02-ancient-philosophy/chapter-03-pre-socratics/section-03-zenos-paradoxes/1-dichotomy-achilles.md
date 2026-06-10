# 3.1 The Dichotomy and Achilles

---

## Two Paradoxes About Infinite Division

Zeno's first two paradoxes of motion share a common structure: they both exploit the infinite divisibility of continuous space and time to generate an apparent impossibility. Both can be understood as targeting the view that space and time are continuous — that any interval of space or time can be divided into further sub-intervals without limit.

### The Dichotomy

Here is the Dichotomy in Aristotle's formulation (*Physics* 239b11–13):

"The first [paradox] says that motion is impossible, because what moves must first reach the midpoint before it reaches the end."

To make this fully explicit: suppose you want to walk from one side of a room to the other. Before you get there, you must first reach the halfway point. Before you reach the halfway point, you must first reach the quarter-point. Before the quarter-point, the eighth-point. And so on, without limit. Your journey has an infinite number of sub-journeys, each requiring a finite time to complete. Since infinitely many finite times sum to infinity, the journey would take an infinite time, and would therefore never be completed. But also — notice the other direction of the argument — you can't even *begin*: before you take the first step, you must have completed infinitely many prior sub-steps, which is also impossible.

Let us put the argument more carefully:

1. Any journey from A to B can be divided into infinitely many sub-journeys (to the midpoint, then to the three-quarter point, then to the seven-eighths point, etc.).
2. To complete a journey with infinitely many sub-journeys, you must complete infinitely many acts of motion.
3. Infinitely many acts of motion cannot be completed in a finite time.
4. Therefore, you cannot complete any journey — motion is impossible.

### The Achilles

The Achilles paradox (*Physics* 239b14–18) makes the same logical point in a more vivid setting. Achilles, the fastest runner of antiquity, races a tortoise. He gives the tortoise a head start of, say, 100 meters. Achilles runs much faster than the tortoise — suppose he runs ten times as fast.

By the time Achilles has covered the first 100 meters (the tortoise's head start), the tortoise has moved another 10 meters. By the time Achilles covers those 10 meters, the tortoise has moved another 1 meter. By the time Achilles covers that 1 meter, the tortoise has moved another 10 centimeters. And so on.

At each stage, Achilles must first reach the point where the tortoise was, but by the time he does, the tortoise has moved on. The tortoise always maintains a positive lead, however small. Therefore Achilles can never catch the tortoise. But he obviously can — and will, very quickly. Contradiction.

Both paradoxes rest on the claim that an infinite series of tasks (or acts, or sub-journeys) cannot be completed in a finite time. This claim is the target of the mathematical response.

### The Mathematical Response

The mathematical response is elegant and compelling. A convergent infinite series can have a finite sum. The series 1/2 + 1/4 + 1/8 + 1/16 + ... converges to 1. Therefore, infinitely many sub-journeys, each taking a time proportional to its length, take a total time that is *finite* — exactly the time required for the whole journey. Achilles' series of chases — taking times proportional to 100/v, 10/v, 1/v, ... (where v is Achilles' speed) — is a geometric series that sums to a finite value. Achilles catches the tortoise in finite time.

This is the response Aristotle anticipated (*Physics* 233a21–31) and that calculus makes rigorous. It is a genuine solution to the Dichotomy and Achilles as mathematical puzzles.

### The Philosophical Residue

But does the mathematical response fully resolve the philosophical puzzle? Many philosophers think not, and their reasons are instructive.

The mathematical response shows that the *sum* of infinitely many quantities can be finite. But Zeno's worry, at least on some interpretations, is not about the sum of the series but about the *completability* of infinitely many acts. The question is: can you actually *do* infinitely many things — cross infinitely many sub-intervals — in a finite time? The mathematical convergence theorem tells you that the sum of the sub-intervals is finite; it does not obviously settle whether infinitely many acts can be performed.

This worry has been formalized in various ways. Adolf Grünbaum (1967) argued that the paradoxes are fully dissolved by the theory of measure: the motion simply occurs, and the infinite divisibility of the interval traversed does not require infinitely many *distinct acts*. Max Black (1950) argued the opposite: "Doing an infinite number of tasks" is a category error — there is no such thing as completing an infinite number of tasks, so the question is malformed. On Black's view, Zeno was right to worry about completing an infinite series; the resolution is to deny that ordinary motion involves completing infinitely many sub-tasks at all.

Another residue concerns the structure of continuity itself. The mathematical response assumes that physical space and time have the structure of the real number line — that they are infinitely divisible, that every interval corresponds to a real number, that Cantor's transfinite arithmetic applies. But is there actually empirical evidence that physical space and time have this structure? At the Planck scale (approximately 10⁻³⁵ meters, 10⁻⁴³ seconds), the very concepts of continuous space and time may break down. If space and time are discrete at the quantum gravity level — if there is a smallest unit of space and time — then the mathematical response, which assumes real-number continuity, may not apply to the physical world even if it applies to the mathematical model.

This connects Zeno's paradoxes to contemporary physics in a surprising way: the question of whether space and time are fundamentally continuous or discrete remains open (Penrose 2004: 961–966). Zeno identified a deep structural issue that has not been fully settled.

### Connection to Aristotle

Aristotle's response to the Dichotomy in the *Physics* (VIII.8, 263a4–b9) is important and worth understanding. Aristotle distinguishes two senses in which something can be "infinite": *infinite in extension* (no bound on the total length) and *infinite in divisibility* (divisible into ever-finer parts without a smallest). Space and time are infinite in divisibility but finite in extension, he argues. The sub-intervals of a finite journey are infinitely many only *potentially* — in the sense that the division *could* be continued without limit — not *actually*, in the sense of actually being divided. Zeno's paradox arises from confusing potential and actual infinity: the journey contains infinitely many potential sub-journeys, but they are not all actually traversed as separate acts.

This response is similar in spirit to the mathematical response — both point out that potential infinite divisibility does not require actual infinite tasks — but Aristotle provides a more explicitly ontological framework for understanding the distinction.

---

**Key texts:** Aristotle, *Physics* 239b11–18; 233a21–31; 263a4–b9.
