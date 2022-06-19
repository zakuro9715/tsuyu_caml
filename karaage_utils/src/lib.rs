// Copyright (c) 2022 zakuro <z@kuro.red>. All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub use derive_more as derives;
pub use paste::paste;

mod option;
pub use option::*;
mod iter;
pub use iter::*;

#[macro_export]
macro_rules! define_with_params_and_init {
    (
        $(
            #[$attr:meta]
        )*
        $vis:vis struct $type:ty {
            $( $field_vis:vis $field_name:ident : $field_type:ty , )+
        }
    ) => {
        $crate::paste! {
            $( #[$attr] )*
            $vis struct $type {
                $(
                    $field_vis $field_name : $field_type,
                )+
            }

            $vis struct [<$type InitParams>]{
                $(
                    pub $field_name : $field_type,
                )+
            }

            impl $type {
                pub fn init(params: [<$type InitParams>]) -> Self {
                    Self {
                        $(
                            $field_name: params.$field_name,
                        )+
                    }
                }
            }
        }
    };
}
