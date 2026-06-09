mod adaptive;
mod config;
mod generator;
mod models;
mod question_bank;
mod session;
mod storage;
mod ui;

use clap::Parser;

use adaptive::{filter_by_scope, Scope};
use config::{app_chapter_map, chapter_meta, phase_names};
use generator::ClaudeGenerator;
use models::UserProfile;
use question_bank::load_questions;
use session::QuizSession;
use storage::{create_profile, list_profiles, load_or_create, save_profile};

// ── CLI args ──────────────────────────────────────────────────────────────────

#[derive(Parser, Debug)]
#[command(name = "quiz_rs", about = "CSSB Adaptive Quiz")]
struct Args {
    #[arg(short = 'p', long)]
    profile: Option<String>,

    #[arg(short = 'c', long)]
    chapter: Option<u32>,

    #[arg(long)]
    phase: Option<u32>,

    #[arg(short = 't', long)]
    tag: Option<String>,

    #[arg(short = 'n', long, default_value = "10")]
    n: usize,

    #[arg(long)]
    study: Option<u32>,

    #[arg(long)]
    app: Option<u32>,

    #[arg(long)]
    cathedral: Option<String>,

    #[arg(long)]
    list_topics: bool,

    /// Export profile to JSON (stdout if FILE omitted)
    #[arg(long, value_name = "FILE", num_args = 0..=1, default_missing_value = "__stdout__")]
    export_progress: Option<String>,

    #[arg(long, value_name = "FILE")]
    import_progress: Option<String>,

    #[arg(long)]
    dry_run: bool,
}

// ── Profile selection ─────────────────────────────────────────────────────────

fn choose_or_create_profile(initial_name: Option<&str>) -> UserProfile {
    if let Some(name) = initial_name {
        return load_or_create(name);
    }
    let profiles = list_profiles();
    if profiles.is_empty() {
        println!();
        ui::section("Welcome to the CSSB Adaptive Quiz!");
        ui::print_wrap(
            "This quiz covers 29 topics across 6 tiers of the Computational \
             Systems & Synthetic Biology curriculum — from mathematical bedrock \
             through synthetic biology, computational tools, and research craft."
        );
        println!();
        let name = ui::ask("  Your name → ");
        let name = if name.is_empty() { "Student".to_string() } else { name };
        return create_profile(&name);
    }
    println!();
    ui::section("Select a profile");
    for (i, name) in profiles.iter().enumerate() {
        println!("  [{}] {}", i + 1, name);
    }
    println!("  [{}] Create new profile", profiles.len() + 1);
    ui::hr('─');
    let choice = ui::ask_int("  → ", 1, (profiles.len() + 1) as i32, Some(1)) as usize;
    if choice <= profiles.len() {
        return load_or_create(&profiles[choice - 1]);
    }
    let name = ui::ask("  New profile name → ");
    let name = if name.is_empty() { "Student".to_string() } else { name };
    create_profile(&name)
}

fn ask_n_questions(pool_size: usize, default: usize) -> usize {
    let cap = pool_size.min(50).max(1);
    println!();
    ui::hr('─');
    ui::ask_int(
        &format!("  How many questions? (1–{}, Enter for {}) → ", cap, default.min(cap)),
        1, cap as i32,
        Some(default.min(cap) as i32),
    ) as usize
}

fn run_onboarding(profile: &mut UserProfile) {
    use rand::seq::SliceRandom;

    println!();
    ui::section("Welcome Diagnostic");
    ui::print_wrap(
        "Since this is your first time, let's run a short diagnostic to calibrate \
         your starting level. One beginner question per tier — answer honestly."
    );
    println!();
    if !ui::confirm("Run the diagnostic now? (recommended)", true) {
        profile.onboarded = true;
        save_profile(profile);
        return;
    }

    let questions = load_questions();
    let pnames = phase_names();
    let mut selected: Vec<models::Question> = Vec::new();
    let mut rng = rand::thread_rng();

    let mut phases: Vec<u32> = pnames.keys().copied().collect();
    phases.sort();
    for ph in phases {
        let candidates: Vec<&models::Question> = questions.iter()
            .filter(|q| q.phase == ph && q.difficulty == "beginner")
            .collect();
        if let Some(q) = candidates.choose(&mut rng) {
            selected.push((*q).clone());
        }
    }

    if selected.is_empty() {
        ui::print_wrap("No beginner questions found — skipping diagnostic.");
        profile.onboarded = true;
        save_profile(profile);
        return;
    }

    let total = selected.len();
    for (i, q) in selected.iter().enumerate() {
        match ui::present_question(q, i + 1, total, None) {
            Ok((correct, _, confidence)) => {
                profile.record_answer(
                    q.chapter, correct, &q.question_id,
                    config::LEARNING_RATE_CORRECT, config::LEARNING_RATE_WRONG, confidence,
                );
                save_profile(profile);
            }
            Err(ui::QuizQuit) => break,
        }
    }

    println!();
    ui::print_wrap("Diagnostic complete. Your mastery levels have been initialised.");
    ui::pause();
    profile.onboarded = true;
    save_profile(profile);
}

// ── Direct-mode handlers ──────────────────────────────────────────────────────

fn list_topics() {
    let meta = chapter_meta();
    let pnames = phase_names();
    let mut chs: Vec<u32> = meta.keys().copied().collect();
    chs.sort();
    for ch in chs {
        let m = &meta[&ch];
        let pname = pnames.get(&m.phase).copied().unwrap_or("Unknown");
        println!("  {:2}  T{}  {:<38}  ({})", ch, m.phase, m.name, pname);
    }
    std::process::exit(0);
}

fn export_progress(profile: &UserProfile, dest: &str) {
    let data = serde_json::to_string_pretty(profile).unwrap_or_default();
    if dest == "__stdout__" {
        println!("{}", data);
    } else {
        std::fs::write(dest, &data).expect("write export");
        println!("  Progress exported to: {}", dest);
    }
    std::process::exit(0);
}

fn import_progress(filepath: &str, override_name: Option<&str>) {
    let data = match std::fs::read_to_string(filepath) {
        Ok(s) => s,
        Err(_) => { eprintln!("  File not found: {}", filepath); std::process::exit(1); }
    };
    let mut profile: UserProfile = match serde_json::from_str(&data) {
        Ok(p) => p,
        Err(e) => { eprintln!("  Could not parse profile: {}", e); std::process::exit(1); }
    };
    if let Some(name) = override_name { profile.name = name.to_string(); }
    save_profile(&profile);
    println!(
        "  Imported profile '{}': {} chapters tracked, {} session(s).",
        profile.name, profile.mastery.len(), profile.session_history.len()
    );
    std::process::exit(0);
}

fn dry_run(chapter: u32) {
    let gen = ClaudeGenerator::new();
    if !gen.available() {
        eprintln!("  --dry-run requires ANTHROPIC_API_KEY to be set.");
        std::process::exit(1);
    }
    let diff = adaptive::preferred_difficulty(0.5);
    let meta = chapter_meta();
    let ch_name = meta.get(&chapter).map(|m| m.name).unwrap_or("Unknown");
    println!("\n  Generating preview: Ch.{} — {}  [{}]\n", chapter, ch_name, diff);
    match gen.get_question(chapter, diff, 50, &[]) {
        None => { eprintln!("  Generation failed."); std::process::exit(1); }
        Some(q) => {
            println!("  [{}] {}", q.kind.to_uppercase(), q.text);
            match &q.answer {
                models::Answer::Index(i) => {
                    for (j, c) in q.choices.iter().enumerate() {
                        println!("    {} {}", if j == *i { "✓" } else { " " }, c);
                    }
                }
                models::Answer::Text(_) => println!("    acceptable: {:?}", q.choices),
            }
            println!("\n  Explanation: {}", q.explanation);
            println!("  Tags: {:?}", q.tags);
        }
    }
    std::process::exit(0);
}

// ── Entry point ───────────────────────────────────────────────────────────────

fn main() {
    let args = Args::parse();

    if args.list_topics { list_topics(); }

    if let Some(filepath) = &args.import_progress {
        import_progress(filepath, args.profile.as_deref());
    }

    if args.dry_run {
        dry_run(args.chapter.unwrap_or(0));
    }

    let is_direct = args.chapter.is_some()
        || args.phase.is_some()
        || args.tag.is_some()
        || args.study.is_some()
        || args.app.is_some()
        || args.cathedral.is_some()
        || args.export_progress.is_some();

    if is_direct {
        run_direct(args);
        return;
    }

    // Interactive mode
    println!();
    ui::hr('═');
    println!("\n  {}\n", ui::bold(&ui::magenta("CSSB Adaptive Quiz")));
    ui::hr('═');

    let mut profile = choose_or_create_profile(args.profile.as_deref());
    let generator = ClaudeGenerator::new();

    if !profile.onboarded {
        run_onboarding(&mut profile);
    }

    loop {
        let due_count = profile.chapters_due_for_review().len();
        let choice = ui::show_main_menu(&profile.name, generator.available(), due_count);

        match choice {
            "1" => {
                let questions = load_questions();
                let n = ask_n_questions(questions.len(), 10);
                let sess = QuizSession::new(profile.clone(), Scope::adaptive(), n, Some(ClaudeGenerator::new()));
                let (_, updated) = sess.run();
                profile = updated;
            }
            "2" => {
                let questions = load_questions();
                let scope  = ui::show_scope_menu();
                let scoped = filter_by_scope(&questions, &scope, Some(&profile));
                let n = ask_n_questions(scoped.len().max(1), 10);
                let sess = QuizSession::new(profile.clone(), scope, n, Some(ClaudeGenerator::new()));
                let (_, updated) = sess.run();
                profile = updated;
            }
            "3" => {
                let ch = ui::show_chapter_menu();
                let wants_quiz = ui::show_study_mode(ch);
                if wants_quiz {
                    let questions = load_questions();
                    let scoped = filter_by_scope(&questions, &Scope::chapter(ch), Some(&profile));
                    if !scoped.is_empty() {
                        let n = ask_n_questions(scoped.len(), 5);
                        let sess = QuizSession::new(profile.clone(), Scope::chapter(ch), n, Some(ClaudeGenerator::new()));
                        let (_, updated) = sess.run();
                        profile = updated;
                    }
                }
            }
            "4" => ui::show_dashboard(&profile),
            "5" => {
                profile = choose_or_create_profile(None);
                if !profile.onboarded { run_onboarding(&mut profile); }
            }
            "6" => ui::show_cathedral_menu(&profile),
            _ => {
                println!("\n  {}\n", ui::dim("See you next time."));
                std::process::exit(0);
            }
        }
    }
}

fn run_direct(args: Args) {
    let mut profile = choose_or_create_profile(args.profile.as_deref());

    if let Some(dest) = &args.export_progress {
        export_progress(&profile, dest);
    }

    if let Some(cid) = &args.cathedral {
        ui::show_cathedral_readiness(&profile, cid);
        return;
    }

    let questions = load_questions();

    let scope = if let Some(app_num) = args.app {
        let map = app_chapter_map();
        let ch = match map.get(&app_num) {
            Some(&ch) => ch,
            None => {
                let mut valid: Vec<u32> = map.keys().copied().collect();
                valid.sort();
                eprintln!("  App {} not in map. Valid: {}–{}", app_num, valid[0], valid[valid.len()-1]);
                std::process::exit(1);
            }
        };
        let meta = chapter_meta();
        let ch_name = meta.get(&ch).map(|m| m.name).unwrap_or("Unknown");
        println!("\n  App {:02} → {} (Chapter {})\n", app_num, ui::cyan(ch_name), ch);
        Scope::chapter(ch)
    } else if let Some(ch) = args.study {
        ui::show_study_mode(ch);
        Scope::chapter(ch)
    } else if let Some(ch) = args.chapter {
        Scope::chapter(ch)
    } else if let Some(ph) = args.phase {
        Scope::phase(ph)
    } else if let Some(tag) = args.tag {
        Scope::tag(tag)
    } else {
        Scope::adaptive()
    };

    let pool = filter_by_scope(&questions, &scope, Some(&profile));
    let n = args.n.min(pool.len().max(1));
    let sess = QuizSession::new(profile, scope, n, Some(ClaudeGenerator::new()));
    sess.run();
}
