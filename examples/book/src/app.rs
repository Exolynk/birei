use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes, A};
use leptos_router::path;

use crate::pages::{
    ButtonPage, CardPage, CheckboxPage, ColorPage, DateTimePage, FontPage, IconPage, InputPage,
    LabelPage, ListPage, MenuButtonPage, SelectPage, SliderPage, TabsPage, TagPage, TextareaPage,
    TooltipPage,
};

const BOOK_CSS: &str = include_str!("book.css");

#[component]
pub fn App() -> impl IntoView {
    view! {
        <style>{BOOK_CSS}</style>
        <Router>
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
                            <A href="/button" exact=true attr:class="book-nav__link">
                                "Button"
                            </A>
                            <A href="/card" exact=true attr:class="book-nav__link">
                                "Card"
                            </A>
                            <A href="/checkbox" exact=true attr:class="book-nav__link">
                                "Checkbox"
                            </A>
                            <A href="/color" exact=true attr:class="book-nav__link">
                                "Color Input"
                            </A>
                            <A href="/datetime" exact=true attr:class="book-nav__link">
                                "DateTime"
                            </A>
                            <A href="/menu-button" exact=true attr:class="book-nav__link">
                                "Menu Button"
                            </A>
                            <A href="/font" exact=true attr:class="book-nav__link">
                                "Font"
                            </A>
                            <A href="/icon" exact=true attr:class="book-nav__link">
                                "Icon"
                            </A>
                            <A href="/input" exact=true attr:class="book-nav__link">
                                "Input"
                            </A>
                            <A href="/label" exact=true attr:class="book-nav__link">
                                "Label"
                            </A>
                            <A href="/list" exact=true attr:class="book-nav__link">
                                "List"
                            </A>
                            <A href="/select" exact=true attr:class="book-nav__link">
                                "Select"
                            </A>
                            <A href="/slider" exact=true attr:class="book-nav__link">
                                "Slider"
                            </A>
                            <A href="/tag" exact=true attr:class="book-nav__link">
                                "Tag"
                            </A>
                            <A href="/tabs" exact=true attr:class="book-nav__link">
                                "Tabs"
                            </A>
                            <A href="/tooltip" exact=true attr:class="book-nav__link">
                                "Tooltip"
                            </A>
                            <A href="/textarea" exact=true attr:class="book-nav__link">
                                "Textarea"
                            </A>
                        </div>
                    </nav>
                </aside>

                <main class="book-content">
                    <Routes fallback=|| view! { <ButtonPage/> }>
                        <Route path=path!("") view=ButtonPage/>
                        <Route path=path!("/button") view=ButtonPage/>
                        <Route path=path!("/card") view=CardPage/>
                        <Route path=path!("/checkbox") view=CheckboxPage/>
                        <Route path=path!("/color") view=ColorPage/>
                        <Route path=path!("/datetime") view=DateTimePage/>
                        <Route path=path!("/menu-button") view=MenuButtonPage/>
                        <Route path=path!("/font") view=FontPage/>
                        <Route path=path!("/icon") view=IconPage/>
                        <Route path=path!("/input") view=InputPage/>
                        <Route path=path!("/label") view=LabelPage/>
                        <Route path=path!("/list") view=ListPage/>
                        <Route path=path!("/select") view=SelectPage/>
                        <Route path=path!("/slider") view=SliderPage/>
                        <Route path=path!("/tag") view=TagPage/>
                        <Route path=path!("/tabs") view=TabsPage/>
                        <Route path=path!("/tooltip") view=TooltipPage/>
                        <Route path=path!("/textarea") view=TextareaPage/>
                    </Routes>
                </main>
            </div>
        </Router>
    }
}
