use std::fs;
use fluent_bundle::{FluentBundle, FluentResource, FluentArgs};
use unic_langid::LanguageIdentifier;

pub struct I18n {
    bundle: FluentBundle<FluentResource>,
}

impl I18n {
    pub fn new(lang: &str) -> Self {
        let langid: LanguageIdentifier = lang.parse().unwrap();
        let source = fs::read_to_string(format!("locales/{}.ftl", lang)).expect("读取语言文件失败");
        let res = FluentResource::try_new(source).expect("Fluent资源解析失败");
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