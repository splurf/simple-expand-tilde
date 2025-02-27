use std::path::*;

/// Expand the tilde (`~`) from within the provided path.
pub fn expand_tilde(path: impl AsRef<Path>) -> Option<PathBuf> {
    let p = path.as_ref();

    let expanded = if p.starts_with("~") {
        #[allow(deprecated)]
        let mut base = std::env::home_dir()?;

        if !p.ends_with("~") {
            base.extend(p.components().skip(1));
        }
        base
    } else {
        p.to_path_buf()
    };
    Some(expanded)
}
