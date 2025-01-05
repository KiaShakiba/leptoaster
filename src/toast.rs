/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

mod builder;
mod data;

use gloo_timers::future::TimeoutFuture;
use leptos::*;

pub use crate::toast::data::{ToastData, ToastId, ToastLevel, ToastPosition};
use crate::toaster::expect_toaster;

/// A toast element with the supplied alert style.
#[component]
pub fn Toast(toast: ToastData) -> impl IntoView {
	let animation_duration = 200;

	let slide_in_animation_name = get_slide_in_animation_name(&toast.position);

	let slide_out_animation_name =
		get_slide_out_animation_name(&toast.position);

	let (animation_name, set_animation_name) =
		create_signal(slide_in_animation_name);

	let (background_color, border_color, text_color) = get_colors(&toast.level);
	let (initial_left, initial_right) = get_initial_positions(&toast.position);

	create_resource(
		|| (),
		move |()| async move {
			let Some(expiry) = toast.expiry else {
				return;
			};

			TimeoutFuture::new(expiry).await;

			if toast.clear_signal.get_untracked() {
				return;
			}

			toast.clear_signal.set(true);
		},
	);

	create_resource(
		move || toast.clear_signal.get(),
		move |clear| async move {
			if clear {
				set_animation_name(slide_out_animation_name);
				TimeoutFuture::new(animation_duration).await;
				expect_toaster().remove(toast.id);
			}
		},
	);

	let handle_click = move |_| {
		if !toast.dismissable {
			return;
		}

		toast.clear_signal.set(true);
	};

	view! {
		<div
			style:width="100%"
			style:margin="12px 0"
			style:padding="16px"
			style:background-color=background_color
			style:border="1px solid"
			style:border-color=border_color
			style:border-radius="4px"
			style:position="relative"
			style:cursor=get_cursor(toast.dismissable)
			style:overflow="hidden"
			style:box-sizing="border-box"
			style:left=initial_left
			style:right=initial_right
			style:display="flex"
			style:transition="transform 150ms ease-out, opacity 150ms ease-out"
			style:transition-delay="250ms, 0s"
			style:animation-name=animation_name
			style:animation-duration=format!("{}ms", animation_duration)
			style:animation-timing-function="linear"
			style:animation-fill-mode="forwards"
			on:click=handle_click
		>
			<span
				style:color=text_color
				style:font-size="var(--leptoaster-font-size)"
				style:line-height="var(--leptoaster-line-height)"
				style:font-family="var(--leptoaster-font-family)"
				style:font-weight="var(--leptoaster-font-weight)"
				style:display="inline-block"
				style:max-width="100%"
				style:text-overflow="ellipsis"
				style:overflow="hidden"
			>
				{toast.message}
			</span>

			<Show
				when=move || { toast.expiry.is_some() && toast.progress }
			>
				<div
					style:height="var(--leptoaster-progress-height)"
					style:width="100%"
					style:background-color=text_color
					style:position="absolute"
					style:bottom="0"
					style:left="0"
					style:animation-name="leptoaster-progress"
					style:animation-duration=format!("{}ms", toast.expiry.unwrap())
					style:animation-timing-function="linear"
					style:animation-fill-mode="forwards"
				/>
			</Show>
		</div>
	}
}

fn get_slide_in_animation_name(position: &ToastPosition) -> &'static str {
	match position {
		ToastPosition::TopLeft | ToastPosition::BottomLeft => {
			"leptoaster-slide-in-left"
		},
		ToastPosition::TopRight | ToastPosition::BottomRight => {
			"leptoaster-slide-in-right"
		},
	}
}

fn get_slide_out_animation_name(position: &ToastPosition) -> &'static str {
	match position {
		ToastPosition::TopLeft | ToastPosition::BottomLeft => {
			"leptoaster-slide-out-left"
		},
		ToastPosition::TopRight | ToastPosition::BottomRight => {
			"leptoaster-slide-out-right"
		},
	}
}

fn get_colors(
	level: &ToastLevel,
) -> (&'static str, &'static str, &'static str) {
	match level {
		ToastLevel::Info => (
			"var(--leptoaster-info-background-color)",
			"var(--leptoaster-info-border-color)",
			"var(--leptoaster-info-text-color)",
		),

		ToastLevel::Success => (
			"var(--leptoaster-success-background-color)",
			"var(--leptoaster-success-border-color)",
			"var(--leptoaster-success-text-color)",
		),

		ToastLevel::Warn => (
			"var(--leptoaster-warn-background-color)",
			"var(--leptoaster-warn-border-color)",
			"var(--leptoaster-warn-text-color)",
		),

		ToastLevel::Error => (
			"var(--leptoaster-error-background-color)",
			"var(--leptoaster-error-border-color)",
			"var(--leptoaster-error-text-color)",
		),
	}
}

fn get_initial_positions(
	position: &ToastPosition,
) -> (&'static str, &'static str) {
	match position {
		ToastPosition::TopLeft | ToastPosition::BottomLeft => {
			("calc((var(--leptoaster-width) + 12px * 2) * -1)", "auto")
		},
		ToastPosition::TopRight | ToastPosition::BottomRight => {
			("auto", "calc((var(--leptoaster-width) + 12px * 2) * -1)")
		},
	}
}

fn get_cursor(dismissable: bool) -> &'static str {
	match dismissable {
		true => "pointer",
		false => "default",
	}
}

pub use crate::toast::builder::ToastBuilder;
