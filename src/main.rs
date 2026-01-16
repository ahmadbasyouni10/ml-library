mod matrix;
use matrix::Matrix;


fn main() {
    let mut m = Matrix::zeros(3, 3);
    m.set(0, 1, 42.0);
    println!("{:?}", m);
    println!("Element at (0, 1): {}", m.get(0, 1));

    let mut m2 = Matrix::from_vec(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
    println!("m2 matrix = {:?}", m2);
    m2 = m2.scale(2.0);
    println!("Scaled Matrix m2 now: {:?}", m2);

    let m3 = Matrix::random(2, 2, 0.0, 1.0);
    println!("m3 matrix = {:?}", m3);

    let m4 = m3.add(&m2);
    match m4 {
        Ok(result) => println!("Addition Result: {:?}", result),
        Err(e) => println!("Error: {}", e),
    }

    let m5 = m2.map(|x| x.sin());
}
