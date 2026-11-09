use gtk4::prelude::*;
use gtk4::{gdk, glib, Application, ApplicationWindow, Button, CssProvider, GestureClick, Label, Orientation, Scale};
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};
use std::cell::Cell;
use std::process::Command;
use std::rc::Rc;

const CSS: &str = include_str!("style.css");

// Horizontal position of the flyout, in px from the left edge of the screen
const MARGIN_LEFT: i32 = 1262;
// Gap between the flyout and the bar, tuned by eye
const MARGIN_BOTTOM: i32 = 7;

// Font Awesome glyphs.
const ICON_MUTED: &str = "\u{f026}";
const ICON_LOW: &str = "\u{f027}";
const ICON_HIGH: &str = "\u{f028}";
const ICON_MIXER: &str = "\u{f1de}";

fn wpctl(args: &[&str]) -> String {
    Command::new("wpctl")
        .args(args)
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).into_owned())
        .unwrap_or_default()
}

// wpctl output: "Volume: 0.85" or "Volume: 0.85 [MUTED]"
fn read_volume() -> (f64, bool) {
    let out = wpctl(&["get-volume", "@DEFAULT_AUDIO_SINK@"]);
    let muted = out.contains("[MUTED]");
    let vol = out
        .split_whitespace()
        .nth(1)
        .and_then(|v| v.parse::<f64>().ok())
        .unwrap_or(0.0);
    (vol * 100.0, muted)
}

fn mute_label(muted: bool) -> &'static str {
    if muted { "Unmute" } else { "Mute" }
}

// Pick the speaker glyph based on the current volume and mute state
fn vol_icon(vol: f64, muted: bool) -> &'static str {
    if muted || vol <= 0.0 {
        ICON_MUTED
    } else if vol < 50.0 {
        ICON_LOW
    } else {
        ICON_HIGH
    }
}

// Left-aligned button with an icon and a text label
fn icon_button(icon: &str, text: &str) -> (Button, Label, Label) {
    let icon_lbl = Label::new(Some(icon));
    icon_lbl.set_width_chars(2);
    let text_lbl = Label::new(Some(text));

    let content = gtk4::Box::new(Orientation::Horizontal, 10);
    content.set_halign(gtk4::Align::Start);
    content.append(&icon_lbl);
    content.append(&text_lbl);

    let btn = Button::new();
    btn.set_child(Some(&content));
    (btn, icon_lbl, text_lbl)
}

fn build_ui(app: &Application) {
    // Load the rofi-inspired stylesheet
    let provider = CssProvider::new();
    provider.load_from_string(CSS);
    gtk4::style_context_add_provider_for_display(
        &gdk::Display::default().expect("no display"),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let window = ApplicationWindow::builder().application(app).build();

    // Layer-shell setup: overlay surface anchored to the bottom-left corner
    window.init_layer_shell();
    window.set_layer(Layer::Overlay);
    window.set_anchor(Edge::Bottom, true);
    window.set_anchor(Edge::Left, true);
    window.set_margin(Edge::Bottom, MARGIN_BOTTOM);
    window.set_margin(Edge::Left, MARGIN_LEFT);
    // Exclusive keyboard grab
    window.set_keyboard_mode(KeyboardMode::Exclusive);
    window.set_default_size(320, -1);

    // Closing the flyout (Escape, mixer button, outside click) ends the whole app
     let app_c = app.clone();
    window.connect_close_request(move |_| {
        app_c.quit();
        glib::Propagation::Proceed
    });

    // Click-catcher: a transparent fullscreen surface below the flyout
    let catcher = ApplicationWindow::builder().application(app).build();
    catcher.init_layer_shell();
    catcher.set_layer(Layer::Top);
    for edge in [Edge::Top, Edge::Bottom, Edge::Left, Edge::Right] {
        catcher.set_anchor(edge, true);
    }
    // -1 means "ignore other surfaces' exclusive zones", so it covers the bar too
    catcher.set_exclusive_zone(-1);
    catcher.add_css_class("catcher");

    let click = GestureClick::new();
    let popup = window.clone();
    click.connect_pressed(move |_, _, _, _| popup.close());
    catcher.add_controller(click);

    let (vol, muted) = read_volume();
    // Shared mute state, so the slider and the mute button closures stay in sync
    let muted_state = Rc::new(Cell::new(muted));

    let (mute_btn, mute_icon, mute_text) =
        icon_button(vol_icon(vol, muted), mute_label(muted));

    let scale = Scale::with_range(Orientation::Horizontal, 0.0, 100.0, 1.0);
    scale.set_hexpand(true);
    scale.set_draw_value(false);
    scale.set_value(vol);

    let pct = Label::new(Some(&format!("{:.0}%", vol)));
    pct.set_width_chars(5);

    // Slider: apply the volume, update the percentage and the icon
    let pct_c = pct.clone();
    let icon_c = mute_icon.clone();
    let state_c = muted_state.clone();
    scale.connect_value_changed(move |s| {
        let v = s.value();
        wpctl(&["set-volume", "-l", "1.0", "@DEFAULT_AUDIO_SINK@", &format!("{:.2}", v / 100.0)]);
        pct_c.set_text(&format!("{:.0}%", v));
        icon_c.set_text(vol_icon(v, state_c.get()));
    });

    let row = gtk4::Box::new(Orientation::Horizontal, 10);
    row.append(&scale);
    row.append(&pct);

    // Mute button: toggle, then re-read the real state from wpctl
    let scale_c = scale.clone();
    let state_c = muted_state.clone();
    mute_btn.connect_clicked(move |_| {
        wpctl(&["set-mute", "@DEFAULT_AUDIO_SINK@", "toggle"]);
        let (_, m) = read_volume();
        state_c.set(m);
        mute_icon.set_text(vol_icon(scale_c.value(), m));
        mute_text.set_text(mute_label(m));
    });

    // Mixer button: open pavucontrol and close the flyout
    let (mixer_btn, _, _) = icon_button(ICON_MIXER, "Mixer");
    let w = window.clone();
    mixer_btn.connect_clicked(move |_| {
        let _ = Command::new("pavucontrol").spawn();
        w.close();
    });

    let root = gtk4::Box::new(Orientation::Vertical, 10);
    root.append(&row);
    root.append(&mute_btn);
    root.append(&mixer_btn);
    window.set_child(Some(&root));

    // Close the flyout with Escape
    let key = gtk4::EventControllerKey::new();
    let w = window.clone();
    key.connect_key_pressed(move |_, k, _, _| {
        if k == gdk::Key::Escape {
            w.close();
            glib::Propagation::Stop
        } else {
            glib::Propagation::Proceed
        }
    });
    window.add_controller(key);

    // Catcher first, flyout on top of it
    catcher.present();
    window.present();
}

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("dev.jota.volume-flyout")
        .build();
    app.connect_activate(build_ui);
    app.run()
}
