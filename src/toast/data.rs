/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

pub type ToastId = u64;

#[derive(Clone, Debug)]
pub enum ToastLevel {
	Info,
	Success,
	Warn,
	Error,
}

#[derive(Clone, Debug)]
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
	pub expiry: u32,
	pub position: ToastPosition,
}
