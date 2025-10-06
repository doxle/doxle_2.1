use crate::bevy_scene_plugin::BevyScenePlugin;
use bevy::{
    prelude::*,
    render::{
        RenderPlugin,
        camera::{ManualTextureView, ManualTextureViewHandle, ManualTextureViews, RenderTarget},
        render_resource::TextureFormat,
        renderer::{
            RenderAdapter, RenderAdapterInfo, RenderDevice, RenderInstance, RenderQueue,
            WgpuWrapper,
        },
        settings::{RenderCreation, RenderResources},
    },
};
use dioxus_native::{CustomPaintCtx, DeviceHandle, TextureHandle};
use std::sync::Arc;
use wgpu::Instance;

#[derive(Resource, Default)]
pub struct UIData {
    pub color: [f32; 3],
}

pub struct BevyRenderer {
    app: App,
    wgpu_device: wgpu::Device,
    last_texture_size: (u32, u32),
    texture_handle: Option<TextureHandle>,
    manual_texture_view_handle: Option<ManualTextureViewHandle>,
}

impl BevyRenderer {
    pub fn new(instance: &Instance, device_handle: &DeviceHandle) -> Self {
        //Creates a headless Bevy App
        let mut app = App::new();
        app.add_plugins(
            DefaultPlugins
                .set(RenderPlugin {
                    //Reuse the render resources from Dioxus Native Renderer
                    render_creation: RenderCreation::Manual(RenderResources(
                        RenderDevice::new(WgpuWrapper::new(device_handle.device.clone())),
                        RenderQueue(Arc::new(WgpuWrapper::new(device_handle.queue.clone()))),
                        RenderAdapterInfo(WgpuWrapper::new(device_handle.adapter.get_info())),
                        RenderAdapter(Arc::new(WgpuWrapper::new(device_handle.adapter.clone()))),
                        RenderInstance(Arc::new(WgpuWrapper::new(instance.clone()))),
                    )),
                    synchronous_pipeline_compilation: true,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: None,
                    exit_condition: bevy::window::ExitCondition::DontExit,
                    close_when_requested: false,
                })
                .disable::<bevy::winit::WinitPlugin>(),
        );
        //Set up rendering to texture
        app.insert_resource(ManualTextureViews::default());

        //Add data from the UI
        app.insert_resource(UIData::default());

        //Add the bevy scene
        app.add_plugins(BevyScenePlugin {});

        app.finish();
        app.cleanup();

        Self {
            app,
            wgpu_device: device_handle.device.clone(),
            last_texture_size: (0, 0),
            texture_handle: None,
            manual_texture_view_handle: None,
        }
    }

    pub fn render(
        &mut self,
        ctx: CustomPaintCtx<'_>,
        color: [f32; 3],
        width: u32,
        height: u32,
        _start_time: &std::time::Instant,
    ) -> Option<TextureHandle> {
        if let Some(mut ui) = self.app.world_mut().get_resource_mut::<UIData>() {
            ui.color = color;
        }

        //Init self.texture_handle if None or if width/height changed;
        self.init_texture(ctx, width, height);
        //Run one frame of the Bevy to render the 3D scene
        self.app.update();
        self.texture_handle
    }

    //Texture = Whiteboard only changes on  resize; Polygons and annotations are drawn on the texture
    // You can draw multiple images in a texture
    fn init_texture(&mut self, mut ctx: CustomPaintCtx<'_>, width: u32, height: u32) {
        //Reuse self.texture_handle if there is no change in Whiteboard, no resize has happened
        let current_size = (width, height);
        if self.texture_handle.is_some() && self.last_texture_size == current_size {
            return;
        }

        //Skip if no camera
        let world = self.app.world_mut();
        if world.query::<&Camera>().single(world).is_err() {
            return;
        }

        if let Some(mut manual_texture_views) = world.get_resource_mut::<ManualTextureViews>() {
            //Clean previous dioxus texture if any
            if let Some(texture_handle) = self.texture_handle {
                ctx.unregister_texture(texture_handle);
                self.texture_handle = None;
            }
            // Clean previous bevy texture
            if let Some(old_handle) = self.manual_texture_view_handle {
                manual_texture_views.remove(&old_handle);
                self.manual_texture_view_handle = None;
            }

            //Create the texture for the camera target and the CustomPaintCtx
            let format = TextureFormat::Rgba8UnormSrgb;
            // Creates a 3D Scene stored in a 2D texture its like a 3D Movie in a 2d flat screen
            // Specifies all info about the texture only done during creating and is heavy GPU compute
            let wgpu_texture = self.wgpu_device.create_texture(&wgpu::TextureDescriptor {
                label: None,
                size: wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format,
                usage: wgpu::TextureUsages::TEXTURE_BINDING
                    | wgpu::TextureUsages::RENDER_ATTACHMENT
                    | wgpu::TextureUsages::COPY_SRC,
                view_formats: &[],
            });

            // This is to view the TextureDescriptor - Light GPU Operations for access only
            let wgpu_texture_view =
                wgpu_texture.create_view(&wgpu::TextureViewDescriptor::default());
            let manual_texture_view = ManualTextureView {
                texture_view: wgpu_texture_view.into(),
                size: bevy::math::UVec2::new(width, height),
                format,
            };

            let manual_texture_view_handle = ManualTextureViewHandle(0);
            manual_texture_views.insert(manual_texture_view_handle, manual_texture_view);

            //This is where Bevy's Camera renders to our Custom Texture instead of a window
            if let Ok(mut camera) = world.query::<&mut Camera>().single_mut(world) {
                //render to our texture, normally to the screen
                camera.target = RenderTarget::TextureView(manual_texture_view_handle);
                self.last_texture_size = current_size;
                self.manual_texture_view_handle = Some(manual_texture_view_handle);
                self.texture_handle = Some(ctx.register_texture(wgpu_texture)) //register with Dioxus
            }
        }
    }
}
