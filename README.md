# GPUI Jig

[Component guide and gallery](https://kahwee.github.io/gpui-jig/)

GPUI Jig is a Rust-first collection of unstyled components for GPUI. It uses the component and part names from [Base UI](https://base-ui.com/). Each constructor returns an ordinary GPUI element. Supply your own typography, colors, borders, child elements and event listeners.

Cratebook owns its desktop typography, theme, spacing, and navigation. GPUI Jig supplies semantics and interaction primitives. Even the pointer cursor is a caller styling choice.

## Why I’m building this

I started GPUI Jig while building a Rust desktop app in stealth. I meant to ship that app. The more I used it myself, the less sure I was that it was something people would keep coming back to. I still care about how a button feels or how much room a sidebar takes up. Getting those details right leaves a bigger question open: what would someone open the app to do tomorrow?

While I figure out where to take the app next, I’m rewriting the tools I built for it and releasing them as open source. GPUI Jig is one of those tools. If you’re building a different kind of Rust app, I’d like you to be able to take the parts you need and make them feel at home in yours.

[![Library CI](https://github.com/kahwee/gpui-jig/actions/workflows/check.yml/badge.svg)](https://github.com/kahwee/gpui-jig/actions/workflows/check.yml)

## Explore the API

Each component has its own Rust module. Compose its named parts, then add GPUI styles and listeners. Keep mutable state in your view and call `cx.notify()` after a change. Give repeated controls stable IDs so focus follows the same item across renders.

Run `cargo doc --no-deps --open` or explore the [online API reference](https://kahwee.github.io/gpui-jig/api/gpui_jig/). The Checkbox, Accordion, Progress, Scroll Area, Avatar, Card and Toast modules contain checked Rust recipes. Cratebook’s **Rust source** panel shows complete interactive views.

## Use it

Generate a Rust project with a compact counter button using `pnpm dlx gpui-jig create my-app` or `npx gpui-jig create my-app`. From the new directory, run `cargo run --release` on macOS with the Rust and GPUI build prerequisites installed. The npm CLI scaffolds the app; Cargo supplies the native library. To add it to an existing project, use the dependencies below.

```toml
[dependencies]
gpui-jig = "0.1.0"
gpui = { package = "gpui-pre", version = "0.3.10" }
```

A controlled switch in a GPUI view can use any design system:

```rust
use gpui::{prelude::*, *};
use gpui_jig::switch;

struct Settings { notifications: bool }

impl Render for Settings {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        switch::root("notifications", "Notifications", self.notifications)
            .px_3().py_2().rounded_md()
            .bg(rgb(if self.notifications { 0x20704a } else { 0x404044 }))
            .text_color(rgb(0xffffff))
            .focus_visible(|style| style.underline())
            .on_click(cx.listener(|this, _, _, cx| {
                this.notifications = !this.notifications;
                cx.notify();
            }))
            .child(if self.notifications { "Notifications on" } else { "Notifications off" })
    }
}
```

## Desktop layouts

Try the [unified and split title-bar recipes](https://kahwee.github.io/gpui-jig/#shells). Cratebook composes both layouts from GPUI containers and Jig controls. The application owns navigation state, sidebar width, palette and native window configuration.

## Components

| Family | Modules |
| --- | --- |
| Actions and choices | Button, Switch, Checkbox, Checkbox Group, Radio Group, Toggle, Toggle Group |
| Navigation and disclosure | Accordion, Collapsible, Tabs, Menu, Context Menu, Menubar, Navigation Menu, Toolbar |
| Forms and editing | Form, Fieldset, Field, Input, OTP Field, Number Field, Slider, Select, Combobox, Autocomplete |
| Overlays | Dialog, Alert Dialog, Drawer, Popover, Preview Card, Tooltip, Toast |
| Display | Avatar, Meter, Progress, Scroll Area, Separator |

## Cards and desktop context menus

Use `card::root`, `card::title` and `card::content` for an ordinary content card. Card is a desktop addition. Base UI’s Preview Card is a separate popup component.

Wrap a target in `context_menu::trigger`. Its secondary-button handler receives window coordinates. Pass that position and your styled Popup to `context_menu::positioner` for a window-fitted overlay. Compose Item children from your own icon and label. SubmenuRoot and SubmenuTrigger can be nested with another Popup at each level. Your view owns the open path, focus, key handling and commands. Provide an ordinary Button for touch and keyboard access to the same menu.

[Working card and context menu example](https://github.com/kahwee/cratebook/blob/main/src/stories/card_menu.rs) · [Base UI Context Menu anatomy](https://base-ui.com/react/components/context-menu)

## Interaction contracts

The initial API covers semantic parts for all 37 families. Radio Group supplies roving focus and arrow-key selection. Optional state helpers implement accordion expansion, bounded numeric values, navigation over disabled items, and popup focus restoration. Scroll Area uses GPUI's scroll container. Buttons and triggers accept ordinary GPUI listeners.

For compound controls, the application connects the parts. Editors supply their GPUI text-input handler for IME and selection. Popup placement, modal focus trapping, dismissal policy, slider pointer tracking, toast timing, avatar image loading and form validation are currently application responsibilities. Platform accessibility and full Base UI behavior parity are still under development.

Run `cargo test --locked` for the state tests and `cargo doc --no-deps --open` for the Rust API.

Released under the [MIT license](LICENSE).

## Packaging

Run `cargo publish --locked --dry-run` to verify the distributable Rust crate. CI uploads the verified archive. To release the checked commit, verify the version and run `cargo publish --locked` with a crates.io publisher account. The browser gallery ships separately as the `cratebook` npm CLI.
