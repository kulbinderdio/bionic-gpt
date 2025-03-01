#![allow(non_snake_case)]
use crate::{
    app_layout::{Layout, SideBar},
    hero::Hero,
    i18n_helper::translate,
};
use assets::files::*;
use daisy_rsx::*;
use db::{authz::Rbac, History};
use dioxus::prelude::*;

pub fn page(rbac: Rbac, team_id: i32, history: Vec<History>) -> String {
    let buckets = super::bucket_history(history);
    let page = rsx! {
        Layout {
            section_class: "p-4",
            selected_item: SideBar::History,
            team_id: team_id,
            rbac: rbac,
            title: "{translate(\"history-title\")}",
            header: rsx! {
                h3 { "{translate(\"history-title\")}" }
                Button {
                    prefix_image_src: "{button_plus_svg.name}",
                    drawer_trigger: "search-history",
                    button_scheme: ButtonScheme::Primary,
                    "{translate(\"history-search\")}"
                }
            },
            super::form::Form {
                team_id: team_id
            }
            if buckets.1 == 0 {
                BlankSlate {
                    heading: "{translate(\"history-empty\")}",
                    visual: nav_history_svg.name,
                    description: "{translate(\"history-empty-summary\")}"
                }
            } else {

                Hero {
                    heading: translate("history-title").to_string(),
                    subheading: "Easily reference past conversations to recall information,
                        follow up on topics, or continue where you left off.".to_string()
                }

                super::history_table::HistoryTable {
                    team_id,
                    buckets: buckets.0
                }
            }
        }
    };

    crate::render(page)
}
