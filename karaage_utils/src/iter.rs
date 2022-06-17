// Copyright (c) 2022 zakuro <z@kuro.red>. All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::array::IntoIter;

pub fn infinity_iter<T: Clone>(v: T) -> std::iter::Cycle<IntoIter<T, 1>> {
    [v].into_iter().cycle()
}

#[cfg(test)]
mod tests {
    use crate::*;
    use karaage_asserts::assert_iter_eq;

    #[test]
    fn test_infinity() {
        assert_iter_eq!(infinity_iter(()).take(4), [(), (), (), ()]);
    }
}
