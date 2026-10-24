use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataProjectOrganizationPolicyData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    constraint: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    project: terrars::PrimField<std::string::String>,
}

struct DataProjectOrganizationPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataProjectOrganizationPolicyData>,
}

#[derive(Clone)]
pub struct DataProjectOrganizationPolicy(std::rc::Rc<DataProjectOrganizationPolicy_>);

impl DataProjectOrganizationPolicy {
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
    pub fn boolean_policy(&self) -> terrars::ListRef<DataProjectOrganizationPolicyBooleanPolicyElRef> {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `list_policy` after provisioning.\nA policy that can define specific values that are allowed or denied for the given constraint. It can also be used to allow or deny all values. "]
    pub fn list_policy(&self) -> terrars::ListRef<DataProjectOrganizationPolicyListPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.list_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project ID."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `restore_policy` after provisioning.\nA restore policy is a constraint to restore the default policy."]
    pub fn restore_policy(&self) -> terrars::ListRef<DataProjectOrganizationPolicyRestorePolicyElRef> {
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

impl terrars::Referable for DataProjectOrganizationPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataProjectOrganizationPolicy { }

impl terrars::ToListMappable for DataProjectOrganizationPolicy {
    type O = terrars::ListRef<DataProjectOrganizationPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataProjectOrganizationPolicy_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_project_organization_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataProjectOrganizationPolicy {
    pub tf_id: std::string::String,
    #[doc = "The name of the Constraint the Policy is configuring, for example, serviceuser.services."]
    pub constraint: terrars::PrimField<std::string::String>,
    #[doc = "The project ID."]
    pub project: terrars::PrimField<std::string::String>,
}

impl BuildDataProjectOrganizationPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> DataProjectOrganizationPolicy {
        let out = DataProjectOrganizationPolicy(std::rc::Rc::new(DataProjectOrganizationPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataProjectOrganizationPolicyData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                constraint: self.constraint,
                id: core::default::Default::default(),
                project: self.project,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataProjectOrganizationPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataProjectOrganizationPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataProjectOrganizationPolicyRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `boolean_policy` after provisioning.\nA boolean policy is a constraint that is either enforced or not."]
    pub fn boolean_policy(&self) -> terrars::ListRef<DataProjectOrganizationPolicyBooleanPolicyElRef> {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `list_policy` after provisioning.\nA policy that can define specific values that are allowed or denied for the given constraint. It can also be used to allow or deny all values. "]
    pub fn list_policy(&self) -> terrars::ListRef<DataProjectOrganizationPolicyListPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.list_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project ID."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `restore_policy` after provisioning.\nA restore policy is a constraint to restore the default policy."]
    pub fn restore_policy(&self) -> terrars::ListRef<DataProjectOrganizationPolicyRestorePolicyElRef> {
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
pub struct DataProjectOrganizationPolicyBooleanPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enforced: std::option::Option<terrars::PrimField<bool>>,
}

impl DataProjectOrganizationPolicyBooleanPolicyEl {
    #[doc = "Set the field `enforced`.\n"]
    pub fn set_enforced(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enforced = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataProjectOrganizationPolicyBooleanPolicyEl {
    type O = terrars::BlockAssignable<DataProjectOrganizationPolicyBooleanPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataProjectOrganizationPolicyBooleanPolicyEl {}

impl BuildDataProjectOrganizationPolicyBooleanPolicyEl {
    pub fn build(self) -> DataProjectOrganizationPolicyBooleanPolicyEl {
        DataProjectOrganizationPolicyBooleanPolicyEl { enforced: core::default::Default::default() }
    }
}

pub struct DataProjectOrganizationPolicyBooleanPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataProjectOrganizationPolicyBooleanPolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataProjectOrganizationPolicyBooleanPolicyElRef {
        DataProjectOrganizationPolicyBooleanPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataProjectOrganizationPolicyBooleanPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enforced` after provisioning.\n"]
    pub fn enforced(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enforced", self.base))
    }
}

#[derive(Serialize)]
pub struct DataProjectOrganizationPolicyListPolicyElAllowEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    all: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataProjectOrganizationPolicyListPolicyElAllowEl {
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

impl terrars::ToListMappable for DataProjectOrganizationPolicyListPolicyElAllowEl {
    type O = terrars::BlockAssignable<DataProjectOrganizationPolicyListPolicyElAllowEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataProjectOrganizationPolicyListPolicyElAllowEl {}

impl BuildDataProjectOrganizationPolicyListPolicyElAllowEl {
    pub fn build(self) -> DataProjectOrganizationPolicyListPolicyElAllowEl {
        DataProjectOrganizationPolicyListPolicyElAllowEl {
            all: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataProjectOrganizationPolicyListPolicyElAllowElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataProjectOrganizationPolicyListPolicyElAllowElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataProjectOrganizationPolicyListPolicyElAllowElRef {
        DataProjectOrganizationPolicyListPolicyElAllowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataProjectOrganizationPolicyListPolicyElAllowElRef {
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
pub struct DataProjectOrganizationPolicyListPolicyElDenyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    all: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataProjectOrganizationPolicyListPolicyElDenyEl {
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

impl terrars::ToListMappable for DataProjectOrganizationPolicyListPolicyElDenyEl {
    type O = terrars::BlockAssignable<DataProjectOrganizationPolicyListPolicyElDenyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataProjectOrganizationPolicyListPolicyElDenyEl {}

impl BuildDataProjectOrganizationPolicyListPolicyElDenyEl {
    pub fn build(self) -> DataProjectOrganizationPolicyListPolicyElDenyEl {
        DataProjectOrganizationPolicyListPolicyElDenyEl {
            all: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataProjectOrganizationPolicyListPolicyElDenyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataProjectOrganizationPolicyListPolicyElDenyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataProjectOrganizationPolicyListPolicyElDenyElRef {
        DataProjectOrganizationPolicyListPolicyElDenyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataProjectOrganizationPolicyListPolicyElDenyElRef {
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
pub struct DataProjectOrganizationPolicyListPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow: std::option::Option<terrars::ListField<DataProjectOrganizationPolicyListPolicyElAllowEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deny: std::option::Option<terrars::ListField<DataProjectOrganizationPolicyListPolicyElDenyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inherit_from_parent: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    suggested_value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataProjectOrganizationPolicyListPolicyEl {
    #[doc = "Set the field `allow`.\n"]
    pub fn set_allow(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataProjectOrganizationPolicyListPolicyElAllowEl>>,
    ) -> Self {
        self.allow = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deny`.\n"]
    pub fn set_deny(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataProjectOrganizationPolicyListPolicyElDenyEl>>,
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

impl terrars::ToListMappable for DataProjectOrganizationPolicyListPolicyEl {
    type O = terrars::BlockAssignable<DataProjectOrganizationPolicyListPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataProjectOrganizationPolicyListPolicyEl {}

impl BuildDataProjectOrganizationPolicyListPolicyEl {
    pub fn build(self) -> DataProjectOrganizationPolicyListPolicyEl {
        DataProjectOrganizationPolicyListPolicyEl {
            allow: core::default::Default::default(),
            deny: core::default::Default::default(),
            inherit_from_parent: core::default::Default::default(),
            suggested_value: core::default::Default::default(),
        }
    }
}

pub struct DataProjectOrganizationPolicyListPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataProjectOrganizationPolicyListPolicyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataProjectOrganizationPolicyListPolicyElRef {
        DataProjectOrganizationPolicyListPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataProjectOrganizationPolicyListPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow` after provisioning.\n"]
    pub fn allow(&self) -> terrars::ListRef<DataProjectOrganizationPolicyListPolicyElAllowElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allow", self.base))
    }

    #[doc = "Get a reference to the value of field `deny` after provisioning.\n"]
    pub fn deny(&self) -> terrars::ListRef<DataProjectOrganizationPolicyListPolicyElDenyElRef> {
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
pub struct DataProjectOrganizationPolicyRestorePolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default: std::option::Option<terrars::PrimField<bool>>,
}

impl DataProjectOrganizationPolicyRestorePolicyEl {
    #[doc = "Set the field `default`.\n"]
    pub fn set_default(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.default = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataProjectOrganizationPolicyRestorePolicyEl {
    type O = terrars::BlockAssignable<DataProjectOrganizationPolicyRestorePolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataProjectOrganizationPolicyRestorePolicyEl {}

impl BuildDataProjectOrganizationPolicyRestorePolicyEl {
    pub fn build(self) -> DataProjectOrganizationPolicyRestorePolicyEl {
        DataProjectOrganizationPolicyRestorePolicyEl { default: core::default::Default::default() }
    }
}

pub struct DataProjectOrganizationPolicyRestorePolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataProjectOrganizationPolicyRestorePolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataProjectOrganizationPolicyRestorePolicyElRef {
        DataProjectOrganizationPolicyRestorePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataProjectOrganizationPolicyRestorePolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default` after provisioning.\n"]
    pub fn default(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default", self.base))
    }
}
