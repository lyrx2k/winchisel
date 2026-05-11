pub fn t(key: &str) -> &'static str {
    match key {
        "nav_title" => "Navigation",
        "nav_subtitle" => "Winchisel-Kontrollzentrum",
        "home" => "Startseite",
        "debloater" => "Debloater",
        "downloads" => "Apps & Downloads",
        "performance" => "Leistung",
        "processes" => "Prozesse",
        "latency" => "Latenz",
        "privacy_security" => "Datenschutz & Sicherheit",
        "privacy_security_title" => "Datenschutz & Sicherheit",
        "privacy_security_subtitle" => {
            "Steuere Datenschutz- und Sicherheitseinstellungen an einem Ort."
        }
        "privacy_security_search" => "Datenschutzeinstellungen suchen",
        "privacy_security_quick" => "Schnell",
        "privacy_security_apply_recommended" => "Empfohlene anwenden",
        "privacy_security_reset_defaults" => "Standardwerte zurücksetzen",
        "privacy_security_current_default" => "Standard: ",
        "privacy_security_current_recommended" => "Empfohlen: ",
        "privacy_security_loading" => "Datenschutzeinstellungen werden geladen...",
        "privacy_security_placeholder_title" => "Demnächst verfügbar",
        "privacy_security_placeholder_desc" => {
            "Dieser Tab ist für zukünftige Datenschutz- und Sicherheitsfunktionen reserviert."
        }
        "privacy_uac_title" => "Benutzerkontensteuerung",
        "privacy_uac_desc" => {
            "Steuert die UAC-Benachrichtigungsstufe und das sichere Desktopverhalten"
        }
        "privacy_uac_opt_0" => "Anmeldeinformationen anfordern",
        "privacy_uac_opt_1" => "Immer benachrichtigen",
        "privacy_uac_opt_2" => "Benachrichtigen, wenn Apps Änderungen vornehmen wollen",
        "privacy_uac_opt_3" => {
            "Benachrichtigen, wenn Apps Änderungen vornehmen wollen (ohne Abdunkelung)"
        }
        "privacy_uac_opt_4" => "Nie benachrichtigen",
        "privacy_workplace_join_title" => "Arbeitsplatzbeitritts-Meldungen",
        "privacy_workplace_join_desc" => {
            "Zeigt die Windows-Aufforderungen zur Verwaltung des Geräts durch die Organisation an"
        }
        "privacy_bitlocker_title" => "BitLocker-Auto-Verschlüsselung",
        "privacy_bitlocker_desc" => {
            "Steuert, ob Windows Laufwerke automatisch mit BitLocker verschlüsseln darf"
        }
        "privacy_wifi_sense_title" => "WiFi-Sense",
        "privacy_wifi_sense_desc" => {
            "Erlaubt das Teilen von WLAN-Passwörtern mit Kontakten und das Verbinden mit vorgeschlagenen Hotspots"
        }
        "privacy_automatic_maintenance_title" => "Automatische Wartung",
        "privacy_automatic_maintenance_desc" => {
            "Legt fest, ob Windows während Leerlaufzeiten automatische Systemwartung ausführen soll"
        }
        "privacy_error_reporting_title" => "Windows-Fehlerberichterstattung",
        "privacy_error_reporting_desc" => {
            "Legt fest, ob Windows Absturzberichte und Fehlerinformationen an Microsoft senden soll"
        }
        "privacy_remote_assistance_title" => "Remoteunterstützung",
        "privacy_remote_assistance_desc" => {
            "Legt fest, ob andere Personen sich zur technischen Unterstützung remote mit dem Computer verbinden dürfen"
        }
        "privacy_smart_app_control_title" => "Smart App Control",
        "privacy_smart_app_control_desc" => {
            "Steuert die Smart-App-Control-Funktion, die nicht vertrauenswürdige Anwendungen blockiert"
        }
        "privacy_smart_app_control_opt_0" => "Aus",
        "privacy_smart_app_control_opt_1" => "Ein (erzwingend)",
        "privacy_smart_app_control_opt_2" => "Evaluierungsmodus",
        "privacy_powershell_title" => "PowerShell-Ausführungsrichtlinie",
        "privacy_powershell_desc" => {
            "Steuert, ob PowerShell-Skripte ausgeführt werden dürfen und unter welchen Bedingungen"
        }
        "privacy_powershell_opt_0" => "Eingeschränkt",
        "privacy_powershell_opt_1" => "Alle signiert",
        "privacy_powershell_opt_2" => "Remote signiert",
        "privacy_powershell_opt_3" => "Uneingeschränkt",
        "privacy_powershell_opt_4" => "Bypass",
        "privacy_developer_title" => "Entwicklermodus",
        "privacy_developer_desc" => {
            "Erlaubt die Installation von Apps aus jeder Quelle, einschließlich loser Dateien"
        }
        "privacy_lock_screen_title" => "Sperrbildschirm",
        "privacy_lock_screen_desc" => {
            "Ermöglicht das Sperren des Computers über Windows+L, das Startmenü oder den Strg+Alt+Entf-Bildschirm"
        }
        "privacy_rotating_lock_title" => "Windows Spotlight auf dem Sperrbildschirm",
        "privacy_rotating_lock_desc" => {
            "Zeigt wechselnde Windows-Spotlight-Bilder auf dem Sperrbildschirm statt eines statischen Hintergrunds an. Winhance setzt die empfohlene Startmenü-Sektion automatisch auf Anzeigen, wenn diese Einstellung aktiviert ist, da sie erforderlich ist"
        }
        "privacy_lock_screen_overlay_title" => "Fakten und Tipps auf dem Sperrbildschirm",
        "privacy_lock_screen_overlay_desc" => {
            "Zeigt Wissenswertes, Tipps und Tricks als Overlay auf dem Sperrbildschirm an"
        }
        "privacy_advertising_id_title" => {
            "Apps dürfen mir personalisierte Werbung anzeigen, indem sie meine Werbe-ID verwenden"
        }
        "privacy_advertising_id_desc" => {
            "Windows erzeugt eine eindeutige Werbe-ID, die Apps verwenden, um dein Verhalten über verschiedene Apps hinweg zu verfolgen und personalisierte Werbung anzuzeigen"
        }
        "privacy_language_list_title" => {
            "Websites dürfen mir lokal relevante Inhalte anzeigen, indem sie auf meine Sprachliste zugreifen"
        }
        "privacy_language_list_desc" => {
            "Erlaubt Websites den Zugriff auf deine Sprachpräferenzen, damit Inhalte automatisch in deiner bevorzugten Sprache angezeigt werden können, ohne jede Seite manuell konfigurieren zu müssen"
        }
        "privacy_app_launch_title" => {
            "Windows darf Start- und Suchergebnisse verbessern, indem App-Starts nachverfolgt werden"
        }
        "privacy_app_launch_desc" => {
            "Windows protokolliert, welche Apps du am häufigsten verwendest, um das Startmenü zu personalisieren und Suchergebnisse zu verbessern, damit du häufiger genutzte Apps schneller erreichst"
        }
        "privacy_settings_content_title" => {
            "Vorgeschlagene Inhalte in der Einstellungen-App anzeigen"
        }
        "privacy_settings_content_desc" => {
            "Zeigt Werbeinhalte, Tipps und Funktionsvorschläge innerhalb der Windows-Einstellungen an. Winhance setzt die empfohlene Startmenü-Sektion automatisch auf Anzeigen, wenn diese Einstellung aktiviert ist, da sie erforderlich ist"
        }
        "privacy_settings_notifications_title" => "Benachrichtigungen der Einstellungen-App",
        "privacy_settings_notifications_desc" => {
            "Zeigt Kontobenachrichtigungen in der Einstellungen-App an, darunter Aufforderungen zur erneuten Anmeldung, zum Sichern des Geräts und zum Verwalten von Abonnements"
        }
        "privacy_ads_title" => "Werbung, Vorschläge und Werbeinhalte",
        "privacy_ads_desc" => "Steuert alle Werbung, Vorschläge und Werbeinhalte in Windows",
        "privacy_ads_opt_0" => "Erlauben",
        "privacy_ads_opt_1" => "Verweigern",
        "privacy_ads_opt_2" => "Benutzerdefiniert",
        "privacy_content_delivery_title" => "Inhaltsauslieferung",
        "privacy_content_delivery_desc" => {
            "Ermöglicht Windows, Werbeinhalte bereitzustellen und vorgeschlagene Apps automatisch zu installieren"
        }
        "privacy_subscribed_content_title" => "Abonnierte Inhalte",
        "privacy_subscribed_content_desc" => {
            "Aktiviert Werbeinhalts-Abonnements von Microsoft und Partnern in Windows"
        }
        "privacy_feature_management_title" => "Funktionsverwaltung",
        "privacy_feature_management_desc" => {
            "Aktiviert die Windows-Funktionsverwaltung für Werbefunktionen und automatische App-Installationen"
        }
        "privacy_soft_landing_title" => "Soft-Landing-Erlebnisse",
        "privacy_soft_landing_desc" => {
            "Aktiviert hilfreiche und werbliche Soft-Landing-Erlebnisse in Windows"
        }
        "privacy_oem_preinstalled_title" => "Vorinstallierte OEM-Apps",
        "privacy_oem_preinstalled_desc" => {
            "Verhindert, dass OEM-Hersteller automatisch Bloatware-Apps installieren"
        }
        "privacy_preinstalled_title" => "Vorinstallierte vorgeschlagene Apps",
        "privacy_preinstalled_desc" => {
            "Verhindert, dass Microsoft vorgeschlagene Apps automatisch installiert"
        }
        "privacy_preinstalled_ever_title" => "Verlauf der vorinstallierten Apps",
        "privacy_preinstalled_ever_desc" => {
            "Deaktiviert die Nachverfolgung, ob vorinstallierte Apps jemals aktiviert waren"
        }
        "privacy_silent_installed_title" => "Stille App-Installation",
        "privacy_silent_installed_desc" => {
            "Verhindert, dass Apps im Hintergrund still installiert werden"
        }
        "privacy_speech_title" => "Online-Spracherkennung",
        "privacy_speech_desc" => {
            "Verwende deine Stimme für Apps, die Microsofts Online-Spracherkennungstechnologie nutzen"
        }
        "privacy_narrator_online_title" => "Sprachausgabe-Online-Dienste",
        "privacy_narrator_online_desc" => {
            "Erlaubt der Sprachausgabe, Microsoft-Cloud-Dienste für Funktionen wie intelligente Bildbeschreibungen und verbesserte Sprachmodelle zu verwenden"
        }
        "privacy_narrator_scripting_title" => "Sprachausgabe-Skripting-Unterstützung",
        "privacy_narrator_scripting_desc" => {
            "Erlaubt der Sprachausgabe, Skripte für Automatisierung und benutzerdefinierte Funktionen auszuführen"
        }
        "privacy_inking_title" => "Benutzerdefiniertes Stift- und Tippwörterbuch",
        "privacy_inking_desc" => {
            "Verwendet deinen Tippverlauf und deine Handschriftmuster, um ein benutzerdefiniertes Wörterbuch zu erstellen (beim Deaktivieren werden alle Wörter aus deinem benutzerdefinierten Wörterbuch gelöscht)"
        }
        "privacy_diagnostics_title" => "Diagnosedaten senden",
        "privacy_diagnostics_desc" => {
            "Sende Diagnosedaten an Microsoft, um Windows zu verbessern und sicher zu halten"
        }
        "privacy_improve_inking_title" => "Stift und Eingabe verbessern",
        "privacy_improve_inking_desc" => {
            "Sende optionale Diagnoseinformationen zu Stift- und Eingabevorgängen an Microsoft"
        }
        "privacy_tailored_experiences_title" => "Personalisierte Erlebnisse",
        "privacy_tailored_experiences_desc" => {
            "Erlaube Microsoft, deine Diagnosedaten zu verwenden, um personalisierte Tipps, Werbung und Empfehlungen anzuzeigen"
        }
        "privacy_feedback_title" => "Windows darf dich um Feedback bitten",
        "privacy_feedback_desc" => {
            "Erlaubt Windows, dich zur Abgabe von Feedback zu Funktionen in Windows aufzufordern"
        }
        "privacy_activity_history_title" => "Aktivitätsverlauf",
        "privacy_activity_history_desc" => {
            "Ermöglicht dir den schnellen Einstieg in zuletzt verwendete Apps, Dokumente oder andere Aktivitäten beim Start"
        }
        "privacy_timeline_title" => "Timeline-Vorschläge",
        "privacy_timeline_desc" => "Zeigt Vorschläge in der Windows-10-Timeline-Funktion an",
        "privacy_search_history_title" => "Suchverlauf auf diesem Gerät",
        "privacy_search_history_desc" => {
            "Verbessert Suchergebnisse, indem Windows Search deinen Suchverlauf lokal auf diesem Gerät speichert (bereits vorhandener Verlauf wird nicht gelöscht)"
        }
        "privacy_search_highlights_title" => "Such-Hervorhebungen anzeigen",
        "privacy_search_highlights_desc" => "Zeigt Inhaltsempfehlungen in der Suche an",
        "privacy_search_msa_title" => "Cloud-Inhaltssuche für Microsoft-Konto",
        "privacy_search_msa_desc" => {
            "Erlaubt Windows Search, Ergebnisse aus Apps und Diensten anzuzeigen, bei denen du mit deinem Microsoft-Konto angemeldet bist"
        }
        "privacy_search_aad_title" => "Cloud-Inhaltssuche für Arbeits- oder Schulkonto",
        "privacy_search_aad_desc" => {
            "Erlaubt Windows Search, Ergebnisse aus Apps und Diensten anzuzeigen, bei denen du mit deinem Arbeits- oder Schulkonto angemeldet bist"
        }
        "privacy_cortana_title" => "Cortana erlauben",
        "privacy_cortana_desc" => {
            "Aktiviert Microsofts virtuelle Assistentin Cortana für Sprachbefehle und Suchanfragen"
        }
        "privacy_location_title" => "Standortdienste",
        "privacy_location_desc" => {
            "Erlaubt Windows und Apps, auf den Standort des Geräts für standortbasierte Funktionen zuzugreifen"
        }
        "privacy_camera_title" => "Kamerazugriff",
        "privacy_camera_desc" => "Apps den Zugriff auf die Kamera erlauben",
        "privacy_microphone_title" => "Mikrofonzugriff",
        "privacy_microphone_desc" => "Apps den Zugriff auf das Mikrofon erlauben",
        "privacy_account_info_title" => "Zugriff auf Kontoinformationen",
        "privacy_account_info_desc" => "Apps den Zugriff auf Kontoinformationen erlauben",
        "privacy_app_diagnostic_title" => "Zugriff auf App-Diagnosedaten",
        "privacy_app_diagnostic_desc" => "Apps den Zugriff auf App-Diagnosedaten erlauben",
        "privacy_onedrive_backup_title" => "Automatische OneDrive-Sicherungen",
        "privacy_onedrive_backup_desc" => {
            "Steuert, ob OneDrive automatisch deine Ordner Dokumente, Bilder und Desktop sichert. Hat keine Wirkung, wenn OneDrive-Sicherungen auf deinem Gerät bereits aktiv sind"
        }
        "privacy_copilot_title" => "Windows Copilot",
        "privacy_copilot_desc" => {
            "Steuert, ob Windows Copilot systemweit per Gruppenrichtlinie für den aktuellen Benutzer und den lokalen Computer verfügbar ist"
        }
        "privacy_ai_data_title" => "KI-Datenanalyse",
        "privacy_ai_data_desc" => {
            "Steuert, ob Windows-KI Benutzerdaten für Personalisierung und Empfehlungen analysieren darf"
        }
        "privacy_recall_enable_title" => "Recall-Aktivierung",
        "privacy_recall_enable_desc" => {
            "Steuert, ob Windows Recall per Richtlinie aktiviert werden kann"
        }
        "privacy_recall_snapshots_title" => "Recall speichert Schnappschüsse",
        "privacy_recall_snapshots_desc" => {
            "Erlaubt Windows Recall, Screenshots deiner Aktivitäten für spätere Erinnerung zu speichern"
        }
        "privacy_click_to_do_title" => "Click to Do",
        "privacy_click_to_do_desc" => {
            "Steuert, ob die KI-Funktion Click to Do in Windows verfügbar ist"
        }
        "privacy_settings_agent_title" => "KI-Einstellungsassistent",
        "privacy_settings_agent_desc" => {
            "Steuert, ob der KI-gestützte Einstellungsassistent in Windows verfügbar ist"
        }
        "privacy_agent_connectors_title" => "KI-Agent-Connectoren",
        "privacy_agent_connectors_desc" => {
            "Steuert, ob KI-Agenten Connectoren verwenden können, um auf externe Dienste zuzugreifen"
        }
        "privacy_agent_workspaces_title" => "KI-Agent-Arbeitsbereiche",
        "privacy_agent_workspaces_desc" => {
            "Steuert, ob KI-Agent-Arbeitsbereiche in Windows verfügbar sind"
        }
        "privacy_remote_agent_connectors_title" => "Remote-KI-Agent-Connectoren",
        "privacy_remote_agent_connectors_desc" => {
            "Steuert, ob KI-Agenten Remote-Connectoren verwenden können, um auf entfernte Dienste zuzugreifen"
        }
        "privacy_copilot_key_title" => "Copilot-Hardwaretaste",
        "privacy_copilot_key_desc" => {
            "Steuert, ob die dedizierte Copilot-Taste auf Tastaturen Copilot öffnet"
        }
        "privacy_copilot_runtime_title" => "Copilot-Laufzeit",
        "privacy_copilot_runtime_desc" => {
            "Steuert, ob die Copilot-Laufzeit per Richtlinie ausgeführt werden darf"
        }
        "privacy_copilot_available_title" => "Copilot-Verfügbarkeit",
        "privacy_copilot_available_desc" => {
            "Steuert, ob Copilot in der Windows-Shell verfügbar ist"
        }
        "privacy_bing_chat_title" => "Bing-Chat-Berechtigung",
        "privacy_bing_chat_desc" => {
            "Steuert, ob der Benutzer für Bing Chat und Copilot in der Suche berechtigt ist"
        }
        "privacy_generative_ai_title" => "Zugriff auf generative KI",
        "privacy_generative_ai_desc" => {
            "Steuert, ob Apps auf die generative KI-Funktion auf deinem Gerät zugreifen können"
        }
        "privacy_system_ai_title" => "Zugriff auf System-KI-Modelle",
        "privacy_system_ai_desc" => {
            "Steuert, ob Apps auf System-KI-Modelle auf deinem Gerät zugreifen und Nutzungsdaten erfassen können"
        }
        "privacy_copilot_microphone_title" => "Copilot-Mikrofonzugriff",
        "privacy_copilot_microphone_desc" => {
            "Steuert, ob Copilot und Office-Hub-Apps Mikrofonberechtigungen haben"
        }
        "privacy_paint_image_creator_title" => "Paint KI-Bildersteller",
        "privacy_paint_image_creator_desc" => {
            "Steuert, ob die Funktion KI-Bildersteller in Microsoft Paint verfügbar ist"
        }
        "privacy_paint_cocreator_title" => "Paint KI-Cocreator",
        "privacy_paint_cocreator_desc" => {
            "Steuert, ob die Funktion KI-Cocreator in Microsoft Paint verfügbar ist"
        }
        "privacy_paint_fill_title" => "Paint Generatives Füllen",
        "privacy_paint_fill_desc" => {
            "Steuert, ob die Funktion KI-Generatives Füllen in Microsoft Paint verfügbar ist"
        }
        "privacy_paint_erase_title" => "Paint Generatives Löschen",
        "privacy_paint_erase_desc" => {
            "Steuert, ob die Funktion KI-Generatives Löschen in Microsoft Paint verfügbar ist"
        }
        "privacy_paint_background_title" => "Paint Hintergrund entfernen",
        "privacy_paint_background_desc" => {
            "Steuert, ob die Funktion KI-Hintergrund entfernen in Microsoft Paint verfügbar ist"
        }
        "privacy_input_insights_title" => "Eingabe-Einblicke",
        "privacy_input_insights_desc" => {
            "Steuert, ob Windows Input Insights Tippmuster verfolgen und Vorschläge bereitstellen kann"
        }
        "privacy_copilot_nudges_title" => "Copilot-Hinweise",
        "privacy_copilot_nudges_desc" => {
            "Steuert, ob Copilot-Werbehinweise und Benachrichtigungen über Hintergrundaufgaben angezeigt werden"
        }
        "privacy_consumer_ai_title" => "KI-Verbraucherinhalt",
        "privacy_consumer_ai_desc" => {
            "Steuert, ob KI-gesteuerte Inhaltsempfehlungen für Verbraucher-Konten angezeigt werden"
        }
        "privacy_edge_cdp_title" => "Edge Copilot CDP-Seitenkontext",
        "privacy_edge_cdp_desc" => {
            "Steuert, ob Copilot CDP verwenden kann, um auf Seiteninhalte in Microsoft Edge zuzugreifen"
        }
        "privacy_edge_page_title" => "Edge Copilot-Seitenkontext",
        "privacy_edge_page_desc" => {
            "Steuert, ob Copilot Seiteninhalte in Microsoft Edge lesen kann"
        }
        "privacy_edge_sidebar_title" => "Edge Copilot-Seitenleiste",
        "privacy_edge_sidebar_desc" => {
            "Steuert, ob die Copilot-Seitenleiste in Microsoft Edge verfügbar ist"
        }
        "privacy_edge_entra_title" => "Edge Entra Copilot-Seitenkontext",
        "privacy_edge_entra_desc" => {
            "Steuert, ob Entra Copilot auf Seitenkontext in Microsoft Edge zugreifen kann"
        }
        "privacy_edge_m365_icon_title" => "Edge M365 Copilot-Chat-Symbol",
        "privacy_edge_m365_icon_desc" => {
            "Steuert, ob das Microsoft 365 Copilot-Chat-Symbol in Microsoft Edge angezeigt wird"
        }
        "privacy_edge_history_title" => "Edge KI-Verlaufssuche",
        "privacy_edge_history_desc" => {
            "Steuert, ob KI-gestützte Verlaufssuche in Microsoft Edge verfügbar ist"
        }
        "privacy_edge_inline_title" => "Edge KI-Inline-Verfassen",
        "privacy_edge_inline_desc" => {
            "Steuert, ob KI-gestützte Inline-Vorschläge zum Verfassen in Microsoft Edge verfügbar sind"
        }
        "privacy_edge_local_model_title" => "Edge lokale KI-Modell-Einstellungen",
        "privacy_edge_local_model_desc" => {
            "Steuert, ob lokale KI-Modell-Einstellungen in Microsoft Edge verfügbar sind"
        }
        "privacy_edge_builtin_title" => "Edge integrierte KI-APIs",
        "privacy_edge_builtin_desc" => {
            "Steuert, ob integrierte KI-APIs für Websites in Microsoft Edge verfügbar sind"
        }
        "privacy_edge_themes_title" => "Edge KI-generierte Designs",
        "privacy_edge_themes_desc" => {
            "Steuert, ob KI-generierte Designs in Microsoft Edge verfügbar sind"
        }
        "privacy_edge_devtools_title" => "Edge DevTools KI",
        "privacy_edge_devtools_desc" => {
            "Steuert, ob KI-Funktionen in den Edge DevTools verfügbar sind"
        }
        "privacy_edge_share_history_title" => "Edge Verlauf mit Copilot teilen",
        "privacy_edge_share_history_desc" => {
            "Steuert, ob der Browserverlauf mit der Copilot-Suche in Microsoft Edge geteilt wird"
        }
        "privacy_office_training_title" => "Office KI-Training",
        "privacy_office_training_desc" => {
            "Steuert, ob Office KI-Trainingsdaten aus deiner Nutzung sammelt"
        }
        "privacy_office_connected_title" => "Office verbundene Dienste",
        "privacy_office_connected_desc" => {
            "Steuert, ob verbundene Erlebnisse und KI-gestützte Dienste in Office verfügbar sind"
        }
        "privacy_word_copilot_title" => "Word Copilot",
        "privacy_word_copilot_desc" => {
            "Steuert, ob KI-Copilot-Funktionen in Microsoft Word verfügbar sind"
        }
        "privacy_excel_copilot_title" => "Excel Copilot",
        "privacy_excel_copilot_desc" => {
            "Steuert, ob KI-Copilot-Funktionen in Microsoft Excel verfügbar sind"
        }
        "privacy_onenote_copilot_title" => "OneNote Copilot",
        "privacy_onenote_copilot_desc" => {
            "Steuert, ob KI-Copilot-Funktionen, Copilot-Notizbücher und Copilot-Skittle in Microsoft OneNote verfügbar sind"
        }
        "privacy_office_safety_title" => "Office KI-Inhaltssicherheit",
        "privacy_office_safety_desc" => {
            "Steuert, ob KI-Inhaltssicherheitsfunktionen für Alternativtext, Umschreiben und Zusammenfassung in Office-Apps verfügbar sind"
        }
        "privacy_security_group_0" => "Sicherheit",
        "privacy_security_group_0_desc" => "Sicherheitsbezogene Windows-Datenschutzfunktionen.",
        "privacy_security_group_1" => "Inhaltsauslieferung & Werbung",
        "privacy_security_group_1_desc" => "Werbung, Vorschläge und Werbeinhalte.",
        "privacy_security_group_2" => "Sperrbildschirm",
        "privacy_security_group_2_desc" => "Sperrbildschirmverhalten und Spotlight-Inhalte.",
        "privacy_security_group_3" => "Allgemein",
        "privacy_security_group_3_desc" => "Allgemeine Windows-Datenschutzeinstellungen.",
        "privacy_security_group_4" => "Sprache",
        "privacy_security_group_4_desc" => "Spracherkennung und Sprachausgabe-Einstellungen.",
        "privacy_security_group_5" => "Stift- und Eingabepersonalisierung",
        "privacy_security_group_5_desc" => {
            "Tipp-Personalisierung und benutzerdefinierte Wörterbücher."
        }
        "privacy_security_group_6" => "Diagnose & Feedback",
        "privacy_security_group_6_desc" => "Telemetrie, Feedback und Diagnosedatenerfassung.",
        "privacy_security_group_7" => "Aktivitätsverlauf",
        "privacy_security_group_7_desc" => "Aktivitätsverlauf und Timeline-Vorschläge.",
        "privacy_security_group_8" => "Suchberechtigungen",
        "privacy_security_group_8_desc" => "Suchverlauf, Cloud-Suche und Cortana.",
        "privacy_security_group_9" => "App-Berechtigungen",
        "privacy_security_group_9_desc" => "Standort-, Kamera-, Mikrofon- und App-Zugriffe.",
        "privacy_security_group_10" => "Windows-KI",
        "privacy_security_group_10_desc" => "Windows-KI und Copilot-bezogene Einstellungen.",
        "privacy_security_group_11" => "Microsoft Edge KI",
        "privacy_security_group_11_desc" => "Microsoft-Edge-KI-Einstellungen.",
        "privacy_security_group_12" => "Microsoft Office KI",
        "privacy_security_group_12_desc" => "Microsoft-Office-KI-Einstellungen.",
        "privacy_option_on" => "Ein",
        "privacy_option_off" => "Aus",
        "settings" => "Einstellungen",
        "extras" => "Extras",
        "extras_title" => "Extras",
        "extras_subtitle" => "Reserviert für zusätzliche Tools und Utilities",
        "extras_loading" => "Extras werden geladen...",
        "extras_brave_label" => "Brave Browser - Debloat",
        "extras_brave_desc" => "Brave-Policy-Debloat ein- oder ausschalten.",
        "extras_brave_enabled_desc" => "Die Brave-Debloat-Policies sind aktiviert.",
        "extras_brave_disabled_desc" => "Die Brave-Debloat-Policies sind deaktiviert.",
        "extras_edge_label" => "Microsoft Edge - Debloat",
        "extras_edge_desc" => "Microsoft-Edge-Policy-Debloat ein- oder ausschalten.",
        "extras_widgets_label" => "Widgets - Entfernen",
        "extras_widgets_desc" => {
            "Das Widgets-Paket aus der Taskleiste entfernen und bei Bedarf wiederherstellen."
        }
        "extras_ctfmon_label" => "CTFMON-Interception stoppen",
        "extras_ctfmon_desc" => "Die Input-Service-Werte deaktivieren, die CTFMON abfangen können.",
        "extras_ctfmon_details_label" => "TextInputManagementService patchen",
        "extras_ctfmon_details_desc" => "ServiceDll zwischen TabSvc.dll und MSCTF.DLL umschalten.",
        "extras_timer_resolution_label" => "Windows 11 Timer-Auflösung",
        "extras_timer_resolution_desc" => {
            "Globale Timer-Resolution-Requests für Spiele und Apps wiederherstellen."
        }
        "extras_ipv6_label" => "IPv6 - IPv4 bevorzugen",
        "extras_ipv6_desc" => {
            "IPv4 auf Netzwerken bevorzugen, auf denen IPv6 nicht konfiguriert ist."
        }
        "extras_teredo_label" => "Teredo - Deaktivieren",
        "extras_teredo_desc" => {
            "Teredo-Tunneling deaktivieren, um auf manchen Netzwerken Latenz zu senken."
        }
        "extras_ps7_label" => "PowerShell 7 Telemetry - Deaktivieren",
        "extras_ps7_desc" => "PowerShell 7 Telemetrie auf diesem System deaktivieren.",
        "extras_hpet_label" => "HPET deaktivieren",
        "extras_hpet_desc" => {
            "HPET deaktivieren für bessere Leistung in manchen Anti-Cheats und Spielen."
        }
        "extras_brave_on_applied" => "Brave Debloat aktiviert.",
        "extras_brave_off_applied" => "Brave Debloat deaktiviert.",
        "check_updates" => "Updates prüfen",
        "settings_title" => "Einstellungen",
        "settings_subtitle" => "Das App-Verhalten an deinen Arbeitsablauf anpassen.",
        "settings_application" => "Anwendung",
        "settings_saved_auto" => "Diese Einstellungen werden automatisch gespeichert.",
        "settings_disk_cleanup" => "Datenträgerbereinigung - Starten",
        "settings_disk_cleanup_desc" => {
            "Startet die Datenträgerbereinigung auf Laufwerk C: und entfernt alte Windows-Updates."
        }
        "settings_disk_cleanup_run" => "Starten",
        "settings_temp_files" => "Temporäre Dateien - Entfernen",
        "settings_temp_files_desc" => "Leert die TEMP-Ordner.",
        "settings_temp_files_run" => "Entfernen",
        "settings_close" => "Schließen",
        "settings_disk_cleanup_success" => "Datenträgerbereinigung erfolgreich abgeschlossen.",
        "settings_disk_cleanup_failed" => "Datenträgerbereinigung fehlgeschlagen.",
        "settings_disk_cleanup_failed_windows" => {
            "Datenträgerbereinigung wird nur unter Windows unterstützt."
        }
        "settings_temp_files_success" => "Temporäre Dateien wurden erfolgreich entfernt.",
        "settings_temp_files_failed" => "Das Entfernen temporärer Dateien ist fehlgeschlagen.",
        "settings_temp_files_failed_windows" => {
            "Das Entfernen temporärer Dateien wird nur unter Windows unterstützt."
        }
        "language" => "Sprache",
        "sidebar_languages" => "Sprachen",
        "archive-restore" => "Wiederherstellen",
        "settings-2" => "Einstellungen",
        "shield-check" => "Sicherheitsstatus",
        "refresh-cw" => "Aktualisieren",
        "monitor" => "Konsole",
        "Winchisel-Updater" => "Winchisel-Updater",
        "Disabled (Recommended)" => "Deaktiviert (empfohlen)",
        "0" => "0",
        "1" => "1",
        "bug_report" => "Fehler melden",
        "donate" => "Spenden",
        "status_admin" => "Administrator",
        "status_standard" => "Standard",
        "open_logs" => "Logs öffnen",
        "check_updates_startup" => "Beim Start auf Updates prüfen",
        "show_console" => "Konsole anzeigen",
        "system_protection" => "Systemschutz",
        "restore_point" => "Systemwiederherstellungspunkt",
        "restore_point_desc" => {
            "Erstelle vor größeren Systemänderungen einen Wiederherstellungspunkt"
        }
        "create_restore_point" => "Wiederherstellungspunkt erstellen",
        "repair_window" => "Systemreparatur",
        "repair_desc" => {
            "Führt DISM und SFC aus, um Windows-Komponenten auf Beschädigungen zu prüfen und zu reparieren"
        }
        "repair_run" => "Reparatur starten",
        "repair_running" => "Reparatur läuft...",
        "repair_running_desc" => {
            "Das kann eine Weile dauern. Das Fenster offen lassen, bis der Vorgang abgeschlossen ist."
        }
        "repair_log_title" => "Live-Log",
        "repair_log_waiting" => "Warte auf Ausgabe...",
        "repair_stage_dism" => "DISM /Online /Cleanup-Image /RestoreHealth wird ausgeführt",
        "repair_stage_sfc" => "sfc /scannow wird ausgeführt",
        "repair_success" => "Systemreparatur erfolgreich abgeschlossen.",
        "repair_failed" => "Systemreparatur fehlgeschlagen",
        "repair_failed_run" => "Die Reparatur konnte nicht gestartet werden.",
        "repair_failed_windows" => "Die Systemreparatur wird nur unter Windows unterstützt.",
        "performance_title" => "Leistung",
        "performance_subtitle" => "Gaming- und Leistungsoptimierungen",
        "performance_search" => "Leistungsoptimierungen suchen...",
        "performance_quick" => "Schnellaktionen",
        "performance_apply_recommended" => "Empfohlene Einstellungen anwenden",
        "performance_reset_defaults" => "Auf Windows-Standard zurücksetzen",
        "performance_loading" => "Leistungsoptimierungen werden geladen...",
        "performance_empty" => "Keine Leistungsoptimierungen vorhanden.",
        "performance_current_default" => "Standard: ",
        "performance_current_recommended" => "Empfohlen: ",
        "update_available_prefix" => "Update verfügbar:",
        "update_failed_prefix" => "Update-Prüfung fehlgeschlagen:",
        "update_newer_version_prefix" => "Eine neuere Version ist verfügbar:",
        "update_ready" => "Bereit",
        "update_checked" => "Du bist bereits auf der neuesten Version.",
        "settings_saved" => "Einstellungen gespeichert",
        "update_no_found_title" => "Kein Update gefunden",
        "update_available_title" => "Update verfügbar",
        "update_failed_title" => "Update-Prüfung fehlgeschlagen",
        "update_up_to_date" => "Du bist bereits auf der neuesten Version.",
        "update_error_check_updates" => "Update-Prüfung fehlgeschlagen",
        "update_error_read_response" => "Antwort konnte nicht gelesen werden",
        "update_error_parse_json" => "JSON konnte nicht geparst werden",
        "update_error_no_tag_name" => "Kein tag_name in der Antwort",
        "update_error_download" => "Update konnte nicht heruntergeladen werden",
        "update_error_create_temp_file" => "Temporäre Datei konnte nicht erstellt werden",
        "update_error_write_update_file" => "Update-Datei konnte nicht geschrieben werden",
        "update_error_download_too_small" => "Heruntergeladene Datei ist zu klein",
        "update_error_resolve_current_exe" => {
            "Aktuelle ausführbare Datei konnte nicht aufgelöst werden"
        }
        "update_error_write_update_script" => "Update-Skript konnte nicht geschrieben werden",
        "update_error_launch_updater" => "Updater konnte nicht gestartet werden",
        "update_download_restart" => {
            "Lade die App herunter und starte sie neu, um das Update zu installieren."
        }
        "update_failed" => "Update-Prüfung fehlgeschlagen.",
        "update_close" => "Schließen",
        "update_check_again" => "Erneut prüfen",
        "update_download_restart_btn" => "Herunterladen & neu starten",
        "restore_point_window" => "Wiederherstellungspunkt erstellen",
        "restore_point_creating" => {
            "Wiederherstellungspunkt wird erstellt. Das kann einen Moment dauern..."
        }
        "restore_point_success" => "Wiederherstellungspunkt erfolgreich erstellt.",
        "restore_point_failed" => "Wiederherstellungspunkt fehlgeschlagen",
        "restore_point_failed_run" => "PowerShell konnte nicht ausgeführt werden.",
        "restore_point_failed_windows" => {
            "Wiederherstellungspunkte werden nur unter Windows unterstützt."
        }
        "download_title" => "Apps & Downloads",
        "download_subtitle" => "Installiere nützliche Apps aus kuratierten Listen.",
        "download_search" => "Apps suchen...",
        "download_website" => "Webseite",
        "download_installed" => "Installiert",
        "download_not_installed" => "Nicht installiert",
        "download_nothing_selected" => "Nichts ausgewählt",
        "download_installing" => "Ausgewählte Downloads werden installiert...",
        "download_install_job_failed" => "Installationsauftrag fehlgeschlagen",
        "download_meta" => "Winget-IDs: {}\nKategorie: {}\nWebseite: {}",
        "download_confirm_install" => "Installation bestätigen",
        "download_cancel" => "Abbrechen",
        "download_confirm_title" => "App-Installation bestätigen",
        "download_confirm_desc" => "Diese Apps und Downloads werden installiert:",
        "download_result" => "Installiert: {}  Fehlgeschlagen: {}",
        "download_refresh" => "Aktualisieren",
        "download_install_selected" => "Ausgewählte installieren",
        "download_loading" => "Apps & Downloads werden geladen...",
        "download_none" => "Keine Apps oder Downloads anzuzeigen.",
        "debloater_title" => "Debloater",
        "debloater_subtitle" => "Windows-Komponenten und optionale Features entfernen.",
        "debloater_search" => "Pakete suchen",
        "debloater_nothing_selected" => "Nichts ausgewählt",
        "debloater_installed_status" => "Installiert",
        "debloater_not_installed_status" => "Nicht installiert",
        "debloater_installing" => "Ausgewählte Elemente werden installiert...",
        "debloater_removing" => "Ausgewählte Elemente werden entfernt...",
        "debloater_all_items" => "Alle Elemente",
        "debloater_installed_only" => "Nur installierte",
        "debloater_not_installed_only" => "Nur nicht installierte",
        "debloater_refresh" => "Aktualisieren",
        "debloater_install_selected" => "Ausgewählte installieren",
        "debloater_remove_selected" => "Ausgewählte entfernen",
        "debloater_loading" => "Pakete werden geladen...",
        "debloater_scanning" => {
            "Installierte Apps, Fähigkeiten und optionale Features werden gescannt."
        }
        "debloater_none" => "Keine Pakete anzuzeigen.",
        "debloater_cannot_reinstall" => "Neuinstallation nicht möglich",
        "debloater_meta" => "Paket: {}\nKategorie: {}\nGruppe: {}",
        "debloater_confirm_install_title" => "Installation bestätigen",
        "debloater_confirm_remove_title" => "Entfernung bestätigen",
        "debloater_confirm_install_desc" => "Diese Apps werden installiert:",
        "debloater_confirm_remove_desc" => "Diese Apps werden entfernt:",
        "debloater_confirm_install_btn" => "Installation bestätigen",
        "debloater_confirm_remove_btn" => "Entfernung bestätigen",
        "debloater_cancel" => "Abbrechen",
        "debloater_installed" => "Installiert",
        "debloater_not_installed" => "Nicht installiert",
        "debloater_result_install" => "Installiert: {}  Fehlgeschlagen: {}",
        "debloater_result_remove" => "Entfernt: {}  Fehlgeschlagen: {}",
        "debloater_tab_0" => "Windows-Apps",
        "debloater_tab_1" => "Funktionen",
        "debloater_tab_2" => "Optionale Features",
        "processes_title" => "Prozesse",
        "processes_subtitle" => "Prozesse, Affinität und Prioritätseinstellungen prüfen.",
        "processes_refresh" => "Aktualisieren",
        "processes_all" => "Alle",
        "processes_active_only" => "Nur aktive",
        "processes_user_only" => "Nur Benutzer",
        "processes_visible" => "Sichtbar:",
        "processes_total_cpu" => "CPU gesamt:",
        "processes_refreshing" => "Prozessliste wird aktualisiert...",
        "processes_waiting_first" => "Warte auf erste Aktualisierung...",
        "processes_reload_queued" => "Neu laden eingeplant",
        "processes_pid" => "PID",
        "processes_name" => "Name",
        "processes_cpu" => "CPU %",
        "processes_priority" => "Priorität",
        "processes_affinity" => "Affinität",
        "processes_status" => "Status",
        "processes_collapse_tree" => "Baum einklappen",
        "processes_expand_tree" => "Baum ausklappen",
        "processes_cpu_priority" => "CPU-Priorität",
        "processes_current" => "Aktuell",
        "processes_always" => "Immer",
        "processes_io_priority" => "I/O-Priorität",
        "processes_affinity_menu" => "Affinität",
        "processes_open_editor" => "Editor öffnen",
        "processes_all_cores" => "Alle Kerne",
        "processes_affinity_prefix" => "CPU",
        "processes_selected" => "Ausgewählter Prozess",
        "processes_realtime_title" => "Echtzeitpriorität setzen?",
        "processes_realtime_warn" => {
            "Echtzeit kann die Reaktionsfähigkeit von Windows einfrieren. Nur fortfahren, wenn du das Risiko verstehst."
        }
        "processes_cancel" => "Abbrechen",
        "processes_confirm" => "Bestätigen",
        "processes_affinity_title" => "CPU-Affinität - {} (PID {})",
        "processes_affinity_mask" => "Affinitäts-Maske (hex): {}",
        "processes_invert" => "Invertieren",
        "processes_clear" => "Leeren",
        "processes_close" => "Schließen",
        "processes_apply" => "Anwenden",
        "processes_last_refresh" => "Letzte Aktualisierung: vor {}s",
        "processes_priority_unknown" => "Unbekannt",
        "processes_priority_idle" => "Leerlauf",
        "processes_priority_below_normal" => "Unter Normal",
        "processes_priority_normal" => "Normal",
        "processes_priority_above_normal" => "Über Normal",
        "processes_priority_high" => "Hoch",
        "processes_priority_realtime" => "Echtzeit",
        "processes_priority_background" => "Hintergrund: 4 (Niedrige I/O und CPU)",
        "processes_priority_low" => "Niedrig",
        "processes_priority_always_below" => "Unter",
        "processes_priority_always_above" => "Über",
        "processes_status_running" => "Läuft",
        "processes_status_sleeping" => "Schlafend",
        "processes_status_idle" => "Leerlauf",
        "processes_status_zombie" => "Zombie",
        "processes_status_stopped" => "Gestoppt",
        "processes_status_tracing" => "Tracing",
        "processes_status_dead" => "Tot",
        "processes_status_wakekill" => "Wakekill",
        "processes_status_waking" => "Wachend",
        "processes_status_lockblocked" => "Gesperrt",
        "processes_status_parked" => "Geparkt",
        "processes_status_unknown" => "Unbekannt",
        "processes_process_scan_failed" => "Prozessscan fehlgeschlagen",
        "processes_openprocess_failed" => "OpenProcess für PID {} fehlgeschlagen: {}",
        "processes_get_affinity_failed" => "GetProcessAffinityMask für PID {} fehlgeschlagen: {}",
        "processes_invalid_system_mask" => "Ungültige System-Affinitätsmaske für PID {}",
        "processes_invalid_affinity_mode" => "Ungültiger Affinitätsmodus",
        "processes_set_priority_failed" => "SetPriorityClass für PID {} fehlgeschlagen",
        "processes_set_io_failed" => "SetProcessInformation(I/O) für PID {} fehlgeschlagen",
        "processes_set_affinity_failed" => "SetProcessAffinityMask für PID {} fehlgeschlagen",
        "processes_perfoptions_open" => {
            "PerfOptions-Schlüssel konnte nicht erstellt/geöffnet werden"
        }
        "processes_perfoptions_write_cpu" => "CpuPriorityClass konnte nicht geschrieben werden",
        "processes_perfoptions_write_io" => "IoPriority konnte nicht geschrieben werden",
        "processes_invalid_priority_level" => "Ungültige Prioritätsstufe",
        "latency_title" => "Latenz",
        "latency_subtitle" => "USB-Latenz und Gerätetopologie analysieren.",
        "latency_button" => "USB-Latenz analysieren",
        "latency_analyzing" => "Analyse läuft...",
        "latency_starting" => "Analyse wird gestartet...",
        "latency_topology" => "USB-Topologie wird analysiert...",
        "latency_scanning" => {
            "PnP-Geräte, Controller-Kette, MSI und Energieeinstellungen werden gescannt."
        }
        "latency_begin" => "Klicke auf 'USB-Latenz analysieren', um zu beginnen.",
        "latency_loading_fail" => "USB-Latenzanalyse fehlgeschlagen",
        "latency_admin" => "Stelle sicher, dass du als Administrator ausgeführt wirst.",
        "latency_error_title" => "FEHLER - USB-LATENZANALYSE FEHLGESCHLAGEN",
        "latency_failed" => "USB-Latenzanalyse fehlgeschlagen",
        "latency_progress_power" => "Energieeinstellungen werden geprüft...",
        "latency_progress_controllers" => "USB-Controller werden gescannt...",
        "latency_progress_usb_registry_tree" => "USB-Registrierungsbaum wird gelesen...",
        "latency_progress_inputs" => "Eingabegeräte werden gesucht...",
        "latency_progress_hubs" => "Geräte bis zu Root-Hubs werden verfolgt...",
        "gaming-game-mode" => {
            "Dein PC fürs Gaming optimieren, indem Hintergrundaufgaben deaktiviert werden"
        }
        "gaming-performance-explorer-mouse-precision" => {
            "Zeigergeschwindigkeit abhängig von der Bewegungsgeschwindigkeit anpassen (Mausbeschleunigung). Die meisten kompetitiven Spieler deaktivieren dies für ein konsistentes Zielen in FPS-Spielen"
        }
        "gaming-performance-mouse-hover-time" => {
            "Steuert, wie lange du über ein Element fahren musst, bevor es aktiv wird (in Millisekunden). Niedrigere Werte lassen Tooltips, Menüs und Hover-Effekte schneller erscheinen. Der Standardwert ist 400 ms"
        }
        "gaming-performance-autostart-delay" => {
            "Verzögert das Starten von Anwendungen um 10 Sekunden nach dem Systemstart, um die anfängliche Systemreaktivität zu verbessern. Windows fühlt sich schneller an, aber Startprogramme brauchen länger zum Laden"
        }
        "gaming-background-apps" => {
            "Verwende Gruppenrichtlinien, um zu steuern, ob Apps im Hintergrund ausgeführt werden dürfen. 'Force Deny' entfernt app-spezifische Hintergrundeinstellungen aus den Windows-Einstellungen. 'User in Control' ist sinnvoll, wenn du Apps wie Teams, Zoom oder WhatsApp brauchst"
        }
        "gaming-storage-sense" => {
            "Gibt automatisch Speicherplatz frei, indem temporäre Dateien entfernt, der Papierkorb geleert und Downloads verwaltet werden"
        }
        "gaming-performance-explorer-search" => {
            "Durchsucht dein gesamtes Dateisystem statt nur indizierter Speicherorte. Das liefert vollständigere Ergebnisse, ist aber deutlich langsamer als die indizierte Suche und erhöht die Datenträgeraktivität"
        }
        "gaming-performance-search-webview2" => {
            "Erlaubt Windows Search, WebView2 (Edge) zum Rendern von Suchergebnissen zu verwenden. Das Deaktivieren entfernt Edge-Prozesse, die von SearchHost.exe erstellt werden, und reduziert so die Ressourcennutzung. Verwendet ein undokumentiertes Windows-Feature-Management-Override (Feature ID 37926450), das sich in zukünftigen Windows-Updates ändern kann"
        }
        "gaming-performance-wallpaper-compression" => {
            "Erlaubt Windows, Hintergrundbilder zu komprimieren, um Speicherplatz zu sparen und die Leistung zu verbessern. Betrifft nur Bilder im JPEG-Format."
        }
        "gaming-performance-explorer-menu-show-delay" => {
            "Fügt eine kurze Verzögerung hinzu, bevor Menüs erscheinen (400 ms - Windows-Standard) oder zeigt sie sofort (0 ms) an, um die Navigation zu beschleunigen"
        }
        "gaming-explorer-alt-tab-filter" => {
            "Alt+Tab zeigt nur traditionelle offene Fenster statt auch Microsoft-Edge-Tabs und andere Windows-Vorschläge"
        }
        "gaming-win32-priority" => {
            "Konfiguriert, wie Windows die CPU-Zeit zwischen Vordergrundanwendungen und Hintergrunddiensten aufteilt"
        }
        "gaming-system-responsiveness" => {
            "Verringert Störungen durch Hintergrundaufgaben, indem aktiven Spielen oder Multimedia-Anwendungen mehr CPU-Zeit zugewiesen wird"
        }
        "gaming-cpu-priority" => {
            "Gibt Spielen eine höhere CPU-Scheduling-Priorität, damit mehr Prozessorzeit bereitsteht"
        }
        "gaming-scheduling-category" => {
            "Weist eine hohe Prioritätskategorie zu, damit Spiele bevorzugt Systemressourcen erhalten"
        }
        "gaming-performance-svchost-split-threshold" => {
            "Legt den Speichergrenzwert fest, ab dem Windows Dienste in separate svchost.exe-Prozesse aufteilt. Höhere Werte gruppieren mehr Dienste zusammen und reduzieren die Anzahl der Prozesse. Wähle den Wert passend zu deinem RAM"
        }
        "gaming-gpu-priority" => {
            "Gibt Spielen eine höhere GPU-Scheduling-Priorität, um die Grafikleistung und Bildrate zu verbessern"
        }
        "gaming-gpu-scheduling" => {
            "Lässt die GPU ihren Speicher und ihr Scheduling selbst verwalten, um Latenz zu reduzieren und die Leistung zu verbessern"
        }
        "gaming-directx-flip-model" => {
            "Reduziert Latenz und nutzt erweiterte Funktionen in kompatiblen Spielen durch das DirectX-Flip-Präsentationsmodell"
        }
        "gaming-directx-vrr-optimizations" => {
            "Aktiviert VRR-Optimierungen (G-Sync/FreeSync) für flüssigeres Gameplay. Erfordert einen VRR-kompatiblen Monitor; diese Einstellung hat keine Wirkung, wenn dein Monitor kein VRR unterstützt"
        }
        "gaming-directx-auto-hdr" => {
            "Konvertiert SDR-Inhalte automatisch in HDR, um Farben und Helligkeit zu verbessern. Erfordert ein HDR-fähiges Display mit aktivem HDR; keine Wirkung, wenn dein Display HDR nicht unterstützt"
        }
        "gaming-nvidia-sharpening" => {
            "Aktiviert den klassischen NVIDIA-Schärfefilter für eine bessere Bildschärfe. Funktioniert nur mit älteren NVIDIA-Treibern; neuere Treiber sollten stattdessen die NVIDIA-Systemsteuerung verwenden"
        }
        "gaming-fullscreen-optimizations" => {
            "Erlaubt Windows, Spiele im Vollbildmodus zu optimieren. Das Deaktivieren kann Leistungsprobleme oder Ruckler in älteren Spielen beheben, die mit randlosem Vollbild nicht gut funktionieren"
        }
        "gaming-performance-desktop-composition" => {
            "Aktiviert visuelle Effekte, die vom Desktop Window Manager verwaltet werden. Das Deaktivieren kann auf älterer Hardware kleine Leistungsgewinne bringen, beeinflusst aber Aero-Effekte"
        }
        "gaming-auto-color-management" => {
            "Erlaubt Windows, Farbprofile für alle angeschlossenen unterstützten Displays automatisch zu verwalten"
        }
        "gaming-disable-mpo" => {
            "Mehrere Anzeigeebenen hardwareseitig über die GPU zusammensetzen. Das Deaktivieren kann Bildschirmflackern, schwarze Bildschirme und Ruckler in Multi-Monitor-Konfigurationen beheben"
        }
        "gaming-disable-mpo-min-fps" => {
            "Erlaubt dem Desktop Window Manager, Apps dynamisch zwischen Overlay-Modi basierend auf der Bildrate umzuschalten. Das Deaktivieren kann Ruckler in Browsern und Discord beheben, ohne MPO komplett zu deaktivieren"
        }
        "gaming-network-throttling" => {
            "Begrenzt die Netzwerkpaketrate für Multimedia-Anwendungen. Es wird empfohlen, das Throttling aktiviert zu lassen (Standard: 10 Pakete/ms), da es eine bessere DPC-Latenz fürs Gaming liefert als das vollständige Deaktivieren"
        }
        "gaming-nagle-algorithm" => {
            "Puffert kleine Netzwerkpakete vor dem Senden, um Overhead zu reduzieren. Deaktiviere es, um die Latenz beim Online-Gaming zu reduzieren, oder lasse es für eine bessere Netzwerkeffizienz aktiviert"
        }
        "gaming-dns-server" => {
            "Wähle einen DNS-Server für alle Netzwerkadapter. Änderungen gelten für jeden Adapter in deinem System (WLAN und Ethernet). Verwende 'Automatisch', um den Standard-DNS deines ISP/Routers wiederherzustellen"
        }
        "gaming-virtualization-based-security" => {
            "Isoliert Teile des Speichers, um das System vor Schwachstellen zu schützen. Das Deaktivieren kann die Gaming-Leistung verbessern, reduziert aber die Systemsicherheit"
        }
        "gaming-memory-integrity" => {
            "Verhindert das Einschleusen von schädlichem Code in hochsichere Prozesse. Das Deaktivieren kann die Gaming-Leistung verbessern, reduziert aber die Systemsicherheit"
        }
        "gaming-xbox-game-dvr" => {
            "Spielclips aufnehmen und Screenshots mit dem Xbox Game Bar-Overlay erstellen. Das Deaktivieren reduziert CPU-/GPU-Nutzung und kann die Bildrate verbessern"
        }
        "gaming-game-bar-controller" => {
            "Erlaubt deinem Xbox- oder kompatiblen Controller, die Game Bar über die Xbox-Taste zu öffnen. Deaktiviere diese Option, um versehentliche Aktivierung während des Spielens zu verhindern"
        }
        "gaming-game-bar-tips" => {
            "Zeigt Tipps und Hinweise zu Game-Bar-Funktionen an, wenn sich das Overlay öffnet. Das Deaktivieren reduziert Ablenkungen beim Spielen"
        }
        "gaming-performance-background-services" => {
            "Reduziert das Start-Timeout für Windows-Dienste von 60 auf 30 Sekunden. Das kann die Boot-Zeit leicht verkürzen"
        }
        "gaming-sysmain-service" => {
            "Lädt häufig genutzte Anwendungen in den RAM, um die Startzeit zu verbessern. 'Automatisch' ist für Festplatten oder gemischte Speicher sinnvoll; 'Manuell' oder 'Deaktiviert' ist nur für reine SSD-Systeme geeignet"
        }
        "gaming-performance-prefetch" => {
            "Lädt häufig genutzte Anwendungen und Boot-Dateien in den Speicher, um den Start zu beschleunigen. In der Regel für HDDs empfohlen, nicht für SSDs"
        }
        "gaming-windows-search-service" => {
            "Indiziert Dateien und Ordner für schnellere Suchergebnisse. Das Deaktivieren reduziert Hintergrund-CPU- und Datenträgeraktivität, bricht aber die Outlook-Suche und macht Startmenü- und Explorer-Suche langsam oder unzuverlässig"
        }
        "gaming-print-spooler-service" => {
            "Verwaltet Druckaufträge an Drucker. Wenn du keinen Drucker verwendest, setze es auf Manuell oder Deaktiviert, um Systemressourcen freizugeben"
        }
        "gaming-telemetry-service" => {
            "Sendet Nutzungsdaten und Diagnosen an Microsoft. Das Setzen auf Manuell oder Deaktiviert reduziert Netzwerk- und CPU-Nutzung im Hintergrund"
        }
        "gaming-connected-devices-platform-service" => {
            "Aktiviert geräteübergreifende Funktionen wie Smartphone-Verknüpfung und Nearby Sharing. Das Deaktivieren reduziert Hintergrundaktivität und das Protokollieren von Geräteinteraktionen"
        }
        "gaming-compatibility-assistant-service" => {
            "Überwacht Programme auf Kompatibilitätsprobleme und schlägt Korrekturen vor. Das Deaktivieren verhindert Kompatibilitätsabfragen und spart geringe Systemressourcen"
        }
        "gaming-error-reporting-service" => {
            "Sammelt und sendet Absturzdaten an Microsoft. Das Deaktivieren verhindert Absturzberichte, reduziert Netzwerkverkehr und verbessert den Datenschutz mit minimalen Auswirkungen"
        }
        "gaming-geolocation-service" => {
            "Verfolgt deinen physischen Standort für Apps und Dienste. Das Deaktivieren verbessert die Privatsphäre und verhindert Standortverfolgung, aber Apps können Standortfunktionen nicht nutzen"
        }
        "gaming-retail-demo-service" => {
            "Steuert die Geräteaktivität im Retail-Demo-Modus. Für PCs ist das Deaktivieren meist sicher, da es nur für den Ausstellungsbetrieb gedacht ist"
        }
        "gaming-insider-service" => {
            "Verwaltet Funktionen des Windows-Insider-Programms und Vorschau-Builds. Das Deaktivieren ist sicher, wenn du nicht am Insider-Programm teilnimmst"
        }
        "gaming-phone-service" => {
            "Verwaltet den Telefonie-Status auf dem Gerät. Du kannst die Funktion leicht deaktivieren, wenn du keine Telefonverbindungsfunktionen oder Anrufe vom PC aus nutzt"
        }
        "gaming-wallet-service" => {
            "Bietet Wallet-Funktionen für Zahlungs- und NFC-Szenarien. Es ist sicher, die Funktion zu deaktivieren, wenn du Microsoft Wallet nicht nutzt"
        }
        "gaming-smart-card-services" => {
            "Aktiviert die Smartcard-Lesefunktion für die Sicherheitsauthentifizierung. Das Deaktivieren ist sicher, wenn du keine physischen Smartcards oder Kartenleser nutzt"
        }
        "gaming-maps-broker-service" => {
            "Bietet Apps Zugriff auf heruntergeladene Karten. Setze die Option auf Manuell, um Karten bei Bedarf zu verwenden und unnötige Hintergrundaktivität zu verhindern"
        }
        "gaming-fax-service" => {
            "Ermöglicht das Senden und Empfangen von Faxen. Für die meisten Nutzer ist das Deaktivieren sicher, da Fax auf modernen Systemen selten genutzt wird"
        }
        "gaming-wmp-network-service" => {
            "Teilt Windows Media Player-Bibliotheken mit anderen Netzwerkplayern und Mediadiensten. Du kannst die Funktion sicher deaktivieren, wenn du keine Medien im Netzwerk teilst"
        }
        "gaming-mixed-reality-service" => {
            "Startet OpenXR-Anwendungen auf Windows-Mixed-Reality-Geräten. Das Deaktivieren ist sicher, wenn du kein VR- oder AR-Headset nutzt"
        }
        "gaming-mobile-hotspot-service" => {
            "Bietet die Möglichkeit, die Internetverbindung mit anderen Geräten zu teilen. Setze auf Manuell, um die Funktion verfügbar zu halten und unnötige Hintergrundaktivität zu verhindern"
        }
        "gaming-sms-router-service" => {
            "Leitet SMS-Nachrichten nach Regeln weiter. Du kannst die Funktion sicher deaktivieren, wenn du SMS-Funktionen auf dem PC nicht nutzt"
        }
        "gaming-parental-controls-service" => {
            "Aktiviert Kindersicherungs- und Familienfunktionen. Du kannst die Funktion sicher deaktivieren, wenn du keine Kindersicherung verwendest"
        }
        "gaming-payments-nfc-service" => {
            "Verwaltet Zahlungen und sichere Elemente für NFC-Szenarien. Es ist sicher, die Funktion zu deaktivieren, wenn du NFC-Zahlungen nicht nutzt"
        }
        "gaming-spot-verifier-service" => {
            "Prüft auf mögliche Dateisystembeschädigungen. Setze die Option auf Manuell, um die Prüfung bei Bedarf zu aktivieren und Hintergrundaktivität zu reduzieren"
        }
        "gaming-remote-access-manager" => {
            "Verwaltet VPN- und DFÜ-Verbindungen. Setze auf Manuell, um Hintergrundaktivität zu reduzieren und VPN bei Bedarf verfügbar zu halten."
        }
        "gaming-remote-access-auto" => {
            "Verbindet automatisch mit entfernten Netzwerken, wenn Programme auf entfernte Ressourcen verweisen. Du kannst die Funktion sicher deaktivieren, wenn du Auto-Connect für VPN nicht nutzt"
        }
        "gaming-remote-desktop-services" => {
            "Ermöglicht es Benutzern, sich interaktiv mit einem entfernten Computer zu verbinden. Setze auf Manuell, um Hintergrundaktivität zu reduzieren und Remote Desktop verfügbar zu halten."
        }
        "gaming-remote-desktop-configuration" => {
            "Verwaltet Remote-Desktop-Dienste und zugehörige Konfigurationen. Setze auf Manuell, um Hintergrundaktivität zu reduzieren und Remote Desktop verfügbar zu halten"
        }
        "gaming-remote-desktop-port-redirector" => {
            "Ermöglicht lokale Geräteumleitung für Remote-Desktop-Verbindungen. Es ist sicher, die Funktion zu deaktivieren, wenn du keine lokalen Geräte in Remote-Desktop-Sitzungen teilen musst"
        }
        "gaming-xbox-auth-manager" => {
            "Bietet Authentifizierungs- und Autorisierungsdienste für Xbox Live. Es ist sicher, die Funktion zu deaktivieren, wenn du Xbox Game Pass, Microsoft-Store-Spiele oder Xbox-Funktionen nicht nutzt"
        }
        "gaming-xbox-game-save" => {
            "Synchronisiert Spielstände mit der Xbox-Live-Cloud. Nur erforderlich für Xbox-Game-Pass- und Microsoft-Store-Spiele mit Cloud-Speicher"
        }
        "gaming-xbox-networking" => {
            "Unterstützt Xbox-Live-Multiplayer-Netzwerke. Erforderlich für Xbox-Multiplayer-Spiele, aber nicht für Steam-, Epic- oder andere Gaming-Plattformen"
        }
        "gaming-biometric-service" => {
            "Ermöglicht Fingerabdruck- und Gesichtserkennung per Windows Hello. Sicher zu deaktivieren auf Desktop-Systemen ohne biometrische Hardware"
        }
        "gaming-touch-keyboard-service" => {
            "Verwaltet die Windows-Eingabeumgebung, einschließlich Bildschirmtastatur, Stift-/Stifteingabe, Handschrifteingabefeld, Emoji-Feld (Win+.), und Xbox-Controller-Tastatur. Das Deaktivieren bricht alle virtuellen/Software-Tastaturen, ist aber auf Desktop-Systemen ohne Touchscreen, Stift oder Gamepad sicher"
        }
        "gaming-sensor-monitoring-service" => {
            "Überwacht verschiedene Sensoren wie Umgebungslicht und Ausrichtung. Sicher zu deaktivieren auf Desktop-Systemen ohne Sensors-Hardware"
        }
        "gaming-sensor-data-service" => {
            "Liefert Daten von verschiedenen Sensoren an Anwendungen. Sicher zu deaktivieren auf Desktop-Systemen ohne Sensor-Hardware"
        }
        "gaming-ai-fabric-service" => {
            "Der Windows AI Fabric Service (WSAIFabricSvc) verwaltet KI-Workloads. Deaktiviere diese Option, wenn du keine Windows-KI-Funktionen nutzt"
        }
        "visual-effects-mode" => "Visuelle Effekte",
        "ui-effects" => "UI-Effekte",
        "window-animation" => "Fensteranimation",
        "taskbar-animations" => "Taskleisten-Animationen",
        "enable-peek" => "Peek aktivieren",
        "menu-animation" => "Menüanimation",
        "fade-tooltip" => "Tooltip-Fade",
        "fade-menu-items" => "Menüeinträge ausblenden",
        "taskbar-thumbnails" => "Taskleisten-Miniaturen",
        "mouse-shadow" => "Mausschatten",
        "window-shadows" => "Fensterschatten",
        "show-thumbnails" => "Miniaturen anzeigen",
        "translucent-selection" => "Transparente Auswahl",
        "drag-full-windows" => "Vollfenster beim Ziehen",
        "combo-box-animation" => "Combo-Box-Animation",
        "font-smoothing" => "Schriftsmoothing",
        "smooth-scroll-listboxes" => "Sanftes Scrollen in Listen",
        "drop-shadows" => "Schlagschatten",
        "gaming-narrator-hotkey" => "Sprachausgabe-Hotkey",
        "accessibility-stickykeys-hotkey" => "Einrastfunktion-Hotkey",
        "accessibility-filterkeys-hotkey" => "Filtertasten-Hotkey",
        "accessibility-togglekeys-hotkey" => "Umschalttasten-Hotkey",
        "accessibility-mousekeys-hotkey" => "Maustasten-Hotkey",
        "accessibility-highcontrast-hotkey" => "Kontrastmodus-Hotkey",
        "performance_group_0" => "Gaming",
        "performance_group_1" => "Prozessor",
        "performance_group_2" => "Grafik",
        "performance_group_3" => "Netzwerk",
        "performance_group_4" => "Sicherheit",
        "performance_group_5" => "Xbox",
        "performance_group_6" => "Systemdienste",
        "performance_group_7" => "Geplante Aufgaben",
        "performance_group_8" => "Visuelle Effekte",
        "performance_group_9" => "Barrierefreiheit",
        "home_system" => "System",
        "home_processor" => "Prozessor",
        "home_graphics" => "Grafik",
        "home_memory" => "Speicher",
        "home_storage" => "Speicherplatz",
        "home_windows" => "Windows",
        "home_uptime" => "Betriebszeit",
        "home_performance" => "Leistung",
        "home_product_name" => "Systemproduktname",
        "home_cpu" => "CPU",
        "home_cpu_model" => "CPU-Modell",
        "home_cores" => "Kerne",
        "home_gpu" => "GPU",
        "home_memory_total" => "Gesamtspeicher",
        "home_memory_used" => "Verwendeter Speicher",
        "home_version" => "Version",
        "home_kernel" => "Kernel",
        "home_name" => "Name",
        "home_bios_version" => "BIOS-Version",
        "home_bios_date" => "BIOS-Datum",
        "home_unknown_cpu" => "Unbekannte CPU",
        "home_unknown_pc" => "Unbekannter PC",
        "home_unknown_os" => "Unbekanntes OS",
        "home_unknown_kernel" => "Unbekannter Kernel",
        "home_unknown_model" => "Unbekanntes Modell",
        "home_unknown_vendor" => "Unbekannter Hersteller",
        "home_unknown_bios" => "Unbekanntes BIOS",
        "home_unknown_date" => "Unbekanntes Datum",
        "home_unknown_gpu" => "Unbekannte GPU",
        "home_unknown_vram" => "Unbekannter VRAM",
        "home_unknown_value" => "Unbekannt",
        "home_cores_suffix" => "{} Kerne",
        "home_gb_total" => "{:.1} GB gesamt",
        "home_gb_used" => "{:.1} GB verwendet",
        "home_tb_total" => "{:.2} TB gesamt",
        "home_tb_used" => "{:.2} TB verwendet",
        "home_uptime_fmt" => "{}T {:02}h {:02}m",
        "home_update_status" => "Update-Status",
        "download_category_0" => "Browser",
        "download_category_1" => "Dokumentenbetrachter",
        "download_category_2" => "Nachrichten, E-Mail & Kalender",
        "download_category_3" => "Online-Speicher & Backup",
        "download_category_4" => "Multimedia",
        "download_category_5" => "Bildbearbeitung",
        "download_category_6" => "Anpassungs-Tools",
        "download_category_7" => "Gaming",
        "download_category_8" => "Komprimierung",
        "download_category_9" => "Datei- & Datenträgerverwaltung",
        "download_category_10" => "Remotezugriff",
        "download_category_11" => "Optische Laufwerke",
        "download_category_12" => "Sonstige Tools",
        "download_category_13" => "Datenschutz & Sicherheit",
        "download_category_14" => "Entwicklungs-Apps",
        "download_category_15" => "Laufzeiten & Abhängigkeiten",
        "Microsoft EdgeWebView" => "WebView2-Laufzeit für Windows-Anwendungen",
        "Thorium" => "Chromium-basierter Browser mit erweiterten Datenschutzfunktionen",
        "Mercury" => "Compiler-optimierter, privater Firefox-Fork",
        "Mozilla Firefox" => "Beliebter Webbrowser mit Fokus auf Datenschutz und Anpassung",
        "Google Chrome" => "Googles Webbrowser mit Synchronisierung und Erweiterungsunterstützung",
        "ungoogled-chromium" => "Chromium-basierter Browser mit Datenschutzverbesserungen",
        "Brave" => "Datenschutzorientierter Browser mit integriertem Werbeblocker",
        "Opera" => "Funktionsreicher Browser mit integriertem VPN und Werbeblocker",
        "Opera GX" => "Gaming-orientierte Version von Opera mit einzigartigen Funktionen",
        "Arc Browser" => "Innovativer Browser mit Fokus auf Design und Nutzererlebnis",
        "Tor Browser" => {
            "Datenschutzorientierter Browser, der Datenverkehr über das Tor-Netzwerk leitet"
        }
        "Vivaldi" => "Hochgradig anpassbarer Browser mit Fokus auf Nutzerkontrolle",
        "Waterfox" => "Firefox-basierter Browser mit Fokus auf Datenschutz und Anpassung",
        "Zen Browser" => "Datenschutzorientierter Browser mit integriertem Werbeblocker",
        "Mullvad Browser" => {
            "Datenschutzorientierter Browser, der Tracking und Fingerprinting minimiert"
        }
        "Pale Moon Browser" => {
            "Open-Source-Webbrowser auf Goanna-Basis mit Fokus auf Effizienz und Anpassung"
        }
        "Maxthon" => "Datenschutzorientierter Browser mit integriertem Werbeblocker und VPN",
        "Ablaze Floorp" => "Datenschutzorientierter Browser mit starkem Tracking-Schutz",
        "DuckDuckGo" => "Datenschutzorientierte Suchmaschine mit Browser-Erweiterung",
        "LibreOffice" => "Kostenlose Open-Source-Office-Suite",
        "ONLYOFFICE Desktop Editors" => {
            "100 % kostenlose Open-Source-Alternative zu Microsoft Office"
        }
        "PDFgear" => {
            "PDF-Dateien auf verschiedenen Geräten lesen, bearbeiten, konvertieren, zusammenführen und signieren – komplett kostenlos und ohne Anmeldung."
        }
        "Foxit PDF Reader" => "Leichter PDF-Reader mit erweiterten Funktionen",
        "SumatraPDF" => {
            "PDF-, eBook- (epub, mobi), Comic-Book- (cbz/cbr), DjVu-, XPS-, CHM- und Bildbetrachter für Windows"
        }
        "OpenOffice" => {
            "Eingestellte Open-Source-Office-Suite. Das aktive Nachfolgeprojekt ist LibreOffice"
        }
        "Adobe Acrobat Reader DC" => "PDF-Reader und -Editor",
        "Evernote" => "Notizen-App",
        "CherryTree" => "Hierarchische Notizanwendung mit Rich Text und Syntax-Hervorhebung",
        "Okular" => "Universeller Dokumentbetrachter für PDF, eBooks und mehr",
        "PDF24 Creator" => "Kostenloser PDF-Ersteller und -Konverter",
        "Telegram Desktop" => "Instant-Messaging- und Sprachanruf-App",
        "WhatsApp" => "Instant-Messaging- und Sprachanruf-App",
        "Zoom Workplace" => "Plattform für Videokonferenzen und Nachrichten",
        "Discord" => "Sprach-, Video- und Textkommunikationsdienst",
        "Pidgin" => "Multi-Protokoll-Client für Sofortnachrichten",
        "Mozilla Thunderbird" => "Kostenlose E-Mail-Anwendung",
        "eM Client" => "E-Mail-Client mit Kalender, Aufgaben und Chat",
        "Proton Mail" => "Sicherer E-Mail-Dienst mit Ende-zu-Ende-Verschlüsselung",
        "Trillian" => "Anwendung für Sofortnachrichten",
        "Google Drive" => "Cloud-Speicher- und Dateisynchronisationsdienst",
        "Dropbox" => {
            "Dateihosting-Dienst mit Cloud-Speicher, Dateisynchronisierung und persönlicher Cloud"
        }
        "SugarSync" => {
            "Automatisch auf Fotos, Videos und Dateien in beliebigen Ordnern zugreifen und sie teilen"
        }
        "Nextcloud" => {
            "Zugriff, Teilen und Schutz deiner Dateien, Kalender, Kontakte, Kommunikation & mehr zu Hause und in deiner Organisation"
        }
        "Proton Drive" => "Sicherer Cloud-Speicher mit Ende-zu-Ende-Verschlüsselung",
        "FreeFileSync" => "Open-Source-Tool zum Vergleichen und Synchronisieren von Ordnern",
        "Hekasoft Backup & Restore" => {
            "Die komplette kostenlose Lösung für Browser-Backups und Verwaltung"
        }
        "VLC media player" => "Open-Source-Mediaplayer und -Framework",
        "iTunes" => "Medienplayer und Mediathek",
        "AIMP" => "Audio-Player mit Unterstützung für verschiedene Formate",
        "foobar2000" => "Fortschrittlicher Audio-Player für Windows",
        "MusicBee" => "Musikverwaltung und -player",
        "Audacity" => "Audio-Editor und -Recorder",
        "GOM Player" => "Medienplayer für Windows",
        "Spotify" => "Musik-Streaming-Dienst",
        "MediaMonkey" => "Medienverwaltung und -player",
        "HandBrake" => "Open-Source-Video-Transcoder",
        "OBS Studio" => "Kostenlose Open-Source-Software für Videoaufzeichnung und Live-Streaming",
        "Streamlabs OBS" => {
            "Streaming-Software auf Basis von OBS mit zusätzlichen Funktionen für Streamer"
        }
        "MPC-BE" => "Media Player Classic - Black Edition",
        "K-Lite Codec Pack (Mega)" => "Sammlung von Codecs und zugehörigen Tools",
        "CapCut" => "Videoeditor",
        "PotPlayer64" => "Umfassender Multimedia-Player für Windows",
        "kdenlive" => "Kostenlose Open-Source-Video-Bearbeitungssoftware",
        "MediaInfo" => "Werkzeug zur Anzeige technischer Informationen für Multimediadateien",
        "fre:ac - free audio converter" => "Kostenloser Audio-Konverter und CD-Ripper",
        "SMPlayer" => {
            "Media Player mit eingebauten Codecs, der praktisch alle Video- und Audioformate abspielen kann"
        }
        "Shotcut" => "Kostenloser, quelloffener, plattformübergreifender Videoeditor",
        "LosslessCut" => {
            "Plattformübergreifende FFmpeg-GUI für schnelles, verlustfreies Schneiden von Video/Audio"
        }
        "FxSound" => "Audio-Verbesserer zur Steigerung der Klangqualität unter Windows",
        "IrfanView64" => "Schneller und kompakter Bildbetrachter und Konverter",
        "Krita" => "Digitale Mal- und Illustrationssoftware",
        "Blender" => "3D-Kreationssuite",
        "Paint.NET" => "Bild- und Foto-Bearbeitungssoftware",
        "GIMP" => "GNU Image Manipulation Program",
        "XnViewMP" => "Bildbetrachter, Browser und Konverter",
        "XnView" => "Bildbetrachter, Browser und Konverter (klassische Version)",
        "Inkscape" => "Vektor-Grafikeditor",
        "Greenshot" => "Screenshot-Tool mit Anmerkungsfunktionen",
        "ShareX" => "Bildschirmaufnahme-, Dateifreigabe- und Produktivitätstool",
        "Flameshot" => "Leistungsfähige und dennoch einfach zu bedienende Screenshot-Software",
        "FastStone Image Viewer" => "Bildbrowser, Konverter und Editor",
        "Nilesoft Shell" => "Tool zur Anpassung des Windows-Kontextmenüs",
        "StartAllBack (Win 11)" => "Anpassung des Windows-11-Startmenüs und der Taskleiste",
        "StartIsBack++ (Win 10)" => "Anpassung des Windows-10-Startmenüs und der Taskleiste",
        "Open-Shell" => "Startmenü im klassischen Stil für Windows",
        "Windhawk" => "Anpassungsplattform für Windows",
        "Lively Wallpaper" => "Kostenlose Open-Source-Anwendung für animierte Desktop-Hintergründe",
        "Sucrose Wallpaper Engine" => {
            "Kostenlose Open-Source-Anwendung für animierte Desktop-Hintergründe"
        }
        "Rainmeter" => "Desktop-Anpassungstool für Windows",
        "ExplorerPatcher" => "Dienstprogramm, das die Windows-Explorer-Erfahrung verbessert",
        "John's Background Switcher" => {
            "Ändert automatisch in regelmäßigen Abständen dein Desktop-Hintergrundbild"
        }
        "Microsoft PowerToys" => {
            "Sammlung von Tools für Power-User, um ihre Windows-Umgebung zu optimieren"
        }
        "Nexus" => "Das fortschrittliche Docking-System für Windows",
        "AutoHotkey v2" => "Kostenlose Skriptsprache für Makros und Automatisierung (v2, aktuell)",
        "Steam" => "Digitale Vertriebsplattform für PC-Spiele",
        "Epic Games Launcher" => "Digitale Vertriebsplattform für PC-Spiele",
        "7-Zip" => "Open-Source-Dateiarchiv mit hoher Kompressionsrate",
        "WinRAR archiver" => "Dateiarchiv mit hoher Kompressionsrate",
        "PeaZip" => "Open-Source-Archivierer und Dateimanager",
        "WinDirStat" => "Werkzeug zur grafischen Anzeige von Datenträgernutzung",
        "WizTree" => "Schneller Analyzer für Speicherplatzbelegung",
        "TreeSize Free" => "Verwaltungstool für Speicherplatz",
        "Everything" => "Schnelle Dateisuche für Windows",
        "TeraCopy" => "Schnelles Kopier- und Verschiebe-Tool für Dateien",
        "File Converter" => "Werkzeug zum Konvertieren von Dateien",
        "Crystal Disk Info" => "Tool zur Anzeige von Laufwerkszustand und S.M.A.R.T.-Daten",
        "Bulk Rename Utility" => "Werkzeug zum massenhaften Umbenennen von Dateien",
        "IObit Unlocker" => "Hilfsprogramm zum Freigeben gesperrter Dateien",
        "HiBit Uninstaller" => "Deinstallations- und Bereinigungstool",
        "SanDisk Dashboard" => "Verwaltungstool für SanDisk-Laufwerke",
        "Rufus" => "Tool zum Erstellen bootfähiger USB-Laufwerke",
        "Advanced Renamer" => "Erweitertes Werkzeug zum Umbenennen von Dateien",
        "RustDesk" => "Open-Source-Remote-Desktop-Software",
        "AnyDesk" => "Remote-Desktop-Software",
        "TeamViewer" => "Remote-Zugriffs- und Support-Software",
        "UltraViewer" => "Fernwartungs-Software",
        "RealVNC Server" => "VNC-Server für Fernzugriff",
        "RealVNC Viewer" => "VNC-Client für Fernzugriff",
        "Chrome Remote Desktop" => "Remote-Desktop-Dienst von Chrome",
        "Parsec" => "Software für Remote-Desktop und Streaming",
        "Parsec Virtual Display Driver" => "Virtueller Anzeigetreiber für Parsec",
        "Parsec Virtual USB Driver" => "Virtueller USB-Treiber für Parsec",
        "InputLeap" => "Werkzeug zum Teilen von Maus und Tastatur zwischen PCs",
        "ImgBurn" => "Tool zum Brennen von Datenträgern",
        "AnyBurn" => "Leichtgewichtiges Brenn- und Imaging-Tool",
        "CDBurnerXP" => "Kostenlose Brennsoftware",
        "CCleaner" => "Systembereinigungs-Tool",
        "Snappy Driver Installer Origin" => "Treiberinstallations- und Update-Tool",
        "Wise Disk Cleaner" => "Festplattenbereinigungs-Tool",
        "Wise Registry Cleaner" => "Registrierungsbereinigungs-Tool",
        "UniGetUI" => "Benutzeroberfläche für Paketmanager unter Windows",
        "OpenRGB" => "Open-Source-RGB-Beleuchtungssteuerung",
        "OpenAudible" => "Audiobook-Manager für Audible",
        "NAPS2" => "Einfaches Dokumentenscan-Tool",
        "IObit Uninstaller" => "Deinstallations- und Bereinigungstool",
        "Revo Uninstaller" => "Deinstallations-Tool mit gründlicher Restentfernung",
        "Malwarebytes" => "Anti-Malware-Software",
        "Malwarebytes AdwCleaner" => "Tool zum Entfernen von Adware und unerwünschter Software",
        "Windows Firewall Control" => "Verwaltungstool für die Windows-Firewall",
        "OnionShare" => "Sicheres und anonymes Teilen von Dateien",
        "Sniffnet" => "Netzwerküberwachungstool",
        "TeleGuard" => "Sichere Messenger-App",
        "Python 3.13" => "Programmiersprache Python",
        "Notepad++" => "Kostenloser Quellcode-Editor und Notepad-Ersatz",
        "WinSCP" => "Kostenloser SFTP-, SCP- und Amazon-S3-Client",
        "PuTTY" => "Kostenloser SSH- und Telnet-Client",
        "WinMerge" => "Open-Source-Tool zum Vergleichen und Zusammenführen",
        "Eclipse IDE for Java" => "Java-IDE und Entwicklungsumgebung",
        "Microsoft Visual Studio Code" => "Code-Editor von Microsoft",
        "Git" => "Verteiltes Versionskontrollsystem",
        "GitHub Desktop" => "GitHub-Desktop-Client",
        "Microsoft .NET Runtime 3.1" => ".NET-Laufzeit 3.1",
        "Microsoft .NET Runtime 5.0" => ".NET-Laufzeit 5.0",
        "Microsoft .NET Runtime 6.0" => ".NET-Laufzeit 6.0",
        "Microsoft .NET Runtime 7.0" => ".NET-Laufzeit 7.0",
        "Microsoft .NET Runtime 8.0" => ".NET-Laufzeit 8.0",
        ".NET Framework 4.8.1" => ".NET Framework 4.8.1",
        "DirectX Runtime" => "DirectX-Laufzeitkomponenten",
        "Java Runtime Environment" => "Java-Laufzeitumgebung",
        "Visual C++ 2005 (x86)" => "Visual C++ 2005 (x86)",
        "Visual C++ 2005 (x64)" => "Visual C++ 2005 (x64)",
        "Visual C++ 2008 (x86)" => "Visual C++ 2008 (x86)",
        "Visual C++ 2008 (x64)" => "Visual C++ 2008 (x64)",
        "Visual C++ 2010 (x86)" => "Visual C++ 2010 (x86)",
        "Visual C++ 2010 (x64)" => "Visual C++ 2010 (x64)",
        "Visual C++ 2012 (x86)" => "Visual C++ 2012 (x86)",
        "Visual C++ 2012 (x64)" => "Visual C++ 2012 (x64)",
        "Visual C++ 2013 (x86)" => "Visual C++ 2013 (x86)",
        "Visual C++ 2013 (x64)" => "Visual C++ 2013 (x64)",
        "Visual C++ 2015-2022 (x86)" => "Visual C++ 2015-2022 (x86)",
        "Visual C++ 2015-2022 (x64)" => "Visual C++ 2015-2022 (x64)",
        "Tabby" => "SSH-Terminal- und Verbindungsmanager",
        "Riot Games Launcher" => "Startprogramm für Riot Games",
        "Helium Browser" => "Datenschutzorientierter Chromium-Browser",
        "DataGrip" => "JetBrains SQL-IDE",
        "Zed" => "Hochleistungs-Code-Editor",
        "app_name_m365_copilot_office_hub" => "MS 365 Copilot (Office Hub)",
        "app_name_remote_assistance" => "Remoteunterstützungs-App",
        "app_name_classic_paint" => "Klassische Paint-App",
        "windows-app-office-hub" => "Microsoft 365 Copilot (früher Office Hub)",
        "windows-app-remote-assistance" => "Remoteunterstützungs-App",
        "capability-paint-legacy" => "Klassische Paint-App",
        "CEIPConsolidatorTask" => {
            "Sammelt Telemetriedaten für das Customer Experience Improvement Program"
        }
        "UsbCeipTask" => "Sammelt USB-bezogene Telemetriedaten",
        "DiskDiagnosticTask" => "Sammelt Diagnoseinformationen zu Datenträgern",
        "FeedbackDmClientTask" => "Sammelt Feedback-Daten für Microsoft",
        "FeedbackDmClientDownloadTask" => "Lädt Feedback-bezogene Daten herunter",
        "ErrorReportingQueueTask" => "Wartet Fehlerberichte in der Warteschlange",
        "SqmTask" => "Software Quality Metrics-Task",
        "MareBackupTask" => "Sichert Daten für Wiederherstellungsszenarien",
        "StartupAppTask" => "Startet Aufgaben beim Systemstart",
        "MapsUpdateTask" => "Aktualisiert heruntergeladene Karten",
        "AutochkProxyTask" => "Hilfsaufgabe für Datenträgerprüfung",
        "FamilySafetyTask" => "Verwaltet Familien- und Kindersicherungsfunktionen",
        "PowerEfficiencyTask" => "Verwaltet energiebezogene Effizienzaufgaben",
        "WindowsAIRecallConfig" => "Windows-AI-Recall-Konfiguration",
        "WindowsAIRecallPipeline" => "Windows-AI-Recall-Pipeline",
        "OfficeActionsServer" => "Office-Aktionsserver",
        "latency_progress_verify" => "Topologie und Energiehinweise werden überprüft...",
        "latency_progress_report" => "Bericht wird erstellt...",
        "CompatibilityAppraiserTask" => {
            "Sammelt Kompatibilitäts-Telemetriedaten von Programmen für Windows-Upgrades. Arbeitet mit dem Dienst 'Connected User Experiences and Telemetry' zusammen. Deaktiviere dies, um Telemetrie und Hintergrundaktivität zu reduzieren"
        }
        "ProgramDataUpdaterTask" => {
            "Aktualisiert Programmdaten für Microsoft-Software. Deaktiviere dies, um Hintergrundaktivität zu reduzieren"
        }
        _ => super::en::t(key),
    }
}
