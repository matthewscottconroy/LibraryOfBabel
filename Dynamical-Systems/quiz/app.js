'use strict';

// ── Data — ALL_QUESTIONS is defined by questions-bundle.js ───────────────────
const PARTS = [
  { id: 1, label: "Part I — Mathematical Foundations", chapters: [1,2,3,4,5] },
  { id: 2, label: "Part II — Dynamical Systems",       chapters: [6,7,8,9,10,11,12,13,14,15] },
  { id: 3, label: "Part III — Information Theory",     chapters: [16,17,18,19,20,21] },
  { id: 4, label: "Part IV — Bridges",                 chapters: [22,23,24,25,26] },
  { id: 5, label: "Part V — Foundations of CS & Math", chapters: [27,28,29,30,31,32] },
  { id: 6, label: "Part VI — Research Frontiers",      chapters: [33,34,35,36,37,38,39,40] },
  { id: 7, label: "Part VII — Personal Connections",   chapters: [41,42,43] },
];

const CH_TITLES = {};
ALL_QUESTIONS.forEach(q => { CH_TITLES[q.ch] = q.chTitle; });

// ── State ─────────────────────────────────────────────────────────────────────
let state = {
  mode: null,          // 'all' | 'part' | 'chapter'
  selectedPart: null,
  selectedChapter: null,
  questions: [],       // shuffled active question set
  current: 0,
  answered: false,
  chosenIdx: null,
  results: [],         // {q, chosen, correct}
};

// ── Shuffle ───────────────────────────────────────────────────────────────────
function shuffle(arr) {
  const a = [...arr];
  for (let i = a.length - 1; i > 0; i--) {
    const j = Math.floor(Math.random() * (i + 1));
    [a[i], a[j]] = [a[j], a[i]];
  }
  return a;
}

// ── DOM refs ──────────────────────────────────────────────────────────────────
const screens = {
  home:  document.getElementById('screen-home'),
  quiz:  document.getElementById('screen-quiz'),
  result: document.getElementById('screen-result'),
};

function show(name) {
  Object.values(screens).forEach(s => s.classList.remove('active'));
  screens[name].classList.add('active');
}

// ── Home screen ───────────────────────────────────────────────────────────────
const modeCards   = document.querySelectorAll('.mode-card');
const partSel     = document.getElementById('part-select');
const chapterSel  = document.getElementById('chapter-select');
const partDropdown    = document.getElementById('part-dropdown');
const chapterDropdown = document.getElementById('chapter-dropdown');
const startBtn    = document.getElementById('start-btn');

// Populate part dropdown
PARTS.forEach(p => {
  const opt = document.createElement('option');
  opt.value = p.id;
  opt.textContent = p.label;
  partDropdown.appendChild(opt);
});

// Populate chapter dropdown
for (let ch = 1; ch <= 43; ch++) {
  const opt = document.createElement('option');
  opt.value = ch;
  const title = CH_TITLES[ch] || `Chapter ${ch}`;
  opt.textContent = `Ch. ${ch} — ${title}`;
  chapterDropdown.appendChild(opt);
}

modeCards.forEach(card => {
  card.addEventListener('click', () => {
    modeCards.forEach(c => c.classList.remove('selected'));
    card.classList.add('selected');
    state.mode = card.dataset.mode;
    partSel.classList.remove('visible');
    chapterSel.classList.remove('visible');
    if (state.mode === 'part')    partSel.classList.add('visible');
    if (state.mode === 'chapter') chapterSel.classList.add('visible');
    updateStartBtn();
  });
});

partDropdown.addEventListener('change', () => {
  state.selectedPart = parseInt(partDropdown.value);
  updateStartBtn();
});

chapterDropdown.addEventListener('change', () => {
  state.selectedChapter = parseInt(chapterDropdown.value);
  updateStartBtn();
});

function updateStartBtn() {
  let ready = false;
  if (state.mode === 'all') ready = true;
  if (state.mode === 'part' && state.selectedPart) ready = true;
  if (state.mode === 'chapter' && state.selectedChapter) ready = true;
  startBtn.disabled = !ready;
}

startBtn.addEventListener('click', startQuiz);

// ── Start quiz ────────────────────────────────────────────────────────────────
function startQuiz() {
  let pool = ALL_QUESTIONS;
  if (state.mode === 'part') {
    const part = PARTS.find(p => p.id === state.selectedPart);
    pool = ALL_QUESTIONS.filter(q => part.chapters.includes(q.ch));
  }
  if (state.mode === 'chapter') {
    pool = ALL_QUESTIONS.filter(q => q.ch === state.selectedChapter);
  }
  state.questions = shuffle(pool);
  state.current = 0;
  state.results = [];
  show('quiz');
  renderQuestion();
}

// ── Quiz screen ───────────────────────────────────────────────────────────────
const progressFill  = document.getElementById('progress-fill');
const questionNum   = document.getElementById('question-num');
const questionTotal = document.getElementById('question-total');
const chapterBadge  = document.getElementById('chapter-badge');
const questionText  = document.getElementById('question-text');
const optionsList   = document.getElementById('options');
const explanationBox = document.getElementById('explanation-box');
const nextBtn       = document.getElementById('next-btn');
const skipBtn       = document.getElementById('skip-btn');

function renderQuestion() {
  const q = state.questions[state.current];
  state.answered = false;
  state.chosenIdx = null;

  const total = state.questions.length;
  const idx   = state.current;

  progressFill.style.width = `${(idx / total) * 100}%`;
  questionNum.textContent   = idx + 1;
  questionTotal.textContent = total;
  chapterBadge.textContent  = `Ch. ${q.ch} — ${q.chTitle}`;
  questionText.textContent  = q.q;
  explanationBox.classList.remove('visible');
  explanationBox.innerHTML = '';
  nextBtn.textContent = (idx === total - 1) ? 'Finish' : 'Next';
  nextBtn.style.display = 'none';
  skipBtn.style.display = 'inline-block';

  // Render options
  optionsList.innerHTML = '';
  const letters = ['A', 'B', 'C', 'D'];
  q.opts.forEach((opt, i) => {
    const div = document.createElement('div');
    div.className = 'option';
    div.innerHTML = `<span class="letter">${letters[i]}</span><span>${opt}</span>`;
    div.addEventListener('click', () => selectOption(i));
    optionsList.appendChild(div);
  });
}

function selectOption(idx) {
  if (state.answered) return;
  state.answered = true;
  state.chosenIdx = idx;

  const q = state.questions[state.current];
  const options = optionsList.querySelectorAll('.option');

  options.forEach((opt, i) => {
    opt.classList.add('disabled');
    if (i === q.ans) opt.classList.add('correct');
    if (i === idx && idx !== q.ans) opt.classList.add('wrong');
  });

  explanationBox.innerHTML = `<strong>${idx === q.ans ? '✓ Correct!' : '✗ Incorrect.'}</strong> ${q.exp}`;
  explanationBox.classList.add('visible');
  nextBtn.style.display = 'inline-block';
  skipBtn.style.display = 'none';

  state.results.push({ q, chosen: idx, correct: idx === q.ans });
}

nextBtn.addEventListener('click', () => {
  if (!state.answered) return;
  state.current++;
  if (state.current >= state.questions.length) {
    showResults();
  } else {
    renderQuestion();
  }
});

skipBtn.addEventListener('click', () => {
  // Skip without answering — record as wrong
  const q = state.questions[state.current];
  state.results.push({ q, chosen: -1, correct: false });
  state.current++;
  if (state.current >= state.questions.length) {
    showResults();
  } else {
    renderQuestion();
  }
});

document.getElementById('quit-btn').addEventListener('click', () => {
  show('home');
});

// ── Results screen ─────────────────────────────────────────────────────────────
const scoreNum    = document.getElementById('score-num');
const scoreDenom  = document.getElementById('score-denom');
const resultMsg   = document.getElementById('result-msg');
const resultList  = document.getElementById('result-list');

function showResults() {
  const correct = state.results.filter(r => r.correct).length;
  const total   = state.results.length;
  const pct     = Math.round((correct / total) * 100);

  progressFill.style.width = '100%';
  scoreNum.textContent  = `${correct}/${total}`;
  scoreDenom.textContent = `${pct}%`;

  if (pct >= 90) resultMsg.textContent = 'Excellent — research-level mastery.';
  else if (pct >= 75) resultMsg.textContent = 'Strong understanding. Review the misses.';
  else if (pct >= 50) resultMsg.textContent = 'Solid foundation — keep working through the chapters.';
  else resultMsg.textContent = 'Keep studying — the depth here rewards persistence.';

  // Build review list
  resultList.innerHTML = '';
  state.results.forEach((r, i) => {
    const letters = ['A', 'B', 'C', 'D'];
    const div = document.createElement('div');
    div.className = `result-item ${r.correct ? 'r-correct' : 'r-wrong'}`;
    const icon = r.correct ? '✓' : '✗';
    const chosenText = r.chosen === -1 ? 'skipped' : letters[r.chosen];
    const correctText = letters[r.q.ans];
    div.innerHTML = `
      <span class="ri">${icon}</span>
      <div>
        <div class="r-q"><strong>Q${i+1}.</strong> ${r.q.q.substring(0, 90)}${r.q.q.length > 90 ? '…' : ''}</div>
        <div class="r-ans">Ch. ${r.q.ch} — ${r.correct ? 'Correct' : `You: ${chosenText} | Correct: ${correctText}`}</div>
      </div>`;
    resultList.appendChild(div);
  });

  show('result');
}

document.getElementById('retry-btn').addEventListener('click', startQuiz);
document.getElementById('home-btn').addEventListener('click', () => show('home'));
