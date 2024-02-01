/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

mod data;
mod builder;

use leptos::*;

use gloo_timers::{
	future::TimeoutFuture,
	callback::Timeout,
};

use crate::toaster::expect_toaster;

pub use crate::toast::data::{
	ToastData,
	ToastId,
	ToastLevel,
	ToastPosition,
};

/// A toast element with the supplied alert style.
#[component]
pub fn Toast(toast: ToastData) -> impl IntoView {
	let animation_duration = 200;

	let slide_in_animation_name = match toast.position {
		ToastPosition::TopLeft | ToastPosition::BottomLeft => "leptoaster-slide-in-left",
		ToastPosition::TopRight | ToastPosition::BottomRight => "leptoaster-slide-in-right",
	};

	let slide_out_animation_name = match toast.position {
		ToastPosition::TopLeft | ToastPosition::BottomLeft => "leptoaster-slide-out-left",
		ToastPosition::TopRight | ToastPosition::BottomRight => "leptoaster-slide-out-right",
	};

	let (animation_name, set_animation_name) = create_signal(slide_in_animation_name);

	let (background_color, border_color) = match toast.level {
		ToastLevel::Info => ("#ffffff", "#222222"),
		ToastLevel::Success => ("#4caf50", "#2e7d32"),
		ToastLevel::Warn => ("#ff9800", "#ff8f00"),
		ToastLevel::Error => ("#f44336", "#c62828"),
	};

	let text_color = match toast.level {
		ToastLevel::Info => "#222222",
		_ => "#ffffff",
	};

	let (initial_left, initial_right) = match toast.position {
		ToastPosition::TopLeft | ToastPosition::BottomLeft => ("-344px", "auto"),
		ToastPosition::TopRight | ToastPosition::BottomRight => ("auto", "-344px"),
	};

	create_resource(|| (), move |()| async move {
		let Some(expiry) = toast.expiry else {
			return;
		};

		TimeoutFuture::new(expiry).await;
		set_animation_name(slide_out_animation_name);
		TimeoutFuture::new(animation_duration).await;
		expect_toaster().remove(toast.id);
	});

	view! {
		<div
			style:width="320px"
			style:max-width="80vw"
			style:margin="12px"
			style:padding="16px"
			style:background-color=background_color
			style:border="1px solid"
			style:border-color=border_color
			style:border-radius="4px"
			style:position="relative"
			style:cursor="pointer"
			style:overflow="hidden"
			style:box-sizing="border-box"
			style:left=initial_left
			style:right=initial_right
			style:display="flex"
			style:animation-name=animation_name
			style:animation-duration=format!("{}ms", animation_duration)
			style:animation-timing-function="linear"
			style:animation-fill-mode="forwards"
			on:click=move |_| {
				set_animation_name(slide_out_animation_name);

				Timeout::new(animation_duration, move || {
					expect_toaster().remove(toast.id);
				}).forget();
			}
		>
			<span
				style:color=text_color
				style:font-size="14px"
				style:line-height="20px"
				style:font-family="Arial"
				style:font-weight="600"
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
					style:height="4px"
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

pub use crate::toast::builder::ToastBuilder;
