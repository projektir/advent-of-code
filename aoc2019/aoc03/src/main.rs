use std::char;

use anyhow::Result;

fn main() -> Result<()> {
    //Restructure code
    let input = std::fs::read_to_string("aoc03/input.txt")?;

    let max_length = 20000;
    let max_height = 20000;

    let center_x = max_length / 2;
    let center_y = max_length / 2;

    let mut grid: Vec<Vec<char>> = vec![vec!['.'; max_length]; max_height];
    
    let mut intersections: Vec<(usize, usize)> = Vec::new();

    let mut wire_number = 0;

    for wire in input.lines() {
        let grid_char = char::from_digit(wire_number, 10).unwrap();

        let first_wire: Vec<&str> = wire
            .split(",")
            .collect();

        let mut index_x = center_x;
        let mut index_y = center_y;

        grid[index_x][index_y] = 'O';

        for wire_path in first_wire {

            let direction = wire_path.chars().nth(0).unwrap();
            let length = wire_path.chars().skip(1).collect::<String>().parse::<u64>()?;

            match direction {
                'R' => {
                    for l in 0..length {
                        index_x += 1;

                        //if grid[index_x][index_y] != '.' {
                        if grid[index_x][index_y] != grid_char && grid[index_x][index_y] != '.' {
                            //grid[index_x][index_y] = 'X';
                            intersections.push((index_x, index_y));
                        } else {
                            //grid[index_x][index_y] = '-';
                            grid[index_x][index_y] = grid_char;
                        }
                    }
                },
                'U' => {
                    for l in 0..length {
                        index_y += 1;

                        //if grid[index_x][index_y] != '.' {
                        if grid[index_x][index_y] != grid_char && grid[index_x][index_y] != '.' {
                            //grid[index_x][index_y] = 'X';
                            intersections.push((index_x, index_y));
                        } else {
                            //grid[index_x][index_y] = '|';
                            grid[index_x][index_y] = grid_char;
                        }
                    }
                },
                'L' => {
                    for l in 0..length {
                        index_x -= 1;

                        //if grid[index_x][index_y] != '.' {
                        if grid[index_x][index_y] != grid_char && grid[index_x][index_y] != '.' {
                            //grid[index_x][index_y] = 'X';
                            intersections.push((index_x, index_y));
                        } else {
                            //grid[index_x][index_y] = '-';
                            grid[index_x][index_y] = grid_char;
                        }
                    }
                },
                'D' => {
                    for l in 0..length {
                        index_y -= 1;

                        //if grid[index_x][index_y] != '.' {
                        if grid[index_x][index_y] != grid_char && grid[index_x][index_y] != '.' {
                            //grid[index_x][index_y] = 'X';
                            intersections.push((index_x, index_y));
                        } else {
                            //grid[index_x][index_y] = '|';
                            grid[index_x][index_y] = grid_char;
                        }
                    }
                },
                _ => {
                    panic!("Oh no.");
                }
            }

            //grid[index_x][index_y] = '+';
        }

        wire_number += 1;
    }

    //print_grid(grid, max_height, max_length);

    part1(intersections, center_x as i32, center_y as i32);
    //part2(input, intersections, center_x as i32, center_y as i32, grid);

    Ok(())
}

fn part1(intersections: Vec<(usize, usize)>, center_x: i32, center_y: i32) {
    let mut closest_intersection = intersections[0];
    for intersection in intersections {
        let (x1, y1) = closest_intersection;
        let (x2, y2) = intersection;

        if taxicab_distance(center_x as i32, center_y as i32, x2 as i32, y2 as i32) <
           taxicab_distance(center_x as i32, center_y as i32, x1 as i32, y1 as i32) {
            closest_intersection = intersection;
        }
    }

    println!("Closest intersection: ({}, {}), distance: {}",
        closest_intersection.0, closest_intersection.1,
        taxicab_distance(center_x as i32, center_y as i32, closest_intersection.0 as i32, closest_intersection.1 as i32));
}

/*
fn part2(input: &str, intersections: Vec<(usize, usize)>, center_x: i32, center_y: i32, grid: Vec<Vec<char>>) {
    for wire in input.lines() {
        let first_wire: Vec<&str> = wire
            .split(",")
            .collect();

        let mut index_x = center_x;
        let mut index_y = center_y;

        for wire_path in first_wire {
            let direction = wire_path.chars().nth(0).unwrap();
            let length = wire_path.chars().skip(1).collect::<String>().parse::<u64>()?;

            match direction {
                'R' => {
                    for l in 0..length {
                        index_x += 1;

                        //if grid[index_x][index_y] != '.' {
                        if grid[index_x][index_y] != grid_char && grid[index_x][index_y] != '.' {
                            //grid[index_x][index_y] = 'X';
                            intersections.push((index_x, index_y));
                        } else {
                            //grid[index_x][index_y] = '-';
                            //grid[index_x][index_y] = grid_char;
                        }
                    }
                },
                'U' => {
                    for l in 0..length {
                        index_y += 1;

                        //if grid[index_x][index_y] != '.' {
                        if grid[index_x][index_y] != grid_char && grid[index_x][index_y] != '.' {
                            //grid[index_x][index_y] = 'X';
                            intersections.push((index_x, index_y));
                        } else {
                            //grid[index_x][index_y] = '|';
                            grid[index_x][index_y] = grid_char;
                        }
                    }
                },
                'L' => {
                    for l in 0..length {
                        index_x -= 1;

                        //if grid[index_x][index_y] != '.' {
                        if grid[index_x][index_y] != grid_char && grid[index_x][index_y] != '.' {
                            //grid[index_x][index_y] = 'X';
                            intersections.push((index_x, index_y));
                        } else {
                            //grid[index_x][index_y] = '-';
                            grid[index_x][index_y] = grid_char;
                        }
                    }
                },
                'D' => {
                    for l in 0..length {
                        index_y -= 1;

                        //if grid[index_x][index_y] != '.' {
                        if grid[index_x][index_y] != grid_char && grid[index_x][index_y] != '.' {
                            //grid[index_x][index_y] = 'X';
                            intersections.push((index_x, index_y));
                        } else {
                            //grid[index_x][index_y] = '|';
                            grid[index_x][index_y] = grid_char;
                        }
                    }
                },
                _ => {
                    panic!("Oh no.");
                }
            }

            //grid[index_x][index_y] = '+';
        }

        wire_number += 1;
    }
}
*/

fn print_grid(grid: Vec<Vec<char>>, max_height: usize, max_length: usize) {
    for i in 0..max_height {
        for j in 0..max_length {
            print!("{} ", grid[j][max_length - i - 1]);
        }

        println!();
    }
}

fn taxicab_distance(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    (x2 - x1).abs() + (y2 - y1).abs()
}
