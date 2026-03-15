use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes, A};
use leptos_router::path;

use crate::pages::{ButtonPage, IconPage, InputPage};

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
                            <A href="/icon" exact=true attr:class="book-nav__link">
                                "Icon"
                            </A>
                            <A href="/input" exact=true attr:class="book-nav__link">
                                "Input"
                            </A>
                        </div>
                    </nav>
                </aside>

                <main class="book-content">
                    <Routes fallback=|| view! { <ButtonPage/> }>
                        <Route path=path!("") view=ButtonPage/>
                        <Route path=path!("/button") view=ButtonPage/>
                        <Route path=path!("/icon") view=IconPage/>
                        <Route path=path!("/input") view=InputPage/>
                    </Routes>
                </main>
            </div>
        </Router>
    }
}
