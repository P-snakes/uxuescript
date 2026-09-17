pub mod ollama;

use super::CommandsResult;

use crate::config::{llm::LLMProvider, CONFIG};

/// 获取当前可用的全部大语言模型提供商列表。
#[tauri::command]
#[specta::specta]
pub fn providers() -> Vec<LLMProvider> {
    log::debug!("正在获取可用 AI Provider 列表...");
    let providers_map = CONFIG.llm.providers.lock();
    let mut providers: Vec<LLMProvider> = providers_map.values().cloned().collect();
    providers.sort_by_key(|p| p.name.clone());
    log::info!("成功获取 AI Provider 列表: {:?}", providers);
    providers
}

/// 获取当前选中的大语言模型提供商。
#[tauri::command]
#[specta::specta]
pub fn current_provider() -> String {
    let provider = CONFIG.llm.active_provider.lock().clone();
    log::debug!("正在获取当前 AI Provider: {}", provider);
    provider
}

/// 新增或更新一个自定义大语言模型提供商。
#[tauri::command]
#[specta::specta]
pub fn upsert_provider(mut provider: LLMProvider) -> CommandsResult<()> {
    provider.name = provider.name.trim().to_owned();
    if provider.name.is_empty() {
        return Err(anyhow::anyhow!("AI Provider 名称不能为空").into());
    }
    if !provider.is_custom {
        return Err(anyhow::anyhow!("只能新增或更新自定义 AI Provider").into());
    }

    let mut providers = CONFIG.llm.providers.lock();
    let name = provider.name.clone();
    if let Some(existing) = providers.get(&name) {
        if !existing.is_custom {
            return Err(anyhow::anyhow!("内置 AI Provider [{}] 不允许覆盖", name).into());
        }
    }

    let was_updated = providers.insert(name.clone(), provider).is_some();
    if was_updated {
        log::info!("成功更新 AI Provider: {}", name);
    } else {
        log::info!("成功添加 AI Provider: {}", name);
    }
    Ok(())
}

/// 移除一个自定义大语言模型提供商。
#[tauri::command]
#[specta::specta]
pub fn remove_provider(name: String) -> CommandsResult<()> {
    let name = name.trim().to_owned();
    if name.is_empty() {
        return Err(anyhow::anyhow!("AI Provider 名称不能为空").into());
    }

    if *CONFIG.llm.active_provider.lock() == name {
        return Err(anyhow::anyhow!("当前正在使用的 AI Provider 不允许移除").into());
    }

    let mut providers = CONFIG.llm.providers.lock();
    let provider = providers
        .get(&name)
        .ok_or_else(|| anyhow::anyhow!("AI Provider [{}] 不存在", name))?;
    if !provider.is_custom {
        return Err(anyhow::anyhow!("内置 AI Provider [{}] 不允许移除", name).into());
    }

    providers.remove(&name);
    log::info!("成功移除自定义 AI Provider: {}", name);
    Ok(())
}

/// 将当前大语言模型提供商切换为指定提供商。
#[tauri::command]
#[specta::specta]
pub fn switch_provider(name: String) -> CommandsResult<()> {
    log::debug!("正在切换 AI Provider 到 [{}]", name);
    let providers = CONFIG.llm.providers.lock();
    if providers.contains_key(&name) {
        *CONFIG.llm.active_provider.lock() = name.clone();
        log::info!("成功切换 AI Provider 到 [{}]", name);
        Ok(())
    } else {
        Err(anyhow::anyhow!("找不到 ID 为 [{}] 的大模型提供商", name).into())
    }
}

/// 获取当前大语言模型提供商所支持的全部模型列表。
#[tauri::command]
#[specta::specta]
pub async fn models() -> Vec<String> {
    let active_id = CONFIG.llm.active_provider.lock().clone();

    if active_id == "ollama" {
        if let Err(error) = ollama::fetch_ollama_models().await {
            log::warn!("Ollama 模型刷新失败: {}", error);
        }
    }

    let providers = CONFIG.llm.providers.lock();
    providers
        .get(&active_id)
        .map(|provider| provider.models.clone())
        .unwrap_or_default()
}

/// 获取当前大语言模型提供商正在使用的具体模型名称。
#[tauri::command]
#[specta::specta]
pub fn current_model() -> String {
    let active_id = CONFIG.llm.active_provider.lock();
    let providers = CONFIG.llm.providers.lock();
    let model = providers
        .get(&*active_id)
        .and_then(|p| p.chosen_model.and_then(|idx| p.models.get(idx)))
        .map(|s| s.as_str())
        .unwrap_or_default()
        .to_string();
    log::debug!("正在获取当前模型: {}", model);
    model
}

/// 将当前大语言模型提供商的选用模型切换为指定模型。
#[tauri::command]
#[specta::specta]
pub fn switch_model(model: String) {
    log::debug!("正在切换模型到 [{}]...", model);
    let active_id = CONFIG.llm.active_provider.lock();
    let mut providers = CONFIG.llm.providers.lock();
    if let Some(p) = providers.get_mut(&*active_id) {
        if let Some(pos) = p.models.iter().position(|m| m == &model) {
            p.chosen_model = Some(pos);
        }
    }
    log::info!("成功切换模型到 [{}]", model);
}

#[tauri::command]
#[specta::specta]
pub fn api_key() -> String {
    log::debug!("正在获取当前 API Key...");
    let active_id = CONFIG.llm.active_provider.lock();
    let providers = CONFIG.llm.providers.lock();
    let key = providers
        .get(&*active_id)
        .and_then(|p| p.api_key.as_ref())
        .map(|key| key.expose().to_owned())
        .unwrap_or_default();
    if let Some(key) = providers.get(&*active_id).and_then(|p| p.api_key.as_ref()) {
        log::debug!("成功获取当前 API Key: {}", key);
    } else {
        log::debug!("当前 API Key 未设置");
    }
    key
}

/// 设置当前大语言模型提供商的 API 密钥。
#[tauri::command]
#[specta::specta]
pub fn set_key(key: String) -> CommandsResult<()> {
    log::debug!("正在设置 API 密钥...");
    let active_id = CONFIG.llm.active_provider.lock();
    let mut providers = CONFIG.llm.providers.lock();
    if let Some(p) = providers.get_mut(&*active_id) {
        p.api_key = if key.trim().is_empty() {
            None
        } else {
            Some(crate::config::llm::ApiKey::new(key)?)
        };
    }
    log::info!("成功设置 API 密钥");
    Ok(())
}
