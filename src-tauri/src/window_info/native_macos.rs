use crate::model::{ActiveWindow, MacOSError};
use cocoa::base::{id, nil};
use cocoa::foundation::NSAutoreleasePool;
use objc::{class, msg_send, sel, sel_impl};
use std::ffi::CStr;

pub fn get_native_window_info() -> Result<ActiveWindow, MacOSError> {
    unsafe {
        let pool = NSAutoreleasePool::new(nil);

        // Get the shared workspace
        let shared_workspace: id = msg_send![class!(NSWorkspace), sharedWorkspace];

        // Get the frontmost application
        let frontmost_app: id = msg_send![shared_workspace, frontmostApplication];

        if frontmost_app == nil {
            let _: () = msg_send![pool, drain];
            return Err(MacOSError::NoActiveWindow);
        }

        // Get application name
        let localized_name: id = msg_send![frontmost_app, localizedName];

        let app_name = if localized_name != nil {
            let c_str: *const std::os::raw::c_char = msg_send![localized_name, UTF8String];
            if !c_str.is_null() {
                CStr::from_ptr(c_str).to_string_lossy().to_string()
            } else {
                "Unknown Application".to_string()
            }
        } else {
            "Unknown Application".to_string()
        };

        let _: () = msg_send![pool, drain];

        Ok(ActiveWindow {
            title: app_name.clone(),
            class: app_name,
        })
    }
}

// Enhanced function to get detailed browser tab information
pub fn get_enhanced_browser_info() -> Result<ActiveWindow, MacOSError> {
    // First get the basic window info
    let basic_info = get_native_window_info()?;

    // If it's a browser, try to get more detailed tab information
    if is_browser(&basic_info.class) {
        if let Ok(enhanced_info) = get_browser_tab_via_applescript(&basic_info.class) {
            return Ok(ActiveWindow {
                class: basic_info.class,
                title: enhanced_info,
            });
        }
    }

    Ok(basic_info)
}

fn is_browser(app_name: &str) -> bool {
    let browsers = [
        "Safari",
        "Chrome",
        "Google Chrome",
        "Firefox",
        "Mozilla Firefox",
        "Brave",
        "Brave Browser",
        "Arc",
        "Edge",
        "Microsoft Edge",
    ];
    browsers.iter().any(|&browser| app_name.contains(browser))
}

fn get_browser_tab_via_applescript(app_name: &str) -> Result<String, MacOSError> {
    let script = if app_name.contains("Chrome") || app_name.contains("Google Chrome") {
        r#"tell application "Google Chrome"
            if (count of windows) > 0 then
                tell front window
                    set activeTab to active tab
                    return title of activeTab
                end tell
            end if
        end tell"#
    } else if app_name.contains("Safari") {
        r#"tell application "Safari"
            if (count of windows) > 0 then
                set currentTab to current tab of front window
                return name of currentTab
            end if
        end tell"#
    } else if app_name.contains("Brave") {
        r#"tell application "Brave Browser"
            if (count of windows) > 0 then
                tell front window
                    set activeTab to active tab
                    return title of activeTab
                end tell
            end if
        end tell"#
    } else if app_name.contains("Firefox") {
        // Firefox doesn't support AppleScript the same way, fall back to window title
        return Err(MacOSError::CoreGraphicsError(
            "Firefox doesn't support tab AppleScript".to_string(),
        ));
    } else {
        return Err(MacOSError::CoreGraphicsError("Unknown browser".to_string()));
    };

    let output = std::process::Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .map_err(|e| MacOSError::CoreGraphicsError(format!("AppleScript failed: {}", e)))?;

    if output.status.success() {
        let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !result.is_empty() {
            return Ok(result);
        }
    }

    Err(MacOSError::CoreGraphicsError(
        "No tab title found".to_string(),
    ))
}
