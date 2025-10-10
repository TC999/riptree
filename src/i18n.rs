use std::fs;
use fluent_bundle::{FluentBundle, FluentResource, FluentArgs};
use unic_langid::LanguageIdentifier;

pub struct I18n {
    bundle: FluentBundle<FluentResource>,
}

impl I18n {
    pub fn new(lang: &str) -> Self {
        let langid: LanguageIdentifier = lang.parse().unwrap();
        let path = format!("locales/{}.ftl", lang);
        let source = fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("语言文件未找到: {}", path));
        let res = FluentResource::try_new(source).unwrap_or_else(|_| panic!("Fluent资源解析失败: {}", path));
        let mut bundle = FluentBundle::new(vec![langid]);
        bundle.add_resource(res).expect("添加Fluent资源失败");
        Self { bundle }
    }

    pub fn text(&self, key: &str, args: Option<&FluentArgs>) -> String {
        let msg = self.bundle.get_message(key).expect("未找到消息");
        let pattern = msg.value().expect("未找到内容");
        self.bundle.format_pattern(pattern, args, &mut vec![]).to_string()
    }
}

use std::env;

pub fn detect_lang() -> String {
    // 优先读取 LC_ALL，其次 LANG，最后默认为 en-US
    env::var("LC_ALL")
        .or_else(|_| env::var("LANG"))
        .unwrap_or_else(|_| "en-US".to_string())
        .split('.')
        .next()  // 移除编码部分
        .unwrap_or("en-US")
        .replace('_', "-") // 变成 zh-CN 这种格式
}