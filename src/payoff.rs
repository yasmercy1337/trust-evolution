#![allow(dead_code)]

#[derive(Clone, Debug)]
pub struct PayoffMatrix {
    data: Vec<Vec<[i8; 2]>>,
    pub row_options: usize,
    pub col_options: usize,
}

impl PayoffMatrix {
    pub fn new(data: Vec<Vec<[i8; 2]>>) -> Self {
        Self {
            row_options: data.len(),
            col_options: data[0].len(),
            data,
        }
    }

    pub fn transpose(self) -> Self {
        Self {
            row_options: self.row_options,
            col_options: self.col_options,
            data: self.data
                .into_iter()
                .map(|row| row.iter().map(|&[x, y]| [y, x]).collect())
                .collect(),
        }
    }

    pub fn new_zero_sum(row_data: &Vec<Vec<i8>>) -> Self {
        Self {
            row_options: row_data.len(),
            col_options: row_data[0].len(),
            data: row_data
                .iter()
                .map(|row| row.iter().map(|&square| [square, -square]).collect())
                .collect(),
        }
    }

    pub fn get_row_payout(&self, row_choice: usize, col_choice: usize) -> i8 {
        self.data[row_choice][col_choice][0]
    }

    pub fn get_col_payout(&self, row_choice: usize, col_choice: usize) -> i8 {
        self.data[row_choice][col_choice][1]
    }

    pub fn data(&self) -> Vec<f64> {
        self.data
            .iter()
            .flat_map(|a| a.iter().flat_map(|&[x, y]| [x as f64, y as f64]))
            .collect()
    }
}
