// Copyright (c) 2022 zakuro <z@kuro.red>. All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::Loc;

pub type Range = std::ops::Range<Loc>;

pub fn range(start: Loc, end: Loc) -> Range {
    Range { start, end }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range() {
        let (a, b, c) = (Loc::new(1, 1, 2), Loc::new(3, 2, 1), Loc::new(5, 3, 2));
        assert!(range(a.clone(), c.clone()).contains(&b));
        assert!(!range(c.clone(), a.clone()).contains(&b));
        assert!(range(c.clone(), a.clone()).is_empty());

        assert!(range(a.clone(), b.clone()).contains(&a));
        assert!(!range(a.clone(), b.clone()).contains(&b));
    }
}
