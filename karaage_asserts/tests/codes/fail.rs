use karaage_asserts::*;

struct Empty {}

fn main() {
    assert_impl!(Empty<Clone, Copy>);
    assert_iter_eq!(Vec::<usize>::new(), Vec::<String>::new());
    assert_iter_ne!(Vec::<usize>::new(), Vec::<String>::new());
}
