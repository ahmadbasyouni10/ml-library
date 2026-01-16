use rand::Rng;

#[derive(Debug, Clone)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<f32>,
}

impl Matrix { 
    pub fn zeros(rows: usize, cols: usize) -> Self {
        Matrix {
            rows,
            cols,
            data: vec![0.0; rows * cols],
        }
    }

    pub fn from_vec(rows: usize, cols: usize, data: Vec<f32>) -> Self {
        assert_eq!(data.len(), rows * cols, "Data length does not match matrix dimensions");
        Matrix { rows, cols, data }
    }

    pub fn get(&self, row: usize, col: usize) -> f32 {
        assert!(row < self.rows && col < self.cols, "Index out of bounds");
        self.data[row * self.cols + col]
    }

    pub fn set(&mut self, row: usize, col: usize, value: f32) {
        assert!(row < self.rows && col < self.cols, "Index out of bounds");
        self.data[row * self.cols + col] = value;
    }

    pub fn random(rows: usize, cols: usize, min: f32, max: f32) -> Self {
        let mut rng = rand::thread_rng();
        let data: Vec<f32> = (0..rows * cols)
            .map(|_| {
                let x = rng.gen_range(min..max);
                (x*100.0).round() / 100.0
            })
            .collect();
        Matrix { rows, cols, data }
    }

    pub fn add(&self, other: &Matrix) -> Result<Matrix, String> {
        if self.rows != other.rows || self.cols != other.cols {
            return Err("Matrix dimensions must match for addition".to_string());
        }
        let data: Vec<f32> = self.data.iter().zip(other.data.iter())
            .map(|(a, b) | a + b)
            .collect();
        Ok(Matrix { rows: self.rows, cols:self.cols, data })
    }

    pub fn scale(&self, factor: f32) -> Matrix {
        let data: Vec<f32> = self.data.iter()
            .map(|x| x * factor)
            .collect();
        Matrix { rows: self.rows, cols: self.cols, data }
    }

    pub fn map<F>(&self, func: F) -> Matrix
    where 
        F: Fn(f32) -> f32,
    {
        let data: Vec<f32> = self.data.iter().map(|&x| func(x)).collect();
        Matrix { rows: self.rows, cols: self.cols, data }
    }
}