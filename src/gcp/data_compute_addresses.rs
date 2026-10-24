use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataComputeAddressesData {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataComputeAddresses_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataComputeAddressesData>,
}

#[derive(Clone)]
pub struct DataComputeAddresses(std::rc::Rc<DataComputeAddresses_>);

impl DataComputeAddresses {
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
        "Set the field `filter`.\nFilter sets the optional parameter \"filter\": A filter expression that\nfilters resources listed in the response. The expression must specify\nthe field name, an operator, and the value that you want to use for\nfiltering. The value must be a string, a number, or a boolean. The\noperator must be either \"=\", \"!=\", \">\", \"<\", \"<=\", \">=\" or \":\". For\nexample, if you are filtering Compute Engine instances, you can\nexclude instances named \"example-instance\" by specifying \"name !=\nexample-instance\". The \":\" operator can be used with string fields to\nmatch substrings. For non-string fields it is equivalent to the \"=\"\noperator. The \":*\" comparison can be used to test whether a key has\nbeen defined. For example, to find all objects with \"owner\" label\nuse: \"\"\" labels.owner:* \"\"\" You can also filter nested fields. For\nexample, you could specify \"scheduling.automaticRestart = false\" to\ninclude instances only if they are not scheduled for automatic\nrestarts. You can use filtering on nested fields to filter based on\nresource labels. To filter on multiple expressions, provide each\nseparate expression within parentheses. For example: \"\"\"\n(scheduling.automaticRestart = true) (cpuPlatform = \"Intel Skylake\")\n\"\"\" By default, each expression is an \"AND\" expression. However, you\ncan include \"AND\" and \"OR\" expressions explicitly. For example: \"\"\"\n(cpuPlatform = \"Intel Skylake\") OR (cpuPlatform = \"Intel Broadwell\")\nAND (scheduling.automaticRestart = true) \"\"\""]
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
        "Set the field `project`.\nThe google project in which addresses are listed. Defaults to provider's configuration if missing."]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion that should be considered to search addresses. All regions are considered if missing."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `addresses` after provisioning.\n"]
    pub fn addresses(&self) -> terrars::ListRef<DataComputeAddressesAddressesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.addresses", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `filter` after provisioning.\nFilter sets the optional parameter \"filter\": A filter expression that\nfilters resources listed in the response. The expression must specify\nthe field name, an operator, and the value that you want to use for\nfiltering. The value must be a string, a number, or a boolean. The\noperator must be either \"=\", \"!=\", \">\", \"<\", \"<=\", \">=\" or \":\". For\nexample, if you are filtering Compute Engine instances, you can\nexclude instances named \"example-instance\" by specifying \"name !=\nexample-instance\". The \":\" operator can be used with string fields to\nmatch substrings. For non-string fields it is equivalent to the \"=\"\noperator. The \":*\" comparison can be used to test whether a key has\nbeen defined. For example, to find all objects with \"owner\" label\nuse: \"\"\" labels.owner:* \"\"\" You can also filter nested fields. For\nexample, you could specify \"scheduling.automaticRestart = false\" to\ninclude instances only if they are not scheduled for automatic\nrestarts. You can use filtering on nested fields to filter based on\nresource labels. To filter on multiple expressions, provide each\nseparate expression within parentheses. For example: \"\"\"\n(scheduling.automaticRestart = true) (cpuPlatform = \"Intel Skylake\")\n\"\"\" By default, each expression is an \"AND\" expression. However, you\ncan include \"AND\" and \"OR\" expressions explicitly. For example: \"\"\"\n(cpuPlatform = \"Intel Skylake\") OR (cpuPlatform = \"Intel Broadwell\")\nAND (scheduling.automaticRestart = true) \"\"\""]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe google project in which addresses are listed. Defaults to provider's configuration if missing."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion that should be considered to search addresses. All regions are considered if missing."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

impl terrars::Referable for DataComputeAddresses {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataComputeAddresses { }

impl terrars::ToListMappable for DataComputeAddresses {
    type O = terrars::ListRef<DataComputeAddressesRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataComputeAddresses_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_compute_addresses".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataComputeAddresses {
    pub tf_id: std::string::String,
}

impl BuildDataComputeAddresses {
    pub fn build(self, stack: &mut terrars::Stack) -> DataComputeAddresses {
        let out = DataComputeAddresses(std::rc::Rc::new(DataComputeAddresses_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataComputeAddressesData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                filter: core::default::Default::default(),
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataComputeAddressesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeAddressesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataComputeAddressesRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `addresses` after provisioning.\n"]
    pub fn addresses(&self) -> terrars::ListRef<DataComputeAddressesAddressesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.addresses", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `filter` after provisioning.\nFilter sets the optional parameter \"filter\": A filter expression that\nfilters resources listed in the response. The expression must specify\nthe field name, an operator, and the value that you want to use for\nfiltering. The value must be a string, a number, or a boolean. The\noperator must be either \"=\", \"!=\", \">\", \"<\", \"<=\", \">=\" or \":\". For\nexample, if you are filtering Compute Engine instances, you can\nexclude instances named \"example-instance\" by specifying \"name !=\nexample-instance\". The \":\" operator can be used with string fields to\nmatch substrings. For non-string fields it is equivalent to the \"=\"\noperator. The \":*\" comparison can be used to test whether a key has\nbeen defined. For example, to find all objects with \"owner\" label\nuse: \"\"\" labels.owner:* \"\"\" You can also filter nested fields. For\nexample, you could specify \"scheduling.automaticRestart = false\" to\ninclude instances only if they are not scheduled for automatic\nrestarts. You can use filtering on nested fields to filter based on\nresource labels. To filter on multiple expressions, provide each\nseparate expression within parentheses. For example: \"\"\"\n(scheduling.automaticRestart = true) (cpuPlatform = \"Intel Skylake\")\n\"\"\" By default, each expression is an \"AND\" expression. However, you\ncan include \"AND\" and \"OR\" expressions explicitly. For example: \"\"\"\n(cpuPlatform = \"Intel Skylake\") OR (cpuPlatform = \"Intel Broadwell\")\nAND (scheduling.automaticRestart = true) \"\"\""]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe google project in which addresses are listed. Defaults to provider's configuration if missing."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion that should be considered to search addresses. All regions are considered if missing."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataComputeAddressesAddressesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    self_link: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeAddressesAddressesEl {
    #[doc = "Set the field `address`.\n"]
    pub fn set_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `address_type`.\n"]
    pub fn set_address_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.address_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `self_link`.\n"]
    pub fn set_self_link(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.self_link = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeAddressesAddressesEl {
    type O = terrars::BlockAssignable<DataComputeAddressesAddressesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeAddressesAddressesEl {}

impl BuildDataComputeAddressesAddressesEl {
    pub fn build(self) -> DataComputeAddressesAddressesEl {
        DataComputeAddressesAddressesEl {
            address: core::default::Default::default(),
            address_type: core::default::Default::default(),
            description: core::default::Default::default(),
            name: core::default::Default::default(),
            region: core::default::Default::default(),
            self_link: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

pub struct DataComputeAddressesAddressesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeAddressesAddressesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataComputeAddressesAddressesElRef {
        DataComputeAddressesAddressesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeAddressesAddressesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address` after provisioning.\n"]
    pub fn address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address", self.base))
    }

    #[doc = "Get a reference to the value of field `address_type` after provisioning.\n"]
    pub fn address_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address_type", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}
