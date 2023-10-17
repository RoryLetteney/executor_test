//! Game project.
use std::collections::HashMap;

use fyrox::{
    core::pool::Handle,
    event::Event,
    event_loop::ControlFlow,
    gui::{message::UiMessage, widget::WidgetBuilder, image::ImageBuilder},
    plugin::{Plugin, PluginConstructor, PluginContext, PluginRegistrationContext},
    scene::{Scene, loader::AsyncSceneLoader},
    core::log::Log, utils::into_gui_texture, resource::texture::Texture
};

pub struct GameConstructor;

impl PluginConstructor for GameConstructor {
    fn register(&self, _context: PluginRegistrationContext) {
        // Register your scripts here.
    }

    fn create_instance(
        &self,
        override_scene: Handle<Scene>,
        context: PluginContext,
    ) -> Box<dyn Plugin> {
        Box::new(Game::new(override_scene, context))
    }
}

pub struct Game {
    scene: Handle<Scene>,
    loader: Option<AsyncSceneLoader>,
}

impl Game {
    pub fn new(override_scene: Handle<Scene>, context: PluginContext) -> Self {
        let mut loader = None;
        let scene = if override_scene.is_some() {
            override_scene
        } else {
            loader = Some(AsyncSceneLoader::begin_loading(
                "data/scene.rgs".into(),
                context.serialization_context.clone(),
                context.resource_manager.clone(),
            ));
            Default::default()
        };

        let mut suffix_map = HashMap::new();

        suffix_map.insert(0, "".to_string());
        suffix_map.insert(1, "K".to_string());
        suffix_map.insert(2, "M".to_string());
        suffix_map.insert(3, "B".to_string());
        suffix_map.insert(4, "T".to_string());
        suffix_map.insert(5, "Qu".to_string());
        suffix_map.insert(6, "Sx".to_string());
        suffix_map.insert(7, "Sp".to_string());
        suffix_map.insert(8, "Oc".to_string());
        suffix_map.insert(9, "No".to_string());
        suffix_map.insert(10, "De".to_string());

        let screen_size = context.user_interface.screen_size();
        let screen_width = screen_size[0];
        let screen_height = screen_size[1];

        let background_image_handle = ImageBuilder::new(
            WidgetBuilder::new()
                .with_width(screen_width)
                .with_height(screen_height),
        )
        .with_texture(into_gui_texture(
            context
                .resource_manager
                .request::<Texture, _>("data/stage_resized.png"),
        ))
        .build(&mut context.user_interface.build_ctx());

        Self { scene, loader }
    }
}

impl Plugin for Game {
    fn on_deinit(&mut self, _context: PluginContext) {
        // Do a cleanup here.
    }

    fn update(&mut self, context: &mut PluginContext, _control_flow: &mut ControlFlow) {
         if let Some(loader) = self.loader.as_ref() {
            if let Some(result) = loader.fetch_result() {
                match result {
                    Ok(scene) => {
                        self.scene = context.scenes.add(scene);
                    }
                    Err(err) => Log::err(err),
                }
            }
        }
    
        // Add your global update code here.
    }

    fn on_os_event(
        &mut self,
        _event: &Event<()>,
        _context: PluginContext,
        _control_flow: &mut ControlFlow,
    ) {
        // Do something on OS event here.
    }

    fn on_ui_message(
        &mut self,
        _context: &mut PluginContext,
        _message: &UiMessage,
        _control_flow: &mut ControlFlow,
    ) {
        // Handle UI events here.
    }
}
