pub use paste::paste;

pub fn assert_send<T: Send>() {}
pub fn assert_sync<T: Sync>() {}
pub fn assert_send_sync<T: Send + Sync>() {}

#[macro_export]
macro_rules! fn_test_send {
    ($t:ty) => {
        $crate::paste! {
            #[test]
            fn [<test_ $t:lower _send>]() {
                $crate::assert_send::<$t>();
            }
        }
    };
}

#[macro_export]
macro_rules! fn_test_sync {
    ($t:ty) => {
        $crate::paste! {
            #[test]
            fn [<test_ $t:lower _sync>]() {
                $crate::assert_sync::<$t>();
            }
        }
    };
}

#[macro_export]
macro_rules! fn_test_send_sync {
    ($t:ty) => {
        $crate::paste! {
            #[test]
            fn [<test_ $t:lower _send_sync>]() {
                $crate::assert_send_sync::<$t>();
            }
        }
    };
}

#[cfg(test)]
mod tests {
    fn_test_send!(i32);
    fn_test_sync!(i32);
    fn_test_send_sync!(i32);
}
