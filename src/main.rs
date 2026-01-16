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
    println!("m2 with sin applied {:?}", m5);

    let a = Matrix::from_vec(2, 3, vec![1.0, 2.0, 3.0,
                                                                4.0, 5.0, 6.0]);

    let b = Matrix::from_vec(3, 2, vec![1.0, 2.0, 3.0,
                                                                4.0, 5.0, 6.0]);
    
    let c = a.dot(&b);
    match c {
        Ok(result) => {
            println!("Multiplication result {:?}", result);
            println!("Result Tranposed {:?}", result.transpose());
        }
        Err(e) => println!("Error: {}", e),
    }

    let c2 = a.subtract(&b);
    println!("subtraction of a and b for loss predicted - target {:?}", c2);

    let c3 = a.hadamard(&b);
    println!("hadamard subtraction for backprop, activation times matrix {:?}", c3);

}
