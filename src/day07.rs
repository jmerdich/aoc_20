use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;
use string_interner::{DefaultSymbol, StringInterner};

type Sym = DefaultSymbol;
lazy_static! {
    static ref INTERN: Mutex<StringInterner<Sym>> = Mutex::new(StringInterner::new());
}

pub struct Container {
    contents: Vec<(usize, Sym)>,
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> HashMap<Sym, Container> {
    let mut map: HashMap<Sym, Container> = HashMap::new();

    input.lines().for_each(|l| {
        let mut cont = Container {
            contents: Vec::new(),
        };
        let (name, rest) = l.split(" bags contain ").collect_tuple().unwrap();
        if rest != "no other bags." {
            rest.split(", ").for_each(|s| {
                let s = s
                    .trim_end_matches('.')
                    .trim_end_matches('s')
                    .trim_end_matches(" bag");
                let (num, color) = s.splitn(2, ' ').collect_tuple().unwrap();
                cont.contents.push((
                    num.parse().unwrap(),
                    INTERN.lock().unwrap().get_or_intern(color),
                ));
            });
        }
        let old = map.insert(INTERN.lock().unwrap().get_or_intern(name), cont);
        assert!(old.is_none());
    });
    map
}

fn get_parents(input: &HashMap<Sym, Container>) -> HashMap<Sym, Vec<Sym>> {
    let mut out: HashMap<Sym, Vec<Sym>> = HashMap::new();

    for (parent, conts) in input.iter() {
        for cont in &conts.contents {
            if let Some(parents) = out.get_mut(&cont.1) {
                parents.push(*parent);
            } else {
                out.insert(cont.1, vec![*parent]);
            }
        }
    }
    out
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &HashMap<Sym, Container>) -> usize {
    let mut possible_conts: HashSet<Sym> = HashSet::new();

    let inverted_conts = get_parents(input);
    possible_conts.extend(
        inverted_conts[&INTERN.lock().unwrap().get_or_intern_static("shiny gold")]
            .clone()
            .into_iter(),
    );

    let mut old_size = 0;
    let mut new_res: Vec<Sym> = possible_conts.iter().cloned().collect();
    while old_size != possible_conts.len() {
        old_size = possible_conts.len();
        new_res = new_res
            .iter()
            .flat_map(|s| inverted_conts.get(s).unwrap_or(&vec![]).clone().into_iter())
            .collect();
        possible_conts.extend(new_res.clone());
    }

    possible_conts.len()
}

fn count_nested_bags(input: &HashMap<Sym, Container>, name: Sym) -> usize {
    input
        .get(&name)
        .unwrap()
        .contents
        .iter()
        .map(|(n, subbag)| n * count_nested_bags(input, *subbag))
        .sum::<usize>()
        + 1
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &HashMap<Sym, Container>) -> usize {
    count_nested_bags(
        input,
        INTERN.lock().unwrap().get_or_intern_static("shiny gold"),
    ) - 1
}

#[cfg(test)]
mod test {
    use super::*;

    const EG_INPUT: &str = "\
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    const EG_INPUT2: &str = "\
shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
    const INPUT: &str = include_str!("../input/2020/day7.txt");
    #[test]
    fn parser() {
        let content1 = input_generator(EG_INPUT);
        assert_eq!(content1.len(), 9);
        input_generator(INPUT);
    }

    #[test]
    fn eg_part1() {
        let content = input_generator(EG_INPUT);
        assert_eq!(solve_part1(&content), 4);
    }
    #[test]
    fn eg_part2() {
        let content = input_generator(EG_INPUT);
        assert_eq!(solve_part2(&content), 32);
        let content = input_generator(EG_INPUT2);
        assert_eq!(solve_part2(&content), 126);
    }
    #[test]
    fn part1() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part1(&content), 139);
    }
    #[test]
    fn part2() {
        let content = input_generator(INPUT);
        assert_eq!(solve_part2(&content), 58175);
    }
}
