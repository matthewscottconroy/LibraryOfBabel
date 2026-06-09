# What CELLO Does: Automating Genetic Circuit Design

In 2016, a research team published a paper in *Science* describing a computer program that accepted a logical specification — "output is high when inputs A and B are both high, and input C is low" — and returned a synthesis-ready DNA sequence. No manual part selection. No iterative debugging of signal compatibility. No checking whether the same transcription factor appeared twice. The program, called CELLO, did all of this automatically, in minutes, and demonstrated that 45 of the 60 circuits it designed actually worked when built. For an engineering discipline where a working circuit design from a non-expert had previously been a major achievement, this was a striking shift.

**CELLO (Cellular Logic)** is a computational tool published by Nielsen et al. in *Science* in 2016 that automates the design of genetic logic circuits in *E. coli*. It takes a desired Boolean function as input and outputs a specific DNA sequence — including all promoters, RBS sequences, and coding sequences — that implements the function using characterized transcription factor gates. CELLO represents the maturation of the genetic parts characterization paradigm: when enough parts are quantitatively characterized in a consistent context, circuit design can be automated from function to sequence.

## The Automated Design Problem

Manually designing a genetic circuit to implement a specific Boolean function involves:
1. Decomposing the function into elementary gates (AND, OR, NOT, NOR, NAND)
2. Assigning biological gate implementations (which TF controls which promoter) to each abstract gate
3. Checking signal compatibility: the output concentration range of each gate must fall within the input sensitivity range of the next gate
4. Verifying that no pair of gates uses conflicting parts (e.g., the same TF used for two different gates)
5. Optimizing for overall circuit performance (highest ON/OFF ratio for the worst-case output state)

For a 3-input Boolean function with 4–8 gates, this is tractable manually. For a 5-input function with 12–20 gates, it is a combinatorial search problem with potentially millions of gate assignments to evaluate.

CELLO automates this process entirely, from Boolean specification to DNA sequence, in minutes.

## The User Interface: Verilog-Like Specification

CELLO accepts circuit specifications in a hardware description language (HDL) format similar to Verilog:

```verilog
// 2-input AND gate specification for CELLO
module circuit(
  input a_sensor,
  input b_sensor,
  output gfp
);

  wire not_a, not_b, nor_ab;
  
  NOT(not_a, a_sensor);
  NOT(not_b, b_sensor);
  NOR(nor_ab, not_a, not_b);
  assign gfp = nor_ab;  // AND(a,b) = NOR(NOT(a), NOT(b))

endmodule
```

This specification tells CELLO:
- There are two inputs (a_sensor, b_sensor) connected to inducer-responsive promoters
- The output is GFP driven by a circuit promoter
- The logic is an AND gate
- Internally, this uses two NOT gates and one NOR gate

CELLO then takes this abstract design and assigns specific biological components from its gate library to implement each logical operation.

## The Gate Library: Quantitative Foundation of CELLO

CELLO's power rests on its **gate library**: a collection of TF-based NOT/NOR gates with measured, quantitative transfer functions in *E. coli* MG1655.

Each gate in the library is characterized by:
- **Input range** [REU]: the range of fluorescence (in REU relative to J23101) that the gate accepts as input from the upstream promoter
- **Output range** [REU]: the range of output fluorescence the gate produces
- **Hill function parameters**: $\alpha$, $K$, $n$ for the TF-promoter interaction
- **ON/OFF ratio**: ratio of output in full-ON vs. full-OFF state

For the 2016 CELLO paper, the library consisted of **12 characterized TF gates** in *E. coli*:
- LacI, TetR, AraC-based repressors (natural)
- Several synthetic TFs from the Voigt lab's collection

Each gate was characterized using the same reporter (sfGFP), same plasmid backbone, and same *E. coli* strain — creating a consistent measurement context that makes inter-gate comparisons valid.

## Inputs and Outputs: Connecting to the External World

**Inputs**: small molecule inducers connected to sensor promoters that produce TF concentrations within the gate library's input range. In the 2016 paper:
- pTac-LacI (IPTG input)
- pBAD-AraC (arabinose input)
- pTet-TetR (aTc input)

**Output**: typically a fluorescent reporter (GFP or RFP) driven by a circuit promoter. The output fluorescence indicates the circuit state.

**Key requirement**: input sensors must produce inducer-to-TF transfer functions with output ranges that fall within the accepted input range of the first layer of gates. If the sensor saturates the gate (produces more TF than the gate K value), or is too weak (TF well below K), the gate will not respond properly.

## What CELLO Outputs

For a given Boolean specification and gate library, CELLO produces:
1. **Gate assignment**: a map of abstract Boolean gates to biological TF gates
2. **Circuit schematic**: diagram showing TF connections and promoter assignments
3. **Score**: the worst-case ON/OFF separation across all input combinations
4. **DNA sequence**: the complete assembled DNA sequence for the entire circuit, including promoters, RBS sequences, coding sequences, terminators, and insulators, in the correct order and orientation

The DNA sequence can be submitted directly to a DNA synthesis company. This end-to-end automation — from Boolean specification to synthesis-ready sequence — is the key innovation of CELLO.

## Circuit Performance Prediction

Before outputting a design, CELLO simulates the circuit for all $2^n$ input combinations (where $n$ is the number of inputs) and predicts the output fluorescence for each. A circuit with:
- Clear separation between predicted ON and OFF outputs → high confidence it will work
- Overlapping ON and OFF predicted outputs → circuit is likely to fail or show intermediate behavior

The simulation uses the characterized Hill functions for each gate. Because parts were characterized in the same context in which they will be used, the predictions are generally reliable — the 2016 paper demonstrated successful implementation of 45 out of 60 circuits designed by CELLO, a success rate far exceeding previous manual design efforts.

## Why This Matters

CELLO was the first demonstration that complex genetic circuits (implementing arbitrary 3-input Boolean functions) could be designed by a non-expert using an automated tool, with a success rate high enough to be practically useful. The key insight is that automation of circuit design requires — and therefore motivates — the investment in quantitative part characterization. The 12-gate library required enormous effort to characterize, but that effort is amortized across every circuit ever designed with CELLO. As gate libraries grow to 50, 100, or more gates, the automated design of increasingly complex circuits becomes feasible. CELLO thus establishes the principle that automation and characterization are inseparable: better characterized parts enable better automated design, which enables more complex circuits, which creates demand for better characterization.
