use crate::parser::Column;
use structopt::StructOpt;

mod parser;

const DATA: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/words.txt"));

fn build_bank() -> Vec<[char; 5]> {
    DATA.lines()
        .filter_map(|s| {
            let s = s.trim();
            if s.is_empty() {
                None
            } else {
                let mut word = ['\0'; 5];
                for (i, x) in s.chars().take(5).enumerate() {
                    word[i] = x.to_ascii_lowercase();
                }
                Some(word)
            }
        })
        .collect()
}

#[derive(Debug, StructOpt)]
#[structopt(name = "turdle", about = "Suggest guesses for an ongoing Wordle game.")]
struct Opts {
    one: Column,
    two: Column,
    three: Column,
    four: Column,
    five: Column,
    #[structopt(long)]
    exclude: Option<String>,
}

fn main() {
    let opts: Opts = Opts::from_args();
    let bank = build_bank();

    let columns = [opts.one, opts.two, opts.three, opts.four, opts.five];
    let exclude = opts.exclude.map(|s| s.trim().chars().collect::<Vec<_>>());

    for candidate in bank.iter().filter(|xs| {
        if let Some(exclude) = &exclude {
            if exclude.iter().any(|c| xs.contains(c)) {
                return false;
            }
        }

        for (idx, column) in columns.iter().enumerate() {
            if let Some(fixed) = &column.fixed {
                if &xs[idx] != fixed {
                    return false;
                }
            }

            if let Some(displaced) = &column.displaced {
                if !displaced.is_empty()
                    && !displaced.iter().all(|c| &xs[idx] != c && xs.contains(c))
                {
                    return false;
                }
            }
        }
        true
    }) {
        println!("{word}", word = candidate.iter().collect::<String>())
    }
}
