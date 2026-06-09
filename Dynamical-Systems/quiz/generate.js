#!/usr/bin/env node
// One-time migration: reads questions-a.js / questions-b.js and writes
// each question to its own JSON file under questions/<part>/<chapter>/.
'use strict';
const fs   = require('fs');
const path = require('path');

// ── Load the existing question arrays via Function constructor ────────────────
function loadArray(filename, varName) {
  const src = fs.readFileSync(path.join(__dirname, filename), 'utf8');
  return new Function(src + `\nreturn ${varName};`)();
}

const all = [
  ...loadArray('questions-a.js', 'QUESTIONS_A'),
  ...loadArray('questions-b.js', 'QUESTIONS_B'),
];

// ── Directory name maps ───────────────────────────────────────────────────────
const PART_DIRS = {
  1: 'part-i-foundations',
  2: 'part-ii-dynamical-systems',
  3: 'part-iii-information-theory',
  4: 'part-iv-bridges',
  5: 'part-v-foundations',
  6: 'part-vi-frontiers',
  7: 'part-vii-connections',
};

const CH_DIRS = {
   1: 'ch01-real-analysis',
   2: 'ch02-measure-theory',
   3: 'ch03-topology',
   4: 'ch04-ordinary-differential-equations',
   5: 'ch05-linear-algebra',
   6: 'ch06-topological-dynamics',
   7: 'ch07-ergodic-theory',
   8: 'ch08-stability-theory',
   9: 'ch09-hyperbolic-dynamics',
  10: 'ch10-bifurcation-theory',
  11: 'ch11-chaos-theory',
  12: 'ch12-symbolic-dynamics',
  13: 'ch13-complex-dynamics',
  14: 'ch14-hamiltonian-systems',
  15: 'ch15-infinite-dimensional-dynamics',
  16: 'ch16-classical-information-theory',
  17: 'ch17-entropy-generalizations',
  18: 'ch18-algorithmic-information-theory',
  19: 'ch19-network-information-theory',
  20: 'ch20-information-geometry',
  21: 'ch21-quantum-information-theory',
  22: 'ch22-entropy-in-dynamical-systems',
  23: 'ch23-ergodic-information-theory',
  24: 'ch24-symbolic-dynamics-and-information',
  25: 'ch25-chaos-randomness-computation',
  26: 'ch26-information-methods-in-cs',
  27: 'ch27-computability-and-dynamics',
  28: 'ch28-category-theory-and-dynamics',
  29: 'ch29-thermodynamics-and-information',
  30: 'ch30-optimal-transport',
  31: 'ch31-ergodic-number-theory',
  32: 'ch32-descriptive-set-theory',
  33: 'ch33-orbit-equivalence',
  34: 'ch34-sofic-entropy',
  35: 'ch35-isomorphism-problem',
  36: 'ch36-zimmer-program',
  37: 'ch37-complex-dynamics-frontiers',
  38: 'ch38-quantum-information-complexity',
  39: 'ch39-one-shot-information-theory',
  40: 'ch40-circuit-complexity-and-it',
  41: 'ch41-collatz-dynamics',
  42: 'ch42-quantum-computing-connections',
  43: 'ch43-hott-connections',
};

// ── Write files ───────────────────────────────────────────────────────────────
const counters = {};

for (const q of all) {
  const n = (counters[q.ch] = (counters[q.ch] || 0) + 1);
  const dir = path.join(
    __dirname, 'questions',
    PART_DIRS[q.part],
    CH_DIRS[q.ch],
  );
  fs.mkdirSync(dir, { recursive: true });
  const file = path.join(dir, `q${String(n).padStart(2, '0')}.json`);
  fs.writeFileSync(file, JSON.stringify(q, null, 2) + '\n');
  console.log('wrote', path.relative(__dirname, file));
}

console.log(`\nDone. ${all.length} questions written.`);
console.log('Run `node build.js` to regenerate questions-bundle.js.');
