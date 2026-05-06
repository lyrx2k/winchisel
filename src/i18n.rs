use crate::Language;

pub fn t(lang: Language, key: &str) -> &'static str {
    match lang {
        Language::German => match key {
            "nav_title" => "Navigation",
            "nav_subtitle" => "Winchisel Steuerzentrale",
            "home" => "Start",
            "debloater" => "Debloater",
            "downloads" => "Apps & Downloads",
            "performance" => "Leistung",
            "processes" => "Prozesse",
            "latency" => "Latenz",
            "settings" => "Einstellungen",
            "check_updates" => "Updates prüfen",
            "donate" => "Spenden",
            "bug_report" => "Fehler melden",
            "status_admin" => "Administrator",
            "status_standard" => "Standard",
            "settings_title" => "Einstellungen",
            "settings_subtitle" => "Passe das Verhalten der App an deinen Workflow an.",
            "settings_application" => "Anwendung",
            "settings_saved_auto" => "Diese Einstellungen werden automatisch gespeichert.",
            "language" => "Sprache",
            "sidebar_languages" => "Sprachen",
            "archive-restore" => "Wiederherstellen",
            "settings-2" => "Einstellungen",
            "shield-check" => "Sicherheitsstatus",
            "refresh-cw" => "Aktualisieren",
            "monitor" => "Konsole",
            "Winchisel-Updater" => "Winchisel-Updater",
            "Disabled (Recommended)" => "Deaktiviert (Empfohlen)",
            "0" => "0",
            "1" => "1",
            "app_name_m365_copilot_office_hub" => "MS 365 Copilot (Office Hub)",
            "app_name_remote_assistance" => "Remote-Hilfe-App",
            "app_name_classic_paint" => "Klassische Paint-App",
            "open_logs" => "Logs öffnen",
            "check_updates_startup" => "Beim Start nach Updates suchen",
            "show_console" => "Konsole anzeigen",
            "system_protection" => "Systemschutz",
            "restore_point" => "Wiederherstellungspunkt",
            "restore_point_desc" => "Erstellt vor größeren Systemänderungen einen Rücksetzpunkt",
            "create_restore_point" => "Wiederherstellungspunkt erstellen",
            "performance_title" => "Leistung",
            "performance_subtitle" => "Gaming- und Leistungsoptimierungen",
            "performance_search" => "Leistungs-Tweaks suchen...",
            "performance_quick" => "Schnellaktionen",
            "performance_apply_recommended" => "Empfohlene Einstellungen anwenden",
            "performance_reset_defaults" => "Auf Windows-Standard zurücksetzen",
            "performance_loading" => "Leistungs-Tweaks werden geladen...",
            "performance_empty" => "Keine Leistungs-Tweaks vorhanden.",
            "performance_current_default" => "Aktueller Standard: ",
            "performance_current_recommended" => "Empfohlen: ",

            "accessibility-filterkeys-hotkey" => {
                "Aktivieren Sie die Tastenkombination zum Aktivieren von FilterKeys, indem Sie die rechte Umschalttaste 8 Sekunden lang gedrückt halten"
            }
            "accessibility-highcontrast-hotkey" => {
                "Aktivieren Sie die Tastenkombination, um den Modus „Hoher Kontrast“ zu aktivieren, indem Sie die linke Alt-Taste + linke Umschalttaste + Bildschirmdruck drücken"
            }
            "accessibility-mousekeys-hotkey" => {
                "Aktivieren Sie die Tastenkombination, um MouseKeys zu aktivieren, wodurch die Verwendung des Ziffernblocks zur Steuerung des Mauszeigers ermöglicht wird"
            }
            "accessibility-stickykeys-hotkey" => {
                "Aktivieren Sie die Tastenkombination zum Aktivieren von StickyKeys, indem Sie fünfmal die Umschalttaste drücken"
            }
            "accessibility-togglekeys-hotkey" => {
                "Aktivieren Sie die Tastenkombination, um ToggleKeys zu aktivieren, indem Sie die Num-Taste 5 Sekunden lang gedrückt halten. Dadurch werden Töne abgespielt, wenn die Feststell-/Num-/Scroll-Taste gedrückt wird"
            }
            "AutochkProxyTask" => {
                "Führt Festplattenüberprüfungen durch und sammelt Diagnosedaten. Erwägen Sie, die Überwachung des Festplattenzustands aktiviert zu lassen"
            }
            "CEIPConsolidatorTask" => {
                "Konsolidiert und lädt Nutzungsdaten im Rahmen des Programms zur Verbesserung der Kundenerfahrung hoch. Funktioniert mit dem Connected User Experiences- und Telemetriedienst. Deaktivieren, um den Datenschutz zu verbessern"
            }
            "combo-box-animation" => {
                "Animiert Kombinationsfelder beim Öffnen mit einem Schiebeeffekt"
            }
            "CompatibilityAppraiserTask" => {
                "Sammelt Telemetriedaten zur Programmkompatibilität für Windows-Upgrades. Funktioniert zusammen mit dem Connected User Experiences- und Telemetriedienst. Deaktivieren, um Telemetrie und Hintergrundsystemaktivität zu reduzieren"
            }
            "DiskDiagnosticTask" => {
                "Sammelt Festplattendiagnoseinformationen und S.M.A.R.T. Daten für Microsoft. Deaktivieren Sie diese Option, um die Festplattenaktivität und Telemetrie im Hintergrund zu reduzieren"
            }
            "drag-full-windows" => "Zeigt beim Ziehen den Fensterinhalt statt nur einer Kontur an",
            "drop-shadows" => {
                "Fügen Sie Schatteneffekte hinter dem Text des Desktop-Symbols hinzu, um die Lesbarkeit vor dem Hintergrund zu verbessern"
            }
            "enable-peek" => {
                "Ermöglicht einen Blick auf den Desktop, wenn Sie mit der Maus über die Schaltfläche „Desktop anzeigen“ fahren"
            }
            "ErrorReportingQueueTask" => {
                "Stellt Absturzberichte und Fehlerdaten in die Warteschlange, um sie an Microsoft zu senden. Funktioniert zusammen mit dem Windows-Fehlerberichtsdienst. Deaktivieren Sie beide, um die Erfassung von Absturzdaten zu verhindern"
            }
            "fade-menu-items" => {
                "Blendet Menüpunkte nach der Auswahl aus, bevor das Menü geschlossen wird"
            }
            "fade-tooltip" => {
                "Animiert Tooltips, wenn sie angezeigt werden, mithilfe von Fade- oder Slide-Effekten"
            }
            "FamilySafetyTask" => {
                "Überwacht Familiensicherheitseinstellungen und -nutzung. Deaktivieren Sie diese Option, wenn Sie die Familiensicherheitsfunktionen nicht nutzen"
            }
            "FeedbackDmClientDownloadTask" => {
                "Lädt Feedback-Szenarien und Konfigurationsdaten von Microsoft herunter. Deaktivieren, um Telemetrie und Netzwerkaktivität zu reduzieren"
            }
            "FeedbackDmClientTask" => {
                "Sammelt Feedback- und Diagnosedaten für Microsoft. Deaktivieren Sie diese Option, um den Datenschutz zu verbessern und die Telemetrie zu reduzieren"
            }
            "font-smoothing" => {
                "Wenden Sie Anti-Aliasing auf Text an, um glattere und besser lesbare Schriftarten auf dem Bildschirm zu erzielen"
            }
            "gaming-ai-fabric-service" => {
                "Der Windows AI Fabric Service (WSAIFabricSvc) verwaltet KI-Workloads. Deaktivieren Sie diese Option, wenn Sie die Windows-KI-Funktionen nicht verwenden"
            }
            "gaming-auto-color-management" => {
                "Ermöglichen Sie Windows, Farbprofile für alle angeschlossenen Displays, die dies unterstützen, automatisch zu verwalten"
            }
            "gaming-background-apps" => {
                "Steuern Sie über Gruppenrichtlinien, ob Apps im Hintergrund ausgeführt werden können. Force Deny entfernt Hintergrundeinstellungen pro App aus den Windows-Einstellungen. Verwenden Sie User in Control, wenn Sie Apps wie Teams, Zoom oder WhatsApp benötigen"
            }
            "gaming-biometric-service" => {
                "Ermöglicht die Anmeldung per Fingerabdruck und Gesichtserkennung über Windows Hello. Auf Desktop-Systemen ohne biometrische Hardware sicher zu deaktivieren"
            }
            "gaming-compatibility-assistant-service" => {
                "Überwacht Programme auf Kompatibilitätsprobleme und schlägt Korrekturen vor. Durch die Deaktivierung werden Kompatibilitätsaufforderungen verhindert und kleinere Systemressourcen eingespart"
            }
            "gaming-connected-devices-platform-service" => {
                "Ermöglicht geräteübergreifende Erlebnisse wie Telefonverknüpfung und Teilen in der Nähe. Durch die Deaktivierung werden Hintergrundaktivitäten und die Protokollierung von Geräteinteraktionen reduziert"
            }
            "gaming-cpu-priority" => {
                "Geben Sie Spielen eine höhere CPU-Planungspriorität, um Ihrem Spiel mehr Prozessorzeit zu widmen"
            }
            "gaming-directx-auto-hdr" => {
                "Konvertieren Sie SDR-Inhalte automatisch in HDR, um Farben und Helligkeit zu verbessern. Erfordert ein HDR-fähiges Display mit aktiviertem HDR; Diese Einstellung hat keine Auswirkung, wenn Ihr Display HDR nicht unterstützt"
            }
            "gaming-directx-flip-model" => {
                "Reduzieren Sie die Latenz und nutzen Sie erweiterte Funktionen in kompatiblen Spielen, indem Sie das DirectX-Flip-Präsentationsmodell verwenden"
            }
            "gaming-directx-vrr-optimizations" => {
                "Aktivieren Sie VRR-Optimierungen (G-Sync/FreeSync) für ein flüssigeres Gameplay. Erfordert einen VRR-kompatiblen Monitor; Diese Einstellung hat keine Auswirkung, wenn Ihr Monitor VRR nicht unterstützt"
            }
            "gaming-disable-mpo" => {
                "Mehrere Anzeigeebenen in Hardware mithilfe der GPU zusammensetzen. Durch die Deaktivierung können Bildschirmflimmern, schwarze Bildschirme und Stottern bei Konfigurationen mit mehreren Monitoren behoben werden"
            }
            "gaming-disable-mpo-min-fps" => {
                "Ermöglichen Sie Desktop Window Manager, Apps basierend auf der Bildrate dynamisch zwischen Overlay-Modi zu wechseln. Durch die Deaktivierung können Stottern in Browsern und Discord behoben werden, ohne dass MPO vollständig deaktiviert werden muss"
            }
            "gaming-dns-server" => {
                "Wählen Sie einen DNS-Server für alle Netzwerkadapter aus. Änderungen gelten für jeden Adapter in Ihrem System (WLAN und Ethernet). Verwenden Sie „Automatisch“, um Ihren Standard-ISP/Router-DNS wiederherzustellen"
            }
            "gaming-error-reporting-service" => {
                "Sammelt und sendet Absturzdaten an Microsoft. Durch die Deaktivierung werden Absturzberichte verhindert, der Netzwerkverkehr reduziert und der Datenschutz bei minimalen Auswirkungen auf das System verbessert"
            }
            "gaming-explorer-alt-tab-filter" => {
                "Zeigen Sie bei Alt+Tab nur herkömmliche geöffnete Fenster an, anstatt Microsoft Edge-Registerkarten und andere Windows-Vorschläge einzubeziehen"
            }
            "gaming-fax-service" => {
                "Ermöglicht das Senden und Empfangen von Faxen. Für die meisten Benutzer ist die Deaktivierung sicher, da die Faxfunktion auf modernen Systemen selten verwendet wird"
            }
            "gaming-fullscreen-optimizations" => {
                "Ermöglichen Sie Windows, Spiele zu optimieren, die im Vollbildmodus ausgeführt werden. Durch die Deaktivierung können Leistungsprobleme oder Ruckler bei einigen älteren Spielen behoben werden, die mit der randlosen Vollbildoptimierung nicht gut funktionieren"
            }
            "gaming-game-bar-controller" => {
                "Erlauben Sie Ihrer Xbox bzw. Ihrem kompatiblen Controller, die Game Bar zu öffnen, indem Sie die Xbox-Taste drücken. Deaktivieren Sie diese Option, um eine versehentliche Aktivierung der Game Bar während des Spielens zu verhindern"
            }
            "gaming-game-bar-tips" => {
                "Zeigt beim Öffnen des Overlays Tipps und Hinweise zu Game Bar-Funktionen an. Durch Deaktivieren werden Ablenkungen während des Spiels reduziert"
            }
            "gaming-game-mode" => {
                "Optimieren Sie Ihren PC für das Spielen, indem Sie die Dinge im Hintergrund ausschalten"
            }
            "gaming-geolocation-service" => {
                "Verfolgt Ihren physischen Standort für Apps und Dienste. Die Deaktivierung verbessert den Datenschutz und verhindert die Standortverfolgung, Apps können die Standortfunktionen jedoch nicht nutzen"
            }
            "gaming-gpu-priority" => {
                "Geben Sie Spielen eine höhere GPU-Planungspriorität, um die Grafikleistung und Bildraten zu verbessern"
            }
            "gaming-gpu-scheduling" => {
                "Lassen Sie Ihre GPU ihren eigenen Speicher und ihre Planung verwalten, um die Latenz zu reduzieren und die Leistung zu verbessern"
            }
            "gaming-insider-service" => {
                "Verwaltet die Funktionen des Windows Insider-Programms und Vorschau-Builds. Die Deaktivierung ist sicher, wenn Sie nicht am Windows-Insider-Programm teilnehmen"
            }
            "gaming-maps-broker-service" => {
                "Bietet Zugriff auf heruntergeladene Karten für Anwendungen. Stellen Sie die Option auf „Manuell“ ein, um den Zugriff auf die Karte bei Bedarf zu ermöglichen und gleichzeitig unnötige Hintergrundaktivitäten zu verhindern"
            }
            "gaming-memory-integrity" => {
                "Verhindert das Einschleusen von Schadcode in Hochsicherheitsprozesse. Das Deaktivieren kann die Spieleleistung verbessern, verringert jedoch die Systemsicherheit"
            }
            "gaming-mixed-reality-service" => {
                "Führt OpenXR-Anwendungen auf Windows Mixed Reality-Geräten aus. Die Deaktivierung ist sicher, wenn Sie keine VR- oder AR-Headsets verwenden"
            }
            "gaming-mobile-hotspot-service" => {
                "Bietet die Möglichkeit, die Internetverbindung mit anderen Geräten zu teilen. Stellen Sie „Manuell“ ein, um die Funktionalität verfügbar zu halten und gleichzeitig unnötige Hintergrundaktivitäten zu verhindern"
            }
            "gaming-nagle-algorithm" => {
                "Puffert kleine Netzwerkpakete vor dem Senden, um den Overhead zu reduzieren. Schalten Sie es aus, um die Latenz bei Online-Spielen zu verringern, oder lassen Sie es eingeschaltet, um eine allgemeine Netzwerkeffizienz zu erreichen"
            }
            "gaming-narrator-hotkey" => {
                "Aktivieren Sie die Tastenkombination Win+Strg+Eingabe, um den Windows Narrator-Bildschirmleser schnell zu starten"
            }
            "gaming-network-throttling" => {
                "Steuert die Begrenzung der Netzwerkpaketrate für Multimediaanwendungen. Es wird empfohlen, die Drosselung aktiviert zu lassen (Standard: 10 Pakete/ms), da sie eine bessere DPC-Latenz für Spiele bietet als eine vollständige Deaktivierung"
            }
            "gaming-nvidia-sharpening" => {
                "Aktivieren Sie den alten NVIDIA-Bildschärfungsfilter für eine verbesserte visuelle Klarheit. Funktioniert nur mit älteren NVIDIA-Treibern; Neuere Treiber sollten stattdessen die Schärfung der NVIDIA-Systemsteuerung verwenden"
            }
            "gaming-parental-controls-service" => {
                "Aktiviert Kindersicherung und Familiensicherheitsfunktionen. Sie können die Funktion sicher deaktivieren, wenn Sie die Kindersicherungsfunktionen nicht verwenden"
            }
            "gaming-payments-nfc-service" => {
                "Verwaltet Zahlungen und sichere Elemente der Nahfeldkommunikation. Die Deaktivierung ist sicher, wenn Sie die NFC-Zahlungsfunktionen nicht nutzen"
            }
            "gaming-performance-autostart-delay" => {
                "Verzögern Sie den Start von Anwendungen um 10 Sekunden nach dem Start, um die anfängliche Systemreaktionsfähigkeit zu verbessern. Windows wird schneller nutzbar, aber das Laden Ihrer Start-Apps dauert länger"
            }
            "gaming-performance-background-services" => {
                "Reduzieren Sie das Startzeitlimit für Windows-Dienste von 60 auf 30 Sekunden. Dies kann die Bootzeit etwas verkürzen"
            }
            "gaming-performance-desktop-composition" => {
                "Aktivieren Sie visuelle Effekte, die vom Desktop Window Manager verwaltet werden. Das Deaktivieren kann auf älterer Hardware zu geringfügigen Leistungssteigerungen führen, beeinträchtigt jedoch die Aero-Effekte"
            }
            "gaming-performance-explorer-menu-show-delay" => {
                "Fügen Sie eine kurze Verzögerung hinzu, bevor Menüs angezeigt werden (400 ms – Windows-Standard), oder zeigen Sie sie sofort an (0 ms), um die Navigation zu beschleunigen"
            }
            "gaming-performance-explorer-mouse-precision" => {
                "Passen Sie die Cursorgeschwindigkeit basierend auf der Bewegungsgeschwindigkeit (Mausbeschleunigung) an. Die meisten Wettkampfspieler deaktivieren dies, um in FPS-Spielen konsistent zielen zu können"
            }
            "gaming-performance-explorer-search" => {
                "Durchsuchen Sie Ihr gesamtes Dateisystem statt nur indizierte Speicherorte. Dies liefert vollständigere Ergebnisse, ist jedoch deutlich langsamer als die indizierte Suche und erhöht die Festplattenaktivität"
            }
            "gaming-performance-mouse-hover-time" => {
                "Steuert, wie lange Sie mit der Maus über ein Element fahren müssen, bevor es aktiviert wird (in Millisekunden). Niedrigere Werte führen dazu, dass Tooltips, Menüs und Hover-Effekte schneller angezeigt werden. Der Standardwert ist 400 ms"
            }
            "gaming-performance-prefetch" => {
                "Laden Sie häufig verwendete Anwendungen und Bootdateien vorab in den Speicher, um den Start zu beschleunigen. Im Allgemeinen für Festplatten und nicht für SSDs empfohlen"
            }
            "gaming-performance-search-webview2" => {
                "Erlauben Sie der Windows-Suche, WebView2 (Edge) zum Rendern von Suchergebnissen zu verwenden. Durch die Deaktivierung werden von SearchHost.exe erzeugte Edge-Prozesse entfernt, wodurch die Ressourcennutzung reduziert wird. Verwendet eine undokumentierte Windows-Feature-Management-Überschreibung (Feature-ID 37926450), die sich in zukünftigen Windows-Updates ändern kann"
            }
            "gaming-performance-svchost-split-threshold" => {
                "Legen Sie den Speicherschwellenwert fest, der bestimmt, wann Windows Dienste in separate svchost.exe-Prozesse aufteilt. Höhere Werte gruppieren mehr Dienste und verringern so die Anzahl der Prozesse. Wählen Sie den Wert aus, der Ihrem System-RAM entspricht"
            }
            "gaming-performance-wallpaper-compression" => {
                "Erlauben Sie Windows, Hintergrundbilder zu komprimieren, um Speicherplatz zu sparen und die Leistung zu verbessern. Betrifft nur Bilder im JPEG-Format."
            }
            "gaming-phone-service" => {
                "Verwaltet den Telefoniestatus auf dem Gerät. Sie können die Funktion problemlos deaktivieren, wenn Sie keine Telefonverbindungsfunktionen nutzen oder keine Anrufe von Ihrem PC aus tätigen"
            }
            "gaming-print-spooler-service" => {
                "Verwaltet an Drucker gesendete Druckaufträge. Wenn Sie keinen Drucker verwenden, stellen Sie ihn auf „Manuell“ oder „Deaktiviert“ ein, um Systemressourcen freizugeben"
            }
            "gaming-remote-access-auto" => {
                "Stellt automatisch eine Verbindung zu Remote-Netzwerken her, wenn Programme auf Remote-Ressourcen verweisen. Sie können die Funktion sicher deaktivieren, wenn Sie die VPN-Funktionen für die automatische Verbindung nicht verwenden"
            }
            "gaming-remote-access-manager" => {
                "Verwaltet VPN- und DFÜ-Verbindungen. Stellen Sie „Manuell“ ein, um die Hintergrundaktivität zu reduzieren und gleichzeitig die VPN-Funktionalität bei Bedarf verfügbar zu halten."
            }
            "gaming-remote-desktop-configuration" => {
                "Verwaltet Remotedesktopdienste und Remotedesktop-bezogene Konfigurationen. Stellen Sie „Manuell“ ein, um die Hintergrundaktivität zu reduzieren und gleichzeitig den Remotedesktop verfügbar zu halten"
            }
            "gaming-remote-desktop-port-redirector" => {
                "Ermöglicht die lokale Geräteumleitung für Remotedesktopverbindungen. Die Deaktivierung ist sicher, wenn Sie während Remotedesktopsitzungen keine lokalen Geräte freigeben müssen"
            }
            "gaming-remote-desktop-services" => {
                "Ermöglicht Benutzern die interaktive Verbindung mit einem Remotecomputer. Stellen Sie „Manuell“ ein, um die Hintergrundaktivität zu reduzieren und gleichzeitig den Remotedesktop verfügbar zu halten."
            }
            "gaming-retail-demo-service" => {
                "Steuert die Geräteaktivität im Einzelhandels-Demomodus. Für PCs kann die Deaktivierung sicher sein, da sie nur zu Anzeigezwecken im Einzelhandel dient"
            }
            "gaming-scheduling-category" => {
                "Weisen Sie eine Planungskategorie mit hoher Priorität zu, um sicherzustellen, dass Spiele eine bevorzugte Systemressourcenzuweisung erhalten"
            }
            "gaming-sensor-data-service" => {
                "Liefert Daten von einer Vielzahl von Sensoren an Anwendungen. Auf Desktop-Systemen ohne Sensorhardware sicher zu deaktivieren"
            }
            "gaming-sensor-monitoring-service" => {
                "Überwacht verschiedene Sensoren wie Umgebungslicht und Ausrichtung. Auf Desktop-Systemen ohne Sensorhardware sicher zu deaktivieren"
            }
            "gaming-smart-card-services" => {
                "Aktiviert die Smartcard-Lesefunktion für die Sicherheitsauthentifizierung. Die Deaktivierung ist sicher, wenn Sie keine physischen Smartcards oder Kartenlesegeräte verwenden"
            }
            "gaming-sms-router-service" => {
                "Leitet SMS-Nachrichten gemäß Regeln weiter. Sie können die Funktion sicher deaktivieren, wenn Sie auf Ihrem PC keine SMS-Funktionen verwenden"
            }
            "gaming-spot-verifier-service" => {
                "Überprüft mögliche Dateisystembeschädigungen. Stellen Sie die Option auf „Manuell“ ein, um bei Bedarf eine Überprüfung zu ermöglichen und gleichzeitig die Hintergrundaktivität zu reduzieren"
            }
            "gaming-storage-sense" => {
                "Geben Sie automatisch Speicherplatz frei, indem Sie temporäre Dateien entfernen, den Papierkorb leeren und Downloads verwalten"
            }
            "gaming-sysmain-service" => {
                "Laden Sie häufig verwendete Anwendungen vorab in den RAM, um schnellere Startzeiten zu erzielen. „Automatisch“ wird für Festplatten oder gemischte Speichersysteme empfohlen; „Manuell“ oder „Deaktiviert“ ist nur für reine SSD-Systeme geeignet"
            }
            "gaming-system-responsiveness" => {
                "Minimieren Sie Störungen durch Hintergrundaufgaben, indem Sie Ihrem aktiven Spiel oder Ihrer Multimedia-Anwendung mehr CPU-Zeit zuweisen"
            }
            "gaming-telemetry-service" => {
                "Sendet Nutzungsdaten und Diagnosen an Microsoft. Durch die Einstellung „Manuell“ oder „Deaktiviert“ wird die Hintergrundnetzwerk- und CPU-Auslastung reduziert"
            }
            "gaming-touch-keyboard-service" => {
                "Verwaltet die Windows-Eingabeerfahrung, einschließlich Touch-Tastatur, Stift-/Stifteingabe, Handschriftfeld, Emoji-Feld (Win+.) und Xbox-Controller-Tastatur. Durch das Deaktivieren werden alle Eingaben über virtuelle/Software-Tastaturen unterbrochen, auf Desktop-Systemen ohne Touchscreen, Stift oder Gamepad ist dies jedoch sicher"
            }
            "gaming-virtualization-based-security" => {
                "Isoliert Teile des Speichers, um das System vor Schwachstellen zu schützen. Das Deaktivieren kann die Spieleleistung verbessern, verringert jedoch die Systemsicherheit"
            }
            "gaming-wallet-service" => {
                "Bietet Wallet-Funktionalität für Zahlungs- und NFC-Szenarien. Die Deaktivierung ist sicher, wenn Sie die Microsoft Wallet-Funktionen nicht nutzen"
            }
            "gaming-win32-priority" => {
                "Konfigurieren Sie, wie Windows CPU-Zeit zwischen Vordergrundanwendungen und Hintergrunddiensten aufteilt"
            }
            "gaming-windows-search-service" => {
                "Indiziert Dateien und Ordner für schnellere Suchergebnisse. Durch die Deaktivierung wird die CPU- und Festplattenaktivität im Hintergrund reduziert, die Outlook-Suche wird jedoch unterbrochen und die Suche im Startmenü und im Datei-Explorer wird langsam oder unzuverlässig"
            }
            "gaming-wmp-network-service" => {
                "Gibt Windows Media Player-Bibliotheken für andere vernetzte Player und Mediengeräte frei. Sie können die Funktion sicher deaktivieren, wenn Sie keine Medien über Ihr Netzwerk freigeben"
            }
            "gaming-xbox-auth-manager" => {
                "Bietet Authentifizierungs- und Autorisierungsdienste für Xbox Live. Die Deaktivierung ist sicher, wenn Sie Xbox Game Pass, Microsoft Store-Spiele oder Xbox-Funktionen nicht verwenden"
            }
            "gaming-xbox-game-dvr" => {
                "Nehmen Sie Gameplay-Clips auf und machen Sie Screenshots mit dem Xbox Game Bar-Overlay. Das Deaktivieren reduziert die CPU-/GPU-Auslastung und kann die Bildraten verbessern"
            }
            "gaming-xbox-game-save" => {
                "Synchronisiert Spielstände mit der Xbox Live-Cloud. Wird nur für Xbox Game Pass- und Microsoft Store-Spiele mit Cloud-Speicherfunktionen benötigt"
            }
            "gaming-xbox-networking" => {
                "Unterstützt Xbox Live-Multiplayer-Netzwerke. Erforderlich für Xbox-Multiplayer-Spiele, jedoch nicht für Steam/Epic/andere Spieleplattformen"
            }
            "MapsUpdateTask" => {
                "Aktualisiert Offline-Kartendaten für die Windows Maps-App. Deaktivieren Sie diese Option, wenn Sie die Karten-App nicht verwenden, um Bandbreite und Speicherplatz zu sparen"
            }
            "MareBackupTask" => {
                "Sichert Microsoft Assisted Recovery-Daten. Deaktivieren Sie diese Option, um die Systemaktivität im Hintergrund zu reduzieren"
            }
            "menu-animation" => {
                "Animiert Menüs, wenn sie angezeigt werden, mithilfe von Fade- oder Slide-Effekten"
            }
            "mouse-shadow" => "Zeigt einen Schatteneffekt unter dem Mauszeiger an",
            "OfficeActionsServer" => {
                "Geplante Aufgabe des Office AI Actions Server. Deaktivieren Sie diese Option, um zu verhindern, dass Office AI im Hintergrund ausgeführt wird"
            }
            "PowerEfficiencyTask" => {
                "Analysiert den Stromverbrauch des Systems und sammelt Daten zur Energieeffizienz. Deaktivieren Sie diese Option, um Telemetrie und Hintergrundanalyse zu reduzieren"
            }
            "ProgramDataUpdaterTask" => {
                "Aktualisiert die Programmkompatibilitätsdatenbank mit Informationen zu installierten Anwendungen. Deaktivieren Sie diese Option, um die Telemetrieerfassung zu reduzieren"
            }
            "show-thumbnails" => {
                "Zeigt Bild- und Dokumentvorschauen anstelle allgemeiner Dateisymbole an"
            }
            "smooth-scroll-listboxes" => {
                "Ermöglicht reibungsloses Scrollen in Listenfeldern statt Springen"
            }
            "SqmTask" => {
                "Sammelt Softwarequalitätsmetriken und Zuverlässigkeitsdaten für Microsoft-Telemetrie. Deaktivieren, um den Datenschutz zu verbessern"
            }
            "StartupAppTask" => {
                "Verfolgt und überwacht Startanwendungen für Telemetrie und Diagnose. Deaktivieren, um die Telemetrie zu reduzieren"
            }
            "taskbar-animations" => {
                "Steuert die Animationseffekte der Taskleiste zum Öffnen, Schließen und Wechseln von Fenstern"
            }
            "taskbar-thumbnails" => {
                "Speichert Miniaturvorschauen von Taskleistenfenstern für eine schnellere Anzeige"
            }
            "translucent-selection" => {
                "Zeigen Sie beim Ziehen ein halbtransparentes Auswahlfeld an, um mehrere Dateien oder Elemente auszuwählen"
            }
            "ui-effects" => "Aktiviert Animationseffekte für Steuerelemente und UI-Elemente",
            "UsbCeipTask" => {
                "Sammelt USB-gerätebezogene Telemetriedaten für das Programm zur Verbesserung der Kundenzufriedenheit. Deaktivieren, um die Telemetrie zu reduzieren"
            }
            "visual-effects-mode" => "Wählen Sie aus, wie Windows visuelle Effekte anzeigt",
            "window-animation" => {
                "Zeigt eine flüssige Animation an, wenn Fenster minimiert oder maximiert werden"
            }
            "WindowsAIRecallConfig" => {
                "Geplante Windows AI-Aufgaben, einschließlich Recall-Konfiguration. Deaktivieren Sie diese Option, um zu verhindern, dass KI-Funktionen im Hintergrund ausgeführt werden"
            }
            "WindowsAIRecallPipeline" => {
                "Windows AI Recall-Pipeline-Aufgabe. Deaktivieren Sie diese Option, um die Verarbeitung der Recall-Pipeline im Hintergrund zu verhindern"
            }
            "window-shadows" => "Zeigt Schatteneffekte unter Fenstern an",

            "windows-app-3d-viewer" => "Sehen Sie sich 3D-Modelle und Animationen an",
            "windows-app-mixed-reality-portal" => "Portal für Windows Mixed Reality-Erlebnisse",
            "windows-app-bing-search" => "Bing-Suchintegration für Windows",
            "windows-app-microsoft-news" => "Microsoft News-App",
            "windows-app-msn-weather" => "Wettervorhersagen und Informationen",
            "windows-app-camera" => "Windows-Kamera-App",
            "windows-app-clipchamp" => "Video-Editor-App",
            "windows-app-alarms-clock" => "Uhr-, Alarm-, Timer- und Stoppuhr-App",
            "windows-app-cortana" => "Der virtuelle Assistent von Microsoft",
            "windows-app-get-help" => "Microsoft-Support-App",
            "windows-app-calculator" => {
                "Rechner-App mit Standard-, Wissenschafts- und Programmiermodus"
            }
            "windows-app-dev-home" => "Entwicklungsumgebung für Windows",
            "windows-app-family-safety" => "Familiensicherheit und Bildschirmzeitmanagement",
            "windows-app-mail-calendar" => "Microsoft Mail- und Kalender-Apps",
            "windows-app-skype" => "Videoanruf- und Messaging-App",
            "windows-app-teams" => "App für Teamzusammenarbeit und Kommunikation",
            "windows-app-feedback-hub" => "App zum Senden von Feedback an Microsoft",
            "windows-app-maps" => "Microsoft Maps-App",
            "windows-app-terminal" => "Moderne Terminalanwendung für Windows",
            "windows-app-office-hub" => "Microsoft 365 Copilot (früher bekannt als Office Hub)",
            "windows-app-outlook" => "Neu gestaltete Outlook-App für Windows",
            "windows-app-paint-3d" => "3D-Modellierungs- und Bearbeitungs-App",
            "windows-app-paint" => "Traditionelle Bildbearbeitungs-App",
            "windows-app-photos" => "App zum Anzeigen und Bearbeiten von Fotos",
            "windows-app-snipping-tool" => "Tool zur Bildschirmaufnahme und Anmerkung",
            "windows-app-people" => "Kontaktverwaltungs-App",
            "windows-app-power-automate" => "Desktop-Automatisierungstool",
            "windows-app-quick-assist" => "Fernunterstützungstool",
            "windows-app-solitaire" => "Spiele der Microsoft Solitaire Collection",
            "windows-app-xbox" => "Xbox-App für Windows",
            "windows-app-xbox-identity-provider" => {
                "Authentifizierungsdienst für Xbox Live und verwandte Microsoft-Gaming-Dienste"
            }
            "windows-app-xbox-game-bar-plugin" => {
                "Erweiterungskomponente für die Xbox Game Bar mit zusätzlicher Funktionalität"
            }
            "windows-app-xbox-live-ingame" => {
                "Kernkomponente für Xbox Live-Dienste innerhalb von Spielen"
            }
            "windows-app-xbox-game-bar" => {
                "Gaming-Overlay mit Bildschirmaufnahme, Leistungsüberwachung und sozialen Funktionen"
            }
            "windows-app-store" => "App-Store für Windows",
            "windows-app-media-player" => "Musik-Player-App",
            "windows-app-movies-tv" => "Videoplayer-App",
            "windows-app-sound-recorder" => "Audioaufnahme-App",
            "windows-app-sticky-notes" => "Notizen-App",
            "windows-app-tips" => "Windows-Tutorial-App",
            "windows-app-todo" => "Aufgabenverwaltungs-App",
            "windows-app-notepad" => "Textbearbeitungs-App",
            "windows-app-phone-link" => "Verbinden Sie Ihr Android- oder iOS-Gerät mit Windows",
            "windows-app-copilot" => {
                "KI-Assistent für Windows, enthält Copilot-Anbieter- und Store-Komponenten"
            }
            "windows-app-client-aix" => {
                "Kernpaket für die Windows-KI-Erfahrung (MicrosoftWindows.Client.AIX)"
            }
            "windows-app-client-copilot" => {
                "System Copilot-Clientpaket (MicrosoftWindows.Client.CoPilot)"
            }
            "windows-app-edge-game-assist" => "Edge Game Assist KI-Overlay für Spiele",
            "windows-app-office-actions-server" => {
                "Office AI Actions Server für automatisierte KI-gestützte Aktionen"
            }
            "windows-app-ai-manager" => "Office AI Manager (aimgr) zur Verwaltung von KI-Diensten",
            "windows-app-writing-assistant" => "KI-Tool „Microsoft Office Writing Assistant“.",
            "windows-app-ai-workloads" => {
                "Windows-KI-Workload-Pakete, einschließlich ONNX Runtime, semantischer Text, Bildsuche, Inhaltsextraktion, Bildschirmbereichserkennung, Texterkennung und Bildinhaltsmoderation"
            }
            "windows-app-copilot-plus-pc" => {
                "KI-Sprach-, Sprach-, Live-Eingabe-, Eingabe- und Dateioperationspakete für mit NPU ausgestattete Copilot+-PCs"
            }
            "windows-app-edge" => "Der Webbrowser von Microsoft",
            "windows-app-onedrive" => "Der Cloud-Speicherdienst von Microsoft",
            "windows-app-onenote" => "Notizen-App von Microsoft",
            "capability-internet-explorer" => "Legacy-Webbrowser",
            "capability-powershell-ise" => "Integrierte PowerShell-Skriptumgebung",
            "capability-quick-assist" => "Fernunterstützungs-App",
            "capability-steps-recorder" => "Bildschirmaufzeichnungstool",
            "capability-windows-media-player" => "Klassischer Mediaplayer",
            "capability-wordpad" => "Rich-Text-Editor",
            "capability-notepad" => "Einfacher Texteditor",
            "capability-paint-legacy" => "Klassische Paint-App",
            "capability-openssh-client" => "Secure Shell-Client für Remoteverbindungen",
            "capability-openssh-server" => "Secure Shell-Server für Remote-Verbindungen",
            "feature-wsl" => {
                "Ermöglicht die native Ausführung binärer ausführbarer Linux-Dateien unter Windows"
            }
            "feature-hyperv-platform" => {
                "Kernvirtualisierungsplattform ohne Hyper-V-Verwaltungstools"
            }
            "feature-hyperv" => "Virtualisierungsplattform zum Ausführen mehrerer Betriebssysteme",
            "feature-hyperv-tools" => "Tools zur Verwaltung virtueller Hyper-V-Maschinen",
            "feature-dotnet35" => "Legacy .NET Framework für ältere Anwendungen",
            "feature-windows-sandbox" => "Isolierte Desktop-Umgebung zum Ausführen von Anwendungen",
            "feature-recall" => "Windows 11-Funktion, die Benutzeraktivitäten aufzeichnet",
            "3D/Mixed Reality" => "3D/Mixed Reality",
            "AI" => "KI",
            "Automation" => "Automatisierung",
            "Bing/Search" => "Bing/Suche",
            "Browser" => "Browser",
            "Browsers" => "Browser",
            "Camera/Media" => "Kamera/Medien",
            "Communication" => "Kommunikation",
            "Development" => "Entwicklung",
            "Games" => "Spiele",
            "Graphics" => "Grafik",
            "Media" => "Medien",
            "Networking" => "Vernetzung",
            "Office" => "Büro",
            "Phone" => "Telefon",
            "Productivity" => "Produktivität",
            "Security" => "Sicherheit",
            "Social" => "Sozial",
            "Store" => "Speichern",
            "Support" => "Unterstützung",
            "System" => "System",
            "System Tools" => "Systemtools",
            "System Utilities" => "Systemdienstprogramme",
            "Utilities" => "Dienstprogramme",
            "Virtualization" => "Virtualisierung",
            "performance_group_0" => "Spielen",
            "performance_group_1" => "Prozessor",
            "performance_group_2" => "Grafik",
            "performance_group_3" => "Netzwerk",
            "performance_group_4" => "Sicherheit",
            "performance_group_5" => "Xbox",
            "performance_group_6" => "Systemdienste",
            "performance_group_7" => "Geplante Aufgaben",
            "performance_group_8" => "Visuelle Effekte",
            "performance_group_9" => "Zugänglichkeit",
            "home_system" => "System",
            "home_processor" => "Prozessor",
            "home_graphics" => "Grafik",
            "home_memory" => "Arbeitsspeicher",
            "home_storage" => "Speicher",
            "home_windows" => "Windows",
            "home_uptime" => "Laufzeit",
            "home_performance" => "Leistung",
            "home_product_name" => "Systemproduktname",
            "home_cpu" => "CPU",
            "home_cpu_model" => "CPU-Modell",
            "home_cores" => "Kerne",
            "home_gpu" => "GPU",
            "home_memory_total" => "Speicher gesamt",
            "home_memory_used" => "Speicher belegt",
            "home_version" => "Version",
            "home_kernel" => "Kernel",
            "home_name" => "Name",
            "home_bios_version" => "BIOS-Version",
            "home_bios_date" => "BIOS-Datum",
            "home_unknown_cpu" => "Unbekannte CPU",
            "home_unknown_pc" => "Unbekannter PC",
            "home_unknown_os" => "Unbekanntes Betriebssystem",
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
            "home_gb_used" => "{:.1} GB belegt",
            "home_tb_total" => "{:.2} TB gesamt",
            "home_tb_used" => "{:.2} TB belegt",
            "home_uptime_fmt" => "{}d {:02}h {:02}m",
            "home_update_status" => "Aktualisierungsstatus",
            "update_available_prefix" => "Update verfügbar:",
            "update_failed_prefix" => "Updateprüfung fehlgeschlagen:",
            "update_newer_version_prefix" => "Eine neuere Version ist verfügbar:",
            "update_ready" => "Bereit",
            "settings_saved" => "Einstellungen gespeichert",
            "update_no_found_title" => "Kein Update gefunden",
            "update_available_title" => "Update verfügbar",
            "update_failed_title" => "Updateprüfung fehlgeschlagen",
            "update_up_to_date" => "Du nutzt bereits die neueste Version.",
            "update_error_check_updates" => "Updateprüfung fehlgeschlagen",
            "update_error_read_response" => "Antwort konnte nicht gelesen werden",
            "update_error_parse_json" => "JSON konnte nicht verarbeitet werden",
            "update_error_no_tag_name" => "Keine tag_name-Antwort erhalten",
            "update_error_download" => "Update konnte nicht heruntergeladen werden",
            "update_error_create_temp_file" => "Temporäre Datei konnte nicht erstellt werden",
            "update_error_write_update_file" => "Update-Datei konnte nicht geschrieben werden",
            "update_error_download_too_small" => "Heruntergeladene Datei ist zu klein",
            "update_error_resolve_current_exe" => "Aktuelle Exe konnte nicht ermittelt werden",
            "update_error_write_update_script" => "Update-Skript konnte nicht geschrieben werden",
            "update_error_launch_updater" => "Updater konnte nicht gestartet werden",
            "update_download_restart" => {
                "Lade die App herunter und starte sie neu, um zu installieren."
            }
            "update_failed" => "Updateprüfung fehlgeschlagen.",
            "update_close" => "Schließen",
            "update_check_again" => "Nochmal prüfen",
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
            "download_subtitle" => "Nützliche Apps aus kuratierten Listen installieren.",
            "download_search" => "Apps suchen...",
            "download_website" => "Website",
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
            "download_none" => "Keine Apps oder Downloads vorhanden.",
            "download_category_0" => "Browser",
            "download_category_1" => "Dokumentenanzeiger",
            "download_category_2" => "Nachrichten, E-Mail & Kalender",
            "download_category_3" => "Online-Speicher & Backup",
            "download_category_4" => "Multimedia",
            "download_category_5" => "Bildbearbeitung",
            "download_category_6" => "Anpassungswerkzeuge",
            "download_category_7" => "Gaming",
            "download_category_8" => "Komprimierung",
            "download_category_9" => "Datei- & Datenträgermanagement",
            "download_category_10" => "Remote-Zugriff",
            "download_category_11" => "Optische Laufwerke",
            "download_category_12" => "Sonstige Werkzeuge",
            "download_category_13" => "Privatsphäre & Sicherheit",
            "download_category_14" => "Entwicklungs-Apps",
            "download_category_15" => "Laufzeiten & Abhängigkeiten",

            "Microsoft EdgeWebView" => "WebView2-Laufzeit für Windows-Anwendungen",
            "Thorium" => "Chromium-basierter Browser mit erweiterten Datenschutzfunktionen",
            "Mercury" => "Compiler-optimierter, privater Firefox-Fork",
            "Mozilla Firefox" => {
                "Beliebter Webbrowser, der für Datenschutz und Anpassungsfähigkeit bekannt ist"
            }
            "Google Chrome" => {
                "Googles Webbrowser mit Synchronisierungs- und Erweiterungsunterstützung"
            }
            "ungoogled-chromium" => "Chromium-basierter Browser mit Datenschutzverbesserungen",
            "Brave" => "Datenschutzorientierter Browser mit integriertem Werbeblocker",
            "Opera" => "Funktionsreicher Webbrowser mit integriertem VPN und Werbeblocker",
            "Opera GX" => "Gaming-orientierte Version von Opera mit einzigartigen Funktionen",
            "Arc Browser" => "Innovativer Browser mit Fokus auf Design und Benutzererfahrung",
            "Tor Browser" => {
                "Datenschutzorientierter Browser, der den Datenverkehr über das Tor-Netzwerk weiterleitet"
            }
            "Vivaldi" => "Hochgradig anpassbarer Browser mit Fokus auf Benutzerkontrolle",
            "Waterfox" => "Firefox-basierter Browser mit Schwerpunkt auf Datenschutz und Anpassung",
            "Zen Browser" => "Datenschutzorientierter Browser mit integriertem Werbeblocker",
            "Mullvad Browser" => {
                "Datenschutzorientierter Browser zur Minimierung von Tracking und Fingerabdrücken"
            }
            "Pale Moon Browser" => {
                "Open Source, Goanna-basierter Webbrowser mit Fokus auf Effizienz und Anpassung"
            }
            "Maxthon" => "Datenschutzorientierter Browser mit integriertem Werbeblocker und VPN",
            "Ablaze Floorp" => "Datenschutzorientierter Browser mit starkem Tracking-Schutz",
            "DuckDuckGo" => "Datenschutzorientierte Suchmaschine mit Browsererweiterung",
            "LibreOffice" => "Kostenlose und Open-Source-Office-Suite",
            "ONLYOFFICE Desktop Editors" => {
                "100 % kostenlose Open-Source-Alternative zu Microsoft Office"
            }
            "PDFgear" => {
                "Lesen, bearbeiten, konvertieren, zusammenführen und signieren Sie PDF-Dateien auf allen Geräten, völlig kostenlos und ohne Anmeldung."
            }
            "Foxit PDF Reader" => "Leichter PDF-Reader mit erweiterten Funktionen",
            "SumatraPDF" => {
                "PDF, eBook (epub, mobi), Comic (cbz/cbr), DjVu, XPS, CHM, Bildbetrachter für Windows"
            }
            "OpenOffice" => {
                "Eingestellte Open-Source-Office-Suite. Aktives Nachfolgeprojekt ist LibreOffice"
            }
            "Adobe Acrobat Reader DC" => "PDF-Reader und -Editor",
            "Evernote" => "Notizen-App",
            "CherryTree" => "Hierarchische Notizanwendung mit Rich-Text- und Syntaxhervorhebung",
            "Okular" => "Universeller Dokumentenbetrachter, der PDF, E-Books und mehr unterstützt",
            "PDF24 Creator" => "Kostenloser PDF-Ersteller und -Konverter",
            "Telegram Desktop" => "Instant-Messaging- und Sprachanruf-App",
            "WhatsApp" => "Instant-Messaging- und Sprachanruf-App",
            "Zoom Workplace" => "Videokonferenz- und Messaging-Plattform",
            "Discord" => "Sprach-, Video- und Textkommunikationsdienst",
            "Pidgin" => "Multiprotokoll-Instant-Messaging-Client",
            "Mozilla Thunderbird" => "Kostenlose E-Mail-Bewerbung",
            "eM Client" => "E-Mail-Client mit Kalender, Aufgaben und Chat",
            "Proton Mail" => "Sicherer E-Mail-Dienst mit Ende-zu-Ende-Verschlüsselung",
            "Trillian" => "Instant-Messaging-Anwendung",
            "Google Drive" => "Cloud-Speicher- und Dateisynchronisierungsdienst",
            "Dropbox" => {
                "Datei-Hosting-Dienst, der Cloud-Speicher, Dateisynchronisierung und persönliche Cloud bietet"
            }
            "SugarSync" => {
                "Greifen Sie automatisch auf Ihre Fotos, Videos und Dateien in jedem Ordner zu und teilen Sie sie"
            }
            "Nextcloud" => {
                "Greifen Sie zu Hause und in Ihrem Unternehmen auf Ihre Dateien, Kalender, Kontakte, Kommunikation und mehr zu, teilen Sie sie und schützen Sie sie"
            }
            "Proton Drive" => "Sicherer Cloud-Speicher mit Ende-zu-Ende-Verschlüsselung",
            "FreeFileSync" => "Open-Source-Tool zum Vergleichen und Synchronisieren von Ordnern",
            "Hekasoft Backup & Restore" => {
                "Die komplette kostenlose Lösung für Browser-Backup und -Verwaltung"
            }
            "VLC media player" => "Open-Source-Multimedia-Player und Framework",
            "iTunes" => "Mediaplayer und Bibliothek",
            "AIMP" => "Audioplayer mit Unterstützung für verschiedene Formate",
            "foobar2000" => "Erweiterter Audioplayer für Windows",
            "MusicBee" => "Musikmanager und Spieler",
            "Audacity" => "Audio-Editor und -Recorder",
            "GOM Player" => "Mediaplayer für Windows",
            "Spotify" => "Musik-Streaming-Dienst",
            "MediaMonkey" => "Medienmanager und Player",
            "HandBrake" => "Open-Source-Videotranscoder",
            "OBS Studio" => {
                "Kostenlose und Open-Source-Software für Videoaufzeichnung und Live-Streaming"
            }
            "Streamlabs OBS" => {
                "Auf OBS basierende Streaming-Software mit zusätzlichen Funktionen für Streamer"
            }
            "MPC-BE" => "Media Player Classic – Black Edition",
            "K-Lite Codec Pack (Mega)" => "Sammlung von Codecs und zugehörigen Tools",
            "CapCut" => "Videoeditor",
            "PotPlayer64" => "Umfangreicher Multimedia-Player für Windows",
            "kdenlive" => "Kostenlose und quelloffene Videobearbeitungssoftware",
            "MediaInfo" => "Tool zur Anzeige technischer Informationen für Multimediadateien",
            "fre:ac - free audio converter" => "Kostenloser Audiokonverter und CD-Ripper",
            "SMPlayer" => {
                "Media Player mit integrierten Codecs, der praktisch alle Video- und Audioformate abspielen kann"
            }
            "Shotcut" => "Kostenloser, plattformübergreifender Open-Source-Videoeditor",
            "LosslessCut" => {
                "Plattformübergreifende FFmpeg-GUI für schnelles, verlustfreies Video-/Audio-Trimmen"
            }
            "FxSound" => "Audio-Enhancer zur Verbesserung der Klangqualität unter Windows",
            "IrfanView64" => "Schneller und kompakter Bildbetrachter und Konverter",
            "Krita" => "Digitale Mal- und Illustrationssoftware",
            "Blender" => "3D-Erstellungssuite",
            "Paint.NET" => "Bild- und Fotobearbeitungssoftware",
            "GIMP" => "GNU-Bildbearbeitungsprogramm",
            "XnViewMP" => "Bildbetrachter, Browser und Konverter",
            "XnView" => "Bildbetrachter, Browser und Konverter (klassische Version)",
            "Inkscape" => "Vektorgrafik-Editor",
            "Greenshot" => "Screenshot-Tool mit Anmerkungsfunktionen",
            "ShareX" => "Tool zur Bildschirmaufnahme, Dateifreigabe und Produktivität",
            "Flameshot" => "Leistungsstarke und dennoch einfach zu bedienende Screenshot-Software",
            "FastStone Image Viewer" => "Bildbrowser, Konverter und Editor",
            "Nilesoft Shell" => "Tool zur Anpassung des Windows-Kontextmenüs",
            "StartAllBack (Win 11)" => "Anpassung des Startmenüs und der Taskleiste von Windows 11",
            "StartIsBack++ (Win 10)" => "Windows 10-Startmenü und Taskleistenanpassung",
            "Open-Shell" => "Klassisches Startmenü für Windows",
            "Windhawk" => "Anpassungsplattform für Windows",
            "Lively Wallpaper" => {
                "Kostenlose und Open-Source-Anwendung für animierte Desktop-Hintergrundbilder"
            }
            "Sucrose Wallpaper Engine" => {
                "Kostenlose und Open-Source-Anwendung für animierte Desktop-Hintergrundbilder"
            }
            "Rainmeter" => "Desktop-Anpassungstool für Windows",
            "ExplorerPatcher" => "Dienstprogramm, das das Windows Explorer-Erlebnis verbessert",
            "John's Background Switcher" => {
                "Ändert Ihren Desktop-Hintergrund automatisch in regelmäßigen Abständen"
            }
            "Microsoft PowerToys" => {
                "Eine Reihe von Dienstprogrammen für Power-User zur Optimierung und Optimierung ihres Windows-Erlebnisses"
            }
            "Nexus" => "Das fortschrittliche Dockingsystem für Windows",
            "AutoHotkey v2" => {
                "Kostenlose Skriptsprache zur Makroerstellung und Automatisierung (v2, aktuell)"
            }
            "Steam" => "Digitale Vertriebsplattform für PC-Gaming",
            "Epic Games Launcher" => "Digitale Vertriebsplattform für PC-Gaming",
            "7-Zip" => "Open-Source-Dateiarchivierer mit hoher Komprimierungsrate",
            "WinRAR archiver" => "Dateiarchivierer mit hoher Komprimierungsrate",
            "PeaZip" => {
                "Kostenloses Dienstprogramm zur Dateiarchivierung. Öffnen und extrahieren Sie RAR-, TAR-, ZIP-Dateien und mehr"
            }
            "WinDirStat" => "Betrachter und Bereinigungstool für Festplattennutzungsstatistiken",
            "WizTree" => "Speicherplatzanalysator mit extrem schnellem Scannen",
            "TreeSize Free" => "Speicherplatzmanager",
            "Everything" => "Suchen Sie Dateien und Ordner sofort nach Namen",
            "TeraCopy" => "Kopieren Sie Dateien schneller und sicherer",
            "File Converter" => "Batch-Dateikonverter für Windows",
            "Crystal Disk Info" => "Dienstprogramm zur Überwachung des Festplattenzustands",
            "Bulk Rename Utility" => "Software zum Umbenennen von Dateien für Windows",
            "IObit Unlocker" => {
                "Tool zum Entsperren von Dateien, die von anderen Prozessen verwendet werden"
            }
            "HiBit Uninstaller" => {
                "Deinstallieren Sie hartnäckige Software, Windows-Apps und Browsererweiterungen vollständig"
            }
            "SanDisk Dashboard" => "Laufwerksverwaltungstool für SanDisk SSDs und Flash-Laufwerke",
            "Rufus" => "Dienstprogramm zum Erstellen bootfähiger USB-Flash-Laufwerke",
            "Advanced Renamer" => {
                "Dienstprogramm zum Umbenennen von Stapeldateien mit erweiterten Optionen"
            }
            "RustDesk" => "Schnelle Open-Source-Fernzugriffs- und Support-Software",
            "AnyDesk" => "Remote-Desktop-Software für Fernzugriff und Support",
            "TeamViewer" => {
                "Fernsteuerung, Desktop-Sharing, Online-Meetings, Webkonferenzen und Dateiübertragung"
            }
            "UltraViewer" => {
                "Hilft Ihnen, den Computer Ihres Partners zu steuern, um ihn zu unterstützen, als ob Sie vor seinem Bildschirm sitzen würden"
            }
            "RealVNC Server" => "Fernzugriffssoftware",
            "RealVNC Viewer" => "Fernzugriffssoftware",
            "Chrome Remote Desktop" => "Fernzugriff auf Ihren Computer über den Chrome-Browser",
            "Parsec" => {
                "Remote-Desktop neu gedacht. Sicherer, flexibler und müheloser Zugriff auf alles, was Sie tun, jederzeit und überall"
            }
            "Parsec Virtual Display Driver" => {
                "Virtueller Anzeigetreiber für Parsec Remote Desktop"
            }
            "Parsec Virtual USB Driver" => "Virtueller USB-Treiber für Parsec Remote Desktop",
            "InputLeap" => {
                "Open-Source-KVM-Software zur gemeinsamen Nutzung von Maus und Tastatur zwischen Computern"
            }
            "ImgBurn" => "Leichte Anwendung zum Brennen von CDs/DVDs/HD-DVDs/Blu-rays",
            "AnyBurn" => "Leichte Software zum Brennen von CDs/DVDs/Blu-rays",
            "CDBurnerXP" => "Kostenlose Software zum Brennen von CDs/DVDs/Blu-rays",
            "CCleaner" => "Systemoptimierungs- und Reinigungstool",
            "Snappy Driver Installer Origin" => {
                "Treiberinstallationsprogramm und -aktualisierungsprogramm"
            }
            "Wise Disk Cleaner" => {
                "Kostenloses Tool zur Datenträgerbereinigung und -defragmentierung"
            }
            "Wise Registry Cleaner" => "Tool zur Bereinigung und Optimierung der Registry",
            "UniGetUI" => {
                "Universelle Paketmanager-Schnittstelle, die WinGet, Chocolatey und mehr unterstützt"
            }
            "OpenRGB" => "Open-Source-Software zur Steuerung der RGB-Beleuchtung",
            "OpenAudible" => "Hörbuch-Manager und Konverter für Audible-Dateien",
            "NAPS2" => "Anwendung zum Scannen von Dokumenten mit OCR-Unterstützung",
            "IObit Uninstaller" => {
                "Deinstallieren Sie unerwünschte Software, Windows-Apps und Browser-Plug-ins vollständig"
            }
            "Revo Uninstaller" => {
                "Revo Uninstaller hilft Ihnen, Software zu deinstallieren und unerwünschte Programme einfach zu entfernen."
            }
            "Malwarebytes" => "Anti-Malware-Software für Windows",
            "Malwarebytes AdwCleaner" => "Tool zum Entfernen von Adware für Windows",
            "Windows Firewall Control" => "Malwarebytes Windows Firewall Control-Anwendung",
            "OnionShare" => {
                "Teilen Sie Dateien sicher und anonym, hosten Sie Websites und chatten Sie über das Tor-Netzwerk"
            }
            "Sniffnet" => "Netzwerküberwachungstool zur Analyse Ihres Internetverkehrs",
            "TeleGuard" => "Sichere Messaging-App mit Ende-zu-Ende-Verschlüsselung",
            "Python 3.13" => "Python-Programmiersprache",
            "Notepad++" => "Kostenloser Quellcode-Editor und Notepad-Ersatz",
            "WinSCP" => "Kostenloser SFTP-, SCP-, Amazon S3-, WebDAV- und FTP-Client",
            "PuTTY" => "Kostenloser SSH- und Telnet-Client",
            "WinMerge" => "Open-Source-Tool zur Differenzierung und Zusammenführung",
            "Eclipse IDE for Java" => "Java IDE und Entwicklungsplattform",
            "Microsoft Visual Studio Code" => {
                "Code-Editor mit Unterstützung für Entwicklungsvorgänge"
            }
            "Git" => "Verteiltes Versionskontrollsystem",
            "GitHub Desktop" => "GitHub-Desktop-Client",
            "Microsoft .NET Runtime 3.1" => ".NET Runtime 3.1 zum Ausführen von Anwendungen",
            "Microsoft .NET Runtime 5.0" => ".NET Runtime 5.0 zum Ausführen von Anwendungen",
            "Microsoft .NET Runtime 6.0" => ".NET Runtime 6.0 LTS zum Ausführen von Anwendungen",
            "Microsoft .NET Runtime 7.0" => ".NET Runtime 7.0 zum Ausführen von Anwendungen",
            "Microsoft .NET Runtime 8.0" => ".NET Runtime 8.0 LTS zum Ausführen von Anwendungen",
            ".NET Framework 4.8.1" => ".NET Framework-Entwicklerpaket",
            "DirectX Runtime" => {
                "DirectX-Laufzeitkomponenten zum Ausführen von Spielen und Multimediaanwendungen"
            }
            "Java Runtime Environment" => {
                "Java-Laufzeitumgebung zum Ausführen von Java-Anwendungen"
            }
            "Visual C++ 2005 (x86)" => "Visual C++ 2005-Laufzeitkomponenten",
            "Visual C++ 2005 (x64)" => "Visual C++ 2005-Laufzeitkomponenten",
            "Visual C++ 2008 (x86)" => "Visual C++ 2008-Laufzeitkomponenten",
            "Visual C++ 2008 (x64)" => "Visual C++ 2008-Laufzeitkomponenten",
            "Visual C++ 2010 (x86)" => "Visual C++ 2010-Laufzeitkomponenten",
            "Visual C++ 2010 (x64)" => "Visual C++ 2010-Laufzeitkomponenten",
            "Visual C++ 2012 (x86)" => "Visual C++ 2012-Laufzeitkomponenten",
            "Visual C++ 2012 (x64)" => "Visual C++ 2012-Laufzeitkomponenten",
            "Visual C++ 2013 (x86)" => "Visual C++ 2013-Laufzeitkomponenten",
            "Visual C++ 2013 (x64)" => "Visual C++ 2013-Laufzeitkomponenten",
            "Visual C++ 2015-2022 (x86)" => "Visual C++ 2015-2022-Laufzeitkomponenten",
            "Visual C++ 2015-2022 (x64)" => "Visual C++ 2015-2022-Laufzeitkomponenten",
            "Tabby" => "SSH-Terminal- und Verbindungsmanager",
            "Riot Games Launcher" => {
                "Riot-Client-Launcher (manuelle Anmeldung kann erforderlich sein)"
            }
            "Helium Browser" => "Datenschutzorientierter Chromium-Browser",
            "DataGrip" => "JetBrains SQL-IDE",
            "Zed" => "Hochleistungs-Code-Editor",
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
                "Installierte Apps, Capabilities und optionale Features werden gescannt."
            }
            "debloater_none" => "Keine Pakete vorhanden.",
            "debloater_cannot_reinstall" => "Kann nicht erneut installiert werden",
            "debloater_meta" => "Paket: {}\nKategorie: {}\nGruppe: {}",
            "debloater_confirm_install_title" => "Installation bestätigen",
            "debloater_confirm_remove_title" => "Entfernen bestätigen",
            "debloater_confirm_install_desc" => "Diese Apps werden installiert:",
            "debloater_confirm_remove_desc" => "Diese Apps werden entfernt:",
            "debloater_confirm_install_btn" => "Installation bestätigen",
            "debloater_confirm_remove_btn" => "Entfernen bestätigen",
            "debloater_cancel" => "Abbrechen",
            "debloater_installed" => "Installiert",
            "debloater_not_installed" => "Nicht installiert",
            "debloater_result_install" => "Installiert: {}  Fehlgeschlagen: {}",
            "debloater_result_remove" => "Entfernt: {}  Fehlgeschlagen: {}",
            "debloater_tab_0" => "Windows-Apps",
            "debloater_tab_1" => "Capabilities",
            "debloater_tab_2" => "Optionale Features",
            "processes_title" => "Prozesse",
            "processes_subtitle" => "Prozesse, Affinität und Priorität prüfen.",
            "processes_refresh" => "Aktualisieren",
            "processes_active_only" => "Nur aktive",
            "processes_visible" => "Sichtbar:",
            "processes_total_cpu" => "Gesamt-CPU:",
            "processes_refreshing" => "Prozessliste wird aktualisiert...",
            "processes_waiting_first" => "Warte auf erste Aktualisierung...",
            "processes_reload_queued" => "Neuladen eingeplant",
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
            "processes_realtime_title" => "Echtzeit-Priorität festlegen?",
            "processes_realtime_warn" => {
                "Echtzeit kann die Windows-Reaktionsfähigkeit einfrieren. Nur fortfahren, wenn du das Risiko verstehst."
            }
            "processes_cancel" => "Abbrechen",
            "processes_confirm" => "Bestätigen",
            "processes_affinity_title" => "CPU-Affinität - {} (PID {})",
            "processes_affinity_mask" => "Affinitäts-Bitmaske (hex): {}",
            "processes_invert" => "Invertieren",
            "processes_clear" => "Löschen",
            "processes_close" => "Schließen",
            "processes_apply" => "Anwenden",
            "processes_last_refresh" => "Letzte Aktualisierung: {}s her",
            "processes_priority_unknown" => "Unbekannt",
            "processes_priority_idle" => "Leerlauf",
            "processes_priority_below_normal" => "Unter Normal",
            "processes_priority_normal" => "Normal",
            "processes_priority_above_normal" => "Über Normal",
            "processes_priority_high" => "Hoch",
            "processes_priority_realtime" => "Echtzeit",
            "processes_priority_background" => "Hintergrund: 4 (wenig I/O und CPU)",
            "processes_priority_low" => "Niedrig",
            "processes_priority_always_below" => "Unter",
            "processes_priority_always_above" => "Über",
            "processes_status_running" => "Läuft",
            "processes_status_sleeping" => "Schläft",
            "processes_status_idle" => "Leerlauf",
            "processes_status_zombie" => "Zombie",
            "processes_status_stopped" => "Angehalten",
            "processes_status_tracing" => "Tracing",
            "processes_status_dead" => "Tot",
            "processes_status_wakekill" => "Wakekill",
            "processes_status_waking" => "Wacht",
            "processes_status_lockblocked" => "Gesperrt",
            "processes_status_parked" => "Geparkt",
            "processes_status_unknown" => "Unbekannt",
            "processes_process_scan_failed" => "Prozessscan fehlgeschlagen",
            "processes_openprocess_failed" => "OpenProcess fehlgeschlagen für PID {}: {}",
            "processes_get_affinity_failed" => {
                "GetProcessAffinityMask fehlgeschlagen für PID {}: {}"
            }
            "processes_invalid_system_mask" => "Ungültige System-Affinitätsmaske für PID {}",
            "processes_invalid_affinity_mode" => "Ungültiger Affinitätsmodus",
            "processes_set_priority_failed" => "SetPriorityClass fehlgeschlagen für PID {}",
            "processes_set_io_failed" => "SetProcessInformation(I/O) fehlgeschlagen für PID {}",
            "processes_set_affinity_failed" => "SetProcessAffinityMask fehlgeschlagen für PID {}",
            "processes_perfoptions_open" => {
                "PerfOptions-Schlüssel konnte nicht erstellt/geöffnet werden"
            }
            "processes_perfoptions_write_cpu" => "CpuPriorityClass konnte nicht geschrieben werden",
            "processes_perfoptions_write_io" => "IoPriority konnte nicht geschrieben werden",
            "processes_invalid_priority_level" => "Ungültige Prioritätsstufe",
            "latency_title" => "Latenz",
            "latency_subtitle" => "USB-Latenz und Gerätetopologie analysieren.",
            "latency_button" => "USB-Latenz analysieren",
            "latency_analyzing" => "Analysiere...",
            "latency_starting" => "Analyse wird gestartet...",
            "latency_topology" => "USB-Topologie wird analysiert...",
            "latency_scanning" => {
                "PnP-Geräte, Controller-Kette, MSI und Energieeinstellungen werden gescannt."
            }
            "latency_begin" => "Klicke auf 'USB-Latenz analysieren', um zu beginnen.",
            "latency_loading_fail" => "USB-Latenzanalyse fehlgeschlagen",
            "latency_admin" => "Bitte als Administrator ausführen.",
            "latency_error_title" => "FEHLER - USB-LATENZANALYSE FEHLGESCHLAGEN",
            "latency_failed" => "USB-Latenzanalyse fehlgeschlagen",
            "latency_progress_power" => "Stromoptionen werden geprüft...",
            "latency_progress_controllers" => "USB-Controller werden gescannt...",
            "latency_progress_usb_registry_tree" => "USB-Registry-Baum wird gelesen...",
            "latency_progress_inputs" => "Eingabegeräte werden gesucht...",
            "latency_progress_hubs" => "Geräte zu Root-Hubs werden verfolgt...",
            "latency_progress_verify" => "Topologie und Energiehinweise werden geprüft...",
            "latency_progress_report" => "Bericht wird erstellt...",
            _ => "",
        },
        Language::English => match key {
            "nav_title" => "Navigation",
            "nav_subtitle" => "Winchisel control center",
            "home" => "Home",
            "debloater" => "Debloater",
            "downloads" => "Apps & Downloads",
            "performance" => "Performance",
            "processes" => "Processes",
            "latency" => "Latency",
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

            "gaming-game-mode" => {
                "Optimize your PC for gaming by turning things off in the background"
            }
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
                "Select a DNS server for all network adapters. Changes apply to each adapter in your system (WiFi and Ethernet). Use “Automatic” to restore your default ISP/Router DNS"
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
                "Preload frequently used applications into RAM for faster boot times. Automatic is recommended for hard drives or mixed storage systems; “Manual” or “Disabled” is only suitable for pure SSD systems"
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
            "window-animation" => {
                "Displays a smooth animation when windows are minimized or maximized"
            }
            "taskbar-animations" => {
                "Controls the taskbar animation effects for opening, closing, and switching windows"
            }
            "enable-peek" => {
                "Allows you to view the desktop when you hover over the Show Desktop button"
            }
            "menu-animation" => "Animates menus when displayed using fade or slide effects",
            "fade-tooltip" => "Animates tooltips when displayed using fade or slide effects",
            "fade-menu-items" => "Hides menu items after selection before closing the menu",
            "taskbar-thumbnails" => {
                "Saves thumbnail previews of taskbar windows for faster viewing"
            }
            "mouse-shadow" => "Displays a shadow effect under the mouse pointer",
            "window-shadows" => "Displays shadow effects under windows",
            "show-thumbnails" => {
                "Displays image and document previews instead of generic file icons"
            }
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
            "Sucrose Wallpaper Engine" => {
                "Free and open-source animated desktop wallpaper application"
            }
            "Rainmeter" => "Desktop customization tool for Windows",
            "ExplorerPatcher" => "Utility that enhances the Windows Explorer experience",
            "John's Background Switcher" => {
                "Automatically changes your desktop wallpaper at regular intervals"
            }
            "Microsoft PowerToys" => {
                "Set of utilities for power users to tune and streamline their Windows experience"
            }
            "Nexus" => "The advanced docking system for Windows",
            "AutoHotkey v2" => {
                "Free macro-creation and automation scripting language (v2, current)"
            }
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
            "InputLeap" => {
                "Open-source KVM software for sharing mouse and keyboard between computers"
            }
            "ImgBurn" => "Lightweight CD / DVD / HD DVD / Blu-ray burning application",
            "AnyBurn" => "Lightweight CD/DVD/Blu-ray burning software",
            "CDBurnerXP" => "Free CD/DVD/Blu-ray burning software",
            "CCleaner" => "System optimization and cleaning tool",
            "Snappy Driver Installer Origin" => "Driver installer and updater",
            "Wise Disk Cleaner" => "Free Disk Cleanup and Defragment Tool",
            "Wise Registry Cleaner" => "Registry cleaning and optimization tool",
            "UniGetUI" => {
                "Universal package manager interface supporting WinGet, Chocolatey, and more"
            }
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
            "processes_active_only" => "Active only",
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
        },
    }
}
