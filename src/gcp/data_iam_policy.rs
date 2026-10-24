use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataIamPolicyData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audit_config: std::option::Option<std::vec::Vec<DataIamPolicyAuditConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    binding: std::option::Option<std::vec::Vec<DataIamPolicyBindingEl>>,
    dynamic: DataIamPolicyDynamic,
}

struct DataIamPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataIamPolicyData>,
}

#[derive(Clone)]
pub struct DataIamPolicy(std::rc::Rc<DataIamPolicy_>);

impl DataIamPolicy {
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

    #[doc = "Set the field `audit_config`.\n"]
    pub fn set_audit_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataIamPolicyAuditConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().audit_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.audit_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `binding`.\n"]
    pub fn set_binding(self, v: impl std::convert::Into<terrars::BlockAssignable<DataIamPolicyBindingEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().binding = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.binding = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy_data` after provisioning.\n"]
    pub fn policy_data(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_data", self.extract_ref()))
    }
}

impl terrars::Referable for DataIamPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataIamPolicy { }

impl terrars::ToListMappable for DataIamPolicy {
    type O = terrars::ListRef<DataIamPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataIamPolicy_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_iam_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataIamPolicy {
    pub tf_id: std::string::String,
}

impl BuildDataIamPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> DataIamPolicy {
        let out = DataIamPolicy(std::rc::Rc::new(DataIamPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataIamPolicyData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                audit_config: core::default::Default::default(),
                binding: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataIamPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIamPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataIamPolicyRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy_data` after provisioning.\n"]
    pub fn policy_data(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_data", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataIamPolicyAuditConfigElAuditLogConfigsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exempted_members: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    log_type: terrars::PrimField<std::string::String>,
}

impl DataIamPolicyAuditConfigElAuditLogConfigsEl {
    #[doc = "Set the field `exempted_members`.\n"]
    pub fn set_exempted_members(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.exempted_members = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataIamPolicyAuditConfigElAuditLogConfigsEl {
    type O = terrars::BlockAssignable<DataIamPolicyAuditConfigElAuditLogConfigsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIamPolicyAuditConfigElAuditLogConfigsEl {
    #[doc = ""]
    pub log_type: terrars::PrimField<std::string::String>,
}

impl BuildDataIamPolicyAuditConfigElAuditLogConfigsEl {
    pub fn build(self) -> DataIamPolicyAuditConfigElAuditLogConfigsEl {
        DataIamPolicyAuditConfigElAuditLogConfigsEl {
            exempted_members: core::default::Default::default(),
            log_type: self.log_type,
        }
    }
}

pub struct DataIamPolicyAuditConfigElAuditLogConfigsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIamPolicyAuditConfigElAuditLogConfigsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataIamPolicyAuditConfigElAuditLogConfigsElRef {
        DataIamPolicyAuditConfigElAuditLogConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIamPolicyAuditConfigElAuditLogConfigsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exempted_members` after provisioning.\n"]
    pub fn exempted_members(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.exempted_members", self.base))
    }

    #[doc = "Get a reference to the value of field `log_type` after provisioning.\n"]
    pub fn log_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataIamPolicyAuditConfigElDynamic {
    audit_log_configs: std::option::Option<terrars::DynamicBlock<DataIamPolicyAuditConfigElAuditLogConfigsEl>>,
}

#[derive(Serialize)]
pub struct DataIamPolicyAuditConfigEl {
    service: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audit_log_configs: std::option::Option<std::vec::Vec<DataIamPolicyAuditConfigElAuditLogConfigsEl>>,
    dynamic: DataIamPolicyAuditConfigElDynamic,
}

impl DataIamPolicyAuditConfigEl {
    #[doc = "Set the field `audit_log_configs`.\n"]
    pub fn set_audit_log_configs(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataIamPolicyAuditConfigElAuditLogConfigsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.audit_log_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.audit_log_configs = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataIamPolicyAuditConfigEl {
    type O = terrars::BlockAssignable<DataIamPolicyAuditConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIamPolicyAuditConfigEl {
    #[doc = ""]
    pub service: terrars::PrimField<std::string::String>,
}

impl BuildDataIamPolicyAuditConfigEl {
    pub fn build(self) -> DataIamPolicyAuditConfigEl {
        DataIamPolicyAuditConfigEl {
            service: self.service,
            audit_log_configs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataIamPolicyAuditConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIamPolicyAuditConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataIamPolicyAuditConfigElRef {
        DataIamPolicyAuditConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIamPolicyAuditConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}

#[derive(Serialize)]
pub struct DataIamPolicyBindingElConditionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    expression: terrars::PrimField<std::string::String>,
    title: terrars::PrimField<std::string::String>,
}

impl DataIamPolicyBindingElConditionEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataIamPolicyBindingElConditionEl {
    type O = terrars::BlockAssignable<DataIamPolicyBindingElConditionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIamPolicyBindingElConditionEl {
    #[doc = ""]
    pub expression: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub title: terrars::PrimField<std::string::String>,
}

impl BuildDataIamPolicyBindingElConditionEl {
    pub fn build(self) -> DataIamPolicyBindingElConditionEl {
        DataIamPolicyBindingElConditionEl {
            description: core::default::Default::default(),
            expression: self.expression,
            title: self.title,
        }
    }
}

pub struct DataIamPolicyBindingElConditionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIamPolicyBindingElConditionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataIamPolicyBindingElConditionElRef {
        DataIamPolicyBindingElConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIamPolicyBindingElConditionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `expression` after provisioning.\n"]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc = "Get a reference to the value of field `title` after provisioning.\n"]
    pub fn title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataIamPolicyBindingElDynamic {
    condition: std::option::Option<terrars::DynamicBlock<DataIamPolicyBindingElConditionEl>>,
}

#[derive(Serialize)]
pub struct DataIamPolicyBindingEl {
    members: terrars::SetField<terrars::PrimField<std::string::String>>,
    role: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition: std::option::Option<std::vec::Vec<DataIamPolicyBindingElConditionEl>>,
    dynamic: DataIamPolicyBindingElDynamic,
}

impl DataIamPolicyBindingEl {
    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataIamPolicyBindingElConditionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.condition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.condition = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataIamPolicyBindingEl {
    type O = terrars::BlockAssignable<DataIamPolicyBindingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIamPolicyBindingEl {
    #[doc = ""]
    pub members: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub role: terrars::PrimField<std::string::String>,
}

impl BuildDataIamPolicyBindingEl {
    pub fn build(self) -> DataIamPolicyBindingEl {
        DataIamPolicyBindingEl {
            members: self.members,
            role: self.role,
            condition: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataIamPolicyBindingElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIamPolicyBindingElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataIamPolicyBindingElRef {
        DataIamPolicyBindingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIamPolicyBindingElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `members` after provisioning.\n"]
    pub fn members(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.members", self.base))
    }

    #[doc = "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role", self.base))
    }

    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> terrars::ListRef<DataIamPolicyBindingElConditionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.condition", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataIamPolicyDynamic {
    audit_config: std::option::Option<terrars::DynamicBlock<DataIamPolicyAuditConfigEl>>,
    binding: std::option::Option<terrars::DynamicBlock<DataIamPolicyBindingEl>>,
}
