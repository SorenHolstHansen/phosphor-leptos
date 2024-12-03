<img src="./meta/phosphor-mark-tight-yellow.png" width="128" align="right" />

# phosphor-leptos

[![crates.io](https://img.shields.io/crates/v/phosphor-leptos)](https://crates.io/crates/phosphor-leptos)
[![docs.rs](https://docs.rs/phosphor-leptos/badge.svg)](https://docs.rs/phosphor-leptos)

Phosphor is a flexible icon family for interfaces, diagrams, presentations — whatever, really. You can explore the available icons at [phosphoricons.com](https://phosphoricons.com).

## Installation

```bash
phosphor-leptos = "0.6.0"
```

or

```bash
cargo add phosphor-leptos
```

## Usage

```rs
use leptos::prelude::*;
use phosphor_leptos::{Icon, IconWeight, HORSE, HEART, CUBE};

#[component]
fn MyComponent() -> impl IntoView {
    view! {
        <Icon icon=HORSE />
        <Icon icon=HEART color="#AE2983" weight=IconWeight::Fill size="32px" />
        <Icon icon=CUBE color="teal" weight=IconWeight::Duotone />
    }
}
```

### Props

All of the props are `Signal`s so they can be static or reactive.

- **color?**: `string` – Icon stroke/fill color. Can be any CSS color string, including `hex`, `rgb`, `rgba`, `hsl`, `hsla`, named colors, or the special `currentColor` variable.
- **size?**: `number | string` – Icon height & width. As with standard React elements, this can be a number, or a string with units in `px`, `%`, `em`, `rem`, `pt`, `cm`, `mm`, `in`.
- **weight?**: `IconWeight` – Icon weight/style. Can also be used, for example, to "toggle" an icon's state: a rating component could use Stars with `IconWeight::Regular` to denote an empty star, and `IconWeight::Fill` to denote a filled star.
- **mirrored?**: `boolean` – Flip the icon horizontally. Can be useful in RTL languages where normal icon orientation is not appropriate.

## License

MIT © [Phosphor Icons](https://github.com/phosphor-icons)

## Knowledge

- [Phosphor Svelte](https://github.com/haruaki07/phosphor-svelte)

## Contributing

To generate all the icons, run

```bash
cargo xtask update
```
