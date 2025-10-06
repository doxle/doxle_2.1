use color::palette::css::WHITE;
use color::{OpaqueColor, Srgb, parse_color};
use demo_renderer::{DemoMessage, DemoPaintSource};
use dioxus::prelude::*;
use dioxus_native::use_wgpu;
use std::any::Any;
use wgpu::Limits;

mod bevy_renderer;
mod bevy_scene_plugin;
mod demo_renderer;

// CSS Styles
static STYLES: Asset = asset!("/src/styles.css");

type Color = OpaqueColor<Srgb>;

//Default is 4 or 8, bevy is complex and might crash so we will need 12
fn limits() -> Limits {
    Limits {
        max_storage_buffers_per_shader_stage: 12,
        ..Limits::default()
    }
}

fn main() {
    #[cfg(feature = "tracing")]
    tracing_subscriber::fmt::init();

    let config: Vec<Box<dyn Any>> = vec![Box::new(limits())];
    dioxus_native::launch_cfg(app, Vec::new(), config); //limits are passed to dioxus_native to suse wgpu
}

fn app() -> Element {
    let show_cube = use_signal(|| true);
    let color_str = use_signal(|| String::from("red"));
    let color = use_memo(move || {
        parse_color(&color_str())
            .map(|c| c.to_alpha_color())
            .unwrap_or(WHITE)
            .split()
            .0
    });
    rsx! {
        document::Stylesheet { href: STYLES }
        main{
            style:"
                height:100%;
                width:100%;
                display: grid;
                grid-template-rows: 100px 1fr;
                grid-template-columns: 100%;
                background: #f4e8d2;
            ",

            header {
                "Blitz Bevy Demo"
            }
            div {
                style: "
                position:absolute;
                width:33%;
                height:100%;
                z-index:-10;
                background-color:black;
                padding-top:40%;
                color:white;
                ",

                h2 {"Underlay"},
                p {"This is under bevy cube"}
            }

            div{
                style:"
                position:absolute;
                width:33%;
                height:100%;
                right:0;
                z-index:10;
                background-color:rgba(0,0,0,0.5);
                padding-top:40%;
                color:white;
                ",
                h2{"Overlay"}
                p {"This is overlaid on top of bevy"}
            }

            if show_cube(){
                SpinningCube{ color: color }
            }
        }

    }
}

#[component]
fn SpinningCube(color: Memo<Color>) -> Element {
    let paint_source = DemoPaintSource::new();
    let sender = paint_source.sender();
    let paint_source_id = use_wgpu(move || paint_source);
    use_effect(move || {
        sender.send(DemoMessage::SetColor(color())).unwrap();
    });
    rsx! {
        div{
            id:"canvas-container",
            style: "
                display:grid;
                opacity:0.8;
                grid-row:2;
            ",
            canvas{
                id:"demo-canvas",
                style: "
                width: 100%;
                height: 100%;
                ",
                "src" : paint_source_id
            }
        }
    }
}
