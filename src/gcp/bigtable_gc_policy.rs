use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct BigtableGcPolicyData {
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
    column_family: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gc_rules: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    instance_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    table: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_age: std::option::Option<std::vec::Vec<BigtableGcPolicyMaxAgeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_version: std::option::Option<std::vec::Vec<BigtableGcPolicyMaxVersionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<BigtableGcPolicyTimeoutsEl>,
    dynamic: BigtableGcPolicyDynamic,
}

struct BigtableGcPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BigtableGcPolicyData>,
}

#[derive(Clone)]
pub struct BigtableGcPolicy(std::rc::Rc<BigtableGcPolicy_>);

impl BigtableGcPolicy {
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
        "Set the field `deletion_policy`.\nThe deletion policy for the GC policy. Setting ABANDON allows the resource\n\t\t\t\tto be abandoned rather than deleted. This is useful for GC policy as it cannot be deleted\n\t\t\t\tin a replicated instance. Possible values are: \"ABANDON\"."]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `gc_rules`.\nSerialized JSON string for garbage collection policy. Conflicts with \"mode\", \"max_age\" and \"max_version\"."]
    pub fn set_gc_rules(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().gc_rules = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `mode`.\nNOTE: 'gc_rules' is more flexible, and should be preferred over this field for new resources. This field may be deprecated in the future. If multiple policies are set, you should choose between UNION OR INTERSECTION."]
    pub fn set_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `project`.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_age`.\n"]
    pub fn set_max_age(self, v: impl std::convert::Into<terrars::BlockAssignable<BigtableGcPolicyMaxAgeEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().max_age = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.max_age = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `max_version`.\n"]
    pub fn set_max_version(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigtableGcPolicyMaxVersionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().max_version = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.max_version = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<BigtableGcPolicyTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `column_family` after provisioning.\nThe name of the column family."]
    pub fn column_family(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.column_family", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nThe deletion policy for the GC policy. Setting ABANDON allows the resource\n\t\t\t\tto be abandoned rather than deleted. This is useful for GC policy as it cannot be deleted\n\t\t\t\tin a replicated instance. Possible values are: \"ABANDON\"."]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `gc_rules` after provisioning.\nSerialized JSON string for garbage collection policy. Conflicts with \"mode\", \"max_age\" and \"max_version\"."]
    pub fn gc_rules(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gc_rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance_name` after provisioning.\nThe name of the Bigtable instance."]
    pub fn instance_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nNOTE: 'gc_rules' is more flexible, and should be preferred over this field for new resources. This field may be deprecated in the future. If multiple policies are set, you should choose between UNION OR INTERSECTION."]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `table` after provisioning.\nThe name of the table."]
    pub fn table(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_age` after provisioning.\n"]
    pub fn max_age(&self) -> terrars::ListRef<BigtableGcPolicyMaxAgeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.max_age", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_version` after provisioning.\n"]
    pub fn max_version(&self) -> terrars::ListRef<BigtableGcPolicyMaxVersionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.max_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigtableGcPolicyTimeoutsElRef {
        BigtableGcPolicyTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for BigtableGcPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BigtableGcPolicy { }

impl terrars::ToListMappable for BigtableGcPolicy {
    type O = terrars::ListRef<BigtableGcPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BigtableGcPolicy_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_bigtable_gc_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBigtableGcPolicy {
    pub tf_id: std::string::String,
    #[doc = "The name of the column family."]
    pub column_family: terrars::PrimField<std::string::String>,
    #[doc = "The name of the Bigtable instance."]
    pub instance_name: terrars::PrimField<std::string::String>,
    #[doc = "The name of the table."]
    pub table: terrars::PrimField<std::string::String>,
}

impl BuildBigtableGcPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> BigtableGcPolicy {
        let out = BigtableGcPolicy(std::rc::Rc::new(BigtableGcPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BigtableGcPolicyData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                column_family: self.column_family,
                deletion_policy: core::default::Default::default(),
                gc_rules: core::default::Default::default(),
                id: core::default::Default::default(),
                instance_name: self.instance_name,
                mode: core::default::Default::default(),
                project: core::default::Default::default(),
                table: self.table,
                max_age: core::default::Default::default(),
                max_version: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BigtableGcPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigtableGcPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BigtableGcPolicyRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `column_family` after provisioning.\nThe name of the column family."]
    pub fn column_family(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.column_family", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nThe deletion policy for the GC policy. Setting ABANDON allows the resource\n\t\t\t\tto be abandoned rather than deleted. This is useful for GC policy as it cannot be deleted\n\t\t\t\tin a replicated instance. Possible values are: \"ABANDON\"."]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `gc_rules` after provisioning.\nSerialized JSON string for garbage collection policy. Conflicts with \"mode\", \"max_age\" and \"max_version\"."]
    pub fn gc_rules(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gc_rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance_name` after provisioning.\nThe name of the Bigtable instance."]
    pub fn instance_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nNOTE: 'gc_rules' is more flexible, and should be preferred over this field for new resources. This field may be deprecated in the future. If multiple policies are set, you should choose between UNION OR INTERSECTION."]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `table` after provisioning.\nThe name of the table."]
    pub fn table(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_age` after provisioning.\n"]
    pub fn max_age(&self) -> terrars::ListRef<BigtableGcPolicyMaxAgeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.max_age", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_version` after provisioning.\n"]
    pub fn max_version(&self) -> terrars::ListRef<BigtableGcPolicyMaxVersionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.max_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigtableGcPolicyTimeoutsElRef {
        BigtableGcPolicyTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct BigtableGcPolicyMaxAgeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    days: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    duration: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BigtableGcPolicyMaxAgeEl {
    #[doc = "Set the field `days`.\nNumber of days before applying GC policy."]
    pub fn set_days(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.days = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `duration`.\nDuration before applying GC policy"]
    pub fn set_duration(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.duration = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BigtableGcPolicyMaxAgeEl {
    type O = terrars::BlockAssignable<BigtableGcPolicyMaxAgeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigtableGcPolicyMaxAgeEl {}

impl BuildBigtableGcPolicyMaxAgeEl {
    pub fn build(self) -> BigtableGcPolicyMaxAgeEl {
        BigtableGcPolicyMaxAgeEl {
            days: core::default::Default::default(),
            duration: core::default::Default::default(),
        }
    }
}

pub struct BigtableGcPolicyMaxAgeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigtableGcPolicyMaxAgeElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigtableGcPolicyMaxAgeElRef {
        BigtableGcPolicyMaxAgeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigtableGcPolicyMaxAgeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `days` after provisioning.\nNumber of days before applying GC policy."]
    pub fn days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.days", self.base))
    }

    #[doc = "Get a reference to the value of field `duration` after provisioning.\nDuration before applying GC policy"]
    pub fn duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.duration", self.base))
    }
}

#[derive(Serialize)]
pub struct BigtableGcPolicyMaxVersionEl {
    number: terrars::PrimField<f64>,
}

impl BigtableGcPolicyMaxVersionEl { }

impl terrars::ToListMappable for BigtableGcPolicyMaxVersionEl {
    type O = terrars::BlockAssignable<BigtableGcPolicyMaxVersionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigtableGcPolicyMaxVersionEl {
    #[doc = "Number of version before applying the GC policy."]
    pub number: terrars::PrimField<f64>,
}

impl BuildBigtableGcPolicyMaxVersionEl {
    pub fn build(self) -> BigtableGcPolicyMaxVersionEl {
        BigtableGcPolicyMaxVersionEl { number: self.number }
    }
}

pub struct BigtableGcPolicyMaxVersionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigtableGcPolicyMaxVersionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigtableGcPolicyMaxVersionElRef {
        BigtableGcPolicyMaxVersionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigtableGcPolicyMaxVersionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `number` after provisioning.\nNumber of version before applying the GC policy."]
    pub fn number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.number", self.base))
    }
}

#[derive(Serialize)]
pub struct BigtableGcPolicyTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BigtableGcPolicyTimeoutsEl {
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

impl terrars::ToListMappable for BigtableGcPolicyTimeoutsEl {
    type O = terrars::BlockAssignable<BigtableGcPolicyTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigtableGcPolicyTimeoutsEl {}

impl BuildBigtableGcPolicyTimeoutsEl {
    pub fn build(self) -> BigtableGcPolicyTimeoutsEl {
        BigtableGcPolicyTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct BigtableGcPolicyTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigtableGcPolicyTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigtableGcPolicyTimeoutsElRef {
        BigtableGcPolicyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigtableGcPolicyTimeoutsElRef {
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

#[derive(Serialize, Default)]
struct BigtableGcPolicyDynamic {
    max_age: std::option::Option<terrars::DynamicBlock<BigtableGcPolicyMaxAgeEl>>,
    max_version: std::option::Option<terrars::DynamicBlock<BigtableGcPolicyMaxVersionEl>>,
}
