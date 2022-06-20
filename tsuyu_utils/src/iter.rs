// Copyright (c) 2022 zakuro <z@kuro.red>. All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub struct InfinityIterator<T: Clone> {
    v: T,
}

impl<T: Clone> InfinityIterator<T> {
    fn new(v: T) -> Self {
        Self { v }
    }
}

impl<T: Clone> Iterator for InfinityIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.v.clone())
    }
}

pub fn infinity_iter<T: Clone>(v: T) -> InfinityIterator<T> {
    InfinityIterator::new(v)
}

#[cfg(test)]
mod tests {
    use crate::*;
    use tsuyu_asserts::{assert_eq, assert_impls, assert_iter_eq};

    #[test]
    fn test_infinity_iterator() {
        assert_eq!(std::mem::size_of::<InfinityIterator<()>>(), 0);
        assert_eq!(std::mem::size_of::<InfinityIterator<u32>>(), 4);
        assert_eq!(
            std::mem::size_of::<InfinityIterator<String>>(),
            std::mem::size_of::<String>(),
        );
        assert_impls!(InfinityIterator<()>: Iterator & IntoIterator & Sized);
    }

    #[test]
    fn test_infinity() {
        assert_iter_eq!(infinity_iter(()).take(4), [(), (), (), ()]);
    }
}
