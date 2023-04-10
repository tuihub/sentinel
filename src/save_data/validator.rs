use jsonschema::JSONSchema;
use once_cell::sync::OnceCell;
use serde_json::Value;

use crate::{err_msg, save_data::config::Config, Result};

const SCHEMA_STR: &str = include_str!("./schema.json");
static SCHEMA: OnceCell<Value> = OnceCell::new();

pub fn validate_config(config_str: &str) -> Result<Config> {
    let schema =
        SCHEMA.get_or_try_init(|| -> Result<Value> { Ok(serde_json::from_str(SCHEMA_STR)?) })?;
    let validator = JSONSchema::compile(schema)?;
    let instance: Value = serde_json::from_str(config_str)?;

    if let Err(errors) = validator.validate(&instance) {
        let err_msgs: Vec<String> = errors.into_iter().map(|e| e.to_string()).collect();
        return Err(err_msg!("json schema validate failed {:?}", err_msgs));
    }

    Ok(serde_json::from_str(config_str)?)
}
