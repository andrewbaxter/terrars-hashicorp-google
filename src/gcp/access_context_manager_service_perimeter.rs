use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct AccessContextManagerServicePerimeterData {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    parent: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    perimeter_type: std::option::Option<terrars::PrimField<std::string::String>>,
    title: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_explicit_dry_run_spec: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spec: std::option::Option<std::vec::Vec<AccessContextManagerServicePerimeterSpecEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<std::vec::Vec<AccessContextManagerServicePerimeterStatusEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<AccessContextManagerServicePerimeterTimeoutsEl>,
    dynamic: AccessContextManagerServicePerimeterDynamic,
}

struct AccessContextManagerServicePerimeter_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AccessContextManagerServicePerimeterData>,
}

#[derive(Clone)]
pub struct AccessContextManagerServicePerimeter(std::rc::Rc<AccessContextManagerServicePerimeter_>);

impl AccessContextManagerServicePerimeter {
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

    #[doc = "Set the field `description`.\nDescription of the ServicePerimeter and its use. Does not affect\nbehavior."]
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
        "Set the field `perimeter_type`.\nSpecifies the type of the Perimeter. There are two types: regular and\nbridge. Regular Service Perimeter contains resources, access levels,\nand restricted services. Every resource can be in at most\nONE regular Service Perimeter.\n\nIn addition to being in a regular service perimeter, a resource can also\nbe in zero or more perimeter bridges. A perimeter bridge only contains\nresources. Cross project operations are permitted if all effected\nresources share some perimeter (whether bridge or regular). Perimeter\nBridge does not contain access levels or services: those are governed\nentirely by the regular perimeter that resource is in.\n\nPerimeter Bridges are typically useful when building more complex\ntopologies with many independent perimeters that need to share some data\nwith a common perimeter, but should not be able to share data among\nthemselves. Default value: \"PERIMETER_TYPE_REGULAR\" Possible values: [\"PERIMETER_TYPE_REGULAR\", \"PERIMETER_TYPE_BRIDGE\"]"]
    pub fn set_perimeter_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().perimeter_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `use_explicit_dry_run_spec`.\nUse explicit dry run spec flag. Ordinarily, a dry-run spec implicitly exists\nfor all Service Perimeters, and that spec is identical to the status for those\nService Perimeters. When this flag is set, it inhibits the generation of the\nimplicit spec, thereby allowing the user to explicitly provide a\nconfiguration (\"spec\") to use in a dry-run version of the Service Perimeter.\nThis allows the user to test changes to the enforced config (\"status\") without\nactually enforcing them. This testing is done through analyzing the differences\nbetween currently enforced and suggested restrictions. useExplicitDryRunSpec must\nbet set to True if any of the fields in the spec are set to non-default values."]
    pub fn set_use_explicit_dry_run_spec(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().use_explicit_dry_run_spec = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `spec`.\n"]
    pub fn set_spec(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AccessContextManagerServicePerimeterSpecEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AccessContextManagerServicePerimeterStatusEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().status = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.status = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<AccessContextManagerServicePerimeterTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTime the AccessPolicy was created in UTC."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nDescription of the ServicePerimeter and its use. Does not affect\nbehavior."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nResource name for the ServicePerimeter. The short_name component must\nbegin with a letter and only include alphanumeric and '_'.\nFormat: accessPolicies/{policy_id}/servicePerimeters/{short_name}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nThe AccessPolicy this ServicePerimeter lives in.\nFormat: accessPolicies/{policy_id}"]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `perimeter_type` after provisioning.\nSpecifies the type of the Perimeter. There are two types: regular and\nbridge. Regular Service Perimeter contains resources, access levels,\nand restricted services. Every resource can be in at most\nONE regular Service Perimeter.\n\nIn addition to being in a regular service perimeter, a resource can also\nbe in zero or more perimeter bridges. A perimeter bridge only contains\nresources. Cross project operations are permitted if all effected\nresources share some perimeter (whether bridge or regular). Perimeter\nBridge does not contain access levels or services: those are governed\nentirely by the regular perimeter that resource is in.\n\nPerimeter Bridges are typically useful when building more complex\ntopologies with many independent perimeters that need to share some data\nwith a common perimeter, but should not be able to share data among\nthemselves. Default value: \"PERIMETER_TYPE_REGULAR\" Possible values: [\"PERIMETER_TYPE_REGULAR\", \"PERIMETER_TYPE_BRIDGE\"]"]
    pub fn perimeter_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.perimeter_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `title` after provisioning.\nHuman readable title. Must be unique within the Policy."]
    pub fn title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.title", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTime the AccessPolicy was updated in UTC."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `use_explicit_dry_run_spec` after provisioning.\nUse explicit dry run spec flag. Ordinarily, a dry-run spec implicitly exists\nfor all Service Perimeters, and that spec is identical to the status for those\nService Perimeters. When this flag is set, it inhibits the generation of the\nimplicit spec, thereby allowing the user to explicitly provide a\nconfiguration (\"spec\") to use in a dry-run version of the Service Perimeter.\nThis allows the user to test changes to the enforced config (\"status\") without\nactually enforcing them. This testing is done through analyzing the differences\nbetween currently enforced and suggested restrictions. useExplicitDryRunSpec must\nbet set to True if any of the fields in the spec are set to non-default values."]
    pub fn use_explicit_dry_run_spec(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_explicit_dry_run_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> terrars::ListRef<AccessContextManagerServicePerimeterSpecElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::ListRef<AccessContextManagerServicePerimeterStatusElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AccessContextManagerServicePerimeterTimeoutsElRef {
        AccessContextManagerServicePerimeterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for AccessContextManagerServicePerimeter {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AccessContextManagerServicePerimeter { }

impl terrars::ToListMappable for AccessContextManagerServicePerimeter {
    type O = terrars::ListRef<AccessContextManagerServicePerimeterRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AccessContextManagerServicePerimeter_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_access_context_manager_service_perimeter".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAccessContextManagerServicePerimeter {
    pub tf_id: std::string::String,
    #[doc =
        "Resource name for the ServicePerimeter. The short_name component must\nbegin with a letter and only include alphanumeric and '_'.\nFormat: accessPolicies/{policy_id}/servicePerimeters/{short_name}"]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The AccessPolicy this ServicePerimeter lives in.\nFormat: accessPolicies/{policy_id}"]
    pub parent: terrars::PrimField<std::string::String>,
    #[doc = "Human readable title. Must be unique within the Policy."]
    pub title: terrars::PrimField<std::string::String>,
}

impl BuildAccessContextManagerServicePerimeter {
    pub fn build(self, stack: &mut terrars::Stack) -> AccessContextManagerServicePerimeter {
        let out = AccessContextManagerServicePerimeter(std::rc::Rc::new(AccessContextManagerServicePerimeter_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AccessContextManagerServicePerimeterData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                parent: self.parent,
                perimeter_type: core::default::Default::default(),
                title: self.title,
                use_explicit_dry_run_spec: core::default::Default::default(),
                spec: core::default::Default::default(),
                status: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AccessContextManagerServicePerimeterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AccessContextManagerServicePerimeterRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTime the AccessPolicy was created in UTC."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nDescription of the ServicePerimeter and its use. Does not affect\nbehavior."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nResource name for the ServicePerimeter. The short_name component must\nbegin with a letter and only include alphanumeric and '_'.\nFormat: accessPolicies/{policy_id}/servicePerimeters/{short_name}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nThe AccessPolicy this ServicePerimeter lives in.\nFormat: accessPolicies/{policy_id}"]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `perimeter_type` after provisioning.\nSpecifies the type of the Perimeter. There are two types: regular and\nbridge. Regular Service Perimeter contains resources, access levels,\nand restricted services. Every resource can be in at most\nONE regular Service Perimeter.\n\nIn addition to being in a regular service perimeter, a resource can also\nbe in zero or more perimeter bridges. A perimeter bridge only contains\nresources. Cross project operations are permitted if all effected\nresources share some perimeter (whether bridge or regular). Perimeter\nBridge does not contain access levels or services: those are governed\nentirely by the regular perimeter that resource is in.\n\nPerimeter Bridges are typically useful when building more complex\ntopologies with many independent perimeters that need to share some data\nwith a common perimeter, but should not be able to share data among\nthemselves. Default value: \"PERIMETER_TYPE_REGULAR\" Possible values: [\"PERIMETER_TYPE_REGULAR\", \"PERIMETER_TYPE_BRIDGE\"]"]
    pub fn perimeter_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.perimeter_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `title` after provisioning.\nHuman readable title. Must be unique within the Policy."]
    pub fn title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.title", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTime the AccessPolicy was updated in UTC."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `use_explicit_dry_run_spec` after provisioning.\nUse explicit dry run spec flag. Ordinarily, a dry-run spec implicitly exists\nfor all Service Perimeters, and that spec is identical to the status for those\nService Perimeters. When this flag is set, it inhibits the generation of the\nimplicit spec, thereby allowing the user to explicitly provide a\nconfiguration (\"spec\") to use in a dry-run version of the Service Perimeter.\nThis allows the user to test changes to the enforced config (\"status\") without\nactually enforcing them. This testing is done through analyzing the differences\nbetween currently enforced and suggested restrictions. useExplicitDryRunSpec must\nbet set to True if any of the fields in the spec are set to non-default values."]
    pub fn use_explicit_dry_run_spec(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_explicit_dry_run_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> terrars::ListRef<AccessContextManagerServicePerimeterSpecElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::ListRef<AccessContextManagerServicePerimeterStatusElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AccessContextManagerServicePerimeterTimeoutsElRef {
        AccessContextManagerServicePerimeterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromElSourcesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_level: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromElSourcesEl {
    #[doc =
        "Set the field `access_level`.\nAn AccessLevel resource name that allows resources outside the ServicePerimeter to be accessed from the inside."]
    pub fn set_access_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.access_level = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromElSourcesEl {
    type O =
        terrars::BlockAssignable<AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromElSourcesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromElSourcesEl {}

impl BuildAccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromElSourcesEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromElSourcesEl {
        AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromElSourcesEl {
            access_level: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromElSourcesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromElSourcesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromElSourcesElRef {
        AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromElSourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromElSourcesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `access_level` after provisioning.\nAn AccessLevel resource name that allows resources outside the ServicePerimeter to be accessed from the inside."]
    pub fn access_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_level", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromElDynamic {
    sources: std
    ::option
    ::Option<terrars::DynamicBlock<AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromElSourcesEl>>,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identities: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_restriction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sources: std
    ::option
    ::Option<std::vec::Vec<AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromElSourcesEl>>,
    dynamic: AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromElDynamic,
}

impl AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromEl {
    #[doc =
        "Set the field `identities`.\nA list of identities that are allowed access through this 'EgressPolicy'.\nShould be in the format of email address. The email address should\nrepresent individual user or service account only."]
    pub fn set_identities(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.identities = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `identity_type`.\nSpecifies the type of identities that are allowed access to outside the\nperimeter. If left unspecified, then members of 'identities' field will\nbe allowed access. Possible values: [\"IDENTITY_TYPE_UNSPECIFIED\", \"ANY_IDENTITY\", \"ANY_USER_ACCOUNT\", \"ANY_SERVICE_ACCOUNT\"]"]
    pub fn set_identity_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.identity_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `source_restriction`.\nWhether to enforce traffic restrictions based on 'sources' field. If the 'sources' field is non-empty, then this field must be set to 'SOURCE_RESTRICTION_ENABLED'. Possible values: [\"SOURCE_RESTRICTION_UNSPECIFIED\", \"SOURCE_RESTRICTION_ENABLED\", \"SOURCE_RESTRICTION_DISABLED\"]"]
    pub fn set_source_restriction(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.source_restriction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sources`.\n"]
    pub fn set_sources(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromElSourcesEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sources = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromEl {
    type O = terrars::BlockAssignable<AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromEl {}

impl BuildAccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromEl {
        AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromEl {
            identities: core::default::Default::default(),
            identity_type: core::default::Default::default(),
            source_restriction: core::default::Default::default(),
            sources: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromElRef {
        AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `identities` after provisioning.\nA list of identities that are allowed access through this 'EgressPolicy'.\nShould be in the format of email address. The email address should\nrepresent individual user or service account only."]
    pub fn identities(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.identities", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identity_type` after provisioning.\nSpecifies the type of identities that are allowed access to outside the\nperimeter. If left unspecified, then members of 'identities' field will\nbe allowed access. Possible values: [\"IDENTITY_TYPE_UNSPECIFIED\", \"ANY_IDENTITY\", \"ANY_USER_ACCOUNT\", \"ANY_SERVICE_ACCOUNT\"]"]
    pub fn identity_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_restriction` after provisioning.\nWhether to enforce traffic restrictions based on 'sources' field. If the 'sources' field is non-empty, then this field must be set to 'SOURCE_RESTRICTION_ENABLED'. Possible values: [\"SOURCE_RESTRICTION_UNSPECIFIED\", \"SOURCE_RESTRICTION_ENABLED\", \"SOURCE_RESTRICTION_DISABLED\"]"]
    pub fn source_restriction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_restriction", self.base))
    }

    #[doc = "Get a reference to the value of field `sources` after provisioning.\n"]
    pub fn sources(
        &self,
    ) -> terrars::ListRef<AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromElSourcesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sources", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permission: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl {
    #[doc =
        "Set the field `method`.\nValue for 'method' should be a valid method name for the corresponding\n'serviceName' in 'ApiOperation'. If '*' used as value for method,\nthen ALL methods and permissions are allowed."]
    pub fn set_method(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.method = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `permission`.\nValue for permission should be a valid Cloud IAM permission for the\ncorresponding 'serviceName' in 'ApiOperation'."]
    pub fn set_permission(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.permission = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl {
    type O =
        terrars
        ::BlockAssignable<
            AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl {}

impl BuildAccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl {
    pub fn build(
        self,
    ) -> AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl {
        AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl {
            method: core::default::Default::default(),
            permission: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsElRef {
        AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `method` after provisioning.\nValue for 'method' should be a valid method name for the corresponding\n'serviceName' in 'ApiOperation'. If '*' used as value for method,\nthen ALL methods and permissions are allowed."]
    pub fn method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.method", self.base))
    }

    #[doc =
        "Get a reference to the value of field `permission` after provisioning.\nValue for permission should be a valid Cloud IAM permission for the\ncorresponding 'serviceName' in 'ApiOperation'."]
    pub fn permission(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.permission", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsElDynamic {
    method_selectors: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    method_selectors: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl>,
    >,
    dynamic: AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsElDynamic,
}

impl AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsEl {
    #[doc =
        "Set the field `service_name`.\nThe name of the API whose methods or permissions the 'IngressPolicy' or\n'EgressPolicy' want to allow. A single 'ApiOperation' with serviceName\nfield set to '*' will allow all methods AND permissions for all services."]
    pub fn set_service_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `method_selectors`.\n"]
    pub fn set_method_selectors(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.method_selectors = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.method_selectors = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsEl {
    type O =
        terrars::BlockAssignable<AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsEl {}

impl BuildAccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsEl {
        AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsEl {
            service_name: core::default::Default::default(),
            method_selectors: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsElRef {
        AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `service_name` after provisioning.\nThe name of the API whose methods or permissions the 'IngressPolicy' or\n'EgressPolicy' want to allow. A single 'ApiOperation' with serviceName\nfield set to '*' will allow all methods AND permissions for all services."]
    pub fn service_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.base))
    }

    #[doc = "Get a reference to the value of field `method_selectors` after provisioning.\n"]
    pub fn method_selectors(
        &self,
    ) -> terrars
    ::ListRef<AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.method_selectors", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElDynamic {
    operations: std
    ::option
    ::Option<
        terrars::DynamicBlock<AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsEl>,
    >,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_resources: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resources: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operations: std
    ::option
    ::Option<std::vec::Vec<AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsEl>>,
    dynamic: AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElDynamic,
}

impl AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToEl {
    #[doc =
        "Set the field `external_resources`.\nA list of external resources that are allowed to be accessed. A request\nmatches if it contains an external resource in this list (Example:\ns3://bucket/path). Currently '*' is not allowed."]
    pub fn set_external_resources(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.external_resources = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `resources`.\nA list of resources, currently only projects in the form\n'projects/<projectnumber>', that match this to stanza. A request matches\nif it contains a resource in this list. If * is specified for resources,\nthen this 'EgressTo' rule will authorize access to all resources outside\nthe perimeter."]
    pub fn set_resources(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resources = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `operations`.\n"]
    pub fn set_operations(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.operations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.operations = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToEl {
    type O = terrars::BlockAssignable<AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToEl {}

impl BuildAccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToEl {
        AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToEl {
            external_resources: core::default::Default::default(),
            resources: core::default::Default::default(),
            operations: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElRef {
        AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `external_resources` after provisioning.\nA list of external resources that are allowed to be accessed. A request\nmatches if it contains an external resource in this list (Example:\ns3://bucket/path). Currently '*' is not allowed."]
    pub fn external_resources(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.external_resources", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resources` after provisioning.\nA list of resources, currently only projects in the form\n'projects/<projectnumber>', that match this to stanza. A request matches\nif it contains a resource in this list. If * is specified for resources,\nthen this 'EgressTo' rule will authorize access to all resources outside\nthe perimeter."]
    pub fn resources(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }

    #[doc = "Get a reference to the value of field `operations` after provisioning.\n"]
    pub fn operations(
        &self,
    ) -> terrars::ListRef<AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.operations", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeterSpecElEgressPoliciesElDynamic {
    egress_from: std
    ::option
    ::Option<terrars::DynamicBlock<AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromEl>>,
    egress_to: std
    ::option
    ::Option<terrars::DynamicBlock<AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToEl>>,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterSpecElEgressPoliciesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    egress_from: std
    ::option
    ::Option<std::vec::Vec<AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    egress_to: std::option::Option<std::vec::Vec<AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToEl>>,
    dynamic: AccessContextManagerServicePerimeterSpecElEgressPoliciesElDynamic,
}

impl AccessContextManagerServicePerimeterSpecElEgressPoliciesEl {
    #[doc = "Set the field `egress_from`.\n"]
    pub fn set_egress_from(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.egress_from = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.egress_from = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `egress_to`.\n"]
    pub fn set_egress_to(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.egress_to = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.egress_to = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerServicePerimeterSpecElEgressPoliciesEl {
    type O = terrars::BlockAssignable<AccessContextManagerServicePerimeterSpecElEgressPoliciesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeterSpecElEgressPoliciesEl {}

impl BuildAccessContextManagerServicePerimeterSpecElEgressPoliciesEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterSpecElEgressPoliciesEl {
        AccessContextManagerServicePerimeterSpecElEgressPoliciesEl {
            egress_from: core::default::Default::default(),
            egress_to: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimeterSpecElEgressPoliciesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeterSpecElEgressPoliciesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeterSpecElEgressPoliciesElRef {
        AccessContextManagerServicePerimeterSpecElEgressPoliciesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeterSpecElEgressPoliciesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `egress_from` after provisioning.\n"]
    pub fn egress_from(
        &self,
    ) -> terrars::ListRef<AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.egress_from", self.base))
    }

    #[doc = "Get a reference to the value of field `egress_to` after provisioning.\n"]
    pub fn egress_to(&self) -> terrars::ListRef<AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.egress_to", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromElSourcesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_level: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromElSourcesEl {
    #[doc =
        "Set the field `access_level`.\nAn 'AccessLevel' resource name that allow resources within the\n'ServicePerimeters' to be accessed from the internet. 'AccessLevels' listed\nmust be in the same policy as this 'ServicePerimeter'. Referencing a nonexistent\n'AccessLevel' will cause an error. If no 'AccessLevel' names are listed,\nresources within the perimeter can only be accessed via Google Cloud calls\nwith request origins within the perimeter.\nExample 'accessPolicies/MY_POLICY/accessLevels/MY_LEVEL.'\nIf * is specified, then all IngressSources will be allowed."]
    pub fn set_access_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.access_level = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `resource`.\nA Google Cloud resource that is allowed to ingress the perimeter.\nRequests from these resources will be allowed to access perimeter data.\nCurrently only projects are allowed. Format 'projects/{project_number}'\nThe project may be in any Google Cloud organization, not just the\norganization that the perimeter is defined in. '*' is not allowed, the case\nof allowing all Google Cloud resources only is not supported."]
    pub fn set_resource(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromElSourcesEl {
    type O =
        terrars::BlockAssignable<AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromElSourcesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromElSourcesEl {}

impl BuildAccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromElSourcesEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromElSourcesEl {
        AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromElSourcesEl {
            access_level: core::default::Default::default(),
            resource: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromElSourcesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromElSourcesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromElSourcesElRef {
        AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromElSourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromElSourcesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `access_level` after provisioning.\nAn 'AccessLevel' resource name that allow resources within the\n'ServicePerimeters' to be accessed from the internet. 'AccessLevels' listed\nmust be in the same policy as this 'ServicePerimeter'. Referencing a nonexistent\n'AccessLevel' will cause an error. If no 'AccessLevel' names are listed,\nresources within the perimeter can only be accessed via Google Cloud calls\nwith request origins within the perimeter.\nExample 'accessPolicies/MY_POLICY/accessLevels/MY_LEVEL.'\nIf * is specified, then all IngressSources will be allowed."]
    pub fn access_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_level", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resource` after provisioning.\nA Google Cloud resource that is allowed to ingress the perimeter.\nRequests from these resources will be allowed to access perimeter data.\nCurrently only projects are allowed. Format 'projects/{project_number}'\nThe project may be in any Google Cloud organization, not just the\norganization that the perimeter is defined in. '*' is not allowed, the case\nof allowing all Google Cloud resources only is not supported."]
    pub fn resource(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromElDynamic {
    sources: std
    ::option
    ::Option<
        terrars::DynamicBlock<AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromElSourcesEl>,
    >,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identities: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sources: std
    ::option
    ::Option<std::vec::Vec<AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromElSourcesEl>>,
    dynamic: AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromElDynamic,
}

impl AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromEl {
    #[doc =
        "Set the field `identities`.\nA list of identities that are allowed access through this ingress policy.\nShould be in the format of email address. The email address should represent\nindividual user or service account only."]
    pub fn set_identities(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.identities = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `identity_type`.\nSpecifies the type of identities that are allowed access from outside the\nperimeter. If left unspecified, then members of 'identities' field will be\nallowed access. Possible values: [\"IDENTITY_TYPE_UNSPECIFIED\", \"ANY_IDENTITY\", \"ANY_USER_ACCOUNT\", \"ANY_SERVICE_ACCOUNT\"]"]
    pub fn set_identity_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.identity_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sources`.\n"]
    pub fn set_sources(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromElSourcesEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sources = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromEl {
    type O = terrars::BlockAssignable<AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromEl {}

impl BuildAccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromEl {
        AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromEl {
            identities: core::default::Default::default(),
            identity_type: core::default::Default::default(),
            sources: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromElRef {
        AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `identities` after provisioning.\nA list of identities that are allowed access through this ingress policy.\nShould be in the format of email address. The email address should represent\nindividual user or service account only."]
    pub fn identities(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.identities", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identity_type` after provisioning.\nSpecifies the type of identities that are allowed access from outside the\nperimeter. If left unspecified, then members of 'identities' field will be\nallowed access. Possible values: [\"IDENTITY_TYPE_UNSPECIFIED\", \"ANY_IDENTITY\", \"ANY_USER_ACCOUNT\", \"ANY_SERVICE_ACCOUNT\"]"]
    pub fn identity_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_type", self.base))
    }

    #[doc = "Get a reference to the value of field `sources` after provisioning.\n"]
    pub fn sources(
        &self,
    ) -> terrars::ListRef<AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromElSourcesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sources", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permission: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl {
    #[doc =
        "Set the field `method`.\nValue for method should be a valid method name for the corresponding\nserviceName in 'ApiOperation'. If '*' used as value for 'method', then\nALL methods and permissions are allowed."]
    pub fn set_method(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.method = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `permission`.\nValue for permission should be a valid Cloud IAM permission for the\ncorresponding 'serviceName' in 'ApiOperation'."]
    pub fn set_permission(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.permission = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl {
    type O =
        terrars
        ::BlockAssignable<
            AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl {}

impl BuildAccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl {
    pub fn build(
        self,
    ) -> AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl {
        AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl {
            method: core::default::Default::default(),
            permission: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsElRef {
        AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `method` after provisioning.\nValue for method should be a valid method name for the corresponding\nserviceName in 'ApiOperation'. If '*' used as value for 'method', then\nALL methods and permissions are allowed."]
    pub fn method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.method", self.base))
    }

    #[doc =
        "Get a reference to the value of field `permission` after provisioning.\nValue for permission should be a valid Cloud IAM permission for the\ncorresponding 'serviceName' in 'ApiOperation'."]
    pub fn permission(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.permission", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsElDynamic {
    method_selectors: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    method_selectors: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl>,
    >,
    dynamic: AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsElDynamic,
}

impl AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsEl {
    #[doc =
        "Set the field `service_name`.\nThe name of the API whose methods or permissions the 'IngressPolicy' or\n'EgressPolicy' want to allow. A single 'ApiOperation' with 'serviceName'\nfield set to '*' will allow all methods AND permissions for all services."]
    pub fn set_service_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `method_selectors`.\n"]
    pub fn set_method_selectors(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.method_selectors = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.method_selectors = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsEl {
    type O =
        terrars::BlockAssignable<AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsEl {}

impl BuildAccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsEl {
        AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsEl {
            service_name: core::default::Default::default(),
            method_selectors: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsElRef {
        AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `service_name` after provisioning.\nThe name of the API whose methods or permissions the 'IngressPolicy' or\n'EgressPolicy' want to allow. A single 'ApiOperation' with 'serviceName'\nfield set to '*' will allow all methods AND permissions for all services."]
    pub fn service_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.base))
    }

    #[doc = "Get a reference to the value of field `method_selectors` after provisioning.\n"]
    pub fn method_selectors(
        &self,
    ) -> terrars
    ::ListRef<AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.method_selectors", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElDynamic {
    operations: std
    ::option
    ::Option<
        terrars::DynamicBlock<AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsEl>,
    >,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resources: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operations: std
    ::option
    ::Option<std::vec::Vec<AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsEl>>,
    dynamic: AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElDynamic,
}

impl AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToEl {
    #[doc =
        "Set the field `resources`.\nA list of resources, currently only projects in the form\n'projects/<projectnumber>', protected by this 'ServicePerimeter'\nthat are allowed to be accessed by sources defined in the\ncorresponding 'IngressFrom'. A request matches if it contains\na resource in this list. If '*' is specified for resources,\nthen this 'IngressTo' rule will authorize access to all\nresources inside the perimeter, provided that the request\nalso matches the 'operations' field."]
    pub fn set_resources(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resources = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `operations`.\n"]
    pub fn set_operations(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.operations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.operations = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToEl {
    type O = terrars::BlockAssignable<AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToEl {}

impl BuildAccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToEl {
        AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToEl {
            resources: core::default::Default::default(),
            operations: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElRef {
        AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `resources` after provisioning.\nA list of resources, currently only projects in the form\n'projects/<projectnumber>', protected by this 'ServicePerimeter'\nthat are allowed to be accessed by sources defined in the\ncorresponding 'IngressFrom'. A request matches if it contains\na resource in this list. If '*' is specified for resources,\nthen this 'IngressTo' rule will authorize access to all\nresources inside the perimeter, provided that the request\nalso matches the 'operations' field."]
    pub fn resources(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }

    #[doc = "Get a reference to the value of field `operations` after provisioning.\n"]
    pub fn operations(
        &self,
    ) -> terrars::ListRef<AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.operations", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeterSpecElIngressPoliciesElDynamic {
    ingress_from: std
    ::option
    ::Option<terrars::DynamicBlock<AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromEl>>,
    ingress_to: std
    ::option
    ::Option<terrars::DynamicBlock<AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToEl>>,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterSpecElIngressPoliciesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ingress_from: std
    ::option
    ::Option<std::vec::Vec<AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ingress_to: std
    ::option
    ::Option<std::vec::Vec<AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToEl>>,
    dynamic: AccessContextManagerServicePerimeterSpecElIngressPoliciesElDynamic,
}

impl AccessContextManagerServicePerimeterSpecElIngressPoliciesEl {
    #[doc = "Set the field `ingress_from`.\n"]
    pub fn set_ingress_from(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ingress_from = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ingress_from = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ingress_to`.\n"]
    pub fn set_ingress_to(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ingress_to = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ingress_to = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerServicePerimeterSpecElIngressPoliciesEl {
    type O = terrars::BlockAssignable<AccessContextManagerServicePerimeterSpecElIngressPoliciesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeterSpecElIngressPoliciesEl {}

impl BuildAccessContextManagerServicePerimeterSpecElIngressPoliciesEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterSpecElIngressPoliciesEl {
        AccessContextManagerServicePerimeterSpecElIngressPoliciesEl {
            ingress_from: core::default::Default::default(),
            ingress_to: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimeterSpecElIngressPoliciesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeterSpecElIngressPoliciesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeterSpecElIngressPoliciesElRef {
        AccessContextManagerServicePerimeterSpecElIngressPoliciesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeterSpecElIngressPoliciesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ingress_from` after provisioning.\n"]
    pub fn ingress_from(
        &self,
    ) -> terrars::ListRef<AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ingress_from", self.base))
    }

    #[doc = "Get a reference to the value of field `ingress_to` after provisioning.\n"]
    pub fn ingress_to(
        &self,
    ) -> terrars::ListRef<AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ingress_to", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterSpecElVpcAccessibleServicesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_services: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_restriction: std::option::Option<terrars::PrimField<bool>>,
}

impl AccessContextManagerServicePerimeterSpecElVpcAccessibleServicesEl {
    #[doc =
        "Set the field `allowed_services`.\nThe list of APIs usable within the Service Perimeter.\nMust be empty unless 'enableRestriction' is True."]
    pub fn set_allowed_services(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_services = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_restriction`.\nWhether to restrict API calls within the Service Perimeter to the\nlist of APIs specified in 'allowedServices'."]
    pub fn set_enable_restriction(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_restriction = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerServicePerimeterSpecElVpcAccessibleServicesEl {
    type O = terrars::BlockAssignable<AccessContextManagerServicePerimeterSpecElVpcAccessibleServicesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeterSpecElVpcAccessibleServicesEl {}

impl BuildAccessContextManagerServicePerimeterSpecElVpcAccessibleServicesEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterSpecElVpcAccessibleServicesEl {
        AccessContextManagerServicePerimeterSpecElVpcAccessibleServicesEl {
            allowed_services: core::default::Default::default(),
            enable_restriction: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimeterSpecElVpcAccessibleServicesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeterSpecElVpcAccessibleServicesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeterSpecElVpcAccessibleServicesElRef {
        AccessContextManagerServicePerimeterSpecElVpcAccessibleServicesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeterSpecElVpcAccessibleServicesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed_services` after provisioning.\nThe list of APIs usable within the Service Perimeter.\nMust be empty unless 'enableRestriction' is True."]
    pub fn allowed_services(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allowed_services", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_restriction` after provisioning.\nWhether to restrict API calls within the Service Perimeter to the\nlist of APIs specified in 'allowedServices'."]
    pub fn enable_restriction(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_restriction", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeterSpecElDynamic {
    egress_policies: std
    ::option
    ::Option<terrars::DynamicBlock<AccessContextManagerServicePerimeterSpecElEgressPoliciesEl>>,
    ingress_policies: std
    ::option
    ::Option<terrars::DynamicBlock<AccessContextManagerServicePerimeterSpecElIngressPoliciesEl>>,
    vpc_accessible_services: std
    ::option
    ::Option<terrars::DynamicBlock<AccessContextManagerServicePerimeterSpecElVpcAccessibleServicesEl>>,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterSpecEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_levels: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resources: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restricted_services: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    egress_policies: std::option::Option<std::vec::Vec<AccessContextManagerServicePerimeterSpecElEgressPoliciesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ingress_policies: std::option::Option<std::vec::Vec<AccessContextManagerServicePerimeterSpecElIngressPoliciesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_accessible_services: std
    ::option
    ::Option<std::vec::Vec<AccessContextManagerServicePerimeterSpecElVpcAccessibleServicesEl>>,
    dynamic: AccessContextManagerServicePerimeterSpecElDynamic,
}

impl AccessContextManagerServicePerimeterSpecEl {
    #[doc =
        "Set the field `access_levels`.\nA list of AccessLevel resource names that allow resources within\nthe ServicePerimeter to be accessed from the internet.\nAccessLevels listed must be in the same policy as this\nServicePerimeter. Referencing a nonexistent AccessLevel is a\nsyntax error. If no AccessLevel names are listed, resources within\nthe perimeter can only be accessed via GCP calls with request\norigins within the perimeter. For Service Perimeter Bridge, must\nbe empty.\n\nFormat: accessPolicies/{policy_id}/accessLevels/{access_level_name}"]
    pub fn set_access_levels(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.access_levels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `resources`.\nA list of GCP resources that are inside of the service perimeter.\nCurrently only projects are allowed.\nFormat: projects/{project_number}"]
    pub fn set_resources(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resources = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `restricted_services`.\nGCP services that are subject to the Service Perimeter\nrestrictions. Must contain a list of services. For example, if\n'storage.googleapis.com' is specified, access to the storage\nbuckets inside the perimeter must meet the perimeter's access\nrestrictions."]
    pub fn set_restricted_services(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.restricted_services = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `egress_policies`.\n"]
    pub fn set_egress_policies(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AccessContextManagerServicePerimeterSpecElEgressPoliciesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.egress_policies = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.egress_policies = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ingress_policies`.\n"]
    pub fn set_ingress_policies(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AccessContextManagerServicePerimeterSpecElIngressPoliciesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ingress_policies = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ingress_policies = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `vpc_accessible_services`.\n"]
    pub fn set_vpc_accessible_services(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AccessContextManagerServicePerimeterSpecElVpcAccessibleServicesEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.vpc_accessible_services = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.vpc_accessible_services = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerServicePerimeterSpecEl {
    type O = terrars::BlockAssignable<AccessContextManagerServicePerimeterSpecEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeterSpecEl {}

impl BuildAccessContextManagerServicePerimeterSpecEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterSpecEl {
        AccessContextManagerServicePerimeterSpecEl {
            access_levels: core::default::Default::default(),
            resources: core::default::Default::default(),
            restricted_services: core::default::Default::default(),
            egress_policies: core::default::Default::default(),
            ingress_policies: core::default::Default::default(),
            vpc_accessible_services: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimeterSpecElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeterSpecElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AccessContextManagerServicePerimeterSpecElRef {
        AccessContextManagerServicePerimeterSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeterSpecElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `access_levels` after provisioning.\nA list of AccessLevel resource names that allow resources within\nthe ServicePerimeter to be accessed from the internet.\nAccessLevels listed must be in the same policy as this\nServicePerimeter. Referencing a nonexistent AccessLevel is a\nsyntax error. If no AccessLevel names are listed, resources within\nthe perimeter can only be accessed via GCP calls with request\norigins within the perimeter. For Service Perimeter Bridge, must\nbe empty.\n\nFormat: accessPolicies/{policy_id}/accessLevels/{access_level_name}"]
    pub fn access_levels(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.access_levels", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resources` after provisioning.\nA list of GCP resources that are inside of the service perimeter.\nCurrently only projects are allowed.\nFormat: projects/{project_number}"]
    pub fn resources(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }

    #[doc =
        "Get a reference to the value of field `restricted_services` after provisioning.\nGCP services that are subject to the Service Perimeter\nrestrictions. Must contain a list of services. For example, if\n'storage.googleapis.com' is specified, access to the storage\nbuckets inside the perimeter must meet the perimeter's access\nrestrictions."]
    pub fn restricted_services(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.restricted_services", self.base))
    }

    #[doc = "Get a reference to the value of field `egress_policies` after provisioning.\n"]
    pub fn egress_policies(&self) -> terrars::ListRef<AccessContextManagerServicePerimeterSpecElEgressPoliciesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.egress_policies", self.base))
    }

    #[doc = "Get a reference to the value of field `ingress_policies` after provisioning.\n"]
    pub fn ingress_policies(&self) -> terrars::ListRef<AccessContextManagerServicePerimeterSpecElIngressPoliciesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ingress_policies", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_accessible_services` after provisioning.\n"]
    pub fn vpc_accessible_services(
        &self,
    ) -> terrars::ListRef<AccessContextManagerServicePerimeterSpecElVpcAccessibleServicesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_accessible_services", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromElSourcesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_level: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromElSourcesEl {
    #[doc =
        "Set the field `access_level`.\nAn AccessLevel resource name that allows resources outside the ServicePerimeter to be accessed from the inside."]
    pub fn set_access_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.access_level = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromElSourcesEl {
    type O =
        terrars::BlockAssignable<AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromElSourcesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromElSourcesEl {}

impl BuildAccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromElSourcesEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromElSourcesEl {
        AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromElSourcesEl {
            access_level: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromElSourcesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromElSourcesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromElSourcesElRef {
        AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromElSourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromElSourcesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `access_level` after provisioning.\nAn AccessLevel resource name that allows resources outside the ServicePerimeter to be accessed from the inside."]
    pub fn access_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_level", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromElDynamic {
    sources: std
    ::option
    ::Option<
        terrars::DynamicBlock<AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromElSourcesEl>,
    >,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identities: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_restriction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sources: std
    ::option
    ::Option<std::vec::Vec<AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromElSourcesEl>>,
    dynamic: AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromElDynamic,
}

impl AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromEl {
    #[doc =
        "Set the field `identities`.\nA list of identities that are allowed access through this 'EgressPolicy'.\nShould be in the format of email address. The email address should\nrepresent individual user or service account only."]
    pub fn set_identities(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.identities = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `identity_type`.\nSpecifies the type of identities that are allowed access to outside the\nperimeter. If left unspecified, then members of 'identities' field will\nbe allowed access. Possible values: [\"IDENTITY_TYPE_UNSPECIFIED\", \"ANY_IDENTITY\", \"ANY_USER_ACCOUNT\", \"ANY_SERVICE_ACCOUNT\"]"]
    pub fn set_identity_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.identity_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `source_restriction`.\nWhether to enforce traffic restrictions based on 'sources' field. If the 'sources' field is non-empty, then this field must be set to 'SOURCE_RESTRICTION_ENABLED'. Possible values: [\"SOURCE_RESTRICTION_UNSPECIFIED\", \"SOURCE_RESTRICTION_ENABLED\", \"SOURCE_RESTRICTION_DISABLED\"]"]
    pub fn set_source_restriction(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.source_restriction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sources`.\n"]
    pub fn set_sources(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromElSourcesEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sources = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromEl {
    type O = terrars::BlockAssignable<AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromEl {}

impl BuildAccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromEl {
        AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromEl {
            identities: core::default::Default::default(),
            identity_type: core::default::Default::default(),
            source_restriction: core::default::Default::default(),
            sources: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromElRef {
        AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `identities` after provisioning.\nA list of identities that are allowed access through this 'EgressPolicy'.\nShould be in the format of email address. The email address should\nrepresent individual user or service account only."]
    pub fn identities(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.identities", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identity_type` after provisioning.\nSpecifies the type of identities that are allowed access to outside the\nperimeter. If left unspecified, then members of 'identities' field will\nbe allowed access. Possible values: [\"IDENTITY_TYPE_UNSPECIFIED\", \"ANY_IDENTITY\", \"ANY_USER_ACCOUNT\", \"ANY_SERVICE_ACCOUNT\"]"]
    pub fn identity_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_restriction` after provisioning.\nWhether to enforce traffic restrictions based on 'sources' field. If the 'sources' field is non-empty, then this field must be set to 'SOURCE_RESTRICTION_ENABLED'. Possible values: [\"SOURCE_RESTRICTION_UNSPECIFIED\", \"SOURCE_RESTRICTION_ENABLED\", \"SOURCE_RESTRICTION_DISABLED\"]"]
    pub fn source_restriction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_restriction", self.base))
    }

    #[doc = "Get a reference to the value of field `sources` after provisioning.\n"]
    pub fn sources(
        &self,
    ) -> terrars::ListRef<AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromElSourcesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sources", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permission: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl {
    #[doc =
        "Set the field `method`.\nValue for 'method' should be a valid method name for the corresponding\n'serviceName' in 'ApiOperation'. If '*' used as value for method,\nthen ALL methods and permissions are allowed."]
    pub fn set_method(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.method = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `permission`.\nValue for permission should be a valid Cloud IAM permission for the\ncorresponding 'serviceName' in 'ApiOperation'."]
    pub fn set_permission(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.permission = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl {
    type O =
        terrars
        ::BlockAssignable<
            AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl {}

impl BuildAccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl {
    pub fn build(
        self,
    ) -> AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl {
        AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl {
            method: core::default::Default::default(),
            permission: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsElRef {
        AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `method` after provisioning.\nValue for 'method' should be a valid method name for the corresponding\n'serviceName' in 'ApiOperation'. If '*' used as value for method,\nthen ALL methods and permissions are allowed."]
    pub fn method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.method", self.base))
    }

    #[doc =
        "Get a reference to the value of field `permission` after provisioning.\nValue for permission should be a valid Cloud IAM permission for the\ncorresponding 'serviceName' in 'ApiOperation'."]
    pub fn permission(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.permission", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsElDynamic {
    method_selectors: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    method_selectors: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl>,
    >,
    dynamic: AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsElDynamic,
}

impl AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsEl {
    #[doc =
        "Set the field `service_name`.\nThe name of the API whose methods or permissions the 'IngressPolicy' or\n'EgressPolicy' want to allow. A single 'ApiOperation' with serviceName\nfield set to '*' will allow all methods AND permissions for all services."]
    pub fn set_service_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `method_selectors`.\n"]
    pub fn set_method_selectors(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.method_selectors = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.method_selectors = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsEl {
    type O =
        terrars::BlockAssignable<AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsEl {}

impl BuildAccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsEl {
        AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsEl {
            service_name: core::default::Default::default(),
            method_selectors: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsElRef {
        AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `service_name` after provisioning.\nThe name of the API whose methods or permissions the 'IngressPolicy' or\n'EgressPolicy' want to allow. A single 'ApiOperation' with serviceName\nfield set to '*' will allow all methods AND permissions for all services."]
    pub fn service_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.base))
    }

    #[doc = "Get a reference to the value of field `method_selectors` after provisioning.\n"]
    pub fn method_selectors(
        &self,
    ) -> terrars
    ::ListRef<AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.method_selectors", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElDynamic {
    operations: std
    ::option
    ::Option<
        terrars::DynamicBlock<AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsEl>,
    >,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_resources: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resources: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operations: std
    ::option
    ::Option<std::vec::Vec<AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsEl>>,
    dynamic: AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElDynamic,
}

impl AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToEl {
    #[doc =
        "Set the field `external_resources`.\nA list of external resources that are allowed to be accessed. A request\nmatches if it contains an external resource in this list (Example:\ns3://bucket/path). Currently '*' is not allowed."]
    pub fn set_external_resources(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.external_resources = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `resources`.\nA list of resources, currently only projects in the form\n'projects/<projectnumber>', that match this to stanza. A request matches\nif it contains a resource in this list. If * is specified for resources,\nthen this 'EgressTo' rule will authorize access to all resources outside\nthe perimeter."]
    pub fn set_resources(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resources = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `operations`.\n"]
    pub fn set_operations(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.operations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.operations = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToEl {
    type O = terrars::BlockAssignable<AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToEl {}

impl BuildAccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToEl {
        AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToEl {
            external_resources: core::default::Default::default(),
            resources: core::default::Default::default(),
            operations: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElRef {
        AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `external_resources` after provisioning.\nA list of external resources that are allowed to be accessed. A request\nmatches if it contains an external resource in this list (Example:\ns3://bucket/path). Currently '*' is not allowed."]
    pub fn external_resources(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.external_resources", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resources` after provisioning.\nA list of resources, currently only projects in the form\n'projects/<projectnumber>', that match this to stanza. A request matches\nif it contains a resource in this list. If * is specified for resources,\nthen this 'EgressTo' rule will authorize access to all resources outside\nthe perimeter."]
    pub fn resources(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }

    #[doc = "Get a reference to the value of field `operations` after provisioning.\n"]
    pub fn operations(
        &self,
    ) -> terrars::ListRef<AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.operations", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeterStatusElEgressPoliciesElDynamic {
    egress_from: std
    ::option
    ::Option<terrars::DynamicBlock<AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromEl>>,
    egress_to: std
    ::option
    ::Option<terrars::DynamicBlock<AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToEl>>,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterStatusElEgressPoliciesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    egress_from: std
    ::option
    ::Option<std::vec::Vec<AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    egress_to: std::option::Option<std::vec::Vec<AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToEl>>,
    dynamic: AccessContextManagerServicePerimeterStatusElEgressPoliciesElDynamic,
}

impl AccessContextManagerServicePerimeterStatusElEgressPoliciesEl {
    #[doc = "Set the field `egress_from`.\n"]
    pub fn set_egress_from(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.egress_from = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.egress_from = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `egress_to`.\n"]
    pub fn set_egress_to(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.egress_to = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.egress_to = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerServicePerimeterStatusElEgressPoliciesEl {
    type O = terrars::BlockAssignable<AccessContextManagerServicePerimeterStatusElEgressPoliciesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeterStatusElEgressPoliciesEl {}

impl BuildAccessContextManagerServicePerimeterStatusElEgressPoliciesEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterStatusElEgressPoliciesEl {
        AccessContextManagerServicePerimeterStatusElEgressPoliciesEl {
            egress_from: core::default::Default::default(),
            egress_to: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimeterStatusElEgressPoliciesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeterStatusElEgressPoliciesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeterStatusElEgressPoliciesElRef {
        AccessContextManagerServicePerimeterStatusElEgressPoliciesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeterStatusElEgressPoliciesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `egress_from` after provisioning.\n"]
    pub fn egress_from(
        &self,
    ) -> terrars::ListRef<AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.egress_from", self.base))
    }

    #[doc = "Get a reference to the value of field `egress_to` after provisioning.\n"]
    pub fn egress_to(
        &self,
    ) -> terrars::ListRef<AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.egress_to", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromElSourcesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_level: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromElSourcesEl {
    #[doc =
        "Set the field `access_level`.\nAn 'AccessLevel' resource name that allow resources within the\n'ServicePerimeters' to be accessed from the internet. 'AccessLevels' listed\nmust be in the same policy as this 'ServicePerimeter'. Referencing a nonexistent\n'AccessLevel' will cause an error. If no 'AccessLevel' names are listed,\nresources within the perimeter can only be accessed via Google Cloud calls\nwith request origins within the perimeter.\nExample 'accessPolicies/MY_POLICY/accessLevels/MY_LEVEL.'\nIf * is specified, then all IngressSources will be allowed."]
    pub fn set_access_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.access_level = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `resource`.\nA Google Cloud resource that is allowed to ingress the perimeter.\nRequests from these resources will be allowed to access perimeter data.\nCurrently only projects are allowed. Format 'projects/{project_number}'\nThe project may be in any Google Cloud organization, not just the\norganization that the perimeter is defined in. '*' is not allowed, the case\nof allowing all Google Cloud resources only is not supported."]
    pub fn set_resource(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromElSourcesEl {
    type O =
        terrars
        ::BlockAssignable<AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromElSourcesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromElSourcesEl {}

impl BuildAccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromElSourcesEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromElSourcesEl {
        AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromElSourcesEl {
            access_level: core::default::Default::default(),
            resource: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromElSourcesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromElSourcesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromElSourcesElRef {
        AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromElSourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromElSourcesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `access_level` after provisioning.\nAn 'AccessLevel' resource name that allow resources within the\n'ServicePerimeters' to be accessed from the internet. 'AccessLevels' listed\nmust be in the same policy as this 'ServicePerimeter'. Referencing a nonexistent\n'AccessLevel' will cause an error. If no 'AccessLevel' names are listed,\nresources within the perimeter can only be accessed via Google Cloud calls\nwith request origins within the perimeter.\nExample 'accessPolicies/MY_POLICY/accessLevels/MY_LEVEL.'\nIf * is specified, then all IngressSources will be allowed."]
    pub fn access_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_level", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resource` after provisioning.\nA Google Cloud resource that is allowed to ingress the perimeter.\nRequests from these resources will be allowed to access perimeter data.\nCurrently only projects are allowed. Format 'projects/{project_number}'\nThe project may be in any Google Cloud organization, not just the\norganization that the perimeter is defined in. '*' is not allowed, the case\nof allowing all Google Cloud resources only is not supported."]
    pub fn resource(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromElDynamic {
    sources: std
    ::option
    ::Option<
        terrars::DynamicBlock<AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromElSourcesEl>,
    >,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identities: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sources: std
    ::option
    ::Option<std::vec::Vec<AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromElSourcesEl>>,
    dynamic: AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromElDynamic,
}

impl AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromEl {
    #[doc =
        "Set the field `identities`.\nA list of identities that are allowed access through this ingress policy.\nShould be in the format of email address. The email address should represent\nindividual user or service account only."]
    pub fn set_identities(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.identities = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `identity_type`.\nSpecifies the type of identities that are allowed access from outside the\nperimeter. If left unspecified, then members of 'identities' field will be\nallowed access. Possible values: [\"IDENTITY_TYPE_UNSPECIFIED\", \"ANY_IDENTITY\", \"ANY_USER_ACCOUNT\", \"ANY_SERVICE_ACCOUNT\"]"]
    pub fn set_identity_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.identity_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sources`.\n"]
    pub fn set_sources(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromElSourcesEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sources = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromEl {
    type O = terrars::BlockAssignable<AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromEl {}

impl BuildAccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromEl {
        AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromEl {
            identities: core::default::Default::default(),
            identity_type: core::default::Default::default(),
            sources: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromElRef {
        AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `identities` after provisioning.\nA list of identities that are allowed access through this ingress policy.\nShould be in the format of email address. The email address should represent\nindividual user or service account only."]
    pub fn identities(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.identities", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identity_type` after provisioning.\nSpecifies the type of identities that are allowed access from outside the\nperimeter. If left unspecified, then members of 'identities' field will be\nallowed access. Possible values: [\"IDENTITY_TYPE_UNSPECIFIED\", \"ANY_IDENTITY\", \"ANY_USER_ACCOUNT\", \"ANY_SERVICE_ACCOUNT\"]"]
    pub fn identity_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_type", self.base))
    }

    #[doc = "Get a reference to the value of field `sources` after provisioning.\n"]
    pub fn sources(
        &self,
    ) -> terrars::ListRef<AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromElSourcesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sources", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permission: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl {
    #[doc =
        "Set the field `method`.\nValue for method should be a valid method name for the corresponding\nserviceName in 'ApiOperation'. If '*' used as value for 'method', then\nALL methods and permissions are allowed."]
    pub fn set_method(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.method = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `permission`.\nValue for permission should be a valid Cloud IAM permission for the\ncorresponding 'serviceName' in 'ApiOperation'."]
    pub fn set_permission(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.permission = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl {
    type O =
        terrars
        ::BlockAssignable<
            AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl {}

impl BuildAccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl {
    pub fn build(
        self,
    ) -> AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl {
        AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl {
            method: core::default::Default::default(),
            permission: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsElRef {
        AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `method` after provisioning.\nValue for method should be a valid method name for the corresponding\nserviceName in 'ApiOperation'. If '*' used as value for 'method', then\nALL methods and permissions are allowed."]
    pub fn method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.method", self.base))
    }

    #[doc =
        "Get a reference to the value of field `permission` after provisioning.\nValue for permission should be a valid Cloud IAM permission for the\ncorresponding 'serviceName' in 'ApiOperation'."]
    pub fn permission(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.permission", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsElDynamic {
    method_selectors: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    method_selectors: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl>,
    >,
    dynamic: AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsElDynamic,
}

impl AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsEl {
    #[doc =
        "Set the field `service_name`.\nThe name of the API whose methods or permissions the 'IngressPolicy' or\n'EgressPolicy' want to allow. A single 'ApiOperation' with 'serviceName'\nfield set to '*' will allow all methods AND permissions for all services."]
    pub fn set_service_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `method_selectors`.\n"]
    pub fn set_method_selectors(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.method_selectors = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.method_selectors = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsEl {
    type O =
        terrars
        ::BlockAssignable<AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsEl {}

impl BuildAccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsEl {
        AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsEl {
            service_name: core::default::Default::default(),
            method_selectors: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsElRef {
        AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `service_name` after provisioning.\nThe name of the API whose methods or permissions the 'IngressPolicy' or\n'EgressPolicy' want to allow. A single 'ApiOperation' with 'serviceName'\nfield set to '*' will allow all methods AND permissions for all services."]
    pub fn service_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.base))
    }

    #[doc = "Get a reference to the value of field `method_selectors` after provisioning.\n"]
    pub fn method_selectors(
        &self,
    ) -> terrars
    ::ListRef<
        AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.method_selectors", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElDynamic {
    operations: std
    ::option
    ::Option<
        terrars::DynamicBlock<AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsEl>,
    >,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resources: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operations: std
    ::option
    ::Option<std::vec::Vec<AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsEl>>,
    dynamic: AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElDynamic,
}

impl AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToEl {
    #[doc =
        "Set the field `resources`.\nA list of resources, currently only projects in the form\n'projects/<projectnumber>', protected by this 'ServicePerimeter'\nthat are allowed to be accessed by sources defined in the\ncorresponding 'IngressFrom'. A request matches if it contains\na resource in this list. If '*' is specified for resources,\nthen this 'IngressTo' rule will authorize access to all\nresources inside the perimeter, provided that the request\nalso matches the 'operations' field."]
    pub fn set_resources(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resources = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `operations`.\n"]
    pub fn set_operations(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.operations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.operations = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToEl {
    type O = terrars::BlockAssignable<AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToEl {}

impl BuildAccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToEl {
        AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToEl {
            resources: core::default::Default::default(),
            operations: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElRef {
        AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `resources` after provisioning.\nA list of resources, currently only projects in the form\n'projects/<projectnumber>', protected by this 'ServicePerimeter'\nthat are allowed to be accessed by sources defined in the\ncorresponding 'IngressFrom'. A request matches if it contains\na resource in this list. If '*' is specified for resources,\nthen this 'IngressTo' rule will authorize access to all\nresources inside the perimeter, provided that the request\nalso matches the 'operations' field."]
    pub fn resources(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }

    #[doc = "Get a reference to the value of field `operations` after provisioning.\n"]
    pub fn operations(
        &self,
    ) -> terrars::ListRef<AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.operations", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeterStatusElIngressPoliciesElDynamic {
    ingress_from: std
    ::option
    ::Option<terrars::DynamicBlock<AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromEl>>,
    ingress_to: std
    ::option
    ::Option<terrars::DynamicBlock<AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToEl>>,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterStatusElIngressPoliciesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ingress_from: std
    ::option
    ::Option<std::vec::Vec<AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ingress_to: std
    ::option
    ::Option<std::vec::Vec<AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToEl>>,
    dynamic: AccessContextManagerServicePerimeterStatusElIngressPoliciesElDynamic,
}

impl AccessContextManagerServicePerimeterStatusElIngressPoliciesEl {
    #[doc = "Set the field `ingress_from`.\n"]
    pub fn set_ingress_from(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ingress_from = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ingress_from = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ingress_to`.\n"]
    pub fn set_ingress_to(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ingress_to = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ingress_to = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerServicePerimeterStatusElIngressPoliciesEl {
    type O = terrars::BlockAssignable<AccessContextManagerServicePerimeterStatusElIngressPoliciesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeterStatusElIngressPoliciesEl {}

impl BuildAccessContextManagerServicePerimeterStatusElIngressPoliciesEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterStatusElIngressPoliciesEl {
        AccessContextManagerServicePerimeterStatusElIngressPoliciesEl {
            ingress_from: core::default::Default::default(),
            ingress_to: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimeterStatusElIngressPoliciesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeterStatusElIngressPoliciesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeterStatusElIngressPoliciesElRef {
        AccessContextManagerServicePerimeterStatusElIngressPoliciesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeterStatusElIngressPoliciesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ingress_from` after provisioning.\n"]
    pub fn ingress_from(
        &self,
    ) -> terrars::ListRef<AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ingress_from", self.base))
    }

    #[doc = "Get a reference to the value of field `ingress_to` after provisioning.\n"]
    pub fn ingress_to(
        &self,
    ) -> terrars::ListRef<AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ingress_to", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterStatusElVpcAccessibleServicesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_services: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_restriction: std::option::Option<terrars::PrimField<bool>>,
}

impl AccessContextManagerServicePerimeterStatusElVpcAccessibleServicesEl {
    #[doc =
        "Set the field `allowed_services`.\nThe list of APIs usable within the Service Perimeter.\nMust be empty unless 'enableRestriction' is True."]
    pub fn set_allowed_services(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_services = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_restriction`.\nWhether to restrict API calls within the Service Perimeter to the\nlist of APIs specified in 'allowedServices'."]
    pub fn set_enable_restriction(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_restriction = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerServicePerimeterStatusElVpcAccessibleServicesEl {
    type O = terrars::BlockAssignable<AccessContextManagerServicePerimeterStatusElVpcAccessibleServicesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeterStatusElVpcAccessibleServicesEl {}

impl BuildAccessContextManagerServicePerimeterStatusElVpcAccessibleServicesEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterStatusElVpcAccessibleServicesEl {
        AccessContextManagerServicePerimeterStatusElVpcAccessibleServicesEl {
            allowed_services: core::default::Default::default(),
            enable_restriction: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimeterStatusElVpcAccessibleServicesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeterStatusElVpcAccessibleServicesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeterStatusElVpcAccessibleServicesElRef {
        AccessContextManagerServicePerimeterStatusElVpcAccessibleServicesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeterStatusElVpcAccessibleServicesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed_services` after provisioning.\nThe list of APIs usable within the Service Perimeter.\nMust be empty unless 'enableRestriction' is True."]
    pub fn allowed_services(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allowed_services", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_restriction` after provisioning.\nWhether to restrict API calls within the Service Perimeter to the\nlist of APIs specified in 'allowedServices'."]
    pub fn enable_restriction(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_restriction", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeterStatusElDynamic {
    egress_policies: std
    ::option
    ::Option<terrars::DynamicBlock<AccessContextManagerServicePerimeterStatusElEgressPoliciesEl>>,
    ingress_policies: std
    ::option
    ::Option<terrars::DynamicBlock<AccessContextManagerServicePerimeterStatusElIngressPoliciesEl>>,
    vpc_accessible_services: std
    ::option
    ::Option<terrars::DynamicBlock<AccessContextManagerServicePerimeterStatusElVpcAccessibleServicesEl>>,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterStatusEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_levels: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resources: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restricted_services: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    egress_policies: std::option::Option<std::vec::Vec<AccessContextManagerServicePerimeterStatusElEgressPoliciesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ingress_policies: std::option::Option<std::vec::Vec<AccessContextManagerServicePerimeterStatusElIngressPoliciesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_accessible_services: std
    ::option
    ::Option<std::vec::Vec<AccessContextManagerServicePerimeterStatusElVpcAccessibleServicesEl>>,
    dynamic: AccessContextManagerServicePerimeterStatusElDynamic,
}

impl AccessContextManagerServicePerimeterStatusEl {
    #[doc =
        "Set the field `access_levels`.\nA list of AccessLevel resource names that allow resources within\nthe ServicePerimeter to be accessed from the internet.\nAccessLevels listed must be in the same policy as this\nServicePerimeter. Referencing a nonexistent AccessLevel is a\nsyntax error. If no AccessLevel names are listed, resources within\nthe perimeter can only be accessed via GCP calls with request\norigins within the perimeter. For Service Perimeter Bridge, must\nbe empty.\n\nFormat: accessPolicies/{policy_id}/accessLevels/{access_level_name}"]
    pub fn set_access_levels(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.access_levels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `resources`.\nA list of GCP resources that are inside of the service perimeter.\nCurrently only projects are allowed.\nFormat: projects/{project_number}"]
    pub fn set_resources(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resources = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `restricted_services`.\nGCP services that are subject to the Service Perimeter\nrestrictions. Must contain a list of services. For example, if\n'storage.googleapis.com' is specified, access to the storage\nbuckets inside the perimeter must meet the perimeter's access\nrestrictions."]
    pub fn set_restricted_services(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.restricted_services = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `egress_policies`.\n"]
    pub fn set_egress_policies(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AccessContextManagerServicePerimeterStatusElEgressPoliciesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.egress_policies = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.egress_policies = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ingress_policies`.\n"]
    pub fn set_ingress_policies(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AccessContextManagerServicePerimeterStatusElIngressPoliciesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ingress_policies = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ingress_policies = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `vpc_accessible_services`.\n"]
    pub fn set_vpc_accessible_services(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AccessContextManagerServicePerimeterStatusElVpcAccessibleServicesEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.vpc_accessible_services = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.vpc_accessible_services = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerServicePerimeterStatusEl {
    type O = terrars::BlockAssignable<AccessContextManagerServicePerimeterStatusEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeterStatusEl {}

impl BuildAccessContextManagerServicePerimeterStatusEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterStatusEl {
        AccessContextManagerServicePerimeterStatusEl {
            access_levels: core::default::Default::default(),
            resources: core::default::Default::default(),
            restricted_services: core::default::Default::default(),
            egress_policies: core::default::Default::default(),
            ingress_policies: core::default::Default::default(),
            vpc_accessible_services: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimeterStatusElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeterStatusElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeterStatusElRef {
        AccessContextManagerServicePerimeterStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeterStatusElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `access_levels` after provisioning.\nA list of AccessLevel resource names that allow resources within\nthe ServicePerimeter to be accessed from the internet.\nAccessLevels listed must be in the same policy as this\nServicePerimeter. Referencing a nonexistent AccessLevel is a\nsyntax error. If no AccessLevel names are listed, resources within\nthe perimeter can only be accessed via GCP calls with request\norigins within the perimeter. For Service Perimeter Bridge, must\nbe empty.\n\nFormat: accessPolicies/{policy_id}/accessLevels/{access_level_name}"]
    pub fn access_levels(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.access_levels", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resources` after provisioning.\nA list of GCP resources that are inside of the service perimeter.\nCurrently only projects are allowed.\nFormat: projects/{project_number}"]
    pub fn resources(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }

    #[doc =
        "Get a reference to the value of field `restricted_services` after provisioning.\nGCP services that are subject to the Service Perimeter\nrestrictions. Must contain a list of services. For example, if\n'storage.googleapis.com' is specified, access to the storage\nbuckets inside the perimeter must meet the perimeter's access\nrestrictions."]
    pub fn restricted_services(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.restricted_services", self.base))
    }

    #[doc = "Get a reference to the value of field `egress_policies` after provisioning.\n"]
    pub fn egress_policies(&self) -> terrars::ListRef<AccessContextManagerServicePerimeterStatusElEgressPoliciesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.egress_policies", self.base))
    }

    #[doc = "Get a reference to the value of field `ingress_policies` after provisioning.\n"]
    pub fn ingress_policies(&self) -> terrars::ListRef<AccessContextManagerServicePerimeterStatusElIngressPoliciesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ingress_policies", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_accessible_services` after provisioning.\n"]
    pub fn vpc_accessible_services(
        &self,
    ) -> terrars::ListRef<AccessContextManagerServicePerimeterStatusElVpcAccessibleServicesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_accessible_services", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AccessContextManagerServicePerimeterTimeoutsEl {
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

impl terrars::ToListMappable for AccessContextManagerServicePerimeterTimeoutsEl {
    type O = terrars::BlockAssignable<AccessContextManagerServicePerimeterTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeterTimeoutsEl {}

impl BuildAccessContextManagerServicePerimeterTimeoutsEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterTimeoutsEl {
        AccessContextManagerServicePerimeterTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimeterTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeterTimeoutsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeterTimeoutsElRef {
        AccessContextManagerServicePerimeterTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeterTimeoutsElRef {
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

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeterDynamic {
    spec: std::option::Option<terrars::DynamicBlock<AccessContextManagerServicePerimeterSpecEl>>,
    status: std::option::Option<terrars::DynamicBlock<AccessContextManagerServicePerimeterStatusEl>>,
}
