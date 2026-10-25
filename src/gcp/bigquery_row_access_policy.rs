#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct BigqueryRowAccessPolicyData_ {
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
    dataset_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    filter_predicate: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grantees: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    policy_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    table_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<BigqueryRowAccessPolicy_Timeouts>,
}

struct BigqueryRowAccessPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BigqueryRowAccessPolicyData_>,
}

#[derive(Clone)]
pub struct BigqueryRowAccessPolicy(std::rc::Rc<BigqueryRowAccessPolicy_>);

impl BigqueryRowAccessPolicy {
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
        "Set the field `grantees`.\nInput only. The optional list of iam_member users or groups that specifies the initial\nmembers that the row-level access policy should be created with.\n\ngrantees types:\n- \"user:alice@example.com\": An email address that represents a specific\nGoogle account.\n- \"serviceAccount:my-other-app@appspot.gserviceaccount.com\": An email\naddress that represents a service account.\n- \"group:admins@example.com\": An email address that represents a Google\ngroup.\n- \"domain:example.com\":The Google Workspace domain (primary) that\nrepresents all the users of that domain.\n- \"allAuthenticatedUsers\": A special identifier that represents all service\naccounts and all users on the internet who have authenticated with a Google\nAccount. This identifier includes accounts that aren't connected to a\nGoogle Workspace or Cloud Identity domain, such as personal Gmail accounts.\nUsers who aren't authenticated, such as anonymous visitors, aren't\nincluded.\n- \"allUsers\":A special identifier that represents anyone who is on\nthe internet, including authenticated and unauthenticated users. Because\nBigQuery requires authentication before a user can access the service,\nallUsers includes only authenticated users."]
    pub fn set_grantees(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().grantees = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<BigqueryRowAccessPolicy_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `creation_time` after provisioning.\nThe time when this row access policy was created, in milliseconds since\nthe epoch."]
    pub fn creation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dataset_id` after provisioning.\nThe ID of the dataset containing this row access policy."]
    pub fn dataset_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `filter_predicate` after provisioning.\nA SQL boolean expression that represents the rows defined by this row\naccess policy, similar to the boolean expression in a WHERE clause of a\nSELECT query on a table.\nReferences to other tables, routines, and temporary functions are not\nsupported.\n\nExamples: region=\"EU\"\ndate_field = CAST('2019-9-27' as DATE)\nnullable_field is not NULL\nnumeric_field BETWEEN 1.0 AND 5.0"]
    pub fn filter_predicate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter_predicate", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `grantees` after provisioning.\nInput only. The optional list of iam_member users or groups that specifies the initial\nmembers that the row-level access policy should be created with.\n\ngrantees types:\n- \"user:alice@example.com\": An email address that represents a specific\nGoogle account.\n- \"serviceAccount:my-other-app@appspot.gserviceaccount.com\": An email\naddress that represents a service account.\n- \"group:admins@example.com\": An email address that represents a Google\ngroup.\n- \"domain:example.com\":The Google Workspace domain (primary) that\nrepresents all the users of that domain.\n- \"allAuthenticatedUsers\": A special identifier that represents all service\naccounts and all users on the internet who have authenticated with a Google\nAccount. This identifier includes accounts that aren't connected to a\nGoogle Workspace or Cloud Identity domain, such as personal Gmail accounts.\nUsers who aren't authenticated, such as anonymous visitors, aren't\nincluded.\n- \"allUsers\":A special identifier that represents anyone who is on\nthe internet, including authenticated and unauthenticated users. Because\nBigQuery requires authentication before a user can access the service,\nallUsers includes only authenticated users."]
    pub fn grantees(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grantees", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_modified_time` after provisioning.\nThe time when this row access policy was last modified, in milliseconds\nsince the epoch."]
    pub fn last_modified_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `policy_id` after provisioning.\nThe ID of the row access policy. The ID must contain only\nletters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum\nlength is 256 characters."]
    pub fn policy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `table_id` after provisioning.\nThe ID of the table containing this row access policy."]
    pub fn table_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigqueryRowAccessPolicy_TimeoutsRef {
        BigqueryRowAccessPolicy_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for BigqueryRowAccessPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BigqueryRowAccessPolicy { }

impl terrars::ToListMappable for BigqueryRowAccessPolicy {
    type O = terrars::ListRef<BigqueryRowAccessPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BigqueryRowAccessPolicy_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_bigquery_row_access_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBigqueryRowAccessPolicy {
    pub tf_id: std::string::String,
    #[doc = "The ID of the dataset containing this row access policy."]
    pub dataset_id: terrars::PrimField<std::string::String>,
    #[doc =
        "A SQL boolean expression that represents the rows defined by this row\naccess policy, similar to the boolean expression in a WHERE clause of a\nSELECT query on a table.\nReferences to other tables, routines, and temporary functions are not\nsupported.\n\nExamples: region=\"EU\"\ndate_field = CAST('2019-9-27' as DATE)\nnullable_field is not NULL\nnumeric_field BETWEEN 1.0 AND 5.0"]
    pub filter_predicate: terrars::PrimField<std::string::String>,
    #[doc =
        "The ID of the row access policy. The ID must contain only\nletters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum\nlength is 256 characters."]
    pub policy_id: terrars::PrimField<std::string::String>,
    #[doc = "The ID of the table containing this row access policy."]
    pub table_id: terrars::PrimField<std::string::String>,
}

impl BuildBigqueryRowAccessPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> BigqueryRowAccessPolicy {
        let out = BigqueryRowAccessPolicy(std::rc::Rc::new(BigqueryRowAccessPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BigqueryRowAccessPolicyData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                dataset_id: self.dataset_id,
                deletion_policy: core::default::Default::default(),
                filter_predicate: self.filter_predicate,
                grantees: core::default::Default::default(),
                id: core::default::Default::default(),
                policy_id: self.policy_id,
                project: core::default::Default::default(),
                table_id: self.table_id,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BigqueryRowAccessPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryRowAccessPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BigqueryRowAccessPolicyRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `creation_time` after provisioning.\nThe time when this row access policy was created, in milliseconds since\nthe epoch."]
    pub fn creation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dataset_id` after provisioning.\nThe ID of the dataset containing this row access policy."]
    pub fn dataset_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `filter_predicate` after provisioning.\nA SQL boolean expression that represents the rows defined by this row\naccess policy, similar to the boolean expression in a WHERE clause of a\nSELECT query on a table.\nReferences to other tables, routines, and temporary functions are not\nsupported.\n\nExamples: region=\"EU\"\ndate_field = CAST('2019-9-27' as DATE)\nnullable_field is not NULL\nnumeric_field BETWEEN 1.0 AND 5.0"]
    pub fn filter_predicate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter_predicate", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `grantees` after provisioning.\nInput only. The optional list of iam_member users or groups that specifies the initial\nmembers that the row-level access policy should be created with.\n\ngrantees types:\n- \"user:alice@example.com\": An email address that represents a specific\nGoogle account.\n- \"serviceAccount:my-other-app@appspot.gserviceaccount.com\": An email\naddress that represents a service account.\n- \"group:admins@example.com\": An email address that represents a Google\ngroup.\n- \"domain:example.com\":The Google Workspace domain (primary) that\nrepresents all the users of that domain.\n- \"allAuthenticatedUsers\": A special identifier that represents all service\naccounts and all users on the internet who have authenticated with a Google\nAccount. This identifier includes accounts that aren't connected to a\nGoogle Workspace or Cloud Identity domain, such as personal Gmail accounts.\nUsers who aren't authenticated, such as anonymous visitors, aren't\nincluded.\n- \"allUsers\":A special identifier that represents anyone who is on\nthe internet, including authenticated and unauthenticated users. Because\nBigQuery requires authentication before a user can access the service,\nallUsers includes only authenticated users."]
    pub fn grantees(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grantees", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_modified_time` after provisioning.\nThe time when this row access policy was last modified, in milliseconds\nsince the epoch."]
    pub fn last_modified_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `policy_id` after provisioning.\nThe ID of the row access policy. The ID must contain only\nletters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum\nlength is 256 characters."]
    pub fn policy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `table_id` after provisioning.\nThe ID of the table containing this row access policy."]
    pub fn table_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigqueryRowAccessPolicy_TimeoutsRef {
        BigqueryRowAccessPolicy_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct BigqueryRowAccessPolicy_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BigqueryRowAccessPolicy_Timeouts {
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

impl terrars::ToListMappable for BigqueryRowAccessPolicy_Timeouts {
    type O = terrars::BlockAssignable<BigqueryRowAccessPolicy_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryRowAccessPolicy_Timeouts {}

impl BuildBigqueryRowAccessPolicy_Timeouts {
    pub fn build(self) -> BigqueryRowAccessPolicy_Timeouts {
        BigqueryRowAccessPolicy_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryRowAccessPolicy_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryRowAccessPolicy_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryRowAccessPolicy_TimeoutsRef {
        BigqueryRowAccessPolicy_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryRowAccessPolicy_TimeoutsRef {
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
