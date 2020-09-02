use seed::{prelude::*, *};
use std::collections::BTreeMap;
use ulid::Ulid;

#[derive(Default)]
struct Model {
    list_items: BTreeMap<Ulid, ListItem>,
    new_item_text: String
}

struct ListItem {
    id: Ulid,
    title: String
}

enum Msg {
    CreateListItem(String),
    RemoveListItem(Ulid),
    ClearList,
}

fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    // use Msg::*;

    match msg {
        Msg::CreateListItem(title) => {
            let title = model.new_item_text.trim();
            if not(title.is_empty()) {
                let id = Ulid::new();
                model.list_items.insert(id, ListItem {
                    id,
                    title: title.to_owned(),
                });
                model.new_item_text.clear();
            }
        }
        Msg::RemoveListItem(id) => {
            model.list_items.remove(&id);
        }
        Msg::ClearList => {
            model.list_items.clear();
        }
    }
}

fn view_header() -> Node<Msg> {
    header![
        C!["header"],
        img![
            attrs!{At::Src => "https://cameras.liveviewtech.com/img/LVLogo_small.png"}
        ]
    ]
}

fn view_list(list_items: &BTreeMap<Ulid, ListItem>) -> Node<Msg> {
    ul![
        C!["list"],
        list_items.values().map(|item| {
            let id = item.id;
            li![
                div![
                    C!["view"],
                    label![&item.title],
                    button![
                        C!["destroy"],
                        "X",
                        ev(Ev::Click, move |_| Msg::RemoveListItem(id))    
                    ],
                ]
            ]
        })
    ]
}

fn view_controls(new_title: &str) -> Vec<Node<Msg>> {
    vec![
        input![
            C!["new"],
            attrs!{At::Placeholder => "Enter some text..."},
            input_ev(Ev::Input, Msg::CreateListItem)
        ],
        div![
            C!["controls"],
            button![
                C!["save"],
                "Save",
                input_ev(Ev::Click, |new_title| Msg::CreateListItem(new_title))
            ],
            button![
                C!["clear"],
                "Clear",
                ev(Ev::Click, |_| Msg::ClearList),
            ],
        ]
    ]
}

fn view(model: &Model) -> Vec<Node<Msg>> {
    nodes![
        view_header(),
        section![
            C!["main"],
            view_controls(&model.new_item_text),
            IF!(not(model.list_items.is_empty()) => vec![
                view_list(&model.list_items),
            ]),
        ]
    ]
}

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model::default()
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
