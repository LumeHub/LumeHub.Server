use include_dir::{Dir, include_dir};
use serde::Deserialize;

use domain::ParamDef;

static BUILTIN_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/config/effects");

#[derive(Debug, Clone)]
pub struct BuiltinEffect {
    pub slug: String,
    pub name: String,
    pub script: String,
    pub params: Vec<ParamDef>,
}

impl BuiltinEffect {
    pub fn id(&self) -> String {
        format!("builtin:{}", self.slug)
    }
}

#[derive(Deserialize)]
struct EffectFile {
    name: String,
    #[serde(default)]
    params: Vec<ParamDef>,
    script: ScriptSection,
}

#[derive(Deserialize)]
struct ScriptSection {
    code: String,
}

pub fn load_builtins() -> Vec<BuiltinEffect> {
    let mut effects = Vec::new();
    for file in BUILTIN_DIR.files() {
        let Some(name) = file.path().file_stem().and_then(|s| s.to_str()) else {
            continue;
        };
        let Some(ext) = file.path().extension().and_then(|s| s.to_str()) else {
            continue;
        };
        if ext != "toml" {
            continue;
        }
        let Some(contents) = file.contents_utf8() else {
            eprintln!(
                "warning: builtin effect '{}' is not valid UTF-8, skipping",
                name
            );
            continue;
        };
        match toml::from_str::<EffectFile>(contents) {
            Ok(ef) => effects.push(BuiltinEffect {
                slug: name.to_string(),
                name: ef.name,
                script: ef.script.code.trim().to_string(),
                params: ef.params,
            }),
            Err(e) => {
                eprintln!("warning: failed to parse builtin effect '{}': {}", name, e);
            }
        }
    }
    effects.sort_by(|a, b| a.name.cmp(&b.name));
    effects
}
