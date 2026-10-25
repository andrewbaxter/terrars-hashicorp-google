#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct BigqueryDatapolicyv2DataPolicyData_ {
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
    data_policy_id: terrars::PrimField<std::string::String>,
    data_policy_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    etag: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grantees: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_governance_tag: std::option::Option<std::vec::Vec<BigqueryDatapolicyv2DataPolicy_DataGovernanceTag>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_masking_policy: std::option::Option<std::vec::Vec<BigqueryDatapolicyv2DataPolicy_DataMaskingPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<BigqueryDatapolicyv2DataPolicy_Timeouts>,
    dynamic: BigqueryDatapolicyv2DataPolicyDynamic,
}

struct BigqueryDatapolicyv2DataPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BigqueryDatapolicyv2DataPolicyData_>,
}

#[derive(Clone)]
pub struct BigqueryDatapolicyv2DataPolicy(std::rc::Rc<BigqueryDatapolicyv2DataPolicy_>);

impl BigqueryDatapolicyv2DataPolicy {
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
        "Set the field `etag`.\nThe etag for this Data Policy.\nThis field is used for UpdateDataPolicy calls. If Data Policy exists, this\nfield is required and must match the server's etag. It will also be\npopulated in the response of GetDataPolicy, CreateDataPolicy, and\nUpdateDataPolicy calls."]
    pub fn set_etag(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().etag = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `grantees`.\nThe list of IAM principals that have Fine Grained Access to the underlying\ndata goverened by this data policy.\n\nUses the [IAM V2 principal\nsyntax](https://cloud.google.com/iam/docs/principal-identifiers#v2) Only\nsupports principal types users, groups, serviceaccounts, cloudidentity.\nThis field is supported in V2 Data Policy only. In case of V1 data policies\n(i.e. verion = 1 and policy_tag is set), this field is not populated."]
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

    #[doc = "Set the field `data_governance_tag`.\n"]
    pub fn set_data_governance_tag(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryDatapolicyv2DataPolicy_DataGovernanceTag>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_governance_tag = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_governance_tag = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `data_masking_policy`.\n"]
    pub fn set_data_masking_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryDatapolicyv2DataPolicy_DataMaskingPolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_masking_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_masking_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<BigqueryDatapolicyv2DataPolicy_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `data_policy_id` after provisioning.\nUser-assigned (human readable) ID of the data policy that needs to be\nunique within a project. Used as {data_policy_id} in part of the resource\nname."]
    pub fn data_policy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_policy_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_policy_type` after provisioning.\nType of data policy.\nPossible values:\nDATA_MASKING_POLICY\nRAW_DATA_ACCESS_POLICY\nCOLUMN_LEVEL_SECURITY_POLICY"]
    pub fn data_policy_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_policy_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nThe etag for this Data Policy.\nThis field is used for UpdateDataPolicy calls. If Data Policy exists, this\nfield is required and must match the server's etag. It will also be\npopulated in the response of GetDataPolicy, CreateDataPolicy, and\nUpdateDataPolicy calls."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `grantees` after provisioning.\nThe list of IAM principals that have Fine Grained Access to the underlying\ndata goverened by this data policy.\n\nUses the [IAM V2 principal\nsyntax](https://cloud.google.com/iam/docs/principal-identifiers#v2) Only\nsupports principal types users, groups, serviceaccounts, cloudidentity.\nThis field is supported in V2 Data Policy only. In case of V1 data policies\n(i.e. verion = 1 and policy_tag is set), this field is not populated."]
    pub fn grantees(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grantees", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. Resource name of this data policy, in the format of\n'projects/{project_number}/locations/{location_id}/dataPolicies/{data_policy_id}'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `policy_tag` after provisioning.\nPolicy tag resource name, in the format of\n'projects/{project_number}/locations/{location_id}/taxonomies/{taxonomy_id}/policyTags/{policyTag_id}'.\npolicy_tag is supported only for V1 data policies."]
    pub fn policy_tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_tag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nThe version of the Data Policy resource.\nPossible values:\nV1\nV2"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_governance_tag` after provisioning.\n"]
    pub fn data_governance_tag(&self) -> terrars::ListRef<BigqueryDatapolicyv2DataPolicy_DataGovernanceTagRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_governance_tag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_masking_policy` after provisioning.\n"]
    pub fn data_masking_policy(&self) -> terrars::ListRef<BigqueryDatapolicyv2DataPolicy_DataMaskingPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_masking_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigqueryDatapolicyv2DataPolicy_TimeoutsRef {
        BigqueryDatapolicyv2DataPolicy_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for BigqueryDatapolicyv2DataPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BigqueryDatapolicyv2DataPolicy { }

impl terrars::ToListMappable for BigqueryDatapolicyv2DataPolicy {
    type O = terrars::ListRef<BigqueryDatapolicyv2DataPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BigqueryDatapolicyv2DataPolicy_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_bigquery_datapolicyv2_data_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBigqueryDatapolicyv2DataPolicy {
    pub tf_id: std::string::String,
    #[doc =
        "User-assigned (human readable) ID of the data policy that needs to be\nunique within a project. Used as {data_policy_id} in part of the resource\nname."]
    pub data_policy_id: terrars::PrimField<std::string::String>,
    #[doc =
        "Type of data policy.\nPossible values:\nDATA_MASKING_POLICY\nRAW_DATA_ACCESS_POLICY\nCOLUMN_LEVEL_SECURITY_POLICY"]
    pub data_policy_type: terrars::PrimField<std::string::String>,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildBigqueryDatapolicyv2DataPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> BigqueryDatapolicyv2DataPolicy {
        let out = BigqueryDatapolicyv2DataPolicy(std::rc::Rc::new(BigqueryDatapolicyv2DataPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BigqueryDatapolicyv2DataPolicyData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                data_policy_id: self.data_policy_id,
                data_policy_type: self.data_policy_type,
                deletion_policy: core::default::Default::default(),
                etag: core::default::Default::default(),
                grantees: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                data_governance_tag: core::default::Default::default(),
                data_masking_policy: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BigqueryDatapolicyv2DataPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryDatapolicyv2DataPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BigqueryDatapolicyv2DataPolicyRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `data_policy_id` after provisioning.\nUser-assigned (human readable) ID of the data policy that needs to be\nunique within a project. Used as {data_policy_id} in part of the resource\nname."]
    pub fn data_policy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_policy_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_policy_type` after provisioning.\nType of data policy.\nPossible values:\nDATA_MASKING_POLICY\nRAW_DATA_ACCESS_POLICY\nCOLUMN_LEVEL_SECURITY_POLICY"]
    pub fn data_policy_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_policy_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nThe etag for this Data Policy.\nThis field is used for UpdateDataPolicy calls. If Data Policy exists, this\nfield is required and must match the server's etag. It will also be\npopulated in the response of GetDataPolicy, CreateDataPolicy, and\nUpdateDataPolicy calls."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `grantees` after provisioning.\nThe list of IAM principals that have Fine Grained Access to the underlying\ndata goverened by this data policy.\n\nUses the [IAM V2 principal\nsyntax](https://cloud.google.com/iam/docs/principal-identifiers#v2) Only\nsupports principal types users, groups, serviceaccounts, cloudidentity.\nThis field is supported in V2 Data Policy only. In case of V1 data policies\n(i.e. verion = 1 and policy_tag is set), this field is not populated."]
    pub fn grantees(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grantees", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. Resource name of this data policy, in the format of\n'projects/{project_number}/locations/{location_id}/dataPolicies/{data_policy_id}'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `policy_tag` after provisioning.\nPolicy tag resource name, in the format of\n'projects/{project_number}/locations/{location_id}/taxonomies/{taxonomy_id}/policyTags/{policyTag_id}'.\npolicy_tag is supported only for V1 data policies."]
    pub fn policy_tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_tag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nThe version of the Data Policy resource.\nPossible values:\nV1\nV2"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_governance_tag` after provisioning.\n"]
    pub fn data_governance_tag(&self) -> terrars::ListRef<BigqueryDatapolicyv2DataPolicy_DataGovernanceTagRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_governance_tag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_masking_policy` after provisioning.\n"]
    pub fn data_masking_policy(&self) -> terrars::ListRef<BigqueryDatapolicyv2DataPolicy_DataMaskingPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_masking_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigqueryDatapolicyv2DataPolicy_TimeoutsRef {
        BigqueryDatapolicyv2DataPolicy_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct BigqueryDatapolicyv2DataPolicyDynamic {
    data_governance_tag: std::option::Option<terrars::DynamicBlock<BigqueryDatapolicyv2DataPolicy_DataGovernanceTag>>,
    data_masking_policy: std
    ::option
    ::Option<terrars::DynamicBlock<BigqueryDatapolicyv2DataPolicy_DataMaskingPolicy>>,
}

#[derive(Serialize)]
pub struct BigqueryDatapolicyv2DataPolicy_DataGovernanceTag {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BigqueryDatapolicyv2DataPolicy_DataGovernanceTag {
    #[doc =
        "Set the field `key`.\nTag keys are globally unique. Tag key is expected to be in the namespaced format, for example \"parent-id/pii\" where \"parent-id\" is the ID of the parent organization or project resource for this tag key."]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\nTag value is expected to be the short name."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BigqueryDatapolicyv2DataPolicy_DataGovernanceTag {
    type O = terrars::BlockAssignable<BigqueryDatapolicyv2DataPolicy_DataGovernanceTag>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryDatapolicyv2DataPolicy_DataGovernanceTag {}

impl BuildBigqueryDatapolicyv2DataPolicy_DataGovernanceTag {
    pub fn build(self) -> BigqueryDatapolicyv2DataPolicy_DataGovernanceTag {
        BigqueryDatapolicyv2DataPolicy_DataGovernanceTag {
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryDatapolicyv2DataPolicy_DataGovernanceTagRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryDatapolicyv2DataPolicy_DataGovernanceTagRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BigqueryDatapolicyv2DataPolicy_DataGovernanceTagRef {
        BigqueryDatapolicyv2DataPolicy_DataGovernanceTagRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryDatapolicyv2DataPolicy_DataGovernanceTagRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `key` after provisioning.\nTag keys are globally unique. Tag key is expected to be in the namespaced format, for example \"parent-id/pii\" where \"parent-id\" is the ID of the parent organization or project resource for this tag key."]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nTag value is expected to be the short name."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryDatapolicyv2DataPolicy_DataMaskingPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    predefined_expression: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routine: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BigqueryDatapolicyv2DataPolicy_DataMaskingPolicy {
    #[doc =
        "Set the field `predefined_expression`.\nA predefined masking expression.\nPossible values:\nSHA256\nALWAYS_NULL\nDEFAULT_MASKING_VALUE\nLAST_FOUR_CHARACTERS\nFIRST_FOUR_CHARACTERS\nEMAIL_MASK\nDATE_YEAR_MASK\nRANDOM_HASH"]
    pub fn set_predefined_expression(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.predefined_expression = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `routine`.\nThe name of the BigQuery routine that contains the custom masking\nroutine, in the format of\n'projects/{project_number}/datasets/{dataset_id}/routines/{routine_id}'."]
    pub fn set_routine(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.routine = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BigqueryDatapolicyv2DataPolicy_DataMaskingPolicy {
    type O = terrars::BlockAssignable<BigqueryDatapolicyv2DataPolicy_DataMaskingPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryDatapolicyv2DataPolicy_DataMaskingPolicy {}

impl BuildBigqueryDatapolicyv2DataPolicy_DataMaskingPolicy {
    pub fn build(self) -> BigqueryDatapolicyv2DataPolicy_DataMaskingPolicy {
        BigqueryDatapolicyv2DataPolicy_DataMaskingPolicy {
            predefined_expression: core::default::Default::default(),
            routine: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryDatapolicyv2DataPolicy_DataMaskingPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryDatapolicyv2DataPolicy_DataMaskingPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BigqueryDatapolicyv2DataPolicy_DataMaskingPolicyRef {
        BigqueryDatapolicyv2DataPolicy_DataMaskingPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryDatapolicyv2DataPolicy_DataMaskingPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `predefined_expression` after provisioning.\nA predefined masking expression.\nPossible values:\nSHA256\nALWAYS_NULL\nDEFAULT_MASKING_VALUE\nLAST_FOUR_CHARACTERS\nFIRST_FOUR_CHARACTERS\nEMAIL_MASK\nDATE_YEAR_MASK\nRANDOM_HASH"]
    pub fn predefined_expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.predefined_expression", self.base))
    }

    #[doc =
        "Get a reference to the value of field `routine` after provisioning.\nThe name of the BigQuery routine that contains the custom masking\nroutine, in the format of\n'projects/{project_number}/datasets/{dataset_id}/routines/{routine_id}'."]
    pub fn routine(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.routine", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryDatapolicyv2DataPolicy_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BigqueryDatapolicyv2DataPolicy_Timeouts {
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

impl terrars::ToListMappable for BigqueryDatapolicyv2DataPolicy_Timeouts {
    type O = terrars::BlockAssignable<BigqueryDatapolicyv2DataPolicy_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryDatapolicyv2DataPolicy_Timeouts {}

impl BuildBigqueryDatapolicyv2DataPolicy_Timeouts {
    pub fn build(self) -> BigqueryDatapolicyv2DataPolicy_Timeouts {
        BigqueryDatapolicyv2DataPolicy_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryDatapolicyv2DataPolicy_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryDatapolicyv2DataPolicy_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryDatapolicyv2DataPolicy_TimeoutsRef {
        BigqueryDatapolicyv2DataPolicy_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryDatapolicyv2DataPolicy_TimeoutsRef {
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
