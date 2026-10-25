#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ClouddeployTargetData_ {
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
    deploy_parameters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
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
    require_approval: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    anthos_cluster: std::option::Option<std::vec::Vec<ClouddeployTarget_AnthosCluster>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    associated_entities: std::option::Option<std::vec::Vec<ClouddeployTarget_AssociatedEntities>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_target: std::option::Option<std::vec::Vec<ClouddeployTarget_CustomTarget>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_configs: std::option::Option<std::vec::Vec<ClouddeployTarget_ExecutionConfigs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gke: std::option::Option<std::vec::Vec<ClouddeployTarget_Gke>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    multi_target: std::option::Option<std::vec::Vec<ClouddeployTarget_MultiTarget>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    run: std::option::Option<std::vec::Vec<ClouddeployTarget_Run>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ClouddeployTarget_Timeouts>,
    dynamic: ClouddeployTargetDynamic,
}

struct ClouddeployTarget_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ClouddeployTargetData_>,
}

#[derive(Clone)]
pub struct ClouddeployTarget(std::rc::Rc<ClouddeployTarget_>);

impl ClouddeployTarget {
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
        "Set the field `annotations`.\nOptional. User annotations. These attributes can only be set and used by the user, and not by Google Cloud Deploy. See https://google.aip.dev/128#annotations for more details such as format and size limitations.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field `effective_annotations` for all of the annotations present on the resource."]
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

    #[doc = "Set the field `deploy_parameters`.\nOptional. The deploy parameters to use for this target."]
    pub fn set_deploy_parameters(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().deploy_parameters = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nOptional. Description of the `Target`. Max length is 255 characters."]
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
        "Set the field `labels`.\nOptional. Labels are attributes that can be set and used by both the user and by Google Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 128 bytes.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field `effective_labels` for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\nThe project for the resource"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `require_approval`.\nOptional. Whether or not the `Target` requires approval."]
    pub fn set_require_approval(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().require_approval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `anthos_cluster`.\n"]
    pub fn set_anthos_cluster(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ClouddeployTarget_AnthosCluster>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().anthos_cluster = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.anthos_cluster = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `associated_entities`.\n"]
    pub fn set_associated_entities(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ClouddeployTarget_AssociatedEntities>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().associated_entities = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.associated_entities = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `custom_target`.\n"]
    pub fn set_custom_target(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ClouddeployTarget_CustomTarget>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().custom_target = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.custom_target = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `execution_configs`.\n"]
    pub fn set_execution_configs(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ClouddeployTarget_ExecutionConfigs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().execution_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.execution_configs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `gke`.\n"]
    pub fn set_gke(self, v: impl std::convert::Into<terrars::BlockAssignable<ClouddeployTarget_Gke>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().gke = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.gke = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `multi_target`.\n"]
    pub fn set_multi_target(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ClouddeployTarget_MultiTarget>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().multi_target = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.multi_target = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `run`.\n"]
    pub fn set_run(self, v: impl std::convert::Into<terrars::BlockAssignable<ClouddeployTarget_Run>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().run = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.run = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ClouddeployTarget_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nOptional. User annotations. These attributes can only be set and used by the user, and not by Google Cloud Deploy. See https://google.aip.dev/128#annotations for more details such as format and size limitations.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field `effective_annotations` for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Time at which the `Target` was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deploy_parameters` after provisioning.\nOptional. The deploy parameters to use for this target."]
    pub fn deploy_parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.deploy_parameters", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOptional. Description of the `Target`. Max length is 255 characters."]
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
        "Get a reference to the value of field `etag` after provisioning.\nOptional. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nOptional. Labels are attributes that can be set and used by both the user and by Google Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 128 bytes.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field `effective_labels` for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the `Target`. Format is `[a-z]([a-z0-9-]{0,61}[a-z0-9])?`."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project for the resource"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `require_approval` after provisioning.\nOptional. Whether or not the `Target` requires approval."]
    pub fn require_approval(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_approval", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `target_id` after provisioning.\nOutput only. Resource id of the `Target`."]
    pub fn target_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nOutput only. Unique identifier of the `Target`."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Most recent time at which the `Target` was updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `anthos_cluster` after provisioning.\n"]
    pub fn anthos_cluster(&self) -> terrars::ListRef<ClouddeployTarget_AnthosClusterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.anthos_cluster", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_target` after provisioning.\n"]
    pub fn custom_target(&self) -> terrars::ListRef<ClouddeployTarget_CustomTargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_configs` after provisioning.\n"]
    pub fn execution_configs(&self) -> terrars::ListRef<ClouddeployTarget_ExecutionConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.execution_configs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gke` after provisioning.\n"]
    pub fn gke(&self) -> terrars::ListRef<ClouddeployTarget_GkeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gke", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `multi_target` after provisioning.\n"]
    pub fn multi_target(&self) -> terrars::ListRef<ClouddeployTarget_MultiTargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.multi_target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `run` after provisioning.\n"]
    pub fn run(&self) -> terrars::ListRef<ClouddeployTarget_RunRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.run", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ClouddeployTarget_TimeoutsRef {
        ClouddeployTarget_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ClouddeployTarget {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ClouddeployTarget { }

impl terrars::ToListMappable for ClouddeployTarget {
    type O = terrars::ListRef<ClouddeployTargetRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ClouddeployTarget_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_clouddeploy_target".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildClouddeployTarget {
    pub tf_id: std::string::String,
    #[doc = "The location for the resource"]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "Name of the `Target`. Format is `[a-z]([a-z0-9-]{0,61}[a-z0-9])?`."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildClouddeployTarget {
    pub fn build(self, stack: &mut terrars::Stack) -> ClouddeployTarget {
        let out = ClouddeployTarget(std::rc::Rc::new(ClouddeployTarget_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ClouddeployTargetData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                annotations: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                deploy_parameters: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                require_approval: core::default::Default::default(),
                anthos_cluster: core::default::Default::default(),
                associated_entities: core::default::Default::default(),
                custom_target: core::default::Default::default(),
                execution_configs: core::default::Default::default(),
                gke: core::default::Default::default(),
                multi_target: core::default::Default::default(),
                run: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ClouddeployTargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployTargetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ClouddeployTargetRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nOptional. User annotations. These attributes can only be set and used by the user, and not by Google Cloud Deploy. See https://google.aip.dev/128#annotations for more details such as format and size limitations.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field `effective_annotations` for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Time at which the `Target` was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deploy_parameters` after provisioning.\nOptional. The deploy parameters to use for this target."]
    pub fn deploy_parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.deploy_parameters", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOptional. Description of the `Target`. Max length is 255 characters."]
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
        "Get a reference to the value of field `etag` after provisioning.\nOptional. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nOptional. Labels are attributes that can be set and used by both the user and by Google Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 128 bytes.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field `effective_labels` for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the `Target`. Format is `[a-z]([a-z0-9-]{0,61}[a-z0-9])?`."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project for the resource"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `require_approval` after provisioning.\nOptional. Whether or not the `Target` requires approval."]
    pub fn require_approval(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_approval", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `target_id` after provisioning.\nOutput only. Resource id of the `Target`."]
    pub fn target_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nOutput only. Unique identifier of the `Target`."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Most recent time at which the `Target` was updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `anthos_cluster` after provisioning.\n"]
    pub fn anthos_cluster(&self) -> terrars::ListRef<ClouddeployTarget_AnthosClusterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.anthos_cluster", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_target` after provisioning.\n"]
    pub fn custom_target(&self) -> terrars::ListRef<ClouddeployTarget_CustomTargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_configs` after provisioning.\n"]
    pub fn execution_configs(&self) -> terrars::ListRef<ClouddeployTarget_ExecutionConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.execution_configs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gke` after provisioning.\n"]
    pub fn gke(&self) -> terrars::ListRef<ClouddeployTarget_GkeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gke", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `multi_target` after provisioning.\n"]
    pub fn multi_target(&self) -> terrars::ListRef<ClouddeployTarget_MultiTargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.multi_target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `run` after provisioning.\n"]
    pub fn run(&self) -> terrars::ListRef<ClouddeployTarget_RunRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.run", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ClouddeployTarget_TimeoutsRef {
        ClouddeployTarget_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ClouddeployTargetDynamic {
    anthos_cluster: std::option::Option<terrars::DynamicBlock<ClouddeployTarget_AnthosCluster>>,
    associated_entities: std::option::Option<terrars::DynamicBlock<ClouddeployTarget_AssociatedEntities>>,
    custom_target: std::option::Option<terrars::DynamicBlock<ClouddeployTarget_CustomTarget>>,
    execution_configs: std::option::Option<terrars::DynamicBlock<ClouddeployTarget_ExecutionConfigs>>,
    gke: std::option::Option<terrars::DynamicBlock<ClouddeployTarget_Gke>>,
    multi_target: std::option::Option<terrars::DynamicBlock<ClouddeployTarget_MultiTarget>>,
    run: std::option::Option<terrars::DynamicBlock<ClouddeployTarget_Run>>,
}

#[derive(Serialize)]
pub struct ClouddeployTarget_AnthosCluster {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    membership: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ClouddeployTarget_AnthosCluster {
    #[doc =
        "Set the field `membership`.\nMembership of the GKE Hub-registered cluster to which to apply the Skaffold configuration. Format is `projects/{project}/locations/{location}/memberships/{membership_name}`."]
    pub fn set_membership(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.membership = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ClouddeployTarget_AnthosCluster {
    type O = terrars::BlockAssignable<ClouddeployTarget_AnthosCluster>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployTarget_AnthosCluster {}

impl BuildClouddeployTarget_AnthosCluster {
    pub fn build(self) -> ClouddeployTarget_AnthosCluster {
        ClouddeployTarget_AnthosCluster { membership: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ClouddeployTarget_AnthosClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployTarget_AnthosClusterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ClouddeployTarget_AnthosClusterRef {
        ClouddeployTarget_AnthosClusterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployTarget_AnthosClusterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `membership` after provisioning.\nMembership of the GKE Hub-registered cluster to which to apply the Skaffold configuration. Format is `projects/{project}/locations/{location}/memberships/{membership_name}`."]
    pub fn membership(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.membership", self.base))
    }
}

#[derive(Serialize)]
pub struct ClouddeployTarget_AssociatedEntities_AnthosClusters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    membership: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ClouddeployTarget_AssociatedEntities_AnthosClusters {
    #[doc =
        "Set the field `membership`.\nOptional. Membership of the GKE Hub-registered cluster to which to apply the Skaffold configuration. Format is `projects/{project}/locations/{location}/memberships/{membership_name}`."]
    pub fn set_membership(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.membership = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ClouddeployTarget_AssociatedEntities_AnthosClusters {
    type O = terrars::BlockAssignable<ClouddeployTarget_AssociatedEntities_AnthosClusters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployTarget_AssociatedEntities_AnthosClusters {}

impl BuildClouddeployTarget_AssociatedEntities_AnthosClusters {
    pub fn build(self) -> ClouddeployTarget_AssociatedEntities_AnthosClusters {
        ClouddeployTarget_AssociatedEntities_AnthosClusters { membership: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ClouddeployTarget_AssociatedEntities_AnthosClustersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployTarget_AssociatedEntities_AnthosClustersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployTarget_AssociatedEntities_AnthosClustersRef {
        ClouddeployTarget_AssociatedEntities_AnthosClustersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployTarget_AssociatedEntities_AnthosClustersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `membership` after provisioning.\nOptional. Membership of the GKE Hub-registered cluster to which to apply the Skaffold configuration. Format is `projects/{project}/locations/{location}/memberships/{membership_name}`."]
    pub fn membership(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.membership", self.base))
    }
}

#[derive(Serialize)]
pub struct ClouddeployTarget_AssociatedEntities_GkeClusters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    internal_ip: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proxy_url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ClouddeployTarget_AssociatedEntities_GkeClusters {
    #[doc =
        "Set the field `cluster`.\nOptional. Information specifying a GKE Cluster. Format is `projects/{project_id}/locations/{location_id}/clusters/{cluster_id}`."]
    pub fn set_cluster(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cluster = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `internal_ip`.\nOptional. If true, `cluster` is accessed using the private IP address of the control plane endpoint. Otherwise, the default IP address of the control plane endpoint is used. The default IP address is the private IP address for clusters with private control-plane endpoints and the public IP address otherwise. Only specify this option when `cluster` is a [private GKE cluster](https://cloud.google.com/kubernetes-engine/docs/concepts/private-cluster-concept)."]
    pub fn set_internal_ip(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.internal_ip = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `proxy_url`.\nOptional. If set, used to configure a [proxy](https://kubernetes.io/docs/concepts/configuration/organize-cluster-access-kubeconfig/#proxy) to the Kubernetes server."]
    pub fn set_proxy_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.proxy_url = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ClouddeployTarget_AssociatedEntities_GkeClusters {
    type O = terrars::BlockAssignable<ClouddeployTarget_AssociatedEntities_GkeClusters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployTarget_AssociatedEntities_GkeClusters {}

impl BuildClouddeployTarget_AssociatedEntities_GkeClusters {
    pub fn build(self) -> ClouddeployTarget_AssociatedEntities_GkeClusters {
        ClouddeployTarget_AssociatedEntities_GkeClusters {
            cluster: core::default::Default::default(),
            internal_ip: core::default::Default::default(),
            proxy_url: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployTarget_AssociatedEntities_GkeClustersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployTarget_AssociatedEntities_GkeClustersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployTarget_AssociatedEntities_GkeClustersRef {
        ClouddeployTarget_AssociatedEntities_GkeClustersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployTarget_AssociatedEntities_GkeClustersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cluster` after provisioning.\nOptional. Information specifying a GKE Cluster. Format is `projects/{project_id}/locations/{location_id}/clusters/{cluster_id}`."]
    pub fn cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.base))
    }

    #[doc =
        "Get a reference to the value of field `internal_ip` after provisioning.\nOptional. If true, `cluster` is accessed using the private IP address of the control plane endpoint. Otherwise, the default IP address of the control plane endpoint is used. The default IP address is the private IP address for clusters with private control-plane endpoints and the public IP address otherwise. Only specify this option when `cluster` is a [private GKE cluster](https://cloud.google.com/kubernetes-engine/docs/concepts/private-cluster-concept)."]
    pub fn internal_ip(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.internal_ip", self.base))
    }

    #[doc =
        "Get a reference to the value of field `proxy_url` after provisioning.\nOptional. If set, used to configure a [proxy](https://kubernetes.io/docs/concepts/configuration/organize-cluster-access-kubeconfig/#proxy) to the Kubernetes server."]
    pub fn proxy_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proxy_url", self.base))
    }
}

#[derive(Serialize, Default)]
struct ClouddeployTarget_AssociatedEntitiesDynamic {
    anthos_clusters: std::option::Option<terrars::DynamicBlock<ClouddeployTarget_AssociatedEntities_AnthosClusters>>,
    gke_clusters: std::option::Option<terrars::DynamicBlock<ClouddeployTarget_AssociatedEntities_GkeClusters>>,
}

#[derive(Serialize)]
pub struct ClouddeployTarget_AssociatedEntities {
    entity_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    anthos_clusters: std::option::Option<std::vec::Vec<ClouddeployTarget_AssociatedEntities_AnthosClusters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gke_clusters: std::option::Option<std::vec::Vec<ClouddeployTarget_AssociatedEntities_GkeClusters>>,
    dynamic: ClouddeployTarget_AssociatedEntitiesDynamic,
}

impl ClouddeployTarget_AssociatedEntities {
    #[doc = "Set the field `anthos_clusters`.\n"]
    pub fn set_anthos_clusters(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ClouddeployTarget_AssociatedEntities_AnthosClusters>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.anthos_clusters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.anthos_clusters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `gke_clusters`.\n"]
    pub fn set_gke_clusters(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ClouddeployTarget_AssociatedEntities_GkeClusters>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.gke_clusters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.gke_clusters = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ClouddeployTarget_AssociatedEntities {
    type O = terrars::BlockAssignable<ClouddeployTarget_AssociatedEntities>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployTarget_AssociatedEntities {
    #[doc = "The name for the key in the map for which this object is mapped to in the API"]
    pub entity_id: terrars::PrimField<std::string::String>,
}

impl BuildClouddeployTarget_AssociatedEntities {
    pub fn build(self) -> ClouddeployTarget_AssociatedEntities {
        ClouddeployTarget_AssociatedEntities {
            entity_id: self.entity_id,
            anthos_clusters: core::default::Default::default(),
            gke_clusters: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployTarget_AssociatedEntitiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployTarget_AssociatedEntitiesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ClouddeployTarget_AssociatedEntitiesRef {
        ClouddeployTarget_AssociatedEntitiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployTarget_AssociatedEntitiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `entity_id` after provisioning.\nThe name for the key in the map for which this object is mapped to in the API"]
    pub fn entity_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entity_id", self.base))
    }

    #[doc = "Get a reference to the value of field `anthos_clusters` after provisioning.\n"]
    pub fn anthos_clusters(&self) -> terrars::ListRef<ClouddeployTarget_AssociatedEntities_AnthosClustersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.anthos_clusters", self.base))
    }

    #[doc = "Get a reference to the value of field `gke_clusters` after provisioning.\n"]
    pub fn gke_clusters(&self) -> terrars::ListRef<ClouddeployTarget_AssociatedEntities_GkeClustersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gke_clusters", self.base))
    }
}

#[derive(Serialize)]
pub struct ClouddeployTarget_CustomTarget {
    custom_target_type: terrars::PrimField<std::string::String>,
}

impl ClouddeployTarget_CustomTarget { }

impl terrars::ToListMappable for ClouddeployTarget_CustomTarget {
    type O = terrars::BlockAssignable<ClouddeployTarget_CustomTarget>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployTarget_CustomTarget {
    #[doc =
        "Required. The name of the CustomTargetType. Format must be `projects/{project}/locations/{location}/customTargetTypes/{custom_target_type}`."]
    pub custom_target_type: terrars::PrimField<std::string::String>,
}

impl BuildClouddeployTarget_CustomTarget {
    pub fn build(self) -> ClouddeployTarget_CustomTarget {
        ClouddeployTarget_CustomTarget { custom_target_type: self.custom_target_type }
    }
}

#[allow(dead_code)]
pub struct ClouddeployTarget_CustomTargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployTarget_CustomTargetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ClouddeployTarget_CustomTargetRef {
        ClouddeployTarget_CustomTargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployTarget_CustomTargetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `custom_target_type` after provisioning.\nRequired. The name of the CustomTargetType. Format must be `projects/{project}/locations/{location}/customTargetTypes/{custom_target_type}`."]
    pub fn custom_target_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_target_type", self.base))
    }
}

#[derive(Serialize)]
pub struct ClouddeployTarget_ExecutionConfigs_DefaultPool {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    artifact_storage: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ClouddeployTarget_ExecutionConfigs_DefaultPool {
    #[doc =
        "Set the field `artifact_storage`.\nOptional. Cloud Storage location where execution outputs should be stored. This can either be a bucket (\"gs://my-bucket\") or a path within a bucket (\"gs://my-bucket/my-dir\"). If unspecified, a default bucket located in the same region will be used."]
    pub fn set_artifact_storage(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.artifact_storage = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `service_account`.\nOptional. Google service account to use for execution. If unspecified, the project execution service account (-compute@developer.gserviceaccount.com) will be used."]
    pub fn set_service_account(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_account = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ClouddeployTarget_ExecutionConfigs_DefaultPool {
    type O = terrars::BlockAssignable<ClouddeployTarget_ExecutionConfigs_DefaultPool>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployTarget_ExecutionConfigs_DefaultPool {}

impl BuildClouddeployTarget_ExecutionConfigs_DefaultPool {
    pub fn build(self) -> ClouddeployTarget_ExecutionConfigs_DefaultPool {
        ClouddeployTarget_ExecutionConfigs_DefaultPool {
            artifact_storage: core::default::Default::default(),
            service_account: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployTarget_ExecutionConfigs_DefaultPoolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployTarget_ExecutionConfigs_DefaultPoolRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployTarget_ExecutionConfigs_DefaultPoolRef {
        ClouddeployTarget_ExecutionConfigs_DefaultPoolRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployTarget_ExecutionConfigs_DefaultPoolRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `artifact_storage` after provisioning.\nOptional. Cloud Storage location where execution outputs should be stored. This can either be a bucket (\"gs://my-bucket\") or a path within a bucket (\"gs://my-bucket/my-dir\"). If unspecified, a default bucket located in the same region will be used."]
    pub fn artifact_storage(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.artifact_storage", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nOptional. Google service account to use for execution. If unspecified, the project execution service account (-compute@developer.gserviceaccount.com) will be used."]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.base))
    }
}

#[derive(Serialize)]
pub struct ClouddeployTarget_ExecutionConfigs_PrivatePool {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    artifact_storage: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account: std::option::Option<terrars::PrimField<std::string::String>>,
    worker_pool: terrars::PrimField<std::string::String>,
}

impl ClouddeployTarget_ExecutionConfigs_PrivatePool {
    #[doc =
        "Set the field `artifact_storage`.\nOptional. Cloud Storage location where execution outputs should be stored. This can either be a bucket (\"gs://my-bucket\") or a path within a bucket (\"gs://my-bucket/my-dir\"). If unspecified, a default bucket located in the same region will be used."]
    pub fn set_artifact_storage(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.artifact_storage = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `service_account`.\nOptional. Google service account to use for execution. If unspecified, the project execution service account (-compute@developer.gserviceaccount.com) will be used."]
    pub fn set_service_account(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_account = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ClouddeployTarget_ExecutionConfigs_PrivatePool {
    type O = terrars::BlockAssignable<ClouddeployTarget_ExecutionConfigs_PrivatePool>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployTarget_ExecutionConfigs_PrivatePool {
    #[doc =
        "Required. Resource name of the Cloud Build worker pool to use. The format is `projects/{project}/locations/{location}/workerPools/{pool}`."]
    pub worker_pool: terrars::PrimField<std::string::String>,
}

impl BuildClouddeployTarget_ExecutionConfigs_PrivatePool {
    pub fn build(self) -> ClouddeployTarget_ExecutionConfigs_PrivatePool {
        ClouddeployTarget_ExecutionConfigs_PrivatePool {
            artifact_storage: core::default::Default::default(),
            service_account: core::default::Default::default(),
            worker_pool: self.worker_pool,
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployTarget_ExecutionConfigs_PrivatePoolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployTarget_ExecutionConfigs_PrivatePoolRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployTarget_ExecutionConfigs_PrivatePoolRef {
        ClouddeployTarget_ExecutionConfigs_PrivatePoolRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployTarget_ExecutionConfigs_PrivatePoolRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `artifact_storage` after provisioning.\nOptional. Cloud Storage location where execution outputs should be stored. This can either be a bucket (\"gs://my-bucket\") or a path within a bucket (\"gs://my-bucket/my-dir\"). If unspecified, a default bucket located in the same region will be used."]
    pub fn artifact_storage(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.artifact_storage", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nOptional. Google service account to use for execution. If unspecified, the project execution service account (-compute@developer.gserviceaccount.com) will be used."]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.base))
    }

    #[doc =
        "Get a reference to the value of field `worker_pool` after provisioning.\nRequired. Resource name of the Cloud Build worker pool to use. The format is `projects/{project}/locations/{location}/workerPools/{pool}`."]
    pub fn worker_pool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.worker_pool", self.base))
    }
}

#[derive(Serialize, Default)]
struct ClouddeployTarget_ExecutionConfigsDynamic {
    default_pool: std::option::Option<terrars::DynamicBlock<ClouddeployTarget_ExecutionConfigs_DefaultPool>>,
    private_pool: std::option::Option<terrars::DynamicBlock<ClouddeployTarget_ExecutionConfigs_PrivatePool>>,
}

#[derive(Serialize)]
pub struct ClouddeployTarget_ExecutionConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    artifact_storage: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_timeout: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account: std::option::Option<terrars::PrimField<std::string::String>>,
    usages: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    verbose: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    worker_pool: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_pool: std::option::Option<std::vec::Vec<ClouddeployTarget_ExecutionConfigs_DefaultPool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_pool: std::option::Option<std::vec::Vec<ClouddeployTarget_ExecutionConfigs_PrivatePool>>,
    dynamic: ClouddeployTarget_ExecutionConfigsDynamic,
}

impl ClouddeployTarget_ExecutionConfigs {
    #[doc =
        "Set the field `artifact_storage`.\nOptional. Cloud Storage location in which to store execution outputs. This can either be a bucket (\"gs://my-bucket\") or a path within a bucket (\"gs://my-bucket/my-dir\"). If unspecified, a default bucket located in the same region will be used."]
    pub fn set_artifact_storage(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.artifact_storage = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `execution_timeout`.\nOptional. Execution timeout for a Cloud Build Execution. This must be between 10m and 24h in seconds format. If unspecified, a default timeout of 1h is used."]
    pub fn set_execution_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.execution_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `service_account`.\nOptional. Google service account to use for execution. If unspecified, the project execution service account (-compute@developer.gserviceaccount.com) is used."]
    pub fn set_service_account(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_account = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `verbose`.\nOptional. If true, additional logging will be enabled when running builds in this execution environment."]
    pub fn set_verbose(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.verbose = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `worker_pool`.\nOptional. The resource name of the `WorkerPool`, with the format `projects/{project}/locations/{location}/workerPools/{worker_pool}`. If this optional field is unspecified, the default Cloud Build pool will be used."]
    pub fn set_worker_pool(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.worker_pool = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_pool`.\n"]
    pub fn set_default_pool(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ClouddeployTarget_ExecutionConfigs_DefaultPool>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.default_pool = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.default_pool = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `private_pool`.\n"]
    pub fn set_private_pool(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ClouddeployTarget_ExecutionConfigs_PrivatePool>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.private_pool = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.private_pool = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ClouddeployTarget_ExecutionConfigs {
    type O = terrars::BlockAssignable<ClouddeployTarget_ExecutionConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployTarget_ExecutionConfigs {
    #[doc = "Required. Usages when this configuration should be applied."]
    pub usages: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildClouddeployTarget_ExecutionConfigs {
    pub fn build(self) -> ClouddeployTarget_ExecutionConfigs {
        ClouddeployTarget_ExecutionConfigs {
            artifact_storage: core::default::Default::default(),
            execution_timeout: core::default::Default::default(),
            service_account: core::default::Default::default(),
            usages: self.usages,
            verbose: core::default::Default::default(),
            worker_pool: core::default::Default::default(),
            default_pool: core::default::Default::default(),
            private_pool: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployTarget_ExecutionConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployTarget_ExecutionConfigsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ClouddeployTarget_ExecutionConfigsRef {
        ClouddeployTarget_ExecutionConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployTarget_ExecutionConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `artifact_storage` after provisioning.\nOptional. Cloud Storage location in which to store execution outputs. This can either be a bucket (\"gs://my-bucket\") or a path within a bucket (\"gs://my-bucket/my-dir\"). If unspecified, a default bucket located in the same region will be used."]
    pub fn artifact_storage(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.artifact_storage", self.base))
    }

    #[doc =
        "Get a reference to the value of field `execution_timeout` after provisioning.\nOptional. Execution timeout for a Cloud Build Execution. This must be between 10m and 24h in seconds format. If unspecified, a default timeout of 1h is used."]
    pub fn execution_timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_timeout", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nOptional. Google service account to use for execution. If unspecified, the project execution service account (-compute@developer.gserviceaccount.com) is used."]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.base))
    }

    #[doc =
        "Get a reference to the value of field `usages` after provisioning.\nRequired. Usages when this configuration should be applied."]
    pub fn usages(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.usages", self.base))
    }

    #[doc =
        "Get a reference to the value of field `verbose` after provisioning.\nOptional. If true, additional logging will be enabled when running builds in this execution environment."]
    pub fn verbose(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.verbose", self.base))
    }

    #[doc =
        "Get a reference to the value of field `worker_pool` after provisioning.\nOptional. The resource name of the `WorkerPool`, with the format `projects/{project}/locations/{location}/workerPools/{worker_pool}`. If this optional field is unspecified, the default Cloud Build pool will be used."]
    pub fn worker_pool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.worker_pool", self.base))
    }

    #[doc = "Get a reference to the value of field `default_pool` after provisioning.\n"]
    pub fn default_pool(&self) -> terrars::ListRef<ClouddeployTarget_ExecutionConfigs_DefaultPoolRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_pool", self.base))
    }

    #[doc = "Get a reference to the value of field `private_pool` after provisioning.\n"]
    pub fn private_pool(&self) -> terrars::ListRef<ClouddeployTarget_ExecutionConfigs_PrivatePoolRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_pool", self.base))
    }
}

#[derive(Serialize)]
pub struct ClouddeployTarget_Gke {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_endpoint: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    internal_ip: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proxy_url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ClouddeployTarget_Gke {
    #[doc =
        "Set the field `cluster`.\nInformation specifying a GKE Cluster. Format is `projects/{project_id}/locations/{location_id}/clusters/{cluster_id}."]
    pub fn set_cluster(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cluster = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `dns_endpoint`.\nOptional. If set, the cluster will be accessed using the DNS endpoint. Note that both `dns_endpoint` and `internal_ip` cannot be set to true."]
    pub fn set_dns_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.dns_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `internal_ip`.\nOptional. If true, `cluster` is accessed using the private IP address of the control plane endpoint. Otherwise, the default IP address of the control plane endpoint is used. The default IP address is the private IP address for clusters with private control-plane endpoints and the public IP address otherwise. Only specify this option when `cluster` is a [private GKE cluster](https://cloud.google.com/kubernetes-engine/docs/concepts/private-cluster-concept)."]
    pub fn set_internal_ip(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.internal_ip = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `proxy_url`.\nOptional. If set, used to configure a [proxy](https://kubernetes.io/docs/concepts/configuration/organize-cluster-access-kubeconfig/#proxy) to the Kubernetes server."]
    pub fn set_proxy_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.proxy_url = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ClouddeployTarget_Gke {
    type O = terrars::BlockAssignable<ClouddeployTarget_Gke>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployTarget_Gke {}

impl BuildClouddeployTarget_Gke {
    pub fn build(self) -> ClouddeployTarget_Gke {
        ClouddeployTarget_Gke {
            cluster: core::default::Default::default(),
            dns_endpoint: core::default::Default::default(),
            internal_ip: core::default::Default::default(),
            proxy_url: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployTarget_GkeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployTarget_GkeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ClouddeployTarget_GkeRef {
        ClouddeployTarget_GkeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployTarget_GkeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cluster` after provisioning.\nInformation specifying a GKE Cluster. Format is `projects/{project_id}/locations/{location_id}/clusters/{cluster_id}."]
    pub fn cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.base))
    }

    #[doc =
        "Get a reference to the value of field `dns_endpoint` after provisioning.\nOptional. If set, the cluster will be accessed using the DNS endpoint. Note that both `dns_endpoint` and `internal_ip` cannot be set to true."]
    pub fn dns_endpoint(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_endpoint", self.base))
    }

    #[doc =
        "Get a reference to the value of field `internal_ip` after provisioning.\nOptional. If true, `cluster` is accessed using the private IP address of the control plane endpoint. Otherwise, the default IP address of the control plane endpoint is used. The default IP address is the private IP address for clusters with private control-plane endpoints and the public IP address otherwise. Only specify this option when `cluster` is a [private GKE cluster](https://cloud.google.com/kubernetes-engine/docs/concepts/private-cluster-concept)."]
    pub fn internal_ip(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.internal_ip", self.base))
    }

    #[doc =
        "Get a reference to the value of field `proxy_url` after provisioning.\nOptional. If set, used to configure a [proxy](https://kubernetes.io/docs/concepts/configuration/organize-cluster-access-kubeconfig/#proxy) to the Kubernetes server."]
    pub fn proxy_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proxy_url", self.base))
    }
}

#[derive(Serialize)]
pub struct ClouddeployTarget_MultiTarget {
    target_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl ClouddeployTarget_MultiTarget { }

impl terrars::ToListMappable for ClouddeployTarget_MultiTarget {
    type O = terrars::BlockAssignable<ClouddeployTarget_MultiTarget>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployTarget_MultiTarget {
    #[doc = "Required. The target_ids of this multiTarget."]
    pub target_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildClouddeployTarget_MultiTarget {
    pub fn build(self) -> ClouddeployTarget_MultiTarget {
        ClouddeployTarget_MultiTarget { target_ids: self.target_ids }
    }
}

#[allow(dead_code)]
pub struct ClouddeployTarget_MultiTargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployTarget_MultiTargetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ClouddeployTarget_MultiTargetRef {
        ClouddeployTarget_MultiTargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployTarget_MultiTargetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `target_ids` after provisioning.\nRequired. The target_ids of this multiTarget."]
    pub fn target_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_ids", self.base))
    }
}

#[derive(Serialize)]
pub struct ClouddeployTarget_Run {
    location: terrars::PrimField<std::string::String>,
}

impl ClouddeployTarget_Run { }

impl terrars::ToListMappable for ClouddeployTarget_Run {
    type O = terrars::BlockAssignable<ClouddeployTarget_Run>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployTarget_Run {
    #[doc =
        "Required. The location where the Cloud Run Service should be located. Format is `projects/{project}/locations/{location}`."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildClouddeployTarget_Run {
    pub fn build(self) -> ClouddeployTarget_Run {
        ClouddeployTarget_Run { location: self.location }
    }
}

#[allow(dead_code)]
pub struct ClouddeployTarget_RunRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployTarget_RunRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ClouddeployTarget_RunRef {
        ClouddeployTarget_RunRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployTarget_RunRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nRequired. The location where the Cloud Run Service should be located. Format is `projects/{project}/locations/{location}`."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
}

#[derive(Serialize)]
pub struct ClouddeployTarget_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ClouddeployTarget_Timeouts {
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

impl terrars::ToListMappable for ClouddeployTarget_Timeouts {
    type O = terrars::BlockAssignable<ClouddeployTarget_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployTarget_Timeouts {}

impl BuildClouddeployTarget_Timeouts {
    pub fn build(self) -> ClouddeployTarget_Timeouts {
        ClouddeployTarget_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployTarget_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployTarget_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ClouddeployTarget_TimeoutsRef {
        ClouddeployTarget_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployTarget_TimeoutsRef {
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
