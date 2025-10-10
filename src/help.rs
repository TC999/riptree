use crate::i18n::I18n;
use fluent::FluentArgs;

pub fn print_help(i18n: &I18n) {
    let mut args = FluentArgs::new();
    println!("{}", i18n.text("help-usage", Some(&args)));
    println!("------- {} -------", i18n.text("help-listing-options", None));
    println!("{}", i18n.text("help-all-files", Some(&args)));
    println!("{}", i18n.text("help-list-dirs-only", Some(&args)));
    //println!("{}", i18n.text("help-follow-symlinks", Some(&args)));
    println!("{}", i18n.text("help-filter-gitignore", Some(&args)));
    println!("------- {} -------", i18n.text("help-misc-options", Some(&args)));
    println!("{}", i18n.text("help-print-version", Some(&args)));
    println!("{}", i18n.text("help-print-help", Some(&args)));
}