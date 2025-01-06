/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::sync::{Arc, Mutex};
use leptos::prelude::*;
use crate::toast::{ToastBuilder, ToastData, ToastId, ToastLevel};

/// The global context of the toaster. You should provide this as a global
/// context in your root component to allow any component in your application to
/// toast using the same toast queue.
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
	pub queue: RwSignal<Vec<ToastData>>,
}

#[derive(Clone, Default, Debug)]
struct ToasterStats {
	visible: u32,
	total: u64,
}

impl ToasterContext {
	/// Adds the supplied toast to the toast queue, displaying it onto the
	/// screen.
	///
	/// # Examples
	/// ```
	/// #[component]
	/// fn Component() -> impl IntoView {
	///     let toaster = expect_context::<ToasterContext>();
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

		let mut queue = self.queue.get_untracked();
		queue.push(toast);
		self.queue.set(queue);
		stats.visible += 1;
		stats.total += 1;
	}

	/// Quickly display an `info` toast with default parameters. For more
	/// customization, use the `toast` function.
	///
	/// # Examples
	/// ```
	/// #[component]
	/// fn Component() -> impl IntoView {
	///     let toaster = expect_context::<ToasterContext>();
	///     toaster.info("My toast message.");
	/// }
	/// ```
	pub fn info(&self, message: &str) {
		self.toast(ToastBuilder::new(message).with_level(ToastLevel::Info));
	}

	/// Quickly display a `success` toast with default parameters. For more
	/// customization, use the `toast` function.
	///
	/// # Examples
	/// ```
	/// #[component]
	/// fn Component() -> impl IntoView {
	///     let toaster = expect_context::<ToasterContext>();
	///     toaster.success("My toast message.");
	/// }
	/// ```
	pub fn success(&self, message: &str) {
		self.toast(ToastBuilder::new(message).with_level(ToastLevel::Success));
	}

	/// Quickly display a `warn` toast with default parameters. For more
	/// customization, use the `toast` function.
	///
	/// # Examples
	/// ```
	/// #[component]
	/// fn Component() -> impl IntoView {
	///     let toaster = expect_context::<ToasterContext>();
	///     toaster.warn("My toast message.");
	/// }
	/// ```
	pub fn warn(&self, message: &str) {
		self.toast(ToastBuilder::new(message).with_level(ToastLevel::Warn));
	}

	/// Quickly display an `error` toast with default parameters. For more
	/// customization, use the `toast` function.
	///
	/// # Examples
	/// ```
	/// #[component]
	/// fn Component() -> impl IntoView {
	///     let toaster = expect_context::<ToasterContext>();
	///     toaster.error("My toast message.");
	/// }
	/// ```
	pub fn error(&self, message: &str) {
		self.toast(ToastBuilder::new(message).with_level(ToastLevel::Error));
	}

	/// Clears all currently visible toasts.
	///
	/// # Examples
	/// ```
	/// #[component]
	/// fn Component() -> impl IntoView {
	///     let toaster = expect_context::<ToasterContext>();
	///
	///     toaster.toast(
	///         ToastBuilder::new("My toast message.")
	///             .with_expiry(None) // the toast will not self-expire
	///     );
	///
	///     toaster.clear();
	/// }
	/// ```
	pub fn clear(&self) {
		for toast in &self.queue.get_untracked() {
			toast.clear_signal.set(true);
		}
	}

	/// Removes the toast corresponding with the supplied `ToastId`.
	pub fn remove(&self, toast_id: ToastId) {
		let index = self
			.queue
			.get_untracked()
			.iter()
			.enumerate()
			.find(|(_, toast)| toast.id == toast_id)
			.map(|(index, _)| index);

		if let Some(index) = index {
			let mut queue = self.queue.get_untracked();
			queue.remove(index);
			self.queue.set(queue);

			self.stats.lock().unwrap().visible -= 1;
		}
	}
}

impl Default for ToasterContext {
	fn default() -> Self {
		ToasterContext {
			stats: Arc::new(Mutex::new(ToasterStats::default())),
			queue: RwSignal::new(Vec::new()),
		}
	}
}
