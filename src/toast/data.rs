/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use leptos::prelude::*;

pub type ToastId = u64;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ToastLevel {
    Info,
    Success,
    Warn,
    Error,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ToastPosition {
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
}

#[derive(Clone, Debug)]
pub struct ToastData {
    pub id: ToastId,

    pub message: String,

    pub level: ToastLevel,

    pub dismissable: bool,
    pub expiry: Option<u32>,
    pub progress: bool,

    pub position: ToastPosition,

    pub clear_signal: RwSignal<bool>,
}
