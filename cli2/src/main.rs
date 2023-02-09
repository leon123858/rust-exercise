use anyhow::{Ok, Result};
use clap::Parser;
use indicatif::ProgressBar;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn find_matches(
    content: &str,
    pattern: &str,
    pb: &ProgressBar,
    mut writer: impl std::io::Write,
) -> Result<()> {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{line}")?;
        }
        pb.inc(1);
    }
    Ok(())
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)?;
    let mut cnt = 0;
    for _ in content.lines() {
        cnt += 1;
    }
    let pb = indicatif::ProgressBar::new(cnt);
    find_matches(&content, &args.pattern, &pb, std::io::stdout())?;
    pb.finish_with_message("done");
    println!("End Search!");
    Ok(())
}

// unittest

#[test]
fn check_answer_validity() -> Result<()> {
    let content = std::fs::read_to_string("./test.txt")?;
    let mut cnt = 0;
    for _ in content.lines() {
        cnt += 1;
    }
    assert_eq!(cnt, 7);
    Ok(())
}

#[test]
fn find_a_match() -> Result<()> {
    let mut result = Vec::new();
    let pb = indicatif::ProgressBar::new(2);
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &pb, &mut result)?;
    pb.finish_with_message("done");
    assert_eq!(result, b"lorem ipsum\n");
    Ok(())
}
