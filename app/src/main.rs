fn main() {
    println!("Hello Rust book 14/03!");

    let slice = [1, 2, 3];
    let it = slices::get_first_or_else(&slice, || &-1);
    assert_eq!(1, *it);
    println!("first value of {slice:?} is {it}");

    let slice = [];
    let it = slices::get_first_or_else(&slice, || &-1);
    assert_eq!(-1, *it);
    println!("first value of {slice:?} is {it}");
}
