pub mod clear;
pub mod dark;

use std::{collections::HashMap};
use image::RgbaImage;

type FnGene = fn(&str, &str, i64, &RgbaImage) -> RgbaImage;
type FnFail = fn(&str) -> RgbaImage;
pub struct ThemeGetter {
    reg_list: HashMap<String, (FnGene, FnFail)>, 
    default_theme: (FnGene, FnFail)
}

impl ThemeGetter {
    pub fn new(default_theme: (FnGene, FnFail)) -> Self {
        Self {
            reg_list: HashMap::new(), default_theme: default_theme.clone()
        }
    }
    pub fn add(mut self, theme_name: &str, theme: (FnGene, FnFail)) -> Self {
        self.reg_list.insert(theme_name.to_string(), theme);
        self
    }
    fn get_theme(&self, theme_name: &str) -> &(FnGene, FnFail) {
        match self.reg_list.get(theme_name) {
            None => &self.default_theme,
            Some(theme) => theme
        }
    }
    pub fn get_gene(&self, theme_name: &str) -> FnGene {
        self.get_theme(theme_name).0
    }
    pub fn get_fail(&self, theme_name: &str) -> FnFail {
        self.get_theme(theme_name).1
    }
    pub fn get_default_gene(&self) -> FnGene {
        self.default_theme.0
    }
    pub fn get_default_fail(&self) -> FnFail {
        self.default_theme.1
    }
}