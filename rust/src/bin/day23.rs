use rustc_hash::FxHashMap;

//   0123456789ABC
// 0 #############
// 1 #...........#
// 2 ###.#.#.#.###
// 3   #.#.#.#.#
// 4   #########

type Positions = FxHashMap<(i64, i64), (char, bool)>;

fn distance(p1: &(i64, i64), p2: &(i64, i64)) -> f64 {
    let both_in_lower = p1.1 > 1 && p2.1 > 1;
    if both_in_lower {
        ((p1.1 - 1) + i64::abs(p1.0 - p2.0) + i64::abs(1 - p2.1)) as f64
    } else {
        (i64::abs(p1.0 - p2.0) + i64::abs(p1.1 - p2.1)) as f64
    }
}

fn adj(
    pos: &(i64, i64),
    amphipod: (char, bool),
    state: &Positions,
    p2: bool,
) -> Vec<(i64, i64, f64)> {
    let waiting_pos =
        vec![(1, 1), (2, 1), (4, 1), (6, 1), (8, 1), (10, 1), (11, 1)];
    let mut start_pos = vec![
        ('A', 3, 2),
        ('A', 3, 3),
        ('B', 5, 2),
        ('B', 5, 3),
        ('C', 7, 2),
        ('C', 7, 3),
        ('D', 9, 2),
        ('D', 9, 3),
    ];

    if p2 {
        start_pos.extend([
            ('A', 3, 4),
            ('A', 3, 5),
            ('B', 5, 4),
            ('B', 5, 5),
            ('C', 7, 4),
            ('C', 7, 5),
            ('D', 9, 4),
            ('D', 9, 5),
        ]);
    }

    let mut destinations = start_pos
        .iter()
        .filter(|p| p.0 == amphipod.0)
        .map(|s| (s.1, s.2, distance(&(s.1, s.2), pos)))
        .collect::<Vec<_>>();

    let mut clear = false;
    for &(x, y, _) in destinations.iter() {
        if let Some(&(atype, _)) = state.get(&(x, y)) {
            if atype != amphipod.0 {
                clear = true;
                break;
            }
        }
    }

    if clear {
        destinations.clear();
    }

    let destination = destinations
        .into_iter()
        .filter(|&(x, y, _)| state.get(&(x, y)).is_none())
        .max_by(|(_, y1, _), (_, y2, _)| y1.cmp(y2));

    if start_pos
        .iter()
        .filter(|(_, x, y)| (*x, *y) == *pos)
        .count()
        >= 1
    {
        if amphipod.1 {
            vec![] // already at destination
        } else {
            destination
                .into_iter()
                .chain(waiting_pos.iter().map(|w| (w.0, w.1, distance(w, pos))))
                .collect()
        }
    } else if waiting_pos.contains(pos) {
        destination.into_iter().collect()
    } else {
        unreachable!()
    }
}

fn path_free(start: &(i64, i64), dest: &(i64, i64), state: &Positions) -> bool {
    let mut current = *start;

    while current != *dest {
        let on_top = current.1 == 1;
        if on_top {
            if current.0 > dest.0 {
                current.0 -= 1; // go left
            } else if current.0 < dest.0 {
                current.0 += 1; // go right
            } else if current.1 < dest.1 {
                current.1 += 1; // go down
            }
        } else {
            // lower
            if current.0 != dest.0 {
                current.1 -= 1; // go up
            } else if current.1 < dest.1 {
                current.1 += 1; // go down
            } else if current.1 > dest.1 {
                current.1 -= 1; // go up
            }
        }

        if state.contains_key(&current) && current != *start {
            return false;
        }
    }

    true
}

fn possible_moves(
    start: &(i64, i64),
    state: &Positions,
    p2: bool,
) -> Vec<(i64, i64, f64)> {
    adj(start, *state.get(start).unwrap(), state, p2)
        .iter()
        .filter(|(x, y, _)| path_free(start, &(*x, *y), state))
        .copied()
        .collect()
}

fn update_state(
    state: &Positions,
    from: &(i64, i64),
    to: &(i64, i64),
    atype: char,
) -> Positions {
    let mut new_state = state.clone();
    new_state.remove(from);
    new_state.insert(*to, (atype, true));

    new_state
}

fn calc_cost(atype: char, cost: f64) -> f64 {
    cost * match atype {
        'A' => 1.0,
        'B' => 10.0,
        'C' => 100.0,
        'D' => 1000.0,
        _ => unreachable!(),
    }
}

fn done(state: &Positions) -> bool {
    let only_in_column = |col, atype| {
        state
            .iter()
            .filter(|(_pos, (t, _))| *t == atype)
            .all(|(pos, _)| pos.1 >= 2 && pos.0 == col)
    };

    only_in_column(3, 'A')
        && only_in_column(5, 'B')
        && only_in_column(7, 'C')
        && only_in_column(9, 'D')
}

fn min_float(a: f64, b: f64) -> f64 {
    if a > b {
        b
    } else {
        a
    }
}

type Cache = FxHashMap<Vec<(i64, i64, char)>, f64>;

fn flatten(state: &Positions) -> Vec<(i64, i64, char)> {
    state.iter().map(|((x, y), (t, _))| (*x, *y, *t)).collect()
}

fn min_energy(cache: &mut Cache, state: &Positions, p2: bool) -> f64 {
    let mut min = f64::INFINITY;
    let flat_state = flatten(state);

    if let Some(value) = cache.get(&flat_state) {
        return *value;
    }

    if done(state) {
        cache.insert(flat_state, 0.0);
        return 0.0;
    }

    for (from, atype) in state {
        for (x, y, cost) in possible_moves(from, state, p2) {
            let new_state = update_state(state, from, &(x, y), atype.0);
            min = min_float(
                min,
                calc_cost(atype.0, cost) + min_energy(cache, &new_state, p2),
            );
        }
    }

    cache.insert(flat_state, min);

    min
}

fn part1(input: &[(char, char)]) -> f64 {
    let mut state = Positions::default();
    let mut cache = Cache::default();

    state.insert((3, 2), (input[0].0, false));
    state.insert((3, 3), (input[0].1, false));
    state.insert((5, 2), (input[1].0, false));
    state.insert((5, 3), (input[1].1, false));
    state.insert((7, 2), (input[2].0, false));
    state.insert((7, 3), (input[2].1, false));
    state.insert((9, 2), (input[3].0, false));
    state.insert((9, 3), (input[3].1, false));

    min_energy(&mut cache, &state, false)
}

fn part2(input: &[(char, char, char, char)]) -> f64 {
    let mut state = Positions::default();
    let mut cache = Cache::default();

    state.insert((3, 2), (input[0].0, false));
    state.insert((3, 3), (input[0].1, false));
    state.insert((3, 4), (input[0].2, false));
    state.insert((3, 5), (input[0].3, false));

    state.insert((5, 2), (input[1].0, false));
    state.insert((5, 3), (input[1].1, false));
    state.insert((5, 4), (input[1].2, false));
    state.insert((5, 5), (input[1].3, false));

    state.insert((7, 2), (input[2].0, false));
    state.insert((7, 3), (input[2].1, false));
    state.insert((7, 4), (input[2].2, false));
    state.insert((7, 5), (input[2].3, false));

    state.insert((9, 2), (input[3].0, false));
    state.insert((9, 3), (input[3].1, false));
    state.insert((9, 4), (input[3].2, false));
    state.insert((9, 5), (input[3].3, false));

    min_energy(&mut cache, &state, true)
}

fn main() {
    println!(
        "part1 = {}",
        part1(&[('C', 'B'), ('A', 'A'), ('D', 'B'), ('D', 'C')])
    );
    println!(
        "part2 = {}",
        part2(&[
            ('C', 'D', 'D', 'B'),
            ('A', 'C', 'B', 'A'),
            ('D', 'B', 'A', 'B'),
            ('D', 'A', 'C', 'C')
        ])
    );
}

#[test]
fn test_day23() {
    assert_eq!(
        part1(&vec![('B', 'A'), ('C', 'D'), ('B', 'C'), ('D', 'A')]),
        12521.0
    );
    assert_eq!(
        part2(&[
            ('B', 'D', 'D', 'A'),
            ('C', 'C', 'B', 'D'),
            ('B', 'B', 'A', 'C'),
            ('D', 'A', 'C', 'A')
        ]),
        44169.0
    );
}
