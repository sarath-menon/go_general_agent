// mod sidecar;
// use sidecar::SidecarManager;

use objc2_app_kit::{NSColor, NSWindow};
use objc2::ffi::nil;
use objc2::runtime::AnyObject;

#[cfg(target_os = "macos")]
use objc2_app_kit::{NSWorkspace, NSBitmapImageRep};
#[cfg(target_os = "macos")]
#[cfg(target_os = "macos")]
use objc2::{msg_send, ClassType};
#[cfg(target_os = "macos")]
use std::ffi::CStr;
#[cfg(target_os = "macos")]
use base64::engine::general_purpose;
#[cfg(target_os = "macos")]
use base64::Engine;

use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{Manager, TitleBarStyle, WebviewUrl, WebviewWindowBuilder};

#[cfg(desktop)]
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[cfg(target_os = "macos")]
#[derive(serde::Serialize)]
struct AppInfo {
    name: String,
    icon_png_base64: String,
}

#[cfg(target_os = "macos")]
#[tauri::command]
async fn list_apps_with_icons() -> Result<Vec<AppInfo>, String> {
    unsafe {
        let workspace = NSWorkspace::sharedWorkspace();
        let apps = workspace.runningApplications();
        let mut result = Vec::with_capacity(apps.len());

        for app in apps.iter() {
            
            // Check if app is visible (not background-only)
            let is_hidden: bool = msg_send![&*app, isHidden];
            let activation_policy: i64 = msg_send![&*app, activationPolicy];
            
            // Only include regular GUI apps (activation policy 0 = NSApplicationActivationPolicyRegular)
            // Skip accessory apps (1) like browser plugins and prohibited apps (2) like background processes
            if is_hidden || activation_policy != 0 {
                continue;
            }

            // Get app name
            let name_ns: *mut AnyObject = msg_send![&*app, localizedName];
            if name_ns == nil {
                continue;
            }
            let utf8_ptr: *const std::os::raw::c_char = msg_send![name_ns, UTF8String];
            let cname = CStr::from_ptr(utf8_ptr)
                .to_string_lossy()
                .into_owned();

            // Skip empty names
            if cname.is_empty() {
                continue;
            }

            // Get app icon
            let icon: *mut AnyObject = msg_send![&*app, icon];
            if icon == nil {
                // Skip apps without icons
                continue;
            }

            // Convert icon to PNG data
            let tiff_data: *mut AnyObject = msg_send![icon, TIFFRepresentation];
            if tiff_data == nil {
                continue;
            }

            // Create bitmap representation from TIFF data
            let bitmap_rep: *mut AnyObject = msg_send![NSBitmapImageRep::class(), alloc];
            let bitmap_rep: *mut AnyObject = msg_send![bitmap_rep, initWithData: tiff_data];
            if bitmap_rep == nil {
                continue;
            }

            // Convert to PNG data (NSBitmapImageFileTypePNG = 4)
            let png_data: *mut AnyObject = msg_send![bitmap_rep, representationUsingType: 4u64, properties: nil];
            if png_data == nil {
                continue;
            }

            // Extract bytes and base64-encode
            let bytes: *const u8 = msg_send![png_data, bytes];
            let len: usize = msg_send![png_data, length];
            let slice = std::slice::from_raw_parts(bytes, len);
            let b64 = general_purpose::STANDARD.encode(slice);

            result.push(AppInfo {
                name: cname,
                icon_png_base64: b64,
            });
        }

        Ok(result)
    }
}

#[cfg(not(target_os = "macos"))]
#[derive(serde::Serialize)]
struct AppInfo {
    name: String,
    icon_png_base64: String,
}

#[cfg(not(target_os = "macos"))]
#[tauri::command]
async fn list_apps_with_icons() -> Result<Vec<AppInfo>, String> {
    // Return empty result on non-macOS platforms 
    Ok(vec![])
}

// #[tauri::command]
// async fn start_sidecar(
//     app: AppHandle,
//     sidecar_manager: State<'_, Arc<SidecarManager>>,
// ) -> Result<(), String> {
//     sidecar_manager.start_sidecar(&app).await
// }

// #[tauri::command]
// async fn stop_sidecar(
//     app: AppHandle,
//     sidecar_manager: State<'_, Arc<SidecarManager>>,
// ) -> Result<(), String> {
//     sidecar_manager.stop_sidecar(&app).await
// }

// #[tauri::command]
// fn sidecar_status(sidecar_manager: State<'_, Arc<SidecarManager>>) -> bool {
//     sidecar_manager.is_running()
// }

// #[tauri::command]
// async fn sidecar_health(sidecar_manager: State<'_, Arc<SidecarManager>>) -> Result<String, String> {
//     sidecar_manager.health_check().await
// }

// #[tauri::command]
// fn sidecar_error(sidecar_manager: State<'_, Arc<SidecarManager>>) -> Option<String> {
//     sidecar_manager.get_error()
// }

// #[tauri::command]
// async fn send_prompt(
//     prompt: String,
//     sidecar_manager: State<'_, Arc<SidecarManager>>,
// ) -> Result<String, String> {
//     sidecar_manager.send_prompt(Option 1 is already implemented, but it does not work. Let's try option 2. But how to ensure that the list of apps can be fetched again after initialization? &prompt).await
// }

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // let sidecar_manager = Arc::new(SidecarManager::new());

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_macos_permissions::init())
        // .manage(sidecar_manager.clone())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            list_apps_with_icons,
            // start_sidecar,
            // stop_sidecar,
            // sidecar_status,
            // sidecar_health,
            // sidecar_error,
            // send_prompt
        ])
        .setup(move |app| {
            // Create the main window programmatically
            let win_builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
                .title("")
                .inner_size(500.0, 600.0)
                .max_inner_size(500.0, 700.0)
                .min_inner_size(500.0, 600.0);

            // set transparent title bar only when building for macOS
            #[cfg(target_os = "macos")]
            let win_builder = win_builder.title_bar_style(TitleBarStyle::Transparent);

            let window = win_builder.build().unwrap();

            // set background color only when building for macOS
            #[cfg(target_os = "macos")]
            {
                let ns_window = window.ns_window().unwrap();
                unsafe {
                    let bg_color = NSColor::colorWithRed_green_blue_alpha(23.0/ 255.0, 23.0/ 255.0, 23.0/ 255.0, 1.0);
                    let ns_window_ref = &*(ns_window as *const NSWindow);
                    ns_window_ref.setBackgroundColor(Some(&bg_color));
                }
            }

            let _app_handle = app.handle().clone();
            // let manager = sidecar_manager.clone();

            // Clone for auto-start
            // let startup_manager = manager.clone();

            // Auto-start sidecar on app launch
            // tauri::async_runtime::spawn(async move {
            //     if let Err(e) = startup_manager.start_sidecar(&startup_handle).await {
            //         eprintln!("Failed to auto-start sidecar: {}", e);
            //     }
            // });

            // Set up cleanup handler for app shutdown
            // let cleanup_manager = manager.clone();
            // let cleanup_handle = app_handle.clone();
            // app.listen("tauri://close-requested", move |_| {
            //     let manager = cleanup_manager.clone();
            //     let handle = cleanup_handle.clone();
            //     tauri::async_runtime::spawn(async move {
            //         if let Err(e) = manager.stop_sidecar(&handle).await {
            //             eprintln!("Failed to stop sidecar during cleanup: {}", e);
            //         }
            //     });
            // });

            // Create system tray
            let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let show_item = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
            let hide_item = MenuItem::with_id(app, "hide", "Hide", true, None::<&str>)?;
            // let sidecar_status_item =
            //     MenuItem::with_id(app, "sidecar_status", "Sidecar Status", true, None::<&str>)?;

            let tray_menu = Menu::with_items(
                app,
                &[&show_item, &hide_item, &quit_item],
            )?;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&tray_menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        println!("Quit menu item clicked");
                        app.exit(0);
                    }
                    "show" => {
                        println!("Show menu item clicked");
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "hide" => {
                        println!("Hide menu item clicked");
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.hide();
                        }
                    }
                    _ => {
                        println!("Unhandled menu item: {:?}", event.id);
                    }
                })
                .on_tray_icon_event(|tray, event| match event {
                    TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } => {
                        println!("Left click on tray icon");
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            if window.is_visible().unwrap_or(false) {
                                let _ = window.hide();
                            } else {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    }
                    TrayIconEvent::DoubleClick {
                        button: MouseButton::Left,
                        ..
                    } => {
                        println!("Double click on tray icon");
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    _ => {
                        println!("Unhandled tray event: {:?}", event);
                    }
                })
                .build(app)?;

            // Register global shortcut for window toggle
            #[cfg(desktop)]
            {
                // Use Cmd+Shift+T on macOS, Ctrl+Shift+T on Windows/Linux
                #[cfg(target_os = "macos")]
                let toggle_shortcut = Shortcut::new(Some(Modifiers::SUPER | Modifiers::SHIFT), Code::KeyT);
                
                #[cfg(not(target_os = "macos"))]
                let toggle_shortcut = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyT);

                app.handle().plugin(
                    tauri_plugin_global_shortcut::Builder::new().with_handler(move |_app, shortcut, event| {
                        if shortcut == &toggle_shortcut {
                            match event.state() {
                                ShortcutState::Pressed => {
                                    println!("Global shortcut pressed - toggling window visibility");
                                    if let Some(window) = _app.get_webview_window("main") {
                                        if window.is_visible().unwrap_or(false) {
                                            let _ = window.hide();
                                        } else {
                                            let _ = window.show();
                                            let _ = window.set_focus();
                                        }
                                    }
                                }
                                ShortcutState::Released => {
                                    // Handle release if needed
                                }
                            }
                        }
                    })
                    .build(),
                )?;

                app.global_shortcut().register(toggle_shortcut)?;
                println!("Global shortcut registered: Cmd+Shift+T (macOS) / Ctrl+Shift+T (Windows/Linux)");
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
