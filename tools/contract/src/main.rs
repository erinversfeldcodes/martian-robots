//! Renders the contract's prose from the contract's data, and validates the
//! data on its own.
//!
//! `contract/` is the contract: the version and the limits, every ruling, the
//! grammar, and the narrative the facts are substituted into. `CONTRACT.md` is
//! a rendering of those files for a reader who wants one linear pass.
//!
//! `check` renders into memory and fails on anything that would make the contract
//! unreadable or incoherent: malformed data, a ruled question with no ruling,
//! an open question with no note, a hole in the template that nothing fills.
//!
//! Usage:
//!   contract render   write CONTRACT.md (a build output, gitignored)
//!   contract check    render into memory and fail on any problem

use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

#[derive(serde::Deserialize)]
struct Limits {
    version: String,
    limits: Bounds,
}

#[derive(serde::Deserialize)]
struct Bounds {
    max_coordinate: u32,
    max_instructions: u32,
}

#[derive(serde::Deserialize)]
struct Rulings {
    ruling: Vec<Ruling>,
}

#[derive(serde::Deserialize)]
struct Ruling {
    id: String,
    status: Status,
    question: String,
    ruling: Option<String>,
    rationale: Option<String>,
    note: Option<String>,
}

#[derive(serde::Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
enum Status {
    Ruled,
    Open,
}

fn main() -> ExitCode {
    let root = match repository_root() {
        Ok(root) => root,
        Err(message) => return fail(&message),
    };

    let rendered = match render(&root) {
        Ok(rendered) => rendered,
        Err(message) => return fail(&message),
    };

    match std::env::args().nth(1).as_deref() {
        Some("render") => {
            let document = root.join("CONTRACT.md");
            match std::fs::write(&document, &rendered.document) {
                Ok(()) => {
                    println!("contract: rendered {}", document.display());
                    ExitCode::SUCCESS
                }
                Err(error) => fail(&format!("{}: {error}", document.display())),
            }
        }
        Some("check") => {
            println!(
                "contract: {} renders cleanly — {} ruled, {} open",
                rendered.version, rendered.ruled, rendered.open
            );
            ExitCode::SUCCESS
        }
        _ => fail("usage: contract <render|check>"),
    }
}

fn fail(message: &str) -> ExitCode {
    eprintln!("contract: {message}");
    ExitCode::FAILURE
}

fn repository_root() -> Result<PathBuf, String> {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest
        .ancestors()
        .nth(2)
        .map(Path::to_path_buf)
        .ok_or_else(|| format!("no repository root above {}", manifest.display()))
}

struct Rendered {
    document: String,
    version: String,
    ruled: usize,
    open: usize,
}

fn render(root: &Path) -> Result<Rendered, String> {
    let read = |relative: &str| -> Result<String, String> {
        let path = root.join(relative);
        std::fs::read_to_string(&path).map_err(|error| format!("{}: {error}", path.display()))
    };

    let limits: Limits = toml::from_str(&read("contract/limits.toml")?)
        .map_err(|error| format!("contract/limits.toml: {error}"))?;
    let rulings: Rulings = toml::from_str(&read("contract/rulings.toml")?)
        .map_err(|error| format!("contract/rulings.toml: {error}"))?;
    let grammar = read("contract/grammar.ebnf")?;
    let template = read("contract/template.md")?;

    let body = template
        .split_once("-->\n")
        .map_or(template.as_str(), |(_, rest)| rest);

    let rendered = body
        .replace("{{version}}", &limits.version)
        .replace(
            "{{max_coordinate}}",
            &limits.limits.max_coordinate.to_string(),
        )
        .replace(
            "{{max_instructions}}",
            &limits.limits.max_instructions.to_string(),
        )
        .replace("{{grammar}}", &grammar_block(&grammar))
        .replace("{{rulings}}", &rulings_table(&rulings)?)
        .replace("{{open_questions}}", &open_questions(&rulings)?);

    if let Some(hole) = rendered.find("{{") {
        let tail = &rendered[hole..];
        let name = tail.find("}}").map_or(tail, |end| &tail[..=end + 1]);
        return Err(format!("template has a hole nothing filled: {name}"));
    }

    Ok(Rendered {
        document: format!(
            "<!-- Rendered from contract/ by `cargo run -p contract -- render`. Not tracked: contract/ is the contract. -->\n\n{rendered}"
        ),
        version: limits.version,
        ruled: rulings
            .ruling
            .iter()
            .filter(|r| r.status == Status::Ruled)
            .count(),
        open: rulings
            .ruling
            .iter()
            .filter(|r| r.status == Status::Open)
            .count(),
    })
}

fn grammar_block(grammar: &str) -> String {
    let productions = grammar
        .split_once("*)\n")
        .map_or(grammar, |(_, rest)| rest)
        .trim();
    format!("```ebnf\n{productions}\n```")
}

fn rulings_table(rulings: &Rulings) -> Result<String, String> {
    let mut table =
        String::from("| ID | Question the brief leaves open | Ruling |\n|---|---|---|\n");
    for ruling in rulings.ruling.iter().filter(|r| r.status == Status::Ruled) {
        let decision = ruling
            .ruling
            .as_deref()
            .ok_or_else(|| format!("{} is ruled but has no ruling", ruling.id))?;
        let rationale = ruling
            .rationale
            .as_deref()
            .map_or_else(String::new, |rationale| format!(" {rationale}"));
        let _ = writeln!(
            table,
            "| {} | {} | {decision}{rationale} |",
            ruling.id, ruling.question
        );
    }
    Ok(table.trim_end().to_string())
}

fn open_questions(rulings: &Rulings) -> Result<String, String> {
    let mut list = String::new();
    for ruling in rulings.ruling.iter().filter(|r| r.status == Status::Open) {
        let note = ruling
            .note
            .as_deref()
            .ok_or_else(|| format!("{} is open but has no note", ruling.id))?;
        let _ = writeln!(list, "- **{}** — {} {note}", ruling.id, ruling.question);
    }
    Ok(list.trim_end().to_string())
}
