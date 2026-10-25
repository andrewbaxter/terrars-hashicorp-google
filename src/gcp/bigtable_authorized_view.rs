#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct BigtableAuthorizedViewData_ {
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
    deletion_protection: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    instance_name: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    table_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subset_view: std::option::Option<std::vec::Vec<BigtableAuthorizedView_SubsetView>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<BigtableAuthorizedView_Timeouts>,
    dynamic: BigtableAuthorizedViewDynamic,
}

struct BigtableAuthorizedView_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BigtableAuthorizedViewData_>,
}

#[derive(Clone)]
pub struct BigtableAuthorizedView(std::rc::Rc<BigtableAuthorizedView_>);

impl BigtableAuthorizedView {
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

    #[doc =
        "Set the field `deletion_protection`.\nA field to make the authorized view protected against data loss i.e. when set to PROTECTED, deleting the authorized view, the table containing the authorized view, and the instance containing the authorized view would be prohibited.\nIf not provided, currently deletion protection will be set to UNPROTECTED as it is the API default value. Note this field configs the deletion protection provided by the API in the backend, and should not be confused with Terraform-side deletion protection."]
    pub fn set_deletion_protection(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_protection = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `project`.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subset_view`.\n"]
    pub fn set_subset_view(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigtableAuthorizedView_SubsetView>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().subset_view = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.subset_view = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<BigtableAuthorizedView_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_protection` after provisioning.\nA field to make the authorized view protected against data loss i.e. when set to PROTECTED, deleting the authorized view, the table containing the authorized view, and the instance containing the authorized view would be prohibited.\nIf not provided, currently deletion protection will be set to UNPROTECTED as it is the API default value. Note this field configs the deletion protection provided by the API in the backend, and should not be confused with Terraform-side deletion protection."]
    pub fn deletion_protection(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance_name` after provisioning.\nThe name of the Bigtable instance in which the authorized view belongs."]
    pub fn instance_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the authorized view. Must be 1-50 characters and must only contain hyphens, underscores, periods, letters and numbers."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `table_name` after provisioning.\nThe name of the Bigtable table in which the authorized view belongs."]
    pub fn table_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subset_view` after provisioning.\n"]
    pub fn subset_view(&self) -> terrars::ListRef<BigtableAuthorizedView_SubsetViewRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subset_view", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigtableAuthorizedView_TimeoutsRef {
        BigtableAuthorizedView_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for BigtableAuthorizedView {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BigtableAuthorizedView { }

impl terrars::ToListMappable for BigtableAuthorizedView {
    type O = terrars::ListRef<BigtableAuthorizedViewRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BigtableAuthorizedView_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_bigtable_authorized_view".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBigtableAuthorizedView {
    pub tf_id: std::string::String,
    #[doc = "The name of the Bigtable instance in which the authorized view belongs."]
    pub instance_name: terrars::PrimField<std::string::String>,
    #[doc =
        "The name of the authorized view. Must be 1-50 characters and must only contain hyphens, underscores, periods, letters and numbers."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The name of the Bigtable table in which the authorized view belongs."]
    pub table_name: terrars::PrimField<std::string::String>,
}

impl BuildBigtableAuthorizedView {
    pub fn build(self, stack: &mut terrars::Stack) -> BigtableAuthorizedView {
        let out = BigtableAuthorizedView(std::rc::Rc::new(BigtableAuthorizedView_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BigtableAuthorizedViewData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                deletion_protection: core::default::Default::default(),
                id: core::default::Default::default(),
                instance_name: self.instance_name,
                name: self.name,
                project: core::default::Default::default(),
                table_name: self.table_name,
                subset_view: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BigtableAuthorizedViewRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigtableAuthorizedViewRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BigtableAuthorizedViewRef {
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

    #[doc =
        "Get a reference to the value of field `deletion_protection` after provisioning.\nA field to make the authorized view protected against data loss i.e. when set to PROTECTED, deleting the authorized view, the table containing the authorized view, and the instance containing the authorized view would be prohibited.\nIf not provided, currently deletion protection will be set to UNPROTECTED as it is the API default value. Note this field configs the deletion protection provided by the API in the backend, and should not be confused with Terraform-side deletion protection."]
    pub fn deletion_protection(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance_name` after provisioning.\nThe name of the Bigtable instance in which the authorized view belongs."]
    pub fn instance_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the authorized view. Must be 1-50 characters and must only contain hyphens, underscores, periods, letters and numbers."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `table_name` after provisioning.\nThe name of the Bigtable table in which the authorized view belongs."]
    pub fn table_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subset_view` after provisioning.\n"]
    pub fn subset_view(&self) -> terrars::ListRef<BigtableAuthorizedView_SubsetViewRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subset_view", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigtableAuthorizedView_TimeoutsRef {
        BigtableAuthorizedView_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct BigtableAuthorizedViewDynamic {
    subset_view: std::option::Option<terrars::DynamicBlock<BigtableAuthorizedView_SubsetView>>,
}

#[derive(Serialize)]
pub struct BigtableAuthorizedView_SubsetView_FamilySubsets {
    family_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    qualifier_prefixes: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    qualifiers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl BigtableAuthorizedView_SubsetView_FamilySubsets {
    #[doc =
        "Set the field `qualifier_prefixes`.\nBase64-encoded prefixes for qualifiers of the column family to be included in the authorized view. Every qualifier starting with one of these prefixes is included in the authorized view. To provide access to all qualifiers, include the empty string as a prefix (\"\")."]
    pub fn set_qualifier_prefixes(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.qualifier_prefixes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `qualifiers`.\nBase64-encoded individual exact column qualifiers of the column family to be included in the authorized view."]
    pub fn set_qualifiers(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.qualifiers = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BigtableAuthorizedView_SubsetView_FamilySubsets {
    type O = terrars::BlockAssignable<BigtableAuthorizedView_SubsetView_FamilySubsets>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigtableAuthorizedView_SubsetView_FamilySubsets {
    #[doc = "Name of the column family to be included in the authorized view."]
    pub family_name: terrars::PrimField<std::string::String>,
}

impl BuildBigtableAuthorizedView_SubsetView_FamilySubsets {
    pub fn build(self) -> BigtableAuthorizedView_SubsetView_FamilySubsets {
        BigtableAuthorizedView_SubsetView_FamilySubsets {
            family_name: self.family_name,
            qualifier_prefixes: core::default::Default::default(),
            qualifiers: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigtableAuthorizedView_SubsetView_FamilySubsetsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigtableAuthorizedView_SubsetView_FamilySubsetsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BigtableAuthorizedView_SubsetView_FamilySubsetsRef {
        BigtableAuthorizedView_SubsetView_FamilySubsetsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigtableAuthorizedView_SubsetView_FamilySubsetsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `family_name` after provisioning.\nName of the column family to be included in the authorized view."]
    pub fn family_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.family_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `qualifier_prefixes` after provisioning.\nBase64-encoded prefixes for qualifiers of the column family to be included in the authorized view. Every qualifier starting with one of these prefixes is included in the authorized view. To provide access to all qualifiers, include the empty string as a prefix (\"\")."]
    pub fn qualifier_prefixes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.qualifier_prefixes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `qualifiers` after provisioning.\nBase64-encoded individual exact column qualifiers of the column family to be included in the authorized view."]
    pub fn qualifiers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.qualifiers", self.base))
    }
}

#[derive(Serialize, Default)]
struct BigtableAuthorizedView_SubsetViewDynamic {
    family_subsets: std::option::Option<terrars::DynamicBlock<BigtableAuthorizedView_SubsetView_FamilySubsets>>,
}

#[derive(Serialize)]
pub struct BigtableAuthorizedView_SubsetView {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    row_prefixes: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    family_subsets: std::option::Option<std::vec::Vec<BigtableAuthorizedView_SubsetView_FamilySubsets>>,
    dynamic: BigtableAuthorizedView_SubsetViewDynamic,
}

impl BigtableAuthorizedView_SubsetView {
    #[doc =
        "Set the field `row_prefixes`.\nBase64-encoded row prefixes to be included in the authorized view. To provide access to all rows, include the empty string as a prefix (\"\")."]
    pub fn set_row_prefixes(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.row_prefixes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `family_subsets`.\n"]
    pub fn set_family_subsets(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigtableAuthorizedView_SubsetView_FamilySubsets>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.family_subsets = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.family_subsets = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BigtableAuthorizedView_SubsetView {
    type O = terrars::BlockAssignable<BigtableAuthorizedView_SubsetView>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigtableAuthorizedView_SubsetView {}

impl BuildBigtableAuthorizedView_SubsetView {
    pub fn build(self) -> BigtableAuthorizedView_SubsetView {
        BigtableAuthorizedView_SubsetView {
            row_prefixes: core::default::Default::default(),
            family_subsets: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigtableAuthorizedView_SubsetViewRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigtableAuthorizedView_SubsetViewRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigtableAuthorizedView_SubsetViewRef {
        BigtableAuthorizedView_SubsetViewRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigtableAuthorizedView_SubsetViewRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `row_prefixes` after provisioning.\nBase64-encoded row prefixes to be included in the authorized view. To provide access to all rows, include the empty string as a prefix (\"\")."]
    pub fn row_prefixes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.row_prefixes", self.base))
    }
}

#[derive(Serialize)]
pub struct BigtableAuthorizedView_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BigtableAuthorizedView_Timeouts {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BigtableAuthorizedView_Timeouts {
    type O = terrars::BlockAssignable<BigtableAuthorizedView_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigtableAuthorizedView_Timeouts {}

impl BuildBigtableAuthorizedView_Timeouts {
    pub fn build(self) -> BigtableAuthorizedView_Timeouts {
        BigtableAuthorizedView_Timeouts {
            create: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigtableAuthorizedView_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigtableAuthorizedView_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigtableAuthorizedView_TimeoutsRef {
        BigtableAuthorizedView_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigtableAuthorizedView_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
