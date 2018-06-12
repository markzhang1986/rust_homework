pub type Matrix = Vec<Vec<f32>>;

pub fn mat_mult(mat1: &Matrix, mat2: &Matrix) -> Matrix {
    assert!(mat1[0].len() == mat2.len()); 

    let mut res = Matrix::new();

    for i in 0..mat1.len() {
        let mut linetmp: Vec<f32> = Vec::new();
        for j in 0..mat2[0].len() {
            let mut tmp:f32 = 0.0;
            for k in 0..mat1[i].len() {
                tmp += mat1[i][k] * mat2[k][j];
            }
            linetmp.push(tmp);
        }
        res.push(linetmp);
    }

    res
}

