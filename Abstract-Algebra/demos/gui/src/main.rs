/// Abstract Algebra GUI — egui/eframe native window.
/// Provides chapter navigation, terminal output panel, SVG preview, and TOML state editor.

use eframe::egui;
use common::*;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Abstract Algebra Explorer")
            .with_inner_size([1200.0, 800.0]),
        ..Default::default()
    };
    if let Err(e) = eframe::run_native(
        "Abstract Algebra Explorer",
        options,
        Box::new(|_cc| Ok(Box::new(AlgebraApp::new()))),
    ) {
        eprintln!("GUI error: {e}");
    }
}

struct AlgebraApp {
    selected_chapter: usize,
    command_input:    String,
    format_choice:    FormatChoice,
    output_text:      Arc<Mutex<String>>,
    svg_source:       Arc<Mutex<String>>,
    toml_editor:      String,
    load_path:        String,
    save_path:        String,
    status_msg:       String,
    panel_mode:       PanelMode,
}

#[derive(PartialEq, Clone, Copy)]
enum FormatChoice { Text, Svg, Dot, Tex, Ascii }

#[derive(PartialEq, Clone, Copy)]
enum PanelMode { Output, Svg, Toml, Help }

static CHAPTERS: &[(&str, &str)] = &[
    ("ch01-logic",                "01 Logic & Proofs"),
    ("ch02-sets",                 "02 Sets & Functions"),
    ("ch03-cardinality",          "03 Cardinality"),
    ("ch04-vector-spaces",        "04 Vector Spaces"),
    ("ch05-bases-dimension",      "05 Bases & Dimension"),
    ("ch06-linear-maps",          "06 Linear Maps"),
    ("ch07-matrices",             "07 Matrices"),
    ("ch08-determinants",         "08 Determinants"),
    ("ch09-eigentheory",          "09 Eigentheory"),
    ("ch10-canonical-forms",      "10 Canonical Forms"),
    ("ch11-inner-products",       "11 Inner Products"),
    ("ch12-multilinear",          "12 Multilinear Algebra"),
    ("ch13-groups",               "13 Groups"),
    ("ch14-cosets",               "14 Cosets & Lagrange"),
    ("ch15-homomorphisms",        "15 Homomorphisms"),
    ("ch16-group-actions",        "16 Group Actions"),
    ("ch17-sylow",                "17 Sylow Theorems"),
    ("ch18-group-structure",      "18 Group Structure"),
    ("ch19-abelian-groups",       "19 Abelian Groups"),
    ("ch20-rings",                "20 Rings"),
    ("ch21-ideals",               "21 Ideals"),
    ("ch22-divisibility",         "22 Divisibility"),
    ("ch23-polynomials",          "23 Polynomials"),
    ("ch24-commutative-algebra",  "24 Commutative Algebra"),
    ("ch25-modules",              "25 Modules"),
    ("ch26-projective-injective", "26 Projective & Injective"),
    ("ch27-structure-theorem",    "27 Structure Theorem"),
    ("ch28-tensor-products",      "28 Tensor Products"),
    ("ch29-field-extensions",     "29 Field Extensions"),
    ("ch30-normal-separable",     "30 Normal & Separable"),
    ("ch31-galois-theory",        "31 Galois Theory"),
    ("ch32-galois-applications",  "32 Galois Applications"),
    ("ch33-categories",           "33 Categories"),
    ("ch34-yoneda",               "34 Yoneda Lemma"),
    ("ch35-adjoints",             "35 Adjoints"),
    ("ch36-limits-colimits",      "36 Limits & Colimits"),
    ("ch37-abelian-categories",   "37 Abelian Categories"),
    ("ch38-chain-complexes",      "38 Chain Complexes"),
    ("ch39-resolutions",          "39 Resolutions"),
    ("ch40-ext-tor",              "40 Ext & Tor"),
    ("ch41-spectral-sequences",   "41 Spectral Sequences"),
    ("ch42-representations",      "42 Representations"),
    ("ch43-complete-reducibility","43 Complete Reducibility"),
    ("ch44-character-theory",     "44 Character Theory"),
    ("ch45-induced-representations","45 Induced Representations"),
    ("ch46-lie-groups",           "46 Lie Groups"),
    ("ch47-lie-algebras",         "47 Lie Algebras"),
    ("ch48-solvable-semisimple",  "48 Solvable & Semisimple"),
    ("ch49-root-systems",         "49 Root Systems"),
    ("ch50-highest-weight",       "50 Highest Weight"),
    ("ch51-modular",              "51 Modular Representation"),
    ("ch52-geometric",            "52 Geometric Representation"),
    ("ch53-quantum-groups",       "53 Quantum Groups"),
    ("ch54-langlands",            "54 Langlands Program"),
];

impl AlgebraApp {
    fn new() -> Self {
        Self {
            selected_chapter: 0,
            command_input:    "demo".to_string(),
            format_choice:    FormatChoice::Text,
            output_text:      Arc::new(Mutex::new(String::new())),
            svg_source:       Arc::new(Mutex::new(String::new())),
            toml_editor:      String::new(),
            load_path:        String::new(),
            save_path:        String::new(),
            status_msg:       String::new(),
            panel_mode:       PanelMode::Output,
        }
    }

    fn run_command(&mut self) {
        let ch = CHAPTERS[self.selected_chapter].0;
        let cmd = self.command_input.clone();
        let fmt = match self.format_choice {
            FormatChoice::Text  => "text",
            FormatChoice::Svg   => "svg",
            FormatChoice::Dot   => "dot",
            FormatChoice::Tex   => "tex",
            FormatChoice::Ascii => "ascii",
        };

        let workspace = std::env::var("ABSTRACT_ALGEBRA_ROOT")
            .unwrap_or_else(|_| {
                // Fall back to the directory containing this binary's Cargo workspace
                std::env::var("CARGO_MANIFEST_DIR")
                    .map(|d| {
                        std::path::Path::new(&d)
                            .parent().unwrap()  // demos/gui -> demos
                            .parent().unwrap()  // demos -> workspace root
                            .to_string_lossy()
                            .into_owned()
                    })
                    .unwrap_or_else(|_| ".".to_string())
            });
        let manifest = format!("{workspace}/demos/{ch}/Cargo.toml");

        let mut cargo_args = vec![
            "run", "--manifest-path", &manifest, "--",
            "--run", &cmd, "--format", fmt,
        ];

        let load_path = self.load_path.clone();
        let save_path = self.save_path.clone();
        if !load_path.is_empty() {
            cargo_args.extend(&["--load", &load_path]);
        }
        if !save_path.is_empty() {
            cargo_args.extend(&["--save", &save_path]);
        }

        let output = Command::new("cargo")
            .args(&cargo_args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output();

        match output {
            Ok(o) => {
                let stdout = String::from_utf8_lossy(&o.stdout).into_owned();
                let stderr = String::from_utf8_lossy(&o.stderr).into_owned();
                let content = if stdout.is_empty() && !stderr.is_empty() {
                    stderr
                } else {
                    stdout
                };

                if fmt == "svg" {
                    *self.svg_source.lock().unwrap() = content.clone();
                    self.panel_mode = PanelMode::Svg;
                } else {
                    *self.output_text.lock().unwrap() = content;
                    self.panel_mode = PanelMode::Output;
                }
                self.status_msg = format!("ran {cmd} on {ch}");
            }
            Err(e) => {
                self.status_msg = format!("error: {e}");
            }
        }
    }

    fn load_toml(&mut self) {
        if self.load_path.is_empty() {
            self.status_msg = "enter a path in Load Path".to_string();
            return;
        }
        match std::fs::read_to_string(&self.load_path) {
            Ok(s) => {
                self.toml_editor = s;
                self.panel_mode = PanelMode::Toml;
                self.status_msg = format!("loaded {}", self.load_path);
            }
            Err(e) => { self.status_msg = format!("load error: {e}"); }
        }
    }

    fn save_toml(&mut self) {
        if self.save_path.is_empty() {
            self.status_msg = "enter a path in Save Path".to_string();
            return;
        }
        match std::fs::write(&self.save_path, &self.toml_editor) {
            Ok(()) => { self.status_msg = format!("saved to {}", self.save_path); }
            Err(e) => { self.status_msg = format!("save error: {e}"); }
        }
    }
}

impl eframe::App for AlgebraApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Left sidebar — chapter list
        egui::SidePanel::left("chapters")
            .min_width(220.0)
            .show(ctx, |ui| {
                ui.heading("Chapters");
                ui.separator();
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for (i, (_bin, label)) in CHAPTERS.iter().enumerate() {
                        let sel = self.selected_chapter == i;
                        if ui.selectable_label(sel, *label).clicked() {
                            self.selected_chapter = i;
                            self.status_msg = format!("selected {label}");
                        }
                    }
                });
            });

        // Bottom status bar
        egui::TopBottomPanel::bottom("status")
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(&self.status_msg);
                });
            });

        // Main panel
        egui::CentralPanel::default().show(ctx, |ui| {
            let ch_name = CHAPTERS[self.selected_chapter].1;
            ui.heading(ch_name);
            ui.separator();

            // Controls row
            ui.horizontal(|ui| {
                ui.label("Command:");
                ui.text_edit_singleline(&mut self.command_input);

                ui.label("Format:");
                egui::ComboBox::new("fmt", "")
                    .selected_text(match self.format_choice {
                        FormatChoice::Text  => "text",
                        FormatChoice::Svg   => "svg",
                        FormatChoice::Dot   => "dot",
                        FormatChoice::Tex   => "tex",
                        FormatChoice::Ascii => "ascii",
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.format_choice, FormatChoice::Text,  "text");
                        ui.selectable_value(&mut self.format_choice, FormatChoice::Svg,   "svg");
                        ui.selectable_value(&mut self.format_choice, FormatChoice::Dot,   "dot");
                        ui.selectable_value(&mut self.format_choice, FormatChoice::Tex,   "tex");
                        ui.selectable_value(&mut self.format_choice, FormatChoice::Ascii, "ascii");
                    });

                if ui.button("Run").clicked() {
                    self.run_command();
                }
            });

            ui.horizontal(|ui| {
                ui.label("Load path:");
                ui.text_edit_singleline(&mut self.load_path);
                if ui.button("Load TOML").clicked() { self.load_toml(); }

                ui.label("Save path:");
                ui.text_edit_singleline(&mut self.save_path);
                if ui.button("Save TOML").clicked() { self.save_toml(); }
            });

            ui.separator();

            // Tab row
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.panel_mode, PanelMode::Output, "Output");
                ui.selectable_value(&mut self.panel_mode, PanelMode::Svg,    "SVG Source");
                ui.selectable_value(&mut self.panel_mode, PanelMode::Toml,   "TOML Editor");
                ui.selectable_value(&mut self.panel_mode, PanelMode::Help,   "Help");
            });

            ui.separator();

            // Panel content
            match self.panel_mode {
                PanelMode::Output => {
                    egui::ScrollArea::both().show(ui, |ui| {
                        let text = self.output_text.lock().unwrap().clone();
                        ui.add(
                            egui::TextEdit::multiline(&mut text.as_str())
                                .font(egui::TextStyle::Monospace)
                                .desired_width(f32::INFINITY)
                        );
                    });
                }
                PanelMode::Svg => {
                    egui::ScrollArea::both().show(ui, |ui| {
                        let src = self.svg_source.lock().unwrap().clone();
                        ui.label(egui::RichText::new("SVG source (copy to browser or save as .svg):").small());
                        ui.add(
                            egui::TextEdit::multiline(&mut src.as_str())
                                .font(egui::TextStyle::Monospace)
                                .desired_width(f32::INFINITY)
                        );
                        if ui.button("Save SVG...").clicked() {
                            if !self.save_path.is_empty() {
                                let _ = std::fs::write(&self.save_path, &src);
                                self.status_msg = format!("saved svg to {}", self.save_path);
                            } else {
                                self.status_msg = "set Save path first".to_string();
                            }
                        }
                    });
                }
                PanelMode::Toml => {
                    egui::ScrollArea::both().show(ui, |ui| {
                        ui.add(
                            egui::TextEdit::multiline(&mut self.toml_editor)
                                .font(egui::TextStyle::Monospace)
                                .desired_width(f32::INFINITY)
                        );
                    });
                }
                PanelMode::Help => {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.label(egui::RichText::new("Abstract Algebra Explorer").heading());
                        ui.separator();
                        ui.label("WORKFLOW:");
                        ui.label("1. Select a chapter from the left sidebar.");
                        ui.label("2. Enter a command (e.g. 'demo', 'cayley', 'galois') and choose output format.");
                        ui.label("3. Click Run — output appears in the Output tab.");
                        ui.label("4. For SVG output, the SVG Source tab shows the markup (save or open in browser).");
                        ui.label("5. Use Load/Save TOML to persist and edit mathematical state between runs.");
                        ui.separator();
                        ui.label("COMMON COMMANDS:");
                        ui.label("  demo     — run the chapter showcase");
                        ui.label("  help     — list all commands for the chapter");
                        ui.label("  cayley   — Cayley table (group chapters)");
                        ui.label("  galois   — Galois correspondence diagram");
                        ui.label("  hasse    — Hasse diagram of a poset");
                        ui.label("  matrix   — matrix display");
                        ui.label("  roots    — root system diagram");
                        ui.separator();
                        ui.label("FORMATS:");
                        ui.label("  text  — plain text (default)");
                        ui.label("  svg   — SVG markup");
                        ui.label("  dot   — Graphviz DOT (pipe to dot -Tpdf)");
                        ui.label("  tex   — LaTeX/TikZ snippet");
                        ui.label("  ascii — ASCII art diagram");
                    });
                }
            }
        });
    }
}
