use std::error::Error;

enum Match {
    Win,
    Draw,
    Loss,
}

enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn from_str(s: &str) -> Result<Self, Box<dyn Error>> {
        match s {
            "A" => Ok(RPS::Rock),
            "B" => Ok(RPS::Paper),
            "C" => Ok(RPS::Scissors),
            "X" => Ok(RPS::Rock),
            "Y" => Ok(RPS::Paper),
            "Z" => Ok(RPS::Scissors),
            _ => Err("Invalid input".into()),
        }
    }

    fn play(&self, other: RPS) -> Match {
        match self {
            RPS::Rock => match other {
                RPS::Rock => Match::Draw,
                RPS::Paper => Match::Loss,
                RPS::Scissors => Match::Win,
            },
            RPS::Paper => match other {
                RPS::Rock => Match::Win,
                RPS::Paper => Match::Draw,
                RPS::Scissors => Match::Loss,
            },
            RPS::Scissors => match other {
                RPS::Rock => Match::Loss,
                RPS::Paper => Match::Win,
                RPS::Scissors => Match::Draw,
            },
        }
    }

    fn score_vs(&self, other: RPS) -> usize {
        let mut score = 0_usize;
        match self {
            RPS::Rock => score += 1,
            RPS::Paper => score += 2,
            RPS::Scissors => score += 3,
        };
        match self.play(other) {
            Match::Win => score += 6,
            Match::Draw => score += 3,
            Match::Loss => {}
        }
        score
    }
}

pub fn solve() -> Result<(), Box<dyn Error>> {
    let mut results: Vec<usize> = Vec::new();
    let mut data: Vec<Vec<&str>> = Vec::new();
    for line in include_str!("day2.input.txt").lines() {
        data.push(line.split(" ").collect());
    }
    for v in data {
        let mut vi = v.iter();
        let opponent: RPS = RPS::from_str(vi.next().ok_or_else(|| "")?)?;
        let me: RPS = RPS::from_str(vi.next().ok_or_else(|| "")?)?;
        //let b: RPS = RPS::from_str(vi.next().unwrap_or(&""))?;
        results.push(me.score_vs(opponent));
    }
    println!("{:?}", results.iter().sum::<usize>());
    // now try this functionally:
    //let result: usize = include_
    //.lines()
    //.map(|line| {
    //line.split(" ")
    //.chunks(2)
    //.map(|pair| RPS::from_str(pair.second()?)?.score_vs(RPS::from_str(pair.first()?)?))
    //})
    //.sum()?;
    Ok(())
}
