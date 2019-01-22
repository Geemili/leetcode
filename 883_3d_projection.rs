// Estimated time: 20 mins
// Actual time: 13 mins
// Completed: 2018-01-22

impl Solution {
    pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
        project_area_xy(&grid) + project_area_xz(&grid) + project_area_yz(&grid)
    }
}

fn project_area_xy(grid: &Vec<Vec<i32>>) -> i32 {
    let mut area = 0;
    for row in grid {
        for col in row {
            if *col > 0 {
                area += 1;
            }
        }
    }
    area
}

fn project_area_xz(grid: &Vec<Vec<i32>>) -> i32 {
    let mut area = 0;
    for i in 0..grid.len() {
        let mut tallest = 0;
        for j in 0..grid.len() {
            tallest = grid[i][j].max(tallest);
        }
        area += tallest;
    }
    area
}

fn project_area_yz(grid: &Vec<Vec<i32>>) -> i32 {
    let mut area = 0;
    for j in 0..grid.len() {
        let mut tallest = 0;
        for i in 0..grid.len() {
            tallest = grid[i][j].max(tallest);
        }
        area += tallest;
    }
    area
}

#[cfg(test)]
use test::*;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        let input = vec![vec![2]];
        let expected = 5;
        assert_eq!(expected, Solution::projection_area(input));
    }

    #[test]
    fn example_2() {
        let input = vec![vec![1, 2], vec![3, 4]];
        let expected = 17;
        assert_eq!(expected, Solution::projection_area(input));
    }

    #[test]
    fn example_3() {
        let input = vec![vec![1, 0], vec![0, 2]];
        let expected = 8;
        assert_eq!(expected, Solution::projection_area(input));
    }

    #[test]
    fn example_4() {
        let input = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        let expected = 14;
        assert_eq!(expected, Solution::projection_area(input));
    }

    #[test]
    fn example_5() {
        let input = vec![vec![2, 2, 2], vec![2, 1, 2], vec![2, 2, 2]];
        let expected = 21;
        assert_eq!(expected, Solution::projection_area(input));
    }

    // Definitions
    pub struct Solution;
}
