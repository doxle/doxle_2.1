use crate::renderer::{PolygonPaintSource, RendererMessage};
use dioxus::prelude::*;
use dioxus_native::use_wgpu;
use doxle_core::{app_state::AppState, point::Point};

pub fn app() -> Element {
    println!("[UI] App component rendering ....");
    let mut app_state = use_signal(|| {
        println!("[UI] Creating initial AppState");
        AppState::new()
    });

    rsx! {
        div{
            h1 {"Polygon Drawing App"}
            PolygonCanvas { app_state }


            button {
                 onclick:move |_| {
                     app_state.write().clear_polygon();
                     println!("[UI] Polygon cleared");
                 },
                 "Clear"
            }
        }
    }
}

#[component]
fn PolygonCanvas(app_state: Signal<AppState>) -> Element {
    //Step 1: Create renderer
    let paint_source = PolygonPaintSource::new();
    println!("[UI] Created Polygon Paint Source");

    //Step 2: Get sender BEFORE moving paint_source
    let sender = paint_source.sender();
    println!("[UI] Got sender from paint_source before moving");

    //Step 3: Register the use_wpu (this moves paint_source into Dioxus
    let paint_source_id = use_wgpu(move || {
        println!("[UI] use_wgpu closure called - registering paint source");
        paint_source
    });
    println!("[UI] Got paint_source_id {:?}", paint_source_id);

    use_effect(move || {
        let polygon = app_state.read().polygon.clone();
        println!(
            "[UI] use_effect triggered - sending polygon with {} points",
            polygon.count_points()
        );

        if let Err(e) = sender.send(RendererMessage::UpdatePolygon(polygon)) {
            println!("[UI] ERROR: Failed to send message: {}", e);
        }
    });

    rsx! {
        // The canvas where renderer draws
        div {
            style: "width: 800px; height: 600px;",
            canvas {
                id: "polygon-canvas",
                style: "border: 2px solid red; width: 100%; height: 100%;",
                "src": paint_source_id, //target paint source
                onclick: move |evt| {
                let x = evt.data.page_coordinates().x as f32;
                let y = evt.data.page_coordinates().y as f32;
                println!("[UI] Canvas clicked at  ({} {})", x, y);

                // Convert to Point and add to Polygon
                let point = Point {x,y};
                println!("[UI] Adding Point: {:?}", point);

                app_state.write().handle_click(point);
                println!("[UI] AppState updated, polygon now has {} points",
                app_state.read().polygon.count_points());

                }
            }
        }
    }
}
