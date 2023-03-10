fn main() {
    println!("Hello Rust book 14/03!");

    let first = rand::random::<i8>();
    let slice = [first, 2, 3];
    let it = slices::get_first_or_else(&slice, || &-1);
    assert_eq!(first, *it);
    println!("first value of {slice:?} is {it}");

    let computes = rand::random::<i8>();
    let slice = [];
    let it = slices::get_first_or_else(&slice, || &computes);
    assert_eq!(computes, *it);
    println!("first value of {slice:?} is {it}");
}
