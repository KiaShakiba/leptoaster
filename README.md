# Leptoaster

Leptoaster is a minimal toast library for Leptos. It provides a simple interface to show toast messages to the user.

## Demo

Take a look at the demo [here](https://kiashakiba.github.io/leptoaster-demo).

## Getting started

Assuming you already have Leptos installed, install Leptoaster
```
cargo add leptoaster
```

Once, installed, provide the toaster in the root component of your application and add the `Toaster` component.
```
use leptos::*;
use leptoaster::*;

#[component]
fn App() -> IntoView {
    provide_toaster();

    view! {
        <Toaster />
        // your other components
    }
}
```

To create a toast message in any component, simple use `expect_toaster()`.
```
use lepto::*;
use leptoaster::*;

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
        .with_expiry(Some(3_000)) // expiry in milliseconds (default is `2500`)
        .with_progress(false) // enable or disable the progress bar (default is `true`)
        .with_position(ToastPosition::TopRight) // set the toast position (default is 'ToastPosition::BottomLeft`)
);
```

## Styling

To customize styling, override any of the following CSS variables:

```
--leptoaster-width
--leptoaster-max-width

--leptoaster-font-family
--leptoaster-font-size
--leptoaster-line-height
--leptoaster-font-weight

--leptoaster-info-background-color
--leptoaster-info-border-color
--leptoaster-info-text-color

--leptoaster-success-background-color
--leptoaster-success-border-color
--leptoaster-success-text-color

--leptoaster-warn-background-color
--leptoaster-warn-border-color
--leptoaster-warn-text-color

--leptoaster-error-background-color
--leptoaster-error-border-color
--leptoaster-error-text-color
```
