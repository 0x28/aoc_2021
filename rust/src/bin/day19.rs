use aoc_2021::input_file;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

type Pos = (i64, i64, i64);

fn parse(input: &str) -> Vec<Vec<Pos>> {
    let mut scanner = vec![];
    for group in input.split("\n\n") {
        let mut s = vec![];
        for b in group
            .lines()
            .skip(1)
            .map(|l| l.split(',').flat_map(str::parse).collect::<Vec<i64>>())
        {
            s.push((b[0], b[1], b[2]));
        }

        scanner.push(s);
    }

    scanner
}

fn sub_pos(pos1: &Pos, pos2: &Pos) -> Pos {
    (pos1.0 - pos2.0, pos1.1 - pos2.1, pos1.2 - pos2.2)
}

fn diffs(scanner: &[Pos]) -> HashMap<Pos, Pos> {
    let mut diffs = HashMap::new();

    for &b_left in scanner {
        for &b_right in scanner {
            if b_left != b_right {
                diffs.insert(sub_pos(&b_left, &b_right), b_left);
            }
        }
    }

    diffs
}

fn common_beacons(scanner1: &[Pos], scanner2: &[Pos]) -> HashSet<(Pos, Pos)> {
    let diffs1 = diffs(scanner1);
    let diffs2 = diffs(scanner2);
    let mut common = HashSet::new();

    for (diff, pos1) in diffs1 {
        if let Some(pos2) = diffs2.get(&diff) {
            common.insert((pos1, *pos2));
        }
    }

    common
}

fn orientations(scanner: &[Pos]) -> Vec<Vec<Pos>> {
    let rotate_left_x = |&(x, y, z): &Pos| (x, z, -y);
    let rotate_left_y = |&(x, y, z): &Pos| (-z, y, x);
    let rotate_left_z = |&(x, y, z): &Pos| (-y, x, z);

    let mut orientations = HashSet::<Vec<Pos>>::from_iter([scanner.to_vec()]);
    let mut old_size = 1;

    loop {
        let mut new_orientations = orientations.clone();
        for scanner in &orientations {
            new_orientations
                .insert(scanner.iter().map(rotate_left_x).collect());
            new_orientations
                .insert(scanner.iter().map(rotate_left_y).collect());
            new_orientations
                .insert(scanner.iter().map(rotate_left_z).collect());
        }

        std::mem::swap(&mut orientations, &mut new_orientations);

        if old_size == orientations.len() {
            break;
        } else {
            old_size = orientations.len();
        }
    }

    orientations.into_iter().collect()
}

type PositionMapping = HashSet<(Pos, Pos)>;

fn common_beacons_with_rotation(
    fixed_scanner: &[Pos],
    scanner: &[Pos],
) -> Option<(Vec<Pos>, PositionMapping)> {
    for o in orientations(scanner) {
        let common = common_beacons(fixed_scanner, &o);
        if common.len() >= 12 {
            return Some((o, common));
        }
    }

    None
}

fn manhatten_distance(p1: &Pos, p2: &Pos) -> i64 {
    let (x, y, z) = sub_pos(p1, p2);

    i64::abs(x) + i64::abs(y) + i64::abs(z)
}

fn solve(puzzle: &[Vec<Pos>]) -> (usize, i64) {
    let mut fixed_scanners =
        HashSet::<Vec<Pos>>::from_iter([puzzle.first().unwrap().clone()]);
    let mut scanners = HashSet::<Vec<Pos>>::from_iter(
        puzzle.iter().skip(1).cloned().collect::<Vec<_>>(),
    );
    let mut scanner_positions = vec![];

    while !scanners.is_empty() {
        let mut remove_scanner = None;
        let mut new_fixed_scanner = None;
        for scanner in &scanners {
            for fixed_scanner in &fixed_scanners {
                if let Some((o, beacons)) =
                    common_beacons_with_rotation(fixed_scanner, scanner)
                {
                    if beacons.len() >= 12 {
                        let (fixed_pos, new_pos) =
                            beacons.iter().next().unwrap();
                        let delta = sub_pos(new_pos, fixed_pos);
                        scanner_positions.push(delta);
                        remove_scanner = Some(scanner.clone());
                        new_fixed_scanner = Some(
                            o.iter().map(|b| sub_pos(b, &delta)).collect(),
                        );
                        break;
                    }
                }
            }
        }

        if let Some(fixed_scanner) = new_fixed_scanner {
            fixed_scanners.insert(fixed_scanner);
        }

        if let Some(scanner) = remove_scanner {
            scanners.remove(&scanner);
        }
    }

    let mut beacons = HashSet::<Pos>::new();

    for s in fixed_scanners {
        beacons.extend(s.iter());
    }

    let mut max_dist = 0;
    for p1 in &scanner_positions {
        for p2 in &scanner_positions {
            if p1 < p2 {
                max_dist = i64::max(max_dist, manhatten_distance(p1, p2));
            }
        }
    }

    (beacons.len(), max_dist)
}

fn main() {
    let input = fs::read_to_string(input_file("input19.txt")).unwrap();
    let input = parse(&input);
    let (p1, p2) = solve(&input);
    println!("part1 = {}", p1);
    println!("part2 = {}", p2);
}

#[test]
fn test_day19() {
    let input = "\
--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14";
    let input = parse(input);

    assert_eq!(solve(&input), (79, 3621));
}
