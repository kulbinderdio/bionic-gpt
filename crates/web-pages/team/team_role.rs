#![allow(non_snake_case)]
use daisy_rsx::*;
use db::Role as DBRole;
use dioxus::prelude::*;
use crate::i18n_helper::translate;

#[component]
pub fn Role(role: DBRole) -> Element {
    match role {
        DBRole::SystemAdministrator => rsx!(
            Label {
                class: "mr-2",
                label_role: LabelRole::Highlight,
                "System Administrator"
            }
        ),
        DBRole::TeamManager => rsx!(
            Label {
                class: "mr-2",
                label_role: LabelRole::Neutral,
                "{translate(\"team-role-manager\")}"
            }
        ),
        DBRole::Collaborator => rsx!(
            Label {
                class: "mr-2",
                label_role: LabelRole::Neutral,
                "{translate(\"team-role-collaborator\")}"
            }
        ),
    }
}
