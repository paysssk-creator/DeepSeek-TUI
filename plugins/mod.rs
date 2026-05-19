//! # VMCardio 跨境工具插件系统
//!
//! 集成239个跨境工具合作伙伴，提供插件搜索、安装、卸载和代码生成功能。
//!
//! 开发者: 自由的风
//! 品牌: 小土豆AI原生 (XiaoTuDou AI Native)

pub mod code_templates;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// 插件分类及其中文名称
const CATEGORY_NAMES: &[(&str, &str)] = &[
    ("proxy", "代理IP服务"),
    ("fingerprint_browser", "指纹浏览器"),
    ("captcha", "验证码服务"),
    ("cloud_phone", "云手机"),
    ("social_media", "社媒营销"),
    ("ecommerce", "电商工具"),
    ("sms_email", "接码/邮箱"),
    ("accounts", "账号服务"),
    ("other", "其他工具"),
];

/// 单个插件的元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plugin {
    /// 插件唯一标识 (slug)
    pub id: String,
    /// 插件名称
    pub name: String,
    /// 分类: proxy | fingerprint_browser | captcha | cloud_phone | social_media | ecommerce | sms_email | accounts | other
    pub category: String,
    /// 插件功能描述
    pub description: String,
    /// 文档链接
    pub doc_url: String,
    /// API类型: rest | sdk | redirect
    pub api_type: String,
    /// 可用命令列表
    pub commands: Vec<String>,
    /// 是否已启用
    pub enabled: bool,
}

/// 插件注册表 - 管理所有VMCardio合作伙伴插件
#[derive(Debug)]
pub struct PluginRegistry {
    /// 所有已注册的插件 (id -> Plugin)
    plugins: HashMap<String, Plugin>,
    /// 注册表文件路径
    registry_path: String,
}

impl PluginRegistry {
    /// 从 registry.json 文件加载插件注册表
    ///
    /// # Arguments
    /// * `path` - registry.json 文件路径
    ///
    /// # Returns
    /// 加载完成的 PluginRegistry 实例
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let plugins_vec: Vec<Plugin> = serde_json::from_str(&content)?;

        let mut plugins = HashMap::new();
        for plugin in plugins_vec {
            plugins.insert(plugin.id.clone(), plugin);
        }

        Ok(Self {
            plugins,
            registry_path: path.to_string(),
        })
    }

    /// 保存当前插件状态到 registry.json
    fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let plugins_vec: Vec<&Plugin> = self.plugins.values().collect();
        let content = serde_json::to_string_pretty(&plugins_vec)?;
        fs::write(&self.registry_path, content)?;
        Ok(())
    }

    /// 列出插件，可按分类筛选
    ///
    /// # Arguments
    /// * `category` - 可选的分类过滤条件
    ///
    /// # Returns
    /// 符合条件的插件列表
    pub fn list_plugins(&self, category: Option<String>) -> Vec<&Plugin> {
        let mut result: Vec<&Plugin> = self
            .plugins
            .values()
            .filter(|p| {
                if let Some(ref cat) = category {
                    &p.category == cat
                } else {
                    true
                }
            })
            .collect();
        result.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        result
    }

    /// 安装（启用）插件
    ///
    /// # Arguments
    /// * `id` - 插件 slug 标识
    pub fn install_plugin(&mut self, id: &str) -> Result<String, String> {
        match self.plugins.get_mut(id) {
            Some(plugin) => {
                if plugin.enabled {
                    return Ok(format!("插件 '{}' 已经安装", plugin.name));
                }
                plugin.enabled = true;
                let name = plugin.name.clone();
                let doc_url = plugin.doc_url.clone();
                self.save().map_err(|e| e.to_string())?;
                Ok(format!(
                    "✅ 插件 '{}' 安装成功!\n📖 文档: {}\n💡 使用 `plugin:config {}` 进行配置\n🚀 使用 `plugin:use {}` 生成集成代码",
                    name, doc_url, id, id
                ))
            }
            None => Err(format!("❌ 未找到插件: '{}'", id)),
        }
    }

    /// 卸载（禁用）插件
    ///
    /// # Arguments
    /// * `id` - 插件 slug 标识
    pub fn uninstall_plugin(&mut self, id: &str) -> Result<String, String> {
        match self.plugins.get_mut(id) {
            Some(plugin) => {
                if !plugin.enabled {
                    return Ok(format!("插件 '{}' 尚未安装", plugin.name));
                }
                plugin.enabled = false;
                let name = plugin.name.clone();
                self.save().map_err(|e| e.to_string())?;
                Ok(format!("🗑️ 插件 '{}' 已卸载", name))
            }
            None => Err(format!("❌ 未找到插件: '{}'", id)),
        }
    }

    /// 模糊搜索插件 - 匹配名称、描述和分类
    ///
    /// # Arguments
    /// * `query` - 搜索关键词
    ///
    /// # Returns
    /// 匹配的插件列表，按相关度排序
    pub fn search_plugins(&self, query: &str) -> Vec<&Plugin> {
        let query_lower = query.to_lowercase();
        let query_terms: Vec<&str> = query_lower.split_whitespace().collect();

        let mut scored: Vec<(&Plugin, i32)> = self
            .plugins
            .values()
            .filter_map(|p| {
                let name_lower = p.name.to_lowercase();
                let desc_lower = p.description.to_lowercase();
                let cat_cn = Self::category_cn(&p.category);

                let mut score = 0i32;

                for term in &query_terms {
                    // 名称精确匹配得分最高
                    if name_lower == *term {
                        score += 100;
                    } else if name_lower.contains(term) {
                        score += 50;
                    }
                    // 描述匹配
                    if desc_lower.contains(term) {
                        score += 20;
                    }
                    // 分类匹配 (支持中英文)
                    if p.category.contains(term) || cat_cn.contains(term) {
                        score += 30;
                    }
                    // ID 匹配
                    if p.id.to_lowercase().contains(term) {
                        score += 40;
                    }
                }

                if score > 0 {
                    Some((p, score))
                } else {
                    None
                }
            })
            .collect();

        scored.sort_by(|a, b| b.1.cmp(&a.1));
        scored.into_iter().map(|(p, _)| p).collect()
    }

    /// 获取插件详细信息
    ///
    /// # Arguments
    /// * `id` - 插件 slug 标识
    pub fn get_plugin_info(&self, id: &str) -> Option<String> {
        self.plugins.get(id).map(|p| {
            let status = if p.enabled { "✅ 已安装" } else { "⬜ 未安装" };
            let cat_cn = Self::category_cn(&p.category);
            format!(
                r#"╔══════════════════════════════════════════╗
║  🔌 {}
╠══════════════════════════════════════════╣
║  分类:   {} ({})
║  状态:   {}
║  API:    {}
║  文档:   {}
╠══════════════════════════════════════════╣
║  {}
╠══════════════════════════════════════════╣
║  可用命令:
{}
╚══════════════════════════════════════════╝"#,
                p.name,
                cat_cn,
                p.category,
                status,
                p.api_type,
                p.doc_url,
                if p.description.is_empty() {
                    "暂无描述".to_string()
                } else {
                    p.description.clone()
                },
                p.commands
                    .iter()
                    .map(|c| format!("║    → {}", c))
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        })
    }

    /// 生成集成代码片段
    ///
    /// # Arguments
    /// * `id` - 插件 slug 标识
    /// * `framework` - 目标框架: nextjs | nuxt | react | vue | html | python | nodejs
    pub fn generate_integration_code(&self, id: &str, framework: &str) -> Result<String, String> {
        let plugin = self
            .plugins
            .get(id)
            .ok_or_else(|| format!("❌ 未找到插件: '{}'", id))?;

        Ok(code_templates::generate(plugin, framework))
    }

    /// 获取分类统计信息
    pub fn category_stats(&self) -> Vec<(String, String, usize)> {
        let mut stats: HashMap<String, usize> = HashMap::new();
        for p in self.plugins.values() {
            *stats.entry(p.category.clone()).or_insert(0) += 1;
        }

        CATEGORY_NAMES
            .iter()
            .map(|(key, cn)| {
                let count = stats.get(*key).copied().unwrap_or(0);
                (key.to_string(), cn.to_string(), count)
            })
            .filter(|(_, _, count)| *count > 0)
            .collect()
    }

    /// 获取分类的中文名称
    pub fn category_cn(category: &str) -> &'static str {
        CATEGORY_NAMES
            .iter()
            .find(|(k, _)| *k == category)
            .map(|(_, v)| *v)
            .unwrap_or("其他工具")
    }

    /// 获取已安装插件列表
    pub fn installed_plugins(&self) -> Vec<&Plugin> {
        let mut result: Vec<&Plugin> = self.plugins.values().filter(|p| p.enabled).collect();
        result.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        result
    }

    /// 获取插件总数
    pub fn total_count(&self) -> usize {
        self.plugins.len()
    }

    /// 打印插件系统概览
    pub fn print_overview(&self) -> String {
        let stats = self.category_stats();
        let installed = self.installed_plugins().len();
        let total = self.total_count();

        let mut output = format!(
            r#"
╔══════════════════════════════════════════════════╗
║  🔌 小土豆AI原生 - VMCardio 跨境工具插件系统     ║
║  开发者: 自由的风                                 ║
╠══════════════════════════════════════════════════╣
║  已安装: {}/{}                                    
╠══════════════════════════════════════════════════╣"#,
            installed, total
        );

        for (_, cn, count) in &stats {
            output.push_str(&format!("\n║  {} - {} 个插件", cn, count));
        }

        output.push_str(
            r#"
╠══════════════════════════════════════════════════╣
║  命令:                                            
║    plugin:list [category]  - 浏览插件              
║    plugin:search <query>   - 搜索插件              
║    plugin:install <id>     - 安装插件              
║    plugin:uninstall <id>   - 卸载插件              
║    plugin:info <id>        - 查看详情              
║    plugin:use <id> [框架]   - 生成集成代码          
╚══════════════════════════════════════════════════╝"#,
        );

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_category_cn() {
        assert_eq!(PluginRegistry::category_cn("proxy"), "代理IP服务");
        assert_eq!(PluginRegistry::category_cn("fingerprint_browser"), "指纹浏览器");
        assert_eq!(PluginRegistry::category_cn("unknown"), "其他工具");
    }
}
