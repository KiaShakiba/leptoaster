/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

pub mod context;

use leptos::*;
use crate::toaster::context::ToasterContext;
use crate::toast::{Toast, ToastData, ToastPosition};

const CONTAINER_POSITIONS: &'static [ToastPosition] = &[
	ToastPosition::TopLeft,
	ToastPosition::TopRight,
	ToastPosition::BottomRight,
	ToastPosition::BottomLeft,
];

/// Creates the toaster containers as fixed-position elements on the corners of the screen.
///
/// # Examples
/// ```
/// use leptos::*;
/// use leptoaster::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <Toaster />
///     }
/// }
/// ```
#[component]
pub fn Toaster() -> impl IntoView {
	let toaster = expect_toaster();

	view! {
		<style>
			"
			@keyframes leptoaster-slide-in-left {
				from { left: -344px }
				to { left: 0 }
			}

			@keyframes leptoaster-slide-out-left {
				from { left: 0 }
				to { left: -344px }
			}

			@keyframes leptoaster-slide-in-right {
				from { right: -344px }
				to { right: 0 }
			}

			@keyframes leptoaster-slide-out-right {
				from { right: 0 }
				to { right: -344px }
			}

			@keyframes leptoaster-progress {
				from { width: 100%; }
				to { width: 0; }
			}
			"
		</style>

		<For
			each=move || CONTAINER_POSITIONS
			key=|position| container_id(position)
			let:position
		>
			<Show
				when=move || !container_empty(position)
			>
				<div
					style:width="320px"
					style:max-width="80vw"
					style:margin=container_margin(position)
					style:position="fixed"
					style:inset=container_inset(position)
					style:z-index="99999"
				>
					<For
						each=move || {
							toaster.queue
								.get().iter()
								.filter(|toast| toast.position.eq(position))
								.map(|toast| toast.clone())
								.collect::<Vec<ToastData>>()
						}
						key=|toast| toast.id
						let:toast
					>
						<Toast toast={toast} />
					</For>
				</div>
			</Show>
		</For>
	}
}

pub fn provide_toaster() {
	if use_context::<ToasterContext>().is_none() {
		provide_context(ToasterContext::default());
	}
}

#[must_use]
pub fn expect_toaster() -> ToasterContext {
	expect_context::<ToasterContext>()
}

fn container_empty(position: &ToastPosition) -> bool {
	!expect_toaster().queue
		.get().iter()
		.any(|toast| toast.position.eq(position))
}

fn container_id(position: &ToastPosition) -> String {
	match position {
		ToastPosition::TopLeft => "top_left".into(),
		ToastPosition::TopRight => "top_right".into(),
		ToastPosition::BottomRight => "bottom_right".into(),
		ToastPosition::BottomLeft => "bottom_left".into(),
	}
}

fn container_inset(position: &ToastPosition) -> String {
	match position {
		ToastPosition::TopLeft => "0 auto auto 0".into(),
		ToastPosition::TopRight => "0 0 auto auto".into(),
		ToastPosition::BottomRight => "auto 0 0 auto".into(),
		ToastPosition::BottomLeft => "auto 0 0 0".into(),
	}
}

fn container_margin(position: &ToastPosition) -> String {
	match position {
		ToastPosition::TopLeft | ToastPosition::BottomLeft => "0 0 0 12px".into(),
		ToastPosition::TopRight | ToastPosition::BottomRight => "0 12px 0 0".into(),
	}
}
