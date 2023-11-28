pub mod filter;
pub mod index;
pub mod table;

use db::{AuditAccessType, AuditAction};

const AUDIT_ACCESS: [AuditAccessType; 2] = [AuditAccessType::UserInterface, AuditAccessType::API];

const AUDIT_ACTION: [AuditAction; 10] = [
    AuditAction::CreateMember,
    AuditAction::CreateInvite,
    AuditAction::DeleteMember,
    AuditAction::CreateTeam,
    AuditAction::DeleteTeam,
    AuditAction::CreateAPIKey,
    AuditAction::DeleteAPIKey,
    AuditAction::CreatePipelineKey,
    AuditAction::DeletePipelineKey,
    AuditAction::TextGeneration,
];

pub fn position_to_access_type(num: usize) -> AuditAccessType {
    AUDIT_ACCESS[num]
}

pub fn position_to_audit_action(num: usize) -> AuditAction {
    AUDIT_ACTION[num]
}

pub fn access_type_to_string(access_type: AuditAccessType) -> String {
    match access_type {
        AuditAccessType::API => "API".to_owned(),
        AuditAccessType::UserInterface => "User Interface".to_owned(),
    }
}

pub fn audit_action_to_string(audit_action: AuditAction) -> String {
    match audit_action {
        AuditAction::CreateMember => "Create Member".to_owned(),
        AuditAction::CreateInvite => "Create Invite".to_owned(),
        AuditAction::DeleteMember => "Delete Member".to_owned(),
        AuditAction::CreateTeam => "Create Team".to_owned(),
        AuditAction::DeleteTeam => "Delete Team".to_owned(),
        AuditAction::CreateAPIKey => "Create API Key".to_owned(),
        AuditAction::DeleteAPIKey => "Delete API Key".to_owned(),
        AuditAction::CreatePipelineKey => "Create Pipeline Key".to_owned(),
        AuditAction::DeletePipelineKey => "Delete Pipeline Key".to_owned(),
        AuditAction::TextGeneration => "Text Generation".to_owned(),
    }
}
