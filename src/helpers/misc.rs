// ============ IMPORTS ============
use iced::widget::image::Handle;
use std::path::Path;





// ============ FUNCTIONS ============
pub fn preload_image(path: &str) -> Option<Handle>
{
    println!("\n=== WALLPAPER PRELOAD ===");
    if Path::new(path).exists()
    {
        println!("Preloading Wallpaper, Please Wait...");
        let option_handle = Some(Handle::from_path(path));
        println!("Image Preload Completed Successfully!!!");
        option_handle
    }
    else 
    { 
        eprintln!("WARNING: Wallpaper path does not exist: '{path}'");
        None
    }
}
