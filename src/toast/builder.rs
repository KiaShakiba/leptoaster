/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use leptos::prelude::RwSignal;

use crate::toast::data::{ToastData, ToastId, ToastLevel, ToastPosition};

pub struct ToastBuilder {
    message: String,

    level: ToastLevel,

    dismissable: bool,
    expiry: Option<u32>,
    progress: bool,

    position: ToastPosition,
}

/// Builds a toast, allowing for the custimization of toast message,
/// level, dismissability, expiry, and position.
///
/// The defaults are:
/// * `level`: `ToastLevel::Info`
/// * `dismissable`: `true`
/// * `expiry`: `2_500`
/// * `progress`: `true`
/// * `position`: `ToastPosition::BottomLeft`
///
/// # Examples
/// ```
/// ToastBuilder::new("My toast message.")
///     .with_level(ToastLevel::Success)
///     .with_expiry(Some(2_500))
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

            dismissable: true,
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

    /// Sets the dismissable flag of the toast to allow or disallow the toast
    /// from being dismissable on click.
    ///
    /// # Examples
    /// ```
    /// ToastBuilder::new("My toast message.")
    ///     .with_dismissable(true); // allows the toast to be dismissable on click
    /// ```
    #[must_use]
    pub fn with_dismissable(mut self, dismissable: bool) -> Self {
        self.dismissable = dismissable;
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

            dismissable: self.dismissable,
            expiry: self.expiry,
            progress: self.progress,

            position: self.position,

            clear_signal: RwSignal::new(false),
        }
    }
}
