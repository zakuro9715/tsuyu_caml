use karaage_asserts::*;

struct Empty {}

fn main() {
    assert_impl!(Empty<Clone, Copy>);
}
