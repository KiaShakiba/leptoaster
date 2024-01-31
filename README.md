# Leptoast

Leptoast is a minimal toast library for Leptos. It provides a simple interface to show toast messages to the user.

## Getting started

Assuming you already have Leptos installed, install Leptoast
```
cargo add leptoast
```

Once, installed, provide the toaster in the root component of your application and add the `Toaster` component.
```
use leptos::*;
use leptoast::*;

#[component]
fn App() -> IntoView {
    provide_toaster();

    view! {
        <Toaster />
        // your other components
    }
}
```

To create a toast message in any component, simple use `expect_toaster()` and call `toast`.
```
use lepto::*;
use leptoast::*;

#[component]
fn MyComponent() -> IntoView {
    let toaster = expect_toaster();

    toaster.success("A toast message should appear!");
}
```

That's it! You can now show toast messages in your UI!

The `toaster` exposes a number of different kinds of toasts:
* `info`
* `success`
* `warn`
* `error`

For more customization, use the `toast` function along with the `ToastBuilder`:
```
toaster.toast(
    ToastBuilder::new("My toast message goes here.")
        .with_level(ToastLevel::Success) // set the toast level (default is `ToastLevel::Info`)
        .with_expiry(3_000) // expiry in milliseconds (default is `2500`)
        .with_progress(false) // enable or disable the progress bar (default is `true`)
        .with_position(ToastPosition::TopRight) // set the toast position (default is 'ToastPosition::BottomLeft`)
);
```
