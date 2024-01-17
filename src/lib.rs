/// Expand the tilde (`~`) from within the provided path.
pub fn expand_tilde(path: impl AsRef<std::path::Path>) -> Option<std::path::PathBuf> {
    let p = path.as_ref();

    let expanded = if p.starts_with("~") {
        let mut base = simple_home_dir::home_dir()?;

        if !p.ends_with("~") {
            base.extend(p.components().skip(1));
        }
        base
    } else {
        p.to_path_buf()
    };
    expanded.exists().then_some(expanded)
}
