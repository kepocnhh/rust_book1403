#[test]
fn test_filled_slice() {
    let first = 42;
    let slice = [first, 2, 3];
    let it = slices::get_first_or_else(&slice, || &-1);
    assert_eq!(first, *it);
}

#[test]
fn test_singleton_slice() {
    let first = 42;
    let slice = [first];
    let it = slices::get_first_or_else(&slice, || &-1);
    assert_eq!(first, *it);
}

#[test]
fn test_empty_slice() {
    let computes = 42;
    let slice = [];
    let it = slices::get_first_or_else(&slice, || &computes);
    assert_eq!(computes, *it);
}
