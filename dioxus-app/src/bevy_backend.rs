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
use doxle_core::polygon::Polygon;
use std::sync::Arc;
use wgpu::Instance;

pub struct BevyBackend {
    app: App,
    wgpu_device: wgpu::Device,
    last_texture_size: (u32, u32),
    texture_handle: Option<TextureHandle>,
    manual_texture_view_handle: Option<ManualTextureViewHandle>,
}

impl BevyBackend {
    pub fn new(instance: &Instance, device_handle: &DeviceHandle) -> Self {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins.set(
            RenderPlugin {
                render_creation:RenderCreation::Manual(RenderResources(
                    RenderDevice::new(WgpuWrapper::new(device_handle.device.clone())),
                    RenderQueue(Arc::new(WgpuWrapper::new(device_handle.queue.clone()))),
                    RenderAdapterInfo(WgpuWrapper::new(device_handle.adapter.get_info())),
                    RenderAdapter(Arc::new(WgpuWrapper::new(device_handle.adapter.clone()))),
                    RenderInstance(Arc::new(WgpuWrapper::new(instance.clone()))),
                )),
                synchronous_pipeline_compilation:true,
                ..default()
            })
            .set(WindowPlugin {
                            primary_window: None,
                            exit_condition: bevy::window::ExitCondition::DontExit,
                            close_when_requested: false,
                        })
                        .disable::<bevy::winit::WinitPlugin>(),
        );
        app.insert_resource(ManualTextureViews::default());

        // BLUE BACKGROUND
        app.insert_resource(ClearColor(bevy::color::Color::srgb(0.0, 0.0, 1.0)));

        // Add camera
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn((
                Camera3d::default(),
                Transform::from_xyz(0.0, 0.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
            ));
        });
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
        _polygon: &Polygon,
        width: u32,
        height: u32,
    ) -> Option<TextureHandle> {
        println!("[BEVY] render called with width={}, height={}", width, height);
        if width == 0 || height == 0 {
            println!("[BEVY] Skipping render - zero dimensions");
            return None;
        }
        self.init_texture(ctx, width, height);
        self.app.update();
        println!("[BEVY] texture_handle exists = {}", self.texture_handle.is_some());
        self.texture_handle
    }

    fn init_texture(&mut self, mut ctx: CustomPaintCtx<'_>, width: u32, height: u32) {
            let current_size = (width, height);
            if self.texture_handle.is_some() && self.last_texture_size == current_size {
                return;
            }

            let world = self.app.world_mut();

            if world.query::<&Camera>().single(world).is_err() {
                return;
            }

            if let Some(mut manual_texture_views) = world.get_resource_mut::<ManualTextureViews>() {
                if let Some(texture_handle) = self.texture_handle {
                    ctx.unregister_texture(texture_handle);
                    self.texture_handle = None;
                }
                if let Some(old_handle) = self.manual_texture_view_handle {
                    manual_texture_views.remove(&old_handle);
                    self.manual_texture_view_handle = None;
                }

                let format = TextureFormat::Rgba8UnormSrgb;
                let wgpu_texture = self.wgpu_device.create_texture(&wgpu::TextureDescriptor {
                    label: None,
                    size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format,
                    usage: wgpu::TextureUsages::TEXTURE_BINDING
                        | wgpu::TextureUsages::RENDER_ATTACHMENT
                        | wgpu::TextureUsages::COPY_SRC,
                    view_formats: &[],
                });

                let wgpu_texture_view = wgpu_texture.create_view(&wgpu::TextureViewDescriptor::default());
                let manual_texture_view = ManualTextureView {
                    texture_view: wgpu_texture_view.into(),
                    size: bevy::math::UVec2::new(width, height),
                    format,
                };
                let manual_texture_view_handle = ManualTextureViewHandle(0);
                manual_texture_views.insert(manual_texture_view_handle, manual_texture_view);

                if let Ok(mut camera) = world.query::<&mut Camera>().single_mut(world) {
                    camera.target = RenderTarget::TextureView(manual_texture_view_handle);
                    self.last_texture_size = current_size;
                    self.manual_texture_view_handle = Some(manual_texture_view_handle);
                    self.texture_handle = Some(ctx.register_texture(wgpu_texture));
                }
            }
    }
}
