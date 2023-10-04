# Vitesse

Vitesse is an open-source UI toolkit for gpui, the fast, native Rust GUI library.

## Overview

As gpui is still in early development things will likely change in both concept and code, but here is the current plan for Vitesse:

Three key components are currently being planned.

**A set of headless UI primitives in the vein of [headlessUI](https://headlessui.com/) or [Radix Primitives](https://www.radix-ui.com/primitives)**. This will be the core of Vitesse, and will be used to build the other two sublibraries. It will be a set of composable, accessible, unstyled components that can be used to build fully custom UIs without imposing any style or layout decisions on the developer. It will also serve as an example of how to build primitive UI element with gpui.

**A unopinionated color library, with an optional set of opinionated themes**. This will be a set of color constants and functions for working with colors. It will also include a set of themes that can be used to style the headless UI primitives. The themes will be designed to be used as is, or to be extended to create custom themes.

**A set of styled components**. This will be a set of components built on top of the headless UI toolkit that provide a set of common UI elements with a default style.

Designed to make bootstrapping an apps UI seemless, these components will come with baked in styles and larger elements built from composing primitives. They will be designed to be used as is, or to be extended to create custom components.

These components are designed to make it fast and easy to bootstrap a gpui app. The components will come with in-built styles and larger elements forged by composing primitives.

## Next Steps

The initial steps for Vitesse will be to study and learn from existing  UI libraries, and start exploring how to build primitives.

The release of Vitesse will be blocked in the near future by the release of gpui, but in the meantime we'll be working on how to build the library and what it will look like.

## Contributing

In the future contribution will be heavily encouraged, but for now we're still in the early stages of planning and development.
