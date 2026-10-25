#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ClouddeployDeployPolicyData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    annotations: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    suspended: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rules: std::option::Option<std::vec::Vec<ClouddeployDeployPolicy_Rules>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    selectors: std::option::Option<std::vec::Vec<ClouddeployDeployPolicy_Selectors>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ClouddeployDeployPolicy_Timeouts>,
    dynamic: ClouddeployDeployPolicyDynamic,
}

struct ClouddeployDeployPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ClouddeployDeployPolicyData_>,
}

#[derive(Clone)]
pub struct ClouddeployDeployPolicy(std::rc::Rc<ClouddeployDeployPolicy_>);

impl ClouddeployDeployPolicy {
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
        "Set the field `annotations`.\nUser annotations. These attributes can only be set and used by the user, and not by Cloud Deploy. Annotations must meet the following constraints: * Annotations are key/value pairs. * Valid annotation keys have two segments: an optional prefix and name, separated by a slash ('/'). * The name segment is required and must be 63 characters or less, beginning and ending with an alphanumeric character ('[a-z0-9A-Z]') with dashes ('-'), underscores ('_'), dots ('.'), and alphanumerics between. * The prefix is optional. If specified, the prefix must be a DNS subdomain: a series of DNS labels separated by dots('.'), not longer than 253 characters in total, followed by a slash ('/'). See https://kubernetes.io/docs/concepts/overview/working-with-objects/annotations/#syntax-and-character-set for more details.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn set_annotations(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().annotations = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nDescription of the 'DeployPolicy'. Max length is 255 characters."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nLabels are attributes that can be set and used by both the user and by Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 63 characters.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `suspended`.\nWhen suspended, the policy will not prevent actions from occurring, even if the action violates the policy."]
    pub fn set_suspended(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().suspended = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rules`.\n"]
    pub fn set_rules(self, v: impl std::convert::Into<terrars::BlockAssignable<ClouddeployDeployPolicy_Rules>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rules = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rules = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `selectors`.\n"]
    pub fn set_selectors(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ClouddeployDeployPolicy_Selectors>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().selectors = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.selectors = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ClouddeployDeployPolicy_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nUser annotations. These attributes can only be set and used by the user, and not by Cloud Deploy. Annotations must meet the following constraints: * Annotations are key/value pairs. * Valid annotation keys have two segments: an optional prefix and name, separated by a slash ('/'). * The name segment is required and must be 63 characters or less, beginning and ending with an alphanumeric character ('[a-z0-9A-Z]') with dashes ('-'), underscores ('_'), dots ('.'), and alphanumerics between. * The prefix is optional. If specified, the prefix must be a DNS subdomain: a series of DNS labels separated by dots('.'), not longer than 253 characters in total, followed by a slash ('/'). See https://kubernetes.io/docs/concepts/overview/working-with-objects/annotations/#syntax-and-character-set for more details.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Time at which the DeployPolicy was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nDescription of the 'DeployPolicy'. Max length is 255 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nThe weak etag of the 'DeployPolicy' resource. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels are attributes that can be set and used by both the user and by Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 63 characters.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the 'DeployPolicy'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `suspended` after provisioning.\nWhen suspended, the policy will not prevent actions from occurring, even if the action violates the policy."]
    pub fn suspended(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.suspended", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nOutput only. Unique identifier of the 'DeployPolicy'."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Time at which the DeployPolicy was updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> terrars::ListRef<ClouddeployDeployPolicy_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `selectors` after provisioning.\n"]
    pub fn selectors(&self) -> terrars::ListRef<ClouddeployDeployPolicy_SelectorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.selectors", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ClouddeployDeployPolicy_TimeoutsRef {
        ClouddeployDeployPolicy_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ClouddeployDeployPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ClouddeployDeployPolicy { }

impl terrars::ToListMappable for ClouddeployDeployPolicy {
    type O = terrars::ListRef<ClouddeployDeployPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ClouddeployDeployPolicy_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_clouddeploy_deploy_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildClouddeployDeployPolicy {
    pub tf_id: std::string::String,
    #[doc = "The location for the resource"]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "Name of the 'DeployPolicy'."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildClouddeployDeployPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> ClouddeployDeployPolicy {
        let out = ClouddeployDeployPolicy(std::rc::Rc::new(ClouddeployDeployPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ClouddeployDeployPolicyData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                annotations: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                suspended: core::default::Default::default(),
                rules: core::default::Default::default(),
                selectors: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ClouddeployDeployPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployDeployPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ClouddeployDeployPolicyRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nUser annotations. These attributes can only be set and used by the user, and not by Cloud Deploy. Annotations must meet the following constraints: * Annotations are key/value pairs. * Valid annotation keys have two segments: an optional prefix and name, separated by a slash ('/'). * The name segment is required and must be 63 characters or less, beginning and ending with an alphanumeric character ('[a-z0-9A-Z]') with dashes ('-'), underscores ('_'), dots ('.'), and alphanumerics between. * The prefix is optional. If specified, the prefix must be a DNS subdomain: a series of DNS labels separated by dots('.'), not longer than 253 characters in total, followed by a slash ('/'). See https://kubernetes.io/docs/concepts/overview/working-with-objects/annotations/#syntax-and-character-set for more details.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Time at which the DeployPolicy was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nDescription of the 'DeployPolicy'. Max length is 255 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nThe weak etag of the 'DeployPolicy' resource. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels are attributes that can be set and used by both the user and by Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 63 characters.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the 'DeployPolicy'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `suspended` after provisioning.\nWhen suspended, the policy will not prevent actions from occurring, even if the action violates the policy."]
    pub fn suspended(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.suspended", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nOutput only. Unique identifier of the 'DeployPolicy'."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Time at which the DeployPolicy was updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> terrars::ListRef<ClouddeployDeployPolicy_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `selectors` after provisioning.\n"]
    pub fn selectors(&self) -> terrars::ListRef<ClouddeployDeployPolicy_SelectorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.selectors", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ClouddeployDeployPolicy_TimeoutsRef {
        ClouddeployDeployPolicy_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ClouddeployDeployPolicyDynamic {
    rules: std::option::Option<terrars::DynamicBlock<ClouddeployDeployPolicy_Rules>>,
    selectors: std::option::Option<terrars::DynamicBlock<ClouddeployDeployPolicy_Selectors>>,
}

#[derive(Serialize)]
pub struct ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_OneTimeWindows_EndDate {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    day: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    month: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    year: std::option::Option<terrars::PrimField<f64>>,
}

impl ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_OneTimeWindows_EndDate {
    #[doc = "Set the field `day`.\nDay of a month. Must be from 1 to 31 and valid for the year and month."]
    pub fn set_day(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.day = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `month`.\nMonth of a year. Must be from 1 to 12."]
    pub fn set_month(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.month = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `year`.\nYear of the date. Must be from 1 to 9999."]
    pub fn set_year(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.year = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_OneTimeWindows_EndDate {
    type O =
        terrars
        ::BlockAssignable<ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_OneTimeWindows_EndDate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_OneTimeWindows_EndDate {}

impl BuildClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_OneTimeWindows_EndDate {
    pub fn build(self) -> ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_OneTimeWindows_EndDate {
        ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_OneTimeWindows_EndDate {
            day: core::default::Default::default(),
            month: core::default::Default::default(),
            year: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_OneTimeWindows_EndDateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_OneTimeWindows_EndDateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_OneTimeWindows_EndDateRef {
        ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_OneTimeWindows_EndDateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_OneTimeWindows_EndDateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `day` after provisioning.\nDay of a month. Must be from 1 to 31 and valid for the year and month."]
    pub fn day(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }

    #[doc = "Get a reference to the value of field `month` after provisioning.\nMonth of a year. Must be from 1 to 12."]
    pub fn month(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.month", self.base))
    }

    #[doc =
        "Get a reference to the value of field `year` after provisioning.\nYear of the date. Must be from 1 to 9999."]
    pub fn year(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.year", self.base))
    }
}

#[derive(Serialize)]
pub struct ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindows_EndTime {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hours: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minutes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindows_EndTime {
    #[doc =
        "Set the field `hours`.\nHours of a day in 24 hour format. Must be greater than or equal to 0 and typically must be less than or equal to 23. An API may choose to allow the value \"24:00:00\" for scenarios like business closing time."]
    pub fn set_hours(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.hours = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `minutes`.\nMinutes of an hour. Must be greater than or equal to 0 and less than or equal to 59."]
    pub fn set_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minutes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `nanos`.\nFractions of seconds, in nanoseconds. Must be greater than or equal to 0 and less than or equal to 999,999,999."]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `seconds`.\nSeconds of a minute. Must be greater than or equal to 0 and typically must be less than or equal to 59. An API may allow the value 60 if it allows leap-seconds."]
    pub fn set_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindows_EndTime {
    type O =
        terrars::BlockAssignable<ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindows_EndTime>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindows_EndTime {}

impl BuildClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindows_EndTime {
    pub fn build(self) -> ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindows_EndTime {
        ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindows_EndTime {
            hours: core::default::Default::default(),
            minutes: core::default::Default::default(),
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindows_EndTimeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindows_EndTimeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindows_EndTimeRef {
        ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindows_EndTimeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindows_EndTimeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `hours` after provisioning.\nHours of a day in 24 hour format. Must be greater than or equal to 0 and typically must be less than or equal to 23. An API may choose to allow the value \"24:00:00\" for scenarios like business closing time."]
    pub fn hours(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hours", self.base))
    }

    #[doc =
        "Get a reference to the value of field `minutes` after provisioning.\nMinutes of an hour. Must be greater than or equal to 0 and less than or equal to 59."]
    pub fn minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `nanos` after provisioning.\nFractions of seconds, in nanoseconds. Must be greater than or equal to 0 and less than or equal to 999,999,999."]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc =
        "Get a reference to the value of field `seconds` after provisioning.\nSeconds of a minute. Must be greater than or equal to 0 and typically must be less than or equal to 59. An API may allow the value 60 if it allows leap-seconds."]
    pub fn seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_OneTimeWindows_StartDate {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    day: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    month: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    year: std::option::Option<terrars::PrimField<f64>>,
}

impl ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_OneTimeWindows_StartDate {
    #[doc =
        "Set the field `day`.\nDay of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant."]
    pub fn set_day(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.day = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `month`.\nMonth of a year. Must be from 1 to 12, or 0 to specify a year without a month and day."]
    pub fn set_month(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.month = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `year`.\nYear of the date. Must be from 1 to 9999, or 0 to specify a date without a year."]
    pub fn set_year(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.year = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_OneTimeWindows_StartDate {
    type O =
        terrars
        ::BlockAssignable<ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_OneTimeWindows_StartDate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_OneTimeWindows_StartDate {}

impl BuildClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_OneTimeWindows_StartDate {
    pub fn build(self) -> ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_OneTimeWindows_StartDate {
        ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_OneTimeWindows_StartDate {
            day: core::default::Default::default(),
            month: core::default::Default::default(),
            year: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_OneTimeWindows_StartDateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_OneTimeWindows_StartDateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_OneTimeWindows_StartDateRef {
        ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_OneTimeWindows_StartDateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_OneTimeWindows_StartDateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `day` after provisioning.\nDay of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant."]
    pub fn day(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }

    #[doc =
        "Get a reference to the value of field `month` after provisioning.\nMonth of a year. Must be from 1 to 12, or 0 to specify a year without a month and day."]
    pub fn month(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.month", self.base))
    }

    #[doc =
        "Get a reference to the value of field `year` after provisioning.\nYear of the date. Must be from 1 to 9999, or 0 to specify a date without a year."]
    pub fn year(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.year", self.base))
    }
}

#[derive(Serialize)]
pub struct ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindows_StartTime {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hours: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minutes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindows_StartTime {
    #[doc =
        "Set the field `hours`.\nHours of a day in 24 hour format. Must be greater than or equal to 0 and typically must be less than or equal to 23. An API may choose to allow the value \"24:00:00\" for scenarios like business closing time."]
    pub fn set_hours(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.hours = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `minutes`.\nMinutes of an hour. Must be greater than or equal to 0 and less than or equal to 59."]
    pub fn set_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minutes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `nanos`.\nFractions of seconds, in nanoseconds. Must be greater than or equal to 0 and less than or equal to 999,999,999."]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `seconds`.\nSeconds of a minute. Must be greater than or equal to 0 and typically must be less than or equal to 59. An API may allow the value 60 if it allows leap-seconds."]
    pub fn set_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindows_StartTime {
    type O =
        terrars
        ::BlockAssignable<ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindows_StartTime>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindows_StartTime {}

impl BuildClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindows_StartTime {
    pub fn build(self) -> ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindows_StartTime {
        ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindows_StartTime {
            hours: core::default::Default::default(),
            minutes: core::default::Default::default(),
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindows_StartTimeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindows_StartTimeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindows_StartTimeRef {
        ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindows_StartTimeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindows_StartTimeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `hours` after provisioning.\nHours of a day in 24 hour format. Must be greater than or equal to 0 and typically must be less than or equal to 23. An API may choose to allow the value \"24:00:00\" for scenarios like business closing time."]
    pub fn hours(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hours", self.base))
    }

    #[doc =
        "Get a reference to the value of field `minutes` after provisioning.\nMinutes of an hour. Must be greater than or equal to 0 and less than or equal to 59."]
    pub fn minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `nanos` after provisioning.\nFractions of seconds, in nanoseconds. Must be greater than or equal to 0 and less than or equal to 999,999,999."]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc =
        "Get a reference to the value of field `seconds` after provisioning.\nSeconds of a minute. Must be greater than or equal to 0 and typically must be less than or equal to 59. An API may allow the value 60 if it allows leap-seconds."]
    pub fn seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize, Default)]
struct ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_OneTimeWindowsDynamic {
    end_date: std
    ::option
    ::Option<
        terrars::DynamicBlock<ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_OneTimeWindows_EndDate>,
    >,
    end_time: std
    ::option
    ::Option<
        terrars::DynamicBlock<ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindows_EndTime>,
    >,
    start_date: std
    ::option
    ::Option<
        terrars::DynamicBlock<ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_OneTimeWindows_StartDate>,
    >,
    start_time: std
    ::option
    ::Option<
        terrars::DynamicBlock<ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindows_StartTime>,
    >,
}

#[derive(Serialize)]
pub struct ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_OneTimeWindows {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_date: std
    ::option
    ::Option<std::vec::Vec<ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_OneTimeWindows_EndDate>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_time: std
    ::option
    ::Option<std::vec::Vec<ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindows_EndTime>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_date: std
    ::option
    ::Option<std::vec::Vec<ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_OneTimeWindows_StartDate>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std
    ::option
    ::Option<std::vec::Vec<ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindows_StartTime>>,
    dynamic: ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_OneTimeWindowsDynamic,
}

impl ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_OneTimeWindows {
    #[doc = "Set the field `end_date`.\n"]
    pub fn set_end_date(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_OneTimeWindows_EndDate,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.end_date = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.end_date = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `end_time`.\n"]
    pub fn set_end_time(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindows_EndTime,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.end_time = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.end_time = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `start_date`.\n"]
    pub fn set_start_date(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_OneTimeWindows_StartDate,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.start_date = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.start_date = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindows_StartTime,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.start_time = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.start_time = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_OneTimeWindows {
    type O = terrars::BlockAssignable<ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_OneTimeWindows>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_OneTimeWindows {}

impl BuildClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_OneTimeWindows {
    pub fn build(self) -> ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_OneTimeWindows {
        ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_OneTimeWindows {
            end_date: core::default::Default::default(),
            end_time: core::default::Default::default(),
            start_date: core::default::Default::default(),
            start_time: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_OneTimeWindowsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_OneTimeWindowsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_OneTimeWindowsRef {
        ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_OneTimeWindowsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_OneTimeWindowsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end_date` after provisioning.\n"]
    pub fn end_date(
        &self,
    ) -> terrars::ListRef<ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_OneTimeWindows_EndDateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.end_date", self.base))
    }

    #[doc = "Get a reference to the value of field `end_time` after provisioning.\n"]
    pub fn end_time(
        &self,
    ) -> terrars::ListRef<ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindows_EndTimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.end_time", self.base))
    }

    #[doc = "Get a reference to the value of field `start_date` after provisioning.\n"]
    pub fn start_date(
        &self,
    ) -> terrars::ListRef<ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_OneTimeWindows_StartDateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.start_date", self.base))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(
        &self,
    ) -> terrars::ListRef<ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindows_StartTimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize, Default)]
struct ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindowsDynamic {
    end_time: std
    ::option
    ::Option<
        terrars::DynamicBlock<ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindows_EndTime>,
    >,
    start_time: std
    ::option
    ::Option<
        terrars::DynamicBlock<ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindows_StartTime>,
    >,
}

#[derive(Serialize)]
pub struct ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindows {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    days_of_week: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_time: std
    ::option
    ::Option<std::vec::Vec<ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindows_EndTime>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std
    ::option
    ::Option<std::vec::Vec<ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindows_StartTime>>,
    dynamic: ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindowsDynamic,
}

impl ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindows {
    #[doc =
        "Set the field `days_of_week`.\nDays of week. If left empty, all days of the week will be included. Possible values: [\"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub fn set_days_of_week(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.days_of_week = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `end_time`.\n"]
    pub fn set_end_time(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindows_EndTime,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.end_time = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.end_time = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindows_StartTime,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.start_time = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.start_time = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindows {
    type O = terrars::BlockAssignable<ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindows>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindows {}

impl BuildClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindows {
    pub fn build(self) -> ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindows {
        ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindows {
            days_of_week: core::default::Default::default(),
            end_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindowsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindowsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindowsRef {
        ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindowsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindowsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `days_of_week` after provisioning.\nDays of week. If left empty, all days of the week will be included. Possible values: [\"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub fn days_of_week(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.days_of_week", self.base))
    }

    #[doc = "Get a reference to the value of field `end_time` after provisioning.\n"]
    pub fn end_time(
        &self,
    ) -> terrars::ListRef<ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindows_EndTimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.end_time", self.base))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(
        &self,
    ) -> terrars::ListRef<ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindows_StartTimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize, Default)]
struct ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindowsDynamic {
    one_time_windows: std
    ::option
    ::Option<terrars::DynamicBlock<ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_OneTimeWindows>>,
    weekly_windows: std
    ::option
    ::Option<terrars::DynamicBlock<ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindows>>,
}

#[derive(Serialize)]
pub struct ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows {
    time_zone: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    one_time_windows: std
    ::option
    ::Option<std::vec::Vec<ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_OneTimeWindows>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weekly_windows: std
    ::option
    ::Option<std::vec::Vec<ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindows>>,
    dynamic: ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindowsDynamic,
}

impl ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows {
    #[doc = "Set the field `one_time_windows`.\n"]
    pub fn set_one_time_windows(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_OneTimeWindows>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.one_time_windows = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.one_time_windows = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `weekly_windows`.\n"]
    pub fn set_weekly_windows(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindows>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.weekly_windows = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.weekly_windows = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows {
    type O = terrars::BlockAssignable<ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows {
    #[doc = "The time zone in IANA format IANA Time Zone Database (e.g. America/New_York)."]
    pub time_zone: terrars::PrimField<std::string::String>,
}

impl BuildClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows {
    pub fn build(self) -> ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows {
        ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows {
            time_zone: self.time_zone,
            one_time_windows: core::default::Default::default(),
            weekly_windows: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindowsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindowsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindowsRef {
        ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindowsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindowsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `time_zone` after provisioning.\nThe time zone in IANA format IANA Time Zone Database (e.g. America/New_York)."]
    pub fn time_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_zone", self.base))
    }

    #[doc = "Get a reference to the value of field `one_time_windows` after provisioning.\n"]
    pub fn one_time_windows(
        &self,
    ) -> terrars::ListRef<ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_OneTimeWindowsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.one_time_windows", self.base))
    }

    #[doc = "Get a reference to the value of field `weekly_windows` after provisioning.\n"]
    pub fn weekly_windows(
        &self,
    ) -> terrars::ListRef<ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows_WeeklyWindowsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.weekly_windows", self.base))
    }
}

#[derive(Serialize, Default)]
struct ClouddeployDeployPolicy_Rules_RolloutRestrictionDynamic {
    time_windows: std
    ::option
    ::Option<terrars::DynamicBlock<ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows>>,
}

#[derive(Serialize)]
pub struct ClouddeployDeployPolicy_Rules_RolloutRestriction {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    actions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invokers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_windows: std::option::Option<std::vec::Vec<ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows>>,
    dynamic: ClouddeployDeployPolicy_Rules_RolloutRestrictionDynamic,
}

impl ClouddeployDeployPolicy_Rules_RolloutRestriction {
    #[doc =
        "Set the field `actions`.\nRollout actions to be restricted as part of the policy. If left empty, all actions will be restricted. Possible values: [\"ADVANCE\", \"APPROVE\", \"CANCEL\", \"CREATE\", \"IGNORE_JOB\", \"RETRY_JOB\", \"ROLLBACK\", \"TERMINATE_JOBRUN\"]"]
    pub fn set_actions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.actions = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `invokers`.\nWhat invoked the action. If left empty, all invoker types will be restricted. Possible values: [\"USER\", \"DEPLOY_AUTOMATION\"]"]
    pub fn set_invokers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.invokers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `time_windows`.\n"]
    pub fn set_time_windows(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindows>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.time_windows = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.time_windows = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ClouddeployDeployPolicy_Rules_RolloutRestriction {
    type O = terrars::BlockAssignable<ClouddeployDeployPolicy_Rules_RolloutRestriction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeployPolicy_Rules_RolloutRestriction {
    #[doc =
        "ID of the rule. This id must be unique in the 'DeployPolicy' resource to which this rule belongs. The format is 'a-z{0,62}'."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildClouddeployDeployPolicy_Rules_RolloutRestriction {
    pub fn build(self) -> ClouddeployDeployPolicy_Rules_RolloutRestriction {
        ClouddeployDeployPolicy_Rules_RolloutRestriction {
            actions: core::default::Default::default(),
            id: self.id,
            invokers: core::default::Default::default(),
            time_windows: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployDeployPolicy_Rules_RolloutRestrictionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployDeployPolicy_Rules_RolloutRestrictionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeployPolicy_Rules_RolloutRestrictionRef {
        ClouddeployDeployPolicy_Rules_RolloutRestrictionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeployPolicy_Rules_RolloutRestrictionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `actions` after provisioning.\nRollout actions to be restricted as part of the policy. If left empty, all actions will be restricted. Possible values: [\"ADVANCE\", \"APPROVE\", \"CANCEL\", \"CREATE\", \"IGNORE_JOB\", \"RETRY_JOB\", \"ROLLBACK\", \"TERMINATE_JOBRUN\"]"]
    pub fn actions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.actions", self.base))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nID of the rule. This id must be unique in the 'DeployPolicy' resource to which this rule belongs. The format is 'a-z{0,62}'."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `invokers` after provisioning.\nWhat invoked the action. If left empty, all invoker types will be restricted. Possible values: [\"USER\", \"DEPLOY_AUTOMATION\"]"]
    pub fn invokers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.invokers", self.base))
    }

    #[doc = "Get a reference to the value of field `time_windows` after provisioning.\n"]
    pub fn time_windows(&self) -> terrars::ListRef<ClouddeployDeployPolicy_Rules_RolloutRestriction_TimeWindowsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.time_windows", self.base))
    }
}

#[derive(Serialize, Default)]
struct ClouddeployDeployPolicy_RulesDynamic {
    rollout_restriction: std
    ::option
    ::Option<terrars::DynamicBlock<ClouddeployDeployPolicy_Rules_RolloutRestriction>>,
}

#[derive(Serialize)]
pub struct ClouddeployDeployPolicy_Rules {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rollout_restriction: std::option::Option<std::vec::Vec<ClouddeployDeployPolicy_Rules_RolloutRestriction>>,
    dynamic: ClouddeployDeployPolicy_RulesDynamic,
}

impl ClouddeployDeployPolicy_Rules {
    #[doc = "Set the field `rollout_restriction`.\n"]
    pub fn set_rollout_restriction(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ClouddeployDeployPolicy_Rules_RolloutRestriction>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rollout_restriction = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rollout_restriction = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ClouddeployDeployPolicy_Rules {
    type O = terrars::BlockAssignable<ClouddeployDeployPolicy_Rules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeployPolicy_Rules {}

impl BuildClouddeployDeployPolicy_Rules {
    pub fn build(self) -> ClouddeployDeployPolicy_Rules {
        ClouddeployDeployPolicy_Rules {
            rollout_restriction: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployDeployPolicy_RulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployDeployPolicy_RulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ClouddeployDeployPolicy_RulesRef {
        ClouddeployDeployPolicy_RulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeployPolicy_RulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rollout_restriction` after provisioning.\n"]
    pub fn rollout_restriction(&self) -> terrars::ListRef<ClouddeployDeployPolicy_Rules_RolloutRestrictionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rollout_restriction", self.base))
    }
}

#[derive(Serialize)]
pub struct ClouddeployDeployPolicy_Selectors_DeliveryPipeline {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl ClouddeployDeployPolicy_Selectors_DeliveryPipeline {
    #[doc =
        "Set the field `id`.\nID of the DeliveryPipeline. The value of this field could be one of the following:\n- The last segment of a pipeline name\n- \"*\", all delivery pipelines in a location"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `labels`.\nDeliveryPipeline labels."]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.labels = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ClouddeployDeployPolicy_Selectors_DeliveryPipeline {
    type O = terrars::BlockAssignable<ClouddeployDeployPolicy_Selectors_DeliveryPipeline>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeployPolicy_Selectors_DeliveryPipeline {}

impl BuildClouddeployDeployPolicy_Selectors_DeliveryPipeline {
    pub fn build(self) -> ClouddeployDeployPolicy_Selectors_DeliveryPipeline {
        ClouddeployDeployPolicy_Selectors_DeliveryPipeline {
            id: core::default::Default::default(),
            labels: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployDeployPolicy_Selectors_DeliveryPipelineRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployDeployPolicy_Selectors_DeliveryPipelineRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployDeployPolicy_Selectors_DeliveryPipelineRef {
        ClouddeployDeployPolicy_Selectors_DeliveryPipelineRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeployPolicy_Selectors_DeliveryPipelineRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nID of the DeliveryPipeline. The value of this field could be one of the following:\n- The last segment of a pipeline name\n- \"*\", all delivery pipelines in a location"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `labels` after provisioning.\nDeliveryPipeline labels."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
}

#[derive(Serialize)]
pub struct ClouddeployDeployPolicy_Selectors_Target {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl ClouddeployDeployPolicy_Selectors_Target {
    #[doc =
        "Set the field `id`.\nID of the 'Target'. The value of this field could be one of the following: * The last segment of a target name. It only needs the ID to determine which target is being referred to * \"*\", all targets in a location."]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `labels`.\nTarget labels."]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.labels = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ClouddeployDeployPolicy_Selectors_Target {
    type O = terrars::BlockAssignable<ClouddeployDeployPolicy_Selectors_Target>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeployPolicy_Selectors_Target {}

impl BuildClouddeployDeployPolicy_Selectors_Target {
    pub fn build(self) -> ClouddeployDeployPolicy_Selectors_Target {
        ClouddeployDeployPolicy_Selectors_Target {
            id: core::default::Default::default(),
            labels: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployDeployPolicy_Selectors_TargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployDeployPolicy_Selectors_TargetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ClouddeployDeployPolicy_Selectors_TargetRef {
        ClouddeployDeployPolicy_Selectors_TargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeployPolicy_Selectors_TargetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nID of the 'Target'. The value of this field could be one of the following: * The last segment of a target name. It only needs the ID to determine which target is being referred to * \"*\", all targets in a location."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `labels` after provisioning.\nTarget labels."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
}

#[derive(Serialize, Default)]
struct ClouddeployDeployPolicy_SelectorsDynamic {
    delivery_pipeline: std::option::Option<terrars::DynamicBlock<ClouddeployDeployPolicy_Selectors_DeliveryPipeline>>,
    target: std::option::Option<terrars::DynamicBlock<ClouddeployDeployPolicy_Selectors_Target>>,
}

#[derive(Serialize)]
pub struct ClouddeployDeployPolicy_Selectors {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delivery_pipeline: std::option::Option<std::vec::Vec<ClouddeployDeployPolicy_Selectors_DeliveryPipeline>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target: std::option::Option<std::vec::Vec<ClouddeployDeployPolicy_Selectors_Target>>,
    dynamic: ClouddeployDeployPolicy_SelectorsDynamic,
}

impl ClouddeployDeployPolicy_Selectors {
    #[doc = "Set the field `delivery_pipeline`.\n"]
    pub fn set_delivery_pipeline(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ClouddeployDeployPolicy_Selectors_DeliveryPipeline>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.delivery_pipeline = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.delivery_pipeline = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `target`.\n"]
    pub fn set_target(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ClouddeployDeployPolicy_Selectors_Target>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.target = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.target = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ClouddeployDeployPolicy_Selectors {
    type O = terrars::BlockAssignable<ClouddeployDeployPolicy_Selectors>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeployPolicy_Selectors {}

impl BuildClouddeployDeployPolicy_Selectors {
    pub fn build(self) -> ClouddeployDeployPolicy_Selectors {
        ClouddeployDeployPolicy_Selectors {
            delivery_pipeline: core::default::Default::default(),
            target: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployDeployPolicy_SelectorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployDeployPolicy_SelectorsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ClouddeployDeployPolicy_SelectorsRef {
        ClouddeployDeployPolicy_SelectorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeployPolicy_SelectorsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delivery_pipeline` after provisioning.\n"]
    pub fn delivery_pipeline(&self) -> terrars::ListRef<ClouddeployDeployPolicy_Selectors_DeliveryPipelineRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.delivery_pipeline", self.base))
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> terrars::ListRef<ClouddeployDeployPolicy_Selectors_TargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target", self.base))
    }
}

#[derive(Serialize)]
pub struct ClouddeployDeployPolicy_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ClouddeployDeployPolicy_Timeouts {
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

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ClouddeployDeployPolicy_Timeouts {
    type O = terrars::BlockAssignable<ClouddeployDeployPolicy_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployDeployPolicy_Timeouts {}

impl BuildClouddeployDeployPolicy_Timeouts {
    pub fn build(self) -> ClouddeployDeployPolicy_Timeouts {
        ClouddeployDeployPolicy_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployDeployPolicy_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployDeployPolicy_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ClouddeployDeployPolicy_TimeoutsRef {
        ClouddeployDeployPolicy_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployDeployPolicy_TimeoutsRef {
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

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
