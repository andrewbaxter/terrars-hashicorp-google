use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct BinaryAuthorizationPolicyData {
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
    global_policy_evaluation_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    admission_whitelist_patterns: std
    ::option
    ::Option<std::vec::Vec<BinaryAuthorizationPolicyAdmissionWhitelistPatternsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_admission_rules: std::option::Option<std::vec::Vec<BinaryAuthorizationPolicyClusterAdmissionRulesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_admission_rule: std::option::Option<std::vec::Vec<BinaryAuthorizationPolicyDefaultAdmissionRuleEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<BinaryAuthorizationPolicyTimeoutsEl>,
    dynamic: BinaryAuthorizationPolicyDynamic,
}

struct BinaryAuthorizationPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BinaryAuthorizationPolicyData>,
}

#[derive(Clone)]
pub struct BinaryAuthorizationPolicy(std::rc::Rc<BinaryAuthorizationPolicy_>);

impl BinaryAuthorizationPolicy {
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

    #[doc = "Set the field `description`.\nA descriptive comment."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `global_policy_evaluation_mode`.\nControls the evaluation of a Google-maintained global admission policy\nfor common system-level images. Images not covered by the global\npolicy will be subject to the project admission policy. Possible values: [\"ENABLE\", \"DISABLE\"]"]
    pub fn set_global_policy_evaluation_mode(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().global_policy_evaluation_mode = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `admission_whitelist_patterns`.\n"]
    pub fn set_admission_whitelist_patterns(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BinaryAuthorizationPolicyAdmissionWhitelistPatternsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().admission_whitelist_patterns = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.admission_whitelist_patterns = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cluster_admission_rules`.\n"]
    pub fn set_cluster_admission_rules(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BinaryAuthorizationPolicyClusterAdmissionRulesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cluster_admission_rules = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cluster_admission_rules = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `default_admission_rule`.\n"]
    pub fn set_default_admission_rule(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BinaryAuthorizationPolicyDefaultAdmissionRuleEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().default_admission_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.default_admission_rule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<BinaryAuthorizationPolicyTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nA descriptive comment."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `global_policy_evaluation_mode` after provisioning.\nControls the evaluation of a Google-maintained global admission policy\nfor common system-level images. Images not covered by the global\npolicy will be subject to the project admission policy. Possible values: [\"ENABLE\", \"DISABLE\"]"]
    pub fn global_policy_evaluation_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.global_policy_evaluation_mode", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `admission_whitelist_patterns` after provisioning.\n"]
    pub fn admission_whitelist_patterns(
        &self,
    ) -> terrars::ListRef<BinaryAuthorizationPolicyAdmissionWhitelistPatternsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.admission_whitelist_patterns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_admission_rule` after provisioning.\n"]
    pub fn default_admission_rule(&self) -> terrars::ListRef<BinaryAuthorizationPolicyDefaultAdmissionRuleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_admission_rule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BinaryAuthorizationPolicyTimeoutsElRef {
        BinaryAuthorizationPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for BinaryAuthorizationPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BinaryAuthorizationPolicy { }

impl terrars::ToListMappable for BinaryAuthorizationPolicy {
    type O = terrars::ListRef<BinaryAuthorizationPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BinaryAuthorizationPolicy_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_binary_authorization_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBinaryAuthorizationPolicy {
    pub tf_id: std::string::String,
}

impl BuildBinaryAuthorizationPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> BinaryAuthorizationPolicy {
        let out = BinaryAuthorizationPolicy(std::rc::Rc::new(BinaryAuthorizationPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BinaryAuthorizationPolicyData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                global_policy_evaluation_mode: core::default::Default::default(),
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                admission_whitelist_patterns: core::default::Default::default(),
                cluster_admission_rules: core::default::Default::default(),
                default_admission_rule: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BinaryAuthorizationPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BinaryAuthorizationPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BinaryAuthorizationPolicyRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nA descriptive comment."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `global_policy_evaluation_mode` after provisioning.\nControls the evaluation of a Google-maintained global admission policy\nfor common system-level images. Images not covered by the global\npolicy will be subject to the project admission policy. Possible values: [\"ENABLE\", \"DISABLE\"]"]
    pub fn global_policy_evaluation_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.global_policy_evaluation_mode", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `admission_whitelist_patterns` after provisioning.\n"]
    pub fn admission_whitelist_patterns(
        &self,
    ) -> terrars::ListRef<BinaryAuthorizationPolicyAdmissionWhitelistPatternsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.admission_whitelist_patterns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_admission_rule` after provisioning.\n"]
    pub fn default_admission_rule(&self) -> terrars::ListRef<BinaryAuthorizationPolicyDefaultAdmissionRuleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_admission_rule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BinaryAuthorizationPolicyTimeoutsElRef {
        BinaryAuthorizationPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct BinaryAuthorizationPolicyAdmissionWhitelistPatternsEl {
    name_pattern: terrars::PrimField<std::string::String>,
}

impl BinaryAuthorizationPolicyAdmissionWhitelistPatternsEl { }

impl terrars::ToListMappable for BinaryAuthorizationPolicyAdmissionWhitelistPatternsEl {
    type O = terrars::BlockAssignable<BinaryAuthorizationPolicyAdmissionWhitelistPatternsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBinaryAuthorizationPolicyAdmissionWhitelistPatternsEl {
    #[doc =
        "An image name pattern to whitelist, in the form\n'registry/path/to/image'. This supports a trailing * as a\nwildcard, but this is allowed only in text after the registry/\npart."]
    pub name_pattern: terrars::PrimField<std::string::String>,
}

impl BuildBinaryAuthorizationPolicyAdmissionWhitelistPatternsEl {
    pub fn build(self) -> BinaryAuthorizationPolicyAdmissionWhitelistPatternsEl {
        BinaryAuthorizationPolicyAdmissionWhitelistPatternsEl { name_pattern: self.name_pattern }
    }
}

pub struct BinaryAuthorizationPolicyAdmissionWhitelistPatternsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BinaryAuthorizationPolicyAdmissionWhitelistPatternsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BinaryAuthorizationPolicyAdmissionWhitelistPatternsElRef {
        BinaryAuthorizationPolicyAdmissionWhitelistPatternsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BinaryAuthorizationPolicyAdmissionWhitelistPatternsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name_pattern` after provisioning.\nAn image name pattern to whitelist, in the form\n'registry/path/to/image'. This supports a trailing * as a\nwildcard, but this is allowed only in text after the registry/\npart."]
    pub fn name_pattern(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_pattern", self.base))
    }
}

#[derive(Serialize)]
pub struct BinaryAuthorizationPolicyClusterAdmissionRulesEl {
    cluster: terrars::PrimField<std::string::String>,
    enforcement_mode: terrars::PrimField<std::string::String>,
    evaluation_mode: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    require_attestations_by: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl BinaryAuthorizationPolicyClusterAdmissionRulesEl {
    #[doc =
        "Set the field `require_attestations_by`.\nThe resource names of the attestors that must attest to a\ncontainer image. If the attestor is in a different project from the\npolicy, it should be specified in the format 'projects/*/attestors/*'.\nEach attestor must exist before a policy can reference it. To add an\nattestor to a policy the principal issuing the policy change\nrequest must be able to read the attestor resource.\n\nNote: this field must be non-empty when the evaluation_mode field\nspecifies REQUIRE_ATTESTATION, otherwise it must be empty."]
    pub fn set_require_attestations_by(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.require_attestations_by = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BinaryAuthorizationPolicyClusterAdmissionRulesEl {
    type O = terrars::BlockAssignable<BinaryAuthorizationPolicyClusterAdmissionRulesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBinaryAuthorizationPolicyClusterAdmissionRulesEl {
    #[doc = ""]
    pub cluster: terrars::PrimField<std::string::String>,
    #[doc =
        "The action when a pod creation is denied by the admission rule. Possible values: [\"ENFORCED_BLOCK_AND_AUDIT_LOG\", \"DRYRUN_AUDIT_LOG_ONLY\"]"]
    pub enforcement_mode: terrars::PrimField<std::string::String>,
    #[doc =
        "How this admission rule will be evaluated. Possible values: [\"ALWAYS_ALLOW\", \"REQUIRE_ATTESTATION\", \"ALWAYS_DENY\"]"]
    pub evaluation_mode: terrars::PrimField<std::string::String>,
}

impl BuildBinaryAuthorizationPolicyClusterAdmissionRulesEl {
    pub fn build(self) -> BinaryAuthorizationPolicyClusterAdmissionRulesEl {
        BinaryAuthorizationPolicyClusterAdmissionRulesEl {
            cluster: self.cluster,
            enforcement_mode: self.enforcement_mode,
            evaluation_mode: self.evaluation_mode,
            require_attestations_by: core::default::Default::default(),
        }
    }
}

pub struct BinaryAuthorizationPolicyClusterAdmissionRulesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BinaryAuthorizationPolicyClusterAdmissionRulesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BinaryAuthorizationPolicyClusterAdmissionRulesElRef {
        BinaryAuthorizationPolicyClusterAdmissionRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BinaryAuthorizationPolicyClusterAdmissionRulesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cluster` after provisioning.\n"]
    pub fn cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enforcement_mode` after provisioning.\nThe action when a pod creation is denied by the admission rule. Possible values: [\"ENFORCED_BLOCK_AND_AUDIT_LOG\", \"DRYRUN_AUDIT_LOG_ONLY\"]"]
    pub fn enforcement_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enforcement_mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `evaluation_mode` after provisioning.\nHow this admission rule will be evaluated. Possible values: [\"ALWAYS_ALLOW\", \"REQUIRE_ATTESTATION\", \"ALWAYS_DENY\"]"]
    pub fn evaluation_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.evaluation_mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `require_attestations_by` after provisioning.\nThe resource names of the attestors that must attest to a\ncontainer image. If the attestor is in a different project from the\npolicy, it should be specified in the format 'projects/*/attestors/*'.\nEach attestor must exist before a policy can reference it. To add an\nattestor to a policy the principal issuing the policy change\nrequest must be able to read the attestor resource.\n\nNote: this field must be non-empty when the evaluation_mode field\nspecifies REQUIRE_ATTESTATION, otherwise it must be empty."]
    pub fn require_attestations_by(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.require_attestations_by", self.base))
    }
}

#[derive(Serialize)]
pub struct BinaryAuthorizationPolicyDefaultAdmissionRuleEl {
    enforcement_mode: terrars::PrimField<std::string::String>,
    evaluation_mode: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    require_attestations_by: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl BinaryAuthorizationPolicyDefaultAdmissionRuleEl {
    #[doc =
        "Set the field `require_attestations_by`.\nThe resource names of the attestors that must attest to a\ncontainer image. If the attestor is in a different project from the\npolicy, it should be specified in the format 'projects/*/attestors/*'.\nEach attestor must exist before a policy can reference it. To add an\nattestor to a policy the principal issuing the policy change\nrequest must be able to read the attestor resource.\n\nNote: this field must be non-empty when the evaluation_mode field\nspecifies REQUIRE_ATTESTATION, otherwise it must be empty."]
    pub fn set_require_attestations_by(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.require_attestations_by = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BinaryAuthorizationPolicyDefaultAdmissionRuleEl {
    type O = terrars::BlockAssignable<BinaryAuthorizationPolicyDefaultAdmissionRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBinaryAuthorizationPolicyDefaultAdmissionRuleEl {
    #[doc =
        "The action when a pod creation is denied by the admission rule. Possible values: [\"ENFORCED_BLOCK_AND_AUDIT_LOG\", \"DRYRUN_AUDIT_LOG_ONLY\"]"]
    pub enforcement_mode: terrars::PrimField<std::string::String>,
    #[doc =
        "How this admission rule will be evaluated. Possible values: [\"ALWAYS_ALLOW\", \"REQUIRE_ATTESTATION\", \"ALWAYS_DENY\"]"]
    pub evaluation_mode: terrars::PrimField<std::string::String>,
}

impl BuildBinaryAuthorizationPolicyDefaultAdmissionRuleEl {
    pub fn build(self) -> BinaryAuthorizationPolicyDefaultAdmissionRuleEl {
        BinaryAuthorizationPolicyDefaultAdmissionRuleEl {
            enforcement_mode: self.enforcement_mode,
            evaluation_mode: self.evaluation_mode,
            require_attestations_by: core::default::Default::default(),
        }
    }
}

pub struct BinaryAuthorizationPolicyDefaultAdmissionRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BinaryAuthorizationPolicyDefaultAdmissionRuleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BinaryAuthorizationPolicyDefaultAdmissionRuleElRef {
        BinaryAuthorizationPolicyDefaultAdmissionRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BinaryAuthorizationPolicyDefaultAdmissionRuleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enforcement_mode` after provisioning.\nThe action when a pod creation is denied by the admission rule. Possible values: [\"ENFORCED_BLOCK_AND_AUDIT_LOG\", \"DRYRUN_AUDIT_LOG_ONLY\"]"]
    pub fn enforcement_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enforcement_mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `evaluation_mode` after provisioning.\nHow this admission rule will be evaluated. Possible values: [\"ALWAYS_ALLOW\", \"REQUIRE_ATTESTATION\", \"ALWAYS_DENY\"]"]
    pub fn evaluation_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.evaluation_mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `require_attestations_by` after provisioning.\nThe resource names of the attestors that must attest to a\ncontainer image. If the attestor is in a different project from the\npolicy, it should be specified in the format 'projects/*/attestors/*'.\nEach attestor must exist before a policy can reference it. To add an\nattestor to a policy the principal issuing the policy change\nrequest must be able to read the attestor resource.\n\nNote: this field must be non-empty when the evaluation_mode field\nspecifies REQUIRE_ATTESTATION, otherwise it must be empty."]
    pub fn require_attestations_by(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.require_attestations_by", self.base))
    }
}

#[derive(Serialize)]
pub struct BinaryAuthorizationPolicyTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BinaryAuthorizationPolicyTimeoutsEl {
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

impl terrars::ToListMappable for BinaryAuthorizationPolicyTimeoutsEl {
    type O = terrars::BlockAssignable<BinaryAuthorizationPolicyTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBinaryAuthorizationPolicyTimeoutsEl {}

impl BuildBinaryAuthorizationPolicyTimeoutsEl {
    pub fn build(self) -> BinaryAuthorizationPolicyTimeoutsEl {
        BinaryAuthorizationPolicyTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct BinaryAuthorizationPolicyTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BinaryAuthorizationPolicyTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BinaryAuthorizationPolicyTimeoutsElRef {
        BinaryAuthorizationPolicyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BinaryAuthorizationPolicyTimeoutsElRef {
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
struct BinaryAuthorizationPolicyDynamic {
    admission_whitelist_patterns: std
    ::option
    ::Option<terrars::DynamicBlock<BinaryAuthorizationPolicyAdmissionWhitelistPatternsEl>>,
    cluster_admission_rules: std
    ::option
    ::Option<terrars::DynamicBlock<BinaryAuthorizationPolicyClusterAdmissionRulesEl>>,
    default_admission_rule: std
    ::option
    ::Option<terrars::DynamicBlock<BinaryAuthorizationPolicyDefaultAdmissionRuleEl>>,
}
