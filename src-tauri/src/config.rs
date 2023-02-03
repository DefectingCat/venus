pub struct VConfig<'a> {
    core_folder_name: &'a str,
}

impl<'a> VConfig<'a> {
    pub fn new() -> Self {
        Self {
            core_folder_name: "v2ray-core",
        }
    }
}
