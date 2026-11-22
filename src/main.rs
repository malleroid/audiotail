#[cfg(target_os = "macos")]
mod processes;

use std::process::ExitCode;

use clap::Parser;

/// Stream the audio of selected apps to stdout as PCM.
#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    /// List the processes known to Core Audio and exit.
    #[arg(long)]
    list: bool,
}

fn main() -> ExitCode {
    let args = Args::parse();
    if args.list {
        return list();
    }
    eprintln!("audiotail: capturing is not implemented yet; use --list");
    ExitCode::FAILURE
}

#[cfg(target_os = "macos")]
fn list() -> ExitCode {
    let mut procs = match processes::list() {
        Ok(procs) => procs,
        Err(e) => {
            eprintln!("audiotail: {e}");
            return ExitCode::FAILURE;
        }
    };
    procs.sort_by(|a, b| a.bundle_id.cmp(&b.bundle_id).then(a.pid.cmp(&b.pid)));

    println!("PID\tOUTPUT\tBUNDLE_ID\tPATH");
    for p in procs {
        println!(
            "{}\t{}\t{}\t{}",
            p.pid.map_or_else(|| "?".to_string(), |pid| pid.to_string()),
            if p.running_output { "yes" } else { "no" },
            if p.bundle_id.is_empty() {
                "-".to_string()
            } else {
                escape_tsv_field(&p.bundle_id)
            },
            p.path
                .as_deref()
                .map_or_else(|| "-".to_string(), escape_tsv_field),
        );
    }
    ExitCode::SUCCESS
}

#[cfg_attr(not(target_os = "macos"), allow(dead_code))]
fn escape_tsv_field(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '\t' => out.push_str("\\t"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            _ => out.push(c),
        }
    }
    out
}

#[cfg(not(target_os = "macos"))]
fn list() -> ExitCode {
    eprintln!("audiotail: only macOS is supported");
    ExitCode::FAILURE
}

#[cfg(test)]
mod tests {
    use super::escape_tsv_field;

    #[test]
    fn plain_text_is_unchanged() {
        assert_eq!(
            escape_tsv_field("/Applications/Slack.app/Contents/MacOS/Slack"),
            "/Applications/Slack.app/Contents/MacOS/Slack"
        );
    }

    #[test]
    fn delimiters_and_backslash_are_escaped() {
        assert_eq!(escape_tsv_field("a\tb\nc\rd\\e"), "a\\tb\\nc\\rd\\\\e");
    }
}
