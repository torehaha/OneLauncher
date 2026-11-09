fn main() {
    #[cfg(windows)]
    {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("icons/icon.ico");

        let version = env!("CARGO_PKG_VERSION");
        res.set("CompanyName", "Pulsar");
        res.set("ProductName", "Pulsar");
        res.set("FileDescription", "Pulsar");
        res.set("LegalCopyright", "© 2026 Polyfrost Inc.");
        res.set("OriginalFilename", "oneclient_app.exe");
        res.set("InternalName", "oneclient_app");
        res.set("FileVersion", version);
        res.set("ProductVersion", version);

        res.compile()
            .expect("failed to embed Windows icon resource");
    }
}
