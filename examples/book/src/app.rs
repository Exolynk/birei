use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes, A};
use leptos_router::path;
use std::borrow::Cow;

use crate::pages::{
    ButtonBarPage, ButtonMenuPage, ButtonPage, CardPage, ChartPage, CheckboxPage, CodeEditorPage,
    ColorPage, DateTimePage, FlexibleColumnsPage, FontPage, IconPage, InputPage, LabelPage,
    ListPage, MapPage, MarkdownPage, SelectPage, SliderPage, TablePage, TabsPage, TagPage,
    TextareaPage, TooltipPage,
};

const BOOK_CSS: &str = include_str!("book.css");

fn router_base() -> Cow<'static, str> {
    let pathname = web_sys::window()
        .and_then(|window| window.location().pathname().ok())
        .unwrap_or_default();

    if pathname == "/birei" || pathname.starts_with("/birei/") {
        Cow::Borrowed("/birei")
    } else {
        Cow::Borrowed("")
    }
}

#[component]
pub fn App() -> impl IntoView {
    let base = router_base();

    view! {
        <style>{BOOK_CSS}</style>
        <Router base=base>
            <div class="book-shell">
                <aside class="book-sidebar">
                    <div class="book-sidebar__brand">
                        <div class="book-sidebar__eyebrow">"birei"</div>
                        <h1>"Component book"</h1>
                        <p>
                            "A runnable Leptos CSR app that documents the component library."
                        </p>
                    </div>

                    <nav class="book-nav" aria-label="Components">
                        <div class="book-nav__group">
                            <div class="book-nav__label">"Components"</div>
                            <A href="button" exact=true attr:class="book-nav__link">
                                "Button"
                            </A>
                            <A href="button-bar" exact=true attr:class="book-nav__link">
                                "Button Bar"
                            </A>
                            <A href="button-menu" exact=true attr:class="book-nav__link">
                                "Button Menu"
                            </A>
                            <A href="card" exact=true attr:class="book-nav__link">
                                "Card"
                            </A>
                            <A href="checkbox" exact=true attr:class="book-nav__link">
                                "Checkbox"
                            </A>
                            <A href="charts" exact=true attr:class="book-nav__link">
                                "Charts"
                            </A>
                            <A href="color" exact=true attr:class="book-nav__link">
                                "Color Input"
                            </A>
                            <A href="code-editor" exact=true attr:class="book-nav__link">
                                "Code Editor"
                            </A>
                            <A href="datetime" exact=true attr:class="book-nav__link">
                                "DateTime"
                            </A>
                            <A href="font" exact=true attr:class="book-nav__link">
                                "Font"
                            </A>
                            <A href="flexible-columns" exact=true attr:class="book-nav__link">
                                "Flexible Columns"
                            </A>
                            <A href="icon" exact=true attr:class="book-nav__link">
                                "Icon"
                            </A>
                            <A href="input" exact=true attr:class="book-nav__link">
                                "Input"
                            </A>
                            <A href="label" exact=true attr:class="book-nav__link">
                                "Label"
                            </A>
                            <A href="list" exact=true attr:class="book-nav__link">
                                "List"
                            </A>
                            <A href="markdown" exact=true attr:class="book-nav__link">
                                "Markdown Editor"
                            </A>
                            <A href="map" exact=true attr:class="book-nav__link">
                                "Map Viewer"
                            </A>
                            <A href="select" exact=true attr:class="book-nav__link">
                                "Select"
                            </A>
                            <A href="slider" exact=true attr:class="book-nav__link">
                                "Slider"
                            </A>
                            <A href="table" exact=true attr:class="book-nav__link">
                                "Table"
                            </A>
                            <A href="tag" exact=true attr:class="book-nav__link">
                                "Tag"
                            </A>
                            <A href="tabs" exact=true attr:class="book-nav__link">
                                "Tabs"
                            </A>
                            <A href="tooltip" exact=true attr:class="book-nav__link">
                                "Tooltip"
                            </A>
                            <A href="textarea" exact=true attr:class="book-nav__link">
                                "Textarea"
                            </A>
                        </div>
                    </nav>
                </aside>

                <main class="book-content">
                    <Routes fallback=|| view! { <ButtonPage/> }>
                        <Route path=path!("") view=ButtonPage/>
                        <Route path=path!("button") view=ButtonPage/>
                        <Route path=path!("button-bar") view=ButtonBarPage/>
                        <Route path=path!("card") view=CardPage/>
                        <Route path=path!("charts") view=ChartPage/>
                        <Route path=path!("checkbox") view=CheckboxPage/>
                        <Route path=path!("color") view=ColorPage/>
                        <Route path=path!("code-editor") view=CodeEditorPage/>
                        <Route path=path!("datetime") view=DateTimePage/>
                        <Route path=path!("button-menu") view=ButtonMenuPage/>
                        <Route path=path!("font") view=FontPage/>
                        <Route path=path!("flexible-columns") view=FlexibleColumnsPage/>
                        <Route path=path!("icon") view=IconPage/>
                        <Route path=path!("input") view=InputPage/>
                        <Route path=path!("label") view=LabelPage/>
                        <Route path=path!("list") view=ListPage/>
                        <Route path=path!("map") view=MapPage/>
                        <Route path=path!("markdown") view=MarkdownPage/>
                        <Route path=path!("select") view=SelectPage/>
                        <Route path=path!("slider") view=SliderPage/>
                        <Route path=path!("table") view=TablePage/>
                        <Route path=path!("tag") view=TagPage/>
                        <Route path=path!("tabs") view=TabsPage/>
                        <Route path=path!("tooltip") view=TooltipPage/>
                        <Route path=path!("textarea") view=TextareaPage/>
                    </Routes>
                </main>
            </div>
        </Router>
    }
}
