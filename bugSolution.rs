fn main() {
    let mut v = vec![1, 2, 3];
    for i in v.iter_mut() {
        *i = *i * 2;
    }
    println!("v: {:?}", v);
} 