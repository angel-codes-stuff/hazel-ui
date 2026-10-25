use std::env;

use slint::{ModelRc, VecModel};

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let args: Vec<String> = env::args().collect();

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

    if args.contains(&"--fullscreen".into()) {
        let ui = ConsoleModeWindow::new()?;
        ui.set_games(model);
        return ui.run();
        // } else {
        //     let ui = MainWindow::new()?;
        //     ui.set_games(model);
        //     ui.run()
        // }
    }

    let ui = MainWindow::new()?;
    let w_ui = ui.as_weak();
    ui.set_games(model.clone());

    ui.on_show_fullscreen(move || {
        let ui = ConsoleModeWindow::new().unwrap();
        ui.set_games(model.clone());
        ui.show().unwrap();

        if let Some(main_win) = w_ui.upgrade() {
            main_win.hide().unwrap();
        }
    });

    ui.run()
}
