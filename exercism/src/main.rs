use std::mem;


fn main() {
    let a: [i32; 10] = [0; 10];

    let slice_of_a = &a;

    println!("{}", a.len());
    println!("{}", slice_of_a.len());
    println!("{}", mem::size_of_val(&a));
    println!("{}", mem::size_of_val(slice_of_a));
    println!("{}", a[0]);
    println!("{}", slice_of_a[0]);
    //analyze_slice();
}
