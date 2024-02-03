/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use crate::toast::data::{
	ToastId,
	ToastLevel,
	ToastPosition,
	ToastData,
};

pub struct ToastBuilder {
	message: String,

	level: ToastLevel,
	expiry: Option<u32>,
	progress: bool,
	position: ToastPosition,
}

/// Builds a toast, allowing for the custimization of toast message,
/// level, expiry, and position.
///
/// The defaults are:
/// * `level`: `ToastLevel::Info`
/// * `expiry`: `2_500`
/// * `progress`: `false`
/// * `position`: `ToastPosition::BottomLeft`
///
/// # Examples
/// ```
/// ToastBuilder::new("My toast message.")
///     .with_level(ToastLevel::Success)
///     .with_expiry(2_500)
///     .with_progress(true)
///     .with_position(ToastPosition::BottomLeft);
/// ```
impl ToastBuilder {
	/// Constructs a new toast builder with the supplied message.
	///
	/// # Examples
	/// ```
	/// let toast = ToastBuilder::new("My toast message.");
	/// ```
	#[must_use]
	pub fn new(message: &str) -> Self {
		ToastBuilder {
			message: message.into(),

			level: ToastLevel::Info,
			expiry: Some(2_500),
			progress: true,
			position: ToastPosition::BottomLeft,
		}
	}

	/// Sets the level of the toast.
	///
	/// # Examples
	/// ```
	/// ToastBuilder::new("My toast message.")
	///     .with_level(ToastLevel::Warn); // sets the level to `warn`.
	/// ```
	#[must_use]
	pub fn with_level(mut self, level: ToastLevel) -> Self {
		self.level = level;
		self
	}

	/// Sets the progress flag of the toast to show or hide the progress bar.
	///
	/// # Examples
	/// ```
	/// ToastBuilder::new("My toast message.")
	///     .with_progress(true); // enables the progress bar.
	/// ```
	#[must_use]
	pub fn with_progress(mut self, progress: bool) -> Self {
		self.progress = progress;
		self
	}

	/// Sets the expiry time of the toast in milliseconds, or disables it on `None`.
	///
	/// # Examples
	/// ```
	/// ToastBuilder::new("My toast message.")
	///     .with_expiry(Some(1_500)); // sets the expiry time to `1500ms`.
	/// ```
	#[must_use]
	pub fn with_expiry(mut self, expiry: Option<u32>) -> Self {
		self.expiry = expiry;
		self
	}

	/// Sets the position of the toast.
	///
	/// # Examples
	/// ```
	/// ToastBuilder::new("My toast message.")
	///     .with_position(ToastPosition::TopRight); // sets the position to the top right.
	/// ```
	#[must_use]
	pub fn with_position(mut self, position: ToastPosition) -> Self {
		self.position = position;
		self
	}

	/// Builds the toast into a `ToastData` with the supplied ID.
	#[must_use]
	pub fn build(self, id: ToastId) -> ToastData {
		ToastData {
			id,
			message: self.message,

			level: self.level,
			expiry: self.expiry,
			progress: self.progress,
			position: self.position,
		}
	}
}
