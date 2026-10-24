use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ContainerAnalysisNoteData {
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
    expiration_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    long_description: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    related_note_names: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    short_description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attestation_authority: std::option::Option<std::vec::Vec<ContainerAnalysisNoteAttestationAuthorityEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    related_url: std::option::Option<std::vec::Vec<ContainerAnalysisNoteRelatedUrlEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ContainerAnalysisNoteTimeoutsEl>,
    dynamic: ContainerAnalysisNoteDynamic,
}

struct ContainerAnalysisNote_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ContainerAnalysisNoteData>,
}

#[derive(Clone)]
pub struct ContainerAnalysisNote(std::rc::Rc<ContainerAnalysisNote_>);

impl ContainerAnalysisNote {
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

    #[doc = "Set the field `expiration_time`.\nTime of expiration for this note. Leave empty if note does not expire."]
    pub fn set_expiration_time(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().expiration_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `long_description`.\nA detailed description of the note"]
    pub fn set_long_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().long_description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `related_note_names`.\nNames of other notes related to this note."]
    pub fn set_related_note_names(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().related_note_names = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `short_description`.\nA one sentence description of the note."]
    pub fn set_short_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().short_description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `attestation_authority`.\n"]
    pub fn set_attestation_authority(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerAnalysisNoteAttestationAuthorityEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().attestation_authority = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.attestation_authority = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `related_url`.\n"]
    pub fn set_related_url(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerAnalysisNoteRelatedUrlEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().related_url = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.related_url = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ContainerAnalysisNoteTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time this note was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `expiration_time` after provisioning.\nTime of expiration for this note. Leave empty if note does not expire."]
    pub fn expiration_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expiration_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kind` after provisioning.\nThe type of analysis this note describes"]
    pub fn kind(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kind", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `long_description` after provisioning.\nA detailed description of the note"]
    pub fn long_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.long_description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the note."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `related_note_names` after provisioning.\nNames of other notes related to this note."]
    pub fn related_note_names(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.related_note_names", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `short_description` after provisioning.\nA one sentence description of the note."]
    pub fn short_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.short_description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time this note was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `attestation_authority` after provisioning.\n"]
    pub fn attestation_authority(&self) -> terrars::ListRef<ContainerAnalysisNoteAttestationAuthorityElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attestation_authority", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ContainerAnalysisNoteTimeoutsElRef {
        ContainerAnalysisNoteTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ContainerAnalysisNote {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ContainerAnalysisNote { }

impl terrars::ToListMappable for ContainerAnalysisNote {
    type O = terrars::ListRef<ContainerAnalysisNoteRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ContainerAnalysisNote_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_container_analysis_note".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildContainerAnalysisNote {
    pub tf_id: std::string::String,
    #[doc = "The name of the note."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildContainerAnalysisNote {
    pub fn build(self, stack: &mut terrars::Stack) -> ContainerAnalysisNote {
        let out = ContainerAnalysisNote(std::rc::Rc::new(ContainerAnalysisNote_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ContainerAnalysisNoteData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                expiration_time: core::default::Default::default(),
                id: core::default::Default::default(),
                long_description: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                related_note_names: core::default::Default::default(),
                short_description: core::default::Default::default(),
                attestation_authority: core::default::Default::default(),
                related_url: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ContainerAnalysisNoteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAnalysisNoteRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ContainerAnalysisNoteRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time this note was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `expiration_time` after provisioning.\nTime of expiration for this note. Leave empty if note does not expire."]
    pub fn expiration_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expiration_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kind` after provisioning.\nThe type of analysis this note describes"]
    pub fn kind(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kind", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `long_description` after provisioning.\nA detailed description of the note"]
    pub fn long_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.long_description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the note."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `related_note_names` after provisioning.\nNames of other notes related to this note."]
    pub fn related_note_names(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.related_note_names", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `short_description` after provisioning.\nA one sentence description of the note."]
    pub fn short_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.short_description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time this note was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `attestation_authority` after provisioning.\n"]
    pub fn attestation_authority(&self) -> terrars::ListRef<ContainerAnalysisNoteAttestationAuthorityElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attestation_authority", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ContainerAnalysisNoteTimeoutsElRef {
        ContainerAnalysisNoteTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ContainerAnalysisNoteAttestationAuthorityElHintEl {
    human_readable_name: terrars::PrimField<std::string::String>,
}

impl ContainerAnalysisNoteAttestationAuthorityElHintEl { }

impl terrars::ToListMappable for ContainerAnalysisNoteAttestationAuthorityElHintEl {
    type O = terrars::BlockAssignable<ContainerAnalysisNoteAttestationAuthorityElHintEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAnalysisNoteAttestationAuthorityElHintEl {
    #[doc = "The human readable name of this Attestation Authority, for\nexample \"qa\"."]
    pub human_readable_name: terrars::PrimField<std::string::String>,
}

impl BuildContainerAnalysisNoteAttestationAuthorityElHintEl {
    pub fn build(self) -> ContainerAnalysisNoteAttestationAuthorityElHintEl {
        ContainerAnalysisNoteAttestationAuthorityElHintEl { human_readable_name: self.human_readable_name }
    }
}

pub struct ContainerAnalysisNoteAttestationAuthorityElHintElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAnalysisNoteAttestationAuthorityElHintElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerAnalysisNoteAttestationAuthorityElHintElRef {
        ContainerAnalysisNoteAttestationAuthorityElHintElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAnalysisNoteAttestationAuthorityElHintElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `human_readable_name` after provisioning.\nThe human readable name of this Attestation Authority, for\nexample \"qa\"."]
    pub fn human_readable_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.human_readable_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct ContainerAnalysisNoteAttestationAuthorityElDynamic {
    hint: std::option::Option<terrars::DynamicBlock<ContainerAnalysisNoteAttestationAuthorityElHintEl>>,
}

#[derive(Serialize)]
pub struct ContainerAnalysisNoteAttestationAuthorityEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hint: std::option::Option<std::vec::Vec<ContainerAnalysisNoteAttestationAuthorityElHintEl>>,
    dynamic: ContainerAnalysisNoteAttestationAuthorityElDynamic,
}

impl ContainerAnalysisNoteAttestationAuthorityEl {
    #[doc = "Set the field `hint`.\n"]
    pub fn set_hint(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ContainerAnalysisNoteAttestationAuthorityElHintEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hint = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hint = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ContainerAnalysisNoteAttestationAuthorityEl {
    type O = terrars::BlockAssignable<ContainerAnalysisNoteAttestationAuthorityEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAnalysisNoteAttestationAuthorityEl {}

impl BuildContainerAnalysisNoteAttestationAuthorityEl {
    pub fn build(self) -> ContainerAnalysisNoteAttestationAuthorityEl {
        ContainerAnalysisNoteAttestationAuthorityEl {
            hint: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ContainerAnalysisNoteAttestationAuthorityElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAnalysisNoteAttestationAuthorityElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ContainerAnalysisNoteAttestationAuthorityElRef {
        ContainerAnalysisNoteAttestationAuthorityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAnalysisNoteAttestationAuthorityElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hint` after provisioning.\n"]
    pub fn hint(&self) -> terrars::ListRef<ContainerAnalysisNoteAttestationAuthorityElHintElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hint", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerAnalysisNoteRelatedUrlEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    label: std::option::Option<terrars::PrimField<std::string::String>>,
    url: terrars::PrimField<std::string::String>,
}

impl ContainerAnalysisNoteRelatedUrlEl {
    #[doc = "Set the field `label`.\nLabel to describe usage of the URL"]
    pub fn set_label(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.label = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ContainerAnalysisNoteRelatedUrlEl {
    type O = terrars::BlockAssignable<ContainerAnalysisNoteRelatedUrlEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAnalysisNoteRelatedUrlEl {
    #[doc = "Specific URL associated with the resource."]
    pub url: terrars::PrimField<std::string::String>,
}

impl BuildContainerAnalysisNoteRelatedUrlEl {
    pub fn build(self) -> ContainerAnalysisNoteRelatedUrlEl {
        ContainerAnalysisNoteRelatedUrlEl {
            label: core::default::Default::default(),
            url: self.url,
        }
    }
}

pub struct ContainerAnalysisNoteRelatedUrlElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAnalysisNoteRelatedUrlElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerAnalysisNoteRelatedUrlElRef {
        ContainerAnalysisNoteRelatedUrlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAnalysisNoteRelatedUrlElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `label` after provisioning.\nLabel to describe usage of the URL"]
    pub fn label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label", self.base))
    }

    #[doc =
        "Get a reference to the value of field `url` after provisioning.\nSpecific URL associated with the resource."]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

#[derive(Serialize)]
pub struct ContainerAnalysisNoteTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ContainerAnalysisNoteTimeoutsEl {
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

impl terrars::ToListMappable for ContainerAnalysisNoteTimeoutsEl {
    type O = terrars::BlockAssignable<ContainerAnalysisNoteTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildContainerAnalysisNoteTimeoutsEl {}

impl BuildContainerAnalysisNoteTimeoutsEl {
    pub fn build(self) -> ContainerAnalysisNoteTimeoutsEl {
        ContainerAnalysisNoteTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct ContainerAnalysisNoteTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ContainerAnalysisNoteTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ContainerAnalysisNoteTimeoutsElRef {
        ContainerAnalysisNoteTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ContainerAnalysisNoteTimeoutsElRef {
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
struct ContainerAnalysisNoteDynamic {
    attestation_authority: std::option::Option<terrars::DynamicBlock<ContainerAnalysisNoteAttestationAuthorityEl>>,
    related_url: std::option::Option<terrars::DynamicBlock<ContainerAnalysisNoteRelatedUrlEl>>,
}
