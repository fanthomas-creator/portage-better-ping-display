fn cache(tab: &mut crate::tab_list::TabList) {
    if crate::tab_list::portage_better_ping_display::changed() {
        tab.cache = None;
    }
}
