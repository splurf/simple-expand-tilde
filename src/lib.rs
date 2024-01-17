/// Expand the tilde (`~`) from within the provided path.
pub fn expand_tilde(path: impl AsRef<std::path::Path>) -> Option<std::path::PathBuf> {
    let p = path.as_ref();

    Some(if p.starts_with("~") {
        let mut home = simple_home_dir::home_dir()?;

        if !p.ends_with("~") {
            let mut cmpts = p.components();
            cmpts.next()?;
            home.extend(cmpts);
        }
        home
    } else {
        p.to_path_buf()
    })
}
