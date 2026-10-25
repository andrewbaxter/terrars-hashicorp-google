#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ClouddeployAutomationData_ {
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
    delivery_pipeline: terrars::PrimField<std::string::String>,
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
    service_account: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    suspended: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rules: std::option::Option<std::vec::Vec<ClouddeployAutomation_Rules>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    selector: std::option::Option<std::vec::Vec<ClouddeployAutomation_Selector>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ClouddeployAutomation_Timeouts>,
    dynamic: ClouddeployAutomationDynamic,
}

struct ClouddeployAutomation_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ClouddeployAutomationData_>,
}

#[derive(Clone)]
pub struct ClouddeployAutomation(std::rc::Rc<ClouddeployAutomation_>);

impl ClouddeployAutomation {
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
        "Set the field `annotations`.\nOptional. User annotations. These attributes can only be set and used by the user, and not by Cloud Deploy. Annotations must meet the following constraints: * Annotations are key/value pairs. * Valid annotation keys have two segments: an optional prefix and name, separated by a slash ('/'). * The name segment is required and must be 63 characters or less, beginning and ending with an alphanumeric character ('[a-z0-9A-Z]') with dashes ('-'), underscores ('_'), dots ('.'), and alphanumerics between. * The prefix is optional. If specified, the prefix must be a DNS subdomain: a series of DNS labels separated by dots('.'), not longer than 253 characters in total, followed by a slash ('/'). See https://kubernetes.io/docs/concepts/overview/working-with-objects/annotations/#syntax-and-character-set for more details.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
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

    #[doc = "Set the field `description`.\nOptional. Description of the 'Automation'. Max length is 255 characters."]
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
        "Set the field `labels`.\nOptional. Labels are attributes that can be set and used by both the user and by Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 63 characters.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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

    #[doc = "Set the field `suspended`.\nOptional. When Suspended, automation is deactivated from execution."]
    pub fn set_suspended(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().suspended = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rules`.\n"]
    pub fn set_rules(self, v: impl std::convert::Into<terrars::BlockAssignable<ClouddeployAutomation_Rules>>) -> Self {
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

    #[doc = "Set the field `selector`.\n"]
    pub fn set_selector(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ClouddeployAutomation_Selector>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().selector = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.selector = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ClouddeployAutomation_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nOptional. User annotations. These attributes can only be set and used by the user, and not by Cloud Deploy. Annotations must meet the following constraints: * Annotations are key/value pairs. * Valid annotation keys have two segments: an optional prefix and name, separated by a slash ('/'). * The name segment is required and must be 63 characters or less, beginning and ending with an alphanumeric character ('[a-z0-9A-Z]') with dashes ('-'), underscores ('_'), dots ('.'), and alphanumerics between. * The prefix is optional. If specified, the prefix must be a DNS subdomain: a series of DNS labels separated by dots('.'), not longer than 253 characters in total, followed by a slash ('/'). See https://kubernetes.io/docs/concepts/overview/working-with-objects/annotations/#syntax-and-character-set for more details.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Time at which the automation was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `delivery_pipeline` after provisioning.\nThe delivery_pipeline for the resource"]
    pub fn delivery_pipeline(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delivery_pipeline", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOptional. Description of the 'Automation'. Max length is 255 characters."]
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
        "Get a reference to the value of field `etag` after provisioning.\nOptional. The weak etag of the 'Automation' resource. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nOptional. Labels are attributes that can be set and used by both the user and by Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 63 characters.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the 'Automation'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nRequired. Email address of the user-managed IAM service account that creates Cloud Deploy release and rollout resources."]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `suspended` after provisioning.\nOptional. When Suspended, automation is deactivated from execution."]
    pub fn suspended(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.suspended", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nOutput only. Unique identifier of the 'Automation'."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Time at which the automation was updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> terrars::ListRef<ClouddeployAutomation_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `selector` after provisioning.\n"]
    pub fn selector(&self) -> terrars::ListRef<ClouddeployAutomation_SelectorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.selector", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ClouddeployAutomation_TimeoutsRef {
        ClouddeployAutomation_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ClouddeployAutomation {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ClouddeployAutomation { }

impl terrars::ToListMappable for ClouddeployAutomation {
    type O = terrars::ListRef<ClouddeployAutomationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ClouddeployAutomation_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_clouddeploy_automation".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildClouddeployAutomation {
    pub tf_id: std::string::String,
    #[doc = "The delivery_pipeline for the resource"]
    pub delivery_pipeline: terrars::PrimField<std::string::String>,
    #[doc = "The location for the resource"]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "Name of the 'Automation'."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc =
        "Required. Email address of the user-managed IAM service account that creates Cloud Deploy release and rollout resources."]
    pub service_account: terrars::PrimField<std::string::String>,
}

impl BuildClouddeployAutomation {
    pub fn build(self, stack: &mut terrars::Stack) -> ClouddeployAutomation {
        let out = ClouddeployAutomation(std::rc::Rc::new(ClouddeployAutomation_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ClouddeployAutomationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                annotations: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                delivery_pipeline: self.delivery_pipeline,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                service_account: self.service_account,
                suspended: core::default::Default::default(),
                rules: core::default::Default::default(),
                selector: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ClouddeployAutomationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployAutomationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ClouddeployAutomationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nOptional. User annotations. These attributes can only be set and used by the user, and not by Cloud Deploy. Annotations must meet the following constraints: * Annotations are key/value pairs. * Valid annotation keys have two segments: an optional prefix and name, separated by a slash ('/'). * The name segment is required and must be 63 characters or less, beginning and ending with an alphanumeric character ('[a-z0-9A-Z]') with dashes ('-'), underscores ('_'), dots ('.'), and alphanumerics between. * The prefix is optional. If specified, the prefix must be a DNS subdomain: a series of DNS labels separated by dots('.'), not longer than 253 characters in total, followed by a slash ('/'). See https://kubernetes.io/docs/concepts/overview/working-with-objects/annotations/#syntax-and-character-set for more details.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Time at which the automation was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `delivery_pipeline` after provisioning.\nThe delivery_pipeline for the resource"]
    pub fn delivery_pipeline(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delivery_pipeline", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOptional. Description of the 'Automation'. Max length is 255 characters."]
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
        "Get a reference to the value of field `etag` after provisioning.\nOptional. The weak etag of the 'Automation' resource. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nOptional. Labels are attributes that can be set and used by both the user and by Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 63 characters.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the 'Automation'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nRequired. Email address of the user-managed IAM service account that creates Cloud Deploy release and rollout resources."]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `suspended` after provisioning.\nOptional. When Suspended, automation is deactivated from execution."]
    pub fn suspended(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.suspended", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nOutput only. Unique identifier of the 'Automation'."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Time at which the automation was updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> terrars::ListRef<ClouddeployAutomation_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `selector` after provisioning.\n"]
    pub fn selector(&self) -> terrars::ListRef<ClouddeployAutomation_SelectorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.selector", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ClouddeployAutomation_TimeoutsRef {
        ClouddeployAutomation_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ClouddeployAutomationDynamic {
    rules: std::option::Option<terrars::DynamicBlock<ClouddeployAutomation_Rules>>,
    selector: std::option::Option<terrars::DynamicBlock<ClouddeployAutomation_Selector>>,
}

#[derive(Serialize)]
pub struct ClouddeployAutomation_Rules_AdvanceRolloutRule {
    id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_phases: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wait: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ClouddeployAutomation_Rules_AdvanceRolloutRule {
    #[doc =
        "Set the field `source_phases`.\nOptional. Proceeds only after phase name matched any one in the list. This value must consist of lower-case letters, numbers, and hyphens, start with a letter and end with a letter or a number, and have a max length of 63 characters. In other words, it must match the following regex: '^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$'."]
    pub fn set_source_phases(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.source_phases = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `wait`.\nOptional. How long to wait after a rollout is finished."]
    pub fn set_wait(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.wait = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ClouddeployAutomation_Rules_AdvanceRolloutRule {
    type O = terrars::BlockAssignable<ClouddeployAutomation_Rules_AdvanceRolloutRule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployAutomation_Rules_AdvanceRolloutRule {
    #[doc =
        "Required. ID of the rule. This id must be unique in the 'Automation' resource to which this rule belongs. The format is 'a-z{0,62}'."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildClouddeployAutomation_Rules_AdvanceRolloutRule {
    pub fn build(self) -> ClouddeployAutomation_Rules_AdvanceRolloutRule {
        ClouddeployAutomation_Rules_AdvanceRolloutRule {
            id: self.id,
            source_phases: core::default::Default::default(),
            wait: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployAutomation_Rules_AdvanceRolloutRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployAutomation_Rules_AdvanceRolloutRuleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployAutomation_Rules_AdvanceRolloutRuleRef {
        ClouddeployAutomation_Rules_AdvanceRolloutRuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployAutomation_Rules_AdvanceRolloutRuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nRequired. ID of the rule. This id must be unique in the 'Automation' resource to which this rule belongs. The format is 'a-z{0,62}'."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_phases` after provisioning.\nOptional. Proceeds only after phase name matched any one in the list. This value must consist of lower-case letters, numbers, and hyphens, start with a letter and end with a letter or a number, and have a max length of 63 characters. In other words, it must match the following regex: '^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$'."]
    pub fn source_phases(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_phases", self.base))
    }

    #[doc =
        "Get a reference to the value of field `wait` after provisioning.\nOptional. How long to wait after a rollout is finished."]
    pub fn wait(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wait", self.base))
    }
}

#[derive(Serialize)]
pub struct ClouddeployAutomation_Rules_PromoteReleaseRule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_phase: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_target_id: std::option::Option<terrars::PrimField<std::string::String>>,
    id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wait: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ClouddeployAutomation_Rules_PromoteReleaseRule {
    #[doc =
        "Set the field `destination_phase`.\nOptional. The starting phase of the rollout created by this operation. Default to the first phase."]
    pub fn set_destination_phase(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.destination_phase = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `destination_target_id`.\nOptional. The ID of the stage in the pipeline to which this 'Release' is deploying. If unspecified, default it to the next stage in the promotion flow. The value of this field could be one of the following: * The last segment of a target name. It only needs the ID to determine if the target is one of the stages in the promotion sequence defined in the pipeline. * \"@next\", the next target in the promotion sequence."]
    pub fn set_destination_target_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.destination_target_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `wait`.\nOptional. How long the release need to be paused until being promoted to the next target."]
    pub fn set_wait(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.wait = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ClouddeployAutomation_Rules_PromoteReleaseRule {
    type O = terrars::BlockAssignable<ClouddeployAutomation_Rules_PromoteReleaseRule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployAutomation_Rules_PromoteReleaseRule {
    #[doc =
        "Required. ID of the rule. This id must be unique in the 'Automation' resource to which this rule belongs. The format is 'a-z{0,62}'."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildClouddeployAutomation_Rules_PromoteReleaseRule {
    pub fn build(self) -> ClouddeployAutomation_Rules_PromoteReleaseRule {
        ClouddeployAutomation_Rules_PromoteReleaseRule {
            destination_phase: core::default::Default::default(),
            destination_target_id: core::default::Default::default(),
            id: self.id,
            wait: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployAutomation_Rules_PromoteReleaseRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployAutomation_Rules_PromoteReleaseRuleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployAutomation_Rules_PromoteReleaseRuleRef {
        ClouddeployAutomation_Rules_PromoteReleaseRuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployAutomation_Rules_PromoteReleaseRuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `destination_phase` after provisioning.\nOptional. The starting phase of the rollout created by this operation. Default to the first phase."]
    pub fn destination_phase(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_phase", self.base))
    }

    #[doc =
        "Get a reference to the value of field `destination_target_id` after provisioning.\nOptional. The ID of the stage in the pipeline to which this 'Release' is deploying. If unspecified, default it to the next stage in the promotion flow. The value of this field could be one of the following: * The last segment of a target name. It only needs the ID to determine if the target is one of the stages in the promotion sequence defined in the pipeline. * \"@next\", the next target in the promotion sequence."]
    pub fn destination_target_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_target_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nRequired. ID of the rule. This id must be unique in the 'Automation' resource to which this rule belongs. The format is 'a-z{0,62}'."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `wait` after provisioning.\nOptional. How long the release need to be paused until being promoted to the next target."]
    pub fn wait(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wait", self.base))
    }
}

#[derive(Serialize)]
pub struct ClouddeployAutomation_Rules_RepairRolloutRule_RepairPhases_Retry {
    attempts: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backoff_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wait: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ClouddeployAutomation_Rules_RepairRolloutRule_RepairPhases_Retry {
    #[doc =
        "Set the field `backoff_mode`.\nOptional. The pattern of how wait time will be increased. Default is linear. Backoff mode will be ignored if wait is 0. Possible values: [\"BACKOFF_MODE_UNSPECIFIED\", \"BACKOFF_MODE_LINEAR\", \"BACKOFF_MODE_EXPONENTIAL\"]"]
    pub fn set_backoff_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.backoff_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `wait`.\nOptional. How long to wait for the first retry. Default is 0, and the maximum value is 14d. A duration in seconds with up to nine fractional digits, ending with 's'. Example: '3.5s'."]
    pub fn set_wait(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.wait = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ClouddeployAutomation_Rules_RepairRolloutRule_RepairPhases_Retry {
    type O = terrars::BlockAssignable<ClouddeployAutomation_Rules_RepairRolloutRule_RepairPhases_Retry>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployAutomation_Rules_RepairRolloutRule_RepairPhases_Retry {
    #[doc =
        "Required. Total number of retries. Retry is skipped if set to 0; The minimum value is 1, and the maximum value is 10."]
    pub attempts: terrars::PrimField<std::string::String>,
}

impl BuildClouddeployAutomation_Rules_RepairRolloutRule_RepairPhases_Retry {
    pub fn build(self) -> ClouddeployAutomation_Rules_RepairRolloutRule_RepairPhases_Retry {
        ClouddeployAutomation_Rules_RepairRolloutRule_RepairPhases_Retry {
            attempts: self.attempts,
            backoff_mode: core::default::Default::default(),
            wait: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployAutomation_Rules_RepairRolloutRule_RepairPhases_RetryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployAutomation_Rules_RepairRolloutRule_RepairPhases_RetryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployAutomation_Rules_RepairRolloutRule_RepairPhases_RetryRef {
        ClouddeployAutomation_Rules_RepairRolloutRule_RepairPhases_RetryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployAutomation_Rules_RepairRolloutRule_RepairPhases_RetryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `attempts` after provisioning.\nRequired. Total number of retries. Retry is skipped if set to 0; The minimum value is 1, and the maximum value is 10."]
    pub fn attempts(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attempts", self.base))
    }

    #[doc =
        "Get a reference to the value of field `backoff_mode` after provisioning.\nOptional. The pattern of how wait time will be increased. Default is linear. Backoff mode will be ignored if wait is 0. Possible values: [\"BACKOFF_MODE_UNSPECIFIED\", \"BACKOFF_MODE_LINEAR\", \"BACKOFF_MODE_EXPONENTIAL\"]"]
    pub fn backoff_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backoff_mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `wait` after provisioning.\nOptional. How long to wait for the first retry. Default is 0, and the maximum value is 14d. A duration in seconds with up to nine fractional digits, ending with 's'. Example: '3.5s'."]
    pub fn wait(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wait", self.base))
    }
}

#[derive(Serialize)]
pub struct ClouddeployAutomation_Rules_RepairRolloutRule_RepairPhases_Rollback {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_phase: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_rollback_if_rollout_pending: std::option::Option<terrars::PrimField<bool>>,
}

impl ClouddeployAutomation_Rules_RepairRolloutRule_RepairPhases_Rollback {
    #[doc =
        "Set the field `destination_phase`.\nOptional. The starting phase ID for the Rollout. If unspecified, the Rollout will start in the stable phase."]
    pub fn set_destination_phase(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.destination_phase = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disable_rollback_if_rollout_pending`.\nOptional. If pending rollout exists on the target, the rollback operation will be aborted."]
    pub fn set_disable_rollback_if_rollout_pending(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.disable_rollback_if_rollout_pending = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ClouddeployAutomation_Rules_RepairRolloutRule_RepairPhases_Rollback {
    type O = terrars::BlockAssignable<ClouddeployAutomation_Rules_RepairRolloutRule_RepairPhases_Rollback>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployAutomation_Rules_RepairRolloutRule_RepairPhases_Rollback {}

impl BuildClouddeployAutomation_Rules_RepairRolloutRule_RepairPhases_Rollback {
    pub fn build(self) -> ClouddeployAutomation_Rules_RepairRolloutRule_RepairPhases_Rollback {
        ClouddeployAutomation_Rules_RepairRolloutRule_RepairPhases_Rollback {
            destination_phase: core::default::Default::default(),
            disable_rollback_if_rollout_pending: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployAutomation_Rules_RepairRolloutRule_RepairPhases_RollbackRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployAutomation_Rules_RepairRolloutRule_RepairPhases_RollbackRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployAutomation_Rules_RepairRolloutRule_RepairPhases_RollbackRef {
        ClouddeployAutomation_Rules_RepairRolloutRule_RepairPhases_RollbackRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployAutomation_Rules_RepairRolloutRule_RepairPhases_RollbackRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `destination_phase` after provisioning.\nOptional. The starting phase ID for the Rollout. If unspecified, the Rollout will start in the stable phase."]
    pub fn destination_phase(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_phase", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disable_rollback_if_rollout_pending` after provisioning.\nOptional. If pending rollout exists on the target, the rollback operation will be aborted."]
    pub fn disable_rollback_if_rollout_pending(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_rollback_if_rollout_pending", self.base))
    }
}

#[derive(Serialize, Default)]
struct ClouddeployAutomation_Rules_RepairRolloutRule_RepairPhasesDynamic {
    retry: std
    ::option
    ::Option<terrars::DynamicBlock<ClouddeployAutomation_Rules_RepairRolloutRule_RepairPhases_Retry>>,
    rollback: std
    ::option
    ::Option<terrars::DynamicBlock<ClouddeployAutomation_Rules_RepairRolloutRule_RepairPhases_Rollback>>,
}

#[derive(Serialize)]
pub struct ClouddeployAutomation_Rules_RepairRolloutRule_RepairPhases {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry: std::option::Option<std::vec::Vec<ClouddeployAutomation_Rules_RepairRolloutRule_RepairPhases_Retry>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rollback: std::option::Option<std::vec::Vec<ClouddeployAutomation_Rules_RepairRolloutRule_RepairPhases_Rollback>>,
    dynamic: ClouddeployAutomation_Rules_RepairRolloutRule_RepairPhasesDynamic,
}

impl ClouddeployAutomation_Rules_RepairRolloutRule_RepairPhases {
    #[doc = "Set the field `retry`.\n"]
    pub fn set_retry(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ClouddeployAutomation_Rules_RepairRolloutRule_RepairPhases_Retry>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.retry = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.retry = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rollback`.\n"]
    pub fn set_rollback(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ClouddeployAutomation_Rules_RepairRolloutRule_RepairPhases_Rollback>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rollback = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rollback = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ClouddeployAutomation_Rules_RepairRolloutRule_RepairPhases {
    type O = terrars::BlockAssignable<ClouddeployAutomation_Rules_RepairRolloutRule_RepairPhases>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployAutomation_Rules_RepairRolloutRule_RepairPhases {}

impl BuildClouddeployAutomation_Rules_RepairRolloutRule_RepairPhases {
    pub fn build(self) -> ClouddeployAutomation_Rules_RepairRolloutRule_RepairPhases {
        ClouddeployAutomation_Rules_RepairRolloutRule_RepairPhases {
            retry: core::default::Default::default(),
            rollback: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployAutomation_Rules_RepairRolloutRule_RepairPhasesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployAutomation_Rules_RepairRolloutRule_RepairPhasesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployAutomation_Rules_RepairRolloutRule_RepairPhasesRef {
        ClouddeployAutomation_Rules_RepairRolloutRule_RepairPhasesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployAutomation_Rules_RepairRolloutRule_RepairPhasesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `retry` after provisioning.\n"]
    pub fn retry(&self) -> terrars::ListRef<ClouddeployAutomation_Rules_RepairRolloutRule_RepairPhases_RetryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry", self.base))
    }

    #[doc = "Get a reference to the value of field `rollback` after provisioning.\n"]
    pub fn rollback(&self) -> terrars::ListRef<ClouddeployAutomation_Rules_RepairRolloutRule_RepairPhases_RollbackRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rollback", self.base))
    }
}

#[derive(Serialize, Default)]
struct ClouddeployAutomation_Rules_RepairRolloutRuleDynamic {
    repair_phases: std
    ::option
    ::Option<terrars::DynamicBlock<ClouddeployAutomation_Rules_RepairRolloutRule_RepairPhases>>,
}

#[derive(Serialize)]
pub struct ClouddeployAutomation_Rules_RepairRolloutRule {
    id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jobs: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    phases: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repair_phases: std::option::Option<std::vec::Vec<ClouddeployAutomation_Rules_RepairRolloutRule_RepairPhases>>,
    dynamic: ClouddeployAutomation_Rules_RepairRolloutRuleDynamic,
}

impl ClouddeployAutomation_Rules_RepairRolloutRule {
    #[doc =
        "Set the field `jobs`.\nOptional. Jobs to repair. Proceeds only after job name matched any one in the list, or for all jobs if unspecified or empty. The phase that includes the job must match the phase ID specified in sourcePhase. This value must consist of lower-case letters, numbers, and hyphens, start with a letter and end with a letter or a number, and have a max length of 63 characters. In other words, it must match the following regex: ^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$."]
    pub fn set_jobs(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.jobs = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `phases`.\nOptional. Phases within which jobs are subject to automatic repair actions on failure. Proceeds only after phase name matched any one in the list, or for all phases if unspecified. This value must consist of lower-case letters, numbers, and hyphens, start with a letter and end with a letter or a number, and have a max length of 63 characters. In other words, it must match the following regex: ^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$."]
    pub fn set_phases(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.phases = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repair_phases`.\n"]
    pub fn set_repair_phases(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ClouddeployAutomation_Rules_RepairRolloutRule_RepairPhases>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.repair_phases = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.repair_phases = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ClouddeployAutomation_Rules_RepairRolloutRule {
    type O = terrars::BlockAssignable<ClouddeployAutomation_Rules_RepairRolloutRule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployAutomation_Rules_RepairRolloutRule {
    #[doc =
        "Required. ID of the rule. This id must be unique in the 'Automation' resource to which this rule belongs. The format is 'a-z{0,62}'."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildClouddeployAutomation_Rules_RepairRolloutRule {
    pub fn build(self) -> ClouddeployAutomation_Rules_RepairRolloutRule {
        ClouddeployAutomation_Rules_RepairRolloutRule {
            id: self.id,
            jobs: core::default::Default::default(),
            phases: core::default::Default::default(),
            repair_phases: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployAutomation_Rules_RepairRolloutRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployAutomation_Rules_RepairRolloutRuleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployAutomation_Rules_RepairRolloutRuleRef {
        ClouddeployAutomation_Rules_RepairRolloutRuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployAutomation_Rules_RepairRolloutRuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nRequired. ID of the rule. This id must be unique in the 'Automation' resource to which this rule belongs. The format is 'a-z{0,62}'."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `jobs` after provisioning.\nOptional. Jobs to repair. Proceeds only after job name matched any one in the list, or for all jobs if unspecified or empty. The phase that includes the job must match the phase ID specified in sourcePhase. This value must consist of lower-case letters, numbers, and hyphens, start with a letter and end with a letter or a number, and have a max length of 63 characters. In other words, it must match the following regex: ^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$."]
    pub fn jobs(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jobs", self.base))
    }

    #[doc =
        "Get a reference to the value of field `phases` after provisioning.\nOptional. Phases within which jobs are subject to automatic repair actions on failure. Proceeds only after phase name matched any one in the list, or for all phases if unspecified. This value must consist of lower-case letters, numbers, and hyphens, start with a letter and end with a letter or a number, and have a max length of 63 characters. In other words, it must match the following regex: ^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$."]
    pub fn phases(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.phases", self.base))
    }

    #[doc = "Get a reference to the value of field `repair_phases` after provisioning.\n"]
    pub fn repair_phases(&self) -> terrars::ListRef<ClouddeployAutomation_Rules_RepairRolloutRule_RepairPhasesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.repair_phases", self.base))
    }
}

#[derive(Serialize)]
pub struct ClouddeployAutomation_Rules_TimedPromoteReleaseRule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_phase: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_target_id: std::option::Option<terrars::PrimField<std::string::String>>,
    id: terrars::PrimField<std::string::String>,
    schedule: terrars::PrimField<std::string::String>,
    time_zone: terrars::PrimField<std::string::String>,
}

impl ClouddeployAutomation_Rules_TimedPromoteReleaseRule {
    #[doc =
        "Set the field `destination_phase`.\nOptional. The starting phase of the rollout created by this rule. Default to the first phase."]
    pub fn set_destination_phase(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.destination_phase = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `destination_target_id`.\nOptional. The ID of the stage in the pipeline to which this Release is deploying. If unspecified, default it to the next stage in the promotion flow. The value of this field could be one of the following:\n  - The last segment of a target name\n  - \"@next\", the next target in the promotion sequence\""]
    pub fn set_destination_target_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.destination_target_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ClouddeployAutomation_Rules_TimedPromoteReleaseRule {
    type O = terrars::BlockAssignable<ClouddeployAutomation_Rules_TimedPromoteReleaseRule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployAutomation_Rules_TimedPromoteReleaseRule {
    #[doc =
        "Required. ID of the rule. This id must be unique in the 'Automation' resource to which this rule belongs. The format is 'a-z{0,62}'."]
    pub id: terrars::PrimField<std::string::String>,
    #[doc = "Required. Schedule in crontab format. e.g. '0 9 * * 1' for every Monday at 9am."]
    pub schedule: terrars::PrimField<std::string::String>,
    #[doc = "Required. The time zone in IANA format IANA Time Zone Database (e.g. America/New_York)."]
    pub time_zone: terrars::PrimField<std::string::String>,
}

impl BuildClouddeployAutomation_Rules_TimedPromoteReleaseRule {
    pub fn build(self) -> ClouddeployAutomation_Rules_TimedPromoteReleaseRule {
        ClouddeployAutomation_Rules_TimedPromoteReleaseRule {
            destination_phase: core::default::Default::default(),
            destination_target_id: core::default::Default::default(),
            id: self.id,
            schedule: self.schedule,
            time_zone: self.time_zone,
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployAutomation_Rules_TimedPromoteReleaseRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployAutomation_Rules_TimedPromoteReleaseRuleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployAutomation_Rules_TimedPromoteReleaseRuleRef {
        ClouddeployAutomation_Rules_TimedPromoteReleaseRuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployAutomation_Rules_TimedPromoteReleaseRuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `destination_phase` after provisioning.\nOptional. The starting phase of the rollout created by this rule. Default to the first phase."]
    pub fn destination_phase(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_phase", self.base))
    }

    #[doc =
        "Get a reference to the value of field `destination_target_id` after provisioning.\nOptional. The ID of the stage in the pipeline to which this Release is deploying. If unspecified, default it to the next stage in the promotion flow. The value of this field could be one of the following:\n  - The last segment of a target name\n  - \"@next\", the next target in the promotion sequence\""]
    pub fn destination_target_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_target_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nRequired. ID of the rule. This id must be unique in the 'Automation' resource to which this rule belongs. The format is 'a-z{0,62}'."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `schedule` after provisioning.\nRequired. Schedule in crontab format. e.g. '0 9 * * 1' for every Monday at 9am."]
    pub fn schedule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule", self.base))
    }

    #[doc =
        "Get a reference to the value of field `time_zone` after provisioning.\nRequired. The time zone in IANA format IANA Time Zone Database (e.g. America/New_York)."]
    pub fn time_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_zone", self.base))
    }
}

#[derive(Serialize, Default)]
struct ClouddeployAutomation_RulesDynamic {
    advance_rollout_rule: std::option::Option<terrars::DynamicBlock<ClouddeployAutomation_Rules_AdvanceRolloutRule>>,
    promote_release_rule: std::option::Option<terrars::DynamicBlock<ClouddeployAutomation_Rules_PromoteReleaseRule>>,
    repair_rollout_rule: std::option::Option<terrars::DynamicBlock<ClouddeployAutomation_Rules_RepairRolloutRule>>,
    timed_promote_release_rule: std
    ::option
    ::Option<terrars::DynamicBlock<ClouddeployAutomation_Rules_TimedPromoteReleaseRule>>,
}

#[derive(Serialize)]
pub struct ClouddeployAutomation_Rules {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advance_rollout_rule: std::option::Option<std::vec::Vec<ClouddeployAutomation_Rules_AdvanceRolloutRule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    promote_release_rule: std::option::Option<std::vec::Vec<ClouddeployAutomation_Rules_PromoteReleaseRule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repair_rollout_rule: std::option::Option<std::vec::Vec<ClouddeployAutomation_Rules_RepairRolloutRule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timed_promote_release_rule: std::option::Option<std::vec::Vec<ClouddeployAutomation_Rules_TimedPromoteReleaseRule>>,
    dynamic: ClouddeployAutomation_RulesDynamic,
}

impl ClouddeployAutomation_Rules {
    #[doc = "Set the field `advance_rollout_rule`.\n"]
    pub fn set_advance_rollout_rule(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ClouddeployAutomation_Rules_AdvanceRolloutRule>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.advance_rollout_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.advance_rollout_rule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `promote_release_rule`.\n"]
    pub fn set_promote_release_rule(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ClouddeployAutomation_Rules_PromoteReleaseRule>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.promote_release_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.promote_release_rule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `repair_rollout_rule`.\n"]
    pub fn set_repair_rollout_rule(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ClouddeployAutomation_Rules_RepairRolloutRule>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.repair_rollout_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.repair_rollout_rule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timed_promote_release_rule`.\n"]
    pub fn set_timed_promote_release_rule(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ClouddeployAutomation_Rules_TimedPromoteReleaseRule>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.timed_promote_release_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.timed_promote_release_rule = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ClouddeployAutomation_Rules {
    type O = terrars::BlockAssignable<ClouddeployAutomation_Rules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployAutomation_Rules {}

impl BuildClouddeployAutomation_Rules {
    pub fn build(self) -> ClouddeployAutomation_Rules {
        ClouddeployAutomation_Rules {
            advance_rollout_rule: core::default::Default::default(),
            promote_release_rule: core::default::Default::default(),
            repair_rollout_rule: core::default::Default::default(),
            timed_promote_release_rule: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployAutomation_RulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployAutomation_RulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ClouddeployAutomation_RulesRef {
        ClouddeployAutomation_RulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployAutomation_RulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `advance_rollout_rule` after provisioning.\n"]
    pub fn advance_rollout_rule(&self) -> terrars::ListRef<ClouddeployAutomation_Rules_AdvanceRolloutRuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advance_rollout_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `promote_release_rule` after provisioning.\n"]
    pub fn promote_release_rule(&self) -> terrars::ListRef<ClouddeployAutomation_Rules_PromoteReleaseRuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.promote_release_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `repair_rollout_rule` after provisioning.\n"]
    pub fn repair_rollout_rule(&self) -> terrars::ListRef<ClouddeployAutomation_Rules_RepairRolloutRuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.repair_rollout_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `timed_promote_release_rule` after provisioning.\n"]
    pub fn timed_promote_release_rule(&self) -> terrars::ListRef<ClouddeployAutomation_Rules_TimedPromoteReleaseRuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timed_promote_release_rule", self.base))
    }
}

#[derive(Serialize)]
pub struct ClouddeployAutomation_Selector_Targets {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl ClouddeployAutomation_Selector_Targets {
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

impl terrars::ToListMappable for ClouddeployAutomation_Selector_Targets {
    type O = terrars::BlockAssignable<ClouddeployAutomation_Selector_Targets>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployAutomation_Selector_Targets {}

impl BuildClouddeployAutomation_Selector_Targets {
    pub fn build(self) -> ClouddeployAutomation_Selector_Targets {
        ClouddeployAutomation_Selector_Targets {
            id: core::default::Default::default(),
            labels: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployAutomation_Selector_TargetsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployAutomation_Selector_TargetsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ClouddeployAutomation_Selector_TargetsRef {
        ClouddeployAutomation_Selector_TargetsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployAutomation_Selector_TargetsRef {
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
struct ClouddeployAutomation_SelectorDynamic {
    targets: std::option::Option<terrars::DynamicBlock<ClouddeployAutomation_Selector_Targets>>,
}

#[derive(Serialize)]
pub struct ClouddeployAutomation_Selector {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    targets: std::option::Option<std::vec::Vec<ClouddeployAutomation_Selector_Targets>>,
    dynamic: ClouddeployAutomation_SelectorDynamic,
}

impl ClouddeployAutomation_Selector {
    #[doc = "Set the field `targets`.\n"]
    pub fn set_targets(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ClouddeployAutomation_Selector_Targets>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.targets = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.targets = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ClouddeployAutomation_Selector {
    type O = terrars::BlockAssignable<ClouddeployAutomation_Selector>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployAutomation_Selector {}

impl BuildClouddeployAutomation_Selector {
    pub fn build(self) -> ClouddeployAutomation_Selector {
        ClouddeployAutomation_Selector {
            targets: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployAutomation_SelectorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployAutomation_SelectorRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ClouddeployAutomation_SelectorRef {
        ClouddeployAutomation_SelectorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployAutomation_SelectorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `targets` after provisioning.\n"]
    pub fn targets(&self) -> terrars::ListRef<ClouddeployAutomation_Selector_TargetsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.targets", self.base))
    }
}

#[derive(Serialize)]
pub struct ClouddeployAutomation_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ClouddeployAutomation_Timeouts {
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

impl terrars::ToListMappable for ClouddeployAutomation_Timeouts {
    type O = terrars::BlockAssignable<ClouddeployAutomation_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployAutomation_Timeouts {}

impl BuildClouddeployAutomation_Timeouts {
    pub fn build(self) -> ClouddeployAutomation_Timeouts {
        ClouddeployAutomation_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployAutomation_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployAutomation_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ClouddeployAutomation_TimeoutsRef {
        ClouddeployAutomation_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployAutomation_TimeoutsRef {
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
