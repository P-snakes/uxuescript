pub mod llm;

use super::CommandsResult;

use crate::config::{metadata::MetadataConfig, options::OptionsConfig, CONFIG};

/// 获取包含版本及作者信息的应用元数据。
#[tauri::command]
#[specta::specta]
pub fn metadata() -> MetadataConfig {
    log::debug!("正在获取元数据...");
    let metadata = CONFIG.metadata.clone();
    log::info!("成功获取元数据: {:?}", metadata);
    metadata
}

#[tauri::command]
#[specta::specta]
pub fn options() -> OptionsConfig {
    log::debug!("正在获取配置信息...");
    let options = *CONFIG.options.lock();
    log::info!("成功获取配置信息: {:?}", options);
    options
}

#[tauri::command]
#[specta::specta]
pub fn set_options(options: OptionsConfig) {
    log::debug!("正在设置配置信息...");
    *CONFIG.options.lock() = options;
    log::info!("成功设置配置信息: {:?}", options);
}

/// 将内存中的全局配置持久化保存至本地文件。
#[tauri::command]
#[specta::specta]
pub fn save_config() -> CommandsResult<()> {
    log::debug!("正在保存配置文件...");
    CONFIG.save()?;
    log::info!("成功保存配置文件");
    Ok(())
}
