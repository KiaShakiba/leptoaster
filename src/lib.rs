/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

mod toast;
mod toaster;

pub use crate::{
    toast::{ToastBuilder, ToastLevel, ToastPosition},
    toaster::{expect_toaster, provide_toaster, Toaster},
};

pub fn demo() {
    let toaster = crate::expect_toaster();

    toaster.toast(
        crate::ToastBuilder::new("My toast message.")
            .with_level(ToastLevel::Info)
            .with_position(ToastPosition::TopRight),
    );
}
