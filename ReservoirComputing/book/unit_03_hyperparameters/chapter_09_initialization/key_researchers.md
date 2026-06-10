# Chapter 9: Key Researchers

## Ali Rodan

Ali Rodan proposed the Simple Cycle Reservoir in collaboration with Peter Tino [RodanTino2011]. The paper was notable for demonstrating, with both theoretical analysis and empirical experiments, that a reservoir with the simplest possible topology — a ring — could match or outperform random ESNs on many benchmark tasks. Rodan's contribution was to see that the uniform eigenspectrum of the permutation matrix was the key property, and that this could be achieved by the most minimal structure imaginable. The SCR paper opened a line of research into minimally structured reservoirs and challenged the implicit assumption that random connectivity was somehow necessary or optimal.

## Peter Tino

Peter Tino is a professor of complex and adaptive systems at the University of Birmingham. His research spans machine learning, dynamical systems, and complex networks, with significant contributions to the theory of recurrent neural networks and reservoir computing. Together with Rodan, he developed the theoretical analysis of the SCR [RodanTino2011], including the exact memory capacity formula $MC = N$ and the connection to the uniform eigenspectrum. Tino has also contributed to the analysis of memory in reservoirs with structured connectivity more broadly, and to the connections between reservoir computing and symbolic dynamics. His group has explored the theoretical foundations of when and why structure helps in recurrent networks, which is one of the central questions of this chapter.

## Jochen Triesch

Jochen Triesch is a professor at the Frankfurt Institute for Advanced Studies (FIAS) and the Goethe University Frankfurt. He proposed intrinsic plasticity in 2005 [Triesch2005] as a homeostatic mechanism for maintaining neurons in their most information-efficient operating regime. The paper drew on earlier work on infomax principles in neural computation [Bell1995, Linsker1988] and applied it specifically to the context of spiking and rate-coded neural networks. Triesch's insight was that the gain and bias of a sigmoid neuron could be adapted by a simple local rule derived from maximizing mutual information, and that this rule could be derived analytically. The IP rule has since been incorporated into reservoir computing frameworks as a standard pre-processing step. Triesch's broader research program concerns self-organization and homeostasis in neural circuits, with the IP rule as a central example of how neurons might adaptively regulate their information transmission properties.
