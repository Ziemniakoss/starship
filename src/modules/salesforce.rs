use crate::config::ModuleConfig;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::configs::salesforce::SalesforceConfig;
use crate::formatter::{StringFormatter};
use super::{Context, Module};


#[derive(Clone, Deserialize, Serialize)]
#[serde(default)]
struct SfdxOrgCommandOutputResult {
    pub alias: Option<String>,
    pub username: Option<String>,
}

impl Default for SfdxOrgCommandOutputResult {
    fn default() -> Self {
        SfdxOrgCommandOutputResult { alias: None, username: None }
    }
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(default)]
struct SfdxOrgCommandOutput {
    pub status: i64,
    pub result: Option<SfdxOrgCommandOutputResult>,
}

impl Default for SfdxOrgCommandOutput {
    fn default() -> Self {
        SfdxOrgCommandOutput { status: 1, result: None }
    }
}

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("salesforce");
    let config = SalesforceConfig::try_load(module.config);
    let output = context.exec_cmd("sfdx", &["force:org:display", "--json"])?.stdout;
    let parsed: Value = serde_json::from_str(output.as_str()).ok()?;


    let org_name = match parsed.get("result") {
        Some(result) => {
            match  result.get("alias"){
                Some(alias) => alias.as_str().unwrap(),
                None => {
                    match result.get("username") {
                        Some(username) => username.as_str().unwrap(),
                        None => return None
                    }
                }
            }
        },
        None => return None
    };

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|var, _| match var {
                "symbol" => Some(config.symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "org_name" => {
                    return Some(Ok(org_name.clone()));
                }
                _ => None,
            })
            .parse(None, Some(context))
    });
    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module 'salesforce':\n{}", error);
            return None;
        }
    });

    Some(module)
}
