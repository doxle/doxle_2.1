use crate::bevy_backend::BevyBackend;
use dioxus_native::{CustomPaintCtx, CustomPaintSource, DeviceHandle, TextureHandle};
use wgpu::Instance;
use doxle_core::polygon::Polygon;
use std::sync::mpsc::{channel, Receiver, Sender};

//Sends msg to bevy on what to what to update
pub enum RendererMessage {
    UpdatePolygon(Polygon),
}

enum RendererState {
    Active(BevyBackend),
    Suspended,
}

//Renderer's mailbox - checks every frame so 60 fps which is
// 60 times per second so its constantly redraws if there are
// no changes
pub struct PolygonPaintSource {
    state: RendererState,
    tx: Sender<RendererMessage>,
    rx: Receiver<RendererMessage>,
    polygon: Polygon,
}

impl PolygonPaintSource {
    pub fn new() -> Self {
        let (tx, rx) = channel();
        Self {
            state: RendererState::Suspended,
            tx,
            rx,
            polygon: Polygon::new(),
        }
    }
    // Clone the sender before we move
    pub fn sender(&self) -> Sender<RendererMessage> {
        self.tx.clone()
    }
}

impl CustomPaintSource for PolygonPaintSource {
    //Called when renderer starts or resumes
    //Later for wgpu setup
    fn resume(&mut self, instance: &Instance, device_handle: &DeviceHandle) {
        let backend = BevyBackend::new(instance, device_handle);
        self.state = RendererState::Active(backend);
    }

    //Called when renderer pauses , like window minimize etc
    fn suspend(&mut self) {
        self.state = RendererState::Suspended;
    }

    //Called every frame (~60 fps) = meaning 60 times per second
    fn render(
        &mut self,
        ctx: CustomPaintCtx<'_>,
        width: u32,
        height: u32,
        _scale: f64,
    ) -> Option<TextureHandle> {
        //Step 1 - Check for message from UI
        while let Ok(msg) = self.rx.try_recv() {
            //Non-blocking check
            match msg {
                RendererMessage::UpdatePolygon(polygon) => {
                    self.polygon = polygon;
                }
            }
        }
        //Step 2 Draw the polygon if active
        match &mut self.state {
            RendererState::Active(backend) => backend.render(ctx, &self.polygon, width, height),
            RendererState::Suspended => None,
        }
    }
}
