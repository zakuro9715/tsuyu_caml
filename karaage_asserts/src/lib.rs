// Copyright (c) 2022 zakuro <z@kuro.red>. All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
pub use impls::impls;
pub use paste::paste;
pub use pretty_assertions::{assert_eq, assert_ne};

#[macro_export]
macro_rules! assert_iter_eq {
    ($lhs:expr, $rhs:expr $(,)?) => {{
        let lhs = $lhs.into_iter().collect::<Vec<_>>();
        let rhs = $rhs.into_iter().collect::<Vec<_>>();
        if !lhs.is_empty() || rhs.is_empty() {
            for i in 1..(std::cmp::min(lhs.len(), rhs.len())) {
                $crate::assert_eq!(lhs[i], rhs[i]);
            }
            $crate::assert_eq!(lhs.len(), rhs.len());
        } else {
            // By put this line at end, show `Lhs == Rhs` error at last for readable error.
            $crate::assert_eq!(lhs, rhs);
        }
    }};
}

#[macro_export]
macro_rules! assert_iter_ne {
    ($lhs:expr, $rhs:expr) => {
        let lhs = $lhs.into_iter().collect::<Vec<_>>();
        let rhs = $rhs.into_iter().collect::<Vec<_>>();
        $crate::assert_ne!(lhs, rhs);
    };
}

#[test]
fn test_assert_iter_eq() {
    assert_iter_eq!(vec![0, 1, 2], vec![0, 1, 2]);
    assert_iter_eq!(vec![0, 1, 2], 0..=2);
    assert_iter_eq!(vec!["a", "b"], ["a", "b"].map(String::from));
}

#[test]
fn test_assert_iter_ne() {
    assert_iter_ne!(vec![0, 1, 2], vec![0, 1]);
    assert_iter_ne!(vec![0, 1, 2], 0..2);
    assert_iter_ne!(vec!["a", "b", "c"], ["a", "b"].map(String::from));
}

#[macro_export]
macro_rules! assert_impls {
    ($type:ty: $($trait_expr:tt)+) => { assert!($crate::impls!($type: $($trait_expr)+)) }
}

#[macro_export]
macro_rules! fn_test_thread_safe {
    ($( #[ $attr:meta ] )* $t:ty) => {
        $crate::paste! {
            #[test]
            $( #[$attr] )*
            fn [<test_ $t:lower _thread_safe>]() {
                $crate::assert_impls!($t: Send & Sync);
            }
        }
    };
}

#[macro_export]
macro_rules! fn_test_data_traits {
    ($( #[ $attr:meta ] )* $t:ty) => {
        $crate::paste! {
            #[test]
            $( #[$attr] )*
            fn [<test_ $t:lower _data_traits>]() {
                use std::fmt::Debug;
                $crate::assert_impls!($t: Clone & Eq & PartialEq & Debug);
            }
        }
    };
}

#[cfg(test)]
mod tests {
    fn_test_data_traits!(i32);

    struct NoTrait {}

    fn_test_data_traits! {
        #[should_panic]
        NoTrait
    }

    #[test]
    fn test_assert_impls() {
        assert_impls!(i32: Eq)
    }

    #[test]
    #[should_panic]
    fn test_assert_impls_fail() {
        assert_impls!(f32: Eq);
    }

    fn_test_thread_safe!(i32);

    type Rc = std::rc::Rc<i32>;
    fn_test_thread_safe! {
        #[should_panic]
        Rc
    }
}
