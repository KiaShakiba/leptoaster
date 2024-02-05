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

const CONTAINER_POSITIONS: &[ToastPosition] = &[
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
			:root {
				--leptoaster-width: 320px;
				--leptoaster-max-width: 80vw;

				--leptoaster-font-family: Arial;
				--leptoaster-font-size: 14px;
				--leptoaster-line-height: 20px;
				--leptoaster-font-weight: 600;

				--leptoaster-info-background-color: #ffffff;
				--leptoaster-info-border-color: #222222;
				--leptoaster-info-text-color: #222222;

				--leptoaster-success-background-color: #4caf50;
				--leptoaster-success-border-color: #2e7d32;
				--leptoaster-success-text-color: #ffffff;

				--leptoaster-warn-background-color: #ff9800;
				--leptoaster-warn-border-color: #ff8f00;
				--leptoaster-warn-text-color: #ffffff;

				--leptoaster-error-background-color: #f44336;
				--leptoaster-error-border-color: #c62828;
				--leptoaster-error-text-color: #ffffff;
			}

			@keyframes leptoaster-slide-in-left {
				from { left: calc((var(--leptoaster-width) + 12px * 2) * -1) }
				to { left: 0 }
			}

			@keyframes leptoaster-slide-out-left {
				from { left: 0 }
				to { left: calc((var(--leptoaster-width) + 12px * 2) * -1) }
			}

			@keyframes leptoaster-slide-in-right {
				from { right: calc((var(--leptoaster-width) + 12px * 2) * -1) }
				to { right: 0 }
			}

			@keyframes leptoaster-slide-out-right {
				from { right: 0 }
				to { right: calc((var(--leptoaster-width) + 12px * 2) * -1) }
			}

			@keyframes leptoaster-progress {
				from { width: 100%; }
				to { width: 0; }
			}
			"
		</style>

		<For
			each=move || CONTAINER_POSITIONS
			key=|position| get_container_id(position)
			let:position
		>
			<Show
				when=move || !is_container_empty(position)
			>
				<div
					style:width="var(--leptoaster-width)"
					style:max-width="var(--leptoaster-max-width)"
					style:margin=get_container_margin(position)
					style:position="fixed"
					style:inset=get_container_inset(position)
					style:z-index="99999"
				>
					<For
						each=move || {
							toaster.queue
								.get().iter()
								.filter(|toast| toast.position.eq(position)).cloned()
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

fn is_container_empty(position: &ToastPosition) -> bool {
	!expect_toaster().queue
		.get().iter()
		.any(|toast| toast.position.eq(position))
}

fn get_container_id(position: &ToastPosition) -> &'static str {
	match position {
		ToastPosition::TopLeft => "top_left",
		ToastPosition::TopRight => "top_right",
		ToastPosition::BottomRight => "bottom_right",
		ToastPosition::BottomLeft => "bottom_left",
	}
}

fn get_container_inset(position: &ToastPosition) -> &'static str {
	match position {
		ToastPosition::TopLeft => "0 auto auto 0",
		ToastPosition::TopRight => "0 0 auto auto",
		ToastPosition::BottomRight => "auto 0 0 auto",
		ToastPosition::BottomLeft => "auto 0 0 0",
	}
}

fn get_container_margin(position: &ToastPosition) -> &'static str {
	match position {
		ToastPosition::TopLeft | ToastPosition::BottomLeft => "0 0 0 12px",
		ToastPosition::TopRight | ToastPosition::BottomRight => "0 12px 0 0",
	}
}
