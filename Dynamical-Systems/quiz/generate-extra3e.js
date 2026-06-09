#!/usr/bin/env node
'use strict';
const fs = require('fs');
const path = require('path');

const BASE = path.join(__dirname, 'questions');
const chPath = path.join(BASE, 'part-vii-connections', 'ch41-collatz-dynamics');
const existing = fs.readdirSync(chPath).filter(f=>f.endsWith('.json')).sort();
const chTitle = JSON.parse(fs.readFileSync(path.join(chPath,existing[0]),'utf8')).chTitle;

const questions = [
  {
    q: "The Collatz conjecture can be reformulated using the concept of a 'stopping time' σ(n)=min{k>0: T^k(n)<n}. For n=27, the stopping time is exceptionally large. Approximately how many steps does the orbit of 27 take before falling below 27?",
    opts:["11","59","111","300"],
    ans:1,
    exp:"The orbit of 27 under the Collatz map reaches a peak of 9232 before eventually falling below 27. The stopping time σ(27)=59 steps — it takes 59 iterations before the sequence first falls below 27. The total trajectory length (to reach 1) is 111 steps."
  },
  {
    q: "The Collatz function viewed as a dynamical system on ℝ (via continuous interpolation) can be extended to a smooth map. The most natural smooth extension to ℝ satisfying T(n)=n/2 (n even) and T(n)=(3n+1)/2 (n odd) uses the formula:",
    opts:["T(x)=x/2·cos²(πx)+((3x+1)/2)·sin²(πx)","T(x)=(1/4)(7x+2−(5x+2)cos(πx))","T(x)=x·(3/2)^{sin²(πx/2)}","T(x)=x/2+sin²(πx/2)·(x+1/2)"],
    ans:1,
    exp:"The Bernstein-Lagarias smooth extension: T(x)=(1/4)(7x+2−(5x+2)cos(πx)). At even integers 2k: T(2k)=(1/4)(14k+2−(10k+2)·1)=k=n/2. At odd integers 2k+1: T(2k+1)=(1/4)(7(2k+1)+2−(5(2k+1)+2)(−1))=(6k+4)/2=3k+2=(3(2k+1)+1)/2. This gives a real-analytic extension."
  },
  {
    q: "The Collatz conjecture is related to the study of aperiodic sequences. If n is a positive integer whose binary representation has a specific pattern, the Collatz orbit structure can be partially predicted. Specifically, numbers of the form n=2^k−1 (all 1s in binary) take approximately how long to reach 1?",
    opts:["O(k)","O(k²)","O(k log k)","O(2^k/k)"],
    ans:2,
    exp:"Numbers 2^k−1 require approximately O(k log k) steps in the Collatz sequence. The 3x+1 steps multiply by ~3/2 while even steps divide by 2; the alternation depends on the bit pattern, and for 2^k−1 (all 1s), the orbit has O(k log k) length (heuristic analysis from the probabilistic model)."
  },
  {
    q: "The Collatz map restricted to the positive integers forms a directed graph. The conjecture states this graph is a tree rooted at 1 (with all edges directed toward 1). The maximum out-degree of a vertex n in the inverse Collatz graph (edges pointing away from 1) is:",
    opts:["1","2","3","Unbounded"],
    ans:1,
    exp:"In the inverse Collatz graph, each n has at most 2 predecessors: 2n (always a predecessor via the even step) and (n−1)/3 (a predecessor via the odd step iff n≡1 mod 3 and (n−1)/3 is a positive odd integer). So the maximum in-degree in the inverse map is 2, making the graph a binary tree (if the conjecture holds)."
  }
];

let idx = existing.length + 1;
for(const {q,opts,ans,exp} of questions){
  const num = String(idx).padStart(2,'0');
  const obj = {ch:41, part:7, chTitle, q, opts, ans, exp};
  const fp = path.join(chPath, `q${num}.json`);
  fs.writeFileSync(fp, JSON.stringify(obj, null, 2));
  console.log(`wrote ${path.relative(path.join(__dirname,'questions'), fp)}`);
  idx++;
}
console.log(`\nDone. ${questions.length} questions written.`);
