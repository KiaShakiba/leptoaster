/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

mod data;
mod builder;

use leptos::*;
use gloo_timers::future::TimeoutFuture;
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
	let background_color = match toast.level {
		ToastLevel::Info => "#f5f5f5",
		ToastLevel::Success => "#4caf50",
		ToastLevel::Warn => "#ff9800",
		ToastLevel::Error => "#f44336",
	};

	let border_color = match toast.level {
		ToastLevel::Info => "#222222",
		ToastLevel::Success => "#2e7d32",
		ToastLevel::Warn => "#ff8f00",
		ToastLevel::Error => "#c62828",
	};

	let text_color = match toast.level {
		ToastLevel::Info => "#222222",
		_ => "#ffffff",
	};

	create_resource(|| (), move |_| async move {
		TimeoutFuture::new(toast.expiry).await;
		//expect_toaster().remove(toast.id);
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
			style:border-radius="8px"
			style:position="relative"
			style:cursor="pointer"
			style:overflow="hidden"
			style:box-sizing="border-box"
			style:left="-344px"
			style:animation-name="leptoast-slide-in"
			style:animation-duration="250ms"
			style:animation-timing-function="linear"
			style:animation-fill-mode="forwards"
			on:click=move |_| expect_toaster().remove(toast.id)
		>
			<span
				style:color=text_color
				style:font-size="14px"
				style:line-height="20px"
				style:font-family="Arial"
				style:font-weight="600"
				style:display="inline-block"
			>
				{toast.message}
			</span>

			<Show
				when=move || { toast.progress }
			>
				<div
					style:height="4px"
					style:width="100%"
					style:background-color=text_color
					style:position="absolute"
					style:bottom="0"
					style:left="0"
					style:animation-name="leptoast-progress"
					style:animation-duration=format!("{}ms", toast.expiry)
					style:animation-timing-function="linear"
					style:animation-fill-mode="forwards"
				/>
			</Show>
		</div>
	}
}

pub use crate::toast::builder::ToastBuilder;
