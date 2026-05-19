//! # 代码生成模板
//!
//! 为各分类插件生成不同框架的集成代码片段。
//! 支持框架: Next.js, Nuxt.js, React, Vue, HTML/JS, Python, Node.js
//!
//! 品牌: 小土豆AI原生 (XiaoTuDou AI Native)
//! 开发者: 自由的风

use super::Plugin;

/// 根据插件信息和目标框架生成集成代码
///
/// # Arguments
/// * `plugin` - 插件元数据
/// * `framework` - 目标框架 (nextjs | nuxt | react | vue | html | python | nodejs)
pub fn generate(plugin: &Plugin, framework: &str) -> String {
    let header = format!(
        "// ============================================\n\
         // 🔌 {} 集成代码\n\
         // 分类: {}\n\
         // 文档: {}\n\
         // 生成: 小土豆AI原生 插件系统\n\
         // ============================================\n\n",
        plugin.name, plugin.category, plugin.doc_url
    );

    let body = match plugin.category.as_str() {
        "proxy" => gen_proxy(plugin, framework),
        "fingerprint_browser" => gen_fingerprint_browser(plugin, framework),
        "captcha" => gen_captcha(plugin, framework),
        "cloud_phone" => gen_cloud_phone(plugin, framework),
        "social_media" => gen_social_media(plugin, framework),
        "ecommerce" => gen_ecommerce(plugin, framework),
        "sms_email" => gen_sms_email(plugin, framework),
        "accounts" => gen_accounts(plugin, framework),
        _ => gen_generic(plugin, framework),
    };

    format!("{}{}", header, body)
}

// ─────────────────────────────────────────────────────
// 代理IP服务 (Proxy)
// ─────────────────────────────────────────────────────

fn gen_proxy(plugin: &Plugin, framework: &str) -> String {
    let name = &plugin.name;
    let id = &plugin.id;

    match framework {
        "nextjs" => format!(
r#"// Next.js API Route - {name} 代理中间件
// 文件: pages/api/proxy/[...path].ts

import type {{ NextApiRequest, NextApiResponse }} from 'next';
import httpProxy from 'http-proxy';

const proxy = httpProxy.createProxyServer({{
  target: process.env.PROXY_TARGET_URL,
  changeOrigin: true,
  headers: {{
    'Proxy-Authorization': `Basic ${{Buffer.from(
      `${{process.env.{id_env}_USERNAME}}:${{process.env.{id_env}_PASSWORD}}`
    ).toString('base64')}}`,
  }},
}});

export const config = {{ api: {{ bodyParser: false, externalResolver: true }} }};

export default function handler(req: NextApiRequest, res: NextApiResponse) {{
  proxy.web(req, res, {{}}, (err) => {{
    console.error('[{name}] Proxy error:', err);
    res.status(502).json({{ error: 'Proxy request failed' }});
  }});
}}

// .env.local
// {id_env}_USERNAME=your_username
// {id_env}_PASSWORD=your_password
// PROXY_TARGET_URL=https://api.example.com
"#, name = name, id_env = id.to_uppercase().replace('-', "_")),

        "nuxt" => format!(
r#"// Nuxt.js Server Middleware - {name} 代理配置
// 文件: server/middleware/proxy.ts

import {{ defineEventHandler, proxyRequest }} from 'h3';

export default defineEventHandler(async (event) => {{
  const proxyUrl = process.env.{id_env}_ENDPOINT || 'http://proxy.example.com';
  const username = process.env.{id_env}_USERNAME;
  const password = process.env.{id_env}_PASSWORD;

  if (event.path?.startsWith('/api/proxy')) {{
    return proxyRequest(event, proxyUrl, {{
      headers: {{
        'Proxy-Authorization': `Basic ${{btoa(`${{username}}:${{password}}`)}}`,
      }},
    }});
  }}
}});

// nuxt.config.ts
// export default defineNuxtConfig({{
//   runtimeConfig: {{
//     {id_lower}Username: '',
//     {id_lower}Password: '',
//     {id_lower}Endpoint: '',
//   }}
// }})
"#, name = name, id_env = id.to_uppercase().replace('-', "_"), id_lower = id.to_lowercase().replace('-', "_")),

        "python" => format!(
r#"# Python - {name} 代理集成
# pip install requests

import os
import requests

class {class_name}Proxy:
    """代理IP服务封装 - {name}"""

    def __init__(self):
        self.username = os.environ.get('{id_env}_USERNAME', '')
        self.password = os.environ.get('{id_env}_PASSWORD', '')
        self.endpoint = os.environ.get('{id_env}_ENDPOINT', '')

    @property
    def proxy_url(self) -> str:
        return f"http://{{self.username}}:{{self.password}}@{{self.endpoint}}"

    @property
    def proxies(self) -> dict:
        url = self.proxy_url
        return {{"http": url, "https": url}}

    def request(self, method: str, url: str, **kwargs) -> requests.Response:
        """通过代理发起HTTP请求"""
        kwargs.setdefault('proxies', self.proxies)
        kwargs.setdefault('timeout', 30)
        return requests.request(method, url, **kwargs)

    def get(self, url: str, **kwargs) -> requests.Response:
        return self.request('GET', url, **kwargs)

    def post(self, url: str, **kwargs) -> requests.Response:
        return self.request('POST', url, **kwargs)

    def check_ip(self) -> str:
        """检测当前代理IP"""
        resp = self.get('https://httpbin.org/ip')
        return resp.json().get('origin', 'unknown')


# 使用示例
if __name__ == '__main__':
    proxy = {class_name}Proxy()
    print(f"当前IP: {{proxy.check_ip()}}")
    resp = proxy.get('https://example.com')
    print(f"状态码: {{resp.status_code}}")
"#, name = name, class_name = to_class_name(id), id_env = id.to_uppercase().replace('-', "_")),

        "nodejs" => format!(
r#"// Node.js - {name} 代理集成
// npm install axios https-proxy-agent

const axios = require('axios');
const {{ HttpsProxyAgent }} = require('https-proxy-agent');

class {class_name}Proxy {{
  constructor() {{
    this.username = process.env.{id_env}_USERNAME || '';
    this.password = process.env.{id_env}_PASSWORD || '';
    this.endpoint = process.env.{id_env}_ENDPOINT || '';
    this.agent = new HttpsProxyAgent(
      `http://${{this.username}}:${{this.password}}@${{this.endpoint}}`
    );
    this.client = axios.create({{
      httpsAgent: this.agent,
      timeout: 30000,
    }});
  }}

  async get(url, config = {{}}) {{
    return this.client.get(url, config);
  }}

  async post(url, data, config = {{}}) {{
    return this.client.post(url, data, config);
  }}

  async checkIP() {{
    const resp = await this.get('https://httpbin.org/ip');
    return resp.data.origin;
  }}
}}

// 使用示例
(async () => {{
  const proxy = new {class_name}Proxy();
  console.log('当前IP:', await proxy.checkIP());
}})();
"#, name = name, class_name = to_class_name(id), id_env = id.to_uppercase().replace('-', "_")),

        "react" | "vue" | "html" | _ => format!(
r#"// {fw} - {name} 代理配置
// ⚠️ 代理服务应在后端使用，前端通过API路由调用

// 后端代理服务 (Node.js / Express)
const express = require('express');
const {{ createProxyMiddleware }} = require('http-proxy-middleware');

const app = express();

app.use('/api/proxy', createProxyMiddleware({{
  target: process.env.PROXY_TARGET_URL,
  changeOrigin: true,
  auth: `${{process.env.{id_env}_USERNAME}}:${{process.env.{id_env}_PASSWORD}}`,
  pathRewrite: {{ '^/api/proxy': '' }},
  onError: (err, req, res) => {{
    console.error('[{name}] Proxy error:', err.message);
    res.status(502).json({{ error: 'Proxy failed' }});
  }},
}}));

// 前端调用示例
// fetch('/api/proxy/target-endpoint')
//   .then(res => res.json())
//   .then(data => console.log(data));
"#, fw = framework, name = name, id_env = id.to_uppercase().replace('-', "_")),
    }
}

// ─────────────────────────────────────────────────────
// 指纹浏览器 (Fingerprint Browser)
// ─────────────────────────────────────────────────────

fn gen_fingerprint_browser(plugin: &Plugin, framework: &str) -> String {
    let name = &plugin.name;
    let id = &plugin.id;

    match framework {
        "python" => format!(
r#"# Python - {name} 指纹浏览器自动化
# pip install selenium requests

import os
import json
import requests
from selenium import webdriver
from selenium.webdriver.chrome.options import Options

class {class_name}Browser:
    """{name} 指纹浏览器自动化封装"""

    API_BASE = os.environ.get('{id_env}_API_URL', 'http://localhost:50325')
    API_KEY = os.environ.get('{id_env}_API_KEY', '')

    def create_profile(self, name: str, proxy: dict = None) -> str:
        """创建浏览器配置文件"""
        payload = {{
            'name': name,
            'os': 'win',
            'browser': 'chrome',
        }}
        if proxy:
            payload['proxy'] = proxy
        resp = requests.post(
            f'{{self.API_BASE}}/api/v1/browser/profiles',
            json=payload,
            headers={{'Authorization': f'Bearer {{self.API_KEY}}'}},
        )
        return resp.json().get('data', {{}}).get('id', '')

    def start_profile(self, profile_id: str) -> webdriver.Chrome:
        """启动指纹浏览器并返回 Selenium WebDriver"""
        resp = requests.get(
            f'{{self.API_BASE}}/api/v1/browser/start?profile_id={{profile_id}}',
            headers={{'Authorization': f'Bearer {{self.API_KEY}}'}},
        )
        data = resp.json().get('data', {{}})
        debugger_url = data.get('ws', {{}}).get('selenium', '')

        options = Options()
        options.debugger_address = debugger_url
        driver = webdriver.Chrome(options=options)
        return driver

    def stop_profile(self, profile_id: str):
        """停止浏览器配置文件"""
        requests.get(
            f'{{self.API_BASE}}/api/v1/browser/stop?profile_id={{profile_id}}',
            headers={{'Authorization': f'Bearer {{self.API_KEY}}'}},
        )


# 使用示例
if __name__ == '__main__':
    browser = {class_name}Browser()
    profile_id = browser.create_profile('测试账号1')
    driver = browser.start_profile(profile_id)
    driver.get('https://whoer.net')
    print(f"页面标题: {{driver.title}}")
    # driver.quit()
    # browser.stop_profile(profile_id)
"#, name = name, class_name = to_class_name(id), id_env = id.to_uppercase().replace('-', "_")),

        "nodejs" => format!(
r#"// Node.js - {name} 指纹浏览器自动化
// npm install puppeteer-core axios

const axios = require('axios');
const puppeteer = require('puppeteer-core');

class {class_name}Browser {{
  constructor() {{
    this.apiBase = process.env.{id_env}_API_URL || 'http://localhost:50325';
    this.apiKey = process.env.{id_env}_API_KEY || '';
  }}

  async createProfile(name, proxy = null) {{
    const payload = {{ name, os: 'win', browser: 'chrome' }};
    if (proxy) payload.proxy = proxy;
    const resp = await axios.post(
      `${{this.apiBase}}/api/v1/browser/profiles`,
      payload,
      {{ headers: {{ Authorization: `Bearer ${{this.apiKey}}` }} }}
    );
    return resp.data?.data?.id;
  }}

  async startProfile(profileId) {{
    const resp = await axios.get(
      `${{this.apiBase}}/api/v1/browser/start?profile_id=${{profileId}}`,
      {{ headers: {{ Authorization: `Bearer ${{this.apiKey}}` }} }}
    );
    const wsEndpoint = resp.data?.data?.ws?.puppeteer;
    return puppeteer.connect({{ browserWSEndpoint: wsEndpoint }});
  }}

  async stopProfile(profileId) {{
    await axios.get(
      `${{this.apiBase}}/api/v1/browser/stop?profile_id=${{profileId}}`,
      {{ headers: {{ Authorization: `Bearer ${{this.apiKey}}` }} }}
    );
  }}
}}

// 使用示例
(async () => {{
  const browser = new {class_name}Browser();
  const profileId = await browser.createProfile('测试账号1');
  const b = await browser.startProfile(profileId);
  const page = (await b.pages())[0];
  await page.goto('https://whoer.net');
  console.log('页面标题:', await page.title());
}})();
"#, name = name, class_name = to_class_name(id), id_env = id.to_uppercase().replace('-', "_")),

        _ => format!(
r#"// {fw} - {name} 指纹浏览器集成说明
//
// 指纹浏览器主要用于后端自动化场景:
// 1. 创建独立浏览器配置文件
// 2. 通过 Selenium/Puppeteer 连接
// 3. 每个配置文件拥有独立指纹
//
// 推荐: 使用 Node.js 或 Python 模板生成后端自动化代码
// 命令: plugin:use {id} nodejs
// 命令: plugin:use {id} python
//
// 前端可通过API调用后端服务来管理浏览器实例:
//
// POST /api/browser/create  - 创建配置文件
// GET  /api/browser/start   - 启动浏览器
// GET  /api/browser/stop    - 停止浏览器
// GET  /api/browser/list    - 列出配置文件
//
// 文档: {doc_url}
"#, fw = framework, name = name, id = id, doc_url = plugin.doc_url),
    }
}

// ─────────────────────────────────────────────────────
// 验证码服务 (CAPTCHA)
// ─────────────────────────────────────────────────────

fn gen_captcha(plugin: &Plugin, framework: &str) -> String {
    let name = &plugin.name;
    let id = &plugin.id;

    match framework {
        "nextjs" => format!(
r#"// Next.js API Route - {name} 验证码服务
// 文件: pages/api/captcha/solve.ts

import type {{ NextApiRequest, NextApiResponse }} from 'next';

const CAPTCHA_API_KEY = process.env.{id_env}_API_KEY || '';
const CAPTCHA_API_URL = process.env.{id_env}_API_URL || 'https://api.captcha-service.com';

interface CaptchaTask {{
  type: 'recaptchav2' | 'recaptchav3' | 'hcaptcha' | 'funcaptcha';
  siteKey: string;
  pageUrl: string;
}}

async function solveCaptcha(task: CaptchaTask): Promise<string> {{
  // 1. 创建任务
  const createResp = await fetch(`${{CAPTCHA_API_URL}}/createTask`, {{
    method: 'POST',
    headers: {{ 'Content-Type': 'application/json' }},
    body: JSON.stringify({{
      clientKey: CAPTCHA_API_KEY,
      task: {{
        type: `${{task.type}}TaskProxyless`,
        websiteURL: task.pageUrl,
        websiteKey: task.siteKey,
      }},
    }}),
  }});
  const {{ taskId }} = await createResp.json();

  // 2. 轮询结果
  for (let i = 0; i < 60; i++) {{
    await new Promise(r => setTimeout(r, 3000));
    const resultResp = await fetch(`${{CAPTCHA_API_URL}}/getTaskResult`, {{
      method: 'POST',
      headers: {{ 'Content-Type': 'application/json' }},
      body: JSON.stringify({{ clientKey: CAPTCHA_API_KEY, taskId }}),
    }});
    const result = await resultResp.json();
    if (result.status === 'ready') {{
      return result.solution.gRecaptchaResponse || result.solution.token;
    }}
  }}
  throw new Error('验证码解决超时');
}}

export default async function handler(req: NextApiRequest, res: NextApiResponse) {{
  try {{
    const token = await solveCaptcha(req.body);
    res.status(200).json({{ success: true, token }});
  }} catch (err: any) {{
    res.status(500).json({{ success: false, error: err.message }});
  }}
}}
"#, name = name, id_env = id.to_uppercase().replace('-', "_")),

        "python" => format!(
r#"# Python - {name} 验证码服务集成
# pip install requests

import os
import time
import requests

class {class_name}Solver:
    """{name} 验证码自动解决"""

    def __init__(self):
        self.api_key = os.environ.get('{id_env}_API_KEY', '')
        self.api_url = os.environ.get('{id_env}_API_URL', 'https://api.captcha-service.com')

    def solve_recaptcha_v2(self, site_key: str, page_url: str) -> str:
        """解决 reCAPTCHA v2"""
        return self._solve({{
            'type': 'RecaptchaV2TaskProxyless',
            'websiteURL': page_url,
            'websiteKey': site_key,
        }})

    def solve_hcaptcha(self, site_key: str, page_url: str) -> str:
        """解决 hCaptcha"""
        return self._solve({{
            'type': 'HCaptchaTaskProxyless',
            'websiteURL': page_url,
            'websiteKey': site_key,
        }})

    def _solve(self, task: dict) -> str:
        """提交并等待验证码解决"""
        # 创建任务
        resp = requests.post(f'{{self.api_url}}/createTask', json={{
            'clientKey': self.api_key,
            'task': task,
        }})
        task_id = resp.json().get('taskId')

        # 轮询结果 (最多3分钟)
        for _ in range(60):
            time.sleep(3)
            resp = requests.post(f'{{self.api_url}}/getTaskResult', json={{
                'clientKey': self.api_key,
                'taskId': task_id,
            }})
            result = resp.json()
            if result.get('status') == 'ready':
                solution = result.get('solution', {{}})
                return solution.get('gRecaptchaResponse') or solution.get('token', '')

        raise TimeoutError('验证码解决超时')


# 使用示例
if __name__ == '__main__':
    solver = {class_name}Solver()
    token = solver.solve_recaptcha_v2(
        site_key='6Le-wvkSAAAAAPBMRTvw0Q4Muexq9bi0DJwx_mJ-',
        page_url='https://example.com/login'
    )
    print(f"Token: {{token[:50]}}...")
"#, name = name, class_name = to_class_name(id), id_env = id.to_uppercase().replace('-', "_")),

        "nodejs" => format!(
r#"// Node.js - {name} 验证码服务集成
// npm install axios

const axios = require('axios');

class {class_name}Solver {{
  constructor() {{
    this.apiKey = process.env.{id_env}_API_KEY || '';
    this.apiUrl = process.env.{id_env}_API_URL || 'https://api.captcha-service.com';
  }}

  async solveRecaptchaV2(siteKey, pageUrl) {{
    return this._solve({{
      type: 'RecaptchaV2TaskProxyless',
      websiteURL: pageUrl,
      websiteKey: siteKey,
    }});
  }}

  async solveHCaptcha(siteKey, pageUrl) {{
    return this._solve({{
      type: 'HCaptchaTaskProxyless',
      websiteURL: pageUrl,
      websiteKey: siteKey,
    }});
  }}

  async _solve(task) {{
    const {{ data }} = await axios.post(`${{this.apiUrl}}/createTask`, {{
      clientKey: this.apiKey,
      task,
    }});

    for (let i = 0; i < 60; i++) {{
      await new Promise(r => setTimeout(r, 3000));
      const result = await axios.post(`${{this.apiUrl}}/getTaskResult`, {{
        clientKey: this.apiKey,
        taskId: data.taskId,
      }});
      if (result.data.status === 'ready') {{
        return result.data.solution.gRecaptchaResponse || result.data.solution.token;
      }}
    }}
    throw new Error('验证码解决超时');
  }}
}}

module.exports = {class_name}Solver;
"#, name = name, class_name = to_class_name(id), id_env = id.to_uppercase().replace('-', "_")),

        _ => format!(
r#"// {fw} - {name} 验证码服务集成
// ⚠️ 验证码解决应在后端处理，前端发起请求

// 前端调用示例:
async function solveCaptcha(siteKey, pageUrl) {{
  const resp = await fetch('/api/captcha/solve', {{
    method: 'POST',
    headers: {{ 'Content-Type': 'application/json' }},
    body: JSON.stringify({{ type: 'recaptchav2', siteKey, pageUrl }}),
  }});
  const {{ token }} = await resp.json();
  return token;
}}

// 推荐: 使用 plugin:use {id} nextjs 生成完整后端代码
// 文档: {doc_url}
"#, fw = framework, name = name, id = id, doc_url = plugin.doc_url),
    }
}

// ─────────────────────────────────────────────────────
// 云手机 (Cloud Phone)
// ─────────────────────────────────────────────────────

fn gen_cloud_phone(plugin: &Plugin, framework: &str) -> String {
    let name = &plugin.name;
    let id = &plugin.id;

    match framework {
        "python" => format!(
r#"# Python - {name} 云手机集成
# pip install requests

import os
import requests

class {class_name}CloudPhone:
    """{name} 云手机管理"""

    def __init__(self):
        self.api_key = os.environ.get('{id_env}_API_KEY', '')
        self.api_url = os.environ.get('{id_env}_API_URL', '')

    def list_devices(self) -> list:
        """列出所有云手机设备"""
        resp = requests.get(f'{{self.api_url}}/api/devices',
            headers={{'Authorization': f'Bearer {{self.api_key}}'}})
        return resp.json().get('data', [])

    def create_device(self, name: str, region: str = 'us') -> dict:
        """创建新云手机"""
        resp = requests.post(f'{{self.api_url}}/api/devices',
            json={{'name': name, 'region': region}},
            headers={{'Authorization': f'Bearer {{self.api_key}}'}})
        return resp.json().get('data', {{}})

    def execute_action(self, device_id: str, action: str, params: dict = None) -> dict:
        """在云手机上执行操作"""
        resp = requests.post(f'{{self.api_url}}/api/devices/{{device_id}}/action',
            json={{'action': action, 'params': params or {{}}}},
            headers={{'Authorization': f'Bearer {{self.api_key}}'}})
        return resp.json()


# 使用示例
if __name__ == '__main__':
    phone = {class_name}CloudPhone()
    devices = phone.list_devices()
    print(f"设备数: {{len(devices)}}")
"#, name = name, class_name = to_class_name(id), id_env = id.to_uppercase().replace('-', "_")),

        _ => gen_generic_with_note(plugin, framework, "云手机服务主要用于后端自动化，建议使用 Python 或 Node.js 模板"),
    }
}

// ─────────────────────────────────────────────────────
// 社媒营销 (Social Media)
// ─────────────────────────────────────────────────────

fn gen_social_media(plugin: &Plugin, framework: &str) -> String {
    let name = &plugin.name;
    let id = &plugin.id;

    match framework {
        "nextjs" => format!(
r#"// Next.js - {name} 社媒营销集成
// 文件: lib/{id_lower}.ts

interface SocialConfig {{
  apiKey: string;
  apiUrl: string;
}}

class {class_name}Client {{
  private config: SocialConfig;

  constructor() {{
    this.config = {{
      apiKey: process.env.{id_env}_API_KEY || '',
      apiUrl: process.env.{id_env}_API_URL || '',
    }};
  }}

  async sendMessage(params: {{
    platform: string;
    to: string;
    content: string;
  }}) {{
    const resp = await fetch(`${{this.config.apiUrl}}/api/message/send`, {{
      method: 'POST',
      headers: {{
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${{this.config.apiKey}}`,
      }},
      body: JSON.stringify(params),
    }});
    return resp.json();
  }}

  async getContacts(platform: string) {{
    const resp = await fetch(
      `${{this.config.apiUrl}}/api/contacts?platform=${{platform}}`,
      {{ headers: {{ Authorization: `Bearer ${{this.config.apiKey}}` }} }}
    );
    return resp.json();
  }}

  async schedulePost(params: {{
    platform: string;
    content: string;
    mediaUrls?: string[];
    scheduledAt: string;
  }}) {{
    const resp = await fetch(`${{this.config.apiUrl}}/api/post/schedule`, {{
      method: 'POST',
      headers: {{
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${{this.config.apiKey}}`,
      }},
      body: JSON.stringify(params),
    }});
    return resp.json();
  }}
}}

export const {id_lower}Client = new {class_name}Client();
"#, name = name, class_name = to_class_name(id), id_env = id.to_uppercase().replace('-', "_"),
    id_lower = id.to_lowercase().replace('-', "_")),

        "python" => format!(
r#"# Python - {name} 社媒营销集成
# pip install requests

import os
import requests

class {class_name}Client:
    """{name} 社交媒体营销API"""

    def __init__(self):
        self.api_key = os.environ.get('{id_env}_API_KEY', '')
        self.api_url = os.environ.get('{id_env}_API_URL', '')

    def _headers(self) -> dict:
        return {{'Authorization': f'Bearer {{self.api_key}}', 'Content-Type': 'application/json'}}

    def send_message(self, platform: str, to: str, content: str) -> dict:
        """发送消息"""
        resp = requests.post(f'{{self.api_url}}/api/message/send',
            json={{'platform': platform, 'to': to, 'content': content}},
            headers=self._headers())
        return resp.json()

    def get_contacts(self, platform: str) -> list:
        """获取联系人列表"""
        resp = requests.get(f'{{self.api_url}}/api/contacts',
            params={{'platform': platform}}, headers=self._headers())
        return resp.json().get('data', [])

    def schedule_post(self, platform: str, content: str, scheduled_at: str,
                      media_urls: list = None) -> dict:
        """定时发帖"""
        payload = {{'platform': platform, 'content': content, 'scheduledAt': scheduled_at}}
        if media_urls:
            payload['mediaUrls'] = media_urls
        resp = requests.post(f'{{self.api_url}}/api/post/schedule',
            json=payload, headers=self._headers())
        return resp.json()

    def get_analytics(self, platform: str, date_range: str = '7d') -> dict:
        """获取数据分析"""
        resp = requests.get(f'{{self.api_url}}/api/analytics',
            params={{'platform': platform, 'range': date_range}},
            headers=self._headers())
        return resp.json()


# 使用示例
if __name__ == '__main__':
    client = {class_name}Client()
    contacts = client.get_contacts('whatsapp')
    print(f"联系人数: {{len(contacts)}}")
"#, name = name, class_name = to_class_name(id), id_env = id.to_uppercase().replace('-', "_")),

        _ => gen_generic(plugin, framework),
    }
}

// ─────────────────────────────────────────────────────
// 电商工具 (E-Commerce)
// ─────────────────────────────────────────────────────

fn gen_ecommerce(plugin: &Plugin, framework: &str) -> String {
    gen_generic(plugin, framework)
}

// ─────────────────────────────────────────────────────
// 接码/邮箱 (SMS/Email)
// ─────────────────────────────────────────────────────

fn gen_sms_email(plugin: &Plugin, framework: &str) -> String {
    let name = &plugin.name;
    let id = &plugin.id;

    match framework {
        "python" => format!(
r#"# Python - {name} 接码/邮箱服务
# pip install requests

import os
import time
import requests

class {class_name}SMS:
    """{name} 短信验证码接收"""

    def __init__(self):
        self.api_key = os.environ.get('{id_env}_API_KEY', '')
        self.api_url = os.environ.get('{id_env}_API_URL', '')

    def get_number(self, country: str = 'US', service: str = 'google') -> dict:
        """获取临时手机号"""
        resp = requests.get(f'{{self.api_url}}/api/number/get',
            params={{'country': country, 'service': service}},
            headers={{'Authorization': f'Bearer {{self.api_key}}'}})
        return resp.json().get('data', {{}})

    def wait_for_code(self, order_id: str, timeout: int = 120) -> str:
        """等待并获取验证码"""
        start = time.time()
        while time.time() - start < timeout:
            resp = requests.get(f'{{self.api_url}}/api/sms/get',
                params={{'orderId': order_id}},
                headers={{'Authorization': f'Bearer {{self.api_key}}'}})
            data = resp.json().get('data', {{}})
            if data.get('code'):
                return data['code']
            time.sleep(5)
        raise TimeoutError('等待验证码超时')

    def release_number(self, order_id: str):
        """释放手机号"""
        requests.post(f'{{self.api_url}}/api/number/release',
            json={{'orderId': order_id}},
            headers={{'Authorization': f'Bearer {{self.api_key}}'}})


# 使用示例: 自动注册流程
if __name__ == '__main__':
    sms = {class_name}SMS()
    # 1. 获取手机号
    number = sms.get_number(country='US', service='google')
    print(f"手机号: {{number.get('phone')}}")
    # 2. 在目标网站输入手机号...
    # 3. 等待验证码
    code = sms.wait_for_code(number.get('orderId'))
    print(f"验证码: {{code}}")
    # 4. 释放
    sms.release_number(number.get('orderId'))
"#, name = name, class_name = to_class_name(id), id_env = id.to_uppercase().replace('-', "_")),

        "nodejs" => format!(
r#"// Node.js - {name} 接码服务
// npm install axios

const axios = require('axios');

class {class_name}SMS {{
  constructor() {{
    this.apiKey = process.env.{id_env}_API_KEY || '';
    this.apiUrl = process.env.{id_env}_API_URL || '';
  }}

  async getNumber(country = 'US', service = 'google') {{
    const {{ data }} = await axios.get(`${{this.apiUrl}}/api/number/get`, {{
      params: {{ country, service }},
      headers: {{ Authorization: `Bearer ${{this.apiKey}}` }},
    }});
    return data.data;
  }}

  async waitForCode(orderId, timeoutMs = 120000) {{
    const start = Date.now();
    while (Date.now() - start < timeoutMs) {{
      const {{ data }} = await axios.get(`${{this.apiUrl}}/api/sms/get`, {{
        params: {{ orderId }},
        headers: {{ Authorization: `Bearer ${{this.apiKey}}` }},
      }});
      if (data.data?.code) return data.data.code;
      await new Promise(r => setTimeout(r, 5000));
    }}
    throw new Error('等待验证码超时');
  }}

  async releaseNumber(orderId) {{
    await axios.post(`${{this.apiUrl}}/api/number/release`,
      {{ orderId }},
      {{ headers: {{ Authorization: `Bearer ${{this.apiKey}}` }} }}
    );
  }}
}}

module.exports = {class_name}SMS;
"#, name = name, class_name = to_class_name(id), id_env = id.to_uppercase().replace('-', "_")),

        _ => gen_generic(plugin, framework),
    }
}

// ─────────────────────────────────────────────────────
// 账号服务 (Accounts)
// ─────────────────────────────────────────────────────

fn gen_accounts(plugin: &Plugin, framework: &str) -> String {
    gen_generic(plugin, framework)
}

// ─────────────────────────────────────────────────────
// 通用模板 (Generic / Other)
// ─────────────────────────────────────────────────────

fn gen_generic(plugin: &Plugin, framework: &str) -> String {
    let name = &plugin.name;
    let id = &plugin.id;

    match framework {
        "python" => format!(
r#"# Python - {name} API 集成
# pip install requests

import os
import requests

class {class_name}Client:
    """{name} API 客户端"""

    def __init__(self):
        self.api_key = os.environ.get('{id_env}_API_KEY', '')
        self.api_url = os.environ.get('{id_env}_API_URL', '')

    def _headers(self) -> dict:
        return {{
            'Authorization': f'Bearer {{self.api_key}}',
            'Content-Type': 'application/json',
        }}

    def get(self, endpoint: str, params: dict = None) -> dict:
        resp = requests.get(f'{{self.api_url}}{{endpoint}}',
            params=params, headers=self._headers())
        resp.raise_for_status()
        return resp.json()

    def post(self, endpoint: str, data: dict = None) -> dict:
        resp = requests.post(f'{{self.api_url}}{{endpoint}}',
            json=data, headers=self._headers())
        resp.raise_for_status()
        return resp.json()


# 使用示例
if __name__ == '__main__':
    client = {class_name}Client()
    # 根据 {name} 的API文档调用具体接口
    # 文档: {doc_url}
"#, name = name, class_name = to_class_name(id), id_env = id.to_uppercase().replace('-', "_"),
    doc_url = plugin.doc_url),

        "nodejs" => format!(
r#"// Node.js - {name} API 集成
// npm install axios

const axios = require('axios');

class {class_name}Client {{
  constructor() {{
    this.apiKey = process.env.{id_env}_API_KEY || '';
    this.apiUrl = process.env.{id_env}_API_URL || '';
    this.client = axios.create({{
      baseURL: this.apiUrl,
      headers: {{ Authorization: `Bearer ${{this.apiKey}}` }},
    }});
  }}

  async get(endpoint, params = {{}}) {{
    const {{ data }} = await this.client.get(endpoint, {{ params }});
    return data;
  }}

  async post(endpoint, body = {{}}) {{
    const {{ data }} = await this.client.post(endpoint, body);
    return data;
  }}
}}

module.exports = {class_name}Client;

// 使用示例:
// const client = new {class_name}Client();
// 文档: {doc_url}
"#, name = name, class_name = to_class_name(id), id_env = id.to_uppercase().replace('-', "_"),
    doc_url = plugin.doc_url),

        "nextjs" => format!(
r#"// Next.js - {name} API 集成
// 文件: lib/{id_lower}.ts

class {class_name}Client {{
  private apiKey: string;
  private apiUrl: string;

  constructor() {{
    this.apiKey = process.env.{id_env}_API_KEY || '';
    this.apiUrl = process.env.{id_env}_API_URL || '';
  }}

  private async request(method: string, endpoint: string, body?: any) {{
    const resp = await fetch(`${{this.apiUrl}}${{endpoint}}`, {{
      method,
      headers: {{
        'Authorization': `Bearer ${{this.apiKey}}`,
        'Content-Type': 'application/json',
      }},
      ...(body ? {{ body: JSON.stringify(body) }} : {{}}),
    }});
    return resp.json();
  }}

  async get(endpoint: string) {{ return this.request('GET', endpoint); }}
  async post(endpoint: string, data: any) {{ return this.request('POST', endpoint, data); }}
}}

export const {id_lower}Client = new {class_name}Client();

// 文档: {doc_url}
"#, name = name, class_name = to_class_name(id), id_env = id.to_uppercase().replace('-', "_"),
    id_lower = id.to_lowercase().replace('-', "_"), doc_url = plugin.doc_url),

        "nuxt" => format!(
r#"// Nuxt.js - {name} API 集成
// 文件: server/utils/{id_lower}.ts

class {class_name}Client {{
  private apiKey: string;
  private apiUrl: string;

  constructor() {{
    const config = useRuntimeConfig();
    this.apiKey = config.{id_lower}ApiKey || '';
    this.apiUrl = config.{id_lower}ApiUrl || '';
  }}

  async get(endpoint: string) {{
    return $fetch(`${{this.apiUrl}}${{endpoint}}`, {{
      headers: {{ Authorization: `Bearer ${{this.apiKey}}` }},
    }});
  }}

  async post(endpoint: string, body: any) {{
    return $fetch(`${{this.apiUrl}}${{endpoint}}`, {{
      method: 'POST',
      headers: {{ Authorization: `Bearer ${{this.apiKey}}` }},
      body,
    }});
  }}
}}

export const {id_lower}Client = new {class_name}Client();
// 文档: {doc_url}
"#, name = name, class_name = to_class_name(id), id_lower = id.to_lowercase().replace('-', "_"),
    doc_url = plugin.doc_url),

        "react" => format!(
r#"// React - {name} API Hook
// 文件: hooks/use{class_name}.ts

import {{ useState, useCallback }} from 'react';

interface Use{class_name}Options {{
  baseUrl?: string;
}}

export function use{class_name}(options: Use{class_name}Options = {{}}) {{
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const apiCall = useCallback(async (endpoint: string, method = 'GET', body?: any) => {{
    setLoading(true);
    setError(null);
    try {{
      const resp = await fetch(`/api/{id_lower}${{endpoint}}`, {{
        method,
        headers: {{ 'Content-Type': 'application/json' }},
        ...(body ? {{ body: JSON.stringify(body) }} : {{}}),
      }});
      if (!resp.ok) throw new Error(`API error: ${{resp.status}}`);
      return await resp.json();
    }} catch (err: any) {{
      setError(err.message);
      throw err;
    }} finally {{
      setLoading(false);
    }}
  }}, []);

  return {{ apiCall, loading, error }};
}}

// 使用:
// const {{ apiCall, loading }} = use{class_name}();
// const data = await apiCall('/endpoint');
// 文档: {doc_url}
"#, class_name = to_class_name(id), id_lower = id.to_lowercase().replace('-', "_"),
    doc_url = plugin.doc_url),

        "vue" => format!(
r#"<!-- Vue - {name} Composable -->
<!-- 文件: composables/use{class_name}.ts -->

<script setup lang="ts">
import {{ ref }} from 'vue';

export function use{class_name}() {{
  const loading = ref(false);
  const error = ref<string | null>(null);

  async function apiCall(endpoint: string, method = 'GET', body?: any) {{
    loading.value = true;
    error.value = null;
    try {{
      const resp = await fetch(`/api/{id_lower}${{endpoint}}`, {{
        method,
        headers: {{ 'Content-Type': 'application/json' }},
        ...(body ? {{ body: JSON.stringify(body) }} : {{}}),
      }});
      if (!resp.ok) throw new Error(`API error: ${{resp.status}}`);
      return await resp.json();
    }} catch (err: any) {{
      error.value = err.message;
      throw err;
    }} finally {{
      loading.value = false;
    }}
  }}

  return {{ apiCall, loading, error }};
}}
</script>

<!-- 使用:
const {{ apiCall, loading }} = use{class_name}();
const data = await apiCall('/endpoint');
文档: {doc_url}
-->
"#, name = name, class_name = to_class_name(id), id_lower = id.to_lowercase().replace('-', "_"),
    doc_url = plugin.doc_url),

        "html" | _ => format!(
r#"<!-- HTML/JS - {name} 集成 -->
<script>
class {class_name}Client {{
  constructor(baseUrl = '/api/{id_lower}') {{
    this.baseUrl = baseUrl;
  }}

  async get(endpoint) {{
    const resp = await fetch(`${{this.baseUrl}}${{endpoint}}`);
    if (!resp.ok) throw new Error(`API error: ${{resp.status}}`);
    return resp.json();
  }}

  async post(endpoint, data) {{
    const resp = await fetch(`${{this.baseUrl}}${{endpoint}}`, {{
      method: 'POST',
      headers: {{ 'Content-Type': 'application/json' }},
      body: JSON.stringify(data),
    }});
    if (!resp.ok) throw new Error(`API error: ${{resp.status}}`);
    return resp.json();
  }}
}}

// 使用:
// const client = new {class_name}Client();
// const data = await client.get('/endpoint');
// 文档: {doc_url}
</script>
"#, name = name, class_name = to_class_name(id), id_lower = id.to_lowercase().replace('-', "_"),
    doc_url = plugin.doc_url),
    }
}

/// 生成带注释的通用模板
fn gen_generic_with_note(plugin: &Plugin, framework: &str, note: &str) -> String {
    format!("// 💡 {}\n\n{}", note, gen_generic(plugin, framework))
}

/// 将 slug 转换为类名 (PascalCase)
fn to_class_name(slug: &str) -> String {
    slug.split(|c: char| c == '-' || c == '_' || c == '.')
        .filter(|s| !s.is_empty())
        .map(|s| {
            let mut chars = s.chars();
            match chars.next() {
                Some(c) => {
                    let first: String = c.to_uppercase().collect();
                    first + &chars.as_str().to_string()
                }
                None => String::new(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_class_name() {
        assert_eq!(to_class_name("bright-data"), "BrightData");
        assert_eq!(to_class_name("CAPTCHAs-IO"), "CAPTCHAsIO");
        assert_eq!(to_class_name("360Proxy"), "360Proxy");
    }
}
