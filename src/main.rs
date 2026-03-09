// ============ IMPORTS ============
use iced_layershell::{application, reexport::Anchor, settings::{LayerShellSettings, Settings, StartMode}, to_layer_message};
use iced::{ContentFit, Element, Length, Task, time, widget::text};
use iced::widget::{image, image::Handle};
use std::time::Duration;





// ============ CRATES ============
use crate::ron::{BackgroundConfig, read_ron_config, reload_ron_config};
use crate::fs::check_if_config_file_exists;
use crate::helpers::misc::preload_image;




// ============ MOD'S ============
pub mod helpers;
pub mod ron;
pub mod fs;





// ============ STRUCTS/ENUM'S ============
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppData 
{
    pub preloaded_image_handle: Option<Handle>,
    pub formatted_content_fit: ContentFit,
    pub bg_config: BackgroundConfig,
}




// ============ FUNCTIONS ============
fn main() -> Result<(), iced_layershell::Error>
{
    check_if_config_file_exists();
    let ron_config = read_ron_config(false);
    let start_mode = match ron_config.display { Some(ref output) => StartMode::TargetScreen(output.to_string()), None => StartMode::Active };
    let formatted_content_fit = ron_config.content_fit.into();
    let handle_option = preload_image(&ron_config.wallpaper);

    let app_data = AppData 
    {
        preloaded_image_handle: handle_option,
        bg_config: ron_config,
        formatted_content_fit,
    };

    application(move || app_data.clone(), namespace, update, view).subscription(subscription).settings(Settings
    {
        layer_settings: LayerShellSettings
        {
            size: None,
            anchor: Anchor::Top | Anchor::Bottom | Anchor::Left | Anchor::Right,
            layer: iced_layershell::reexport::Layer::Background,
            keyboard_interactivity: iced_layershell::reexport::KeyboardInteractivity::None,
            start_mode,
            ..Default::default()
        },
        ..Default::default()
    }).run()
}
fn namespace() -> String { String::from("icebg") }



#[to_layer_message]
#[derive(Debug, Clone)]
enum Message { Tick }
fn update(app_data: &mut AppData, message: Message) -> Task<Message> 
{ 
    if let Message::Tick = message { reload_ron_config(app_data) }
    Task::none()
}



fn subscription(app_data: &AppData) -> iced::Subscription<Message>
{
    if let Some(update_interval) = app_data.bg_config.update_interval
    {
        time::every(Duration::from_millis(update_interval)).map(|_| Message::Tick)
    }
    else
    {
        iced::Subscription::none()
    }
}



fn view(app_data: &AppData) -> Element<'_, Message> 
{
    if let Some(handle) = &app_data.preloaded_image_handle
    {
        image(handle).content_fit(app_data.formatted_content_fit).width(Length::Fill).height(Length::Fill).into()
    }
    else
    {
        text("Warning!!!: Parsed Image Doesn't Exists!!!").height(Length::Fill).width(Length::Fill).size(30).center().into()
    }
}
