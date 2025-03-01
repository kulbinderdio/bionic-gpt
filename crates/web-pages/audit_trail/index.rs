#![allow(non_snake_case)]
use daisy_rsx::*;
use db::{authz::Rbac, AuditTrail, Member};
use dioxus::prelude::*;
use crate::i18n_helper::translate;

use crate::{
    app_layout::{Layout, SideBar},
    render,
};

pub fn page(
    team_users: Vec<Member>,
    audits: Vec<AuditTrail>,
    team_id: i32,
    rbac: Rbac,
    reset_search: bool,
) -> String {
    let page = rsx! {

        Layout {
            section_class: "p-4",
            selected_item: SideBar::AuditTrail,
            team_id: team_id,
            rbac: rbac,
            title: "{translate(\"audit-trail-title\")}",
            header: rsx! {
                h3 { "{translate(\"audit-trail-title\")}" }
                Button {
                    drawer_trigger: super::filter::DRAW_TRIGGER,
                    button_scheme: ButtonScheme::Default,
                    "{translate(\"common-filter\")}"
                }
            },
            super::table::AuditTable {
                audits: audits
            }
            super::filter::FilterDrawer {
                team_users: team_users.clone(),
                reset_search: reset_search,
                submit_action: crate::routes::audit_trail::Index {team_id}
            }
        }
    };

    render(page)
}
