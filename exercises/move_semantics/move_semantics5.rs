// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.

// 생명주기는 참조가 만들어지고, 삭제되기까지이다. 

#[test]
fn main() {
    let mut x = 100;
    let y = &mut x;
    *y += 100;

    let z = &mut x;
    *z += 1000;
    assert_eq!(x, 1200);
}
