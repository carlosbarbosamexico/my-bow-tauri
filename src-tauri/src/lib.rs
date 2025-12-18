use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem, Submenu},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Listener, Manager, RunEvent, WindowEvent,
};

mod navigation;

/// Allowed hosts for navigation (main app + auth providers)
const ALLOWED_HOSTS: &[&str] = &[
    "app.bowsapp.com",
    "bowsapp.com",
    "accounts.google.com",
    "appleid.apple.com",
    "github.com",
    "login.microsoftonline.com",
    "auth0.com",
];

/// Check if a URL is allowed for navigation
pub fn is_allowed_url(url: &str) -> bool {
    if let Ok(parsed) = url::Url::parse(url) {
        if let Some(host) = parsed.host_str() {
            return ALLOWED_HOSTS.iter().any(|allowed| {
                host == *allowed || host.ends_with(&format!(".{}", allowed))
            });
        }
    }
    // Allow local resources
    url.starts_with("tauri://") || url.starts_with("asset://")
}

/// Build the native application menu
fn build_menu(app: &tauri::AppHandle) -> tauri::Result<Menu<tauri::Wry>> {
    let menu = Menu::new(app)?;

    // App menu (macOS)
    #[cfg(target_os = "macos")]
    {
        let app_menu = Submenu::new(app, "Bow", true)?;
        app_menu.append(&MenuItem::with_id(app, "about", "About Bow", true, None::<&str>)?)?;
        app_menu.append(&PredefinedMenuItem::separator(app)?)?;
        app_menu.append(&PredefinedMenuItem::services(app, Some("Services"))?)?;
        app_menu.append(&PredefinedMenuItem::separator(app)?)?;
        app_menu.append(&PredefinedMenuItem::hide(app, Some("Hide Bow"))?)?;
        app_menu.append(&PredefinedMenuItem::hide_others(app, Some("Hide Others"))?)?;
        app_menu.append(&PredefinedMenuItem::show_all(app, Some("Show All"))?)?;
        app_menu.append(&PredefinedMenuItem::separator(app)?)?;
        app_menu.append(&MenuItem::with_id(app, "quit", "Quit Bow", true, Some("CmdOrCtrl+Q"))?)?;
        menu.append(&app_menu)?;
    }

    // File menu
    let file_menu = Submenu::new(app, "File", true)?;
    file_menu.append(&MenuItem::with_id(app, "reload", "Reload", true, Some("CmdOrCtrl+R"))?)?;
    file_menu.append(&MenuItem::with_id(app, "hard_reload", "Hard Reload", true, Some("CmdOrCtrl+Shift+R"))?)?;
    file_menu.append(&PredefinedMenuItem::separator(app)?)?;
    #[cfg(not(target_os = "macos"))]
    {
        file_menu.append(&MenuItem::with_id(app, "quit", "Quit", true, Some("Alt+F4"))?)?;
    }
    menu.append(&file_menu)?;

    // Edit menu
    let edit_menu = Submenu::new(app, "Edit", true)?;
    edit_menu.append(&PredefinedMenuItem::undo(app, Some("Undo"))?)?;
    edit_menu.append(&PredefinedMenuItem::redo(app, Some("Redo"))?)?;
    edit_menu.append(&PredefinedMenuItem::separator(app)?)?;
    edit_menu.append(&PredefinedMenuItem::cut(app, Some("Cut"))?)?;
    edit_menu.append(&PredefinedMenuItem::copy(app, Some("Copy"))?)?;
    edit_menu.append(&PredefinedMenuItem::paste(app, Some("Paste"))?)?;
    edit_menu.append(&PredefinedMenuItem::select_all(app, Some("Select All"))?)?;
    menu.append(&edit_menu)?;

    // View menu
    let view_menu = Submenu::new(app, "View", true)?;
    view_menu.append(&MenuItem::with_id(app, "zoom_in", "Zoom In", true, Some("CmdOrCtrl+Plus"))?)?;
    view_menu.append(&MenuItem::with_id(app, "zoom_out", "Zoom Out", true, Some("CmdOrCtrl+Minus"))?)?;
    view_menu.append(&MenuItem::with_id(app, "zoom_reset", "Actual Size", true, Some("CmdOrCtrl+0"))?)?;
    view_menu.append(&PredefinedMenuItem::separator(app)?)?;
    view_menu.append(&PredefinedMenuItem::fullscreen(app, Some("Toggle Full Screen"))?)?;
    menu.append(&view_menu)?;

    // Window menu
    let window_menu = Submenu::new(app, "Window", true)?;
    window_menu.append(&PredefinedMenuItem::minimize(app, Some("Minimize"))?)?;
    #[cfg(target_os = "macos")]
    {
        window_menu.append(&PredefinedMenuItem::maximize(app, Some("Zoom"))?)?;
        window_menu.append(&PredefinedMenuItem::separator(app)?)?;
        window_menu.append(&PredefinedMenuItem::close_window(app, Some("Close"))?)?;
    }
    #[cfg(not(target_os = "macos"))]
    {
        window_menu.append(&PredefinedMenuItem::maximize(app, Some("Maximize"))?)?;
        window_menu.append(&PredefinedMenuItem::close_window(app, Some("Close"))?)?;
    }
    menu.append(&window_menu)?;

    // Help menu
    let help_menu = Submenu::new(app, "Help", true)?;
    help_menu.append(&MenuItem::with_id(app, "website", "Visit Website", true, None::<&str>)?)?;
    help_menu.append(&MenuItem::with_id(app, "check_updates", "Check for Updates...", true, None::<&str>)?)?;
    menu.append(&help_menu)?;

    Ok(menu)
}

/// Handle menu events
fn handle_menu_event(app: &tauri::AppHandle, event: tauri::menu::MenuEvent) {
    match event.id().as_ref() {
        "quit" => {
            app.exit(0);
        }
        "reload" => {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.eval("window.location.reload()");
            }
        }
        "hard_reload" => {
            if let Some(window) = app.get_webview_window("main") {
                // Clear cache and reload
                let _ = window.eval("window.location.reload(true)");
            }
        }
        "zoom_in" => {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.eval("document.body.style.zoom = (parseFloat(document.body.style.zoom || 1) + 0.1).toString()");
            }
        }
        "zoom_out" => {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.eval("document.body.style.zoom = Math.max(0.5, parseFloat(document.body.style.zoom || 1) - 0.1).toString()");
            }
        }
        "zoom_reset" => {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.eval("document.body.style.zoom = '1'");
            }
        }
        "website" => {
            let _ = tauri::async_runtime::spawn(async {
                let _ = open::that("https://bowsapp.com");
            });
        }
        "check_updates" => {
            log::info!("Check for updates requested");
            // Updater check would be triggered here
        }
        "about" => {
            log::info!("About Bow - Version {}", env!("CARGO_PKG_VERSION"));
            // About dialog is handled via native menu on macOS
        }
        _ => {}
    }
}

/// Setup system tray
fn setup_tray(app: &tauri::AppHandle) -> tauri::Result<()> {
    let show_item = MenuItem::with_id(app, "show", "Show Window", true, None::<&str>)?;
    let reload_item = MenuItem::with_id(app, "tray_reload", "Reload", true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "tray_quit", "Quit", true, None::<&str>)?;

    let tray_menu = Menu::new(app)?;
    tray_menu.append(&show_item)?;
    tray_menu.append(&reload_item)?;
    tray_menu.append(&PredefinedMenuItem::separator(app)?)?;
    tray_menu.append(&quit_item)?;

    let _tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&tray_menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id().as_ref() {
            "show" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "tray_reload" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.eval("window.location.reload()");
                }
            }
            "tray_quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })
        .build(app)?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();

    let mut builder = tauri::Builder::default();

    // Single instance plugin (desktop only)
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        builder = builder.plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            // Focus the main window when another instance tries to launch
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }));
    }

    builder
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            // Build and set the menu
            let menu = build_menu(app.handle())?;
            app.set_menu(menu)?;

            // Setup tray icon (disabled for now - uncomment when icons are properly configured)
            // setup_tray(app.handle())?;

            // Handle deep links
            #[cfg(any(target_os = "macos", target_os = "linux"))]
            {
                use tauri_plugin_deep_link::DeepLinkExt;
                if let Err(e) = app.deep_link().register("bows") {
                    log::error!("Failed to register deep link: {}", e);
                }
            }

            // Listen for deep link events
            let handle = app.handle().clone();
            app.listen("deep-link://new-url", move |event| {
                log::info!("Deep link received: {:?}", event.payload());
                if let Some(window) = handle.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                    // Handle the deep link URL - navigate to the appropriate route
                    if let Some(url) = event.payload().strip_prefix("bows://") {
                        let target = format!("https://app.bowsapp.com/{}", url);
                        let _ = window.eval(&format!("window.location.href = '{}'", target));
                    }
                }
            });

            Ok(())
        })
        .on_menu_event(handle_menu_event)
        .on_window_event(|window, event| {
            // Handle window close - hide to tray instead of quitting
            if let WindowEvent::CloseRequested { api, .. } = event {
                #[cfg(target_os = "macos")]
                {
                    let _ = window.hide();
                    api.prevent_close();
                }
            }
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app, event| {
            if let RunEvent::ExitRequested { api, .. } = event {
                // Keep app running in background on macOS
                #[cfg(target_os = "macos")]
                {
                    api.prevent_exit();
                }
                let _ = app;
            }
        });
}

