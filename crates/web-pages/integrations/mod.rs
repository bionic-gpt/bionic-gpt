pub mod delete_drawer;
pub mod details_modal;
pub mod index;
use db::Integration;
pub mod integration_cards;
pub mod integration_type;
pub mod status_type;
pub mod upsert;
pub mod view;

#[derive(Clone, PartialEq, Debug)]
pub struct IntegrationOas3 {
    pub integration: Integration,
    pub spec: oas3::Spec,
}
