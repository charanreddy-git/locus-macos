use std::sync::Arc;
use tauri::{Emitter, EventTarget};
use tokio::{
    sync::Mutex,
    time::{sleep, Duration},
};

use crate::model::{ActiveWindow, MacOSError};

// For now, let's use a simple approach with system commands
// In a production app, we'd use proper CoreGraphics bindings
use std::process::Command;

pub fn stream_macos(
    cancel_flag: Arc<Mutex<bool>>,
    app: tauri::AppHandle,
    sleep_duration: Duration,
) -> Result<(), MacOSError> {
    tokio::spawn(async move {
        let mut old_window_info = ActiveWindow::none();
        let mut consecutive_failures = 0;
        const MAX_FAILURES: u32 = 5;
        
        loop {
            if *cancel_flag.lock().await {
                break;
            }

            let window_info = get_macos_active_window_info()
                .unwrap_or_else(|error| {
                    consecutive_failures += 1;
                    if consecutive_failures <= MAX_FAILURES {
                        eprintln!("Error getting window info (attempt {}): {:?}", consecutive_failures, error);
                    }
                    
                    // If we keep failing, try to get basic app info as fallback
                    get_basic_app_info().unwrap_or_else(|_| ActiveWindow::none())
                });

            // Reset failure counter on success
            if window_info.title != "none" {
                consecutive_failures = 0;
            }

            // Only emit if the window info has actually changed
            if window_info != old_window_info {
                old_window_info = window_info.clone();
                
                // Stream the window info to the frontend
                if let Err(e) = app.emit_to(EventTarget::app(), "active-window-title", window_info) {
                    eprintln!("Error emitting window info: {:?}", e);
                }
            }
            
            sleep(sleep_duration).await;
        }
    });

    Ok(())
}

fn get_macos_active_window_info() -> Result<ActiveWindow, MacOSError> {
    // Try multiple approaches in order of reliability
    
    // 1. Try native CoreGraphics approach first (most reliable)
    if let Ok(native_info) = crate::window_info::native_macos::get_native_window_info() {
        if !native_info.title.is_empty() && native_info.title != "Unknown Application" {
            return Ok(native_info);
        }
    }

    // 2. Try the enhanced AppleScript approach
    if let Ok(enhanced_info) = get_enhanced_window_info() {
        if !enhanced_info.title.is_empty() && enhanced_info.title != "No Window" && !enhanced_info.title.contains("Error|") && enhanced_info.title != "Active" {
            return Ok(enhanced_info);
        }
    }

    // 3. Try browser-specific detection for browsers
    if let Ok(browser_info) = get_browser_tab_info() {
        if !browser_info.title.is_empty() && browser_info.title != "No Window" && !browser_info.title.contains("Error|") {
            return Ok(browser_info);
        }
    }

    // 4. Fallback to general window detection
    get_general_window_info()
}

fn get_browser_tab_info() -> Result<ActiveWindow, MacOSError> {
    // Enhanced AppleScript that handles browser tabs specifically
    let script = r#"
tell application "System Events"
    set frontApp to first application process whose frontmost is true
    set appName to name of frontApp
end tell

-- Handle different browsers specifically
try
    if appName contains "Safari" then
        tell application "Safari"
            if (count of windows) > 0 then
                set currentTab to current tab of front window
                set tabTitle to name of currentTab
                return "Safari|" & tabTitle
            end if
        end tell
    else if (appName contains "Chrome") or (appName contains "Google Chrome") then
        tell application "Google Chrome"
            if (count of windows) > 0 then
                tell front window
                    set activeTab to active tab
                    set tabTitle to title of activeTab
                    return "Chrome|" & tabTitle
                end tell
            end if
        end tell
    else if appName contains "Firefox" or appName contains "Mozilla Firefox" then
        -- Firefox uses a different approach through System Events
        tell application "System Events"
            tell process "Firefox"
                if (count of windows) > 0 then
                    set windowTitle to name of front window
                    -- Clean up Firefox title format
                    if windowTitle contains " - Mozilla Firefox" then
                        set windowTitle to text 1 thru ((offset of " - Mozilla Firefox" in windowTitle) - 1) of windowTitle
                    end if
                    return "Firefox|" & windowTitle
                end if
            end tell
        end tell
    else if appName contains "Brave" or appName contains "Brave Browser" then
        -- Brave browser handling
        try
            tell application "Brave Browser"
                if (count of windows) > 0 then
                    tell front window
                        set activeTab to active tab
                        set tabTitle to title of activeTab
                        return "Brave|" & tabTitle
                    end tell
                end if
            end tell
        on error
            -- Fallback to System Events for Brave
            tell application "System Events"
                tell process "Brave Browser"
                    if (count of windows) > 0 then
                        set windowTitle to name of front window
                        if windowTitle contains " - Brave" then
                            set windowTitle to text 1 thru ((offset of " - Brave" in windowTitle) - 1) of windowTitle
                        end if
                        return "Brave|" & windowTitle
                    end if
                end tell
            end tell
        end try
    else if appName contains "Arc" then
        tell application "Arc"
            if (count of windows) > 0 then
                tell front window
                    set activeTab to active tab
                    set tabTitle to title of activeTab
                    return "Arc|" & tabTitle
                end tell
            end if
        end tell
    else if (appName contains "Edge") or (appName contains "Microsoft Edge") then
        tell application "Microsoft Edge"
            if (count of windows) > 0 then
                tell front window
                    set activeTab to active tab
                    set tabTitle to title of activeTab
                    return "Edge|" & tabTitle
                end tell
            end if
        end tell
    end if
on error errMsg
    -- If browser-specific detection fails, fall back to System Events
    try
        tell application "System Events"
            tell frontApp
                if (count of windows) > 0 then
                    set windowTitle to name of front window
                    return appName & "|" & windowTitle
                end if
            end tell
        end tell
    on error
        return appName & "|" & appName
    end try
end try

return "Unknown|No Browser Tab"
"#;

    let output = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .map_err(|e| MacOSError::CoreGraphicsError(format!("Failed to run browser AppleScript: {}", e)))?;

    if output.status.success() {
        let result = String::from_utf8_lossy(&output.stdout);
        let parts: Vec<&str> = result.trim().split('|').collect();
        
        if parts.len() >= 2 && parts[0] != "Unknown" {
            return Ok(ActiveWindow {
                class: parts[0].to_string(),
                title: parts[1].to_string(),
            });
        }
    }

    Err(MacOSError::NoActiveWindow)
}

fn get_enhanced_window_info() -> Result<ActiveWindow, MacOSError> {
    // This approach focuses on getting the most accurate window title possible
    let script = r#"
tell application "System Events"
    set frontApp to first application process whose frontmost is true
    set appName to name of frontApp
    
    -- Try multiple approaches to get window information
    set windowTitle to ""
    
    try
        -- Method 1: Try to get the frontmost window title directly
        set frontWindow to front window of frontApp
        set windowTitle to name of frontWindow
    on error
        try
            -- Method 2: Try AXTitle attribute which sometimes works when name doesn't
            set frontWindow to front window of frontApp
            set windowTitle to value of attribute "AXTitle" of frontWindow
        on error
            try
                -- Method 3: Try to get document name for some apps
                set windowTitle to name of document 1 of frontApp
            on error
                -- Method 4: For browsers, try to get the current tab title
                if appName contains "Chrome" or appName contains "Google Chrome" then
                    try
                        tell application "Google Chrome"
                            set windowTitle to title of active tab of front window
                        end tell
                    end try
                else if appName contains "Safari" then
                    try
                        tell application "Safari"
                            set windowTitle to name of current tab of front window
                        end tell
                    end try
                else if appName contains "Firefox" then
                    try
                        tell application "Firefox"
                            set windowTitle to name of front window
                        end tell
                    end try
                end if
            end try
        end try
    end try
    
    -- Clean up window title if we got something
    if windowTitle is not "" then
        -- Clean up and enhance the window title based on the application
        if appName contains "Chrome" or appName contains "Google Chrome" then
            if windowTitle contains " - Google Chrome" then
                set windowTitle to text 1 thru ((offset of " - Google Chrome" in windowTitle) - 1) of windowTitle
            end if
            return "Chrome|" & windowTitle
            
        else if appName contains "Safari" then
            if windowTitle contains " — " then
                set windowTitle to text 1 thru ((offset of " — " in windowTitle) - 1) of windowTitle
            end if
            return "Safari|" & windowTitle
            
        else if appName contains "Firefox" then
            if windowTitle contains " - Mozilla Firefox" then
                set windowTitle to text 1 thru ((offset of " - Mozilla Firefox" in windowTitle) - 1) of windowTitle
            end if
            return "Firefox|" & windowTitle
            
        else if appName contains "Code" or appName contains "VS Code" or appName contains "Visual Studio Code" then
            return "VS Code|" & windowTitle
            
        else if appName contains "Terminal" then
            return "Terminal|" & windowTitle
            
        else if appName contains "Finder" then
            return "Finder|" & windowTitle
            
        else
            return appName & "|" & windowTitle
        end if
    else
        -- If we still couldn't get a window title, use the app name as title
        return appName & "|" & appName
    end if
end tell
"#;

    let output = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .map_err(|e| MacOSError::CoreGraphicsError(format!("Failed to run enhanced AppleScript: {}", e)))?;

    if output.status.success() {
        let result = String::from_utf8_lossy(&output.stdout);
        let parts: Vec<&str> = result.trim().split('|').collect();
        
        if parts.len() >= 2 {
            return Ok(ActiveWindow {
                class: parts[0].to_string(),
                title: parts[1].to_string(),
            });
        }
    }

    Err(MacOSError::NoActiveWindow)
}

fn get_general_window_info() -> Result<ActiveWindow, MacOSError> {
    // Enhanced general window detection for non-browser apps
    let script = r#"
tell application "System Events"
    set frontApp to first application process whose frontmost is true
    set appName to name of frontApp
    
    -- Try to get detailed window information
    try
        set frontWindow to front window of frontApp
        set windowTitle to name of frontWindow
        
        -- For some apps, try to get more specific information
        if appName contains "Code" or appName contains "VS Code" or appName contains "Visual Studio Code" then
            -- VS Code window titles usually contain the file name and path
            return "VS Code|" & windowTitle
        else if appName contains "Terminal" then
            return "Terminal|" & windowTitle
        else if appName contains "Finder" then
            return "Finder|" & windowTitle
        else if appName contains "Xcode" then
            return "Xcode|" & windowTitle
        else if appName contains "Slack" then
            return "Slack|" & windowTitle
        else if appName contains "Discord" then
            return "Discord|" & windowTitle
        else if appName contains "Zoom" then
            return "Zoom|" & windowTitle
        else if appName contains "Teams" or appName contains "Microsoft Teams" then
            return "Teams|" & windowTitle
        else if appName contains "Notion" then
            return "Notion|" & windowTitle
        else if appName contains "Obsidian" then
            return "Obsidian|" & windowTitle
        else
            return appName & "|" & windowTitle
        end if
    on error
        -- Fallback if window name cannot be retrieved
        return appName & "|Active"
    end try
end tell
"#;

    let output = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .map_err(|e| MacOSError::CoreGraphicsError(format!("Failed to run general AppleScript: {}", e)))?;

    if !output.status.success() {
        return Err(MacOSError::CoreGraphicsError(
            "AppleScript execution failed".to_string(),
        ));
    }

    let result = String::from_utf8_lossy(&output.stdout);
    let parts: Vec<&str> = result.trim().split('|').collect();

    if parts.len() >= 2 {
        Ok(ActiveWindow {
            class: parts[0].to_string(),
            title: parts[1].to_string(),
        })
    } else {
        Ok(ActiveWindow::none())
    }
}

fn get_basic_app_info() -> Result<ActiveWindow, MacOSError> {
    // Simple fallback that just gets the frontmost application name
    let script = r#"
tell application "System Events"
    set frontApp to first application process whose frontmost is true
    return name of frontApp
end tell
"#;

    let output = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .map_err(|e| MacOSError::CoreGraphicsError(format!("Failed to run basic AppleScript: {}", e)))?;

    if output.status.success() {
        let app_name = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !app_name.is_empty() {
            return Ok(ActiveWindow {
                class: app_name.clone(),
                title: format!("{} - Active", app_name),
            });
        }
    }

    Err(MacOSError::NoActiveWindow)
}

