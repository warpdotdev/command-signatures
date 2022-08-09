use std::collections::HashMap;
use warp_completion_metadata::Filters;

mod docker;

pub fn template_filters() -> HashMap<String, Filters> {
    let filters = [docker::filter()];

    HashMap::from_iter(filters.map(Into::into))
}
