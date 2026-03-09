// ============ IMPORTS ============
use serde::{Deserialize, Serialize};
use iced::ContentFit;
use std::fs;





// ============ CRATES ============
use crate::{AppData, helpers::misc::preload_image};





// ============ STRUCTS/ENUM'S ============
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum UserContentFit
{
    /// Scale as big as it can be without needing to crop or hide parts.
    #[default]
    Contain,
    /// Scale the image to cover all of the bounding box, cropping if needed.
    Cover,
    /// Distort the image so the widget is 100% covered without cropping.
    Fill,
    /// Don't resize or scale the image at all.
    None,
    /// Scale the image down if it's too big for the space, but never scale it
    ScaleDown,
}

#[derive(Default, Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(default)]
pub struct BackgroundConfig
{
    pub display: Option<String>,
    pub wallpaper: String,
    pub content_fit: UserContentFit, 
    pub update_interval: Option<u64>
}





// ============ IMPLEMENTATIONS ============
impl From<UserContentFit> for ContentFit
{
    fn from(val: UserContentFit) -> Self
    {
        match val
        {
            UserContentFit::Contain => ContentFit::Contain,
            UserContentFit::Cover => ContentFit::Cover,
            UserContentFit::Fill => ContentFit::Fill,
            UserContentFit::None => ContentFit::None,
            UserContentFit::ScaleDown => ContentFit::ScaleDown,
        }
    }
}





// ============ FUNCTIONS ============
pub fn read_ron_config(is_reload: bool) -> BackgroundConfig
{
    if !is_reload { println!("\n=== READING CONFIG FILE ==="); };
    let home_path = home::home_dir().expect("Failed To Get Home Directory");
    let path = home_path.join(".config/icebg/config_bg.ron");

    let content = fs::read_to_string(&path).unwrap_or_else(|e| panic!("Failed to read config: {e}"));
    if !is_reload { println!("Config loaded successfully!!!."); };
    ron::from_str::<BackgroundConfig>(&content).unwrap_or_else(|e| panic!("RON parse error:\n{e}"))
}



pub fn reload_ron_config(app_data: &mut AppData)
{
    let ron_config = read_ron_config(true);

    if ron_config.wallpaper != app_data.bg_config.wallpaper 
    { 
        println!("\n=== RELOAD CONFIG FILE ===");
        println!("Wallpaper Updated!!!");
        app_data.bg_config.wallpaper = ron_config.wallpaper;
        app_data.preloaded_image_handle = preload_image(&app_data.bg_config.wallpaper)
    } 

    if ron_config.content_fit != app_data.bg_config.content_fit 
    { 
        println!("\n=== RELOAD CONFIG FILE ===");
        println!("Content Fit Updated!!!");
        app_data.bg_config.content_fit = ron_config.content_fit;
        app_data.formatted_content_fit = app_data.bg_config.content_fit.into()
    };

    if ron_config.update_interval != app_data.bg_config.update_interval 
    {
        println!("\n=== RELOAD CONFIG FILE ===");
        println!("Update Interval Updated!!!");
        app_data.bg_config.update_interval = ron_config.update_interval;
    }

    if ron_config.display != app_data.bg_config.display
    {
        println!("\n=== RELOAD CONFIG FILE ===");
        eprintln!("Warning!!!: Changing the display needs an reboot in the app");
    }
}
