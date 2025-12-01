use anyhow::{Context, Result};
use aoc_template::Solution;
use chrono::{Datelike, FixedOffset, Local};
use clap::{Parser, Subcommand};
use dotenv::dotenv;
use reqwest::blocking::Client;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new day
    Init { day: Option<u32> },
    /// Run a solution
    Run { day: Option<u32> },
}

fn main() -> Result<()> {
    dotenv().ok();
    let cli = Cli::parse();
    let year = env::var("AOC_YEAR").context("AOC_YEAR not set in .env")?;

    match cli.command {
        Commands::Init { day } => {
            let day = get_day(day)?;
            init_day(&year, day)?;
        }
        Commands::Run { day } => {
            let day = get_day(day)?;
            run_day(&year, day)?;
        }
    }
    Ok(())
}

fn init_day(year: &str, day: u32) -> Result<()> {
    let cwd = env::current_dir()?;
    let folder_format = env::var("AOC_FOLDER_FORMAT").unwrap_or_else(|_| "aoc{year}".to_string());
    let folder_name = folder_format.replace("{year}", year);
    let year_folder = cwd.join("src").join(&folder_name);

    // 1. Ensure year module exists
    if !year_folder.exists() {
        fs::create_dir_all(&year_folder)?;
        fs::write(year_folder.join("mod.rs"), "")?;

        // Add to lib.rs
        let lib_rs = cwd.join("src").join("lib.rs");
        let mut content = fs::read_to_string(&lib_rs)?;
        if !content.contains(&format!("pub mod {};", folder_name)) {
            content = format!("pub mod {folder_name};\n{content}");
            fs::write(lib_rs, content)?;
        }
    }

    // 2. Create day file
    let file_format = env::var("AOC_FILE_FORMAT").unwrap_or_else(|_| "day{day}".to_string());
    let file_name = format!("{}.rs", file_format.replace("{day}", &day.to_string()));
    let file_path = year_folder.join(&file_name);

    if !file_path.exists() {
        let template_path = cwd.join("templates").join("template.rs.example");
        let template = fs::read_to_string(template_path).context("Template file not found")?;
        let filled = template.replace("{day}", &day.to_string());
        fs::write(&file_path, filled)?;
        println!("Created {}", file_path.display());

        // Add to mod.rs
        let mod_rs = year_folder.join("mod.rs");
        let mut mod_content = fs::read_to_string(&mod_rs)?;
        let mod_decl = format!("pub mod {};", file_name.replace(".rs", ""));
        if !mod_content.contains(&mod_decl) {
            mod_content.push_str(&format!("\n{}\n", mod_decl));
            fs::write(mod_rs, mod_content)?;
        }
    } else {
        println!("File {} already exists", file_path.display());
    }

    // 3. Download input
    let _ = get_input(year, day)?;

    Ok(())
}

fn run_day(year: &str, day: u32) -> Result<()> {
    let input = get_input(year, day)?;

    let solution = inventory::iter::<Solution>().find(|s| s.day == day);
    if let Some(solution) = solution {
        let (input, generator_timings) = timings(solution.generator, &input);
        println!("Running Day {}", day);
        println!(
            "Generator          @ {}",
            format_duration(generator_timings)
        );

        // Run Part 1
        (solution.part1_checks)();
        let (part1_result, part1_timings) = timings(solution.part1, input.as_ref());
        println!(
            "Part 1: {part1_result:10} @ {}",
            format_duration(part1_timings)
        );

        // Run Part 2
        (solution.part2_checks)();
        let (part2_result, part2_timings) = timings(solution.part2, input.as_ref());
        println!(
            "Part 2: {part2_result:10} @ {}",
            format_duration(part2_timings)
        );
    } else {
        println!(
            "Solution for day {} not found. Did you run `cargo run -- init {day}` and recompile?",
            day
        );
    }
    Ok(())
}

fn timings<T, R>(f: fn(T) -> R, input: T) -> (R, std::time::Duration) {
    let start = std::time::Instant::now();
    let result = f(input);
    let end = std::time::Instant::now();
    (result, end - start)
}

fn format_duration(d: std::time::Duration) -> String {
    let secs = d.as_secs();
    let ms = d.subsec_millis();
    let ns = d.subsec_micros() % 1000;

    if secs > 0 {
        format!("{secs}s {ms}ms {ns}ns")
    } else if ms > 0 {
        format!("{ms}ms {ns}ns")
    } else {
        format!("{ns}ns")
    }
}

pub fn get_day(day: Option<u32>) -> Result<u32> {
    if let Some(d) = day {
        if !(1..=25).contains(&d) {
            anyhow::bail!("Day must be between 1 and 25");
        }
        return Ok(d);
    }

    let now = Local::now().with_timezone(&FixedOffset::west_opt(5 * 3600).unwrap());
    if now.month() != 12 {
        anyhow::bail!("It is not December!");
    }
    let d = now.day();
    if d > 25 {
        anyhow::bail!("Advent is over!");
    }
    Ok(d)
}

pub fn get_input_path(year: &str, day: u32) -> PathBuf {
    let cwd = env::current_dir().unwrap();
    let folder_format = env::var("AOC_FOLDER_FORMAT").unwrap_or_else(|_| "aoc{year}".to_string());
    let folder_name = folder_format.replace("{year}", year);
    cwd.join("input")
        .join(folder_name)
        .join(format!("day{}.txt", day))
}

pub fn retrieve_input(year: &str, day: u32, path: &Path) -> Result<()> {
    if path.exists() {
        return Ok(());
    }

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let session = env::var("AOC_SESSION_TOKEN").context("AOC_SESSION_TOKEN not found in .env")?;
    if session == "your-session-token-here" {
        anyhow::bail!("Please set your valid session token in .env");
    }

    let client = Client::new();
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let resp = client
        .get(&url)
        .header("Cookie", format!("session={}", session))
        .send()?;

    if !resp.status().is_success() {
        anyhow::bail!("Failed to download input: {}", resp.status());
    }

    let text = resp.text()?;
    fs::write(path, text.trim())?;
    println!("Downloaded input for day {}", day);
    Ok(())
}

pub fn get_input(year: &str, day: u32) -> Result<String> {
    let path = get_input_path(year, day);
    retrieve_input(year, day, &path)?;
    let content = fs::read_to_string(path)?;
    Ok(content)
}
