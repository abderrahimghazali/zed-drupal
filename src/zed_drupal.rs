use zed_extension_api as zed;

struct DrupalExtension;

impl zed::Extension for DrupalExtension {
    fn new() -> Self {
        Self
    }
}

zed::register_extension!(DrupalExtension);
