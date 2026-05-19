# 🔌 小土豆AI原生 - VMCardio 跨境工具插件系统

> **开发者:** 自由的风  
> **品牌:** 小土豆AI原生 (XiaoTuDou AI Native)  
> **合作伙伴:** VMCardio 跨境工具联盟  
> **插件总数:** 239 个

---

## 📋 目录

- [概述](#概述)
- [分类总览](#分类总览)
- [命令参考](#命令参考)
- [使用指南](#使用指南)
- [代码生成](#代码生成)
- [工作流示例](#工作流示例)

---

## 概述

VMCardio 跨境工具插件系统为 DeepSeek-TUI 提供了 **239 个跨境工具合作伙伴** 的一键集成能力。
通过简单的命令即可搜索、安装、配置并生成集成代码，大幅加速跨境业务开发。

### 核心功能

- 🔍 **插件搜索** - 模糊搜索，支持中英文关键词
- 📦 **一键安装** - 启用/禁用插件
- ⚙️ **配置管理** - 环境变量配置
- 🧑‍💻 **代码生成** - 自动生成 7 种框架的集成代码
- 📖 **文档链接** - 直达合作伙伴API文档

---

## 分类总览

| 分类标识 | 中文名称 | 插件数量 |
|---------|---------|---------|
| `proxy` | 代理IP服务 | 118 |
| `fingerprint_browser` | 指纹浏览器 | 20 |
| `social_media` | 社媒营销 | 26 |
| `other` | 其他工具 | 66 |
| `accounts` | 账号服务 | 4 |
| `captcha` | 验证码服务 | 3 |
| `cloud_phone` | 云手机 | 1 |
| `sms_email` | 接码/邮箱 | 1 |

### 各分类代表工具

- **代理IP服务**: 24在线服务。, 高达75%的批量购买折扣, 360Proxy 等
- **指纹浏览器**: Antidetect Browser, Antidetect Browser Hidemyacc, Adspower指纹浏览器 等
- **社媒营销**: TK跨境电商, AccountDiversity, Accounts Facebook for ADS and MarketPlace 等
- **验证码服务**: CAPTCHAs.IO, CaptchaAI, NextCaptcha 等
- **云手机**: DuoPlus云手机 等
- **接码/邮箱**: SMS-Activate 等
- **账号服务**: FIRE ACCS, GenZolo, adshine.pro 等
- **其他工具**: Flashid, Hidemium, Laurel Agency 等

---

## 命令参考

### 浏览插件

```bash
# 列出所有插件
plugin:list

# 按分类浏览
plugin:list proxy              # 代理IP服务
plugin:list fingerprint_browser # 指纹浏览器
plugin:list social_media       # 社媒营销
plugin:list captcha            # 验证码服务
```

### 搜索插件

```bash
# 英文搜索
plugin:search brightdata
plugin:search whatsapp

# 中文搜索
plugin:search 代理
plugin:search 指纹浏览器
```

### 安装/卸载

```bash
# 安装插件
plugin:install 360Proxy

# 卸载插件
plugin:uninstall 360Proxy
```

### 查看详情

```bash
plugin:info 360Proxy
```

输出示例:
```
╔══════════════════════════════════════════╗
║  🔌 360Proxy
╠══════════════════════════════════════════╣
║  分类:   代理IP服务 (proxy)
║  状态:   ✅ 已安装
║  API:    rest
║  文档:   https://partners.vmcardio.com/docs/360Proxy
╠══════════════════════════════════════════╣
║  360Proxy - 高品质住宅代理服务
╚══════════════════════════════════════════╝
```

---

## 代码生成

使用 `plugin:use` 命令自动生成集成代码，支持以下框架:

| 框架 | 命令参数 | 适用场景 |
|------|---------|---------|
| Next.js | `nextjs` | React SSR 全栈 |
| Nuxt.js | `nuxt` | Vue SSR 全栈 |
| React | `react` | React SPA |
| Vue | `vue` | Vue SPA |
| HTML/JS | `html` | 原生前端 |
| Python | `python` | 后端/爬虫/自动化 |
| Node.js | `nodejs` | 后端服务 |

### 使用方式

```bash
# 生成 Next.js 代理中间件
plugin:use 360Proxy nextjs

# 生成 Python 代理客户端
plugin:use BrightData python

# 生成 Node.js 指纹浏览器自动化
plugin:use Adspower-zhi-wen-liu-lan-qi nodejs

# 生成验证码解决后端
plugin:use CAPTCHAs-IO python
```

### 各分类生成的代码类型

| 分类 | 生成内容 |
|------|---------|
| 代理IP | 代理中间件、HTTP客户端、IP检测 |
| 指纹浏览器 | 浏览器自动化脚本、配置文件管理 |
| 验证码 | 验证码解决API、轮询等待逻辑 |
| 社媒营销 | 消息发送、联系人管理、定时发帖 |
| 接码/邮箱 | 号码获取、验证码等待、号码释放 |
| 云手机 | 设备管理、远程操作 |
| 通用 | REST API 客户端封装 |

---

## 工作流示例

### 场景1: 搭建跨境电商代理系统

```bash
# 1. 搜索代理服务
plugin:search residential proxy

# 2. 安装代理插件
plugin:install BrightData

# 3. 生成 Next.js 代理中间件
plugin:use BrightData nextjs

# 4. 配置环境变量
plugin:config BrightData
```

### 场景2: 社交媒体自动化

```bash
# 1. 安装指纹浏览器
plugin:install Adspower-zhi-wen-liu-lan-qi

# 2. 安装社媒工具
plugin:search whatsapp
plugin:install WAPlus

# 3. 生成自动化脚本
plugin:use Adspower-zhi-wen-liu-lan-qi python
plugin:use WAPlus python
```

### 场景3: 网站注册自动化

```bash
# 1. 安装验证码服务
plugin:install CaptchaAI

# 2. 安装接码服务
plugin:search sms
plugin:install SMS-Man

# 3. 生成 Node.js 注册流程
plugin:use CaptchaAI nodejs
plugin:use SMS-Man nodejs
```

---

## 文件结构

```
plugins/
├── registry.json        # 插件注册表 (239个插件)
├── mod.rs              # Rust 插件系统核心模块
├── code_templates.rs   # 代码生成模板
└── README.md           # 本文档
```

---

## 环境变量

所有插件使用统一的环境变量命名规则:

```
{PLUGIN_ID}_API_KEY=xxx
{PLUGIN_ID}_API_URL=xxx
{PLUGIN_ID}_USERNAME=xxx
{PLUGIN_ID}_PASSWORD=xxx
{PLUGIN_ID}_ENDPOINT=xxx
```

其中 `PLUGIN_ID` 为插件 slug 的大写形式，连字符替换为下划线。

---

> 🥔 小土豆AI原生 | 自由的风 出品 | Powered by VMCardio
