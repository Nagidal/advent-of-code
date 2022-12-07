use std::collections::HashSet;
use std::error::Error;

fn part1() -> Result<Vec<char>, Box<dyn Error>> {
    let data = include_str!("day3.input.txt").lines();
    let mut shared_items: Vec<char> = Vec::new();
    for line in data {
        let (first, second) = line.split_at(line.chars().count() / 2);
        assert_eq!(first.chars().count(), second.chars().count());
        let first_set: HashSet<char> = HashSet::from_iter(first.chars());
        let second_set: HashSet<char> = HashSet::from_iter(second.chars());
        let intersection: HashSet<_> = first_set.intersection(&second_set).collect();
        assert_eq!(intersection.len(), 1);
        shared_items.push(
            *intersection
                .into_iter()
                .next()
                .unwrap_or_else(|| &char::REPLACEMENT_CHARACTER),
        );
    }
    Ok(shared_items)
}

fn part2() -> Result<Vec<char>, Box<dyn Error>> {
    let data = include_str!("day3.input2.txt").lines();
    let mut group_priorities: Vec<char> = Vec::new();
    for [first, second, third] in data.array_chunks() {
        //println!("{} - {} - {}", first, second, third);
        let (first_set, second_set, third_set): (HashSet<char>, HashSet<char>, HashSet<char>) = (
            HashSet::from_iter(first.chars()),
            HashSet::from_iter(second.chars()),
            HashSet::from_iter(third.chars()),
        );
        // for a bettes solution of the intersection of multiple sets see
        // https://www.reddit.com/r/adventofcode/comments/zbscd2/day3_is_there_a_more_idiomatic_way_to_check_the/.compact
        // or (this one does not really work:)
        // https://www.reddit.com/r/rust/comments/5v35l6/intersection_of_more_than_two_sets/.compact
        let intersection12: HashSet<char> =
            HashSet::from_iter(first_set.into_iter().filter(|c| second_set.contains(c)));
        let intersection: HashSet<char> =
            HashSet::from_iter(intersection12.into_iter().filter(|c| third_set.contains(c)));
        //println!("{:?}", intersection);
        group_priorities.push(
            *intersection
                .iter()
                .next()
                .unwrap_or_else(|| &char::REPLACEMENT_CHARACTER),
        );
    }
    Ok(group_priorities)
}

fn sum_types(v: Vec<char>) -> Result<usize, Box<dyn Error>> {
    let mut sum: usize = 0;
    for c in v {
        let mut shift: usize = 64 + 32;
        if c.is_ascii_uppercase() {
            shift = 64 - 26;
        }
        sum += c as usize - shift
    }
    Ok(sum)
}

pub fn solve() -> Result<(), Box<dyn Error>> {
    println!("individual priorities: {}", sum_types(part1()?)?);
    println!("group priorities: {}", sum_types(part2()?)?);
    Ok(())
}
