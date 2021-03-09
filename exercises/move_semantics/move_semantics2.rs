// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)



fn main() {
    let mut vec0 = Vec::new();
    //let pass = vec0.clone(); //2.

    let mut vec1 = fill_vec(&mut vec0); //ownership of vec0 is taken: 1. fix by only giving reference/address
    //let mut vec1 = fill_vec(&mut vec0); //3.


    //previously vec0  not used after this line

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
 

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: &Vec<i32>) -> Vec<i32> { //receiving address
    let mut vec = vec.clone(); //1. cloning data behind address
    //let mut vec = vec; 3.

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
