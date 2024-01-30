/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

pub mod context;

use leptos::*;

use crate::toaster::context::ToasterContext;
use crate::toast::Toast;

/// Creates the toaster containers as fixed-position elements on the corners of the screen.
///
/// # Examples
/// ```
/// use leptos::*;
/// use leptoast::*;
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
	let toaster = use_context::<ToasterContext>()
		.expect("Could not use toaster context.");

	view! {
		<style>
			"@keyframes leptoast-progress {
				from { width: 100%; }
				to { width: 0; }
			}"
		</style>

		<div
			style:position="fixed"
			style:top="0"
			style:left="0"
			style:z-index="99999"
		>
			<For
				each=toaster.top_left_queue
				key=|toast| toast.id
				let:toast
			>
				<Toast toast={toast} />
			</For>
		</div>

		<div
			style:position="fixed"
			style:top="0"
			style:right="0"
			style:z-index="99999"
		>
			<For
				each=toaster.top_right_queue
				key=|toast| toast.id
				let:toast
			>
				<Toast toast={toast} />
			</For>
		</div>

		<div
			style:position="fixed"
			style:bottom="0"
			style:right="0"
			style:z-index="99999"
		>
			<For
				each=toaster.bottom_right_queue
				key=|toast| toast.id
				let:toast
			>
				<Toast toast={toast} />
			</For>
		</div>

		<div
			style:position="fixed"
			style:bottom="0"
			style:left="0"
			style:z-index="99999"
		>
			<For
				each=toaster.bottom_left_queue
				key=|toast| toast.id
				let:toast
			>
				<Toast toast={toast} />
			</For>
		</div>
	}
}
