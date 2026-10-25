#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct StorageFtpServerData_ {
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
    access_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    server_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_config: std::option::Option<std::vec::Vec<StorageFtpServer_ExternalConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    internal_config: std::option::Option<std::vec::Vec<StorageFtpServer_InternalConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<StorageFtpServer_Timeouts>,
    dynamic: StorageFtpServerDynamic,
}

struct StorageFtpServer_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<StorageFtpServerData_>,
}

#[derive(Clone)]
pub struct StorageFtpServer(std::rc::Rc<StorageFtpServer_>);

impl StorageFtpServer {
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

    #[doc = "Set the field `display_name`.\nA display name for the server."]
    pub fn set_display_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nA set of key/value label pairs to assign to the Storage FTP Server.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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

    #[doc = "Set the field `external_config`.\n"]
    pub fn set_external_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageFtpServer_ExternalConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().external_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.external_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `internal_config`.\n"]
    pub fn set_internal_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageFtpServer_InternalConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().internal_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.internal_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<StorageFtpServer_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `access_type` after provisioning.\nThe access type for this SFTP server. Possible values: INTERNAL, EXTERNAL Possible values: [\"INTERNAL\", \"EXTERNAL\"]"]
    pub fn access_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nA display name for the server."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs to assign to the Storage FTP Server.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location (region) of the Storage FTP Server."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `server_id` after provisioning.\nA unique ID for the server. Must start with a lowercase letter, and end with a lowercase letter or number. Can contain lowercase letters, numbers, and hyphens. Maximum 30 characters."]
    pub fn server_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_agent` after provisioning.\nThe email address of the service agent associated with the SFTP server."]
    pub fn service_agent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_agent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe operational lifecycle state of the SFTP server."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `external_config` after provisioning.\n"]
    pub fn external_config(&self) -> terrars::ListRef<StorageFtpServer_ExternalConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.external_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `internal_config` after provisioning.\n"]
    pub fn internal_config(&self) -> terrars::ListRef<StorageFtpServer_InternalConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.internal_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> StorageFtpServer_TimeoutsRef {
        StorageFtpServer_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for StorageFtpServer {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for StorageFtpServer { }

impl terrars::ToListMappable for StorageFtpServer {
    type O = terrars::ListRef<StorageFtpServerRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for StorageFtpServer_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_storage_ftp_server".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildStorageFtpServer {
    pub tf_id: std::string::String,
    #[doc =
        "The access type for this SFTP server. Possible values: INTERNAL, EXTERNAL Possible values: [\"INTERNAL\", \"EXTERNAL\"]"]
    pub access_type: terrars::PrimField<std::string::String>,
    #[doc = "The location (region) of the Storage FTP Server."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc =
        "A unique ID for the server. Must start with a lowercase letter, and end with a lowercase letter or number. Can contain lowercase letters, numbers, and hyphens. Maximum 30 characters."]
    pub server_id: terrars::PrimField<std::string::String>,
}

impl BuildStorageFtpServer {
    pub fn build(self, stack: &mut terrars::Stack) -> StorageFtpServer {
        let out = StorageFtpServer(std::rc::Rc::new(StorageFtpServer_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(StorageFtpServerData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                access_type: self.access_type,
                deletion_policy: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                server_id: self.server_id,
                external_config: core::default::Default::default(),
                internal_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct StorageFtpServerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageFtpServerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl StorageFtpServerRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `access_type` after provisioning.\nThe access type for this SFTP server. Possible values: INTERNAL, EXTERNAL Possible values: [\"INTERNAL\", \"EXTERNAL\"]"]
    pub fn access_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nA display name for the server."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs to assign to the Storage FTP Server.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location (region) of the Storage FTP Server."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `server_id` after provisioning.\nA unique ID for the server. Must start with a lowercase letter, and end with a lowercase letter or number. Can contain lowercase letters, numbers, and hyphens. Maximum 30 characters."]
    pub fn server_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_agent` after provisioning.\nThe email address of the service agent associated with the SFTP server."]
    pub fn service_agent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_agent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe operational lifecycle state of the SFTP server."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `external_config` after provisioning.\n"]
    pub fn external_config(&self) -> terrars::ListRef<StorageFtpServer_ExternalConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.external_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `internal_config` after provisioning.\n"]
    pub fn internal_config(&self) -> terrars::ListRef<StorageFtpServer_InternalConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.internal_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> StorageFtpServer_TimeoutsRef {
        StorageFtpServer_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct StorageFtpServerDynamic {
    external_config: std::option::Option<terrars::DynamicBlock<StorageFtpServer_ExternalConfig>>,
    internal_config: std::option::Option<terrars::DynamicBlock<StorageFtpServer_InternalConfig>>,
}

#[derive(Serialize)]
pub struct StorageFtpServer_ExternalConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_cidr_blocks: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl StorageFtpServer_ExternalConfig {
    #[doc =
        "Set the field `allowed_cidr_blocks`.\nA list of allowed IPv4 or IPv6 CIDR block ranges that can connect to this server."]
    pub fn set_allowed_cidr_blocks(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_cidr_blocks = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for StorageFtpServer_ExternalConfig {
    type O = terrars::BlockAssignable<StorageFtpServer_ExternalConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageFtpServer_ExternalConfig {}

impl BuildStorageFtpServer_ExternalConfig {
    pub fn build(self) -> StorageFtpServer_ExternalConfig {
        StorageFtpServer_ExternalConfig { allowed_cidr_blocks: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct StorageFtpServer_ExternalConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageFtpServer_ExternalConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StorageFtpServer_ExternalConfigRef {
        StorageFtpServer_ExternalConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageFtpServer_ExternalConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed_cidr_blocks` after provisioning.\nA list of allowed IPv4 or IPv6 CIDR block ranges that can connect to this server."]
    pub fn allowed_cidr_blocks(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allowed_cidr_blocks", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ip_address` after provisioning.\nThe public IP address of the external load balancer for the SFTP server."]
    pub fn ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageFtpServer_InternalConfig_ConsumerAcceptList {
    connection_limit: terrars::PrimField<f64>,
    project: terrars::PrimField<std::string::String>,
}

impl StorageFtpServer_InternalConfig_ConsumerAcceptList { }

impl terrars::ToListMappable for StorageFtpServer_InternalConfig_ConsumerAcceptList {
    type O = terrars::BlockAssignable<StorageFtpServer_InternalConfig_ConsumerAcceptList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageFtpServer_InternalConfig_ConsumerAcceptList {
    #[doc = "The maximum number of Private Service Connect endpoints that can be created in the consumer project."]
    pub connection_limit: terrars::PrimField<f64>,
    #[doc = "The project that is allowed to connect, in the format 'projects/{project}'."]
    pub project: terrars::PrimField<std::string::String>,
}

impl BuildStorageFtpServer_InternalConfig_ConsumerAcceptList {
    pub fn build(self) -> StorageFtpServer_InternalConfig_ConsumerAcceptList {
        StorageFtpServer_InternalConfig_ConsumerAcceptList {
            connection_limit: self.connection_limit,
            project: self.project,
        }
    }
}

#[allow(dead_code)]
pub struct StorageFtpServer_InternalConfig_ConsumerAcceptListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageFtpServer_InternalConfig_ConsumerAcceptListRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> StorageFtpServer_InternalConfig_ConsumerAcceptListRef {
        StorageFtpServer_InternalConfig_ConsumerAcceptListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageFtpServer_InternalConfig_ConsumerAcceptListRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `connection_limit` after provisioning.\nThe maximum number of Private Service Connect endpoints that can be created in the consumer project."]
    pub fn connection_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_limit", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe project that is allowed to connect, in the format 'projects/{project}'."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageFtpServer_InternalConfig_ConsumerRejectList {
    project: terrars::PrimField<std::string::String>,
}

impl StorageFtpServer_InternalConfig_ConsumerRejectList { }

impl terrars::ToListMappable for StorageFtpServer_InternalConfig_ConsumerRejectList {
    type O = terrars::BlockAssignable<StorageFtpServer_InternalConfig_ConsumerRejectList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageFtpServer_InternalConfig_ConsumerRejectList {
    #[doc = "The project that is rejected from connecting, in the format 'projects/{project}'."]
    pub project: terrars::PrimField<std::string::String>,
}

impl BuildStorageFtpServer_InternalConfig_ConsumerRejectList {
    pub fn build(self) -> StorageFtpServer_InternalConfig_ConsumerRejectList {
        StorageFtpServer_InternalConfig_ConsumerRejectList { project: self.project }
    }
}

#[allow(dead_code)]
pub struct StorageFtpServer_InternalConfig_ConsumerRejectListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageFtpServer_InternalConfig_ConsumerRejectListRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> StorageFtpServer_InternalConfig_ConsumerRejectListRef {
        StorageFtpServer_InternalConfig_ConsumerRejectListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageFtpServer_InternalConfig_ConsumerRejectListRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe project that is rejected from connecting, in the format 'projects/{project}'."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }
}

#[derive(Serialize, Default)]
struct StorageFtpServer_InternalConfigDynamic {
    consumer_accept_list: std
    ::option
    ::Option<terrars::DynamicBlock<StorageFtpServer_InternalConfig_ConsumerAcceptList>>,
    consumer_reject_list: std
    ::option
    ::Option<terrars::DynamicBlock<StorageFtpServer_InternalConfig_ConsumerRejectList>>,
}

#[derive(Serialize)]
pub struct StorageFtpServer_InternalConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consumer_accept_list: std::option::Option<std::vec::Vec<StorageFtpServer_InternalConfig_ConsumerAcceptList>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consumer_reject_list: std::option::Option<std::vec::Vec<StorageFtpServer_InternalConfig_ConsumerRejectList>>,
    dynamic: StorageFtpServer_InternalConfigDynamic,
}

impl StorageFtpServer_InternalConfig {
    #[doc = "Set the field `consumer_accept_list`.\n"]
    pub fn set_consumer_accept_list(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageFtpServer_InternalConfig_ConsumerAcceptList>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.consumer_accept_list = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.consumer_accept_list = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `consumer_reject_list`.\n"]
    pub fn set_consumer_reject_list(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageFtpServer_InternalConfig_ConsumerRejectList>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.consumer_reject_list = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.consumer_reject_list = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for StorageFtpServer_InternalConfig {
    type O = terrars::BlockAssignable<StorageFtpServer_InternalConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageFtpServer_InternalConfig {}

impl BuildStorageFtpServer_InternalConfig {
    pub fn build(self) -> StorageFtpServer_InternalConfig {
        StorageFtpServer_InternalConfig {
            consumer_accept_list: core::default::Default::default(),
            consumer_reject_list: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct StorageFtpServer_InternalConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageFtpServer_InternalConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StorageFtpServer_InternalConfigRef {
        StorageFtpServer_InternalConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageFtpServer_InternalConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `service_attachment` after provisioning.\nThe Private Service Connect service attachment URI for the SFTP server."]
    pub fn service_attachment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_attachment", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageFtpServer_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl StorageFtpServer_Timeouts {
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

impl terrars::ToListMappable for StorageFtpServer_Timeouts {
    type O = terrars::BlockAssignable<StorageFtpServer_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageFtpServer_Timeouts {}

impl BuildStorageFtpServer_Timeouts {
    pub fn build(self) -> StorageFtpServer_Timeouts {
        StorageFtpServer_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct StorageFtpServer_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageFtpServer_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StorageFtpServer_TimeoutsRef {
        StorageFtpServer_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageFtpServer_TimeoutsRef {
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
