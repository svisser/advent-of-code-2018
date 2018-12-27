use std::cmp;
use std::collections::HashMap;

type Coordinate = (u32, u32);
type Distance = u32;
type Area = u64;

fn calc_manhattan_distance(c1: &Coordinate, c2: &Coordinate) -> Distance {
    let v0 = cmp::max(c1.0, c2.0) - cmp::min(c1.0, c2.0);
    let v1 = cmp::max(c1.1, c2.1) - cmp::min(c1.1, c2.1);
    v0 + v1
}

fn calc_top_left_corner(coordinates: &Vec<Coordinate>) -> Coordinate {
    let min_x = coordinates.iter().min_by_key(|c| c.0).unwrap().0;
    let min_y = coordinates.iter().min_by_key(|c| c.1).unwrap().1;
    (min_x, min_y)
}

fn calc_bottom_right_corner(coordinates: &Vec<Coordinate>) -> Coordinate {
    let max_x = coordinates.iter().max_by_key(|c| c.0).unwrap().0;
    let max_y = coordinates.iter().max_by_key(|c| c.1).unwrap().1;
    (max_x, max_y)
}

fn decide_for_coordinate(
    coordinates: &Vec<Coordinate>,
    grid_coordinate: &Coordinate,
) -> Option<Coordinate> {
    let n_coordinates = coordinates.len();
    let mut distances: Vec<Distance> = Vec::with_capacity(n_coordinates);
    let mut min_index: usize = usize::max_value();
    let mut min_distance: Distance = Distance::max_value();
    let mut min_distance_count: u64 = 0;
    for (index, coordinate) in coordinates.iter().enumerate() {
        let distance = calc_manhattan_distance(coordinate, grid_coordinate);
        distances.insert(index, distance);
        if distance < min_distance {
            min_index = index;
            min_distance = distance;
            min_distance_count = 1
        } else if distance == min_distance {
            min_distance_count += 1;
        }
    }
    if min_distance_count > 1 {
        return None;
    }
    Some(*coordinates.get(min_index).unwrap())
}

fn is_infinite_area_coordinate(
    coordinate: &Coordinate,
    top_left_corner: &Coordinate,
    bottom_right_corner: &Coordinate,
) -> bool {
    // Not sure this is correct
    let min_x = top_left_corner.0;
    let max_x = bottom_right_corner.0;
    let min_y = top_left_corner.1;
    let max_y = bottom_right_corner.1;
    let c_x = coordinate.0;
    let c_y = coordinate.1;
    c_x == min_x || c_x == max_x || c_y == min_y || c_y == max_y
}

fn calc_coordinate_assignments(
    coordinates: &Vec<Coordinate>,
    top_left_corner: &Coordinate,
    bottom_right_corner: &Coordinate,
) -> HashMap<Coordinate, Area> {
    let min_x = top_left_corner.0;
    let max_x = bottom_right_corner.0;
    let min_y = top_left_corner.1;
    let max_y = bottom_right_corner.1;
    let mut assignments: HashMap<Coordinate, Area> = HashMap::new();
    for x in min_x..(max_x + 1) {
        for y in min_y..(max_y + 1) {
            let grid_coordinate = (x, y);
            let outcome = decide_for_coordinate(coordinates, &grid_coordinate);
            match outcome {
                Some(coordinate) => {
                    if !is_infinite_area_coordinate(
                        &coordinate,
                        top_left_corner,
                        bottom_right_corner,
                    ) {
                        let entry = assignments.entry(coordinate).or_insert(0);
                        *entry += 1;
                    }
                }
                _ => (),
            }
        }
    }
    assignments
}

fn calc_area(coordinates: &Vec<Coordinate>) -> Option<Area> {
    if coordinates.is_empty() {
        return None;
    }

    let top_left_corner: Coordinate = calc_top_left_corner(coordinates);
    let bottom_right_corner: Coordinate = calc_bottom_right_corner(coordinates);
    let coordinate_counts: HashMap<Coordinate, u64> =
        calc_coordinate_assignments(coordinates, &top_left_corner, &bottom_right_corner);
    if coordinate_counts.is_empty() {
        return None;
    }
    Some(*coordinate_counts.values().max().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_coordinates_provided() {
        let input: Vec<Coordinate> = vec![];
        let expected_output: Option<Area> = None; // No coordinate with finite area
        assert_eq!(calc_area(&input), expected_output);
    }

    #[test]
    fn test_one_coordinate_provided() {
        let input: Vec<Coordinate> = vec![(0, 0)];
        let expected_output: Option<Area> = None; // Infinite area
        assert_eq!(calc_area(&input), expected_output);
    }

    #[test]
    fn test_square() {
        let input: Vec<Coordinate> = vec![(0, 0), (1, 1)];
        let expected_output: Option<Area> = None; // Both infinite areas
        assert_eq!(calc_area(&input), expected_output);
    }

    #[test]
    fn test_one_finite_area() {
        // X.X
        // .X.
        // X.X
        let input: Vec<Coordinate> = vec![(0, 0), (2, 2), (0, 2), (2, 0), (1, 1)];
        let expected_output: Option<Area> = Some(1); // Center coordinate
        assert_eq!(calc_area(&input), expected_output);
    }

    #[test]
    fn test_provided_example() {
        let input: Vec<Coordinate> = vec![(1, 1), (1, 6), (8, 3), (3, 4), (5, 5), (8, 9)];
        let expected_output: Option<Area> = Some(17); // (5, 5) coordinate
        assert_eq!(calc_area(&input), expected_output);
    }
}
