use zed_extension_api::{self as zed, LanguageServerId, Result};

const LSP_BINARY_NAME: &str = "drupal-lsp";

struct DrupalExtension;

impl zed::Extension for DrupalExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let path = worktree.which(LSP_BINARY_NAME).ok_or_else(|| {
            format!(
                "`{LSP_BINARY_NAME}` not found in PATH. Install from \
                 https://github.com/abderrahimghazali/drupal-lsp \
                 (`cargo install --path .` from a clone)."
            )
        })?;
        Ok(zed::Command {
            command: path,
            args: vec![],
            env: Default::default(),
        })
    }
}

zed::register_extension!(DrupalExtension);
