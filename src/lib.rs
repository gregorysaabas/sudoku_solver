use std::collections::{HashMap, HashSet};

use lazy_static::lazy_static;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Clone)]
struct GridContent {
    grid: [[u8; 9]; 9],
    missing_numbers_cache: HashMap<(usize, usize), Vec<u8>>,
}

impl GridContent {
    fn row(&self, index: usize) -> [u8; 9] {
        self.grid[index]
    }
    fn col(&self, index: usize) -> [u8; 9] {
        let mut column = [0; 9];
        column.iter_mut().enumerate().for_each(|(i, value)| *value = self.grid[i][index]);
        column
    }

    fn sec(&self, row_index: usize, col_index: usize) -> [[u8; 3]; 3] {
        let mut section = [[0; 3]; 3];
        (0..3).for_each(|i| (0..3).for_each(|j| section[i][j] = self.grid[row_index * 3 + i][col_index * 3 + j]));
        section
    }

    fn is_solved(&self) -> bool {
        !self.grid.iter().any(|row| row.iter().any(|&value| value == 0))
    }

    fn find_easy_row(&self) -> Option<(usize, usize, u8)> {
        self.grid.iter().enumerate().find_map(|(row_index, row)| {
            let zero_positions: Vec<usize> = row.iter().enumerate()
                .filter(|&(_, &value)| value == 0)
                .map(|(col_index, _)| col_index)
                .collect();

            if zero_positions.len() == 1 {
                let missing_number = (1..=9).find(|&num| !row.contains(&num));
                missing_number.map(|missing| (row_index, zero_positions[0], missing))
            } else {
                None
            }
        })
    }

    fn find_easy_col(&self) -> Option<(usize, usize, u8)> {
        (0..9).find_map(|col_index| {
            let zero_rows: Vec<usize> = self.grid.iter()
                .enumerate()
                .filter_map(|(row_index, row)| {
                    if row[col_index] == 0 {
                        Some(row_index)
                    } else {
                        None
                    }
                })
                .collect();

            if zero_rows.len() == 1 {
                let missing_number = (1..=9).find(|&num| !self.grid.iter().any(|row| row[col_index] == num));
                missing_number.map(|missing| (zero_rows[0], col_index, missing))
            } else {
                None
            }
        })
    }

    fn find_easy_sec(&self) -> Option<(usize, usize, u8)> {
        for row_index in (0..9).step_by(3) {
            for col_index in (0..9).step_by(3) {
                let mut zero_count = 0;
                let mut zero_row = None;
                let mut zero_col = None;

                for i in 0..3 {
                    for j in 0..3 {
                        if self.grid[row_index + i][col_index + j] == 0 {
                            zero_count += 1;
                            zero_row = Some(row_index + i);
                            zero_col = Some(col_index + j);
                        }
                    }
                }

                if zero_count == 1 {
                    let missing_number = (1..=9).find(|&num| {
                        !(0..3).any(|i| {
                            (0..3).any(|j| self.grid[row_index + i][col_index + j] == num)
                        })
                    });

                    if let Some(missing) = missing_number {
                        return Some((zero_row.unwrap(), zero_col.unwrap(), missing));
                    }
                }
            }
        }
        None
    }

    fn is_valid_sudoku(&self) -> bool {
        // Check if all values are within the range of 0 to 9
        if !self.grid.iter().all(|row| row.iter().all(|&value| value <= 9)) {
            return false;
        }

        // Check each row, column, and sector for validity
        let rows_valid = (0..9).all(|row| Self::is_valid(&self.grid[row]));
        let cols_valid = (0..9).all(|col| Self::is_valid(&(0..9).map(|row| self.grid[row][col]).collect::<Vec<_>>()));
        let sectors_valid = (0..9).all(|sector| {
            let row_start = (sector / 3) * 3;
            let col_start = (sector % 3) * 3;
            Self::is_valid(&(0..3).flat_map(|i| (0..3).map(move |j| self.grid[row_start + i][col_start + j])).collect::<Vec<_>>())
        });

        rows_valid && cols_valid && sectors_valid
    }

    fn is_valid(values: &[u8]) -> bool {
        let mut seen = [false; 10];
        values.iter().all(|&value| {
            if value != 0 && value <= 9 {
                if seen[value as usize] {
                    false // Value repeated
                } else {
                    seen[value as usize] = true;
                    true
                }
            } else {
                true
            }
        })
    }

    fn missing_numbers(&mut self, row: usize, col: usize) -> Vec<u8> {
        // Check if the result is already cached
        if let Some(result) = self.missing_numbers_cache.get(&(row, col)) {
            return result.clone();
        }

        if self.grid[row][col] != 0 {
            self.missing_numbers_cache.insert((row, col), vec![]);
            return vec![];
        }

        // Initialize a set with numbers from 1 to 9
        let mut available_numbers: Vec<u8> = (1..=9).collect();

        // Check the row and remove numbers that are present
        for &value in &self.grid[row] {
            if value != 0 {
                if let Some(pos) = available_numbers.iter().position(|&x| x == value) {
                    available_numbers.remove(pos);
                }
            }
        }

        // Check the column and remove numbers that are present
        for row_index in 0..9 {
            if self.grid[row_index][col] != 0 {
                if let Some(pos) = available_numbers.iter().position(|&x| x == self.grid[row_index][col]) {
                    available_numbers.remove(pos);
                }
            }
        }

        // Check the sector and remove numbers that are present
        let row_start = (row / 3) * 3;
        let col_start = (col / 3) * 3;
        for i in 0..3 {
            for j in 0..3 {
                if self.grid[row_start + i][col_start + j] != 0 {
                    if let Some(pos) = available_numbers.iter().position(|&x| x == self.grid[row_start + i][col_start + j]) {
                        available_numbers.remove(pos);
                    }
                }
            }
        }
        self.missing_numbers_cache.insert((row, col), available_numbers.clone());

        available_numbers
    }
    fn remove_number_from_related_cells(&mut self, row: usize, col: usize, number: u8) {
        self.missing_numbers_cache.insert((row, col), vec![]);
        for i in 0..9 {
            // Remove the number from cells in the same row
            self.missing_numbers_cache
                .entry((row, i))
                .and_modify(|missing_numbers| missing_numbers.retain(|&num| num != number));

            // Remove the number from cells in the same column
            self.missing_numbers_cache
                .entry((i, col))
                .and_modify(|missing_numbers| missing_numbers.retain(|&num| num != number));

            // Calculate the sector coordinates and remove the number from cells in the same sector
            let sector_row = (row / 3) * 3 + i / 3;
            let sector_col = (col / 3) * 3 + i % 3;
            self.missing_numbers_cache
                .entry((sector_row, sector_col))
                .and_modify(|missing_numbers| missing_numbers.retain(|&num| num != number));
        }
    }

    fn unique_missing_numbers(&mut self, row: usize, col: usize) -> Vec<u8> {
        // Get the missing numbers for the current cell
        let missing_numbers = self.missing_numbers(row, col);

        // Create a set of missing numbers for the entire row
        let mut row_missing_numbers: HashSet<u8> = HashSet::new();
        for c in 0..9 {
            if c != col {
                row_missing_numbers.extend(self.missing_numbers(row, c));
            }
        }

        // Create a set of missing numbers for the entire column
        let mut col_missing_numbers: HashSet<u8> = HashSet::new();
        for r in 0..9 {
            if r != row {
                col_missing_numbers.extend(self.missing_numbers(r, col));
            }
        }

        // Create a set of missing numbers for the entire sector
        let row_start = (row / 3) * 3;
        let col_start = (col / 3) * 3;
        let mut sector_missing_numbers: HashSet<u8> = HashSet::new();
        for i in 0..3 {
            for j in 0..3 {
                let r = row_start + i;
                let c = col_start + j;
                if r != row || c != col {
                    sector_missing_numbers.extend(self.missing_numbers(r, c));
                }
            }
        }

        // Filter out numbers that are already present in any other cell's missing numbers
        let unique_missing_numbers: Vec<u8> = missing_numbers
            .into_iter()
            .filter(|&num| {
                [
                    !row_missing_numbers.contains(&num),
                    !col_missing_numbers.contains(&num),
                    !sector_missing_numbers.contains(&num),
                ]
                    .iter()
                    .filter(|&&contains| contains)
                    .count() >= 1
            })
            .collect();

        unique_missing_numbers
    }

    fn find_cell_with_one_missing(&mut self) -> Option<(usize, usize, u8)> {
        for row in 0..9 {
            for col in 0..9 {
                let missing_numbers = self.missing_numbers(row, col);
                if missing_numbers.len() == 1 {
                    return Some((row, col, missing_numbers[0]));
                }
            }
        }
        None
    }

    fn find_cell_with_unique_missing(&mut self) -> Option<(usize, usize, u8)> {
        for row in 0..9 {
            for col in 0..9 {
                let unique_missing_numbers = self.unique_missing_numbers(row, col);
                if !unique_missing_numbers.is_empty() {
                    return Some((row, col, unique_missing_numbers[0]));
                }
            }
        }
        None
    }

    fn solve(&mut self) -> Result<GridContent, String> {
        if !self.is_valid_sudoku() {
            return Err("Not valid".to_owned());
        };
        loop {
            if self.is_solved() {
                break;
            }
            if let Some((row, col, val)) = self.find_easy_row()
                .or_else(|| self.find_easy_col())
                .or_else(|| self.find_easy_sec())
                .or_else(|| self.find_cell_with_one_missing())
                .or_else(|| self.find_cell_with_unique_missing())
            {
                self.grid[row][col] = val;
                self.remove_number_from_related_cells(row, col, val);
                //self.missing_numbers_cache = HashMap::new();
                continue;
            }
            return Err("Cannot find next move".to_owned());
        }

        Ok(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    lazy_static!(

    static ref TEST_GRID: GridContent = GridContent {
        grid
        : [
            [11, 12, 13, 14, 15, 16, 17, 18, 19],
            [21, 22, 23, 24, 25, 26, 27, 28, 29],
            [31, 32, 33, 34, 35, 36, 37, 38, 39],
            [41, 42, 43, 44, 45, 46, 47, 48, 49],
            [51, 52, 53, 54, 55, 56, 57, 58, 59],
            [61, 62, 63, 64, 65, 66, 67, 68, 69],
            [71, 72, 73, 74, 75, 76, 77, 78, 79],
            [81, 82, 83, 84, 85, 86, 87, 88, 89],
            [91, 92, 93, 94, 95, 96, 97, 98, 99],
        ],
        missing_numbers_cache: HashMap::new(),
    };

    static ref EMPTY_GRID: GridContent = GridContent{
        grid
        : [
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
        ],
        missing_numbers_cache: HashMap::new(),
    };

    static ref COMPLETED_GRID: GridContent = GridContent {
        grid
        : [
            [5, 3, 4, 6, 7, 8, 9, 1, 2],
            [6, 7, 2, 1, 9, 5, 3, 4, 8],
            [1, 9, 8, 3, 4, 2, 5, 6, 7],
            [8, 5, 9, 7, 6, 1, 4, 2, 3],
            [4, 2, 6, 8, 5, 3, 7, 9, 1],
            [7, 1, 3, 9, 2, 4, 8, 5, 6],
            [9, 6, 1, 5, 3, 7, 2, 8, 4],
            [2, 8, 7, 4, 1, 9, 6, 3, 5],
            [3, 4, 5, 2, 8, 6, 1, 7, 9],
        ],
        missing_numbers_cache: HashMap::new(),
    };

    static ref INVALID_ROW_GRID: GridContent = GridContent {
        grid
        : [
            [5, 3, 6, 6, 7, 8, 9, 1, 2],
            [0, 7, 2, 1, 9, 5, 3, 4, 8],
            [1, 9, 8, 3, 4, 2, 5, 6, 7],
            [8, 5, 9, 7, 6, 1, 4, 2, 3],
            [4, 2, 0, 8, 5, 3, 7, 9, 1],
            [7, 1, 3, 9, 2, 4, 8, 5, 6],
            [9, 6, 1, 5, 3, 7, 2, 8, 4],
            [2, 8, 7, 4, 1, 9, 6, 3, 5],
            [3, 4, 5, 2, 8, 6, 1, 7, 9],
        ],
        missing_numbers_cache: HashMap::new(),
    };

    static ref INVALID_COL_GRID: GridContent = GridContent {
        grid
        : [
            [5, 3, 4, 6, 7, 8, 9, 1, 2],
            [6, 7, 2, 1, 9, 5, 4, 0, 8],
            [1, 9, 8, 3, 4, 2, 5, 6, 7],
            [8, 5, 9, 7, 6, 1, 4, 2, 3],
            [4, 2, 6, 8, 5, 3, 7, 9, 1],
            [7, 1, 3, 9, 2, 4, 8, 5, 6],
            [9, 6, 1, 5, 3, 7, 2, 8, 4],
            [2, 8, 7, 4, 1, 9, 6, 3, 5],
            [3, 4, 5, 2, 8, 6, 1, 7, 9],
        ],
        missing_numbers_cache: HashMap::new(),
    };

    static ref INVALID_SEC_GRID: GridContent = GridContent {
        grid
        : [
            [5, 3, 4, 6, 7, 8, 9, 1, 2],
            [6, 7, 2, 1, 9, 5, 3, 4, 8],
            [1, 9, 8, 3, 4, 2, 5, 6, 7],
            [8, 0, 9, 5, 6, 1, 4, 2, 3],
            [4, 2, 6, 8, 5, 3, 7, 9, 1],
            [7, 1, 3, 9, 2, 4, 8, 5, 6],
            [9, 6, 1, 0, 3, 7, 2, 8, 4],
            [2, 8, 7, 4, 1, 9, 6, 3, 5],
            [3, 4, 5, 2, 8, 6, 1, 7, 9],
        ],
        missing_numbers_cache: HashMap::new(),
    };

    static ref INVALID_TOO_GREAT_GRID: GridContent = GridContent {
        grid
        : [
            [5, 3, 4, 6, 7, 8, 9, 1, 2],
            [6, 7, 2, 1, 9, 5, 3, 4, 8],
            [1, 9, 8, 3, 4, 2, 5, 6, 7],
            [8, 5, 9, 7, 6, 1, 4, 2, 3],
            [4, 2, 6, 8, 5, 3, 7, 16, 1],
            [7, 1, 3, 9, 2, 4, 8, 5, 6],
            [9, 6, 1, 5, 3, 7, 2, 8, 4],
            [2, 8, 7, 4, 1, 9, 6, 3, 5],
            [3, 4, 5, 2, 8, 6, 1, 7, 9],
        ],
        missing_numbers_cache: HashMap::new(),
    };

    static ref ALMOST_COMPLETED_GRID: GridContent = GridContent {
        grid
        : [
            [5, 3, 4, 6, 7, 8, 9, 1, 2],
            [6, 7, 2, 1, 9, 5, 3, 4, 8],
            [1, 9, 8, 0, 4, 2, 5, 6, 7],
            [8, 5, 9, 7, 6, 1, 4, 2, 3],
            [4, 2, 6, 8, 5, 3, 7, 9, 1],
            [7, 1, 3, 9, 2, 4, 8, 5, 6],
            [9, 6, 1, 5, 3, 7, 2, 8, 4],
            [2, 8, 7, 4, 1, 9, 6, 3, 5],
            [3, 4, 5, 2, 8, 6, 1, 7, 9],
        ],
        missing_numbers_cache: HashMap::new(),
    };

    static ref EASY_ROW_GRID: GridContent = GridContent {
        grid
        : [
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [4, 2, 6, 8, 5, 0, 7, 9, 1],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
        ],
        missing_numbers_cache: HashMap::new(),
    };

    static ref EASY_COL_GRID: GridContent = GridContent {
        grid
        : [
            [0, 0, 0, 0, 0, 0, 0, 1, 0],
            [0, 0, 0, 0, 0, 0, 0, 4, 0],
            [0, 0, 0, 0, 0, 0, 0, 6, 0],
            [0, 0, 0, 0, 0, 0, 0, 2, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 5, 0],
            [0, 0, 0, 0, 0, 0, 0, 8, 0],
            [0, 0, 0, 0, 0, 0, 0, 3, 0],
            [0, 0, 0, 0, 0, 0, 0, 7, 0],
        ],
        missing_numbers_cache: HashMap::new(),
    };

    static ref EASY_SEC_GRID: GridContent = GridContent {
        grid
        : [
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 5, 3, 7, 0, 0, 0],
            [0, 0, 0, 4, 1, 9, 0, 0, 0],
            [0, 0, 0, 0, 8, 6, 0, 0, 0],
        ],
        missing_numbers_cache: HashMap::new(),
    };

        static ref UNIQUE_GRID: GridContent = GridContent{
        grid
        : [
            [0, 0, 0, 0, 0, 0, 1, 2, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 1, 2, 0, 0, 0, 0],
            [1, 0, 0, 0, 0, 0, 0, 0, 0],
            [2, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 2, 0, 0, 0, 0, 0, 0],
            [0, 0, 1, 0, 0, 0, 0, 0, 0],
        ],
        missing_numbers_cache: HashMap::new(),
    };

        static ref FORCE_GRID: GridContent = GridContent{
        grid
        : [
            [0, 0, 3, 0, 0, 0, 0, 0, 0],
            [0, 0, 2, 0, 0, 0, 0, 0, 0],
            [0, 0, 1, 0, 0, 0, 0, 0, 0],
            [0, 4, 0, 0, 0, 0, 0, 0, 0],
            [0, 3, 0, 0, 0, 0, 0, 0, 0],
            [0, 2, 0, 0, 0, 0, 0, 0, 0],
            [3, 6, 0, 0, 0, 0, 0, 0, 0],
            [2, 5, 0, 0, 0, 0, 0, 0, 0],
            [4, 0, 0, 0, 0, 0, 0, 0, 0],
        ],
        missing_numbers_cache: HashMap::new(),
    };

static ref FORCE_GRID_2: GridContent = GridContent{
        grid
        : [
            [4, 8, 3, 7, 1, 6, 5, 9, 2],
            [0, 0, 1, 0, 0, 0, 4, 0, 0],
            [9, 0, 6, 2, 0, 4, 8, 0, 1],
            [8, 0, 7, 0, 0, 0, 6, 0, 5],
            [6, 0, 2, 0, 0, 0, 9, 0, 0],
            [1, 0, 5, 0, 6, 0, 7, 0, 4],
            [3, 0, 4, 6, 0, 2, 1, 0, 9],
            [0, 0, 9, 0, 0, 0, 2, 4, 0],
            [0, 0, 8, 4, 0, 9, 3, 0, 0],
        ],
        missing_numbers_cache: HashMap::new(),
    };

        static ref PUZZLE_1: GridContent = GridContent{
        grid
        : [
            [0, 0, 0, 6, 0, 3, 5, 0, 4],
            [9, 0, 4, 5, 1, 0, 3, 0, 0],
            [0, 3, 0, 0, 0, 2, 0, 0, 0],
            [3, 9, 0, 0, 0, 0, 6, 4, 0],
            [0, 6, 0, 3, 4, 0, 7, 1, 0],
            [0, 4, 2, 0, 0, 0, 0, 3, 8],
            [0, 5, 0, 1, 0, 0, 0, 9, 0],
            [0, 0, 1, 0, 5, 7, 2, 6, 0],
            [0, 0, 9, 2, 0, 4, 8, 0, 0],
        ],
        missing_numbers_cache: HashMap::new(),
    };

        static ref PUZZLE_2: GridContent = GridContent{
        grid
        : [
            [0, 0, 3, 7, 0, 6, 5, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [9, 0, 0, 2, 0, 4, 0, 0, 1],
            [8, 0, 7, 0, 0, 0, 6, 0, 5],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [1, 0, 5, 0, 0, 0, 7, 0, 4],
            [3, 0, 0, 6, 0, 2, 0, 0, 9],
            [0, 0, 9, 0, 0, 0, 0, 0, 0],
            [0, 0, 8, 4, 0, 9, 3, 0, 0],
        ],
        missing_numbers_cache: HashMap::new(),
    };
        );


    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn row_3() {
        assert_eq!(TEST_GRID.row(3),
                   [41, 42, 43, 44, 45, 46, 47, 48, 49]
        );
    }

    #[test]
    fn col_3() {
        assert_eq!(TEST_GRID.col(3),
                   [14, 24, 34, 44, 54, 64, 74, 84, 94]
        );
    }

    #[test]
    fn sec_1x1() {
        assert_eq!(TEST_GRID.sec(1, 1),
                   [
                       [44, 45, 46, ],
                       [54, 55, 56, ],
                       [64, 65, 66, ],
                   ]
        );
    }

    #[test]
    fn is_solved() {
        assert!(COMPLETED_GRID.is_solved());
        assert!(!EMPTY_GRID.is_solved());
        assert!(!ALMOST_COMPLETED_GRID.is_solved());
    }

    #[test]
    fn is_valid_sudoku() {
        assert!(COMPLETED_GRID.is_valid_sudoku());
        assert!(EMPTY_GRID.is_valid_sudoku());
        assert!(ALMOST_COMPLETED_GRID.is_valid_sudoku());
        assert!(EASY_ROW_GRID.is_valid_sudoku());
        assert!(EASY_COL_GRID.is_valid_sudoku());
        assert!(EASY_SEC_GRID.is_valid_sudoku());
        assert!(!TEST_GRID.is_valid_sudoku());
        assert!(!INVALID_ROW_GRID.is_valid_sudoku());
        assert!(!INVALID_COL_GRID.is_valid_sudoku());
        assert!(!INVALID_SEC_GRID.is_valid_sudoku());
        assert!(!INVALID_TOO_GREAT_GRID.is_valid_sudoku());
    }

    #[test]
    fn find_easy_row() {
        assert_eq!(COMPLETED_GRID.find_easy_row(), None);
        assert_eq!(EMPTY_GRID.find_easy_row(), None);
        assert_eq!(ALMOST_COMPLETED_GRID.find_easy_row(), Some((2, 3, 3)));
        assert_eq!(EASY_ROW_GRID.find_easy_row(), Some((5, 5, 3)));
        assert_eq!(EASY_COL_GRID.find_easy_row(), None);
        assert_eq!(EASY_SEC_GRID.find_easy_row(), None);
    }

    #[test]
    fn find_easy_col() {
        assert_eq!(COMPLETED_GRID.find_easy_col(), None);
        assert_eq!(EMPTY_GRID.find_easy_col(), None);
        assert_eq!(ALMOST_COMPLETED_GRID.find_easy_col(), Some((2, 3, 3)));
        assert_eq!(EASY_ROW_GRID.find_easy_col(), None);
        assert_eq!(EASY_COL_GRID.find_easy_col(), Some((4, 7, 9)));
        assert_eq!(EASY_SEC_GRID.find_easy_col(), None);
    }

    #[test]
    fn find_easy_sec() {
        assert_eq!(COMPLETED_GRID.find_easy_sec(), None);
        assert_eq!(EMPTY_GRID.find_easy_sec(), None);
        assert_eq!(ALMOST_COMPLETED_GRID.find_easy_sec(), Some((2, 3, 3)));
        assert_eq!(EASY_ROW_GRID.find_easy_sec(), None);
        assert_eq!(EASY_COL_GRID.find_easy_sec(), None);
        assert_eq!(EASY_SEC_GRID.find_easy_sec(), Some((8, 3, 2)));
    }

    #[test]
    fn missing_numbers() {
        assert_eq!(COMPLETED_GRID.clone().missing_numbers(0, 0), []);
        assert_eq!(EMPTY_GRID.clone().missing_numbers(0, 0), [1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(ALMOST_COMPLETED_GRID.clone().missing_numbers(2, 3), [3]);
        assert_eq!(ALMOST_COMPLETED_GRID.clone().missing_numbers(1, 3), []);
        assert_eq!(EASY_ROW_GRID.clone().missing_numbers(5, 0), []);
    }

    #[test]
    fn unique_missing_numbers() {
        assert_eq!(UNIQUE_GRID.clone().unique_missing_numbers(1, 1), [1, 2]);
        assert_eq!(FORCE_GRID.clone().unique_missing_numbers(8, 1), [1]);
        assert_eq!(FORCE_GRID_2.clone().unique_missing_numbers(8, 1), [1]);
    }

    #[test]
    fn solve() {
        assert_eq!(ALMOST_COMPLETED_GRID.clone().solve().unwrap().grid, COMPLETED_GRID.grid);
        assert!(EMPTY_GRID.clone().solve().is_err());
        assert!(INVALID_SEC_GRID.clone().solve().is_err());
        assert_eq!(PUZZLE_1.clone().solve().unwrap().grid,
                   [
                       [7, 1, 8, 6, 9, 3, 5, 2, 4],
                       [9, 2, 4, 5, 1, 8, 3, 7, 6],
                       [5, 3, 6, 4, 7, 2, 1, 8, 9],
                       [3, 9, 7, 8, 2, 1, 6, 4, 5],
                       [8, 6, 5, 3, 4, 9, 7, 1, 2],
                       [1, 4, 2, 7, 6, 5, 9, 3, 8],
                       [2, 5, 3, 1, 8, 6, 4, 9, 7],
                       [4, 8, 1, 9, 5, 7, 2, 6, 3],
                       [6, 7, 9, 2, 3, 4, 8, 5, 1],
                   ]
        );
        assert_eq!(PUZZLE_2.clone().solve().unwrap().grid,
                   [
                       [4, 8, 3, 7, 1, 6, 5, 9, 2],
                       [7, 2, 1, 8, 9, 5, 4, 6, 3],
                       [9, 5, 6, 2, 3, 4, 8, 7, 1],
                       [8, 4, 7, 9, 2, 1, 6, 3, 5],
                       [6, 3, 2, 5, 4, 7, 9, 1, 8],
                       [1, 9, 5, 3, 6, 8, 7, 2, 4],
                       [3, 7, 4, 6, 5, 2, 1, 8, 9],
                       [5, 6, 9, 1, 8, 3, 2, 4, 7],
                       [2, 1, 8, 4, 7, 9, 3, 5, 6],
                   ]
        );
    }
}
