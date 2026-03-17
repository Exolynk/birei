use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes, A};
use leptos_router::path;

use crate::pages::{ButtonPage, FontPage, IconPage, InputPage, LabelPage, SelectPage, TagPage};

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
                            <A href="/select" exact=true attr:class="book-nav__link">
                                "Select"
                            </A>
                            <A href="/tag" exact=true attr:class="book-nav__link">
                                "Tag"
                            </A>
                        </div>
                    </nav>
                </aside>

                <main class="book-content">
                    <Routes fallback=|| view! { <ButtonPage/> }>
                        <Route path=path!("") view=ButtonPage/>
                        <Route path=path!("/button") view=ButtonPage/>
                        <Route path=path!("/font") view=FontPage/>
                        <Route path=path!("/icon") view=IconPage/>
                        <Route path=path!("/input") view=InputPage/>
                        <Route path=path!("/label") view=LabelPage/>
                        <Route path=path!("/select") view=SelectPage/>
                        <Route path=path!("/tag") view=TagPage/>
                    </Routes>
                </main>
            </div>
        </Router>
    }
}
