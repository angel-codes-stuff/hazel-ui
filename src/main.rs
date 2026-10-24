use slint::{ModelRc, VecModel};

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = MainWindow::new()?;

    let games: Vec<GameManifest> = std::iter::repeat(GameManifest {
        id: 1,
        title: "Settings".into(),
        version: "1.0".into(),
        install_path: "/home/user/.hazel/games/settings-1".into(),
        executable: "settings".into(),
        arguments: ModelRc::new(VecModel::from(vec!["".into()])),
        installed: true,
    })
    .take(50)
    .collect();

    let model = ModelRc::new(VecModel::from(games));

    ui.set_games(model);

    ui.run()
}
