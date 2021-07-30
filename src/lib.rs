#![allow(incomplete_features)]
#![feature(generic_associated_types)]
#![feature(min_specialization)]

use baseplug::{Plugin, ProcessContext, WindowOpenResult};
use baseview::{Size, WindowOpenOptions, WindowScalePolicy};
use raw_window_handle::HasRawWindowHandle;
use serde::{Deserialize, Serialize};
mod ui;

baseplug::model! {
    #[derive(Debug, Serialize, Deserialize)]
    struct MyModel {
        click_amplitude: f32,
    }
}

impl Default for MyModel {
    fn default() -> Self {
        Self {
            click_amplitude: 1.0
        }
    }
}

struct MyPlugin {}

impl Plugin for MyPlugin {
    const NAME: &'static str = "visual metronome";
    const PRODUCT: &'static str = "visual metronome";
    const VENDOR: &'static str = "DJ Monad";

    const INPUT_CHANNELS: usize = 2;
    const OUTPUT_CHANNELS: usize = 2;

    type Model = MyModel;

    #[inline]
    fn new(_sample_rate: f32, _model: &MyModel) -> Self {
        // simple_logging::log_to_file("test.log", log::LevelFilter::Trace).unwrap();
        Self {
        }
    }

    #[inline]
    fn process(&mut self, _model: &MyModelProcess, _ctx: &mut ProcessContext<Self>) {

    }
}

impl baseplug::PluginUI for MyPlugin {
    type Handle = iced_baseview::WindowHandle<ui::Message>;

    fn ui_size() -> (i16, i16) {
        (230, 130)
    }

    fn ui_open(parent: &impl HasRawWindowHandle) -> WindowOpenResult<Self::Handle> {
        let settings = iced_baseview::Settings {
            window: WindowOpenOptions {
                title: String::from("iced-baseplug-examples gain"),
                size: Size::new(Self::ui_size().0 as f64, Self::ui_size().1 as f64),
                scale: WindowScalePolicy::SystemScaleFactor,
            },
            flags: (),
        };

        let handle = iced_baseview::IcedWindow::<ui::MyPluginUI>::open_parented(parent, settings);

        Ok(handle)
    }

    fn ui_param_notify(
        _handle: &Self::Handle,
        _param: &'static baseplug::Param<Self, <Self::Model as baseplug::Model<Self>>::Smooth>,
        _val: f32,
    ) {
        // TODO: implement this
    }

    fn ui_close(mut handle: Self::Handle) {
        handle.close_window().unwrap();
    }
}

baseplug::vst2!(MyPlugin, b"tAnE");