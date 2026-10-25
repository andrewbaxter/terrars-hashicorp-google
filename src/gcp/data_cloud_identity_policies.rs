#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataCloudIdentityPoliciesData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataCloudIdentityPolicies_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataCloudIdentityPoliciesData_>,
}

#[derive(Clone)]
pub struct DataCloudIdentityPolicies(std::rc::Rc<DataCloudIdentityPolicies_>);

impl DataCloudIdentityPolicies {
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

    #[doc =
        "Set the field `filter`.\nFilter expression for listing policies, as documented in the Cloud Identity Policy API policies.list method"]
    pub fn set_filter(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `filter` after provisioning.\nFilter expression for listing policies, as documented in the Cloud Identity Policy API policies.list method"]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `policies` after provisioning.\nList of Cloud Identity policies that match the filter (or all policies if no filter is provided)."]
    pub fn policies(&self) -> terrars::ListRef<DataCloudIdentityPolicies_PoliciesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policies", self.extract_ref()))
    }
}

impl terrars::Referable for DataCloudIdentityPolicies {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataCloudIdentityPolicies { }

impl terrars::ToListMappable for DataCloudIdentityPolicies {
    type O = terrars::ListRef<DataCloudIdentityPoliciesRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataCloudIdentityPolicies_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_cloud_identity_policies".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCloudIdentityPolicies {
    pub tf_id: std::string::String,
}

impl BuildDataCloudIdentityPolicies {
    pub fn build(self, stack: &mut terrars::Stack) -> DataCloudIdentityPolicies {
        let out = DataCloudIdentityPolicies(std::rc::Rc::new(DataCloudIdentityPolicies_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataCloudIdentityPoliciesData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                filter: core::default::Default::default(),
                id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCloudIdentityPoliciesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudIdentityPoliciesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataCloudIdentityPoliciesRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `filter` after provisioning.\nFilter expression for listing policies, as documented in the Cloud Identity Policy API policies.list method"]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `policies` after provisioning.\nList of Cloud Identity policies that match the filter (or all policies if no filter is provided)."]
    pub fn policies(&self) -> terrars::ListRef<DataCloudIdentityPolicies_PoliciesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policies", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataCloudIdentityPolicies_Policies_PolicyQuery {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    org_unit: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sort_order: std::option::Option<terrars::PrimField<f64>>,
}

impl DataCloudIdentityPolicies_Policies_PolicyQuery {
    #[doc = "Set the field `group`.\n"]
    pub fn set_group(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.group = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `org_unit`.\n"]
    pub fn set_org_unit(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.org_unit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `query`.\n"]
    pub fn set_query(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.query = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sort_order`.\n"]
    pub fn set_sort_order(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.sort_order = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudIdentityPolicies_Policies_PolicyQuery {
    type O = terrars::BlockAssignable<DataCloudIdentityPolicies_Policies_PolicyQuery>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudIdentityPolicies_Policies_PolicyQuery {}

impl BuildDataCloudIdentityPolicies_Policies_PolicyQuery {
    pub fn build(self) -> DataCloudIdentityPolicies_Policies_PolicyQuery {
        DataCloudIdentityPolicies_Policies_PolicyQuery {
            group: core::default::Default::default(),
            org_unit: core::default::Default::default(),
            query: core::default::Default::default(),
            sort_order: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudIdentityPolicies_Policies_PolicyQueryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudIdentityPolicies_Policies_PolicyQueryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudIdentityPolicies_Policies_PolicyQueryRef {
        DataCloudIdentityPolicies_Policies_PolicyQueryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudIdentityPolicies_Policies_PolicyQueryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `group` after provisioning.\n"]
    pub fn group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group", self.base))
    }

    #[doc = "Get a reference to the value of field `org_unit` after provisioning.\n"]
    pub fn org_unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.org_unit", self.base))
    }

    #[doc = "Get a reference to the value of field `query` after provisioning.\n"]
    pub fn query(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query", self.base))
    }

    #[doc = "Get a reference to the value of field `sort_order` after provisioning.\n"]
    pub fn sort_order(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sort_order", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudIdentityPolicies_Policies {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customer: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_query: std::option::Option<terrars::ListField<DataCloudIdentityPolicies_Policies_PolicyQuery>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    setting: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudIdentityPolicies_Policies {
    #[doc = "Set the field `customer`.\n"]
    pub fn set_customer(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.customer = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `policy_query`.\n"]
    pub fn set_policy_query(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudIdentityPolicies_Policies_PolicyQuery>>,
    ) -> Self {
        self.policy_query = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `setting`.\n"]
    pub fn set_setting(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.setting = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudIdentityPolicies_Policies {
    type O = terrars::BlockAssignable<DataCloudIdentityPolicies_Policies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudIdentityPolicies_Policies {}

impl BuildDataCloudIdentityPolicies_Policies {
    pub fn build(self) -> DataCloudIdentityPolicies_Policies {
        DataCloudIdentityPolicies_Policies {
            customer: core::default::Default::default(),
            name: core::default::Default::default(),
            policy_query: core::default::Default::default(),
            setting: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudIdentityPolicies_PoliciesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudIdentityPolicies_PoliciesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudIdentityPolicies_PoliciesRef {
        DataCloudIdentityPolicies_PoliciesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudIdentityPolicies_PoliciesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `customer` after provisioning.\n"]
    pub fn customer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `policy_query` after provisioning.\n"]
    pub fn policy_query(&self) -> terrars::ListRef<DataCloudIdentityPolicies_Policies_PolicyQueryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policy_query", self.base))
    }

    #[doc = "Get a reference to the value of field `setting` after provisioning.\n"]
    pub fn setting(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.setting", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
