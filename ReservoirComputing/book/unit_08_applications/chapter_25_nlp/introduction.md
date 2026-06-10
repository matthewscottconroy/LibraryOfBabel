# Chapter 25: Natural Language Processing and Symbolic Computation

## Introduction

Natural language is the most complex sequential structure that humans produce and consume. Its complexity operates at multiple levels simultaneously: phonological (sound patterns), morphological (word structure), syntactic (sentence structure), semantic (meaning), pragmatic (contextual interpretation), and discourse (multi-sentence coherence). Any computational system that processes natural language must grapple with this multi-level, hierarchically structured complexity.

Reservoir computing's engagement with natural language is therefore both intellectually ambitious and revealing of fundamental properties and limitations. Can a reservoir — a system with no built-in inductive biases toward hierarchical structure, no explicit grammar, and limited long-range memory — learn anything useful about language? The answer, perhaps surprisingly, is yes — within important limits. Understanding precisely where reservoir computing succeeds and where it fails in language processing illuminates both the capabilities of reservoir dynamics and the specific computational demands of linguistic structure.

The intellectual history here is rich. Early connectionionist models of language (Elman's SRN, 1990) are direct precursors to ESNs, and Elman's experiments on grammaticality judgments in simple recurrent networks are among the most theoretically important results in computational linguistics. The reservoir computing framework clarifies and extends Elman's findings: the hidden state of a recurrent network encodes syntactic structure in a form detectable by simple linear probes — the readout layer of an ESN.

This chapter examines four aspects of reservoir computing and language: language modeling (predicting the next word), agreement dependency tracking (subject-verb number agreement across intervening material), grammar induction (learning grammatical constraints from corpus statistics), and the limits of reservoir computation for language (what linguistic phenomena reservoir systems cannot handle).

### A Note on the Post-Transformer Era

The chapter is written in the context of a fundamental shift in NLP: transformer-based large language models (GPT-4, Claude, Gemini, Llama) now achieve near-human performance on a wide range of language tasks, trained on vast corpora with hundreds of billions of parameters. Reservoir computing's role in this context is not as a competitive language model but as a tool for:

1. **Scientific investigation** — reservoir systems are theoretically tractable and provide insight into what information is encoded in recurrent neural states
2. **Low-resource scenarios** — when labeled data is scarce and full fine-tuning of a large model is impractical
3. **Online learning** — when language patterns change continuously and the model must update in real time
4. **Edge deployment** — when computation is constrained and a trained LLM is too expensive to run

These are meaningful niches, and we treat them seriously.
