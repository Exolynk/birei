use birei::{Card, Label, MapCoordinate, MapViewer};
use leptos::prelude::*;
use crate::code_example::CodeExample;

#[component]
pub fn MapPage() -> impl IntoView {
    let selected = RwSignal::new(Some(MapCoordinate::new(47.3769, 8.5417)));
    let readonly = RwSignal::new(Some(MapCoordinate::new(48.2082, 16.3738)));
    let empty = RwSignal::new(None::<MapCoordinate>);

    view! {
        <section class="page-header">
            <div class="page-header__eyebrow">"Component"</div>
            <h2>"Map Viewer"</h2>
            <p class="page-header__lede">
                "Interactive OpenStreetMap viewer with draggable marker, drag panning, and wheel or button zoom."
            </p>
        </section>

        <section class="doc-grid">
            <Card header="Controlled marker selection" class="doc-card">
                <span class="doc-card__kicker">"Basics"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <div class="field">
                        <Label text="Delivery location" for_id="book-map-viewer"/>
                        <MapViewer
                            id="book-map-viewer"
                            value=selected
                            name="delivery_location"
                            on_value_change=Callback::new(move |next| selected.set(next))
                        />
                    </div>
                    <p class="doc-card__copy">
                        "Selected position: "
                        <strong>
                            {move || {
                                selected.get().map(|position| {
                                    format!("{:.5}, {:.5}", position.lat, position.lng)
                                }).unwrap_or_else(|| String::from("None"))
                            }}
                        </strong>
                    </p>
                </div>
                <CodeExample code={r#"<MapViewer
    value=selected
    on_value_change=Callback::new(move |next| selected.set(next))
/>"#}/>
            </Card>

            <Card header="Empty start state" class="doc-card">
                <span class="doc-card__kicker">"Optional marker"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <MapViewer
                        center=MapCoordinate::new(46.9480, 7.4474)
                        zoom=12
                        value=empty
                        on_value_change=Callback::new(move |next| empty.set(next))
                    />
                    <p class="doc-card__copy">
                        "No marker is shown until a position is provided."
                    </p>
                </div>
                <CodeExample code={r#"<MapViewer
    center=MapCoordinate::new(46.9480, 7.4474)
    value=None::<MapCoordinate>
/>"#}/>
            </Card>

            <Card header="Readonly viewport" class="doc-card">
                <span class="doc-card__kicker">"State"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <MapViewer value=readonly readonly=true zoom=11/>
                    <p class="doc-card__copy">
                        "Readonly blocks marker changes but still allows pan and zoom so the location can be inspected."
                    </p>
                </div>
                <CodeExample code={r#"<MapViewer value=marker readonly=true/>"#}/>
            </Card>
        </section>
    }
}
