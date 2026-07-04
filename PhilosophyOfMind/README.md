# Philosophy of Mind: A Textbook

This repository contains a comprehensive, encyclopedic treatment of philosophy of mind, organized as the scaffolding for a full textbook. The goal is to cover the field in its entirety — from ancient Greek theories of the soul to contemporary debates about artificial intelligence and consciousness — with depth sufficient for both undergraduate coursework and graduate-level research.

## How This Textbook Is Organized

The material is arranged hierarchically:

- **Topics** — the broadest thematic divisions of the field
- **Units** — major divisions within each topic
- **Chapters** — focused treatments of a specific question or tradition
- **Sections** — subdivisions of each chapter
- **Subsections** — individual markdown documents representing the finest-grained content

Each subsection is written in a conversational textbook tone: intellectually rigorous but accessible, treating the reader as a thoughtful person who wants to understand not just what philosophers have said but why those questions matter and how the debates connect.

## Topics

| # | Topic | Description |
|---|-------|-------------|
| 01 | [Foundations and History](01_Foundations_and_History/) | From ancient Greek psychology to the cognitive revolution |
| 02 | [The Mind-Body Problem](02_Mind_Body_Problem/) | Dualism, physicalism, panpsychism, and the alternatives |
| 03 | [Consciousness](03_Consciousness/) | The hard problem, theories of consciousness, altered states |
| 04 | [Intentionality and Mental Content](04_Intentionality_and_Mental_Content/) | Aboutness, propositional attitudes, externalism |
| 05 | [Mental Representation](05_Mental_Representation/) | The language of thought, mental imagery, anti-representationalism |
| 06 | [Functionalism](06_Functionalism/) | Causal-role theories, machine functionalism, qualia objections |
| 07 | [Perception](07_Perception/) | Sense data, direct realism, predictive processing |
| 08 | [Action and Agency](08_Action_and_Agency/) | Philosophy of action, free will, mental causation |
| 09 | [Cognitive Architecture](09_Cognitive_Architecture/) | Computationalism, connectionism, embodied and extended mind |
| 10 | [Emotions and Affect](10_Emotions_and_Affect/) | Theories of emotion, moral emotions, moods |
| 11 | [Personal Identity and the Self](11_Personal_Identity_and_Self/) | Continuity, the self, thought experiments |
| 12 | [Language and Thought](12_Language_and_Thought/) | LOT, concepts, linguistic relativity, inner speech |
| 13 | [Memory and Time-Consciousness](13_Memory_and_Time_Consciousness/) | Philosophy of memory, temporal experience, prospection |
| 14 | [Other Minds and Social Cognition](14_Other_Minds_and_Social_Cognition/) | The other minds problem, theory of mind, collective intentionality |
| 15 | [Phenomenology](15_Phenomenology/) | Husserl, Heidegger, Merleau-Ponty, Sartre |
| 16 | [Philosophy of Neuroscience](16_Philosophy_of_Neuroscience/) | NCC, reduction, neuroethics |
| 17 | [Animal Minds and Consciousness](17_Animal_Minds/) | Animal cognition, pain, moral status |
| 18 | [Artificial Minds and AI](18_Artificial_Minds/) | The Turing test, LLMs, machine consciousness |
| 19 | [Ethics and Philosophy of Mind](19_Ethics_and_Philosophy_of_Mind/) | Moral psychology, neuroethics, consciousness and moral status |
| 20 | [Methodology](20_Methodology/) | Intuitions, thought experiments, naturalism, interdisciplinary connections |

## A Note on Scope and Approach

Philosophy of mind sits at the intersection of several disciplines — neuroscience, cognitive science, linguistics, computer science, psychology — and this textbook tries to honor that interdisciplinary character without losing its philosophical focus. The central questions driving the field are not merely empirical but conceptual: What is the relationship between mind and body? What is it for a mental state to be *about* something? Is consciousness something over and above physical brain processes? Could a machine ever genuinely think?

These questions have occupied philosophers for millennia and show no signs of being settled. That is not a failure of the field; it is a sign that the questions are genuinely deep. Good philosophy of mind cultivates the ability to hold multiple positions simultaneously, to understand why each has seemed compelling, and to see clearly where the pressure points are.

## Reading Paths

For structured routes through the twenty topics tailored to different
backgrounds and goals, see **[READING-PATHS.md](READING-PATHS.md)**.

## Commands

Run from the repository root.

```bash
# Build the book (also --html, --markdown, --check)
python3 tools/build_book.py PhilosophyOfMind --pdf

# Take the adaptive quiz
cd quiz && cargo run -p quiz-cli -- --subject ../PhilosophyOfMind

# Validate before opening a PR
python3 tools/validate.py
```

See [PROCESS.md](../PROCESS.md) for the full pipeline and
[CONTRIBUTING.md](../CONTRIBUTING.md) to contribute.
