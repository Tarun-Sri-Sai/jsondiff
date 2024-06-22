use difference::{Changeset, Difference};
use serde_json::Value;
use std::io::Write;
use termcolor::{Color as TermColor, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub fn show(json1: &Value, json2: &Value) {
    let json1_str = serde_json::to_string_pretty(json1).unwrap();
    let json2_str = serde_json::to_string_pretty(json2).unwrap();

    let changeset = Changeset::new(&json1_str, &json2_str, "\n");

    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    for diff in changeset.diffs {
        match diff {
            Difference::Same(ref x) => {
                stdout.set_color(ColorSpec::new().set_fg(None)).unwrap();
                write!(stdout, " {}", x).unwrap();
            }
            Difference::Add(ref x) => {
                stdout
                    .set_color(ColorSpec::new().set_fg(Some(TermColor::Green)))
                    .unwrap();
                write!(stdout, "+{}", x).unwrap();
            }
            Difference::Rem(ref x) => {
                stdout
                    .set_color(ColorSpec::new().set_fg(Some(TermColor::Red)))
                    .unwrap();
                write!(stdout, "-{}", x).unwrap();
            }
        }
        writeln!(stdout).unwrap();
    }
    stdout.set_color(ColorSpec::new().set_fg(None)).unwrap();
}
