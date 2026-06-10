# Chapter 25: Key Researchers

## Jeffrey Elman (University of California San Diego)

Jeffrey Elman's simple recurrent network (SRN) [Elman1990] is the direct precursor to reservoir computing applied to language. His experiments on grammaticality judgments — discovering that the SRN's hidden state encodes syntactic categories — are foundational for the reservoir NLP program. Elman's paper "Finding structure in time" (1990) introduced the idea that temporal sequence structure is encoded in the hidden state of a recurrent network, providing the conceptual foundation for probing classifiers.

**Representative works**: [Elman1990], [Elman1991], [Elman1993]

## Peter Tino (University of Birmingham)

Peter Tino has made sustained contributions to the theory and application of reservoir computing for NLP, including grammar induction [TinoEtAl2001], language model benchmarking [TinoEtAl2010], and the theoretical analysis of what symbolic structures reservoir states can represent. His work bridges formal language theory and dynamical systems.

**Representative works**: [TinoEtAl2001], [TinoEtAl2010], [TinoKotismannis2010]

## Tal Linzen (New York University / Google)

Tal Linzen developed the subject-verb agreement benchmark [LinzenEtAl2016] that has become the standard test for syntactic processing in language models, including reservoir systems. His work systematically analyzing how well recurrent models track agreement — with varying numbers of attractor nouns — defines the evaluation methodology used throughout Section 25.2.

**Representative works**: [LinzenEtAl2016], [LinzenEtAl2019], [LinzenBaroni2021]

## Yoav Goldberg (Bar-Ilan University / AI2)

Yoav Goldberg contributed to the agreement tracking paper [LinzenEtAl2016] and has written extensively on what linguistic knowledge is encoded in neural network representations, directly relevant to the probing classifier methodology.

**Representative works**: [LinzenEtAl2016], [GoldbergEtAl2019]

## Chris Manning (Stanford University)

Chris Manning's group developed NLP tools and benchmarks (Stanford Parser, GloVe embeddings, GLUE) that define the competitive landscape for NLP. His work on evaluating syntactic knowledge in neural networks [ManningEtAl2020] frames the question of what reservoir systems can and cannot learn.

**Representative works**: [ManningEtAl2020], [TenneySurvey2019]

## Mantas Lukoševičius (Constructor University, Bremen)

Lukoševičius's practical guide [Lukosevičius2012] includes language modeling examples that have served as starting points for many reservoir NLP implementations. His analysis of hyperparameter effects applies directly to the NLP setting.

**Representative works**: [Lukosevičius2012], [LukoseviciusJaeger2009]

## Herbert Jaeger (Constructor University, Bremen)

Jaeger's original ESN work [Jaeger2001, Jaeger2002MC] demonstrated reservoir networks on temporal tasks that include language-like sequential patterns. His theoretical analysis of fading memory provides the theoretical basis for understanding what linguistic structures are representable in reservoir states.

**Representative works**: [Jaeger2001], [Jaeger2002MC], [JaegerHaas2004]

## Radim Rehurek (RaRe Technologies)

While not a reservoir computing researcher, Radim Rehurek developed the Gensim library and popularized Word2Vec training, providing the pre-trained word embeddings used as reservoir inputs in modern RC language models.
