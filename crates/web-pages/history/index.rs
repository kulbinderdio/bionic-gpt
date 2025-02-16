#![allow(non_snake_case)]
use crate::{
    app_layout::{Layout, SideBar},
    hero::Hero,
};
use assets::files::*;
use daisy_rsx::*;
use db::{authz::Rbac, History};
use dioxus::prelude::*;
use rust_i18n::t;

pub fn page(rbac: Rbac, team_id: i32, history: Vec<History>) -> String {
    let buckets = super::bucket_history(history);
    let page = rsx! {
        Layout {
            section_class: "p-4",
            selected_item: SideBar::History,
            team_id: team_id,
            rbac: rbac,
            title: &t!("history-title"),
            header: rsx! {
                h3 { {t!("history-title")} }
                Button {
                    prefix_image_src: "{button_plus_svg.name}",
                    drawer_trigger: "search-history",
                    button_scheme: ButtonScheme::Primary,
                    {t!("history-search")}
                }
            },
            super::form::Form {
                team_id: team_id
            }
            if buckets.1 == 0 {
                BlankSlate {
                    heading: &t!("history-empty-heading"),
                    visual: nav_history_svg.name,
                    description: &t!("history-empty-description")
                }
            } else {

                Hero {
                    heading: t!("history-title").to_string(),
                    subheading: t!("history-hero-subheading").to_string()
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
