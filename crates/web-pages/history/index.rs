#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use assets::files::*;
use daisy_rsx::*;
use db::{authz::Rbac, History};
use dioxus::prelude::*;

#[component]
pub fn Page(rbac: Rbac, team_id: i32, history: Vec<History>) -> Element {
    let buckets = super::bucket_history(history);
    rsx! {
        Layout {
            section_class: "p-4",
            selected_item: SideBar::History,
            team_id: team_id,
            rbac: rbac,
            title: "Chat History",
            header: rsx! {
                h3 { "Chat History" }
                Button {
                    prefix_image_src: "{button_plus_svg.name}",
                    drawer_trigger: "search-history",
                    button_scheme: ButtonScheme::Primary,
                    "Search History"
                }
            },
            super::form::Form {
                team_id: team_id
            }
            if buckets.1 == 0 {
                BlankSlate {
                    heading: "Looks like you haven't had any conversations yet",
                    visual: nav_history_svg.name,
                    description: "When you do a summary will appear on this page"
                }
            } else {
                super::history_table::HistoryTable {
                    team_id,
                    buckets: buckets.0
                }
            }
        }
    }
}
