//! `Rust` book lessons.
//! Chapter [14/03](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html).
//! Slices utils.

/// Returns the contained first value or computes it from a closure.
/// See: [`Option::unwrap_or_else`]
///
/// # Examples
///
/// ```
/// let slice = [1, 2, 3];
/// let it = slices::get_first_or_else(&slice, || &-1);
/// assert_eq!(1, *it);
///
/// let slice = [];
/// let it = slices::get_first_or_else(&slice, || &-1);
/// assert_eq!(-1, *it);
/// ```
pub fn get_first_or_else<'a, T, F: FnOnce() -> &'a T>(slice: &'a [T], f: F) -> &'a T {
    if slice.is_empty() {
        return f();
    }
    return &slice[0];
    // return slice.get(0).unwrap_or_else(f);
}
