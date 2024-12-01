use crate::direction::Direction;

/// I'm too lazy to pick a third party matrix library from the shelf
pub struct Matrix<T> {
    pub values: Vec<T>,
    pub n_rows: usize,
    pub n_cols: usize,
}

impl<T: Copy> Matrix<T> {
    pub fn new(n_rows: usize, n_cols: usize, initial_value: T) -> Self {
        let size = n_rows * n_cols;
        let mut values = Vec::with_capacity(size);
        for _ in 0..size {
            values.push(initial_value);
        }
        Self {
            values,
            n_rows,
            n_cols,
        }
    }

    pub fn assign_row(&mut self, row_index: usize, value: T) {
        let n_cols = self.n_cols;

        for idx in n_cols * row_index..n_cols * (row_index + 1) {
            self.values[idx] = value;
        }
    }

    pub fn assign_col(&mut self, col_index: usize, value: T) {
        for row_index in 0..self.n_rows {
            let idx = row_index * self.n_cols + col_index;
            self.values[idx] = value;
        }
    }

    pub fn get(&self, x: usize, y: usize, c_like_indexing: bool) -> T {
        self.values[self.coords(x, y, c_like_indexing)]
    }

    pub fn set(&mut self, value: T, x: usize, y: usize, c_like_indexing: bool) {
        let idx = self.coords(x, y, c_like_indexing);
        self.values[idx] = value;
    }

    fn coords(&self, x: usize, y: usize, c_like_indexing: bool) -> usize {
        if c_like_indexing {
            self.n_cols * y + x
        } else {
            // fortran-like indexing
            self.n_rows * x + y
        }
    }

    pub fn neighbours(&self, mut x: usize, mut y: usize, direction: Direction) -> Vec<T> {
        let mut result = vec![];
        match direction {
            Direction::North => {
                if y == 0 {
                    return vec![];
                }
                loop {
                    y -= 1;
                    result.push(self.get(x, y, true));
                    if y == 0 {
                        break;
                    }
                }
            }
            Direction::South => {
                if y + 1 >= self.n_rows {
                    return vec![];
                }
                loop {
                    y += 1;
                    result.push(self.get(x, y, true));
                    if y + 1 == self.n_rows {
                        break;
                    }
                }
            }
            Direction::East => {
                if x + 1 >= self.n_cols {
                    return vec![];
                }
                loop {
                    x += 1;
                    result.push(self.get(x, y, true));
                    if x + 1 >= self.n_cols {
                        break;
                    }
                }
            }
            Direction::West => {
                if x == 0 {
                    return vec![];
                }
                loop {
                    x -= 1;
                    result.push(self.get(x, y, true));
                    if x == 0 {
                        break;
                    }
                }
            }
        }
        result
    }
}
