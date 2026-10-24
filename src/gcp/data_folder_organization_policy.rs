use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataFolderOrganizationPolicyData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    constraint: terrars::PrimField<std::string::String>,
    folder: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataFolderOrganizationPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataFolderOrganizationPolicyData>,
}

#[derive(Clone)]
pub struct DataFolderOrganizationPolicy(std::rc::Rc<DataFolderOrganizationPolicy_>);

impl DataFolderOrganizationPolicy {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `boolean_policy` after provisioning.\nA boolean policy is a constraint that is either enforced or not."]
    pub fn boolean_policy(&self) -> terrars::ListRef<DataFolderOrganizationPolicyBooleanPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.boolean_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `constraint` after provisioning.\nThe name of the Constraint the Policy is configuring, for example, serviceuser.services."]
    pub fn constraint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.constraint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nThe etag of the organization policy. etag is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `folder` after provisioning.\nThe resource name of the folder to set the policy for. Its format is folders/{folder_id}."]
    pub fn folder(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.folder", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `list_policy` after provisioning.\nA policy that can define specific values that are allowed or denied for the given constraint. It can also be used to allow or deny all values. "]
    pub fn list_policy(&self) -> terrars::ListRef<DataFolderOrganizationPolicyListPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.list_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `restore_policy` after provisioning.\nA restore policy is a constraint to restore the default policy."]
    pub fn restore_policy(&self) -> terrars::ListRef<DataFolderOrganizationPolicyRestorePolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.restore_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds, representing when the variable was last updated. Example: \"2016-10-09T12:33:37.578138407Z\"."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nVersion of the Policy. Default version is 0."]
    pub fn version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }
}

impl terrars::Referable for DataFolderOrganizationPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataFolderOrganizationPolicy { }

impl terrars::ToListMappable for DataFolderOrganizationPolicy {
    type O = terrars::ListRef<DataFolderOrganizationPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataFolderOrganizationPolicy_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_folder_organization_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataFolderOrganizationPolicy {
    pub tf_id: std::string::String,
    #[doc = "The name of the Constraint the Policy is configuring, for example, serviceuser.services."]
    pub constraint: terrars::PrimField<std::string::String>,
    #[doc = "The resource name of the folder to set the policy for. Its format is folders/{folder_id}."]
    pub folder: terrars::PrimField<std::string::String>,
}

impl BuildDataFolderOrganizationPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> DataFolderOrganizationPolicy {
        let out = DataFolderOrganizationPolicy(std::rc::Rc::new(DataFolderOrganizationPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataFolderOrganizationPolicyData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                constraint: self.constraint,
                folder: self.folder,
                id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataFolderOrganizationPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFolderOrganizationPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataFolderOrganizationPolicyRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `boolean_policy` after provisioning.\nA boolean policy is a constraint that is either enforced or not."]
    pub fn boolean_policy(&self) -> terrars::ListRef<DataFolderOrganizationPolicyBooleanPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.boolean_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `constraint` after provisioning.\nThe name of the Constraint the Policy is configuring, for example, serviceuser.services."]
    pub fn constraint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.constraint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nThe etag of the organization policy. etag is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `folder` after provisioning.\nThe resource name of the folder to set the policy for. Its format is folders/{folder_id}."]
    pub fn folder(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.folder", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `list_policy` after provisioning.\nA policy that can define specific values that are allowed or denied for the given constraint. It can also be used to allow or deny all values. "]
    pub fn list_policy(&self) -> terrars::ListRef<DataFolderOrganizationPolicyListPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.list_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `restore_policy` after provisioning.\nA restore policy is a constraint to restore the default policy."]
    pub fn restore_policy(&self) -> terrars::ListRef<DataFolderOrganizationPolicyRestorePolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.restore_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds, representing when the variable was last updated. Example: \"2016-10-09T12:33:37.578138407Z\"."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nVersion of the Policy. Default version is 0."]
    pub fn version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataFolderOrganizationPolicyBooleanPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enforced: std::option::Option<terrars::PrimField<bool>>,
}

impl DataFolderOrganizationPolicyBooleanPolicyEl {
    #[doc = "Set the field `enforced`.\n"]
    pub fn set_enforced(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enforced = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataFolderOrganizationPolicyBooleanPolicyEl {
    type O = terrars::BlockAssignable<DataFolderOrganizationPolicyBooleanPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFolderOrganizationPolicyBooleanPolicyEl {}

impl BuildDataFolderOrganizationPolicyBooleanPolicyEl {
    pub fn build(self) -> DataFolderOrganizationPolicyBooleanPolicyEl {
        DataFolderOrganizationPolicyBooleanPolicyEl { enforced: core::default::Default::default() }
    }
}

pub struct DataFolderOrganizationPolicyBooleanPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFolderOrganizationPolicyBooleanPolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataFolderOrganizationPolicyBooleanPolicyElRef {
        DataFolderOrganizationPolicyBooleanPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFolderOrganizationPolicyBooleanPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enforced` after provisioning.\n"]
    pub fn enforced(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enforced", self.base))
    }
}

#[derive(Serialize)]
pub struct DataFolderOrganizationPolicyListPolicyElAllowEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    all: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataFolderOrganizationPolicyListPolicyElAllowEl {
    #[doc = "Set the field `all`.\n"]
    pub fn set_all(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.all = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.values = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataFolderOrganizationPolicyListPolicyElAllowEl {
    type O = terrars::BlockAssignable<DataFolderOrganizationPolicyListPolicyElAllowEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFolderOrganizationPolicyListPolicyElAllowEl {}

impl BuildDataFolderOrganizationPolicyListPolicyElAllowEl {
    pub fn build(self) -> DataFolderOrganizationPolicyListPolicyElAllowEl {
        DataFolderOrganizationPolicyListPolicyElAllowEl {
            all: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataFolderOrganizationPolicyListPolicyElAllowElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFolderOrganizationPolicyListPolicyElAllowElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataFolderOrganizationPolicyListPolicyElAllowElRef {
        DataFolderOrganizationPolicyListPolicyElAllowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFolderOrganizationPolicyListPolicyElAllowElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `all` after provisioning.\n"]
    pub fn all(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.all", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataFolderOrganizationPolicyListPolicyElDenyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    all: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataFolderOrganizationPolicyListPolicyElDenyEl {
    #[doc = "Set the field `all`.\n"]
    pub fn set_all(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.all = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.values = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataFolderOrganizationPolicyListPolicyElDenyEl {
    type O = terrars::BlockAssignable<DataFolderOrganizationPolicyListPolicyElDenyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFolderOrganizationPolicyListPolicyElDenyEl {}

impl BuildDataFolderOrganizationPolicyListPolicyElDenyEl {
    pub fn build(self) -> DataFolderOrganizationPolicyListPolicyElDenyEl {
        DataFolderOrganizationPolicyListPolicyElDenyEl {
            all: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataFolderOrganizationPolicyListPolicyElDenyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFolderOrganizationPolicyListPolicyElDenyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataFolderOrganizationPolicyListPolicyElDenyElRef {
        DataFolderOrganizationPolicyListPolicyElDenyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFolderOrganizationPolicyListPolicyElDenyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `all` after provisioning.\n"]
    pub fn all(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.all", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataFolderOrganizationPolicyListPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow: std::option::Option<terrars::ListField<DataFolderOrganizationPolicyListPolicyElAllowEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deny: std::option::Option<terrars::ListField<DataFolderOrganizationPolicyListPolicyElDenyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inherit_from_parent: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    suggested_value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataFolderOrganizationPolicyListPolicyEl {
    #[doc = "Set the field `allow`.\n"]
    pub fn set_allow(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataFolderOrganizationPolicyListPolicyElAllowEl>>,
    ) -> Self {
        self.allow = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deny`.\n"]
    pub fn set_deny(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataFolderOrganizationPolicyListPolicyElDenyEl>>,
    ) -> Self {
        self.deny = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `inherit_from_parent`.\n"]
    pub fn set_inherit_from_parent(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.inherit_from_parent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `suggested_value`.\n"]
    pub fn set_suggested_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.suggested_value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataFolderOrganizationPolicyListPolicyEl {
    type O = terrars::BlockAssignable<DataFolderOrganizationPolicyListPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFolderOrganizationPolicyListPolicyEl {}

impl BuildDataFolderOrganizationPolicyListPolicyEl {
    pub fn build(self) -> DataFolderOrganizationPolicyListPolicyEl {
        DataFolderOrganizationPolicyListPolicyEl {
            allow: core::default::Default::default(),
            deny: core::default::Default::default(),
            inherit_from_parent: core::default::Default::default(),
            suggested_value: core::default::Default::default(),
        }
    }
}

pub struct DataFolderOrganizationPolicyListPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFolderOrganizationPolicyListPolicyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataFolderOrganizationPolicyListPolicyElRef {
        DataFolderOrganizationPolicyListPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFolderOrganizationPolicyListPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow` after provisioning.\n"]
    pub fn allow(&self) -> terrars::ListRef<DataFolderOrganizationPolicyListPolicyElAllowElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allow", self.base))
    }

    #[doc = "Get a reference to the value of field `deny` after provisioning.\n"]
    pub fn deny(&self) -> terrars::ListRef<DataFolderOrganizationPolicyListPolicyElDenyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deny", self.base))
    }

    #[doc = "Get a reference to the value of field `inherit_from_parent` after provisioning.\n"]
    pub fn inherit_from_parent(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inherit_from_parent", self.base))
    }

    #[doc = "Get a reference to the value of field `suggested_value` after provisioning.\n"]
    pub fn suggested_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.suggested_value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataFolderOrganizationPolicyRestorePolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default: std::option::Option<terrars::PrimField<bool>>,
}

impl DataFolderOrganizationPolicyRestorePolicyEl {
    #[doc = "Set the field `default`.\n"]
    pub fn set_default(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.default = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataFolderOrganizationPolicyRestorePolicyEl {
    type O = terrars::BlockAssignable<DataFolderOrganizationPolicyRestorePolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFolderOrganizationPolicyRestorePolicyEl {}

impl BuildDataFolderOrganizationPolicyRestorePolicyEl {
    pub fn build(self) -> DataFolderOrganizationPolicyRestorePolicyEl {
        DataFolderOrganizationPolicyRestorePolicyEl { default: core::default::Default::default() }
    }
}

pub struct DataFolderOrganizationPolicyRestorePolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFolderOrganizationPolicyRestorePolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataFolderOrganizationPolicyRestorePolicyElRef {
        DataFolderOrganizationPolicyRestorePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFolderOrganizationPolicyRestorePolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default` after provisioning.\n"]
    pub fn default(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default", self.base))
    }
}
