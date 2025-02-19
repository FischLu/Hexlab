use clap::Parser;
use config::{read_config, Config};
use std::process::exit;
use crate::options::Options;
use once_cell::sync::Lazy;
use std::sync::Mutex;

mod config;
mod error;
mod expression;
mod format;
mod options;
mod cmd;
mod gui_func;

static CONFIG: Lazy<Mutex<Config>> = Lazy::new(|| {
    let options = Options::parse();
    let config = match read_config(options.config.as_ref()) {
        Ok(conf) => conf,
        Err(err) => {
            eprintln!("Failed to parse config: {}", err);
            exit(1);
        }
    };
    Mutex::new(config)
});

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 使用已有的 Options 替代 Cli
    let options = Options::parse();
    
    // 当有任何命令行参数时，进入命令行模式
    if options.expr.is_some() || options.file.is_some() || options.interactive {
        let config = CONFIG.lock().unwrap();
        cmd::cmd_main((*config).clone(), options);
        return;
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![gui_func::evaluate_expression])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}