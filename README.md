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

Run `cargo doc --no-deps --open` or explore the [online API reference](https://kahwee.github.io/gpui-jig/api/gpui_jig/). The Switch, Tabs, Checkbox, Accordion, Progress, Scroll Area, Avatar, Card and Toast modules contain Rust recipes checked by the documentation tests. Cratebook’s **Rust source** panel shows complete interactive views.

## Use it

Add the library to an existing GPUI project with the dependencies below. The repository also contains a starter generator; its npm package is not yet published.

```toml
[dependencies]
gpui-jig = "0.1"
gpui = { package = "gpui-pre", version = "0.3.10" }
```

### Switch anatomy

Compose a root and a thumb, then style each part with ordinary GPUI modifiers. The view owns the value; use the same value for the root's semantic state and the thumb's position. Give the root a stable ID and accessible name.

```rust
use gpui::{prelude::*, *};
use gpui_jig::switch;

struct Settings { notifications: bool }

impl Render for Settings {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let checked = self.notifications;
        div().flex().items_center().gap_2()
            .child("Notifications")
            .child(
                switch::root("notifications", "Notifications", checked)
                    .w(px(36.)).h(px(20.)).p(px(2.))
                    .flex().items_center().rounded_full()
                    .when(checked, |s| s.justify_end())
                    .bg(rgb(if checked { 0x20704a } else { 0x707070 }))
                    .focus_visible(|s| s.border_2().border_color(rgb(0x000000)))
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.notifications = !this.notifications;
                        cx.notify();
                    }))
                    .child(switch::thumb().size(px(16.)).rounded_full().bg(rgb(0xffffff))),
            )
    }
}
```

The root supplies the switch role and checked state; the thumb is an unstyled child. Colors, dimensions, focus treatment and label layout are application choices. Extract these modifiers into a theme or wrapper to reuse the same Jig parts with your design system.

### Integration checklist

1. Keep state in your view or model and give each interactive root a stable, unique ID and an accessible name.
2. Compose the parts in the module's API reference. Style each with GPUI modifiers, including visible focus and a suitable target size.
3. Connect events to your state and call `cx.notify()` when the view needs to render again. Derive semantics and appearance from the same value.
4. Check the interaction contracts below and implement any behavior Jig does not yet provide. Verify keyboard, pointer and accessibility behavior in your application.

Jig borrows part names from [Base UI](https://base-ui.com/react/components/switch), but its parts are GPUI elements: use Rust modifiers and application state instead of CSS classes and data attributes. See the [Switch API](https://kahwee.github.io/gpui-jig/api/gpui_jig/switch/) for the Rust contract.

For a compound navigation example, see the [Tabs API](https://kahwee.github.io/gpui-jig/api/gpui_jig/tabs/) and the [interactive Cratebook story](https://github.com/kahwee/cratebook/blob/main/src/stories/tabs.rs). Its view owns selected document IDs, keyboard focus and closing; the Jig parts supply the elements and navigation helper.

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

### Tables

The `table` module adds unstyled table, row, column-header and cell parts for desktop data views. Define a `ColumnDef` for each field and keep `Sorting` in your view to cycle between ascending, descending and input order. Columns sort by their native values even when `format_cell` changes the displayed text. The caller owns the rows, layout, controls and focus behavior. See the [Cratebook table example](https://github.com/kahwee/cratebook/blob/main/src/stories/table.rs) and [table API](https://kahwee.github.io/gpui-jig/api/gpui_jig/table/).

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
