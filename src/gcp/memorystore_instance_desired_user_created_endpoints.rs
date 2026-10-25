#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct MemorystoreInstanceDesiredUserCreatedEndpointsData_ {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    region: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    desired_user_created_endpoints: std
    ::option
    ::Option<std::vec::Vec<MemorystoreInstanceDesiredUserCreatedEndpoints_DesiredUserCreatedEndpoints>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<MemorystoreInstanceDesiredUserCreatedEndpoints_Timeouts>,
    dynamic: MemorystoreInstanceDesiredUserCreatedEndpointsDynamic,
}

struct MemorystoreInstanceDesiredUserCreatedEndpoints_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<MemorystoreInstanceDesiredUserCreatedEndpointsData_>,
}

#[derive(Clone)]
pub struct MemorystoreInstanceDesiredUserCreatedEndpoints(
    std::rc::Rc<MemorystoreInstanceDesiredUserCreatedEndpoints_>,
);

impl MemorystoreInstanceDesiredUserCreatedEndpoints {
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

    #[doc = "Set the field `desired_user_created_endpoints`.\n"]
    pub fn set_desired_user_created_endpoints(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<MemorystoreInstanceDesiredUserCreatedEndpoints_DesiredUserCreatedEndpoints>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().desired_user_created_endpoints = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.desired_user_created_endpoints = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(
        self,
        v: impl std::convert::Into<MemorystoreInstanceDesiredUserCreatedEndpoints_Timeouts>,
    ) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the Memorystore instance these endpoints should be added to."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe name of the region of the Memorystore instance these endpoints should be added to."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `desired_user_created_endpoints` after provisioning.\n"]
    pub fn desired_user_created_endpoints(
        &self,
    ) -> terrars::ListRef<MemorystoreInstanceDesiredUserCreatedEndpoints_DesiredUserCreatedEndpointsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.desired_user_created_endpoints", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MemorystoreInstanceDesiredUserCreatedEndpoints_TimeoutsRef {
        MemorystoreInstanceDesiredUserCreatedEndpoints_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for MemorystoreInstanceDesiredUserCreatedEndpoints {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for MemorystoreInstanceDesiredUserCreatedEndpoints { }

impl terrars::ToListMappable for MemorystoreInstanceDesiredUserCreatedEndpoints {
    type O = terrars::ListRef<MemorystoreInstanceDesiredUserCreatedEndpointsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for MemorystoreInstanceDesiredUserCreatedEndpoints_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_memorystore_instance_desired_user_created_endpoints".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildMemorystoreInstanceDesiredUserCreatedEndpoints {
    pub tf_id: std::string::String,
    #[doc = "The name of the Memorystore instance these endpoints should be added to."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The name of the region of the Memorystore instance these endpoints should be added to."]
    pub region: terrars::PrimField<std::string::String>,
}

impl BuildMemorystoreInstanceDesiredUserCreatedEndpoints {
    pub fn build(self, stack: &mut terrars::Stack) -> MemorystoreInstanceDesiredUserCreatedEndpoints {
        let out =
            MemorystoreInstanceDesiredUserCreatedEndpoints(
                std::rc::Rc::new(MemorystoreInstanceDesiredUserCreatedEndpoints_ {
                    shared: stack.shared.clone(),
                    tf_id: self.tf_id,
                    data: std::cell::RefCell::new(MemorystoreInstanceDesiredUserCreatedEndpointsData_ {
                        depends_on: core::default::Default::default(),
                        provider: std::option::Option::None,
                        lifecycle: core::default::Default::default(),
                        for_each: std::option::Option::None,
                        provisioner: core::default::Default::default(),
                        deletion_policy: core::default::Default::default(),
                        id: core::default::Default::default(),
                        name: self.name,
                        project: core::default::Default::default(),
                        region: self.region,
                        desired_user_created_endpoints: core::default::Default::default(),
                        timeouts: core::default::Default::default(),
                        dynamic: core::default::Default::default(),
                    }),
                }),
            );
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct MemorystoreInstanceDesiredUserCreatedEndpointsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MemorystoreInstanceDesiredUserCreatedEndpointsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl MemorystoreInstanceDesiredUserCreatedEndpointsRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the Memorystore instance these endpoints should be added to."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe name of the region of the Memorystore instance these endpoints should be added to."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `desired_user_created_endpoints` after provisioning.\n"]
    pub fn desired_user_created_endpoints(
        &self,
    ) -> terrars::ListRef<MemorystoreInstanceDesiredUserCreatedEndpoints_DesiredUserCreatedEndpointsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.desired_user_created_endpoints", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MemorystoreInstanceDesiredUserCreatedEndpoints_TimeoutsRef {
        MemorystoreInstanceDesiredUserCreatedEndpoints_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct MemorystoreInstanceDesiredUserCreatedEndpointsDynamic {
    desired_user_created_endpoints: std
    ::option
    ::Option<terrars::DynamicBlock<MemorystoreInstanceDesiredUserCreatedEndpoints_DesiredUserCreatedEndpoints>>,
}

#[derive(Serialize)]
pub struct MemorystoreInstanceDesiredUserCreatedEndpoints_DesiredUserCreatedEndpoints_Connections_PscConnection {
    forwarding_rule: terrars::PrimField<std::string::String>,
    ip_address: terrars::PrimField<std::string::String>,
    network: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id: std::option::Option<terrars::PrimField<std::string::String>>,
    psc_connection_id: terrars::PrimField<std::string::String>,
    service_attachment: terrars::PrimField<std::string::String>,
}

impl MemorystoreInstanceDesiredUserCreatedEndpoints_DesiredUserCreatedEndpoints_Connections_PscConnection {
    #[doc = "Set the field `project_id`.\nThe consumer project_id where the forwarding rule is created from."]
    pub fn set_project_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MemorystoreInstanceDesiredUserCreatedEndpoints_DesiredUserCreatedEndpoints_Connections_PscConnection {
    type O =
        terrars
        ::BlockAssignable<
            MemorystoreInstanceDesiredUserCreatedEndpoints_DesiredUserCreatedEndpoints_Connections_PscConnection,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMemorystoreInstanceDesiredUserCreatedEndpoints_DesiredUserCreatedEndpoints_Connections_PscConnection {
    #[doc =
        "The URI of the consumer side forwarding rule.\nFormat:\nprojects/{project}/regions/{region}/forwardingRules/{forwarding_rule}"]
    pub forwarding_rule: terrars::PrimField<std::string::String>,
    #[doc = "The IP allocated on the consumer network for the PSC forwarding rule."]
    pub ip_address: terrars::PrimField<std::string::String>,
    #[doc =
        "The consumer network where the IP address resides, in the form of\nprojects/{project_id}/global/networks/{network_id}."]
    pub network: terrars::PrimField<std::string::String>,
    #[doc = "The PSC connection id of the forwarding rule connected to the\nservice attachment."]
    pub psc_connection_id: terrars::PrimField<std::string::String>,
    #[doc =
        "The service attachment which is the target of the PSC connection, in the form of projects/{project-id}/regions/{region}/serviceAttachments/{service-attachment-id}."]
    pub service_attachment: terrars::PrimField<std::string::String>,
}

impl BuildMemorystoreInstanceDesiredUserCreatedEndpoints_DesiredUserCreatedEndpoints_Connections_PscConnection {
    pub fn build(
        self,
    ) -> MemorystoreInstanceDesiredUserCreatedEndpoints_DesiredUserCreatedEndpoints_Connections_PscConnection {
        MemorystoreInstanceDesiredUserCreatedEndpoints_DesiredUserCreatedEndpoints_Connections_PscConnection {
            forwarding_rule: self.forwarding_rule,
            ip_address: self.ip_address,
            network: self.network,
            project_id: core::default::Default::default(),
            psc_connection_id: self.psc_connection_id,
            service_attachment: self.service_attachment,
        }
    }
}

#[allow(dead_code)]
pub struct MemorystoreInstanceDesiredUserCreatedEndpoints_DesiredUserCreatedEndpoints_Connections_PscConnectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MemorystoreInstanceDesiredUserCreatedEndpoints_DesiredUserCreatedEndpoints_Connections_PscConnectionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MemorystoreInstanceDesiredUserCreatedEndpoints_DesiredUserCreatedEndpoints_Connections_PscConnectionRef {
        MemorystoreInstanceDesiredUserCreatedEndpoints_DesiredUserCreatedEndpoints_Connections_PscConnectionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MemorystoreInstanceDesiredUserCreatedEndpoints_DesiredUserCreatedEndpoints_Connections_PscConnectionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `connection_type` after provisioning.\nOutput Only. Type of a PSC Connection. \n Possible values:\n CONNECTION_TYPE_DISCOVERY \n CONNECTION_TYPE_PRIMARY \n CONNECTION_TYPE_READER"]
    pub fn connection_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `forwarding_rule` after provisioning.\nThe URI of the consumer side forwarding rule.\nFormat:\nprojects/{project}/regions/{region}/forwardingRules/{forwarding_rule}"]
    pub fn forwarding_rule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.forwarding_rule", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ip_address` after provisioning.\nThe IP allocated on the consumer network for the PSC forwarding rule."]
    pub fn ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nThe consumer network where the IP address resides, in the form of\nprojects/{project_id}/global/networks/{network_id}."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project_id` after provisioning.\nThe consumer project_id where the forwarding rule is created from."]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `psc_connection_id` after provisioning.\nThe PSC connection id of the forwarding rule connected to the\nservice attachment."]
    pub fn psc_connection_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.psc_connection_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `psc_connection_status` after provisioning.\nOutput Only. The status of the PSC connection: whether a connection exists and ACTIVE or it no longer exists. \n Possible values:\n ACTIVE \n NOT_FOUND"]
    pub fn psc_connection_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.psc_connection_status", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_attachment` after provisioning.\nThe service attachment which is the target of the PSC connection, in the form of projects/{project-id}/regions/{region}/serviceAttachments/{service-attachment-id}."]
    pub fn service_attachment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_attachment", self.base))
    }
}

#[derive(Serialize, Default)]
struct MemorystoreInstanceDesiredUserCreatedEndpoints_DesiredUserCreatedEndpoints_ConnectionsDynamic {
    psc_connection: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MemorystoreInstanceDesiredUserCreatedEndpoints_DesiredUserCreatedEndpoints_Connections_PscConnection,
        >,
    >,
}

#[derive(Serialize)]
pub struct MemorystoreInstanceDesiredUserCreatedEndpoints_DesiredUserCreatedEndpoints_Connections {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_connection: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<MemorystoreInstanceDesiredUserCreatedEndpoints_DesiredUserCreatedEndpoints_Connections_PscConnection>,
    >,
    dynamic: MemorystoreInstanceDesiredUserCreatedEndpoints_DesiredUserCreatedEndpoints_ConnectionsDynamic,
}

impl MemorystoreInstanceDesiredUserCreatedEndpoints_DesiredUserCreatedEndpoints_Connections {
    #[doc = "Set the field `psc_connection`.\n"]
    pub fn set_psc_connection(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MemorystoreInstanceDesiredUserCreatedEndpoints_DesiredUserCreatedEndpoints_Connections_PscConnection,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.psc_connection = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.psc_connection = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for MemorystoreInstanceDesiredUserCreatedEndpoints_DesiredUserCreatedEndpoints_Connections {
    type O =
        terrars
        ::BlockAssignable<MemorystoreInstanceDesiredUserCreatedEndpoints_DesiredUserCreatedEndpoints_Connections>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMemorystoreInstanceDesiredUserCreatedEndpoints_DesiredUserCreatedEndpoints_Connections {}

impl BuildMemorystoreInstanceDesiredUserCreatedEndpoints_DesiredUserCreatedEndpoints_Connections {
    pub fn build(self) -> MemorystoreInstanceDesiredUserCreatedEndpoints_DesiredUserCreatedEndpoints_Connections {
        MemorystoreInstanceDesiredUserCreatedEndpoints_DesiredUserCreatedEndpoints_Connections {
            psc_connection: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MemorystoreInstanceDesiredUserCreatedEndpoints_DesiredUserCreatedEndpoints_ConnectionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MemorystoreInstanceDesiredUserCreatedEndpoints_DesiredUserCreatedEndpoints_ConnectionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MemorystoreInstanceDesiredUserCreatedEndpoints_DesiredUserCreatedEndpoints_ConnectionsRef {
        MemorystoreInstanceDesiredUserCreatedEndpoints_DesiredUserCreatedEndpoints_ConnectionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MemorystoreInstanceDesiredUserCreatedEndpoints_DesiredUserCreatedEndpoints_ConnectionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `psc_connection` after provisioning.\n"]
    pub fn psc_connection(
        &self,
    ) -> terrars
    ::ListRef<
        MemorystoreInstanceDesiredUserCreatedEndpoints_DesiredUserCreatedEndpoints_Connections_PscConnectionRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_connection", self.base))
    }
}

#[derive(Serialize, Default)]
struct MemorystoreInstanceDesiredUserCreatedEndpoints_DesiredUserCreatedEndpointsDynamic {
    connections: std
    ::option
    ::Option<
        terrars::DynamicBlock<MemorystoreInstanceDesiredUserCreatedEndpoints_DesiredUserCreatedEndpoints_Connections>,
    >,
}

#[derive(Serialize)]
pub struct MemorystoreInstanceDesiredUserCreatedEndpoints_DesiredUserCreatedEndpoints {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connections: std
    ::option
    ::Option<std::vec::Vec<MemorystoreInstanceDesiredUserCreatedEndpoints_DesiredUserCreatedEndpoints_Connections>>,
    dynamic: MemorystoreInstanceDesiredUserCreatedEndpoints_DesiredUserCreatedEndpointsDynamic,
}

impl MemorystoreInstanceDesiredUserCreatedEndpoints_DesiredUserCreatedEndpoints {
    #[doc = "Set the field `connections`.\n"]
    pub fn set_connections(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MemorystoreInstanceDesiredUserCreatedEndpoints_DesiredUserCreatedEndpoints_Connections,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.connections = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.connections = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MemorystoreInstanceDesiredUserCreatedEndpoints_DesiredUserCreatedEndpoints {
    type O = terrars::BlockAssignable<MemorystoreInstanceDesiredUserCreatedEndpoints_DesiredUserCreatedEndpoints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMemorystoreInstanceDesiredUserCreatedEndpoints_DesiredUserCreatedEndpoints {}

impl BuildMemorystoreInstanceDesiredUserCreatedEndpoints_DesiredUserCreatedEndpoints {
    pub fn build(self) -> MemorystoreInstanceDesiredUserCreatedEndpoints_DesiredUserCreatedEndpoints {
        MemorystoreInstanceDesiredUserCreatedEndpoints_DesiredUserCreatedEndpoints {
            connections: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MemorystoreInstanceDesiredUserCreatedEndpoints_DesiredUserCreatedEndpointsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MemorystoreInstanceDesiredUserCreatedEndpoints_DesiredUserCreatedEndpointsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MemorystoreInstanceDesiredUserCreatedEndpoints_DesiredUserCreatedEndpointsRef {
        MemorystoreInstanceDesiredUserCreatedEndpoints_DesiredUserCreatedEndpointsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MemorystoreInstanceDesiredUserCreatedEndpoints_DesiredUserCreatedEndpointsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connections` after provisioning.\n"]
    pub fn connections(
        &self,
    ) -> terrars::ListRef<MemorystoreInstanceDesiredUserCreatedEndpoints_DesiredUserCreatedEndpoints_ConnectionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connections", self.base))
    }
}

#[derive(Serialize)]
pub struct MemorystoreInstanceDesiredUserCreatedEndpoints_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MemorystoreInstanceDesiredUserCreatedEndpoints_Timeouts {
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

impl terrars::ToListMappable for MemorystoreInstanceDesiredUserCreatedEndpoints_Timeouts {
    type O = terrars::BlockAssignable<MemorystoreInstanceDesiredUserCreatedEndpoints_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMemorystoreInstanceDesiredUserCreatedEndpoints_Timeouts {}

impl BuildMemorystoreInstanceDesiredUserCreatedEndpoints_Timeouts {
    pub fn build(self) -> MemorystoreInstanceDesiredUserCreatedEndpoints_Timeouts {
        MemorystoreInstanceDesiredUserCreatedEndpoints_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MemorystoreInstanceDesiredUserCreatedEndpoints_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MemorystoreInstanceDesiredUserCreatedEndpoints_TimeoutsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MemorystoreInstanceDesiredUserCreatedEndpoints_TimeoutsRef {
        MemorystoreInstanceDesiredUserCreatedEndpoints_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MemorystoreInstanceDesiredUserCreatedEndpoints_TimeoutsRef {
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
