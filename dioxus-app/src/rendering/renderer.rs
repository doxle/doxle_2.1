use doxle_core::polygon::Polygon;
use std::sync::mpsc::{channel, Receiver, Sender}

//Only polygon messages can be can through the channel, nothing else
pub enum RendererMessage{
    UpdatePolygon(Polygon),
}

enum RendererState{
    Active,
    Suspended,
}

pub struct PolygonRenderer {
    state:RendererState,
    tx: Sender<RendererMessage>,
    rx: Receiver<RendererMessage>,
    polygon:Polygon,
}

impl PolygonRenderer {
    pub fn new() -> Self {
        Self {
            state: RendererState::Suspended,
            tx,
            rx,
            polygon:Polygon:new(),
        }
    }

    pub fn sender(&self) -> Sender <RendererMessage> {
        self.tx.clone()
    }
}

// CustomPaintSource is required by dioxus-native or it wont work
impl CustomPaintSource for PolygonRenderer {
    // This will trigger onload or when resumed after window has been minimised
    fn resume(&mut self, _instance:&Instance, _device_handle:&DeviceHandle) {
        self.state = RendererState::Active; //We can access RendererState through Self object here
        tracing::info!("Polygon render resumed");
    }

    // Window minimised
    fn suspend(&mut self){
        self.state = RendererState::Suspended;
        tracing::info!("Polygon render suspended");
    }

    // This is where we hook on to Bevy
    fn render(
        &mut self,
        _ctx:CustomPaintCtx<'_>,
        _width:u32,
        _height:u32,
        _scale:f64,
    )->Option<TextureHandle>{
        while let Ok(msg) = self.rx.try_recv() {
            match msg {
                RendererMessage::UpdatePolygon(polygon) => {
                    self.polygon = polygon; //New polygon from UI
                    tracing::debug!(
                        "Polygon updated: {} points",
                        self.polygon.count_points()
                    );
                }
            }
        }

        //TODO - Render Polygon with Bevy
        match &self.state {
            RendererState::Active => None,
            RendererState::Suspended => None,

        }

    }
}
