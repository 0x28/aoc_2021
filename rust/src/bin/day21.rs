use rustc_hash::FxHashMap;

fn wrap(pos: u64, by: u64, limit: u64) -> u64 {
    (pos - 1 + by) % limit + 1
}

fn part1((start1, start2): &(u64, u64)) -> u64 {
    let mut player1 = (*start1, 0);
    let mut player2 = (*start2, 0);
    let mut die = 1;
    let mut die_count = 0;

    loop {
        let mut step = 0;
        for _ in 0..3 {
            step += die;
            die = wrap(die, 1, 100);
            die_count += 1;
        }
        player1.0 = wrap(player1.0, step, 10);
        player1.1 += player1.0;

        if player1.1 >= 1000 {
            break;
        }

        let mut step = 0;
        for _ in 0..3 {
            step += die;
            die = wrap(die, 1, 100);
            die_count += 1;
        }
        player2.0 = wrap(player2.0, step, 10);
        player2.1 += player2.0;

        if player2.1 >= 1000 {
            break;
        }
    }

    std::cmp::min(player1.1, player2.1) * die_count
}

type CountCache = FxHashMap<(u64, u64, u64, u64, bool), (usize, usize)>;

fn count_wins(
    cache: &mut CountCache,
    pos1: u64,
    score1: u64,
    pos2: u64,
    score2: u64,
    turn: bool,
) -> (usize, usize) {
    let cache_key = (pos1, score1, pos2, score2, turn);
    if let Some(result) = cache.get(&cache_key) {
        return *result;
    }

    if score1 >= 21 {
        return (1, 0);
    } else if score2 >= 21 {
        return (0, 1);
    }

    let mut sum = (0, 0);
    for d1 in 1..=3 {
        for d2 in 1..=3 {
            for d3 in 1..=3 {
                let (sum1, sum2) = if turn {
                    let pos = wrap(pos1, d1 + d2 + d3, 10);
                    count_wins(cache, pos, score1 + pos, pos2, score2, !turn)
                } else {
                    let pos = wrap(pos2, d1 + d2 + d3, 10);
                    count_wins(cache, pos1, score1, pos, score2 + pos, !turn)
                };

                sum.0 += sum1;
                sum.1 += sum2;
            }
        }
    }

    cache.insert(cache_key, sum);

    sum
}

fn part2(puzzle: &(u64, u64)) -> usize {
    let mut cache = FxHashMap::default();

    let (wins1, wins2) = count_wins(&mut cache, puzzle.0, 0, puzzle.1, 0, true);

    std::cmp::max(wins1, wins2)
}

fn main() {
    let input = (8, 7);
    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input));
}

#[test]
fn test_day21() {
    let input1 = (4, 8);

    assert_eq!(part1(&input1), 739785);
    assert_eq!(part2(&input1), 444356092776315);
}
