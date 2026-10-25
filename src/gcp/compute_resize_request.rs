#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeResizeRequestData_ {
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
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    instance_group_manager: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    resize_by: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    requested_run_duration: std::option::Option<std::vec::Vec<ComputeResizeRequest_RequestedRunDuration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ComputeResizeRequest_Timeouts>,
    dynamic: ComputeResizeRequestDynamic,
}

struct ComputeResizeRequest_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ComputeResizeRequestData_>,
}

#[derive(Clone)]
pub struct ComputeResizeRequest(std::rc::Rc<ComputeResizeRequest_>);

impl ComputeResizeRequest {
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

    #[doc = "Set the field `description`.\nAn optional description of this resize-request."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
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

    #[doc =
        "Set the field `zone`.\nThe reference of the compute zone scoping this request. If it is not provided, the provider zone is used."]
    pub fn set_zone(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `requested_run_duration`.\n"]
    pub fn set_requested_run_duration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeResizeRequest_RequestedRunDuration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().requested_run_duration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.requested_run_duration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ComputeResizeRequest_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `creation_timestamp` after provisioning.\nThe creation timestamp for this resize request in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resize-request."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance_group_manager` after provisioning.\nThe reference of the instance group manager this ResizeRequest is a part of."]
    pub fn instance_group_manager(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_group_manager", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of this resize request. The name must be 1-63 characters long, and comply with RFC1035."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resize_by` after provisioning.\nThe number of instances to be created by this resize request. The group's target size will be increased by this number."]
    pub fn resize_by(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resize_by", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nCurrent state of the request."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nStatus of the request."]
    pub fn status(&self) -> terrars::ListRef<ComputeResizeRequest_StatusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone` after provisioning.\nThe reference of the compute zone scoping this request. If it is not provided, the provider zone is used."]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `requested_run_duration` after provisioning.\n"]
    pub fn requested_run_duration(&self) -> terrars::ListRef<ComputeResizeRequest_RequestedRunDurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.requested_run_duration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeResizeRequest_TimeoutsRef {
        ComputeResizeRequest_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ComputeResizeRequest {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ComputeResizeRequest { }

impl terrars::ToListMappable for ComputeResizeRequest {
    type O = terrars::ListRef<ComputeResizeRequestRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ComputeResizeRequest_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_compute_resize_request".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputeResizeRequest {
    pub tf_id: std::string::String,
    #[doc = "The reference of the instance group manager this ResizeRequest is a part of."]
    pub instance_group_manager: terrars::PrimField<std::string::String>,
    #[doc = "The name of this resize request. The name must be 1-63 characters long, and comply with RFC1035."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc =
        "The number of instances to be created by this resize request. The group's target size will be increased by this number."]
    pub resize_by: terrars::PrimField<f64>,
}

impl BuildComputeResizeRequest {
    pub fn build(self, stack: &mut terrars::Stack) -> ComputeResizeRequest {
        let out = ComputeResizeRequest(std::rc::Rc::new(ComputeResizeRequest_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ComputeResizeRequestData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                instance_group_manager: self.instance_group_manager,
                name: self.name,
                project: core::default::Default::default(),
                resize_by: self.resize_by,
                zone: core::default::Default::default(),
                requested_run_duration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComputeResizeRequestRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeResizeRequestRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComputeResizeRequestRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `creation_timestamp` after provisioning.\nThe creation timestamp for this resize request in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resize-request."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance_group_manager` after provisioning.\nThe reference of the instance group manager this ResizeRequest is a part of."]
    pub fn instance_group_manager(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_group_manager", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of this resize request. The name must be 1-63 characters long, and comply with RFC1035."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resize_by` after provisioning.\nThe number of instances to be created by this resize request. The group's target size will be increased by this number."]
    pub fn resize_by(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resize_by", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nCurrent state of the request."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nStatus of the request."]
    pub fn status(&self) -> terrars::ListRef<ComputeResizeRequest_StatusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone` after provisioning.\nThe reference of the compute zone scoping this request. If it is not provided, the provider zone is used."]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `requested_run_duration` after provisioning.\n"]
    pub fn requested_run_duration(&self) -> terrars::ListRef<ComputeResizeRequest_RequestedRunDurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.requested_run_duration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeResizeRequest_TimeoutsRef {
        ComputeResizeRequest_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ComputeResizeRequestDynamic {
    requested_run_duration: std::option::Option<terrars::DynamicBlock<ComputeResizeRequest_RequestedRunDuration>>,
}

#[derive(Serialize)]
pub struct ComputeResizeRequest_Status_Error_Errors_ErrorDetails_ErrorInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadatas: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reason: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeResizeRequest_Status_Error_Errors_ErrorDetails_ErrorInfo {
    #[doc = "Set the field `domain`.\n"]
    pub fn set_domain(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.domain = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metadatas`.\n"]
    pub fn set_metadatas(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.metadatas = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `reason`.\n"]
    pub fn set_reason(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.reason = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeResizeRequest_Status_Error_Errors_ErrorDetails_ErrorInfo {
    type O = terrars::BlockAssignable<ComputeResizeRequest_Status_Error_Errors_ErrorDetails_ErrorInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeResizeRequest_Status_Error_Errors_ErrorDetails_ErrorInfo {}

impl BuildComputeResizeRequest_Status_Error_Errors_ErrorDetails_ErrorInfo {
    pub fn build(self) -> ComputeResizeRequest_Status_Error_Errors_ErrorDetails_ErrorInfo {
        ComputeResizeRequest_Status_Error_Errors_ErrorDetails_ErrorInfo {
            domain: core::default::Default::default(),
            metadatas: core::default::Default::default(),
            reason: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeResizeRequest_Status_Error_Errors_ErrorDetails_ErrorInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeResizeRequest_Status_Error_Errors_ErrorDetails_ErrorInfoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeResizeRequest_Status_Error_Errors_ErrorDetails_ErrorInfoRef {
        ComputeResizeRequest_Status_Error_Errors_ErrorDetails_ErrorInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeResizeRequest_Status_Error_Errors_ErrorDetails_ErrorInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }

    #[doc = "Get a reference to the value of field `metadatas` after provisioning.\n"]
    pub fn metadatas(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.metadatas", self.base))
    }

    #[doc = "Get a reference to the value of field `reason` after provisioning.\n"]
    pub fn reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reason", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeResizeRequest_Status_Error_Errors_ErrorDetails_Help_Links {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeResizeRequest_Status_Error_Errors_ErrorDetails_Help_Links {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `url`.\n"]
    pub fn set_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.url = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeResizeRequest_Status_Error_Errors_ErrorDetails_Help_Links {
    type O = terrars::BlockAssignable<ComputeResizeRequest_Status_Error_Errors_ErrorDetails_Help_Links>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeResizeRequest_Status_Error_Errors_ErrorDetails_Help_Links {}

impl BuildComputeResizeRequest_Status_Error_Errors_ErrorDetails_Help_Links {
    pub fn build(self) -> ComputeResizeRequest_Status_Error_Errors_ErrorDetails_Help_Links {
        ComputeResizeRequest_Status_Error_Errors_ErrorDetails_Help_Links {
            description: core::default::Default::default(),
            url: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeResizeRequest_Status_Error_Errors_ErrorDetails_Help_LinksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeResizeRequest_Status_Error_Errors_ErrorDetails_Help_LinksRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeResizeRequest_Status_Error_Errors_ErrorDetails_Help_LinksRef {
        ComputeResizeRequest_Status_Error_Errors_ErrorDetails_Help_LinksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeResizeRequest_Status_Error_Errors_ErrorDetails_Help_LinksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeResizeRequest_Status_Error_Errors_ErrorDetails_Help {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    links: std::option::Option<terrars::ListField<ComputeResizeRequest_Status_Error_Errors_ErrorDetails_Help_Links>>,
}

impl ComputeResizeRequest_Status_Error_Errors_ErrorDetails_Help {
    #[doc = "Set the field `links`.\n"]
    pub fn set_links(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<ComputeResizeRequest_Status_Error_Errors_ErrorDetails_Help_Links>>,
    ) -> Self {
        self.links = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeResizeRequest_Status_Error_Errors_ErrorDetails_Help {
    type O = terrars::BlockAssignable<ComputeResizeRequest_Status_Error_Errors_ErrorDetails_Help>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeResizeRequest_Status_Error_Errors_ErrorDetails_Help {}

impl BuildComputeResizeRequest_Status_Error_Errors_ErrorDetails_Help {
    pub fn build(self) -> ComputeResizeRequest_Status_Error_Errors_ErrorDetails_Help {
        ComputeResizeRequest_Status_Error_Errors_ErrorDetails_Help { links: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ComputeResizeRequest_Status_Error_Errors_ErrorDetails_HelpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeResizeRequest_Status_Error_Errors_ErrorDetails_HelpRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeResizeRequest_Status_Error_Errors_ErrorDetails_HelpRef {
        ComputeResizeRequest_Status_Error_Errors_ErrorDetails_HelpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeResizeRequest_Status_Error_Errors_ErrorDetails_HelpRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `links` after provisioning.\n"]
    pub fn links(&self) -> terrars::ListRef<ComputeResizeRequest_Status_Error_Errors_ErrorDetails_Help_LinksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.links", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeResizeRequest_Status_Error_Errors_ErrorDetails_LocalizedMessage {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    locale: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeResizeRequest_Status_Error_Errors_ErrorDetails_LocalizedMessage {
    #[doc = "Set the field `locale`.\n"]
    pub fn set_locale(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.locale = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeResizeRequest_Status_Error_Errors_ErrorDetails_LocalizedMessage {
    type O = terrars::BlockAssignable<ComputeResizeRequest_Status_Error_Errors_ErrorDetails_LocalizedMessage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeResizeRequest_Status_Error_Errors_ErrorDetails_LocalizedMessage {}

impl BuildComputeResizeRequest_Status_Error_Errors_ErrorDetails_LocalizedMessage {
    pub fn build(self) -> ComputeResizeRequest_Status_Error_Errors_ErrorDetails_LocalizedMessage {
        ComputeResizeRequest_Status_Error_Errors_ErrorDetails_LocalizedMessage {
            locale: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeResizeRequest_Status_Error_Errors_ErrorDetails_LocalizedMessageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeResizeRequest_Status_Error_Errors_ErrorDetails_LocalizedMessageRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeResizeRequest_Status_Error_Errors_ErrorDetails_LocalizedMessageRef {
        ComputeResizeRequest_Status_Error_Errors_ErrorDetails_LocalizedMessageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeResizeRequest_Status_Error_Errors_ErrorDetails_LocalizedMessageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `locale` after provisioning.\n"]
    pub fn locale(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.locale", self.base))
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeResizeRequest_Status_Error_Errors_ErrorDetails_QuotaInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimensions: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    future_limit: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    limit: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    limit_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rollout_status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeResizeRequest_Status_Error_Errors_ErrorDetails_QuotaInfo {
    #[doc = "Set the field `dimensions`.\n"]
    pub fn set_dimensions(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.dimensions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `future_limit`.\n"]
    pub fn set_future_limit(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.future_limit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `limit`.\n"]
    pub fn set_limit(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.limit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `limit_name`.\n"]
    pub fn set_limit_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.limit_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metric_name`.\n"]
    pub fn set_metric_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.metric_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rollout_status`.\n"]
    pub fn set_rollout_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rollout_status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeResizeRequest_Status_Error_Errors_ErrorDetails_QuotaInfo {
    type O = terrars::BlockAssignable<ComputeResizeRequest_Status_Error_Errors_ErrorDetails_QuotaInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeResizeRequest_Status_Error_Errors_ErrorDetails_QuotaInfo {}

impl BuildComputeResizeRequest_Status_Error_Errors_ErrorDetails_QuotaInfo {
    pub fn build(self) -> ComputeResizeRequest_Status_Error_Errors_ErrorDetails_QuotaInfo {
        ComputeResizeRequest_Status_Error_Errors_ErrorDetails_QuotaInfo {
            dimensions: core::default::Default::default(),
            future_limit: core::default::Default::default(),
            limit: core::default::Default::default(),
            limit_name: core::default::Default::default(),
            metric_name: core::default::Default::default(),
            rollout_status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeResizeRequest_Status_Error_Errors_ErrorDetails_QuotaInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeResizeRequest_Status_Error_Errors_ErrorDetails_QuotaInfoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeResizeRequest_Status_Error_Errors_ErrorDetails_QuotaInfoRef {
        ComputeResizeRequest_Status_Error_Errors_ErrorDetails_QuotaInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeResizeRequest_Status_Error_Errors_ErrorDetails_QuotaInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dimensions` after provisioning.\n"]
    pub fn dimensions(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.dimensions", self.base))
    }

    #[doc = "Get a reference to the value of field `future_limit` after provisioning.\n"]
    pub fn future_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.future_limit", self.base))
    }

    #[doc = "Get a reference to the value of field `limit` after provisioning.\n"]
    pub fn limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.limit", self.base))
    }

    #[doc = "Get a reference to the value of field `limit_name` after provisioning.\n"]
    pub fn limit_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.limit_name", self.base))
    }

    #[doc = "Get a reference to the value of field `metric_name` after provisioning.\n"]
    pub fn metric_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metric_name", self.base))
    }

    #[doc = "Get a reference to the value of field `rollout_status` after provisioning.\n"]
    pub fn rollout_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rollout_status", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeResizeRequest_Status_Error_Errors_ErrorDetails {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_info: std::option::Option<terrars::ListField<ComputeResizeRequest_Status_Error_Errors_ErrorDetails_ErrorInfo>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    help: std::option::Option<terrars::ListField<ComputeResizeRequest_Status_Error_Errors_ErrorDetails_Help>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    localized_message: std
    ::option
    ::Option<terrars::ListField<ComputeResizeRequest_Status_Error_Errors_ErrorDetails_LocalizedMessage>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    quota_info: std::option::Option<terrars::ListField<ComputeResizeRequest_Status_Error_Errors_ErrorDetails_QuotaInfo>>,
}

impl ComputeResizeRequest_Status_Error_Errors_ErrorDetails {
    #[doc = "Set the field `error_info`.\n"]
    pub fn set_error_info(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<ComputeResizeRequest_Status_Error_Errors_ErrorDetails_ErrorInfo>>,
    ) -> Self {
        self.error_info = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `help`.\n"]
    pub fn set_help(
        mut self,
        v: impl std::convert::Into<terrars::ListField<ComputeResizeRequest_Status_Error_Errors_ErrorDetails_Help>>,
    ) -> Self {
        self.help = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `localized_message`.\n"]
    pub fn set_localized_message(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<ComputeResizeRequest_Status_Error_Errors_ErrorDetails_LocalizedMessage>,
                    >,
    ) -> Self {
        self.localized_message = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `quota_info`.\n"]
    pub fn set_quota_info(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<ComputeResizeRequest_Status_Error_Errors_ErrorDetails_QuotaInfo>>,
    ) -> Self {
        self.quota_info = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeResizeRequest_Status_Error_Errors_ErrorDetails {
    type O = terrars::BlockAssignable<ComputeResizeRequest_Status_Error_Errors_ErrorDetails>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeResizeRequest_Status_Error_Errors_ErrorDetails {}

impl BuildComputeResizeRequest_Status_Error_Errors_ErrorDetails {
    pub fn build(self) -> ComputeResizeRequest_Status_Error_Errors_ErrorDetails {
        ComputeResizeRequest_Status_Error_Errors_ErrorDetails {
            error_info: core::default::Default::default(),
            help: core::default::Default::default(),
            localized_message: core::default::Default::default(),
            quota_info: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeResizeRequest_Status_Error_Errors_ErrorDetailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeResizeRequest_Status_Error_Errors_ErrorDetailsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeResizeRequest_Status_Error_Errors_ErrorDetailsRef {
        ComputeResizeRequest_Status_Error_Errors_ErrorDetailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeResizeRequest_Status_Error_Errors_ErrorDetailsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `error_info` after provisioning.\n"]
    pub fn error_info(&self) -> terrars::ListRef<ComputeResizeRequest_Status_Error_Errors_ErrorDetails_ErrorInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.error_info", self.base))
    }

    #[doc = "Get a reference to the value of field `help` after provisioning.\n"]
    pub fn help(&self) -> terrars::ListRef<ComputeResizeRequest_Status_Error_Errors_ErrorDetails_HelpRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.help", self.base))
    }

    #[doc = "Get a reference to the value of field `localized_message` after provisioning.\n"]
    pub fn localized_message(
        &self,
    ) -> terrars::ListRef<ComputeResizeRequest_Status_Error_Errors_ErrorDetails_LocalizedMessageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.localized_message", self.base))
    }

    #[doc = "Get a reference to the value of field `quota_info` after provisioning.\n"]
    pub fn quota_info(&self) -> terrars::ListRef<ComputeResizeRequest_Status_Error_Errors_ErrorDetails_QuotaInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.quota_info", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeResizeRequest_Status_Error_Errors {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_details: std::option::Option<terrars::ListField<ComputeResizeRequest_Status_Error_Errors_ErrorDetails>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeResizeRequest_Status_Error_Errors {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `error_details`.\n"]
    pub fn set_error_details(
        mut self,
        v: impl std::convert::Into<terrars::ListField<ComputeResizeRequest_Status_Error_Errors_ErrorDetails>>,
    ) -> Self {
        self.error_details = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeResizeRequest_Status_Error_Errors {
    type O = terrars::BlockAssignable<ComputeResizeRequest_Status_Error_Errors>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeResizeRequest_Status_Error_Errors {}

impl BuildComputeResizeRequest_Status_Error_Errors {
    pub fn build(self) -> ComputeResizeRequest_Status_Error_Errors {
        ComputeResizeRequest_Status_Error_Errors {
            code: core::default::Default::default(),
            error_details: core::default::Default::default(),
            location: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeResizeRequest_Status_Error_ErrorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeResizeRequest_Status_Error_ErrorsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeResizeRequest_Status_Error_ErrorsRef {
        ComputeResizeRequest_Status_Error_ErrorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeResizeRequest_Status_Error_ErrorsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }

    #[doc = "Get a reference to the value of field `error_details` after provisioning.\n"]
    pub fn error_details(&self) -> terrars::ListRef<ComputeResizeRequest_Status_Error_Errors_ErrorDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.error_details", self.base))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeResizeRequest_Status_Error {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    errors: std::option::Option<terrars::ListField<ComputeResizeRequest_Status_Error_Errors>>,
}

impl ComputeResizeRequest_Status_Error {
    #[doc = "Set the field `errors`.\n"]
    pub fn set_errors(
        mut self,
        v: impl std::convert::Into<terrars::ListField<ComputeResizeRequest_Status_Error_Errors>>,
    ) -> Self {
        self.errors = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeResizeRequest_Status_Error {
    type O = terrars::BlockAssignable<ComputeResizeRequest_Status_Error>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeResizeRequest_Status_Error {}

impl BuildComputeResizeRequest_Status_Error {
    pub fn build(self) -> ComputeResizeRequest_Status_Error {
        ComputeResizeRequest_Status_Error { errors: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ComputeResizeRequest_Status_ErrorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeResizeRequest_Status_ErrorRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeResizeRequest_Status_ErrorRef {
        ComputeResizeRequest_Status_ErrorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeResizeRequest_Status_ErrorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `errors` after provisioning.\n"]
    pub fn errors(&self) -> terrars::ListRef<ComputeResizeRequest_Status_Error_ErrorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.errors", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeResizeRequest_Status_LastAttempt {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error: std::option::Option<terrars::ListField<ComputeResizeRequest_Status_Error>>,
}

impl ComputeResizeRequest_Status_LastAttempt {
    #[doc = "Set the field `error`.\n"]
    pub fn set_error(
        mut self,
        v: impl std::convert::Into<terrars::ListField<ComputeResizeRequest_Status_Error>>,
    ) -> Self {
        self.error = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeResizeRequest_Status_LastAttempt {
    type O = terrars::BlockAssignable<ComputeResizeRequest_Status_LastAttempt>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeResizeRequest_Status_LastAttempt {}

impl BuildComputeResizeRequest_Status_LastAttempt {
    pub fn build(self) -> ComputeResizeRequest_Status_LastAttempt {
        ComputeResizeRequest_Status_LastAttempt { error: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ComputeResizeRequest_Status_LastAttemptRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeResizeRequest_Status_LastAttemptRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeResizeRequest_Status_LastAttemptRef {
        ComputeResizeRequest_Status_LastAttemptRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeResizeRequest_Status_LastAttemptRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `error` after provisioning.\n"]
    pub fn error(&self) -> terrars::ListRef<ComputeResizeRequest_Status_ErrorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.error", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeResizeRequest_Status {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error: std::option::Option<terrars::ListField<ComputeResizeRequest_Status_Error>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_attempt: std::option::Option<terrars::ListField<ComputeResizeRequest_Status_LastAttempt>>,
}

impl ComputeResizeRequest_Status {
    #[doc = "Set the field `error`.\n"]
    pub fn set_error(
        mut self,
        v: impl std::convert::Into<terrars::ListField<ComputeResizeRequest_Status_Error>>,
    ) -> Self {
        self.error = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `last_attempt`.\n"]
    pub fn set_last_attempt(
        mut self,
        v: impl std::convert::Into<terrars::ListField<ComputeResizeRequest_Status_LastAttempt>>,
    ) -> Self {
        self.last_attempt = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeResizeRequest_Status {
    type O = terrars::BlockAssignable<ComputeResizeRequest_Status>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeResizeRequest_Status {}

impl BuildComputeResizeRequest_Status {
    pub fn build(self) -> ComputeResizeRequest_Status {
        ComputeResizeRequest_Status {
            error: core::default::Default::default(),
            last_attempt: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeResizeRequest_StatusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeResizeRequest_StatusRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeResizeRequest_StatusRef {
        ComputeResizeRequest_StatusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeResizeRequest_StatusRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `error` after provisioning.\n"]
    pub fn error(&self) -> terrars::ListRef<ComputeResizeRequest_Status_ErrorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.error", self.base))
    }

    #[doc = "Get a reference to the value of field `last_attempt` after provisioning.\n"]
    pub fn last_attempt(&self) -> terrars::ListRef<ComputeResizeRequest_Status_LastAttemptRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.last_attempt", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeResizeRequest_RequestedRunDuration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    seconds: terrars::PrimField<std::string::String>,
}

impl ComputeResizeRequest_RequestedRunDuration {
    #[doc =
        "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations less than one second are represented with a 0 seconds field and a positive nanos field. Must be from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeResizeRequest_RequestedRunDuration {
    type O = terrars::BlockAssignable<ComputeResizeRequest_RequestedRunDuration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeResizeRequest_RequestedRunDuration {
    #[doc =
        "Span of time at a resolution of a second. Must be from 600 to 604800 inclusive. Note: minimum and maximum allowed range for requestedRunDuration is 10 minutes (600 seconds) and 7 days(604800 seconds) correspondingly."]
    pub seconds: terrars::PrimField<std::string::String>,
}

impl BuildComputeResizeRequest_RequestedRunDuration {
    pub fn build(self) -> ComputeResizeRequest_RequestedRunDuration {
        ComputeResizeRequest_RequestedRunDuration {
            nanos: core::default::Default::default(),
            seconds: self.seconds,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeResizeRequest_RequestedRunDurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeResizeRequest_RequestedRunDurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeResizeRequest_RequestedRunDurationRef {
        ComputeResizeRequest_RequestedRunDurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeResizeRequest_RequestedRunDurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `nanos` after provisioning.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations less than one second are represented with a 0 seconds field and a positive nanos field. Must be from 0 to 999,999,999 inclusive."]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc =
        "Get a reference to the value of field `seconds` after provisioning.\nSpan of time at a resolution of a second. Must be from 600 to 604800 inclusive. Note: minimum and maximum allowed range for requestedRunDuration is 10 minutes (600 seconds) and 7 days(604800 seconds) correspondingly."]
    pub fn seconds(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeResizeRequest_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeResizeRequest_Timeouts {
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

impl terrars::ToListMappable for ComputeResizeRequest_Timeouts {
    type O = terrars::BlockAssignable<ComputeResizeRequest_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeResizeRequest_Timeouts {}

impl BuildComputeResizeRequest_Timeouts {
    pub fn build(self) -> ComputeResizeRequest_Timeouts {
        ComputeResizeRequest_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeResizeRequest_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeResizeRequest_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeResizeRequest_TimeoutsRef {
        ComputeResizeRequest_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeResizeRequest_TimeoutsRef {
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
