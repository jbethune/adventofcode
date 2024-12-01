mod calories;
mod climbing;
mod cpu;
mod crates;
mod direction;
mod directory;
mod forest;
mod matrix;
mod monkey;
mod rockpaper;
mod rope;
mod rucksack;
mod section_cleanup;
mod signal;

pub fn dispatch(day: usize) -> Option<(usize, usize)> {
    let result = match day {
        1 => {
            let mut sorted = calories::sorted_calories_from_file("input/day1");
            let highest = sorted.pop().unwrap();
            let mut top3_sum = highest;
            top3_sum += sorted.pop().unwrap();
            top3_sum += sorted.pop().unwrap();
            (highest, top3_sum)
        }
        2 => {
            let choices = rockpaper::read_choices_from_file("input/day2");
            let mut total_score = 0;
            for choice in choices {
                total_score += choice.score();
            }
            let choices_with_outcomes =
                rockpaper::read_outcome_based_choices_from_file("input/day2");
            let mut total_score2 = 0;
            for choice in choices_with_outcomes {
                total_score2 += choice.score();
            }

            (total_score, total_score2)
        }
        3 => {
            let rucksacks = rucksack::get_rucksacks_from_file("input/day3");
            let mut total = 0;
            for rucksack in &rucksacks {
                let duplicate = rucksack.get_duplicate_item();
                total += duplicate;
            }
            let iter = &mut rucksacks.iter();
            let mut sum = 0;
            loop {
                if let Some(r1) = iter.next() {
                    let r2 = iter.next().unwrap();
                    let r3 = iter.next().unwrap();
                    sum += rucksack::get_badge(r1, r2, r3);
                } else {
                    break;
                }
            }
            (total, sum)
        }
        4 => {
            let range_pairs = section_cleanup::get_assignments_from_file("input/day4");
            let mut total_contained = 0;
            let mut total_overlap = 0;
            for pair in range_pairs {
                if pair.is_completely_contained() {
                    total_contained += 1;
                }
                if pair.overlaps() {
                    total_overlap += 1;
                }
            }
            (total_contained, total_overlap)
        }
        5 => {
            for all_at_once in &[false, true] {
                let mut warehouse = crates::read_stacks_from_file("input/day5");
                let instructions = crates::read_instructions_from_file("input/day5");
                for instruction in &instructions {
                    warehouse.apply(instruction, *all_at_once);
                }
                warehouse.report_top_row();
            }
            (0, 0)
        }
        6 => {
            let data_start = signal::find_signal_start_in_file("input/day6", 4);
            let message_start = signal::find_signal_start_in_file("input/day6", 14);
            (data_start, message_start)
        }
        7 => {
            let root = directory::walk_through_commands_from_file("input/day7");
            let sizes = directory::traverse_directories_and_gather_sizes(&root);
            let sum = sizes
                .iter()
                .filter(|x| **x <= 100000)
                .fold(0, |acc, x| acc + x);
            let capacity = 70000000;
            let needed_free_space = 30000000;
            let currently_occupied = root.total_size();
            let need_to_delete = needed_free_space - (capacity - currently_occupied);
            let delete_size = sizes
                .iter()
                .filter(|x| **x >= need_to_delete)
                .reduce(std::cmp::min)
                .unwrap();
            (sum, *delete_size)
        }
        8 => {
            let heights = forest::matrix_from_file("input/day8");
            let visibility = forest::visibility_map(&heights);
            let total_visible = visibility
                .values
                .iter()
                .filter(|is_visible| **is_visible)
                .count();

            let best_view_score = forest::best_view_score(&heights);
            (total_visible, best_view_score)
        }
        9 => {
            let moves = rope::read_moves_from_file("input/day9");
            let plank = rope::visited_places(2, &moves);
            let long_rope = rope::visited_places(10, &moves);
            (plank.len(), long_rope.len())
        }
        10 => {
            let commands = cpu::read_commands_from_file("input/day10");
            let mut cpu = cpu::CPU::new(commands);
            let signals = cpu.run(&[20, 60, 100, 140, 180, 220]);
            let part1: isize = signals.iter().cloned().sum();
            println!("part1={}", part1);
            cpu.render_screen();

            (0, 0)
        }
        11 => {
            fn repeat(iterations: usize, reduce_stress: bool) -> usize {
                let mut monkies = monkey::read_monkies_from_file("input/day11", reduce_stress);
                let total_items: usize = monkies.iter().map(|m| m.items.len()).sum();
                let common_divisor: usize = monkies.iter().map(|m| m.divisor).product();
                for _ in 0..iterations {
                    for i in 0..monkies.len() {
                        let thrown_items = monkies[i].throw(common_divisor);
                        let catcher = &mut monkies[thrown_items.targets.0];
                        catcher.catch(&thrown_items.items.0);
                        let catcher = &mut monkies[thrown_items.targets.1];
                        catcher.catch(&thrown_items.items.1);
                    }
                    assert_eq!(total_items, monkies.iter().map(|m| m.items.len()).sum());
                }
                let mut monkey_buisiness: Vec<usize> =
                    monkies.iter().map(|m| m.inspections).collect();
                monkey_buisiness.sort();
                let mut top_buisiness = monkey_buisiness.pop().unwrap();
                top_buisiness *= monkey_buisiness.pop().unwrap();
                top_buisiness
            }
            (repeat(20, true), repeat(10000, false))
        }
        12 => {
            let map = climbing::read_heightmap_from_file("input/day12");
            let steps: usize = climbing::shortest_path_length(&map);
            (steps, 0)
        }
        _ => return None,
    };
    Some(result)
}
