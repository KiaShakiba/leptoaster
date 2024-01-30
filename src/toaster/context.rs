/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::sync::{Arc, Mutex};
use leptos::*;

use crate::toast::{
	ToastBuilder,
	ToastData,
	ToastId,
	ToastLevel,
	ToastPosition,
};

/// The global context of the toaster. You should provide this as a global context
/// in your root component to allow any component in your application to toast
/// using the same toast queue.
///
///  # Examples
///  ```
///  #[component]
///  fn App() -> impl IntoView {
///      provide_context(ToasterContext::default());
///  }
///  ```
#[derive(Clone, Debug)]
pub struct ToasterContext {
	stats: Arc<Mutex<ToasterStats>>,

	pub top_left_queue: RwSignal<Vec<ToastData>>,
	pub top_right_queue: RwSignal<Vec<ToastData>>,
	pub bottom_right_queue: RwSignal<Vec<ToastData>>,
	pub bottom_left_queue: RwSignal<Vec<ToastData>>,
}

#[derive(Clone, Default, Debug)]
struct ToasterStats {
	visible: u32,
	total: u64,
}

impl ToasterContext {
	/// Adds the supplied toast to the toast queue, displaying it onto the screen.
	///
	/// # Examples
	/// ```
	/// #[component]
	/// fn Component() -> impl IntoView {
	///     let toaster = use_context::<ToasterContext>
	///         .expect("Could not use toaster context.");
	///
	///     toaster.toast(
	///         ToastBuilder::new("My toast message.")
	///             .with_expiry(1_500)
	///     );
	/// }
	/// ```
	pub fn toast(&self, builder: ToastBuilder) {
		let mut stats = self.stats.lock().unwrap();
		let toast = builder.build(stats.total + 1);

		match toast.position {
			ToastPosition::TopLeft => {
				let mut queue = self.top_left_queue.get_untracked();
				queue.push(toast);
				self.top_left_queue.set(queue);
			},

			ToastPosition::TopRight => {
				let mut queue = self.top_right_queue.get_untracked();
				queue.push(toast);
				self.top_right_queue.set(queue);
			},

			ToastPosition::BottomRight => {
				let mut queue = self.bottom_right_queue.get_untracked();
				queue.push(toast);
				self.bottom_right_queue.set(queue);
			},

			ToastPosition::BottomLeft => {
				let mut queue = self.bottom_left_queue.get_untracked();
				queue.push(toast);
				self.bottom_left_queue.set(queue);
			},
		}

		stats.visible += 1;
		stats.total += 1;
	}

	/// Quickly display an `info` toast with default parameters. For more customization,
	/// use the `toast` function.
	///
	/// # Examples
	/// ```
	/// #[component]
	/// fn Component() -> impl IntoView {
	///     let toaster = use_context::<ToasterContext>
	///         .expect("Could not use toaster context.");
	///
	///     toaster.info("My toast message.");
	/// }
	/// ```
	pub fn info(&self, message: &str) {
		self.toast(
			ToastBuilder::new(message)
				.with_level(ToastLevel::Info)
		);
	}

	/// Quickly display a `success` toast with default parameters. For more customization,
	/// use the `toast` function.
	///
	/// # Examples
	/// ```
	/// #[component]
	/// fn Component() -> impl IntoView {
	///     let toaster = use_context::<ToasterContext>
	///         .expect("Could not use toaster context.");
	///
	///     toaster.success("My toast message.");
	/// }
	/// ```
	pub fn success(&self, message: &str) {
		self.toast(
			ToastBuilder::new(message)
				.with_level(ToastLevel::Success)
		);
	}

	/// Quickly display a `warn` toast with default parameters. For more customization,
	/// use the `toast` function.
	///
	/// # Examples
	/// ```
	/// #[component]
	/// fn Component() -> impl IntoView {
	///     let toaster = use_context::<ToasterContext>
	///         .expect("Could not use toaster context.");
	///
	///     toaster.warn("My toast message.");
	/// }
	/// ```
	pub fn warn(&self, message: &str) {
		self.toast(
			ToastBuilder::new(message)
				.with_level(ToastLevel::Warn)
		);
	}

	/// Quickly display an `error` toast with default parameters. For more customization,
	/// use the `toast` function.
	///
	/// # Examples
	/// ```
	/// #[component]
	/// fn Component() -> impl IntoView {
	///     let toaster = use_context::<ToasterContext>
	///         .expect("Could not use toaster context.");
	///
	///     toaster.error("My toast message.");
	/// }
	/// ```
	pub fn error(&self, message: &str) {
		self.toast(
			ToastBuilder::new(message)
				.with_level(ToastLevel::Error)
		);
	}

	/// Removes the toast corresponding with the supplied `ToastId`.
	pub fn remove(&self, toast_id: ToastId) {
		let mut stats = self.stats.lock().unwrap();

		let top_left_index = self.top_left_queue
			.get_untracked()
			.iter().enumerate()
			.find(|(_, toast)| toast.id == toast_id)
			.map(|(index, _)| index);

		let top_right_index = self.top_right_queue
			.get_untracked()
			.iter().enumerate()
			.find(|(_, toast)| toast.id == toast_id)
			.map(|(index, _)| index);

		let bottom_right_index = self.bottom_right_queue
			.get_untracked()
			.iter().enumerate()
			.find(|(_, toast)| toast.id == toast_id)
			.map(|(index, _)| index);

		let bottom_left_index = self.bottom_left_queue
			.get_untracked()
			.iter().enumerate()
			.find(|(_, toast)| toast.id == toast_id)
			.map(|(index, _)| index);

		if let Some(index) = top_left_index {
			let mut queue = self.top_left_queue.get_untracked();
			queue.remove(index);
			self.top_left_queue.set(queue);

			stats.visible -= 1;
		}

		if let Some(index) = top_right_index {
			let mut queue = self.top_right_queue.get_untracked();
			queue.remove(index);
			self.top_right_queue.set(queue);

			stats.visible -= 1;
		}

		if let Some(index) = bottom_right_index {
			let mut queue = self.bottom_right_queue.get_untracked();
			queue.remove(index);
			self.bottom_right_queue.set(queue);

			stats.visible -= 1;
		}

		if let Some(index) = bottom_left_index {
			let mut queue = self.bottom_left_queue.get_untracked();
			queue.remove(index);
			self.bottom_left_queue.set(queue);

			stats.visible -= 1;
		}
	}
}

impl Default for ToasterContext {
	fn default() -> Self {
		ToasterContext {
			stats: Arc::new(Mutex::new(ToasterStats::default())),

			top_left_queue: create_rw_signal(Vec::new()),
			top_right_queue: create_rw_signal(Vec::new()),
			bottom_right_queue: create_rw_signal(Vec::new()),
			bottom_left_queue: create_rw_signal(Vec::new()),
		}
	}
}
