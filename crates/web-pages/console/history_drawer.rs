#![allow(non_snake_case)]
use daisy_rsx::*;
use db::queries::conversations::History;
use dioxus::prelude::*;
use rust_i18n::t;

#[component]
pub fn HistoryDrawer(trigger_id: String, team_id: i32, history: Vec<History>) -> Element {
    rsx! {
        Drawer {
            label: &t!("recent-chats"),
            trigger_id: &trigger_id,
            DrawerBody {
                for history in history {
                    li {
                        class: "w-full overflow-hidden truncate",
                        a {
                            href: crate::routes::console::Conversation{team_id, conversation_id: history.id}.to_string(),
                            {history.summary.clone()}
                        }
                    }
                }
            }
            DrawerFooter {
            }
        }
    }
}
