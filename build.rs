fn main() {
    glib_build_tools::compile_resources(
        &["data/resources"],
        "data/resources/resources.gresource.xml",
        "hiit.gresource",
    );

    relm4_icons_build::bundle_icons(
        "icon_names.rs",
        Some("xyz.safeworlds.hiit"),
        None::<&str>,
        None::<&str>,
        [
            "edit",
            "pause",
            "play",
            "arrow-circular-top-right",
            "weight2",
        ],
    );
}
