use crate::config::ModuleConfig;
use crate::configs::salesforce::SalesforceConfig;
use crate::formatter::{StringFormatter, VersionFormatter};
use super::{Context, Module};

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("salesforce");
    let config = SalesforceConfig::try_load(module.config);
    // context.dir_contents().ok()?.has_file_name()

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
                    return Some(Ok("Adasd"));
                }
                _ => None,
            })
            .parse(None, Some(context))
    });
    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module 'salesforce':\n{}", error);
            return None
        }
    });


    Some(module)
}
