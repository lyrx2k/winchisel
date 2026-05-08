pub fn t(key: &str) -> &'static str {
    match key {
        "nav_title" => "Navigation",
        "nav_subtitle" => "Winchisel control center",
        "home" => "Home",
        "debloater" => "Debloater",
        "downloads" => "Apps & Downloads",
        "performance" => "Performance",
        "processes" => "Processes",
        "latency" => "Latency",
        "privacy_security" => "Privacy & Security",
        "privacy_security_title" => "Privacy & Security",
        "privacy_security_subtitle" => "Control privacy and security settings from a single place.",
        "privacy_security_search" => "Search privacy settings",
        "privacy_security_quick" => "Quick",
        "privacy_security_apply_recommended" => "Apply Recommended",
        "privacy_security_reset_defaults" => "Reset Defaults",
        "privacy_security_current_default" => "Default: ",
        "privacy_security_current_recommended" => "Recommended: ",
        "privacy_security_loading" => "Loading privacy settings...",
        "privacy_security_placeholder_title" => "Coming soon",
        "privacy_security_placeholder_desc" => {
            "This tab is reserved for future security and privacy controls."
        }
        "privacy_uac_title" => "User Account Control Level",
        "privacy_uac_desc" => "Controls UAC notification level and secure desktop behavior",
        "privacy_uac_opt_0" => "Prompt for Credentials",
        "privacy_uac_opt_1" => "Always notify",
        "privacy_uac_opt_2" => "Notify when apps try to make changes",
        "privacy_uac_opt_3" => "Notify when apps try to make changes (no dim)",
        "privacy_uac_opt_4" => "Never notify",
        "privacy_workplace_join_title" => "Workplace Join Message Prompts",
        "privacy_workplace_join_desc" => {
            "Show Allow my organization to manage my device' prompts throughout Windows"
        }
        "privacy_bitlocker_title" => "BitLocker Auto Encryption",
        "privacy_bitlocker_desc" => {
            "Controls whether Windows can automatically encrypt drives with BitLocker"
        }
        "privacy_wifi_sense_title" => "WiFi-Sense",
        "privacy_wifi_sense_desc" => {
            "Allow sharing WiFi passwords with contacts and connecting to suggested hotspots"
        }
        "privacy_automatic_maintenance_title" => "Automatic Maintenance",
        "privacy_automatic_maintenance_desc" => {
            "Choose if Windows should run automatic system maintenance during idle time"
        }
        "privacy_error_reporting_title" => "Windows Error Reporting",
        "privacy_error_reporting_desc" => {
            "Choose if Windows should collect and send crash reports and error information to Microsoft"
        }
        "privacy_remote_assistance_title" => "Remote Assistance",
        "privacy_remote_assistance_desc" => {
            "Choose if other people can connect to your computer remotely to provide technical support"
        }
        "privacy_smart_app_control_title" => "Smart App Control",
        "privacy_smart_app_control_desc" => {
            "Controls the Smart App Control feature which blocks untrusted applications"
        }
        "privacy_smart_app_control_opt_0" => "Off",
        "privacy_smart_app_control_opt_1" => "On (Enforced)",
        "privacy_smart_app_control_opt_2" => "Evaluation Mode",
        "privacy_powershell_title" => "PowerShell Execution Policy",
        "privacy_powershell_desc" => {
            "Controls whether PowerShell scripts are allowed to run and under what conditions"
        }
        "privacy_powershell_opt_0" => "Restricted",
        "privacy_powershell_opt_1" => "AllSigned",
        "privacy_powershell_opt_2" => "RemoteSigned",
        "privacy_powershell_opt_3" => "Unrestricted",
        "privacy_powershell_opt_4" => "Bypass",
        "privacy_developer_title" => "Developer Mode",
        "privacy_developer_desc" => {
            "Allows the installation of apps from any source, including loose files"
        }
        "privacy_lock_screen_title" => "Lock Screen",
        "privacy_lock_screen_desc" => {
            "Allows users to lock their computer using Windows+L, Start menu, or Ctrl+Alt+Del screen"
        }
        "privacy_rotating_lock_title" => "Windows Spotlight on Lock Screen",
        "privacy_rotating_lock_desc" => {
            "Displays rotating Windows Spotlight images on your lock screen instead of a static background. Winhance automatically sets the Start Menu Recommended Section to Show when this setting is enabled as it is required"
        }
        "privacy_lock_screen_overlay_title" => "Lock Screen Fun Facts and Tips",
        "privacy_lock_screen_overlay_desc" => {
            "Displays fun facts, tips, and tricks as an overlay on your lock screen"
        }
        "privacy_advertising_id_title" => {
            "Let apps show me personalized ads by using my advertising ID"
        }
        "privacy_advertising_id_desc" => {
            "Windows generates a unique advertising ID that apps use to track your activity and deliver personalized ads based on your behavior across different apps"
        }
        "privacy_language_list_title" => {
            "Let websites show me locally relevant content by accessing my language list"
        }
        "privacy_language_list_desc" => {
            "Allows websites to access your language preferences so they can automatically display content in your preferred language without requiring manual configuration on each site"
        }
        "privacy_app_launch_title" => {
            "Let Windows improve Start and search results by tracking app launches"
        }
        "privacy_app_launch_desc" => {
            "Windows records which apps you use most frequently to personalize your Start menu and improve search results, making your most-used apps more accessible"
        }
        "privacy_settings_content_title" => "Show me suggested content in the Settings app",
        "privacy_settings_content_desc" => {
            "Displays promotional content, tips, and feature suggestions within the Windows Settings app. Winhance automatically sets the Start Menu Recommended Section to Show when this setting is enabled as it is required"
        }
        "privacy_settings_notifications_title" => "Settings App Notifications",
        "privacy_settings_notifications_desc" => {
            "Shows account notifications in the Settings app, including prompts to reauthenticate, backup your device, and manage subscriptions"
        }
        "privacy_ads_title" => "Ads, Suggestions and Promotional Content",
        "privacy_ads_desc" => {
            "Controls all advertising, suggestions, and promotional content throughout Windows"
        }
        "privacy_ads_opt_0" => "Allow",
        "privacy_ads_opt_1" => "Deny",
        "privacy_ads_opt_2" => "Custom",
        "privacy_content_delivery_title" => "Content Delivery",
        "privacy_content_delivery_desc" => {
            "Allows Windows to deliver promotional content and automatically install suggested apps"
        }
        "privacy_subscribed_content_title" => "Subscribed Content",
        "privacy_subscribed_content_desc" => {
            "Enables promotional content subscriptions from Microsoft and partners throughout Windows"
        }
        "privacy_feature_management_title" => "Feature Management",
        "privacy_feature_management_desc" => {
            "Enables Windows feature management functionality for promotional features and automatic app installations"
        }
        "privacy_soft_landing_title" => "Soft Landing Experiences",
        "privacy_soft_landing_desc" => {
            "Enables helpful and promotional soft landing experiences in Windows"
        }
        "privacy_oem_preinstalled_title" => "OEM Pre-installed Apps",
        "privacy_oem_preinstalled_desc" => {
            "Prevents OEM manufacturers from automatically installing bloatware apps"
        }
        "privacy_preinstalled_title" => "Pre-installed Suggested Apps",
        "privacy_preinstalled_desc" => {
            "Prevents Microsoft from automatically installing suggested apps"
        }
        "privacy_preinstalled_ever_title" => "Pre-installed Apps History Tracking",
        "privacy_preinstalled_ever_desc" => {
            "Disables tracking of whether pre-installed apps were ever enabled"
        }
        "privacy_silent_installed_title" => "Silent App Installation",
        "privacy_silent_installed_desc" => {
            "Prevents apps from being silently installed in the background"
        }
        "privacy_speech_title" => "Online Speech Recognition",
        "privacy_speech_desc" => {
            "Use your voice for apps using Microsoft's online speech recognition technology"
        }
        "privacy_narrator_online_title" => "Narrator Online Services",
        "privacy_narrator_online_desc" => {
            "Allow Narrator to use Microsoft cloud services for features like intelligent image descriptions and enhanced voice models"
        }
        "privacy_narrator_scripting_title" => "Narrator Scripting Support",
        "privacy_narrator_scripting_desc" => {
            "Allow Narrator to execute scripts for automation and custom functionality"
        }
        "privacy_inking_title" => "Custom Inking and Typing Dictionary",
        "privacy_inking_desc" => {
            "Uses your typing history and handwriting patterns to create a custom dictionary (turning off will clear all words in your custom dictionary)"
        }
        "privacy_diagnostics_title" => "Send Diagnostic Data",
        "privacy_diagnostics_desc" => {
            "Send diagnostic data to Microsoft to help improve Windows and keep it secure"
        }
        "privacy_improve_inking_title" => "Improve inking and typing",
        "privacy_improve_inking_desc" => {
            "Send optional inking and typing diagnostic data to Microsoft"
        }
        "privacy_tailored_experiences_title" => "Tailored Experiences",
        "privacy_tailored_experiences_desc" => {
            "Let Microsoft use your diagnostic data to show personalized tips, ads and recommendations"
        }
        "privacy_feedback_title" => "Allow Windows to ask you for feedback",
        "privacy_feedback_desc" => {
            "Let Windows ask you to provide feedback on experiences in Windows"
        }
        "privacy_activity_history_title" => "Activity History",
        "privacy_activity_history_desc" => {
            "Allows you to jump back into what you were doing with apps, docs, or other activities on startup"
        }
        "privacy_timeline_title" => "Timeline Suggestions",
        "privacy_timeline_desc" => "Shows suggestions in the Windows 10 Timeline feature",
        "privacy_search_history_title" => "Search history on this device",
        "privacy_search_history_desc" => {
            "Improves search results by allowing Windows Search to store your search history locally on this device (Does not clear existing history)"
        }
        "privacy_search_highlights_title" => "Show search highlights",
        "privacy_search_highlights_desc" => "See content suggestions in search",
        "privacy_search_msa_title" => "Cloud Content Search for Microsoft account",
        "privacy_search_msa_desc" => {
            "Allow Windows Search to show results from apps and services that you are signed in to with your Microsoft account"
        }
        "privacy_search_aad_title" => "Cloud Content Search for Work or School account",
        "privacy_search_aad_desc" => {
            "Allow Windows Search to show results from apps and services that you are signed in to with your work or school account"
        }
        "privacy_cortana_title" => "Allow Cortana",
        "privacy_cortana_desc" => {
            "Enables Microsoft's Cortana virtual assistant for voice commands and searches"
        }
        "privacy_location_title" => "Location Services",
        "privacy_location_desc" => {
            "Allows Windows and apps to access your device location for location-based features"
        }
        "privacy_camera_title" => "Camera Access",
        "privacy_camera_desc" => "Allow apps to have camera access",
        "privacy_microphone_title" => "Microphone Access",
        "privacy_microphone_desc" => "Allow apps to have microphone access",
        "privacy_account_info_title" => "Account Info Access",
        "privacy_account_info_desc" => "Allow apps to have account info access",
        "privacy_app_diagnostic_title" => "App Diagnostic Access",
        "privacy_app_diagnostic_desc" => "Allow apps to have app diagnostic access",
        "privacy_onedrive_backup_title" => "OneDrive Automatic Backups",
        "privacy_onedrive_backup_desc" => {
            "Controls whether OneDrive automatically backs up your Documents, Pictures, and Desktop folders. Has no effect if OneDrive backups are already active on your device"
        }
        "privacy_copilot_title" => "Windows Copilot",
        "privacy_copilot_desc" => {
            "Controls whether Windows Copilot is available system-wide via group policy for both current user and local machine"
        }
        "privacy_ai_data_title" => "AI Data Analysis",
        "privacy_ai_data_desc" => {
            "Controls whether Windows AI can analyze user data for personalization and recommendations"
        }
        "privacy_recall_enable_title" => "Recall Enablement",
        "privacy_recall_enable_desc" => "Controls whether Windows Recall can be enabled via policy",
        "privacy_recall_snapshots_title" => "Recall Saving Snapshots",
        "privacy_recall_snapshots_desc" => {
            "Allows Windows Recall to save screenshots of your activity for later recall"
        }
        "privacy_click_to_do_title" => "Click to Do",
        "privacy_click_to_do_desc" => {
            "Controls whether the Click to Do AI feature is available in Windows"
        }
        "privacy_settings_agent_title" => "AI Settings Agent",
        "privacy_settings_agent_desc" => {
            "Controls whether the AI-powered Settings Agent is available in Windows"
        }
        "privacy_agent_connectors_title" => "AI Agent Connectors",
        "privacy_agent_connectors_desc" => {
            "Controls whether AI agents can use connectors to access external services"
        }
        "privacy_agent_workspaces_title" => "AI Agent Workspaces",
        "privacy_agent_workspaces_desc" => {
            "Controls whether AI Agent Workspaces are available in Windows"
        }
        "privacy_remote_agent_connectors_title" => "Remote AI Agent Connectors",
        "privacy_remote_agent_connectors_desc" => {
            "Controls whether AI agents can use remote connectors to access remote services"
        }
        "privacy_copilot_key_title" => "Copilot Hardware Key",
        "privacy_copilot_key_desc" => {
            "Controls whether the dedicated Copilot key on keyboards opens Copilot"
        }
        "privacy_copilot_runtime_title" => "Copilot Runtime",
        "privacy_copilot_runtime_desc" => {
            "Controls whether the Copilot runtime is allowed to run via policy"
        }
        "privacy_copilot_available_title" => "Copilot Availability",
        "privacy_copilot_available_desc" => {
            "Controls whether Copilot is available in the Windows Shell"
        }
        "privacy_bing_chat_title" => "Bing Chat Eligibility",
        "privacy_bing_chat_desc" => {
            "Controls whether the user is eligible for Bing Chat and Copilot in Search"
        }
        "privacy_generative_ai_title" => "Generative AI Access",
        "privacy_generative_ai_desc" => {
            "Controls whether apps can access the generative AI capability on your device"
        }
        "privacy_system_ai_title" => "System AI Models Access",
        "privacy_system_ai_desc" => {
            "Controls whether apps can access system AI models on your device and collect usage data"
        }
        "privacy_copilot_microphone_title" => "Copilot Microphone Access",
        "privacy_copilot_microphone_desc" => {
            "Controls whether Copilot and Office Hub apps have microphone permission"
        }
        "privacy_paint_image_creator_title" => "Paint AI Image Creator",
        "privacy_paint_image_creator_desc" => {
            "Controls whether the AI Image Creator feature is available in Microsoft Paint"
        }
        "privacy_paint_cocreator_title" => "Paint AI Cocreator",
        "privacy_paint_cocreator_desc" => {
            "Controls whether the AI Cocreator feature is available in Microsoft Paint"
        }
        "privacy_paint_fill_title" => "Paint Generative Fill",
        "privacy_paint_fill_desc" => {
            "Controls whether the AI Generative Fill feature is available in Microsoft Paint"
        }
        "privacy_paint_erase_title" => "Paint Generative Erase",
        "privacy_paint_erase_desc" => {
            "Controls whether the AI Generative Erase feature is available in Microsoft Paint"
        }
        "privacy_paint_background_title" => "Paint Remove Background",
        "privacy_paint_background_desc" => {
            "Controls whether the AI Remove Background feature is available in Microsoft Paint"
        }
        "privacy_input_insights_title" => "Input Insights",
        "privacy_input_insights_desc" => {
            "Controls whether Windows Input Insights can track typing patterns and provide suggestions"
        }
        "privacy_copilot_nudges_title" => "Copilot Nudges",
        "privacy_copilot_nudges_desc" => {
            "Controls whether Copilot promotional nudges and background task notifications are shown"
        }
        "privacy_consumer_ai_title" => "AI Consumer Content",
        "privacy_consumer_ai_desc" => {
            "Controls whether AI-driven consumer account content recommendations are shown"
        }
        "privacy_edge_cdp_title" => "Edge Copilot CDP Page Context",
        "privacy_edge_cdp_desc" => {
            "Controls whether Copilot can use CDP to access page content in Microsoft Edge"
        }
        "privacy_edge_page_title" => "Edge Copilot Page Context",
        "privacy_edge_page_desc" => {
            "Controls whether Copilot can read page content in Microsoft Edge"
        }
        "privacy_edge_sidebar_title" => "Edge Copilot Sidebar",
        "privacy_edge_sidebar_desc" => {
            "Controls whether the Copilot sidebar is available in Microsoft Edge"
        }
        "privacy_edge_entra_title" => "Edge Entra Copilot Page Context",
        "privacy_edge_entra_desc" => {
            "Controls whether Entra Copilot can access page context in Microsoft Edge"
        }
        "privacy_edge_m365_icon_title" => "Edge M365 Copilot Chat Icon",
        "privacy_edge_m365_icon_desc" => {
            "Controls whether the Microsoft 365 Copilot chat icon is shown in Microsoft Edge"
        }
        "privacy_edge_history_title" => "Edge AI History Search",
        "privacy_edge_history_desc" => {
            "Controls whether AI-powered history search is available in Microsoft Edge"
        }
        "privacy_edge_inline_title" => "Edge Inline AI Compose",
        "privacy_edge_inline_desc" => {
            "Controls whether AI-powered inline compose suggestions are available in Microsoft Edge"
        }
        "privacy_edge_local_model_title" => "Edge Local AI Model Settings",
        "privacy_edge_local_model_desc" => {
            "Controls whether local AI model settings are available in Microsoft Edge"
        }
        "privacy_edge_builtin_title" => "Edge Built-in AI APIs",
        "privacy_edge_builtin_desc" => {
            "Controls whether built-in AI APIs are available in Microsoft Edge for websites to use"
        }
        "privacy_edge_themes_title" => "Edge AI Generated Themes",
        "privacy_edge_themes_desc" => {
            "Controls whether AI-generated themes are available in Microsoft Edge"
        }
        "privacy_edge_devtools_title" => "Edge DevTools AI",
        "privacy_edge_devtools_desc" => {
            "Controls whether AI features are available in Edge DevTools"
        }
        "privacy_edge_share_history_title" => "Edge Share History with Copilot",
        "privacy_edge_share_history_desc" => {
            "Controls whether browsing history is shared with Copilot search in Microsoft Edge"
        }
        "privacy_office_training_title" => "Office AI Training",
        "privacy_office_training_desc" => {
            "Controls whether Office collects AI training data from your usage"
        }
        "privacy_office_connected_title" => "Office Connected Services",
        "privacy_office_connected_desc" => {
            "Controls whether Office connected experiences and AI-powered services are available"
        }
        "privacy_word_copilot_title" => "Word Copilot",
        "privacy_word_copilot_desc" => {
            "Controls whether Copilot AI features are available in Microsoft Word"
        }
        "privacy_excel_copilot_title" => "Excel Copilot",
        "privacy_excel_copilot_desc" => {
            "Controls whether Copilot AI features are available in Microsoft Excel"
        }
        "privacy_onenote_copilot_title" => "OneNote Copilot",
        "privacy_onenote_copilot_desc" => {
            "Controls whether Copilot AI features, Copilot notebooks, and Copilot skittle are available in Microsoft OneNote"
        }
        "privacy_office_safety_title" => "Office AI Content Safety",
        "privacy_office_safety_desc" => {
            "Controls whether AI content safety features for alt text, rewrite, and summarization are available in Office apps"
        }
        "privacy_security_group_0" => "Security",
        "privacy_security_group_0_desc" => "Security-related Windows privacy controls.",
        "privacy_security_group_1" => "Content Delivery & Advertising",
        "privacy_security_group_1_desc" => "Advertising, suggestions, and promotional content.",
        "privacy_security_group_2" => "Lock Screen",
        "privacy_security_group_2_desc" => "Lock screen behaviors and Spotlight content.",
        "privacy_security_group_3" => "General",
        "privacy_security_group_3_desc" => "General Windows privacy settings.",
        "privacy_security_group_4" => "Speech",
        "privacy_security_group_4_desc" => "Speech recognition and Narrator settings.",
        "privacy_security_group_5" => "Inking and typing personalization",
        "privacy_security_group_5_desc" => "Typing personalization and custom dictionaries.",
        "privacy_security_group_6" => "Diagnostics & Feedback",
        "privacy_security_group_6_desc" => "Telemetry, feedback, and diagnostic collection.",
        "privacy_security_group_7" => "Activity History",
        "privacy_security_group_7_desc" => "Activity history and timeline suggestions.",
        "privacy_security_group_8" => "Search permissions",
        "privacy_security_group_8_desc" => "Search history, cloud search, and Cortana.",
        "privacy_security_group_9" => "App Permissions",
        "privacy_security_group_9_desc" => "Location, camera, microphone, and app access.",
        "privacy_security_group_10" => "Windows AI",
        "privacy_security_group_10_desc" => "Windows AI and Copilot-related settings.",
        "privacy_security_group_11" => "Microsoft Edge AI",
        "privacy_security_group_11_desc" => "Microsoft Edge AI settings.",
        "privacy_security_group_12" => "Microsoft Office AI",
        "privacy_security_group_12_desc" => "Microsoft Office AI settings.",
        "privacy_option_on" => "On",
        "privacy_option_off" => "Off",
        "settings" => "Settings",
        "check_updates" => "Check Updates",
        "donate" => "Donate",
        "bug_report" => "Bug Report",
        "status_admin" => "Administrator",
        "status_standard" => "Standard",
        "settings_title" => "Settings",
        "settings_subtitle" => "Keep the app behavior aligned with your workflow.",
        "settings_application" => "Application",
        "settings_saved_auto" => "These settings are saved automatically.",
        "language" => "Language",
        "sidebar_languages" => "Languages",
        "archive-restore" => "Restore",
        "settings-2" => "Settings",
        "shield-check" => "Security Status",
        "refresh-cw" => "Refresh",
        "monitor" => "Console",
        "Winchisel-Updater" => "Winchisel-Updater",
        "Disabled (Recommended)" => "Disabled (Recommended)",
        "0" => "0",
        "1" => "1",
        "app_name_m365_copilot_office_hub" => "MS 365 Copilot (Office Hub)",
        "app_name_remote_assistance" => "Remote assistance app",
        "app_name_classic_paint" => "Classic Paint app",
        "windows-app-office-hub" => "Microsoft 365 Copilot (formerly known as Office hub)",
        "windows-app-remote-assistance" => "Remote assistance app",
        "capability-paint-legacy" => "Classic Paint app",
        "open_logs" => "Open Logs",
        "check_updates_startup" => "Check updates on startup",
        "show_console" => "Show console",
        "system_protection" => "System Protection",
        "restore_point" => "System Restore Point",
        "restore_point_desc" => "Create a rollback point before major system changes",
        "create_restore_point" => "Create Restore Point",
        "performance_title" => "Performance",
        "performance_subtitle" => "Gaming and Performance Tweaks",
        "performance_search" => "Search performance tweaks...",
        "performance_quick" => "Quick Actions",
        "performance_apply_recommended" => "Apply Recommended Settings",
        "performance_reset_defaults" => "Reset to Windows Defaults",
        "performance_loading" => "Loading performance tweaks...",
        "performance_empty" => "No performance tweaks to display.",
        "performance_current_default" => "Default: ",
        "performance_current_recommended" => "Recommended: ",
        "gaming-game-mode" => "Optimize your PC for gaming by turning things off in the background",
        "gaming-performance-explorer-mouse-precision" => {
            "Adjust cursor speed based on movement speed (mouse acceleration). Most competitive gamers disable this for consistent aiming in FPS games"
        }
        "gaming-performance-mouse-hover-time" => {
            "Controls how long you have to hover over an element before it becomes active (in milliseconds). Lower values cause tooltips, menus and hover effects to appear faster. The default value is 400 ms"
        }
        "gaming-performance-autostart-delay" => {
            "Delay the launch of applications for 10 seconds after startup to improve initial system responsiveness. Windows becomes faster to use, but your startup apps take longer to load"
        }
        "gaming-background-apps" => {
            "Use Group Policy to control whether apps can run in the background. Force Deny removes per-app background settings from Windows Settings. Use User in Control if you need apps like Teams, Zoom or WhatsApp"
        }
        "gaming-storage-sense" => {
            "Automatically free up space by removing temporary files, emptying the trash, and managing downloads"
        }
        "gaming-performance-explorer-search" => {
            "Search your entire file system instead of just indexed locations. This provides more complete results, but is significantly slower than indexed search and increases disk activity"
        }
        "gaming-performance-search-webview2" => {
            "Allow Windows Search to use WebView2 (Edge) to render search results. Disabling it will remove edge processes created by SearchHost.exe, thereby reducing resource usage. Uses an undocumented Windows Feature Management override (Feature ID 37926450), which may change in future Windows updates"
        }
        "gaming-performance-wallpaper-compression" => {
            "Allow Windows to compress wallpapers to save disk space and improve performance. Only affects images in JPEG format."
        }
        "gaming-performance-explorer-menu-show-delay" => {
            "Add a short delay before menus appear (400ms - Windows default) or show them immediately (0ms) to speed up navigation"
        }
        "gaming-explorer-alt-tab-filter" => {
            "Alt+Tab only shows traditional open windows instead of including Microsoft Edge tabs and other Windows suggestions"
        }
        "gaming-win32-priority" => {
            "Configure how Windows allocates CPU time between foreground applications and background services"
        }
        "gaming-system-responsiveness" => {
            "Minimize disruption from background tasks by allocating more CPU time to your active game or multimedia application"
        }
        "gaming-cpu-priority" => {
            "Give games a higher CPU scheduling priority to dedicate more processor time to your game"
        }
        "gaming-scheduling-category" => {
            "Assign a high priority scheduling category to ensure games receive preferential system resource allocation"
        }
        "gaming-performance-svchost-split-threshold" => {
            "Set the memory threshold that determines when Windows splits services into separate svchost.exe processes. Higher values group more services together, reducing the number of processes. Select the value that corresponds to your system RAM"
        }
        "gaming-gpu-priority" => {
            "Give games a higher GPU scheduling priority to improve graphics performance and frame rates"
        }
        "gaming-gpu-scheduling" => {
            "Let your GPU manage its own memory and scheduling to reduce latency and improve performance"
        }
        "gaming-directx-flip-model" => {
            "Reduce latency and leverage advanced features in compatible games by using the DirectX Flip presentation model"
        }
        "gaming-directx-vrr-optimizations" => {
            "Enable VRR optimizations (G-Sync/FreeSync) for smoother gameplay. Requires a VRR compatible monitor; This setting has no effect if your monitor does not support VRR"
        }
        "gaming-directx-auto-hdr" => {
            "Automatically convert SDR content to HDR to improve colors and brightness. Requires an HDR-capable display with HDR enabled; This setting has no effect if your display does not support HDR"
        }
        "gaming-nvidia-sharpening" => {
            "Enable the legacy NVIDIA image sharpening filter for improved visual clarity. Only works with older NVIDIA drivers; Newer drivers should use NVIDIA Control Panel sharpening instead"
        }
        "gaming-fullscreen-optimizations" => {
            "Allow Windows to optimize games running in full screen mode. Disabling it may resolve performance issues or stuttering in some older games that don't work well with borderless fullscreen optimization"
        }
        "gaming-performance-desktop-composition" => {
            "Enable visual effects managed by Desktop Window Manager. Disabling this may provide small performance gains on older hardware, but will affect aero effects"
        }
        "gaming-auto-color-management" => {
            "Allow Windows to automatically manage color profiles for all connected displays that support it"
        }
        "gaming-disable-mpo" => {
            "Compositing multiple display layers in hardware using the GPU. Disabling it can resolve screen flickering, black screens, and stuttering in multi-monitor configurations"
        }
        "gaming-disable-mpo-min-fps" => {
            "Enable Desktop Window Managers to dynamically switch apps between overlay modes based on frame rate. Disabling it can fix stuttering in browsers and Discord without having to completely disable MPO"
        }
        "gaming-network-throttling" => {
            "Controls network packet rate limiting for multimedia applications. It is recommended to leave throttling enabled (default: 10 packets/ms) as it provides better DPC latency for gaming than disabling it completely"
        }
        "gaming-nagle-algorithm" => {
            "Buffers small network packets before sending to reduce overhead. Turn it off to reduce latency during online gaming or leave it on for overall network efficiency"
        }
        "gaming-dns-server" => {
            "Select a DNS server for all network adapters. Changes apply to each adapter in your system (WiFi and Ethernet). Use �Automatic� to restore your default ISP/Router DNS"
        }
        "gaming-virtualization-based-security" => {
            "Isolates portions of memory to protect the system from vulnerabilities. Disabling it may improve gaming performance but reduces system security"
        }
        "gaming-memory-integrity" => {
            "Prevents the introduction of malicious code into high-security processes. Disabling it may improve gaming performance but reduces system security"
        }
        "gaming-xbox-game-dvr" => {
            "Record gameplay clips and take screenshots with the Xbox Game Bar overlay. Disabling this reduces CPU/GPU usage and can improve frame rates"
        }
        "gaming-game-bar-controller" => {
            "Allow your Xbox or compatible controller to open the Game Bar by pressing the Xbox button. Disable this option to prevent accidental activation of the Game Bar while playing"
        }
        "gaming-game-bar-tips" => {
            "Displays tips and hints about Game Bar features when the overlay opens. Disabling it will reduce distractions during gameplay"
        }
        "gaming-performance-background-services" => {
            "Reduce the startup timeout for Windows services from 60 to 30 seconds. This may reduce boot time slightly"
        }
        "gaming-sysmain-service" => {
            "Preload frequently used applications into RAM for faster boot times. Automatic is recommended for hard drives or mixed storage systems; �Manual� or �Disabled� is only suitable for pure SSD systems"
        }
        "gaming-performance-prefetch" => {
            "Preload frequently used applications and boot files into memory to speed up startup. Generally recommended for HDDs and not SSDs"
        }
        "gaming-windows-search-service" => {
            "Indexes files and folders for faster search results. Disabling it will reduce CPU and disk activity in the background, but will break Outlook search and make Start menu and File Explorer search slow or unreliable"
        }
        "gaming-print-spooler-service" => {
            "Manages print jobs sent to printers. If you are not using a printer, set it to Manual or Disabled to free up system resources"
        }
        "gaming-telemetry-service" => {
            "Sends usage data and diagnostics to Microsoft. Setting it to Manual or Disabled reduces background network and CPU usage"
        }
        "gaming-connected-devices-platform-service" => {
            "Enables cross-device experiences like phone linking and nearby sharing. Disabling reduces background activity and logging of device interactions"
        }
        "gaming-compatibility-assistant-service" => {
            "Monitors programs for compatibility issues and suggests fixes. Disabling it will prevent compatibility prompts and save minor system resources"
        }
        "gaming-error-reporting-service" => {
            "Collects and sends crash data to Microsoft. Disabling it prevents crash reports, reduces network traffic, and improves data protection with minimal impact to the system"
        }
        "gaming-geolocation-service" => {
            "Tracks your physical location for apps and services. Disabling improves privacy and prevents location tracking, but apps cannot use location features"
        }
        "gaming-retail-demo-service" => {
            "Controls device activity in retail demo mode. For PCs, disabling may be safe as it is for retail display purposes only"
        }
        "gaming-insider-service" => {
            "Manages Windows Insider Program features and preview builds. Deactivation is safe if you do not participate in the Windows Insider Program"
        }
        "gaming-phone-service" => {
            "Manages telephony status on the device. You can easily disable the feature if you don't use phone connection features or don't make calls from your PC"
        }
        "gaming-wallet-service" => {
            "Provides wallet functionality for payment and NFC scenarios. It is safe to deactivate if you do not use Microsoft Wallet features"
        }
        "gaming-smart-card-services" => {
            "Enables the smart card reader function for security authentication. Deactivation is safe if you do not use physical smart cards or card readers"
        }
        "gaming-maps-broker-service" => {
            "Provides access to downloaded maps for applications. Set the option to Manual to allow access to the map when needed while preventing unnecessary background activity"
        }
        "gaming-fax-service" => {
            "Allows you to send and receive faxes. For most users, disabling it is safe because the fax feature is rarely used on modern systems"
        }
        "gaming-wmp-network-service" => {
            "Shares Windows Media Player libraries with other networked players and media devices. You can safely disable the feature if you don't share media over your network"
        }
        "gaming-mixed-reality-service" => {
            "Runs OpenXR applications on Windows Mixed Reality devices. It's safe to deactivate if you don't use VR or AR headsets"
        }
        "gaming-mobile-hotspot-service" => {
            "Provides the ability to share the Internet connection with other devices. Set to Manual to keep functionality available while preventing unnecessary background activity"
        }
        "gaming-sms-router-service" => {
            "Forwards SMS messages according to rules. You can safely disable the feature if you don't use SMS features on your PC"
        }
        "gaming-parental-controls-service" => {
            "Enables parental controls and family safety features. You can safely disable the feature if you don't use parental control features"
        }
        "gaming-payments-nfc-service" => {
            "Manages payments and secure elements of near field communication. It is safe to deactivate if you do not use the NFC payment functions"
        }
        "gaming-spot-verifier-service" => {
            "Checks for possible file system corruption. Set the option to Manual to enable checking when needed while reducing background activity"
        }
        "gaming-remote-access-manager" => {
            "Manages VPN and dial-up connections. Set to Manual to reduce background activity while keeping VPN functionality available when needed."
        }
        "gaming-remote-access-auto" => {
            "Automatically connects to remote networks when programs reference remote resources. You can safely disable the feature if you don't use the VPN's auto-connect features"
        }
        "gaming-remote-desktop-services" => {
            "Allows users to interactively connect to a remote computer. Set to Manual to reduce background activity while keeping remote desktop available."
        }
        "gaming-remote-desktop-configuration" => {
            "Manages Remote Desktop Services and Remote Desktop-related configurations. Set to Manual to reduce background activity while keeping remote desktop available"
        }
        "gaming-remote-desktop-port-redirector" => {
            "Enables local device redirection for remote desktop connections. It's safe to disable if you don't need to share local devices during remote desktop sessions"
        }
        "gaming-xbox-auth-manager" => {
            "Provides authentication and authorization services for Xbox Live. It's safe to deactivate if you don't use Xbox Game Pass, Microsoft Store games, or Xbox features"
        }
        "gaming-xbox-game-save" => {
            "Syncs save games to the Xbox Live cloud. Only required for Xbox Game Pass and Microsoft Store games with cloud save capabilities"
        }
        "gaming-xbox-networking" => {
            "Supports Xbox Live multiplayer networks. Required for Xbox multiplayer games, but not for Steam/Epic/other gaming platforms"
        }
        "gaming-biometric-service" => {
            "Enables fingerprint and facial recognition login via Windows Hello. Safe to disable on desktop systems without biometric hardware"
        }
        "gaming-touch-keyboard-service" => {
            "Manages the Windows input experience, including touch keyboard, pen/stylus input, handwriting panel, emoji panel (Win+.), and Xbox controller keyboard. Disabling this will break all virtual/software keyboard input, but is safe on desktop systems without a touchscreen, stylus, or gamepad"
        }
        "gaming-sensor-monitoring-service" => {
            "Monitors various sensors such as ambient light and orientation. Safe to disable on desktop systems without sensor hardware"
        }
        "gaming-sensor-data-service" => {
            "Delivers data from a variety of sensors to applications. Safe to disable on desktop systems without sensor hardware"
        }
        "gaming-ai-fabric-service" => {
            "Windows AI Fabric Service (WSAIFabricSvc) manages AI workloads. Disable this option if you do not use Windows AI features"
        }
        "CompatibilityAppraiserTask" => {
            "Collects program compatibility telemetry data for Windows upgrades. Works in conjunction with the Connected User Experiences and Telemetry service. Disable to reduce telemetry and background system activity"
        }
        "ProgramDataUpdaterTask" => {
            "Updates the program compatibility database with information about installed applications. Disable this option to reduce telemetry collection"
        }
        "CEIPConsolidatorTask" => {
            "Consolidates and uploads usage data as part of the customer experience improvement program. Works with the Connected User Experiences and Telemetry service. Disable to improve privacy"
        }
        "UsbCeipTask" => {
            "Collects USB device related telemetry data for customer satisfaction improvement program. Disable to reduce telemetry"
        }
        "DiskDiagnosticTask" => {
            "Collects hard drive diagnostic information and S.M.A.R.T. Data for Microsoft. Disable this option to reduce background disk activity and telemetry"
        }
        "FeedbackDmClientTask" => {
            "Collects feedback and diagnostic data for Microsoft. Disable this option to improve privacy and reduce telemetry"
        }
        "FeedbackDmClientDownloadTask" => {
            "Downloads feedback scenarios and configuration data from Microsoft. Disable to reduce telemetry and network activity"
        }
        "ErrorReportingQueueTask" => {
            "Queues crash reports and error data to be sent to Microsoft. Works in conjunction with the Windows Error Reporting Service. Disable both to prevent crash data collection"
        }
        "SqmTask" => {
            "Collects software quality metrics and reliability data for Microsoft telemetry. Disable to improve privacy"
        }
        "MareBackupTask" => {
            "Backs up Microsoft Assisted Recovery data. Disable this option to reduce system activity in the background"
        }
        "StartupAppTask" => {
            "Tracks and monitors startup applications for telemetry and diagnostics. Disable to reduce telemetry"
        }
        "MapsUpdateTask" => {
            "Updates offline map data for the Windows Maps app. Disable this option when you are not using the Maps app to save bandwidth and storage space"
        }
        "AutochkProxyTask" => {
            "Performs hard drive checks and collects diagnostic data. Consider leaving disk health monitoring enabled"
        }
        "FamilySafetyTask" => {
            "Monitors family security settings and usage. Disable this option if you do not use the family safety features"
        }
        "PowerEfficiencyTask" => {
            "Analyzes system power consumption and collects energy efficiency data. Disable this option to reduce telemetry and background analysis"
        }
        "WindowsAIRecallConfig" => {
            "Scheduled Windows AI tasks, including recall configuration. Disable this option to prevent AI features from running in the background"
        }
        "WindowsAIRecallPipeline" => {
            "Windows AI Recall Pipeline Task. Disable this option to prevent the recall pipeline from processing in the background"
        }
        "OfficeActionsServer" => {
            "Office AI Actions Server scheduled task. Disable this option to prevent Office AI from running in the background"
        }
        "visual-effects-mode" => "Choose how Windows displays visual effects",
        "ui-effects" => "Enables animation effects for controls and UI elements",
        "window-animation" => "Displays a smooth animation when windows are minimized or maximized",
        "taskbar-animations" => {
            "Controls the taskbar animation effects for opening, closing, and switching windows"
        }
        "enable-peek" => {
            "Allows you to view the desktop when you hover over the Show Desktop button"
        }
        "menu-animation" => "Animates menus when displayed using fade or slide effects",
        "fade-tooltip" => "Animates tooltips when displayed using fade or slide effects",
        "fade-menu-items" => "Hides menu items after selection before closing the menu",
        "taskbar-thumbnails" => "Saves thumbnail previews of taskbar windows for faster viewing",
        "mouse-shadow" => "Displays a shadow effect under the mouse pointer",
        "window-shadows" => "Displays shadow effects under windows",
        "show-thumbnails" => "Displays image and document previews instead of generic file icons",
        "translucent-selection" => {
            "Display a semi-transparent selection box as you drag to select multiple files or items"
        }
        "drag-full-windows" => {
            "When you drag, displays the window contents instead of just an outline"
        }
        "combo-box-animation" => "Animates combo boxes when opened with a sliding effect",
        "font-smoothing" => {
            "Apply anti-aliasing to text for smoother, more readable fonts on the screen"
        }
        "smooth-scroll-listboxes" => "Allows smooth scrolling in list boxes instead of jumping",
        "drop-shadows" => {
            "Add shadow effects behind desktop icon text to improve readability against the background"
        }
        "gaming-narrator-hotkey" => {
            "Enable the Win+Ctrl+Enter keyboard shortcut to quickly launch the Windows Narrator screen reader"
        }
        "accessibility-stickykeys-hotkey" => {
            "Activate the keyboard shortcut to activate StickyKeys by pressing the Shift key five times"
        }
        "accessibility-filterkeys-hotkey" => {
            "Enable the keyboard shortcut to enable FilterKeys by holding down the right Shift key for 8 seconds"
        }
        "accessibility-togglekeys-hotkey" => {
            "To activate ToggleKeys, activate the keyboard shortcut by holding down the Num Lock key for 5 seconds. This will play sounds when the Caps Lock/Num Lock/Scroll key is pressed"
        }
        "accessibility-mousekeys-hotkey" => {
            "Enable the keyboard shortcut to enable MouseKeys, which allows using the numeric keypad to control the mouse pointer"
        }
        "accessibility-highcontrast-hotkey" => {
            "To enable High Contrast mode, activate the keyboard shortcut by pressing Left Alt + Left Shift + Screen Print"
        }
        "performance_group_0" => "Gaming",
        "performance_group_1" => "Processor",
        "performance_group_2" => "Graphics",
        "performance_group_3" => "Network",
        "performance_group_4" => "Security",
        "performance_group_5" => "Xbox",
        "performance_group_6" => "System Services",
        "performance_group_7" => "Scheduled Tasks",
        "performance_group_8" => "Visual Effects",
        "performance_group_9" => "Accessibility",
        "home_system" => "System",
        "home_processor" => "Processor",
        "home_graphics" => "Graphics",
        "home_memory" => "Memory",
        "home_storage" => "Storage",
        "home_windows" => "Windows",
        "home_uptime" => "Uptime",
        "home_performance" => "Performance",
        "home_product_name" => "System Product Name",
        "home_cpu" => "CPU",
        "home_cpu_model" => "CPU Model",
        "home_cores" => "Cores",
        "home_gpu" => "GPU",
        "home_memory_total" => "Memory Total",
        "home_memory_used" => "Memory Used",
        "home_version" => "Version",
        "home_kernel" => "Kernel",
        "home_name" => "Name",
        "home_bios_version" => "BIOS Version",
        "home_bios_date" => "BIOS Date",
        "home_unknown_cpu" => "Unknown CPU",
        "home_unknown_pc" => "Unknown PC",
        "home_unknown_os" => "Unknown OS",
        "home_unknown_kernel" => "Unknown kernel",
        "home_unknown_model" => "Unknown Model",
        "home_unknown_vendor" => "Unknown Vendor",
        "home_unknown_bios" => "Unknown BIOS",
        "home_unknown_date" => "Unknown Date",
        "home_unknown_gpu" => "Unknown GPU",
        "home_unknown_vram" => "Unknown VRAM",
        "home_unknown_value" => "Unknown",
        "home_cores_suffix" => "{} cores",
        "home_gb_total" => "{:.1} GB total",
        "home_gb_used" => "{:.1} GB used",
        "home_tb_total" => "{:.2} TB total",
        "home_tb_used" => "{:.2} TB used",
        "home_uptime_fmt" => "{}d {:02}h {:02}m",
        "home_update_status" => "Update status",
        "update_available_prefix" => "Update available:",
        "update_failed_prefix" => "Update check failed:",
        "update_newer_version_prefix" => "A newer version is available:",
        "update_ready" => "Ready",
        "update_checked" => "You are already on the latest version.",
        "settings_saved" => "Settings saved",
        "update_no_found_title" => "No Update Found",
        "update_available_title" => "Update Available",
        "update_failed_title" => "Update Check Failed",
        "update_up_to_date" => "You are already on the latest version.",
        "update_error_check_updates" => "Update check failed",
        "update_error_read_response" => "Failed to read response",
        "update_error_parse_json" => "Failed to parse JSON",
        "update_error_no_tag_name" => "No tag_name in response",
        "update_error_download" => "Failed to download update",
        "update_error_create_temp_file" => "Failed to create temp file",
        "update_error_write_update_file" => "Failed to write update file",
        "update_error_download_too_small" => "Downloaded file is too small",
        "update_error_resolve_current_exe" => "Failed to resolve current executable",
        "update_error_write_update_script" => "Failed to write update script",
        "update_error_launch_updater" => "Failed to launch updater",
        "update_download_restart" => "Download and restart the app to install it.",
        "update_failed" => "Update check failed.",
        "update_close" => "Close",
        "update_check_again" => "Check Again",
        "update_download_restart_btn" => "Download & Restart",
        "restore_point_window" => "Create Restore Point",
        "restore_point_creating" => "Creating restore point. This can take a moment...",
        "restore_point_success" => "Restore point created successfully.",
        "restore_point_failed" => "Restore point failed",
        "restore_point_failed_run" => "Failed to run PowerShell.",
        "restore_point_failed_windows" => "Restore points are only supported on Windows.",
        "download_title" => "Apps & Downloads",
        "download_subtitle" => "Install useful apps from curated lists.",
        "download_search" => "Search apps...",
        "download_website" => "Website",
        "download_installed" => "Installed",
        "download_not_installed" => "Not installed",
        "download_nothing_selected" => "Nothing selected",
        "download_installing" => "Installing selected downloads...",
        "download_install_job_failed" => "Install job failed",
        "download_meta" => "Winget IDs: {}\nCategory: {}\nWebsite: {}",
        "download_confirm_install" => "Confirm Install",
        "download_cancel" => "Cancel",
        "download_confirm_title" => "Confirm App Install",
        "download_confirm_desc" => "These apps and downloads will be installed:",
        "download_result" => "Installed: {}  Failed: {}",
        "download_refresh" => "Refresh",
        "download_install_selected" => "Install Selected",
        "download_loading" => "Loading apps & downloads...",
        "download_none" => "No apps or downloads to display.",
        "download_category_0" => "Browsers",
        "download_category_1" => "Document Viewers",
        "download_category_2" => "Messaging, Email & Calendar",
        "download_category_3" => "Online Storage & Backup",
        "download_category_4" => "Multimedia",
        "download_category_5" => "Imaging",
        "download_category_6" => "Customization Utilities",
        "download_category_7" => "Gaming",
        "download_category_8" => "Compression",
        "download_category_9" => "File & Disk Management",
        "download_category_10" => "Remote Access",
        "download_category_11" => "Optical Disc Tools",
        "download_category_12" => "Other Utilities",
        "download_category_13" => "Privacy & Security",
        "download_category_14" => "Development Apps",
        "download_category_15" => "Runtimes & Dependencies",

        "Microsoft EdgeWebView" => "WebView2 runtime for Windows applications",
        "Thorium" => "Chromium-based browser with enhanced privacy features",
        "Mercury" => "Compiler optimized, private Firefox fork",
        "Mozilla Firefox" => "Popular web browser known for privacy and customization",
        "Google Chrome" => "Google's web browser with sync and extension support",
        "ungoogled-chromium" => "Chromium-based browser with privacy enhancements",
        "Brave" => "Privacy-focused browser with built-in ad blocking",
        "Opera" => "Feature-rich web browser with built-in VPN and ad blocker",
        "Opera GX" => "Gaming-oriented version of Opera with unique features",
        "Arc Browser" => "Innovative browser with a focus on design and user experience",
        "Tor Browser" => "Privacy-focused browser that routes traffic through the Tor network",
        "Vivaldi" => "Highly customizable browser with a focus on user control",
        "Waterfox" => "Firefox-based browser with a focus on privacy and customization",
        "Zen Browser" => "Privacy-focused browser with built-in ad blocking",
        "Mullvad Browser" => {
            "Privacy-focused browser designed to minimize tracking and fingerprints"
        }
        "Pale Moon Browser" => {
            "Open Source, Goanna-based web browser focusing on efficiency and customization"
        }
        "Maxthon" => "Privacy focused browser with built-in ad blocking and VPN",
        "Ablaze Floorp" => "Privacy focused browser with strong tracking protection",
        "DuckDuckGo" => "Privacy-focused search engine with a browser extension",
        "LibreOffice" => "Free and open-source office suite",
        "ONLYOFFICE Desktop Editors" => "100% open-source free alternative to Microsoft Office",
        "PDFgear" => {
            "Read, edit, convert, merge, and sign PDF files across devices, for completely free and without signing up."
        }
        "Foxit PDF Reader" => "Lightweight PDF reader with advanced features",
        "SumatraPDF" => {
            "PDF, eBook (epub, mobi), comic book (cbz/cbr), DjVu, XPS, CHM, image viewer for Windows"
        }
        "OpenOffice" => {
            "Discontinued open-source office suite. Active successor projects is LibreOffice"
        }
        "Adobe Acrobat Reader DC" => "PDF reader and editor",
        "Evernote" => "Note-taking app",
        "CherryTree" => {
            "Hierarchical note taking application with rich text and syntax highlighting"
        }
        "Okular" => "Universal document viewer supporting PDF, eBook, and more",
        "PDF24 Creator" => "Free PDF creator and converter",
        "Telegram Desktop" => "Instant messaging and voice calling app",
        "WhatsApp" => "Instant messaging and voice calling app",
        "Zoom Workplace" => "Video conferencing and messaging platform",
        "Discord" => "Voice, video and text communication service",
        "Pidgin" => "Multi-protocol instant messaging client",
        "Mozilla Thunderbird" => "Free email application",
        "eM Client" => "Email client with calendar, tasks, and chat",
        "Proton Mail" => "Secure email service with end-to-end encryption",
        "Trillian" => "Instant messaging application",
        "Google Drive" => "Cloud storage and file synchronization service",
        "Dropbox" => {
            "File hosting service that offers cloud storage, file synchronization, personal cloud"
        }
        "SugarSync" => {
            "Automatically access and share your photos, videos, and files in any folder"
        }
        "Nextcloud" => {
            "Access, share and protect your files, calendars, contacts, communication & more at home and in your organization"
        }
        "Proton Drive" => "Secure cloud storage with end-to-end encryption",
        "FreeFileSync" => "Open-source folder comparison and synchronization tool",
        "Hekasoft Backup & Restore" => {
            "The complete free solution for browser backup and management"
        }
        "VLC media player" => "Open-source multimedia player and framework",
        "iTunes" => "Media player and library",
        "AIMP" => "Audio player with support for various formats",
        "foobar2000" => "Advanced audio player for Windows",
        "MusicBee" => "Music manager and player",
        "Audacity" => "Audio editor and recorder",
        "GOM Player" => "Media player for Windows",
        "Spotify" => "Music streaming service",
        "MediaMonkey" => "Media manager and player",
        "HandBrake" => "Open-source video transcoder",
        "OBS Studio" => "Free and open source software for video recording and live streaming",
        "Streamlabs OBS" => {
            "Streaming software built on OBS with additional features for streamers"
        }
        "MPC-BE" => "Media Player Classic - Black Edition",
        "K-Lite Codec Pack (Mega)" => "Collection of codecs and related tools",
        "CapCut" => "Video editor",
        "PotPlayer64" => "Comprehensive multimedia player for Windows",
        "kdenlive" => "Free and open-source video editing software",
        "MediaInfo" => "Technical information display tool for multimedia files",
        "fre:ac - free audio converter" => "Free audio converter and CD ripper",
        "SMPlayer" => {
            "Media Player with built-in codecs that can play virtually all video and audio formats"
        }
        "Shotcut" => "Free, open-source, cross-platform video editor",
        "LosslessCut" => "Cross-platform FFmpeg GUI for fast, lossless video/audio trimming",
        "FxSound" => "Audio enhancer for boosting sound quality on Windows",
        "IrfanView64" => "Fast and compact image viewer and converter",
        "Krita" => "Digital painting and illustration software",
        "Blender" => "3D creation suite",
        "Paint.NET" => "Image and photo editing software",
        "GIMP" => "GNU Image Manipulation Program",
        "XnViewMP" => "Image viewer, browser and converter",
        "XnView" => "Image viewer, browser and converter (Classic Version)",
        "Inkscape" => "Vector graphics editor",
        "Greenshot" => "Screenshot tool with annotation features",
        "ShareX" => "Screen capture, file sharing and productivity tool",
        "Flameshot" => "Powerful yet simple to use screenshot software",
        "FastStone Image Viewer" => "Image browser, converter and editor",
        "Nilesoft Shell" => "Windows context menu customization tool",
        "StartAllBack (Win 11)" => "Windows 11 Start menu and taskbar customization",
        "StartIsBack++ (Win 10)" => "Windows 10 Start menu and taskbar customization",
        "Open-Shell" => "Classic style Start Menu for Windows",
        "Windhawk" => "Customization platform for Windows",
        "Lively Wallpaper" => "Free and open-source animated desktop wallpaper application",
        "Sucrose Wallpaper Engine" => "Free and open-source animated desktop wallpaper application",
        "Rainmeter" => "Desktop customization tool for Windows",
        "ExplorerPatcher" => "Utility that enhances the Windows Explorer experience",
        "John's Background Switcher" => {
            "Automatically changes your desktop wallpaper at regular intervals"
        }
        "Microsoft PowerToys" => {
            "Set of utilities for power users to tune and streamline their Windows experience"
        }
        "Nexus" => "The advanced docking system for Windows",
        "AutoHotkey v2" => "Free macro-creation and automation scripting language (v2, current)",
        "Steam" => "Digital distribution platform for PC gaming",
        "Epic Games Launcher" => "Digital distribution platform for PC gaming",
        "7-Zip" => "Open-source file archiver with a high compression ratio",
        "WinRAR archiver" => "File archiver with a high compression ratio",
        "PeaZip" => "Free file archiver utility. Open and extract RAR, TAR, ZIP files and more",
        "WinDirStat" => "Disk usage statistics viewer and cleanup tool",
        "WizTree" => "Disk space analyzer with extremely fast scanning",
        "TreeSize Free" => "Disk space manager",
        "Everything" => "Locate files and folders by name instantly",
        "TeraCopy" => "Copy files faster and more securely",
        "File Converter" => "Batch file converter for Windows",
        "Crystal Disk Info" => "Hard drive health monitoring utility",
        "Bulk Rename Utility" => "File renaming software for Windows",
        "IObit Unlocker" => "Tool to unlock files that are in use by other processes",
        "HiBit Uninstaller" => {
            "Completely Uninstall Stubborn Software, Windows Apps & Browser Extension"
        }
        "SanDisk Dashboard" => "Drive management tool for SanDisk SSDs and flash drives",
        "Rufus" => "Utility to create bootable USB flash drives",
        "Advanced Renamer" => "Batch file renaming utility with advanced options",
        "RustDesk" => "Fast Open-Source Remote Access and Support Software",
        "AnyDesk" => "Remote desktop software for remote access and support",
        "TeamViewer" => {
            "Remote control, desktop sharing, online meetings, web conferencing and file transfer"
        }
        "UltraViewer" => {
            "Helps you control your partner's computer to support them as if you were sitting in front of their screen"
        }
        "RealVNC Server" => "Remote access software",
        "RealVNC Viewer" => "Remote access software",
        "Chrome Remote Desktop" => "Remote access to your computer through Chrome browser",
        "Parsec" => {
            "Remote desktop reimagined. Secure, flexible, effortless access to whatever you do, at any time, from wherever you go"
        }
        "Parsec Virtual Display Driver" => "Virtual display driver for Parsec Remote Desktop",
        "Parsec Virtual USB Driver" => "Virtual USB driver for Parsec Remote Desktop",
        "InputLeap" => "Open-source KVM software for sharing mouse and keyboard between computers",
        "ImgBurn" => "Lightweight CD / DVD / HD DVD / Blu-ray burning application",
        "AnyBurn" => "Lightweight CD/DVD/Blu-ray burning software",
        "CDBurnerXP" => "Free CD/DVD/Blu-ray burning software",
        "CCleaner" => "System optimization and cleaning tool",
        "Snappy Driver Installer Origin" => "Driver installer and updater",
        "Wise Disk Cleaner" => "Free Disk Cleanup and Defragment Tool",
        "Wise Registry Cleaner" => "Registry cleaning and optimization tool",
        "UniGetUI" => "Universal package manager interface supporting WinGet, Chocolatey, and more",
        "OpenRGB" => "Open source RGB lighting control software",
        "OpenAudible" => "Audiobook manager and converter for Audible files",
        "NAPS2" => "Document scanning application with OCR support",
        "IObit Uninstaller" => {
            "Completely Uninstall Unwanted Software, Windows Apps & Browser Plug-ins"
        }
        "Revo Uninstaller" => {
            "Revo Uninstaller helps you to uninstall software and remove unwanted programs easily."
        }
        "Malwarebytes" => "Anti-malware software for Windows",
        "Malwarebytes AdwCleaner" => "Adware removal tool for Windows",
        "Windows Firewall Control" => "Malwarebytes Windows Firewall Control application",
        "OnionShare" => {
            "Securely and anonymously share files, host websites, and chat via Tor network"
        }
        "Sniffnet" => "Network monitoring tool to analyze your internet traffic",
        "TeleGuard" => "Secure messaging app with end-to-end encryption",
        "Python 3.13" => "Python programming language",
        "Notepad++" => "Free source code editor and Notepad replacement",
        "WinSCP" => "Free SFTP, SCP, Amazon S3, WebDAV, and FTP client",
        "PuTTY" => "Free SSH and telnet client",
        "WinMerge" => "Open source differencing and merging tool",
        "Eclipse IDE for Java" => "Java IDE and development platform",
        "Microsoft Visual Studio Code" => "Code editor with support for development operations",
        "Git" => "Distributed version control system",
        "GitHub Desktop" => "GitHub desktop client",
        "Microsoft .NET Runtime 3.1" => ".NET Runtime 3.1 for running applications",
        "Microsoft .NET Runtime 5.0" => ".NET Runtime 5.0 for running applications",
        "Microsoft .NET Runtime 6.0" => ".NET Runtime 6.0 LTS for running applications",
        "Microsoft .NET Runtime 7.0" => ".NET Runtime 7.0 for running applications",
        "Microsoft .NET Runtime 8.0" => ".NET Runtime 8.0 LTS for running applications",
        ".NET Framework 4.8.1" => ".NET Framework Developer Pack",
        "DirectX Runtime" => {
            "DirectX runtime components for running games and multimedia applications"
        }
        "Java Runtime Environment" => "Java runtime environment for running Java applications",
        "Visual C++ 2005 (x86)" => "Visual C++ 2005 runtime components",
        "Visual C++ 2005 (x64)" => "Visual C++ 2005 runtime components",
        "Visual C++ 2008 (x86)" => "Visual C++ 2008 runtime components",
        "Visual C++ 2008 (x64)" => "Visual C++ 2008 runtime components",
        "Visual C++ 2010 (x86)" => "Visual C++ 2010 runtime components",
        "Visual C++ 2010 (x64)" => "Visual C++ 2010 runtime components",
        "Visual C++ 2012 (x86)" => "Visual C++ 2012 runtime components",
        "Visual C++ 2012 (x64)" => "Visual C++ 2012 runtime components",
        "Visual C++ 2013 (x86)" => "Visual C++ 2013 runtime components",
        "Visual C++ 2013 (x64)" => "Visual C++ 2013 runtime components",
        "Visual C++ 2015-2022 (x86)" => "Visual C++ 2015-2022 runtime components",
        "Visual C++ 2015-2022 (x64)" => "Visual C++ 2015-2022 runtime components",
        "Tabby" => "SSH terminal and connection manager",
        "Riot Games Launcher" => "Riot client launcher (manual login may be required)",
        "Helium Browser" => "Privacy-focused Chromium browser",
        "DataGrip" => "JetBrains SQL IDE",
        "Zed" => "High-performance code editor",
        "debloater_title" => "Debloater",
        "debloater_subtitle" => "Remove Windows components and optional features.",
        "debloater_search" => "Search packages",
        "debloater_nothing_selected" => "Nothing selected",
        "debloater_installed_status" => "Installed",
        "debloater_not_installed_status" => "Not installed",
        "debloater_installing" => "Installing selected items...",
        "debloater_removing" => "Removing selected items...",
        "debloater_all_items" => "All items",
        "debloater_installed_only" => "Installed only",
        "debloater_not_installed_only" => "Not installed only",
        "debloater_refresh" => "Refresh",
        "debloater_install_selected" => "Install Selected",
        "debloater_remove_selected" => "Remove Selected",
        "debloater_loading" => "Loading packages...",
        "debloater_scanning" => "Scanning installed apps, capabilities and optional features.",
        "debloater_none" => "No packages to display.",
        "debloater_cannot_reinstall" => "Cannot reinstall",
        "debloater_meta" => "Package: {}\nCategory: {}\nGroup: {}",
        "debloater_confirm_install_title" => "Confirm Installation",
        "debloater_confirm_remove_title" => "Confirm Removal",
        "debloater_confirm_install_desc" => "These apps will be installed:",
        "debloater_confirm_remove_desc" => "These apps will be removed:",
        "debloater_confirm_install_btn" => "Confirm Install",
        "debloater_confirm_remove_btn" => "Confirm Remove",
        "debloater_cancel" => "Cancel",
        "debloater_installed" => "Installed",
        "debloater_not_installed" => "Not installed",
        "debloater_result_install" => "Installed: {}  Failed: {}",
        "debloater_result_remove" => "Removed: {}  Failed: {}",
        "debloater_tab_0" => "Windows Apps",
        "debloater_tab_1" => "Capabilities",
        "debloater_tab_2" => "Optional Features",
        "processes_title" => "Processes",
        "processes_subtitle" => "Inspect processes, affinity and priority settings.",
        "processes_refresh" => "Refresh",
        "processes_all" => "All",
        "processes_active_only" => "Active only",
        "processes_user_only" => "User only",
        "processes_visible" => "Visible:",
        "processes_total_cpu" => "Total CPU:",
        "processes_refreshing" => "Refreshing process list...",
        "processes_waiting_first" => "Waiting for first refresh...",
        "processes_reload_queued" => "Reload queued",
        "processes_pid" => "PID",
        "processes_name" => "Name",
        "processes_cpu" => "CPU %",
        "processes_priority" => "Priority",
        "processes_affinity" => "Affinity",
        "processes_status" => "Status",
        "processes_collapse_tree" => "Collapse tree",
        "processes_expand_tree" => "Expand tree",
        "processes_cpu_priority" => "CPU Priority",
        "processes_current" => "Current",
        "processes_always" => "Always",
        "processes_io_priority" => "I/O Priority",
        "processes_affinity_menu" => "Affinity",
        "processes_open_editor" => "Open editor",
        "processes_all_cores" => "All cores",
        "processes_affinity_prefix" => "CPU",
        "processes_selected" => "Selected Process",
        "processes_realtime_title" => "Set Realtime Priority?",
        "processes_realtime_warn" => {
            "Realtime can freeze Windows responsiveness. Continue only if you understand the risk."
        }
        "processes_cancel" => "Cancel",
        "processes_confirm" => "Confirm",
        "processes_affinity_title" => "CPU Affinity - {} (PID {})",
        "processes_affinity_mask" => "Affinity bitmask (hex): {}",
        "processes_invert" => "Invert",
        "processes_clear" => "Clear",
        "processes_close" => "Close",
        "processes_apply" => "Apply",
        "processes_last_refresh" => "Last refresh: {}s ago",
        "processes_priority_unknown" => "Unknown",
        "processes_priority_idle" => "Idle",
        "processes_priority_below_normal" => "Below Normal",
        "processes_priority_normal" => "Normal",
        "processes_priority_above_normal" => "Above Normal",
        "processes_priority_high" => "High",
        "processes_priority_realtime" => "Realtime",
        "processes_priority_background" => "Background: 4 (Low I/O and CPU)",
        "processes_priority_low" => "Low",
        "processes_priority_always_below" => "Below",
        "processes_priority_always_above" => "Above",
        "processes_status_running" => "Running",
        "processes_status_sleeping" => "Sleeping",
        "processes_status_idle" => "Idle",
        "processes_status_zombie" => "Zombie",
        "processes_status_stopped" => "Stopped",
        "processes_status_tracing" => "Tracing",
        "processes_status_dead" => "Dead",
        "processes_status_wakekill" => "Wakekill",
        "processes_status_waking" => "Waking",
        "processes_status_lockblocked" => "Lock blocked",
        "processes_status_parked" => "Parked",
        "processes_status_unknown" => "Unknown",
        "processes_process_scan_failed" => "Process scan failed",
        "processes_openprocess_failed" => "OpenProcess failed for PID {}: {}",
        "processes_get_affinity_failed" => "GetProcessAffinityMask failed for PID {}: {}",
        "processes_invalid_system_mask" => "Invalid system affinity mask for PID {}",
        "processes_invalid_affinity_mode" => "Invalid affinity mode",
        "processes_set_priority_failed" => "SetPriorityClass failed for PID {}",
        "processes_set_io_failed" => "SetProcessInformation(I/O) failed for PID {}",
        "processes_set_affinity_failed" => "SetProcessAffinityMask failed for PID {}",
        "processes_perfoptions_open" => "Failed to create/open PerfOptions key",
        "processes_perfoptions_write_cpu" => "Failed to write CpuPriorityClass",
        "processes_perfoptions_write_io" => "Failed to write IoPriority",
        "processes_invalid_priority_level" => "Invalid priority level",
        "latency_title" => "Latency",
        "latency_subtitle" => "Analyze USB latency and device topology.",
        "latency_button" => "Analyze USB Latency",
        "latency_analyzing" => "Analyzing...",
        "latency_starting" => "Starting analysis...",
        "latency_topology" => "Analyzing USB topology...",
        "latency_scanning" => "Scanning PnP devices, controller chain, MSI and power settings.",
        "latency_begin" => "Click 'Analyze USB Latency' to begin analysis.",
        "latency_loading_fail" => "USB latency analysis failed",
        "latency_admin" => "Make sure you are running as Administrator.",
        "latency_error_title" => "ERROR - USB LATENCY ANALYSIS FAILED",
        "latency_failed" => "USB latency analysis failed",
        "latency_progress_power" => "Checking power settings...",
        "latency_progress_controllers" => "Scanning USB controllers...",
        "latency_progress_usb_registry_tree" => "Reading USB registry tree...",
        "latency_progress_inputs" => "Finding input devices...",
        "latency_progress_hubs" => "Tracing devices to root hubs...",
        "latency_progress_verify" => "Verifying topology and power hints...",
        "latency_progress_report" => "Building report...",
        _ => "",
    }
}
