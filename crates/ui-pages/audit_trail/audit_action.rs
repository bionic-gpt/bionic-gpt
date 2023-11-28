#![allow(non_snake_case)]
use daisy_rsx::*;
use db::AuditAction;
use dioxus::prelude::*;

#[inline_props]
pub fn AuditAction(cx: Scope, audit_action: AuditAction) -> Element {
    match audit_action {
        AuditAction::CreateMember => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_role: LabelRole::Neutral,
                "Create Member"
            }
        )),
        AuditAction::CreateInvite => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_role: LabelRole::Neutral,
                "Create Invite"
            }
        )),
        AuditAction::DeleteMember => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_role: LabelRole::Neutral,
                "Delete Member"
            }
        )),
        AuditAction::CreateTeam => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_role: LabelRole::Neutral,
                "Create Team"
            }
        )),
        AuditAction::DeleteTeam => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_role: LabelRole::Neutral,
                "Delete Team"
            }
        )),
        AuditAction::CreateAPIKey => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_role: LabelRole::Neutral,
                "Create API Key"
            }
        )),
        AuditAction::DeleteAPIKey => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_role: LabelRole::Neutral,
                "Delete API Key"
            }
        )),
        AuditAction::CreatePipelineKey => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_role: LabelRole::Neutral,
                "Create Pipeline"
            }
        )),
        AuditAction::DeletePipelineKey => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_role: LabelRole::Neutral,
                "Delete Pipeline"
            }
        )),
        AuditAction::TextGeneration => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_role: LabelRole::Neutral,
                "Text Generation"
            }
        )),
    }
}
