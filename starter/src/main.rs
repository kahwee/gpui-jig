use gpui::{prelude::*, *};
use gpui_jig::button;

struct Counter(usize);

impl Render for Counter {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .p_4()
            .bg(rgb(0xffffff))
            .text_color(rgb(0x242424))
            .child(
                button::root("count", "Increment counter")
                    .min_h(px(28.))
                    .px_2()
                    .text_size(px(13.))
                    .rounded_md()
                    .flex()
                    .items_center()
                    .bg(rgb(0xececea))
                    .focus_visible(|s| s.underline())
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.0 += 1;
                        cx.notify();
                    }))
                    .child(format!("Count: {}", self.0)),
            )
    }
}

fn main() {
    gpui_platform::application().run(|cx| {
        cx.open_window(WindowOptions::default(), |_, cx| cx.new(|_| Counter(0)))
            .expect("open starter window");
        cx.activate(true);
    });
}
