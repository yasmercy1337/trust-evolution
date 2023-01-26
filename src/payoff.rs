#![allow(dead_code)]

pub struct PayoffMatrix {
    data: Vec<Vec<(i8, i8)>>,
}

impl PayoffMatrix {
    fn new_zero_sum(row_data: &Vec<Vec<i8>>) -> Self {
        Self {
            data: row_data
                .iter()
                .map(|row| row.iter().map(|&square| (square, -square)).collect())
                .collect(),
        }
    }

    fn get_row_payout(&self, row_choice: usize, col_choice: usize) -> i8 {
        self.data[row_choice][col_choice].0
    }

    fn get_col_payout(&self, row_choice: usize, col_choice: usize) -> i8 {
        self.data[row_choice][col_choice].1
    }
}
