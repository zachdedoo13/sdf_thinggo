use std::iter;
use std::sync::{Mutex, MutexGuard};
use eframe::{App, CreationContext, Frame};
use eframe::egui_wgpu::Renderer;
use eframe::wgpu::{CommandEncoderDescriptor, Device, Extent3d, Queue, TextureFormat};
use egui::load::SizedTexture;
use egui::{Image, Ui, Vec2, Window};
use once_cell::sync::Lazy;
use crate::{get, init_static};
use crate::packages::time_package::TimePackage;
use crate::render_state::structs::EguiTexturePackage;
use crate::render_state::test::test_render_pipeline::TestRenderPipeline;


// Globals
init_static!(TIME: TimePackage => {TimePackage::new()});


pub struct MehApp {
    meh_renderer: MehRenderer,
}
impl MehApp {
    pub fn new(cc: &CreationContext<'_>) -> Self {
        let render_state = cc.wgpu_render_state.as_ref().unwrap();
        let renderer = &mut render_state.renderer.write();
        let device = &render_state.device;


        let meh_renderer = MehRenderer::new(device, renderer);


        Self {
            meh_renderer,
        }
    }

    fn update(&mut self) {
        get!(TIME).update();
    }

    fn render(&mut self, frame: &mut Frame) {
        let r_thing = frame.wgpu_render_state().unwrap();
        let device = &r_thing.device;
        let queue = &r_thing.queue;

        self.meh_renderer.render_pass(device, queue)

    }

    fn ui(&mut self, ctx: &egui::Context) {
        Window::new("test")
            .show(ctx, |ui| {
                self.meh_renderer.display(ui);
                ui.label(format!("{}", get!(TIME).fps));
            });
    }
}

impl App for MehApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        self.update();
        self.render(frame);
        self.ui(ctx);

        ctx.request_repaint();
    }
}


pub struct MehRenderer {
    pub test_render_pipeline: TestRenderPipeline,
    pub egui_texture_package: EguiTexturePackage,
}
impl MehRenderer {
    pub fn new(device: &Device, renderer: &mut Renderer) -> Self {
        let test_render_pipeline = TestRenderPipeline::new(device, TextureFormat::Rgba8Unorm);

        let egui_texture_package = EguiTexturePackage::new(Extent3d {
            width: 250,
            height: 250,
            depth_or_array_layers: 1,
        }, device, renderer);

        Self {
            test_render_pipeline,
            egui_texture_package,
        }
    }

    pub fn update(&mut self) {

    }

    pub fn render_pass(&self, device: &Device, queue: &Queue) {
        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("the only encoder"),
        });

        self.test_render_pipeline.render_pass(&mut encoder, &self.egui_texture_package.view);


        queue.submit(iter::once(encoder.finish()));
    }

    pub fn display(&self, ui: &mut Ui) {
        ui.add(
            Image::from_texture(
                SizedTexture::new(
                    self.egui_texture_package.texture_id,
                    Vec2::new(
                        self.egui_texture_package.texture.size().width as f32,
                        self.egui_texture_package.texture.size().height as f32
                    )
                )
            )
        );
    }
}