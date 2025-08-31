use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}


#[component]
fn Home() -> Element {
    rsx! {
        Hero {}
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div { id: "hero",
            div { id: "title", "Веган ресурсы Алматы" }
            div { id: "links",
                Resource {
                    src: "https://veganrussian.ru/",
                    title: "VeganRussian.ru",
                    description: "Каталог веганских продуктов",
                }
                Resource {
                    src: "https://www.instagram.com/omirfest/",
                    title: "OmirFest",
                    platform: SocialMedia::Instagram,
                }
                Resource {
                    src: "https://www.instagram.com/omirconf/",
                    title: "OmirConf",
                    platform: SocialMedia::Instagram,
                }
                Resource {
                    src: "https://t.me/veganDvizhAlmaty",
                    title: "Веган Движ Алматы",
                    description: "Канал с веган мероприятиями в Алматы",
                    platform: SocialMedia::Telegram,
                }
                Resource {
                    src: "https://t.me/veganstan",
                    title: "Веганстан",
                    description: "Группа для веганов и стремящихся к веганству в Казахстане",
                    platform: SocialMedia::Telegram,
                }
                Resource {

                    src: "",
                    title: "Vegan🌱Almaty",
                    description: "Группа веганов Алматы (вход по приглашению участника)",
                    platform: SocialMedia::Telegram,
                }
            }
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum SocialMedia {
    Instagram,
    Telegram,
}

fn get_icon_for_platform(platform: SocialMedia) -> Element {
    match platform {
    SocialMedia::Instagram =>    
        rsx!{
            div { class: "icon-holder",
                img {
                    src: "https://img.icons8.com/?size=100&id=RhYNENh5cxlS&format=png&color=000000",
                    // src: "https://img.icons8.com/ios/100/instagram-new--v1.png",
                    alt: "instagram",
                    width: "30px",
                }
            }
        },
    SocialMedia::Telegram =>
        rsx!{
            div { class: "icon-holder",
                img {
                    src: "https://img.icons8.com/?size=100&id=lUktdBVdL4Kb&format=png&color=000000",
                    // src: "https://img.icons8.com/?size=100&id=A4uyUh8LBg90&format=png&color=000000",
                    alt: "instagram",
                    width: "28px",
                }
            }
        }
    }
}

#[component]
pub fn Resource( title: String, src: String, description: Option<String>, platform: Option<SocialMedia>) -> Element {
    rsx!{
        Link { to: src.clone(), new_tab: true,
            onclick: move |_| {
            },
            onclick_only: src == "",
            
            div { class: "resource-title",
                if let Some(p) = platform {
                    {get_icon_for_platform(p)}
                }
                "{title}"
            }
            if let Some(d) = description {
                div { class: "description", "{d}" }
            }
        }
    }
}
