#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ChronicleDataAccessScopeData_ {
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
    allow_all: std::option::Option<terrars::PrimField<bool>>,
    data_access_scope_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    instance: terrars::PrimField<std::string::String>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_data_access_labels: std::option::Option<std::vec::Vec<ChronicleDataAccessScope_AllowedDataAccessLabels>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    denied_data_access_labels: std::option::Option<std::vec::Vec<ChronicleDataAccessScope_DeniedDataAccessLabels>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ChronicleDataAccessScope_Timeouts>,
    dynamic: ChronicleDataAccessScopeDynamic,
}

struct ChronicleDataAccessScope_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ChronicleDataAccessScopeData_>,
}

#[derive(Clone)]
pub struct ChronicleDataAccessScope(std::rc::Rc<ChronicleDataAccessScope_>);

impl ChronicleDataAccessScope {
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
        "Set the field `allow_all`.\nOptional. Whether or not the scope allows all labels, allow_all and\nallowed_data_access_labels are mutually exclusive and one of them must be\npresent. denied_data_access_labels can still be used along with allow_all.\nWhen combined with denied_data_access_labels, access will be granted to all\ndata that doesn't have labels mentioned in denied_data_access_labels. E.g.:\nA customer with scope with denied labels A and B and allow_all will be able\nto see all data except data labeled with A and data labeled with B and data\nwith labels A and B."]
    pub fn set_allow_all(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allow_all = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nOptional. A description of the data access scope for a human reader."]
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

    #[doc = "Set the field `allowed_data_access_labels`.\n"]
    pub fn set_allowed_data_access_labels(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleDataAccessScope_AllowedDataAccessLabels>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().allowed_data_access_labels = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.allowed_data_access_labels = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `denied_data_access_labels`.\n"]
    pub fn set_denied_data_access_labels(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleDataAccessScope_DeniedDataAccessLabels>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().denied_data_access_labels = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.denied_data_access_labels = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ChronicleDataAccessScope_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `allow_all` after provisioning.\nOptional. Whether or not the scope allows all labels, allow_all and\nallowed_data_access_labels are mutually exclusive and one of them must be\npresent. denied_data_access_labels can still be used along with allow_all.\nWhen combined with denied_data_access_labels, access will be granted to all\ndata that doesn't have labels mentioned in denied_data_access_labels. E.g.:\nA customer with scope with denied labels A and B and allow_all will be able\nto see all data except data labeled with A and data labeled with B and data\nwith labels A and B."]
    pub fn allow_all(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_all", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `author` after provisioning.\nOutput only. The user who created the data access scope."]
    pub fn author(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.author", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time at which the data access scope was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_access_scope_id` after provisioning.\nRequired. The user provided scope id which will become the last part of the name\nof the scope resource.\nNeeds to be compliant with https://google.aip.dev/122"]
    pub fn data_access_scope_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_access_scope_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOptional. A description of the data access scope for a human reader."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nOutput only. The name to be used for display to customers of the data access scope."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance` after provisioning.\nThe unique identifier for the Chronicle instance, which is the same as the customer ID."]
    pub fn instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_editor` after provisioning.\nOutput only. The user who last updated the data access scope."]
    pub fn last_editor(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_editor", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location of the resource. This is the geographical region where the Chronicle instance resides, such as \"us\" or \"europe-west2\"."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe unique full name of the data access scope. This unique identifier is generated using values provided for the URL parameters.\nFormat:\nprojects/{project}/locations/{location}/instances/{instance}/dataAccessScopes/{data_access_scope_id}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time at which the data access scope was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `allowed_data_access_labels` after provisioning.\n"]
    pub fn allowed_data_access_labels(&self) -> terrars::ListRef<ChronicleDataAccessScope_AllowedDataAccessLabelsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_data_access_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `denied_data_access_labels` after provisioning.\n"]
    pub fn denied_data_access_labels(&self) -> terrars::ListRef<ChronicleDataAccessScope_DeniedDataAccessLabelsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.denied_data_access_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ChronicleDataAccessScope_TimeoutsRef {
        ChronicleDataAccessScope_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ChronicleDataAccessScope {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ChronicleDataAccessScope { }

impl terrars::ToListMappable for ChronicleDataAccessScope {
    type O = terrars::ListRef<ChronicleDataAccessScopeRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ChronicleDataAccessScope_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_chronicle_data_access_scope".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildChronicleDataAccessScope {
    pub tf_id: std::string::String,
    #[doc =
        "Required. The user provided scope id which will become the last part of the name\nof the scope resource.\nNeeds to be compliant with https://google.aip.dev/122"]
    pub data_access_scope_id: terrars::PrimField<std::string::String>,
    #[doc = "The unique identifier for the Chronicle instance, which is the same as the customer ID."]
    pub instance: terrars::PrimField<std::string::String>,
    #[doc =
        "The location of the resource. This is the geographical region where the Chronicle instance resides, such as \"us\" or \"europe-west2\"."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildChronicleDataAccessScope {
    pub fn build(self, stack: &mut terrars::Stack) -> ChronicleDataAccessScope {
        let out = ChronicleDataAccessScope(std::rc::Rc::new(ChronicleDataAccessScope_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ChronicleDataAccessScopeData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                allow_all: core::default::Default::default(),
                data_access_scope_id: self.data_access_scope_id,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                instance: self.instance,
                location: self.location,
                project: core::default::Default::default(),
                allowed_data_access_labels: core::default::Default::default(),
                denied_data_access_labels: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ChronicleDataAccessScopeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleDataAccessScopeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ChronicleDataAccessScopeRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_all` after provisioning.\nOptional. Whether or not the scope allows all labels, allow_all and\nallowed_data_access_labels are mutually exclusive and one of them must be\npresent. denied_data_access_labels can still be used along with allow_all.\nWhen combined with denied_data_access_labels, access will be granted to all\ndata that doesn't have labels mentioned in denied_data_access_labels. E.g.:\nA customer with scope with denied labels A and B and allow_all will be able\nto see all data except data labeled with A and data labeled with B and data\nwith labels A and B."]
    pub fn allow_all(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_all", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `author` after provisioning.\nOutput only. The user who created the data access scope."]
    pub fn author(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.author", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time at which the data access scope was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_access_scope_id` after provisioning.\nRequired. The user provided scope id which will become the last part of the name\nof the scope resource.\nNeeds to be compliant with https://google.aip.dev/122"]
    pub fn data_access_scope_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_access_scope_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOptional. A description of the data access scope for a human reader."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nOutput only. The name to be used for display to customers of the data access scope."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance` after provisioning.\nThe unique identifier for the Chronicle instance, which is the same as the customer ID."]
    pub fn instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_editor` after provisioning.\nOutput only. The user who last updated the data access scope."]
    pub fn last_editor(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_editor", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location of the resource. This is the geographical region where the Chronicle instance resides, such as \"us\" or \"europe-west2\"."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe unique full name of the data access scope. This unique identifier is generated using values provided for the URL parameters.\nFormat:\nprojects/{project}/locations/{location}/instances/{instance}/dataAccessScopes/{data_access_scope_id}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time at which the data access scope was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `allowed_data_access_labels` after provisioning.\n"]
    pub fn allowed_data_access_labels(&self) -> terrars::ListRef<ChronicleDataAccessScope_AllowedDataAccessLabelsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_data_access_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `denied_data_access_labels` after provisioning.\n"]
    pub fn denied_data_access_labels(&self) -> terrars::ListRef<ChronicleDataAccessScope_DeniedDataAccessLabelsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.denied_data_access_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ChronicleDataAccessScope_TimeoutsRef {
        ChronicleDataAccessScope_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ChronicleDataAccessScopeDynamic {
    allowed_data_access_labels: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleDataAccessScope_AllowedDataAccessLabels>>,
    denied_data_access_labels: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleDataAccessScope_DeniedDataAccessLabels>>,
}

#[derive(Serialize)]
pub struct ChronicleDataAccessScope_DeniedDataAccessLabels_IngestionLabel {
    ingestion_label_key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ingestion_label_value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleDataAccessScope_DeniedDataAccessLabels_IngestionLabel {
    #[doc =
        "Set the field `ingestion_label_value`.\nOptional. The value of the ingestion label. Optional. An object\nwith no provided value and some key provided would match\nagainst the given key and ANY value."]
    pub fn set_ingestion_label_value(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.ingestion_label_value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleDataAccessScope_DeniedDataAccessLabels_IngestionLabel {
    type O = terrars::BlockAssignable<ChronicleDataAccessScope_DeniedDataAccessLabels_IngestionLabel>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleDataAccessScope_DeniedDataAccessLabels_IngestionLabel {
    #[doc = "Required. The key of the ingestion label. Always required."]
    pub ingestion_label_key: terrars::PrimField<std::string::String>,
}

impl BuildChronicleDataAccessScope_DeniedDataAccessLabels_IngestionLabel {
    pub fn build(self) -> ChronicleDataAccessScope_DeniedDataAccessLabels_IngestionLabel {
        ChronicleDataAccessScope_DeniedDataAccessLabels_IngestionLabel {
            ingestion_label_key: self.ingestion_label_key,
            ingestion_label_value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleDataAccessScope_DeniedDataAccessLabels_IngestionLabelRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleDataAccessScope_DeniedDataAccessLabels_IngestionLabelRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleDataAccessScope_DeniedDataAccessLabels_IngestionLabelRef {
        ChronicleDataAccessScope_DeniedDataAccessLabels_IngestionLabelRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleDataAccessScope_DeniedDataAccessLabels_IngestionLabelRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ingestion_label_key` after provisioning.\nRequired. The key of the ingestion label. Always required."]
    pub fn ingestion_label_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ingestion_label_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ingestion_label_value` after provisioning.\nOptional. The value of the ingestion label. Optional. An object\nwith no provided value and some key provided would match\nagainst the given key and ANY value."]
    pub fn ingestion_label_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ingestion_label_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleDataAccessScope_AllowedDataAccessLabelsDynamic {
    ingestion_label: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleDataAccessScope_DeniedDataAccessLabels_IngestionLabel>>,
}

#[derive(Serialize)]
pub struct ChronicleDataAccessScope_AllowedDataAccessLabels {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    asset_namespace: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_access_label: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ingestion_label: std::option::Option<std::vec::Vec<ChronicleDataAccessScope_DeniedDataAccessLabels_IngestionLabel>>,
    dynamic: ChronicleDataAccessScope_AllowedDataAccessLabelsDynamic,
}

impl ChronicleDataAccessScope_AllowedDataAccessLabels {
    #[doc =
        "Set the field `asset_namespace`.\nThe asset namespace configured in the forwarder\nof the customer's events."]
    pub fn set_asset_namespace(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.asset_namespace = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_access_label`.\nThe name of the data access label."]
    pub fn set_data_access_label(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data_access_label = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_type`.\nThe name of the log type."]
    pub fn set_log_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ingestion_label`.\n"]
    pub fn set_ingestion_label(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ChronicleDataAccessScope_DeniedDataAccessLabels_IngestionLabel>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ingestion_label = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ingestion_label = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleDataAccessScope_AllowedDataAccessLabels {
    type O = terrars::BlockAssignable<ChronicleDataAccessScope_AllowedDataAccessLabels>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleDataAccessScope_AllowedDataAccessLabels {}

impl BuildChronicleDataAccessScope_AllowedDataAccessLabels {
    pub fn build(self) -> ChronicleDataAccessScope_AllowedDataAccessLabels {
        ChronicleDataAccessScope_AllowedDataAccessLabels {
            asset_namespace: core::default::Default::default(),
            data_access_label: core::default::Default::default(),
            log_type: core::default::Default::default(),
            ingestion_label: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleDataAccessScope_AllowedDataAccessLabelsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleDataAccessScope_AllowedDataAccessLabelsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleDataAccessScope_AllowedDataAccessLabelsRef {
        ChronicleDataAccessScope_AllowedDataAccessLabelsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleDataAccessScope_AllowedDataAccessLabelsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `asset_namespace` after provisioning.\nThe asset namespace configured in the forwarder\nof the customer's events."]
    pub fn asset_namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.asset_namespace", self.base))
    }

    #[doc =
        "Get a reference to the value of field `data_access_label` after provisioning.\nThe name of the data access label."]
    pub fn data_access_label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_access_label", self.base))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nOutput only. The display name of the label.\nData access label and log types's name\nwill match the display name of the resource.\nThe asset namespace will match the namespace itself.\nThe ingestion key value pair will match the key of the tuple."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `log_type` after provisioning.\nThe name of the log type."]
    pub fn log_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_type", self.base))
    }

    #[doc = "Get a reference to the value of field `ingestion_label` after provisioning.\n"]
    pub fn ingestion_label(&self) -> terrars::ListRef<ChronicleDataAccessScope_DeniedDataAccessLabels_IngestionLabelRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ingestion_label", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleDataAccessScope_DeniedDataAccessLabelsDynamic {
    ingestion_label: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleDataAccessScope_DeniedDataAccessLabels_IngestionLabel>>,
}

#[derive(Serialize)]
pub struct ChronicleDataAccessScope_DeniedDataAccessLabels {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    asset_namespace: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_access_label: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ingestion_label: std::option::Option<std::vec::Vec<ChronicleDataAccessScope_DeniedDataAccessLabels_IngestionLabel>>,
    dynamic: ChronicleDataAccessScope_DeniedDataAccessLabelsDynamic,
}

impl ChronicleDataAccessScope_DeniedDataAccessLabels {
    #[doc =
        "Set the field `asset_namespace`.\nThe asset namespace configured in the forwarder\nof the customer's events."]
    pub fn set_asset_namespace(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.asset_namespace = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_access_label`.\nThe name of the data access label."]
    pub fn set_data_access_label(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data_access_label = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_type`.\nThe name of the log type."]
    pub fn set_log_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ingestion_label`.\n"]
    pub fn set_ingestion_label(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ChronicleDataAccessScope_DeniedDataAccessLabels_IngestionLabel>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ingestion_label = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ingestion_label = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleDataAccessScope_DeniedDataAccessLabels {
    type O = terrars::BlockAssignable<ChronicleDataAccessScope_DeniedDataAccessLabels>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleDataAccessScope_DeniedDataAccessLabels {}

impl BuildChronicleDataAccessScope_DeniedDataAccessLabels {
    pub fn build(self) -> ChronicleDataAccessScope_DeniedDataAccessLabels {
        ChronicleDataAccessScope_DeniedDataAccessLabels {
            asset_namespace: core::default::Default::default(),
            data_access_label: core::default::Default::default(),
            log_type: core::default::Default::default(),
            ingestion_label: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleDataAccessScope_DeniedDataAccessLabelsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleDataAccessScope_DeniedDataAccessLabelsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleDataAccessScope_DeniedDataAccessLabelsRef {
        ChronicleDataAccessScope_DeniedDataAccessLabelsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleDataAccessScope_DeniedDataAccessLabelsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `asset_namespace` after provisioning.\nThe asset namespace configured in the forwarder\nof the customer's events."]
    pub fn asset_namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.asset_namespace", self.base))
    }

    #[doc =
        "Get a reference to the value of field `data_access_label` after provisioning.\nThe name of the data access label."]
    pub fn data_access_label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_access_label", self.base))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nOutput only. The display name of the label.\nData access label and log types's name\nwill match the display name of the resource.\nThe asset namespace will match the namespace itself.\nThe ingestion key value pair will match the key of the tuple."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `log_type` after provisioning.\nThe name of the log type."]
    pub fn log_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_type", self.base))
    }

    #[doc = "Get a reference to the value of field `ingestion_label` after provisioning.\n"]
    pub fn ingestion_label(&self) -> terrars::ListRef<ChronicleDataAccessScope_DeniedDataAccessLabels_IngestionLabelRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ingestion_label", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleDataAccessScope_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleDataAccessScope_Timeouts {
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

impl terrars::ToListMappable for ChronicleDataAccessScope_Timeouts {
    type O = terrars::BlockAssignable<ChronicleDataAccessScope_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleDataAccessScope_Timeouts {}

impl BuildChronicleDataAccessScope_Timeouts {
    pub fn build(self) -> ChronicleDataAccessScope_Timeouts {
        ChronicleDataAccessScope_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleDataAccessScope_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleDataAccessScope_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleDataAccessScope_TimeoutsRef {
        ChronicleDataAccessScope_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleDataAccessScope_TimeoutsRef {
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
