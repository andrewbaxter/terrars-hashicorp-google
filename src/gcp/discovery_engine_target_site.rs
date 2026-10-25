#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DiscoveryEngineTargetSiteData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::ResourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    provisioner: std::vec::Vec<terrars::Provisioner>,
    data_store_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact_match: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    provided_uri_pattern: terrars::PrimField<std::string::String>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DiscoveryEngineTargetSite_Timeouts>,
}

struct DiscoveryEngineTargetSite_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DiscoveryEngineTargetSiteData_>,
}

#[derive(Clone)]
pub struct DiscoveryEngineTargetSite(std::rc::Rc<DiscoveryEngineTargetSite_>);

impl DiscoveryEngineTargetSite {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
        self
    }

    pub fn set_create_before_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.create_before_destroy = v;
        self
    }

    pub fn set_prevent_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.prevent_destroy = v;
        self
    }

    pub fn ignore_changes_to_all(self) -> Self {
        self.0.data.borrow_mut().lifecycle.ignore_changes =
            std::option::Option::Some(terrars::IgnoreChanges::All(terrars::IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl std::string::ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                std::option::Option::Some(i) => match i {
                    terrars::IgnoreChanges::All(_) => {
                        true
                    },
                    terrars::IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                std::option::Option::None => true,
            } {
                d.lifecycle.ignore_changes =
                    std::option::Option::Some(terrars::IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl terrars::Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl std::string::ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = r" Add a provisioner (ex: `local-exec`, `remote-exec`, `file`) to this resource."]
    #[doc = r" Provisioners run in the order they're added."]
    pub fn add_provisioner(self, v: impl std::convert::Into<terrars::Provisioner>) -> Self {
        self.0.data.borrow_mut().provisioner.push(v.into());
        self
    }

    pub fn add_lifecycle_precondition(
        self,
        condition: impl std::convert::Into<terrars::PrimField<bool>>,
        error_message: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().lifecycle.precondition.push(terrars::LifecycleCondition {
            condition: condition.into(),
            error_message: error_message.into(),
        });
        self
    }

    pub fn add_lifecycle_postcondition(
        self,
        condition: impl std::convert::Into<terrars::PrimField<bool>>,
        error_message: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().lifecycle.postcondition.push(terrars::LifecycleCondition {
            condition: condition.into(),
            error_message: error_message.into(),
        });
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `exact_match`.\nIf set to false, a uri_pattern is generated to include all pages whose\naddress contains the provided_uri_pattern. If set to true, an uri_pattern\nis generated to try to be an exact match of the provided_uri_pattern or\njust the specific page if the provided_uri_pattern is a specific one.\nprovided_uri_pattern is always normalized to generate the URI pattern to\nbe used by the search engine."]
    pub fn set_exact_match(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().exact_match = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\nThe possible target site types. Possible values: [\"INCLUDE\", \"EXCLUDE\"]"]
    pub fn set_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DiscoveryEngineTargetSite_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `data_store_id` after provisioning.\nThe unique id of the data store."]
    pub fn data_store_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_store_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `exact_match` after provisioning.\nIf set to false, a uri_pattern is generated to include all pages whose\naddress contains the provided_uri_pattern. If set to true, an uri_pattern\nis generated to try to be an exact match of the provided_uri_pattern or\njust the specific page if the provided_uri_pattern is a specific one.\nprovided_uri_pattern is always normalized to generate the URI pattern to\nbe used by the search engine."]
    pub fn exact_match(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exact_match", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `failure_reason` after provisioning.\nSite search indexing failure reasons."]
    pub fn failure_reason(&self) -> terrars::ListRef<DiscoveryEngineTargetSite_FailureReasonRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.failure_reason", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `generated_uri_pattern` after provisioning.\nThis is system-generated based on the 'provided_uri_pattern'."]
    pub fn generated_uri_pattern(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.generated_uri_pattern", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `indexing_status` after provisioning.\nThe indexing status."]
    pub fn indexing_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.indexing_status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the data store should reside. The value can\nonly be one of \"global\", \"us\" and \"eu\"."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe unique full resource name of the target site. Values are of the format\n'projects/{project}/locations/{location}/collections/{collection_id}/dataStores/{data_store_id}/siteSearchEngine/targetSites/{target_site_id}'.\nThis field must be a UTF-8 encoded string with a length limit of 1024\ncharacters."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `provided_uri_pattern` after provisioning.\nThe user provided URI pattern from which the 'generated_uri_pattern' is\ngenerated."]
    pub fn provided_uri_pattern(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provided_uri_pattern", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `root_domain_uri` after provisioning.\nRoot domain of the 'provided_uri_pattern'."]
    pub fn root_domain_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.root_domain_uri", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `site_verification_info` after provisioning.\nSite ownership and validity verification status."]
    pub fn site_verification_info(&self) -> terrars::ListRef<DiscoveryEngineTargetSite_SiteVerificationInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.site_verification_info", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `target_site_id` after provisioning.\nThe unique id of the target site."]
    pub fn target_site_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_site_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe possible target site types. Possible values: [\"INCLUDE\", \"EXCLUDE\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe target site's last updated time."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DiscoveryEngineTargetSite_TimeoutsRef {
        DiscoveryEngineTargetSite_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for DiscoveryEngineTargetSite {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DiscoveryEngineTargetSite { }

impl terrars::ToListMappable for DiscoveryEngineTargetSite {
    type O = terrars::ListRef<DiscoveryEngineTargetSiteRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DiscoveryEngineTargetSite_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_discovery_engine_target_site".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDiscoveryEngineTargetSite {
    pub tf_id: std::string::String,
    #[doc = "The unique id of the data store."]
    pub data_store_id: terrars::PrimField<std::string::String>,
    #[doc =
        "The geographic location where the data store should reside. The value can\nonly be one of \"global\", \"us\" and \"eu\"."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "The user provided URI pattern from which the 'generated_uri_pattern' is\ngenerated."]
    pub provided_uri_pattern: terrars::PrimField<std::string::String>,
}

impl BuildDiscoveryEngineTargetSite {
    pub fn build(self, stack: &mut terrars::Stack) -> DiscoveryEngineTargetSite {
        let out = DiscoveryEngineTargetSite(std::rc::Rc::new(DiscoveryEngineTargetSite_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DiscoveryEngineTargetSiteData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                data_store_id: self.data_store_id,
                deletion_policy: core::default::Default::default(),
                exact_match: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                provided_uri_pattern: self.provided_uri_pattern,
                type_: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DiscoveryEngineTargetSiteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineTargetSiteRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DiscoveryEngineTargetSiteRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `data_store_id` after provisioning.\nThe unique id of the data store."]
    pub fn data_store_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_store_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `exact_match` after provisioning.\nIf set to false, a uri_pattern is generated to include all pages whose\naddress contains the provided_uri_pattern. If set to true, an uri_pattern\nis generated to try to be an exact match of the provided_uri_pattern or\njust the specific page if the provided_uri_pattern is a specific one.\nprovided_uri_pattern is always normalized to generate the URI pattern to\nbe used by the search engine."]
    pub fn exact_match(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exact_match", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `failure_reason` after provisioning.\nSite search indexing failure reasons."]
    pub fn failure_reason(&self) -> terrars::ListRef<DiscoveryEngineTargetSite_FailureReasonRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.failure_reason", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `generated_uri_pattern` after provisioning.\nThis is system-generated based on the 'provided_uri_pattern'."]
    pub fn generated_uri_pattern(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.generated_uri_pattern", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `indexing_status` after provisioning.\nThe indexing status."]
    pub fn indexing_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.indexing_status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the data store should reside. The value can\nonly be one of \"global\", \"us\" and \"eu\"."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe unique full resource name of the target site. Values are of the format\n'projects/{project}/locations/{location}/collections/{collection_id}/dataStores/{data_store_id}/siteSearchEngine/targetSites/{target_site_id}'.\nThis field must be a UTF-8 encoded string with a length limit of 1024\ncharacters."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `provided_uri_pattern` after provisioning.\nThe user provided URI pattern from which the 'generated_uri_pattern' is\ngenerated."]
    pub fn provided_uri_pattern(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provided_uri_pattern", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `root_domain_uri` after provisioning.\nRoot domain of the 'provided_uri_pattern'."]
    pub fn root_domain_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.root_domain_uri", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `site_verification_info` after provisioning.\nSite ownership and validity verification status."]
    pub fn site_verification_info(&self) -> terrars::ListRef<DiscoveryEngineTargetSite_SiteVerificationInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.site_verification_info", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `target_site_id` after provisioning.\nThe unique id of the target site."]
    pub fn target_site_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_site_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe possible target site types. Possible values: [\"INCLUDE\", \"EXCLUDE\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe target site's last updated time."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DiscoveryEngineTargetSite_TimeoutsRef {
        DiscoveryEngineTargetSite_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DiscoveryEngineTargetSite_FailureReason_QuotaFailure {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_required_quota: std::option::Option<terrars::PrimField<f64>>,
}

impl DiscoveryEngineTargetSite_FailureReason_QuotaFailure {
    #[doc = "Set the field `total_required_quota`.\n"]
    pub fn set_total_required_quota(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.total_required_quota = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DiscoveryEngineTargetSite_FailureReason_QuotaFailure {
    type O = terrars::BlockAssignable<DiscoveryEngineTargetSite_FailureReason_QuotaFailure>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineTargetSite_FailureReason_QuotaFailure {}

impl BuildDiscoveryEngineTargetSite_FailureReason_QuotaFailure {
    pub fn build(self) -> DiscoveryEngineTargetSite_FailureReason_QuotaFailure {
        DiscoveryEngineTargetSite_FailureReason_QuotaFailure {
            total_required_quota: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineTargetSite_FailureReason_QuotaFailureRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineTargetSite_FailureReason_QuotaFailureRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DiscoveryEngineTargetSite_FailureReason_QuotaFailureRef {
        DiscoveryEngineTargetSite_FailureReason_QuotaFailureRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineTargetSite_FailureReason_QuotaFailureRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `total_required_quota` after provisioning.\n"]
    pub fn total_required_quota(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_required_quota", self.base))
    }
}

#[derive(Serialize)]
pub struct DiscoveryEngineTargetSite_FailureReason {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    quota_failure: std::option::Option<terrars::ListField<DiscoveryEngineTargetSite_FailureReason_QuotaFailure>>,
}

impl DiscoveryEngineTargetSite_FailureReason {
    #[doc = "Set the field `quota_failure`.\n"]
    pub fn set_quota_failure(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DiscoveryEngineTargetSite_FailureReason_QuotaFailure>>,
    ) -> Self {
        self.quota_failure = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DiscoveryEngineTargetSite_FailureReason {
    type O = terrars::BlockAssignable<DiscoveryEngineTargetSite_FailureReason>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineTargetSite_FailureReason {}

impl BuildDiscoveryEngineTargetSite_FailureReason {
    pub fn build(self) -> DiscoveryEngineTargetSite_FailureReason {
        DiscoveryEngineTargetSite_FailureReason { quota_failure: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineTargetSite_FailureReasonRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineTargetSite_FailureReasonRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DiscoveryEngineTargetSite_FailureReasonRef {
        DiscoveryEngineTargetSite_FailureReasonRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineTargetSite_FailureReasonRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `quota_failure` after provisioning.\n"]
    pub fn quota_failure(&self) -> terrars::ListRef<DiscoveryEngineTargetSite_FailureReason_QuotaFailureRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.quota_failure", self.base))
    }
}

#[derive(Serialize)]
pub struct DiscoveryEngineTargetSite_SiteVerificationInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    site_verification_state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    verify_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DiscoveryEngineTargetSite_SiteVerificationInfo {
    #[doc = "Set the field `site_verification_state`.\n"]
    pub fn set_site_verification_state(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.site_verification_state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `verify_time`.\n"]
    pub fn set_verify_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.verify_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DiscoveryEngineTargetSite_SiteVerificationInfo {
    type O = terrars::BlockAssignable<DiscoveryEngineTargetSite_SiteVerificationInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineTargetSite_SiteVerificationInfo {}

impl BuildDiscoveryEngineTargetSite_SiteVerificationInfo {
    pub fn build(self) -> DiscoveryEngineTargetSite_SiteVerificationInfo {
        DiscoveryEngineTargetSite_SiteVerificationInfo {
            site_verification_state: core::default::Default::default(),
            verify_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineTargetSite_SiteVerificationInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineTargetSite_SiteVerificationInfoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DiscoveryEngineTargetSite_SiteVerificationInfoRef {
        DiscoveryEngineTargetSite_SiteVerificationInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineTargetSite_SiteVerificationInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `site_verification_state` after provisioning.\n"]
    pub fn site_verification_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.site_verification_state", self.base))
    }

    #[doc = "Get a reference to the value of field `verify_time` after provisioning.\n"]
    pub fn verify_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.verify_time", self.base))
    }
}

#[derive(Serialize)]
pub struct DiscoveryEngineTargetSite_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DiscoveryEngineTargetSite_Timeouts {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DiscoveryEngineTargetSite_Timeouts {
    type O = terrars::BlockAssignable<DiscoveryEngineTargetSite_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineTargetSite_Timeouts {}

impl BuildDiscoveryEngineTargetSite_Timeouts {
    pub fn build(self) -> DiscoveryEngineTargetSite_Timeouts {
        DiscoveryEngineTargetSite_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineTargetSite_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineTargetSite_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DiscoveryEngineTargetSite_TimeoutsRef {
        DiscoveryEngineTargetSite_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineTargetSite_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
}
