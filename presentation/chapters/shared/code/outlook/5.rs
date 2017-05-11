let button = document().query_selector("#hide-button").unwrap();
button.add_event_listener(move |_: ClickEvent| {
    for anchor in document().query_selector_all("#main a") {
        js!(@{anchor}.style = "display: none;";);
    }
});