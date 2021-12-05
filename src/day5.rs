use std::convert::TryFrom;
use std::fmt;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::usize;

pub fn run() {
    let input = read_file("inputs/day5.txt");
    let coordinate_pairs: Vec<[Coordinate; 2]> =
        input.iter().map(|x| read_coordinates(x)).collect();

    //let filtered_pairs: Vec<[Coordinate; 2]> = coordinate_pairs
    //    .into_iter()
    //   .filter(|a| is_diag(a) == false)
    //    .collect::<Vec<[Coordinate; 2]>>();

    let lines: Vec<Vec<Coordinate>> = coordinate_pairs
        .iter()
        .map(|x| interpolate_points(x))
        .collect();

    let flat_points: Vec<Coordinate> = lines.into_iter().flatten().collect();

    let grid = populate_grid(flat_points);
    //println!("{:?}", grid);
    println!("{}", grid.num_overlaps())
}
struct Grid {
    grid: [[u8; 999]; 999],
}

impl Grid {
    fn new() -> Self {
        Grid {
            grid: [[0; 999]; 999],
        }
    }

    fn increment_point(&mut self, x: usize, y: usize) {
        self.grid[y][x] += 1
    }
    fn num_overlaps(&self) -> u32 {
        let overlapping_points = self.grid.iter().flatten().filter(|x| **x > 1).count();

        return u32::try_from(overlapping_points).unwrap();
    }
}
impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Ok(for row in self.grid {
            println!("");
            for value in row {
                if value != 0 {
                    print!("{}", value)
                } else {
                    print! {"."}
                }
            }
        })
    }
}
#[derive(Debug, PartialEq, Copy, Clone)]
struct Coordinate {
    x: i16,
    y: i16,
}

fn read_coordinates(line: &String) -> [Coordinate; 2] {
    // turn a line into a len 2 array of two x:y points
    let pairs: Vec<String> = line.split(" -> ").map(|s| s.to_string()).collect();
    let c1: Vec<i16> = pairs[0]
        .split(',')
        .map(|s| s.to_string().parse::<i16>().unwrap())
        .collect();
    let c2: Vec<i16> = pairs[1]
        .split(',')
        .map(|s| s.to_string().parse::<i16>().unwrap())
        .collect();

    let coordinate1 = Coordinate { x: c1[0], y: c1[1] };
    let coordinate2 = Coordinate { x: c2[0], y: c2[1] };
    return [coordinate1, coordinate2];
}

fn interpolate_points(coordinate_pair: &[Coordinate; 2]) -> Vec<Coordinate> {
    // do the linear interpolation between the two points
    // to get the rest of the points
    let c0 = coordinate_pair[0];
    let c1 = coordinate_pair[1];
    let mut interp = vec![c0, c1];
    if c0.x == c1.x {
        interp.sort_by_key(|v| v.y);
        for i in interp[0].y + 1..interp[1].y {
            interp.push(Coordinate { x: c0.x, y: i })
        }
    } else if c0.y == c1.y {
        interp.sort_by_key(|v| v.x);
        for i in interp[0].x + 1..interp[1].x {
            interp.push(Coordinate { x: i, y: c0.y });
        }
    } else if is_diag(coordinate_pair) {
        interp.sort_by_key(|v| v.x);
        if interp[0].y > interp[1].y {
            let y_increment: i16 = -1;
            let mut y_val = interp[0].y;
            for i in interp[0].x + 1..interp[1].x {
                y_val += y_increment;
                interp.push(Coordinate { x: i, y: y_val });
            }
        } else {
            let y_increment: i16 = 1;
            let mut y_val = interp[0].y;
            for i in interp[0].x + 1..interp[1].x {
                y_val += y_increment;
                interp.push(Coordinate { x: i, y: y_val });
            }
        }
    }
    return interp;
}

fn is_diag(pair: &[Coordinate; 2]) -> bool {
    let a = pair[0];
    let b = pair[1];
    return a.x != b.x && a.y != b.y;
}

fn populate_grid(points: Vec<Coordinate>) -> Grid {
    let mut grid: Grid = Grid::new();
    for point in points {
        grid.increment_point(
            usize::try_from(point.x).unwrap(),
            usize::try_from(point.y).unwrap(),
        );
    }
    return grid;
}

fn read_file(file: &str) -> Vec<String> {
    let reader = BufReader::new(File::open(file).expect("could not open file"));
    reader
        .lines()
        .map(|l| l.expect("could not parse line"))
        .collect()
}

#[cfg(test)]
mod unit_test {

    use crate::day5::{interpolate_points, read_coordinates, run, Coordinate, Grid};
    extern crate test;
    use test::Bencher;

    use super::is_diag;

    #[test]
    fn test_increment_grid_index() {
        let mut grid = Grid::new();
        grid.increment_point(5, 5);
        assert_eq!(grid.grid[5][5], 1);
    }
    #[test]
    fn test_create_coordinates() {
        let coordinates = read_coordinates(&"0,9 -> 5,9".to_string());
        assert_eq!(coordinates[0], Coordinate { x: 0, y: 9 });
        assert_eq!(coordinates[1], Coordinate { x: 5, y: 9 });

        let coordinates = read_coordinates(&"738,5 -> 282,52".to_string());
        assert_eq!(coordinates[0], Coordinate { x: 738, y: 5 });
        assert_eq!(coordinates[1], Coordinate { x: 282, y: 52 })
    }

    #[test]
    fn test_interpolate_x() {
        let coordinate_pair_right = read_coordinates(&"0,9 -> 5,9".to_string());
        let coordinate_pair_left = read_coordinates(&"5,9 -> 0,9".to_string());

        let line_right: Vec<Coordinate> = interpolate_points(&coordinate_pair_right);
        let line_left: Vec<Coordinate> = interpolate_points(&coordinate_pair_left);
        println! {"{:?}", line_left};
        println! {"{:?}", line_right};
        let coordinates = [
            Coordinate { x: 0, y: 9 },
            Coordinate { x: 1, y: 9 },
            Coordinate { x: 2, y: 9 },
            Coordinate { x: 3, y: 9 },
            Coordinate { x: 4, y: 9 },
            Coordinate { x: 5, y: 9 },
        ];
        for coordinate in coordinates {
            assert_eq!(line_right.contains(&coordinate), true);
            assert_eq!(line_left.contains(&coordinate), true);
        }
    }
    #[test]
    fn test_interpolate_y() {
        let coordinate_pair_up = read_coordinates(&"0,5 -> 0,9".to_string());
        let coordinate_pair_down = read_coordinates(&"0,9 -> 0,5".to_string());

        let line_up: Vec<Coordinate> = interpolate_points(&coordinate_pair_up);
        println!("{:?}", line_up);
        let line_down: Vec<Coordinate> = interpolate_points(&coordinate_pair_down);
        println!("{:?}", line_down);
        let coordinates = vec![
            Coordinate { x: 0, y: 5 },
            Coordinate { x: 0, y: 6 },
            Coordinate { x: 0, y: 7 },
            Coordinate { x: 0, y: 8 },
            Coordinate { x: 0, y: 9 },
        ];
        for coordinate in coordinates {
            assert_eq!(line_up.contains(&coordinate), true);
            assert_eq!(line_down.contains(&coordinate), true);
        }
    }
    #[test]
    fn test_interpolate_diag() {
        let coordinate_pair_diag1 = read_coordinates(&"8,0 -> 0,8".to_string());
        let coordinate_pair_diag2 = read_coordinates(&"2,0 -> 6,4".to_string());

        let line_diag1: Vec<Coordinate> = interpolate_points(&coordinate_pair_diag1);
        let line_diag2: Vec<Coordinate> = interpolate_points(&coordinate_pair_diag2);

        println!("{:?}", line_diag1);
        println!("{:?}", line_diag2);

        let coordinates1 = vec![
            Coordinate { x: 0, y: 8 },
            Coordinate { x: 1, y: 7 },
            Coordinate { x: 2, y: 6 },
            Coordinate { x: 3, y: 5 },
            Coordinate { x: 4, y: 4 },
            Coordinate { x: 5, y: 3 },
            Coordinate { x: 6, y: 2 },
            Coordinate { x: 7, y: 1 },
            Coordinate { x: 8, y: 0 },
        ];
        for coordinate in coordinates1 {
            assert_eq!(line_diag1.contains(&coordinate), true);
        }
        let coordinates2 = vec![
            Coordinate { x: 2, y: 0 },
            Coordinate { x: 3, y: 1 },
            Coordinate { x: 4, y: 2 },
            Coordinate { x: 5, y: 3 },
            Coordinate { x: 6, y: 4 },
        ];
        for coordinate in coordinates2 {
            assert_eq!(line_diag2.contains(&coordinate), true);
        }
    }

    #[test]
    fn test_populate_grid() {
        let mut grid: Grid = Grid::new();
        let points = vec![
            Coordinate { x: 1, y: 9 },
            Coordinate { x: 1, y: 9 },
            Coordinate { x: 529, y: 669 },
            Coordinate { x: 529, y: 669 },
            Coordinate { x: 529, y: 669 },
            Coordinate { x: 2, y: 2 },
        ];
        for point in points {
            grid.increment_point(
                usize::try_from(point.x).unwrap(),
                usize::try_from(point.y).unwrap(),
            );
        }
        assert_eq!(grid.num_overlaps(), 2)
    }
    #[test]
    fn test_diag() {
        let straight = [Coordinate { x: 0, y: 9 }, Coordinate { x: 5, y: 9 }];
        assert_eq!(is_diag(&straight), false);
        let diag = [Coordinate { x: 5, y: 10 }, Coordinate { x: 15, y: 19 }];
        assert_eq!(is_diag(&diag), true)
    }
    #[bench]
    fn bench_run(b: &mut Bencher) {
        b.iter(|| run())
    }
}
