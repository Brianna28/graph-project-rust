use nalgebra::DMatrix;

fn logical_or(mat1: &DMatrix<usize>, mat2: &DMatrix<usize>) -> DMatrix<usize> {
    // Ensure the input matays have the same shape

    // Perform element-wise logical_or operation
    let result = mat1.zip_map(mat2, |a, b| (a > 0 || b > 0) as usize);

    result
}


fn all_non_zero(m: &DMatrix<usize>)-> bool{
    for i in m.iter(){
        if (*i) == 0 {
            return false
        }
    }
    return true
}

pub fn diameter(mat: DMatrix<usize>) -> u32{
    let mut mat_clone = mat.clone();
    let iden = DMatrix::<usize>::identity(mat_clone.nrows(),mat_clone.nrows());
    mat_clone += iden;
    let mut t: u32 = 1;
    for _ in 0..1000{
        let temp_mat = mat_clone.pow(t);
        if all_non_zero(&temp_mat){
            return t;
        }
        t += 1;
        if t>20{
            break
        }
        //mat_clone = mat_clone.map(|x| if x > 1 {1} else {x});
        mat_clone = logical_or(&mat_clone, &temp_mat)
    }
    return 1000;

}