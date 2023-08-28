use std::{env, fs};

#[derive(Debug)]
struct ElfBag(Vec<i32>);

trait TotalExt {
    fn total(&self) -> i32;
}
impl TotalExt for ElfBag {
    fn total(&self) -> i32 {
        let mut sum: i32 = 0;
        for v in self.0.iter() {
            sum += v
        }

        sum
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let arg = match args.get(1) {
        Some(a) => a,
        None => panic!("Please provide a file name"),
    };

    let file = match fs::read_to_string(arg) {
        Ok(result) => result,
        Err(..) => panic!("Cannot read file {}", arg),
    };

    let mut elves: Vec<ElfBag> = vec![];
    for block in file.split("\n\n") {
        let mut elf_bag: Vec<i32> = vec![];

        for entry in block.split("\n") {
            if entry == "" {
                continue;
            }
            let calories = match entry.parse::<i32>() {
                Ok(c) => c,
                Err(e) => panic!("{}", e),
            };
            elf_bag.push(calories);
        }

        elves.push(ElfBag(elf_bag));
    }

    let mut totals: Vec<i32> = vec![];
    for v in elves {
        totals.push(v.total())
    }

    let mut top3total = 0;
    for _ in 0..3 {
        let max = *totals.iter().max().unwrap_or(&0);
        top3total += max;
        match totals.iter().position(|v| *v == max) {
            Some(idx) => totals.swap_remove(idx),
            None => 0,
        };
    }

    println!("{:#?}", top3total)
}
