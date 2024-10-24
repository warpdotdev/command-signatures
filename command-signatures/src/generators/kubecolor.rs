//! `kubecolor` is a wrapper around kubectl providing colorized output; thus its CommandSignature
//! and generators are mirrors of the kubectl signature and generators.
use warp_completion_metadata::CommandSignatureGenerators;

use super::kubectl::{
    CLUSTER_GENERATOR, CLUSTER_ROLE_GENERATOR, CONTEXT_GENERATOR, DEPLOYMENTS_GENERATOR,
    NAMESPACE_GENERATOR, NODE_GENERATOR, RESOURCE_GENERATOR, RESOURCE_TYPE_GENERATOR,
    ROLE_GENERATOR, RUNNING_PODS_GENERATOR, TYPE_OR_TYPE_SLASH_NAME,
};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("kubecolor")
        .add_generator("resource_type", RESOURCE_TYPE_GENERATOR.clone())
        .add_generator("running_pods", RUNNING_PODS_GENERATOR.clone())
        .add_generator("deployments", DEPLOYMENTS_GENERATOR.clone())
        .add_generator("node", NODE_GENERATOR.clone())
        .add_generator("cluster_role", CLUSTER_ROLE_GENERATOR.clone())
        .add_generator("role", ROLE_GENERATOR.clone())
        .add_generator("resource", RESOURCE_GENERATOR.clone())
        .add_generator("context", CONTEXT_GENERATOR.clone())
        .add_generator("cluster", CLUSTER_GENERATOR.clone())
        .add_generator("namespace", NAMESPACE_GENERATOR.clone())
        .add_generator("type_or_type_slash_name", TYPE_OR_TYPE_SLASH_NAME.clone())
}
