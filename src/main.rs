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
    #[structopt(short = "1")]
    one: Option<char>,
    #[structopt(short = "2")]
    two: Option<char>,
    #[structopt(short = "3")]
    three: Option<char>,
    #[structopt(short = "4")]
    four: Option<char>,
    #[structopt(short = "5")]
    five: Option<char>,
    #[structopt(long = "x1")]
    not_one: Option<String>,
    #[structopt(long = "x2")]
    not_two: Option<String>,
    #[structopt(long = "x3")]
    not_three: Option<String>,
    #[structopt(long = "x4")]
    not_four: Option<String>,
    #[structopt(long = "x5")]
    not_five: Option<String>,
    #[structopt(long)]
    exclude: Option<String>,
}

fn main() {
    let opts: Opts = Opts::from_args();
    let bank = build_bank();

    let fixed: Vec<_> = [opts.one, opts.two, opts.three, opts.four, opts.five]
        .into_iter()
        .enumerate()
        .filter_map(|(i, c)| c.map(|c| (i, c.to_ascii_lowercase())))
        .collect();

    let displaced: Vec<(usize, Vec<char>)> = [
        opts.not_one,
        opts.not_two,
        opts.not_three,
        opts.not_four,
        opts.not_five,
    ]
    .into_iter()
    .enumerate()
    .filter_map(|(i, cs)| match cs {
        Some(cs) if !cs.is_empty() => {
            Some((i, cs.chars().map(|c| c.to_ascii_lowercase()).collect()))
        }
        _ => None,
    })
    .collect();

    let exclude = opts.exclude.map(|s| s.trim().chars().collect::<Vec<_>>());

    for candidate in bank.iter().filter(|xs| {
        if let Some(exclude) = &exclude {
            if exclude.iter().any(|c| xs.contains(c)) {
                return false;
            }
        }

        if !fixed.is_empty() && !fixed.iter().all(|(i, c)| &xs[*i] == c) {
            return false;
        }

        if !displaced.is_empty() && !displaced
                .iter()
                .all(|(i, cs)| cs.iter().all(|c| &xs[*i] != c && xs.contains(c))) {
            return false;
        }
        true
    }) {
        println!("{word}", word = candidate.iter().collect::<String>())
    }
}
