use std::fmt::Debug;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

#[derive(Clone, Debug)]
pub struct Matrix<T> {
    width: usize,
    height: usize,
    values: Vec<T>, // laid out as values[y][x]
}

impl<T> Matrix<T>
where
    T: Default,
{
    pub fn new(height: usize, width: usize) -> Self {
        let mut values = Vec::new();
        values.resize_with(width * height, Default::default);

        Self {
            height,
            width,
            values,
        }
    }
}

impl<T> Matrix<T> {
    pub fn new_map<F>(height: usize, width: usize, mut func: F) -> Self
    where
        F: FnMut(usize, usize) -> T,
    {
        let mut options_matrix: Matrix<Option<T>> = Matrix::new(height, width);

        for row in 0..height {
            for col in 0..width {
                options_matrix[row][col] = Some(func(row, col))
            }
        }

        let result_values = options_matrix
            .values
            .into_iter()
            .map(|option_value: Option<T>| option_value.unwrap())
            .collect::<Vec<T>>();

        Self {
            height,
            width,
            values: result_values,
        }
    }

    pub fn map<F>(mut self, func: F) -> Self
    where
        F: Fn(T) -> T,
    {
        self.values = self.values.into_iter().map(|val: T| func(val)).collect();

        self
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }
}

impl<T> Matrix<T>
where
    T: Debug,
{
    pub fn print(&self) {
        println!("[");
        for row in 0..self.height {
            print!("\t[");
            for col in 0..self.width - 1 {
                print!("{:?} ", self[row][col]);
            }

            println!("{:?}]", self[row][self.width - 1]);
        }
        println!("]");
    }
}

impl<T> Index<usize> for Matrix<T> {
    type Output = [T];

    fn index(&self, row: usize) -> &Self::Output {
        if row >= self.height {
            panic!(
                "Indexing with row {} is invalid for matrix with height of {}",
                row, self.height
            );
        }

        &self.values[(row * self.width)..((row + 1) * self.width)]
    }
}

impl<T> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        if row >= self.height {
            panic!(
                "Indexing with row {} is invalid for matrix with height of {}",
                row, self.height
            );
        }

        &mut self.values[(row * self.width)..((row + 1) * self.width)]
    }
}

impl<T> Add<Matrix<T>> for Matrix<T>
where
    T: Add<T, Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.width != rhs.width {
            panic!(
                "lhs width is {} and rhs width is {}, which is an invalid combination",
                self.width, rhs.width
            );
        }

        if self.height != rhs.height {
            panic!(
                "lhs height is {} and rhs height is {}, which is an invalid combination",
                self.height, rhs.height
            );
        }

        let values = self
            .values
            .into_iter()
            .zip(rhs.values.into_iter())
            .map(|(lhs_value, rhs_value): (T, T)| lhs_value + rhs_value)
            .collect::<Vec<T>>();

        Self {
            height: self.height,
            width: self.width,
            values,
        }
    }
}

impl<'a, T> Add<&'a Matrix<T>> for &'a Matrix<T>
where
    T: Add<T, Output = T> + Copy,
{
    type Output = Matrix<T>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.width != rhs.width {
            panic!(
                "lhs width is {} and rhs width is {}, which is an invalid combination",
                self.width, rhs.width
            );
        }

        if self.height != rhs.height {
            panic!(
                "lhs height is {} and rhs height is {}, which is an invalid combination",
                self.height, rhs.height
            );
        }

        Matrix::new_map(self.height, self.width, |row: usize, col: usize| {
            self[row][col] + rhs[row][col]
        })
    }
}

impl<T> AddAssign<Matrix<T>> for Matrix<T>
where
    T: AddAssign<T>,
{
    fn add_assign(&mut self, rhs: Self) {
        if self.width != rhs.width {
            panic!(
                "lhs width is {} and rhs width is {}, which is an invalid combination",
                self.width, rhs.width
            );
        }

        if self.height != rhs.height {
            panic!(
                "lhs height is {} and rhs height is {}, which is an invalid combination",
                self.height, rhs.height
            );
        }

        self.values
            .iter_mut()
            .zip(rhs.values.into_iter())
            .for_each(|(lhs_mut_ref, rhs_value): (&mut T, T)| *lhs_mut_ref += rhs_value);
    }
}

impl<T> AddAssign<&Matrix<T>> for Matrix<T>
where
    T: AddAssign<T> + Copy,
{
    fn add_assign(&mut self, rhs: &Matrix<T>) {
        if self.width != rhs.width {
            panic!(
                "lhs width is {} and rhs width is {}, which is an invalid combination",
                self.width, rhs.width
            );
        }

        if self.height != rhs.height {
            panic!(
                "lhs height is {} and rhs height is {}, which is an invalid combination",
                self.height, rhs.height
            );
        }

        self.values
            .iter_mut()
            .zip(rhs.values.iter().copied())
            .for_each(|(lhs_mut_ref, rhs_value): (&mut T, T)| *lhs_mut_ref += rhs_value);
    }
}

impl<T> Sub<Matrix<T>> for Matrix<T>
where
    T: Sub<T, Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.width != rhs.width {
            panic!(
                "lhs width is {} and rhs width is {}, which is an invalid combination",
                self.width, rhs.width
            );
        }

        if self.height != rhs.height {
            panic!(
                "lhs height is {} and rhs height is {}, which is an invalid combination",
                self.height, rhs.height
            );
        }

        let values = self
            .values
            .into_iter()
            .zip(rhs.values.into_iter())
            .map(|(lhs_value, rhs_value): (T, T)| lhs_value - rhs_value)
            .collect::<Vec<T>>();

        Self {
            height: self.height,
            width: self.width,
            values,
        }
    }
}

impl<'a, T> Sub<&'a Matrix<T>> for &'a Matrix<T>
where
    T: Sub<T, Output = T> + Copy,
{
    type Output = Matrix<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.width != rhs.width {
            panic!(
                "lhs width is {} and rhs width is {}, which is an invalid combination",
                self.width, rhs.width
            );
        }

        if self.height != rhs.height {
            panic!(
                "lhs height is {} and rhs height is {}, which is an invalid combination",
                self.height, rhs.height
            );
        }

        Matrix::new_map(self.height, self.width, |row: usize, col: usize| {
            self[row][col] - rhs[row][col]
        })
    }
}

impl<T> SubAssign<Matrix<T>> for Matrix<T>
where
    T: SubAssign<T>,
{
    fn sub_assign(&mut self, rhs: Self) {
        if self.width != rhs.width {
            panic!(
                "lhs width is {} and rhs width is {}, which is an invalid combination",
                self.width, rhs.width
            );
        }

        if self.height != rhs.height {
            panic!(
                "lhs height is {} and rhs height is {}, which is an invalid combination",
                self.height, rhs.height
            );
        }

        self.values
            .iter_mut()
            .zip(rhs.values.into_iter())
            .for_each(|(lhs_mut_ref, rhs_value): (&mut T, T)| *lhs_mut_ref -= rhs_value);
    }
}

impl<T> SubAssign<&Matrix<T>> for Matrix<T>
where
    T: SubAssign<T> + Copy,
{
    fn sub_assign(&mut self, rhs: &Matrix<T>) {
        if self.width != rhs.width {
            panic!(
                "lhs width is {} and rhs width is {}, which is an invalid combination",
                self.width, rhs.width
            );
        }

        if self.height != rhs.height {
            panic!(
                "lhs height is {} and rhs height is {}, which is an invalid combination",
                self.height, rhs.height
            );
        }

        self.values
            .iter_mut()
            .zip(rhs.values.iter().copied())
            .for_each(|(lhs_mut_ref, rhs_value): (&mut T, T)| *lhs_mut_ref -= rhs_value);
    }
}

impl<T> Mul<Matrix<T>> for Matrix<T>
where
    T: Mul<T, Output = T> + Add<T, Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.width != rhs.height {
            panic!(
                "lhs width is {} and rhs height is {}, which is an invalid combination",
                self.width, rhs.height
            );
        }

        Matrix::new_map(self.height, rhs.width, |row, col| {
            let mut values_iter = (0..self.width).map(|i| self[row][i] * rhs[i][col]);

            let first_item = values_iter.next().unwrap();
            values_iter.fold(first_item, |sum: T, val: T| sum + val)
        })
    }
}

impl<'a, T> Mul<&'a Matrix<T>> for &'a Matrix<T>
where
    T: Mul<T, Output = T> + Add<T, Output = T> + Copy,
{
    type Output = Matrix<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.width != rhs.height {
            panic!(
                "lhs width is {} and rhs height is {}, which is an invalid combination",
                self.width, rhs.height
            );
        }

        Matrix::new_map(self.height, rhs.width, |row, col| {
            let mut values_iter = (0..self.width).map(|i| self[row][i] * rhs[i][col]);

            let first_item = values_iter.next().unwrap();
            values_iter.fold(first_item, |sum: T, val: T| sum + val)
        })
    }
}

impl<T> Mul<T> for Matrix<T>
where
    T: Mul<T, Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        &self * rhs
    }
}

impl<T> Mul<T> for &Matrix<T>
where
    T: Mul<T, Output = T> + Copy,
{
    type Output = Matrix<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Matrix::new_map(self.height, self.width, |row, col| self[row][col] * rhs)
    }
}

impl<T> MulAssign<T> for Matrix<T>
where
    T: MulAssign<T> + Copy,
{
    fn mul_assign(&mut self, rhs: T) {
        self.values
            .iter_mut()
            .for_each(|lhs_mut_ref: &mut T| *lhs_mut_ref *= rhs);
    }
}

impl<T> Div<T> for Matrix<T>
where
    T: Div<T, Output = T> + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        &self / rhs
    }
}

impl<T> Div<T> for &Matrix<T>
where
    T: Div<T, Output = T> + Copy,
{
    type Output = Matrix<T>;

    fn div(self, rhs: T) -> Self::Output {
        Matrix::new_map(self.height, self.width, |row, col| self[row][col] / rhs)
    }
}

impl<T> DivAssign<T> for Matrix<T>
where
    T: DivAssign<T> + Clone,
{
    fn div_assign(&mut self, rhs: T) {
        self.values
            .iter_mut()
            .for_each(|lhs_mut_ref: &mut T| *lhs_mut_ref /= rhs.clone());
    }
}
