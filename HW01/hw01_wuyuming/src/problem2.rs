/// Represents a matrix in row-major order
pub type Matrix = Vec<Vec<f32>>;

/// Computes the product of the inputs `mat1` and `mat2`.
pub fn mat_mult(mat1: &Matrix, mat2: &Matrix) -> Matrix {
    let row_num = mat1.len();
    let col_num = mat2.len();

    assert_eq!(row_num, col_num);

    let mut ret: Matrix = Vec::new();
    for i in 0 .. mat1.len() {
        let mut row: Vec<f32> = Vec::new();
        for k in 0 .. mat2[0].len() {
            let mut tmp: f32 = 0f32;
            for j in 0 .. mat1[i].len() {
                tmp += mat1[i][j] * mat2[j][k];
            }
            row.push(tmp);
        }
        ret.push(row);
    }
    return ret;
}

