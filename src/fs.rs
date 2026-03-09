// ============ IMPORTS ============
use std::{io::Write, fs, fs::File};





// ============ FUNCTIONS ============
pub fn check_if_config_file_exists()
{
    println!("\n=== FS CHECK RUNNING... ===");
    let home_path = home::home_dir().expect("Failed To Get Home Directory");
    let ron_config_dir = home_path.join(".config/icebg");
    let ron_config_file = ron_config_dir.join("config_bg.ron");


    if ron_config_dir.exists()
    {
        println!("Ron Config Directory Exists!!!");
    }
    else
    {
        println!("Ron config directory doesn't exist, Creating...");
        fs::create_dir_all(ron_config_dir).expect("Couldn't Create Ron Config Directory");
    };


    if ron_config_file.exists()
    {
        println!("Ron Config File Exists!!!");
    }
    else
    {
        println!("Ron config file doesn't exist, Creating...");
        let ron_default_data = 

r#"//Tip: Available options for "content_fit" are: "Contain", "Cover", "Fill", "None", "ScaleDown"
BackgroundConfig
(
    display: None,
    wallpaper: "/path/to/your/wallpaper.png",
    content_fit: Fill,
    update_interval: Some(1000)
)"#;

        let mut file = File::create(ron_config_file).expect("Couldn't Create Default Config File");
        file.write_all(ron_default_data.as_bytes()).expect("Couldn't Create Default Config File");
    };
}

