use std::fmt;

struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "( {} {} )", self.0, self.1)?;
        write!(f, "( {} {} )", self.2, self.3)
    }
}

fn transpose(matrix: Matrix) -> Matrix {
    // Solution 1
    // let Matrix(one, two, three, four) = matrix;
    // Matrix(one, three, two, four)

    // Solution 2
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}

fn main() {
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{}", matrix);
    println!("{}", transpose(matrix));
}
