
// FIX the errors
fn main() {
    let mut v = vec![1, 2, 3];

    let slice1 = &v[..];
    // Out of bounds will cause a panic
    // You must use `v.len` here
    let slice2 = &v[..v.len()];
    
    assert_eq!(slice1, slice2);
    
    // A slice can also be mutable, in which
    // case mutating it will mutate its underlying Vec.
    // Note: slice and &Vec are different
    let vec_ref: &mut Vec<i32> = &mut v;
    (*vec_ref).push(4);
    let slice3 = &mut v[..v.len()];
    slice3[3] = 42;

    assert_eq!(slice3, &[1, 2, 3, 42]);
    assert_eq!(v, &[1, 2, 3, 42]);

    println!("Success!");
}