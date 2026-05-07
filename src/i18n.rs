use crate::Language;

pub fn t(lang: Language, key: &str) -> &'static str {
    match lang {
        Language::German => match key {
            "nav_title" => "Navigation",
            "nav_subtitle" => "Winchisel Steuerzentrale",
            "home" => "Start",
            "debloater" => "Desinstalador",
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
                "Gère l'expérience de saisie Windows, y compris le clavier tactile, l'entrée stylet, le panneau d'écriture manuscrite, le panneau Emoji (Win+.) et le clavier de la manette Xbox. La désactivation coupe toutes les saisies via les claviers virtuels/logiciels, mais reste sûre sur les systèmes de bureau sans écran tactile, stylet ou manette"
            }
            "gaming-virtualization-based-security" => {
                "Isole certaines parties de la mémoire afin de protéger le système contre les vulnérabilités. La désactivation peut améliorer les performances en jeu, mais réduit la sécurité du système"
            }
            "gaming-wallet-service" => {
                "Fournit des fonctions de portefeuille pour les scénarios de paiement et de NFC. La désactivation est sans risque si vous n'utilisez pas les fonctions Microsoft Wallet"
            }
            "gaming-win32-priority" => {
                "Configure la répartition du temps CPU entre les applications au premier plan et les services en arrière-plan"
            }
            "gaming-windows-search-service" => {
                "Indexe les fichiers et dossiers pour des résultats de recherche plus rapides. La désactivation réduit l'activité CPU et disque en arrière-plan, mais casse la recherche Outlook et ralentit ou rend peu fiable la recherche dans le menu Démarrer et l'Explorateur de fichiers"
            }
            "gaming-wmp-network-service" => {
                "Partage les bibliothèques de Windows Media Player avec d'autres lecteurs et appareils multimédias du réseau. Vous pouvez désactiver cette fonction en toute sécurité si vous ne partagez pas de médias sur votre réseau"
            }
            "gaming-xbox-auth-manager" => {
                "Fournit les services d'authentification et d'autorisation pour Xbox Live. La désactivation est sûre si vous n'utilisez pas Xbox Game Pass, les jeux du Microsoft Store ou les fonctionnalités Xbox"
            }
            "gaming-xbox-game-dvr" => {
                "Enregistre des clips de jeu et prend des captures d'écran via la barre de jeu Xbox. La désactivation réduit l'utilisation CPU/GPU et peut améliorer les fréquences d'images"
            }
            "gaming-xbox-game-save" => {
                "Synchronise les sauvegardes de jeu avec le cloud Xbox Live. Nécessaire uniquement pour Xbox Game Pass et les jeux du Microsoft Store disposant de la sauvegarde cloud"
            }
            "gaming-xbox-networking" => {
                "Prend en charge les réseaux multijoueurs Xbox Live. Requis pour les jeux multijoueurs Xbox, mais pas pour Steam/Epic/autres plateformes de jeu"
            }
            "MapsUpdateTask" => {
                "Met à jour les données cartographiques hors ligne pour l'application Cartes Windows. Désactivez cette option si vous n'utilisez pas l'application Cartes afin d'économiser la bande passante et l'espace disque"
            }
            "MareBackupTask" => {
                "Sauvegarde les données Microsoft Assisted Recovery. Désactivez cette option pour réduire l'activité système en arrière-plan"
            }
            "menu-animation" => {
                "Anime les menus lorsqu'ils apparaissent à l'aide d'effets de fondu ou de glissement"
            }
            "mouse-shadow" => "Affiche un effet d'ombre sous le pointeur de la souris",
            "OfficeActionsServer" => {
                "Tâche planifiée du serveur Office AI Actions. Désactivez cette option pour empêcher Office AI de s'exécuter en arrière-plan"
            }
            "PowerEfficiencyTask" => {
                "Analyse la consommation d'énergie du système et collecte des données sur l'efficacité énergétique. Désactivez cette option pour réduire la télémétrie et l'analyse en arrière-plan"
            }
            "ProgramDataUpdaterTask" => {
                "Met à jour la base de compatibilité des programmes avec les informations sur les applications installées. Désactivez cette option pour réduire la collecte de télémétrie"
            }
            "show-thumbnails" => {
                "Affiche des aperçus d'images et de documents au lieu d'icônes de fichiers génériques"
            }
            "smooth-scroll-listboxes" => {
                "Active le défilement fluide dans les listes au lieu d'un défilement par à-coups"
            }
            "SqmTask" => {
                "Collecte des métriques de qualité logicielle et des données de fiabilité pour la télémétrie Microsoft. Désactivez-le pour améliorer la confidentialité"
            }
            "StartupAppTask" => {
                "Suit et surveille les applications au démarrage pour la télémétrie et le diagnostic. Désactivez-le pour réduire la télémétrie"
            }
            "taskbar-animations" => {
                "Contrôle les effets d'animation de la barre des tâches lors de l'ouverture, de la fermeture et du changement de fenêtres"
            }
            "taskbar-thumbnails" => {
                "Met en cache les miniatures des fenêtres de la barre des tâches pour un affichage plus rapide"
            }
            "translucent-selection" => {
                "Affiche un rectangle de sélection semi-transparent lors du glissement pour sélectionner plusieurs fichiers ou éléments"
            }
            "ui-effects" => {
                "Active les effets d'animation pour les contrôles et les éléments de l'interface"
            }
            "UsbCeipTask" => {
                "Collecte des données de télémétrie liées aux périphériques USB pour le programme d'amélioration de l'expérience utilisateur. Désactivez-le pour réduire la télémétrie"
            }
            "visual-effects-mode" => {
                "Choisissez la manière dont Windows affiche les effets visuels"
            }
            "window-animation" => {
                "Affiche une animation fluide lorsque les fenêtres sont minimisées ou maximisées"
            }
            "WindowsAIRecallConfig" => {
                "Tâches Windows AI planifiées, y compris la configuration de Recall. Désactivez cette option pour empêcher l'exécution en arrière-plan des fonctions d'IA"
            }
            "WindowsAIRecallPipeline" => {
                "Tâche du pipeline Windows AI Recall. Désactivez cette option pour empêcher le traitement en arrière-plan du pipeline Recall"
            }
            "window-shadows" => "Affiche des ombres sous les fenêtres",

            "windows-app-3d-viewer" => "Visionneuse de modèles et d'animations 3D",
            "windows-app-mixed-reality-portal" => {
                "Portail pour les expériences Windows Mixed Reality"
            }
            "windows-app-bing-search" => "Intégration de la recherche Bing pour Windows",
            "windows-app-microsoft-news" => "Application Microsoft News",
            "windows-app-msn-weather" => "Prévisions et informations météo",
            "windows-app-camera" => "Application Appareil photo Windows",
            "windows-app-clipchamp" => "Application de montage vidéo",
            "windows-app-alarms-clock" => "Application Horloge, alarmes, minuteur et chronomètre",
            "windows-app-cortana" => "L'assistant virtuel de Microsoft",
            "windows-app-get-help" => "Application d'assistance Microsoft",
            "windows-app-calculator" => {
                "Application Calculatrice avec modes standard, scientifique et programmeur"
            }
            "windows-app-dev-home" => "Environnement de développement pour Windows",
            "windows-app-family-safety" => "Sécurité familiale et gestion du temps d'écran",
            "windows-app-mail-calendar" => "Applications Courrier et Calendrier Microsoft",
            "windows-app-skype" => "Application d'appel vidéo et de messagerie",
            "windows-app-teams" => "Application de collaboration et de communication d'équipe",
            "windows-app-feedback-hub" => "Application pour envoyer des commentaires à Microsoft",
            "windows-app-maps" => "Application Cartes Microsoft",
            "windows-app-terminal" => "Application Terminal moderne pour Windows",
            "windows-app-office-hub" => "Microsoft 365 Copilot (anciennement Office Hub)",
            "windows-app-outlook" => "Application Outlook repensée pour Windows",
            "windows-app-paint-3d" => "Application de modélisation et d'édition 3D",
            "windows-app-paint" => "Application d'édition d'images classique",
            "windows-app-photos" => "Application d'affichage et de retouche de photos",
            "windows-app-snipping-tool" => "Outil de capture et d'annotation d'écran",
            "windows-app-people" => "Application de gestion des contacts",
            "windows-app-power-automate" => "Outil d'automatisation de bureau",
            "windows-app-quick-assist" => "Outil d'assistance à distance",
            "windows-app-solitaire" => "Jeux de la collection Microsoft Solitaire",
            "windows-app-xbox" => "Application Xbox pour Windows",
            "windows-app-xbox-identity-provider" => {
                "Service d'authentification pour Xbox Live et les services de jeu Microsoft associés"
            }
            "windows-app-xbox-game-bar-plugin" => {
                "Composant d'extension pour la barre de jeu Xbox avec des fonctionnalités supplémentaires"
            }
            "windows-app-xbox-live-ingame" => {
                "Composant principal pour les services Xbox Live dans les jeux"
            }
            "windows-app-xbox-game-bar" => {
                "Overlay de jeu avec capture d'écran, suivi des performances et fonctions sociales"
            }
            "windows-app-store" => "Boutique d'applications pour Windows",
            "windows-app-media-player" => "Application Lecteur multimédia",
            "windows-app-movies-tv" => "Application Lecteur vidéo",
            "windows-app-sound-recorder" => "Application d'enregistrement audio",
            "windows-app-sticky-notes" => "Application de notes",
            "windows-app-tips" => "Application d'astuces Windows",
            "windows-app-todo" => "Application de gestion des tâches",
            "windows-app-notepad" => "Application d'édition de texte",
            "windows-app-phone-link" => "Connectez votre appareil Android ou iOS à Windows",
            "windows-app-copilot" => {
                "Assistant IA pour Windows, comprenant les composants du fournisseur Copilot et du Store"
            }
            "windows-app-client-aix" => {
                "Package principal pour l'expérience IA Windows (MicrosoftWindows.Client.AIX)"
            }
            "windows-app-client-copilot" => {
                "Package client Copilot système (MicrosoftWindows.Client.CoPilot)"
            }
            "windows-app-edge-game-assist" => "Superposition IA Edge Game Assist pour les jeux",
            "windows-app-office-actions-server" => {
                "Office AI Actions Server pour les actions automatisées pilotées par l'IA"
            }
            "windows-app-ai-manager" => "Office AI Manager (aimgr) pour gérer les services d'IA",
            "windows-app-writing-assistant" => "Outil IA « Microsoft Office Writing Assistant »",
            "windows-app-ai-workloads" => {
                "Packages de charge de travail IA Windows, y compris ONNX Runtime, texte sémantique, recherche d'images, extraction de contenu, détection de zone à l'écran, reconnaissance de texte et modération du contenu des images"
            }
            "windows-app-copilot-plus-pc" => {
                "Packages IA pour la voix, le langage, la saisie en direct, la saisie et les opérations sur les fichiers pour les PC Copilot+ équipés d'un NPU"
            }
            "windows-app-edge" => "Le navigateur Web de Microsoft",
            "windows-app-onedrive" => "Le service de stockage cloud de Microsoft",
            "windows-app-onenote" => "Application de notes Microsoft",
            "capability-internet-explorer" => "Navigateur Web hérité",
            "capability-powershell-ise" => "Environnement de script PowerShell intégré",
            "capability-quick-assist" => "Application d'assistance à distance",
            "capability-steps-recorder" => "Outil d'enregistrement d'écran",
            "capability-windows-media-player" => "Lecteur multimédia classique",
            "capability-wordpad" => "Éditeur de texte enrichi",
            "capability-notepad" => "Éditeur de texte simple",
            "capability-paint-legacy" => "Application Paint classique",
            "capability-openssh-client" => "Client Secure Shell pour connexions distantes",
            "capability-openssh-server" => "Serveur Secure Shell pour connexions distantes",
            "feature-wsl" => "Permet l'exécution native de fichiers binaires Linux sous Windows",
            "feature-hyperv-platform" => {
                "Plateforme de virtualisation de base sans les outils de gestion Hyper-V"
            }
            "feature-hyperv" => {
                "Plateforme de virtualisation pour exécuter plusieurs systèmes d'exploitation"
            }
            "feature-hyperv-tools" => "Outils de gestion des machines virtuelles Hyper-V",
            "feature-dotnet35" => "Ancien .NET Framework pour les applications plus anciennes",
            "feature-windows-sandbox" => {
                "Environnement de bureau isolé pour exécuter des applications"
            }
            "feature-recall" => "Fonction Windows 11 qui enregistre l'activité de l'utilisateur",
            "3D/Mixed Reality" => "3D/Réalité mixte",
            "AI" => "IA",
            "Automation" => "Automatisation",
            "Bing/Search" => "Bing/Recherche",
            "Browser" => "Navigateur",
            "Browsers" => "Navigateurs",
            "Camera/Media" => "Caméra/Média",
            "Communication" => "Communication",
            "Development" => "Développement",
            "Games" => "Jeux",
            "Graphics" => "Graphiques",
            "Media" => "Média",
            "Networking" => "Réseau",
            "Office" => "Bureau",
            "Phone" => "Téléphone",
            "Productivity" => "Productivité",
            "Security" => "Sécurité",
            "Social" => "Social",
            "Store" => "Boutique",
            "Support" => "Assistance",
            "System" => "Système",
            "System Tools" => "Outils système",
            "System Utilities" => "Utilitaires système",
            "Utilities" => "Utilitaires",
            "Virtualization" => "Virtualisation",
            "performance_group_0" => "Jeu",
            "performance_group_1" => "Processeur",
            "performance_group_2" => "Graphiques",
            "performance_group_3" => "Réseau",
            "performance_group_4" => "Sécurité",
            "performance_group_5" => "Xbox",
            "performance_group_6" => "Services système",
            "performance_group_7" => "Tâches planifiées",
            "performance_group_8" => "Effets visuels",
            "performance_group_9" => "Accessibilité",
            "home_system" => "Système",
            "home_processor" => "Processeur",
            "home_graphics" => "Graphiques",
            "home_memory" => "Mémoire",
            "home_storage" => "Stockage",
            "home_windows" => "Windows",
            "home_uptime" => "Temps de fonctionnement",
            "home_performance" => "Performances",
            "home_product_name" => "Nom du produit système",
            "home_cpu" => "CPU",
            "home_cpu_model" => "Modèle de CPU",
            "home_cores" => "Cœurs",
            "home_gpu" => "GPU",
            "home_memory_total" => "Mémoire totale",
            "home_memory_used" => "Mémoire utilisée",
            "home_version" => "Version",
            "home_kernel" => "Noyau",
            "home_name" => "Nom",
            "home_bios_version" => "Version du BIOS",
            "home_bios_date" => "Date du BIOS",
            "home_unknown_cpu" => "CPU inconnu",
            "home_unknown_pc" => "PC inconnu",
            "home_unknown_os" => "Système d'exploitation inconnu",
            "home_unknown_kernel" => "Noyau inconnu",
            "home_unknown_model" => "Modèle inconnu",
            "home_unknown_vendor" => "Fabricant inconnu",
            "home_unknown_bios" => "BIOS inconnu",
            "home_unknown_date" => "Date inconnue",
            "home_unknown_gpu" => "GPU inconnu",
            "home_unknown_vram" => "VRAM inconnue",
            "home_unknown_value" => "Inconnu",
            "home_cores_suffix" => "{} cœurs",
            "home_gb_total" => "{:.1} Go au total",
            "home_gb_used" => "{:.1} Go utilisés",
            "home_tb_total" => "{:.2} To au total",
            "home_tb_used" => "{:.2} To utilisés",
            "home_uptime_fmt" => "{}j {:02}h {:02}m",
            "home_update_status" => "État des mises à jour",
            "update_available_prefix" => "Mise à jour disponible :",
            "update_failed_prefix" => "Échec de la vérification des mises à jour :",
            "update_newer_version_prefix" => "Une version plus récente est disponible :",
            "update_ready" => "Prêt",
            "update_checked" => "Vous êtes déjà sur la dernière version.",
            "settings_saved" => "Paramètres enregistrés",
            "update_no_found_title" => "Aucune mise à jour trouvée",
            "update_available_title" => "Mise à jour disponible",
            "update_failed_title" => "Échec de la vérification des mises à jour",
            "update_up_to_date" => "Vous êtes déjà sur la dernière version.",
            "update_error_check_updates" => "Échec de la vérification des mises à jour",
            "update_error_read_response" => "Impossible de lire la réponse",
            "update_error_parse_json" => "Impossible d'analyser le JSON",
            "update_error_no_tag_name" => "Aucun tag_name reçu",
            "update_error_download" => "Impossible de télécharger la mise à jour",
            "update_error_create_temp_file" => "Impossible de créer le fichier temporaire",
            "update_error_write_update_file" => "Impossible d'écrire le fichier de mise à jour",
            "update_error_download_too_small" => {
                "Le téléchargement de la mise à jour est trop petit"
            }
            "update_error_resolve_current_exe" => "Impossible de résoudre l'exécutable actuel",
            "update_error_write_update_script" => "Impossible d'écrire le script de mise à jour",
            "update_error_launch_updater" => "Impossible de lancer le programme de mise à jour",
            "update_download_restart" => {
                "Téléchargez l'application et redémarrez-la pour l'installer."
            }
            "update_failed" => "Échec de la vérification des mises à jour.",
            "update_close" => "Fermer",
            "update_check_again" => "Vérifier à nouveau",
            "update_download_restart_btn" => "Télécharger et redémarrer",
            "restore_point_window" => "Créer un point de restauration",
            "restore_point_creating" => {
                "Création du point de restauration. Cela peut prendre un moment..."
            }
            "restore_point_success" => "Point de restauration créé avec succès.",
            "restore_point_failed" => "Échec du point de restauration",
            "restore_point_failed_run" => "Impossible d'exécuter PowerShell.",
            "restore_point_failed_windows" => {
                "Les points de restauration sont uniquement pris en charge sous Windows."
            }
            "download_title" => "Applications et téléchargements",
            "download_subtitle" => {
                "Installez des applications utiles à partir de listes sélectionnées."
            }
            "download_search" => "Rechercher des applications...",
            "download_website" => "Site web",
            "download_installed" => "Installé",
            "download_not_installed" => "Non installé",
            "download_nothing_selected" => "Rien de sélectionné",
            "download_installing" => "Installation des téléchargements sélectionnés...",
            "download_install_job_failed" => "Échec de la tâche d'installation",
            "download_meta" => "IDs Winget : {}\nCatégorie : {}\nSite web : {}",
            "download_confirm_install" => "Confirmer l'installation",
            "download_cancel" => "Annuler",
            "download_confirm_title" => "Confirmer l'installation de l'application",
            "download_confirm_desc" => {
                "Les applications et téléchargements suivants seront installés :"
            }
            "download_result" => "Installés : {}  Échecs : {}",
            "download_refresh" => "Actualiser",
            "download_install_selected" => "Installer la sélection",
            "download_loading" => "Chargement des applications et téléchargements...",
            "download_none" => "Aucune application ou téléchargement disponible.",
            "download_category_0" => "Navigateurs",
            "download_category_1" => "Visionneuses de documents",
            "download_category_2" => "Messagerie, e-mail et calendrier",
            "download_category_3" => "Stockage en ligne et sauvegarde",
            "download_category_4" => "Multimédia",
            "download_category_5" => "Images",
            "download_category_6" => "Outils de personnalisation",
            "download_category_7" => "Jeux",
            "download_category_8" => "Compression",
            "download_category_9" => "Gestion de fichiers et de disque",
            "download_category_10" => "Accès à distance",
            "download_category_11" => "Outils pour disques optiques",
            "download_category_12" => "Autres utilitaires",
            "download_category_13" => "Confidentialité et sécurité",
            "download_category_14" => "Applications de développement",
            "download_category_15" => "Runtimes et dépendances",

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
            "debloater_title" => "Débloatage",
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
            "processes_status_wakekill" => "Réveil-kill",
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
        Language::French => match key {
            "nav_title" => "Navigation",
            "nav_subtitle" => "Centre de contrôle Winchisel",
            "home" => "Accueil",
            "debloater" => "Debloater",
            "downloads" => "Applications et téléchargements",
            "performance" => "Performances",
            "processes" => "Processus",
            "latency" => "Latence",
            "settings" => "Paramètres",
            "check_updates" => "Rechercher des mises à jour",
            "donate" => "Faire un don",
            "bug_report" => "Signaler un bug",
            "status_admin" => "Administrateur",
            "status_standard" => "Standard",
            "settings_title" => "Paramètres",
            "settings_subtitle" => {
                "Ajustez le comportement de l'application à votre flux de travail."
            }
            "settings_application" => "Application",
            "settings_saved_auto" => "Ces paramètres sont enregistrés automatiquement.",
            "language" => "Langue",
            "sidebar_languages" => "Langues",
            "archive-restore" => "Restaurer",
            "settings-2" => "Paramètres",
            "shield-check" => "État de sécurité",
            "refresh-cw" => "Actualiser",
            "monitor" => "Console",
            "Winchisel-Updater" => "Mise à jour Winchisel",
            "Disabled (Recommended)" => "Désactivé (recommandé)",
            "0" => "0",
            "1" => "1",
            "app_name_m365_copilot_office_hub" => "MS 365 Copilot (Office Hub)",
            "app_name_remote_assistance" => "Application d'assistance à distance",
            "app_name_classic_paint" => "Application Paint classique",
            "windows-app-office-hub" => "Microsoft 365 Copilot (anciennement Office Hub)",
            "windows-app-remote-assistance" => "Application d'assistance à distance",
            "capability-paint-legacy" => "Application Paint classique",
            "open_logs" => "Ouvrir les journaux",
            "check_updates_startup" => "Rechercher les mises à jour au démarrage",
            "show_console" => "Afficher la console",
            "system_protection" => "Protection du système",
            "restore_point" => "Point de restauration système",
            "restore_point_desc" => {
                "Crée un point de restauration avant les modifications majeures du système"
            }
            "create_restore_point" => "Créer un point de restauration",
            "performance_title" => "Performances",
            "performance_subtitle" => "Optimisations de jeu et de performances",
            "performance_search" => "Rechercher des réglages de performance...",
            "performance_quick" => "Actions rapides",
            "performance_apply_recommended" => "Appliquer les paramètres recommandés",
            "performance_reset_defaults" => "Rétablir les paramètres Windows par défaut",
            "performance_loading" => "Chargement des réglages de performance...",
            "performance_empty" => "Aucun réglage de performance à afficher.",
            "performance_current_default" => "Par défaut : ",
            "performance_current_recommended" => "Recommandé : ",

            "gaming-game-mode" => {
                "Optimisez votre PC pour le jeu en désactivant certaines tâches en arrière-plan"
            }
            "gaming-performance-explorer-mouse-precision" => {
                "Ajuste la vitesse du curseur selon la vitesse de mouvement (accélération de la souris). La plupart des joueurs compétitifs désactivent cette option pour viser de manière constante dans les jeux FPS"
            }
            "gaming-performance-mouse-hover-time" => {
                "Détermine le temps de survol nécessaire avant qu'un élément devienne actif (en millisecondes). Des valeurs plus faibles font apparaître plus vite les info-bulles, menus et effets de survol. La valeur par défaut est 400 ms"
            }
            "gaming-performance-autostart-delay" => {
                "Retarde de 10 secondes le lancement des applications après le démarrage afin d'améliorer la réactivité initiale du système. Windows devient plus rapide à utiliser, mais les applications de démarrage mettent plus de temps à se charger"
            }
            "gaming-background-apps" => {
                "Utilisez la stratégie de groupe pour contrôler si les applications peuvent s'exécuter en arrière-plan. 'Force Deny' supprime les réglages d'arrière-plan par application dans les paramètres Windows. Utilisez 'User in Control' si vous avez besoin d'applications comme Teams, Zoom ou WhatsApp"
            }
            "gaming-storage-sense" => {
                "Libère automatiquement de l'espace en supprimant les fichiers temporaires, en vidant la corbeille et en gérant les téléchargements"
            }
            "gaming-performance-explorer-search" => {
                "Recherche dans tout le système de fichiers au lieu de se limiter aux emplacements indexés. Les résultats sont plus complets, mais la recherche est nettement plus lente qu'une recherche indexée et l'activité disque augmente"
            }
            "gaming-performance-search-webview2" => {
                "Autorise Windows Search à utiliser WebView2 (Edge) pour afficher les résultats de recherche. La désactivation supprime les processus Edge créés par SearchHost.exe, ce qui réduit l'utilisation des ressources. Utilise un contournement non documenté du Feature Management Windows (ID de fonctionnalité 37926450), susceptible de changer dans de futures mises à jour"
            }
            "gaming-performance-wallpaper-compression" => {
                "Autorise Windows à compresser les fonds d'écran pour économiser de l'espace disque et améliorer les performances. N'affecte que les images au format JPEG."
            }
            "gaming-performance-explorer-menu-show-delay" => {
                "Ajoute un court délai avant l'affichage des menus (400 ms - valeur Windows par défaut) ou les affiche immédiatement (0 ms) pour accélérer la navigation"
            }
            "gaming-explorer-alt-tab-filter" => {
                "Alt+Tab n'affiche que les fenêtres ouvertes traditionnelles au lieu d'inclure les onglets Microsoft Edge et d'autres suggestions Windows"
            }
            "gaming-win32-priority" => {
                "Configure la manière dont Windows répartit le temps CPU entre les applications au premier plan et les services en arrière-plan"
            }
            "gaming-system-responsiveness" => {
                "Réduit l'impact des tâches en arrière-plan en allouant plus de temps CPU à votre jeu ou à votre application multimédia active"
            }
            "gaming-cpu-priority" => {
                "Donne aux jeux une priorité de planification CPU plus élevée afin de leur consacrer davantage de temps processeur"
            }
            "gaming-scheduling-category" => {
                "Attribue une catégorie de planification à priorité élevée pour garantir une allocation préférentielle des ressources système aux jeux"
            }
            "gaming-performance-svchost-split-threshold" => {
                "Définit le seuil mémoire qui détermine quand Windows sépare les services en processus svchost.exe distincts. Des valeurs plus élevées regroupent davantage de services, réduisant ainsi le nombre de processus. Sélectionnez la valeur correspondant à la RAM de votre système"
            }
            "gaming-gpu-priority" => {
                "Donne aux jeux une priorité de planification GPU plus élevée pour améliorer les performances graphiques et les fréquences d'images"
            }
            "gaming-gpu-scheduling" => {
                "Laissez votre GPU gérer sa propre mémoire et sa planification pour réduire la latence et améliorer les performances"
            }
            "gaming-directx-flip-model" => {
                "Réduit la latence et exploite des fonctions avancées dans les jeux compatibles en utilisant le modèle de présentation DirectX Flip"
            }
            "gaming-directx-vrr-optimizations" => {
                "Active les optimisations VRR (G-Sync/FreeSync) pour un gameplay plus fluide. Nécessite un moniteur compatible VRR ; ce réglage n'a aucun effet si votre écran ne prend pas en charge le VRR"
            }
            "gaming-directx-auto-hdr" => {
                "Convertit automatiquement le contenu SDR en HDR pour améliorer les couleurs et la luminosité. Nécessite un écran compatible HDR avec HDR activé ; ce réglage n'a aucun effet si votre écran ne prend pas en charge le HDR"
            }
            "gaming-nvidia-sharpening" => {
                "Active l'ancien filtre de netteté d'image NVIDIA pour une meilleure clarté visuelle. Ne fonctionne qu'avec les anciens pilotes NVIDIA ; les pilotes plus récents doivent utiliser la netteté du panneau de configuration NVIDIA"
            }
            "gaming-fullscreen-optimizations" => {
                "Autorise Windows à optimiser les jeux exécutés en plein écran. La désactivation peut résoudre des problèmes de performances ou des micro-saccades dans certains anciens jeux qui fonctionnent mal avec l'optimisation plein écran sans bordure"
            }
            "gaming-performance-desktop-composition" => {
                "Active les effets visuels gérés par Desktop Window Manager. La désactivation peut apporter de petits gains de performances sur du matériel ancien, mais affectera les effets Aero"
            }
            "gaming-auto-color-management" => {
                "Autorise Windows à gérer automatiquement les profils de couleur de tous les écrans connectés qui le prennent en charge"
            }
            "gaming-disable-mpo" => {
                "Compose plusieurs couches d'affichage dans le matériel à l'aide du GPU. La désactivation peut résoudre les scintillements, les écrans noirs et les saccades dans les configurations multi-écrans"
            }
            "gaming-disable-mpo-min-fps" => {
                "Permet à Desktop Window Manager de faire basculer dynamiquement les applications entre les modes de superposition en fonction du nombre d'images par seconde. La désactivation peut corriger les saccades dans les navigateurs et Discord sans désactiver complètement MPO"
            }
            "gaming-network-throttling" => {
                "Contrôle la limitation du débit des paquets réseau pour les applications multimédias. Il est recommandé de laisser la limitation activée (valeur par défaut : 10 paquets/ms), car elle offre une meilleure latence DPC pour le jeu qu'une désactivation complète"
            }
            "gaming-nagle-algorithm" => {
                "Met en tampon les petits paquets réseau avant l'envoi afin de réduire la surcharge. Désactivez-le pour réduire la latence en jeu en ligne ou laissez-le activé pour une meilleure efficacité réseau globale"
            }
            "gaming-dns-server" => {
                "Sélectionnez un serveur DNS pour tous les adaptateurs réseau. Les modifications s'appliquent à chaque adaptateur de votre système (Wi-Fi et Ethernet). Utilisez « Automatique » pour restaurer le DNS par défaut de votre FAI/routeur"
            }
            "gaming-virtualization-based-security" => {
                "Isole des portions de la mémoire afin de protéger le système contre les vulnérabilités. La désactivation peut améliorer les performances de jeu, mais réduit la sécurité du système"
            }
            "gaming-memory-integrity" => {
                "Empêche l'introduction de code malveillant dans les processus à haute sécurité. La désactivation peut améliorer les performances de jeu, mais réduit la sécurité du système"
            }
            "gaming-xbox-game-dvr" => {
                "Enregistre des extraits de jeu et capture des captures d'écran avec la superposition Xbox Game Bar. La désactivation réduit l'utilisation du CPU/GPU et peut améliorer les fréquences d'images"
            }
            "gaming-game-bar-controller" => {
                "Autorise votre manette Xbox ou compatible à ouvrir la Game Bar en appuyant sur le bouton Xbox. Désactivez cette option pour éviter l'activation accidentelle de la Game Bar pendant le jeu"
            }
            "gaming-game-bar-tips" => {
                "Affiche des conseils et astuces sur les fonctionnalités de la Game Bar lorsque la superposition s'ouvre. La désactivation réduit les distractions pendant le jeu"
            }
            "gaming-performance-background-services" => {
                "Réduit le délai de démarrage des services Windows de 60 à 30 secondes. Cela peut légèrement réduire le temps de démarrage"
            }
            "gaming-sysmain-service" => {
                "Charge à l'avance les applications fréquemment utilisées dans la RAM pour accélérer les démarrages. Le mode Automatique est recommandé pour les disques durs ou les systèmes de stockage mixtes ; « Manuel » ou « Désactivé » ne convient qu'aux systèmes SSD purs"
            }
            "gaming-performance-prefetch" => {
                "Charge à l'avance les applications fréquemment utilisées et les fichiers de démarrage en mémoire pour accélérer le lancement. Généralement recommandé pour les HDD, pas pour les SSD"
            }
            "gaming-windows-search-service" => {
                "Indexe les fichiers et dossiers pour accélérer les résultats de recherche. Le désactiver réduit l'activité CPU et disque en arrière-plan, mais casse la recherche Outlook et rend la recherche du menu Démarrer et de l'Explorateur de fichiers lente ou peu fiable"
            }
            "gaming-print-spooler-service" => {
                "Gère les travaux d'impression envoyés aux imprimantes. Si vous n'utilisez pas d'imprimante, définissez-le sur Manuel ou Désactivé pour libérer des ressources système"
            }
            "gaming-telemetry-service" => {
                "Envoie des données d'utilisation et des diagnostics à Microsoft. Le définir sur Manuel ou Désactivé réduit l'activité réseau et CPU en arrière-plan"
            }
            "gaming-connected-devices-platform-service" => {
                "Active les expériences inter-appareils comme la liaison téléphonique et le partage à proximité. La désactivation réduit l'activité en arrière-plan et la journalisation des interactions avec les appareils"
            }
            "gaming-compatibility-assistant-service" => {
                "Surveille les programmes pour détecter des problèmes de compatibilité et suggère des correctifs. Le désactiver supprime les invites de compatibilité et économise de petites ressources système"
            }
            "gaming-error-reporting-service" => {
                "Collecte et envoie les données de crash à Microsoft. Le désactiver empêche les rapports de crash, réduit le trafic réseau et améliore la protection des données avec un impact minimal sur le système"
            }
            "gaming-geolocation-service" => {
                "Suit votre position physique pour les applications et services. La désactivation améliore la confidentialité et empêche le suivi de localisation, mais les applications ne pourront plus utiliser les fonctions de localisation"
            }
            "gaming-retail-demo-service" => {
                "Contrôle l'activité de l'appareil en mode démonstration commerciale. Sur PC, la désactivation peut être sûre car elle ne sert qu'à l'affichage en magasin"
            }
            "gaming-insider-service" => {
                "Gère les fonctionnalités du programme Windows Insider et les versions préliminaires. La désactivation est sûre si vous ne participez pas au programme Windows Insider"
            }
            "gaming-phone-service" => {
                "Gère l'état de la téléphonie sur l'appareil. Vous pouvez facilement désactiver cette fonction si vous n'utilisez pas les fonctionnalités de connexion téléphonique et n'effectuez pas d'appels depuis votre PC"
            }
            "gaming-wallet-service" => {
                "Fournit les fonctionnalités de portefeuille pour les scénarios de paiement et NFC. Il est sûr de le désactiver si vous n'utilisez pas les fonctions Microsoft Wallet"
            }
            "gaming-smart-card-services" => {
                "Active la fonction de lecteur de carte à puce pour l'authentification de sécurité. La désactivation est sûre si vous n'utilisez pas de cartes à puce physiques ni de lecteurs de cartes"
            }
            "gaming-maps-broker-service" => {
                "Fournit l'accès aux cartes téléchargées pour les applications. Réglez l'option sur Manuel pour permettre l'accès aux cartes au besoin tout en empêchant une activité inutile en arrière-plan"
            }
            "gaming-fax-service" => {
                "Permet d'envoyer et de recevoir des fax. Pour la plupart des utilisateurs, la désactivation est sûre, car la fonction fax est rarement utilisée sur les systèmes modernes"
            }
            "gaming-wmp-network-service" => {
                "Partage les bibliothèques de Windows Media Player avec d'autres lecteurs et appareils multimédias du réseau. Vous pouvez désactiver cette fonction en toute sécurité si vous ne partagez pas de médias sur votre réseau"
            }
            "gaming-mixed-reality-service" => {
                "Exécute les applications OpenXR sur les appareils Windows Mixed Reality. Vous pouvez le désactiver sans risque si vous n'utilisez pas de casque VR ou AR"
            }
            "gaming-mobile-hotspot-service" => {
                "Permet de partager la connexion Internet avec d'autres appareils. Réglez sur Manuel pour conserver la fonctionnalité tout en empêchant une activité inutile en arrière-plan"
            }
            "gaming-sms-router-service" => {
                "Transmet les messages SMS selon des règles. Vous pouvez désactiver cette fonction sans risque si vous n'utilisez pas les fonctions SMS sur votre PC"
            }
            "gaming-parental-controls-service" => {
                "Active le contrôle parental et les fonctions de sécurité familiale. Vous pouvez désactiver cette fonction sans risque si vous n'utilisez pas le contrôle parental"
            }
            "gaming-payments-nfc-service" => {
                "Gère les paiements et les éléments sécurisés de la communication en champ proche. Il est sûr de le désactiver si vous n'utilisez pas les fonctions de paiement NFC"
            }
            "gaming-spot-verifier-service" => {
                "Vérifie l'éventuelle corruption du système de fichiers. Réglez l'option sur Manuel pour activer la vérification au besoin tout en réduisant l'activité en arrière-plan"
            }
            "gaming-remote-access-manager" => {
                "Gère les connexions VPN et numérotation. Réglez sur Manuel pour réduire l'activité en arrière-plan tout en conservant la fonctionnalité VPN lorsque nécessaire."
            }
            "gaming-remote-access-auto" => {
                "Se connecte automatiquement aux réseaux distants lorsque des programmes référencent des ressources distantes. Vous pouvez désactiver cette fonction sans risque si vous n'utilisez pas la connexion automatique du VPN"
            }
            "gaming-remote-desktop-services" => {
                "Permet aux utilisateurs de se connecter de manière interactive à un ordinateur distant. Réglez sur Manuel pour réduire l'activité en arrière-plan tout en gardant le Bureau à distance disponible."
            }
            "gaming-remote-desktop-configuration" => {
                "Gère les services Bureau à distance et les configurations associées. Réglez sur Manuel pour réduire l'activité en arrière-plan tout en gardant le Bureau à distance disponible"
            }
            "gaming-remote-desktop-port-redirector" => {
                "Active la redirection des périphériques locaux pour les connexions Bureau à distance. Vous pouvez le désactiver sans risque si vous n'avez pas besoin de partager des périphériques locaux pendant les sessions Bureau à distance"
            }
            "gaming-xbox-auth-manager" => {
                "Fournit les services d'authentification et d'autorisation pour Xbox Live. Il est sûr de le désactiver si vous n'utilisez pas Xbox Game Pass, les jeux du Microsoft Store ou les fonctions Xbox"
            }
            "gaming-xbox-game-save" => {
                "Synchronise les sauvegardes de jeu avec le cloud Xbox Live. Nécessaire uniquement pour Xbox Game Pass et les jeux du Microsoft Store avec sauvegarde cloud"
            }
            "gaming-xbox-networking" => {
                "Prend en charge les réseaux multijoueurs Xbox Live. Requis pour les jeux multijoueurs Xbox, mais pas pour Steam/Epic ou d'autres plateformes"
            }
            "gaming-biometric-service" => {
                "Active la connexion par empreinte digitale et reconnaissance faciale via Windows Hello. Peut être désactivé sans risque sur les systèmes de bureau sans matériel biométrique"
            }
            "gaming-touch-keyboard-service" => {
                "Gère l'expérience de saisie Windows, y compris le clavier tactile, la saisie au stylet, le panneau d'écriture manuscrite, le panneau emoji (Win+.) et le clavier de la manette Xbox. Le désactiver cassera toute saisie par clavier virtuel/logiciel, mais il est sûr sur les systèmes de bureau sans écran tactile, stylet ou manette"
            }
            "gaming-sensor-monitoring-service" => {
                "Surveille divers capteurs comme la lumière ambiante et l'orientation. Peut être désactivé sans risque sur les systèmes de bureau sans matériel de capteurs"
            }
            "gaming-sensor-data-service" => {
                "Fournit aux applications les données provenant de divers capteurs. Peut être désactivé sans risque sur les systèmes de bureau sans matériel de capteurs"
            }
            "gaming-ai-fabric-service" => {
                "Le service Windows AI Fabric (WSAIFabricSvc) gère les charges de travail d'IA. Désactivez cette option si vous n'utilisez pas les fonctionnalités IA de Windows"
            }
            "CompatibilityAppraiserTask" => {
                "Collecte des données de télémétrie sur la compatibilité des programmes pour les mises à niveau de Windows. Fonctionne avec le service Expériences utilisateur connectées et télémétrie. Désactivez pour réduire la télémétrie et l'activité système en arrière-plan"
            }
            "ProgramDataUpdaterTask" => {
                "Met à jour la base de données de compatibilité des programmes avec les informations sur les applications installées. Désactivez cette option pour réduire la collecte de télémétrie"
            }
            "CEIPConsolidatorTask" => {
                "Consolide et envoie des données d'utilisation dans le cadre du programme d'amélioration de l'expérience client. Fonctionne avec le service Expériences utilisateur connectées et télémétrie. Désactivez pour améliorer la confidentialité"
            }
            "UsbCeipTask" => {
                "Collecte des données de télémétrie liées aux périphériques USB pour le programme d'amélioration de la satisfaction client. Désactivez pour réduire la télémétrie"
            }
            "DiskDiagnosticTask" => {
                "Collecte des informations de diagnostic du disque dur et des données S.M.A.R.T. pour Microsoft. Désactivez cette option pour réduire l'activité disque en arrière-plan et la télémétrie"
            }
            "FeedbackDmClientTask" => {
                "Collecte les commentaires et les données de diagnostic pour Microsoft. Désactivez cette option pour améliorer la confidentialité et réduire la télémétrie"
            }
            "FeedbackDmClientDownloadTask" => {
                "Télécharge les scénarios de retour d'expérience et les données de configuration depuis Microsoft. Désactivez pour réduire la télémétrie et l'activité réseau"
            }
            "ErrorReportingQueueTask" => {
                "Met en file d'attente les rapports de plantage et les données d'erreur à envoyer à Microsoft. Fonctionne avec le service de rapport d'erreurs Windows. Désactivez les deux pour empêcher la collecte des données de crash"
            }
            "SqmTask" => {
                "Collecte des métriques de qualité logicielle et des données de fiabilité pour la télémétrie Microsoft. Désactivez pour améliorer la confidentialité"
            }
            "MareBackupTask" => {
                "Sauvegarde les données Microsoft Assisted Recovery. Désactivez cette option pour réduire l'activité système en arrière-plan"
            }
            "StartupAppTask" => {
                "Surveille et suit les applications au démarrage pour la télémétrie et le diagnostic. Désactivez pour réduire la télémétrie"
            }
            "MapsUpdateTask" => {
                "Met à jour les données cartographiques hors ligne pour l'application Cartes Windows. Désactivez cette option si vous n'utilisez pas l'application Cartes pour économiser la bande passante et l'espace de stockage"
            }
            "AutochkProxyTask" => {
                "Effectue des vérifications du disque dur et collecte des données de diagnostic. Il est recommandé de laisser la surveillance de l'état du disque activée"
            }
            "FamilySafetyTask" => {
                "Surveille les paramètres et l'utilisation de la sécurité familiale. Désactivez cette option si vous n'utilisez pas les fonctions de sécurité familiale"
            }
            "PowerEfficiencyTask" => {
                "Analyse la consommation d'énergie du système et collecte des données d'efficacité énergétique. Désactivez cette option pour réduire la télémétrie et l'analyse en arrière-plan"
            }
            "WindowsAIRecallConfig" => {
                "Tâches planifiées Windows AI, y compris la configuration de Recall. Désactivez cette option pour empêcher les fonctionnalités IA de s'exécuter en arrière-plan"
            }
            "WindowsAIRecallPipeline" => {
                "Tâche du pipeline Windows AI Recall. Désactivez cette option pour empêcher le traitement du pipeline Recall en arrière-plan"
            }
            "OfficeActionsServer" => {
                "Tâche planifiée du serveur Office AI Actions. Désactivez cette option pour empêcher Office AI de s'exécuter en arrière-plan"
            }
            "visual-effects-mode" => "Choisissez comment Windows affiche les effets visuels",
            "ui-effects" => {
                "Active les effets d'animation pour les contrôles et les éléments de l'interface"
            }
            "window-animation" => {
                "Affiche une animation fluide lorsque les fenêtres sont réduites ou agrandies"
            }
            "taskbar-animations" => {
                "Contrôle les effets d'animation de la barre des tâches lors de l'ouverture, de la fermeture et du changement de fenêtres"
            }
            "enable-peek" => {
                "Permet de voir le bureau lorsque vous survolez le bouton Afficher le bureau"
            }
            "menu-animation" => {
                "Anime les menus lorsqu'ils s'affichent avec des effets de fondu ou de glissement"
            }
            "fade-tooltip" => {
                "Anime les infobulles lorsqu'elles s'affichent avec des effets de fondu ou de glissement"
            }
            "fade-menu-items" => {
                "Masque les éléments de menu après sélection avant de fermer le menu"
            }
            "taskbar-thumbnails" => {
                "Enregistre des aperçus miniatures des fenêtres de la barre des tâches pour un affichage plus rapide"
            }
            "mouse-shadow" => "Affiche un effet d'ombre sous le pointeur de la souris",
            "window-shadows" => "Affiche des effets d'ombre sous les fenêtres",
            "show-thumbnails" => {
                "Affiche des aperçus d'images et de documents au lieu d'icônes de fichiers génériques"
            }
            "translucent-selection" => {
                "Affiche un cadre de sélection semi-transparent lorsque vous faites glisser pour sélectionner plusieurs fichiers ou éléments"
            }
            "drag-full-windows" => {
                "Lors du déplacement, affiche le contenu de la fenêtre au lieu d'un simple contour"
            }
            "combo-box-animation" => "Anime les listes déroulantes avec un effet de glissement",
            "font-smoothing" => {
                "Applique l'anti-crénelage au texte pour des polices plus douces et plus lisibles à l'écran"
            }
            "smooth-scroll-listboxes" => {
                "Permet un défilement fluide dans les listes au lieu de sauts brusques"
            }
            "drop-shadows" => {
                "Ajoute des effets d'ombre derrière le texte des icônes du bureau pour améliorer la lisibilité sur le fond"
            }
            "gaming-narrator-hotkey" => {
                "Active le raccourci clavier Win+Ctrl+Entrée pour lancer rapidement le lecteur d'écran Narrateur de Windows"
            }
            "accessibility-stickykeys-hotkey" => {
                "Active le raccourci clavier pour activer les touches rémanentes en appuyant cinq fois sur la touche Maj"
            }
            "accessibility-filterkeys-hotkey" => {
                "Active le raccourci clavier pour activer les touches filtres en maintenant la touche Maj droite enfoncée pendant 8 secondes"
            }
            "accessibility-togglekeys-hotkey" => {
                "Pour activer les touches bascules, activez le raccourci clavier en maintenant la touche Verr Num pendant 5 secondes. Des sons seront joués lorsque les touches Verr Maj/Verr Num/Arrêt défil sont pressées"
            }
            "accessibility-mousekeys-hotkey" => {
                "Active le raccourci clavier pour activer les touches de la souris, qui permettent d'utiliser le pavé numérique pour contrôler le pointeur"
            }
            "accessibility-highcontrast-hotkey" => {
                "Pour activer le mode Contraste élevé, utilisez le raccourci clavier en appuyant sur Alt gauche + Maj gauche + Impr écran"
            }
            "performance_group_0" => "Jeu",
            "performance_group_1" => "Processeur",
            "performance_group_2" => "Graphiques",
            "performance_group_3" => "Réseau",
            "performance_group_4" => "Sécurité",
            "performance_group_5" => "Xbox",
            "performance_group_6" => "Services système",
            "performance_group_7" => "Tâches planifiées",
            "performance_group_8" => "Effets visuels",
            "performance_group_9" => "Accessibilité",
            "home_system" => "Système",
            "home_processor" => "Processeur",
            "home_graphics" => "Graphiques",
            "home_memory" => "Mémoire",
            "home_storage" => "Stockage",
            "home_windows" => "Windows",
            "home_uptime" => "Temps de fonctionnement",
            "home_performance" => "Performances",
            "home_product_name" => "Nom du produit système",
            "home_cpu" => "CPU",
            "home_cpu_model" => "Modèle de CPU",
            "home_cores" => "Cœurs",
            "home_gpu" => "GPU",
            "home_memory_total" => "Mémoire totale",
            "home_memory_used" => "Mémoire utilisée",
            "home_version" => "Version",
            "home_kernel" => "Noyau",
            "home_name" => "Nom",
            "home_bios_version" => "Version du BIOS",
            "home_bios_date" => "Date du BIOS",
            "home_unknown_cpu" => "CPU inconnu",
            "home_unknown_pc" => "PC inconnu",
            "home_unknown_os" => "OS inconnu",
            "home_unknown_kernel" => "Noyau inconnu",
            "home_unknown_model" => "Modèle inconnu",
            "home_unknown_vendor" => "Fabricant inconnu",
            "home_unknown_bios" => "BIOS inconnu",
            "home_unknown_date" => "Date inconnue",
            "home_unknown_gpu" => "GPU inconnu",
            "home_unknown_vram" => "VRAM inconnue",
            "home_unknown_value" => "Inconnu",
            "home_cores_suffix" => "{} cœurs",
            "home_gb_total" => "{:.1} Go au total",
            "home_gb_used" => "{:.1} Go utilisés",
            "home_tb_total" => "{:.2} To au total",
            "home_tb_used" => "{:.2} To utilisés",
            "home_uptime_fmt" => "{} j {:02} h {:02} min",
            "home_update_status" => "État des mises à jour",
            "update_available_prefix" => "Mise à jour disponible :",
            "update_failed_prefix" => "Échec de la vérification des mises à jour :",
            "update_newer_version_prefix" => "Une version plus récente est disponible :",
            "update_ready" => "Prêt",
            "update_checked" => "Vous utilisez déjà la dernière version.",
            "settings_saved" => "Paramètres enregistrés",
            "update_no_found_title" => "Aucune mise à jour trouvée",
            "update_available_title" => "Mise à jour disponible",
            "update_failed_title" => "Échec de la vérification des mises à jour",
            "update_up_to_date" => "Vous utilisez déjà la dernière version.",
            "update_error_check_updates" => "Échec de la vérification des mises à jour",
            "update_error_read_response" => "Impossible de lire la réponse",
            "update_error_parse_json" => "Impossible d'analyser le JSON",
            "update_error_no_tag_name" => "Aucun tag_name dans la réponse",
            "update_error_download" => "Impossible de télécharger la mise à jour",
            "update_error_create_temp_file" => "Impossible de créer le fichier temporaire",
            "update_error_write_update_file" => "Impossible d'écrire le fichier de mise à jour",
            "update_error_download_too_small" => "Le fichier téléchargé est trop petit",
            "update_error_resolve_current_exe" => "Impossible de résoudre l'exécutable actuel",
            "update_error_write_update_script" => "Impossible d'écrire le script de mise à jour",
            "update_error_launch_updater" => "Impossible de lancer le programme de mise à jour",
            "update_download_restart" => {
                "Téléchargez puis redémarrez l'application pour l'installer."
            }
            "update_failed" => "Échec de la vérification des mises à jour.",
            "update_close" => "Fermer",
            "update_check_again" => "Vérifier à nouveau",
            "update_download_restart_btn" => "Télécharger et redémarrer",
            "restore_point_window" => "Créer un point de restauration",
            "restore_point_creating" => {
                "Création du point de restauration. Cela peut prendre un moment..."
            }
            "restore_point_success" => "Point de restauration créé avec succès.",
            "restore_point_failed" => "Échec du point de restauration",
            "restore_point_failed_run" => "Impossible d'exécuter PowerShell.",
            "restore_point_failed_windows" => {
                "Les points de restauration ne sont pris en charge que sous Windows."
            }
            "download_title" => "Applications et téléchargements",
            "download_subtitle" => {
                "Installez des applications utiles depuis des listes sélectionnées."
            }
            "download_search" => "Rechercher des applications...",
            "download_website" => "Site web",
            "download_installed" => "Installé",
            "download_not_installed" => "Non installé",
            "download_nothing_selected" => "Aucune sélection",
            "download_installing" => "Installation des téléchargements sélectionnés...",
            "download_install_job_failed" => "Échec de la tâche d'installation",
            "download_meta" => "IDs Winget : {}\nCatégorie : {}\nSite web : {}",
            "download_confirm_install" => "Confirmer l'installation",
            "download_cancel" => "Annuler",
            "download_confirm_title" => "Confirmer l'installation de l'application",
            "download_confirm_desc" => "Ces applications et téléchargements seront installés :",
            "download_result" => "Installé : {}  Échec : {}",
            "download_refresh" => "Actualiser",
            "download_install_selected" => "Installer la sélection",
            "download_loading" => "Chargement des applications et téléchargements...",
            "download_none" => "Aucune application ou téléchargement à afficher.",
            "download_category_0" => "Navigateurs",
            "download_category_1" => "Lecteurs de documents",
            "download_category_2" => "Messagerie, e-mail et calendrier",
            "download_category_3" => "Stockage en ligne et sauvegarde",
            "download_category_4" => "Multimédia",
            "download_category_5" => "Imagerie",
            "download_category_6" => "Utilitaires de personnalisation",
            "download_category_7" => "Jeu",
            "download_category_8" => "Compression",
            "download_category_9" => "Gestion des fichiers et des disques",
            "download_category_10" => "Accès à distance",
            "download_category_11" => "Outils pour disques optiques",
            "download_category_12" => "Autres utilitaires",
            "download_category_13" => "Confidentialité et sécurité",
            "download_category_14" => "Applications de développement",
            "download_category_15" => "Runtime et dépendances",

            "Microsoft EdgeWebView" => "Runtime WebView2 pour les applications Windows",
            "Thorium" => {
                "Navigateur basé sur Chromium avec des fonctions de confidentialité renforcées"
            }
            "Mercury" => {
                "Fork de Firefox optimisé par le compilateur et axé sur la confidentialité"
            }
            "Mozilla Firefox" => {
                "Navigateur Web populaire, reconnu pour sa confidentialité et sa personnalisation"
            }
            "Google Chrome" => {
                "Navigateur Web de Google avec synchronisation et prise en charge des extensions"
            }
            "ungoogled-chromium" => {
                "Navigateur basé sur Chromium avec des améliorations de confidentialité"
            }
            "Brave" => "Navigateur axé sur la confidentialité avec blocage des publicités intégré",
            "Opera" => {
                "Navigateur Web riche en fonctionnalités avec VPN et bloqueur de publicités intégrés"
            }
            "Opera GX" => "Version d'Opera orientée jeu avec des fonctionnalités uniques",
            "Arc Browser" => "Navigateur innovant axé sur le design et l'expérience utilisateur",
            "Tor Browser" => {
                "Navigateur axé sur la confidentialité qui achemine le trafic via le réseau Tor"
            }
            "Vivaldi" => "Navigateur hautement personnalisable axé sur le contrôle utilisateur",
            "Waterfox" => {
                "Navigateur basé sur Firefox axé sur la confidentialité et la personnalisation"
            }
            "Zen Browser" => {
                "Navigateur axé sur la confidentialité avec blocage des publicités intégré"
            }
            "Mullvad Browser" => {
                "Navigateur axé sur la confidentialité conçu pour minimiser le pistage et l'empreinte numérique"
            }
            "Pale Moon Browser" => {
                "Navigateur open source basé sur Goanna, axé sur l'efficacité et la personnalisation"
            }
            "Maxthon" => {
                "Navigateur axé sur la confidentialité avec blocage des publicités et VPN intégrés"
            }
            "Ablaze Floorp" => {
                "Navigateur axé sur la confidentialité avec une forte protection contre le suivi"
            }
            "DuckDuckGo" => {
                "Moteur de recherche axé sur la confidentialité avec extension de navigateur"
            }
            "LibreOffice" => "Suite bureautique libre et open source",
            "ONLYOFFICE Desktop Editors" => {
                "Alternative gratuite et 100 % open source à Microsoft Office"
            }
            "PDFgear" => {
                "Lisez, modifiez, convertissez, fusionnez et signez des fichiers PDF sur plusieurs appareils, entièrement gratuitement et sans inscription."
            }
            "Foxit PDF Reader" => "Lecteur PDF léger avec fonctions avancées",
            "SumatraPDF" => {
                "Lecteur Windows pour PDF, eBooks (epub, mobi), bandes dessinées (cbz/cbr), DjVu, XPS, CHM et images"
            }
            "OpenOffice" => {
                "Suite bureautique open source abandonnée. Le successeur actif est LibreOffice"
            }
            "Adobe Acrobat Reader DC" => "Lecteur et éditeur PDF",
            "Evernote" => "Application de prise de notes",
            "CherryTree" => {
                "Application de prise de notes hiérarchique avec texte enrichi et coloration syntaxique"
            }
            "Okular" => {
                "Visionneuse universelle de documents prenant en charge les PDF, eBooks et plus encore"
            }
            "PDF24 Creator" => "Créateur et convertisseur PDF gratuit",
            "Telegram Desktop" => "Application de messagerie instantanée et d'appels vocaux",
            "WhatsApp" => "Application de messagerie instantanée et d'appels vocaux",
            "Zoom Workplace" => "Plateforme de visioconférence et de messagerie",
            "Discord" => "Service de communication vocale, vidéo et textuelle",
            "Pidgin" => "Client de messagerie instantanée multiprotocole",
            "Mozilla Thunderbird" => "Application e-mail gratuite",
            "eM Client" => "Client e-mail avec calendrier, tâches et chat",
            "Proton Mail" => "Service de messagerie sécurisé avec chiffrement de bout en bout",
            "Trillian" => "Application de messagerie instantanée",
            "Google Drive" => "Service de stockage cloud et de synchronisation de fichiers",
            "Dropbox" => {
                "Service d'hébergement de fichiers offrant stockage cloud, synchronisation de fichiers et cloud personnel"
            }
            "SugarSync" => {
                "Accédez automatiquement à vos photos, vidéos et fichiers et partagez-les dans n'importe quel dossier"
            }
            "Nextcloud" => {
                "Accédez à vos fichiers, calendriers, contacts, communications et plus encore, partagez-les et protégez-les chez vous et dans votre organisation"
            }
            "Proton Drive" => "Stockage cloud sécurisé avec chiffrement de bout en bout",
            "FreeFileSync" => "Outil open source de comparaison et de synchronisation de dossiers",
            "Hekasoft Backup & Restore" => {
                "La solution gratuite complète pour la sauvegarde et la gestion des navigateurs"
            }
            "VLC media player" => "Lecteur multimédia et cadre open source",
            "iTunes" => "Lecteur multimédia et bibliothèque",
            "AIMP" => "Lecteur audio prenant en charge de nombreux formats",
            "foobar2000" => "Lecteur audio avancé pour Windows",
            "MusicBee" => "Gestionnaire et lecteur de musique",
            "Audacity" => "Éditeur et enregistreur audio",
            "GOM Player" => "Lecteur multimédia pour Windows",
            "Spotify" => "Service de streaming musical",
            "MediaMonkey" => "Gestionnaire et lecteur multimédia",
            "HandBrake" => "Transcodeur vidéo open source",
            "OBS Studio" => {
                "Logiciel libre et open source pour l'enregistrement vidéo et le streaming en direct"
            }
            "Streamlabs OBS" => {
                "Logiciel de streaming basé sur OBS avec des fonctionnalités supplémentaires pour les streamers"
            }
            "MPC-BE" => "Media Player Classic - Édition Black",
            "K-Lite Codec Pack (Mega)" => "Collection de codecs et d'outils associés",
            "CapCut" => "Éditeur vidéo",
            "PotPlayer64" => "Lecteur multimédia complet pour Windows",
            "kdenlive" => "Logiciel de montage vidéo libre et open source",
            "MediaInfo" => {
                "Outil d'affichage d'informations techniques pour les fichiers multimédias"
            }
            "fre:ac - free audio converter" => "Convertisseur audio et extracteur de CD gratuit",
            "SMPlayer" => {
                "Lecteur multimédia avec codecs intégrés pouvant lire pratiquement tous les formats vidéo et audio"
            }
            "Shotcut" => "Éditeur vidéo libre, open source et multiplateforme",
            "LosslessCut" => {
                "Interface FFmpeg multiplateforme pour un découpage vidéo/audio rapide et sans perte"
            }
            "FxSound" => "Améliorateur audio pour renforcer la qualité sonore sous Windows",
            "IrfanView64" => "Visionneuse et convertisseur d'images rapide et compacte",
            "Krita" => "Logiciel de peinture numérique et d'illustration",
            "Blender" => "Suite de création 3D",
            "Paint.NET" => "Logiciel de retouche d'images et de photos",
            "GIMP" => "Programme de manipulation d'images GNU",
            "XnViewMP" => "Visionneuse, navigateur et convertisseur d'images",
            "XnView" => "Visionneuse, navigateur et convertisseur d'images (version classique)",
            "Inkscape" => "Éditeur de graphismes vectoriels",
            "Greenshot" => "Outil de capture d'écran avec fonctions d'annotation",
            "ShareX" => "Outil de capture d'écran, de partage de fichiers et de productivité",
            "Flameshot" => "Logiciel de capture d'écran puissant mais simple à utiliser",
            "FastStone Image Viewer" => "Navigateur, convertisseur et éditeur d'images",
            "Nilesoft Shell" => "Outil de personnalisation du menu contextuel de Windows",
            "StartAllBack (Win 11)" => {
                "Personnalisation du menu Démarrer et de la barre des tâches de Windows 11"
            }
            "StartIsBack++ (Win 10)" => {
                "Personnalisation du menu Démarrer et de la barre des tâches de Windows 10"
            }
            "Open-Shell" => "Menu Démarrer de style classique pour Windows",
            "Windhawk" => "Plateforme de personnalisation pour Windows",
            "Lively Wallpaper" => "Application de fond d'écran animé libre et open source",
            "Sucrose Wallpaper Engine" => "Application de fond d'écran animé libre et open source",
            "Rainmeter" => "Outil de personnalisation du bureau pour Windows",
            "ExplorerPatcher" => "Utilitaire qui améliore l'expérience de l'Explorateur Windows",
            "John's Background Switcher" => {
                "Change automatiquement votre fond d'écran à intervalles réguliers"
            }
            "Microsoft PowerToys" => {
                "Ensemble d'utilitaires pour les utilisateurs avancés afin d'ajuster et de simplifier leur expérience Windows"
            }
            "Nexus" => "Le système d'ancrage avancé pour Windows",
            "AutoHotkey v2" => {
                "Langage de script gratuit pour créer des macros et automatiser des tâches (v2, actuelle)"
            }
            "Steam" => "Plateforme de distribution numérique pour les jeux PC",
            "Epic Games Launcher" => "Plateforme de distribution numérique pour les jeux PC",
            "7-Zip" => "Archiveur de fichiers open source avec un taux de compression élevé",
            "WinRAR archiver" => "Archiveur de fichiers avec un taux de compression élevé",
            "PeaZip" => {
                "Utilitaire d'archivage de fichiers gratuit. Ouvre et extrait les fichiers RAR, TAR, ZIP et plus encore"
            }
            "WinDirStat" => {
                "Visionneuse des statistiques d'utilisation du disque et outil de nettoyage"
            }
            "WizTree" => "Analyseur d'espace disque avec un balayage extrêmement rapide",
            "TreeSize Free" => "Gestionnaire d'espace disque",
            "Everything" => "Localise instantanément les fichiers et dossiers par leur nom",
            "TeraCopy" => "Copie des fichiers plus rapidement et plus sûrement",
            "File Converter" => "Convertisseur de fichiers par lots pour Windows",
            "Crystal Disk Info" => "Utilitaire de surveillance de l'état des disques durs",
            "Bulk Rename Utility" => "Logiciel de renommage de fichiers pour Windows",
            "IObit Unlocker" => {
                "Outil pour déverrouiller les fichiers utilisés par d'autres processus"
            }
            "HiBit Uninstaller" => {
                "Désinstalle complètement les logiciels tenaces, les applications Windows et les extensions de navigateur"
            }
            "SanDisk Dashboard" => "Outil de gestion des disques pour les SSD et clés USB SanDisk",
            "Rufus" => "Utilitaire pour créer des clés USB amorçables",
            "Advanced Renamer" => {
                "Utilitaire de renommage de fichiers par lot avec options avancées"
            }
            "RustDesk" => "Logiciel d'accès à distance et de support rapide, open source",
            "AnyDesk" => "Logiciel de bureau à distance pour l'accès et le support",
            "TeamViewer" => {
                "Contrôle à distance, partage de bureau, réunions en ligne, visioconférence et transfert de fichiers"
            }
            "UltraViewer" => {
                "Vous aide à contrôler l'ordinateur de votre partenaire pour le dépanner comme si vous étiez assis devant son écran"
            }
            "RealVNC Server" => "Logiciel d'accès à distance",
            "RealVNC Viewer" => "Logiciel d'accès à distance",
            "Chrome Remote Desktop" => {
                "Accès à votre ordinateur à distance via le navigateur Chrome"
            }
            "Parsec" => {
                "Le bureau à distance réinventé. Un accès sécurisé, flexible et sans effort à tout ce que vous faites, à tout moment et où que vous soyez"
            }
            "Parsec Virtual Display Driver" => {
                "Pilote d'affichage virtuel pour Parsec Remote Desktop"
            }
            "Parsec Virtual USB Driver" => "Pilote USB virtuel pour Parsec Remote Desktop",
            "InputLeap" => {
                "Logiciel KVM open source pour partager la souris et le clavier entre plusieurs ordinateurs"
            }
            "ImgBurn" => "Application légère de gravure CD / DVD / HD DVD / Blu-ray",
            "AnyBurn" => "Logiciel léger de gravure CD/DVD/Blu-ray",
            "CDBurnerXP" => "Logiciel gratuit de gravure CD/DVD/Blu-ray",
            "CCleaner" => "Outil d'optimisation et de nettoyage du système",
            "Snappy Driver Installer Origin" => "Installateur et mise à jour de pilotes",
            "Wise Disk Cleaner" => "Outil gratuit de nettoyage et de défragmentation du disque",
            "Wise Registry Cleaner" => "Outil de nettoyage et d'optimisation du registre",
            "UniGetUI" => {
                "Interface universelle de gestion de paquets prenant en charge WinGet, Chocolatey et plus encore"
            }
            "OpenRGB" => "Logiciel open source de contrôle de l'éclairage RGB",
            "OpenAudible" => {
                "Gestionnaire et convertisseur de livres audio pour les fichiers Audible"
            }
            "NAPS2" => "Application de numérisation de documents avec prise en charge de l'OCR",
            "IObit Uninstaller" => {
                "Désinstalle complètement les logiciels indésirables, les applications Windows et les extensions de navigateur"
            }
            "Revo Uninstaller" => {
                "Revo Uninstaller vous aide à désinstaller des logiciels et à supprimer facilement les programmes indésirables."
            }
            "Malwarebytes" => "Logiciel antimalware pour Windows",
            "Malwarebytes AdwCleaner" => "Outil de suppression des adwares pour Windows",
            "Windows Firewall Control" => "Application Malwarebytes Windows Firewall Control",
            "OnionShare" => {
                "Partagez des fichiers, hébergez des sites web et discutez de manière sécurisée et anonyme via le réseau Tor"
            }
            "Sniffnet" => "Outil de surveillance réseau pour analyser votre trafic Internet",
            "TeleGuard" => "Application de messagerie sécurisée avec chiffrement de bout en bout",
            "Python 3.13" => "Langage de programmation Python",
            "Notepad++" => "Éditeur de code source gratuit et remplacement du Bloc-notes",
            "WinSCP" => "Client gratuit SFTP, SCP, Amazon S3, WebDAV et FTP",
            "PuTTY" => "Client SSH et Telnet gratuit",
            "WinMerge" => "Outil libre de comparaison et de fusion",
            "Eclipse IDE for Java" => "IDE Java et plateforme de développement",
            "Microsoft Visual Studio Code" => {
                "Éditeur de code avec prise en charge des opérations de développement"
            }
            "Git" => "Système de gestion de versions distribué",
            "GitHub Desktop" => "Client de bureau GitHub",
            "Microsoft .NET Runtime 3.1" => "Runtime .NET 3.1 pour exécuter des applications",
            "Microsoft .NET Runtime 5.0" => "Runtime .NET 5.0 pour exécuter des applications",
            "Microsoft .NET Runtime 6.0" => "Runtime .NET 6.0 LTS pour exécuter des applications",
            "Microsoft .NET Runtime 7.0" => "Runtime .NET 7.0 pour exécuter des applications",
            "Microsoft .NET Runtime 8.0" => "Runtime .NET 8.0 LTS pour exécuter des applications",
            ".NET Framework 4.8.1" => "Pack développeur .NET Framework",
            "DirectX Runtime" => {
                "Composants runtime DirectX pour exécuter des jeux et des applications multimédias"
            }
            "Java Runtime Environment" => {
                "Environnement d'exécution Java pour lancer des applications Java"
            }
            "Visual C++ 2005 (x86)" => "Composants runtime Visual C++ 2005",
            "Visual C++ 2005 (x64)" => "Composants runtime Visual C++ 2005",
            "Visual C++ 2008 (x86)" => "Composants runtime Visual C++ 2008",
            "Visual C++ 2008 (x64)" => "Composants runtime Visual C++ 2008",
            "Visual C++ 2010 (x86)" => "Composants runtime Visual C++ 2010",
            "Visual C++ 2010 (x64)" => "Composants runtime Visual C++ 2010",
            "Visual C++ 2012 (x86)" => "Composants runtime Visual C++ 2012",
            "Visual C++ 2012 (x64)" => "Composants runtime Visual C++ 2012",
            "Visual C++ 2013 (x86)" => "Composants runtime Visual C++ 2013",
            "Visual C++ 2013 (x64)" => "Composants runtime Visual C++ 2013",
            "Visual C++ 2015-2022 (x86)" => "Composants runtime Visual C++ 2015-2022",
            "Visual C++ 2015-2022 (x64)" => "Composants runtime Visual C++ 2015-2022",
            "Tabby" => "Terminal SSH et gestionnaire de connexions",
            "Riot Games Launcher" => {
                "Lanceur du client Riot (une connexion manuelle peut être nécessaire)"
            }
            "Helium Browser" => "Navigateur Chromium axé sur la confidentialité",
            "DataGrip" => "IDE SQL de JetBrains",
            "Zed" => "Éditeur de code haute performance",
            "debloater_title" => "Débloatage",
            "debloater_subtitle" => {
                "Supprime les composants Windows et les fonctionnalités facultatives."
            }
            "debloater_search" => "Rechercher des paquets",
            "debloater_nothing_selected" => "Aucune sélection",
            "debloater_installed_status" => "Installé",
            "debloater_not_installed_status" => "Non installé",
            "debloater_installing" => "Installation des éléments sélectionnés...",
            "debloater_removing" => "Suppression des éléments sélectionnés...",
            "debloater_all_items" => "Tous les éléments",
            "debloater_installed_only" => "Installés uniquement",
            "debloater_not_installed_only" => "Non installés uniquement",
            "debloater_refresh" => "Actualiser",
            "debloater_install_selected" => "Installer la sélection",
            "debloater_remove_selected" => "Supprimer la sélection",
            "debloater_loading" => "Chargement des paquets...",
            "debloater_scanning" => {
                "Analyse des applications installées, des capacités et des fonctionnalités facultatives."
            }
            "debloater_none" => "Aucun paquet à afficher.",
            "debloater_cannot_reinstall" => "Impossible de réinstaller",
            "debloater_meta" => "Paquet : {}\nCatégorie : {}\nGroupe : {}",
            "debloater_confirm_install_title" => "Confirmer l'installation",
            "debloater_confirm_remove_title" => "Confirmer la suppression",
            "debloater_confirm_install_desc" => "Ces applications seront installées :",
            "debloater_confirm_remove_desc" => "Ces applications seront supprimées :",
            "debloater_confirm_install_btn" => "Confirmer l'installation",
            "debloater_confirm_remove_btn" => "Confirmer la suppression",
            "debloater_cancel" => "Annuler",
            "debloater_installed" => "Installé",
            "debloater_not_installed" => "Non installé",
            "debloater_result_install" => "Installés : {}  Échecs : {}",
            "debloater_result_remove" => "Supprimés : {}  Échecs : {}",
            "debloater_tab_0" => "Applications Windows",
            "debloater_tab_1" => "Capacités",
            "debloater_tab_2" => "Fonctionnalités facultatives",
            "processes_title" => "Processus",
            "processes_subtitle" => {
                "Inspectez les processus, l'affinité et les paramètres de priorité."
            }
            "processes_refresh" => "Actualiser",
            "processes_active_only" => "Actifs uniquement",
            "processes_visible" => "Visibles :",
            "processes_total_cpu" => "CPU total :",
            "processes_refreshing" => "Actualisation de la liste des processus...",
            "processes_waiting_first" => "En attente du premier rafraîchissement...",
            "processes_reload_queued" => "Rechargement en file d'attente",
            "processes_pid" => "PID",
            "processes_name" => "Nom",
            "processes_cpu" => "CPU %",
            "processes_priority" => "Priorité",
            "processes_affinity" => "Affinité",
            "processes_status" => "État",
            "processes_collapse_tree" => "Réduire l'arborescence",
            "processes_expand_tree" => "Développer l'arborescence",
            "processes_cpu_priority" => "Priorité CPU",
            "processes_current" => "Actuel",
            "processes_always" => "Toujours",
            "processes_io_priority" => "Priorité E/S",
            "processes_affinity_menu" => "Affinité",
            "processes_open_editor" => "Ouvrir l'éditeur",
            "processes_all_cores" => "Tous les cœurs",
            "processes_affinity_prefix" => "CPU",
            "processes_selected" => "Processus sélectionné",
            "processes_realtime_title" => "Définir la priorité temps réel ?",
            "processes_realtime_warn" => {
                "Le temps réel peut figer la réactivité de Windows. Ne continuez que si vous comprenez le risque."
            }
            "processes_cancel" => "Annuler",
            "processes_confirm" => "Confirmer",
            "processes_affinity_title" => "Affinité CPU - {} (PID {})",
            "processes_affinity_mask" => "Masque d'affinité (hex) : {}",
            "processes_invert" => "Inverser",
            "processes_clear" => "Effacer",
            "processes_close" => "Fermer",
            "processes_apply" => "Appliquer",
            "processes_last_refresh" => "Dernière actualisation : il y a {} s",
            "processes_priority_unknown" => "Inconnu",
            "processes_priority_idle" => "Inactif",
            "processes_priority_below_normal" => "Inférieur à la normale",
            "processes_priority_normal" => "Normal",
            "processes_priority_above_normal" => "Supérieur à la normale",
            "processes_priority_high" => "Élevé",
            "processes_priority_realtime" => "Temps réel",
            "processes_priority_background" => "Arrière-plan : 4 (faible I/O et CPU)",
            "processes_priority_low" => "Faible",
            "processes_priority_always_below" => "Inférieur",
            "processes_priority_always_above" => "Supérieur",
            "processes_status_running" => "En cours d'exécution",
            "processes_status_sleeping" => "En veille",
            "processes_status_idle" => "Inactif",
            "processes_status_zombie" => "Zombie",
            "processes_status_stopped" => "Arrêté",
            "processes_status_tracing" => "Traçage",
            "processes_status_dead" => "Mort",
            "processes_status_wakekill" => "Wakekill",
            "processes_status_waking" => "Réveil",
            "processes_status_lockblocked" => "Verrouillage bloqué",
            "processes_status_parked" => "Parqué",
            "processes_status_unknown" => "Inconnu",
            "processes_process_scan_failed" => "Échec de l'analyse des processus",
            "processes_openprocess_failed" => "Échec de OpenProcess pour le PID {} : {}",
            "processes_get_affinity_failed" => {
                "Échec de GetProcessAffinityMask pour le PID {} : {}"
            }
            "processes_invalid_system_mask" => "Masque d'affinité système invalide pour le PID {}",
            "processes_invalid_affinity_mode" => "Mode d'affinité invalide",
            "processes_set_priority_failed" => "Échec de SetPriorityClass pour le PID {}",
            "processes_set_io_failed" => "Échec de SetProcessInformation(I/O) pour le PID {}",
            "processes_set_affinity_failed" => "Échec de SetProcessAffinityMask pour le PID {}",
            "processes_perfoptions_open" => "Impossible de créer/ouvrir la clé PerfOptions",
            "processes_perfoptions_write_cpu" => "Impossible d'écrire CpuPriorityClass",
            "processes_perfoptions_write_io" => "Impossible d'écrire IoPriority",
            "processes_invalid_priority_level" => "Niveau de priorité invalide",
            "latency_title" => "Latence",
            "latency_subtitle" => "Analyser la latence USB et la topologie des périphériques.",
            "latency_button" => "Analyser la latence USB",
            "latency_analyzing" => "Analyse...",
            "latency_starting" => "Démarrage de l'analyse...",
            "latency_topology" => "Analyse de la topologie USB...",
            "latency_scanning" => {
                "Analyse des périphériques PnP, de la chaîne des contrôleurs, du MSI et des paramètres d'alimentation."
            }
            "latency_begin" => "Cliquez sur « Analyser la latence USB » pour commencer.",
            "latency_loading_fail" => "Échec de l'analyse de la latence USB",
            "latency_admin" => "Veuillez exécuter en tant qu'administrateur.",
            "latency_error_title" => "ERREUR - ÉCHEC DE L'ANALYSE DE LATENCE USB",
            "latency_failed" => "Échec de l'analyse de la latence USB",
            "latency_progress_power" => "Vérification des options d'alimentation...",
            "latency_progress_controllers" => "Analyse des contrôleurs USB...",
            "latency_progress_usb_registry_tree" => "Lecture de l'arborescence du registre USB...",
            "latency_progress_inputs" => "Recherche des périphériques d'entrée...",
            "latency_progress_hubs" => "Suivi des périphériques vers les hubs racine...",
            "latency_progress_verify" => {
                "Vérification de la topologie et des indices d'alimentation..."
            }
            "latency_progress_report" => "Génération du rapport...",
            _ => t(Language::English, key),
            //_ => "",
        },
        Language::Spanish => match key {
            "nav_title" => "Navegación",
            "nav_subtitle" => "Centro de control de Winchisel",
            "home" => "Inicio",
            "debloater" => "Debloater",
            "downloads" => "Aplicaciones y descargas",
            "performance" => "Rendimiento",
            "processes" => "Procesos",
            "latency" => "Latencia",
            "settings" => "Configuración",
            "check_updates" => "Buscar actualizaciones",
            "donate" => "Donar",
            "bug_report" => "Informar de un error",
            "status_admin" => "Administrador",
            "status_standard" => "Estándar",
            "settings_title" => "Configuración",
            "settings_subtitle" => {
                "Mantén el comportamiento de la app alineado con tu flujo de trabajo."
            }
            "settings_application" => "Aplicación",
            "settings_saved_auto" => "Estos ajustes se guardan automáticamente.",
            "language" => "Idioma",
            "sidebar_languages" => "Idiomas",
            "archive-restore" => "Restaurar",
            "settings-2" => "Configuración",
            "shield-check" => "Estado de seguridad",
            "refresh-cw" => "Actualizar",
            "monitor" => "Consola",
            "Winchisel-Updater" => "Actualizador de Winchisel",
            "Disabled (Recommended)" => "Desactivado (recomendado)",
            "0" => "0",
            "1" => "1",
            "app_name_m365_copilot_office_hub" => "MS 365 Copilot (Office Hub)",
            "app_name_remote_assistance" => "Aplicación de asistencia remota",
            "app_name_classic_paint" => "Aplicación Paint clásica",
            "windows-app-office-hub" => "Microsoft 365 Copilot (antes conocido como Office hub)",
            "windows-app-remote-assistance" => "Aplicación de asistencia remota",
            "capability-paint-legacy" => "Aplicación Paint clásica",
            "open_logs" => "Abrir registros",
            "check_updates_startup" => "Buscar actualizaciones al iniciar",
            "show_console" => "Mostrar consola",
            "system_protection" => "Protección del sistema",
            "restore_point" => "Punto de restauración del sistema",
            "restore_point_desc" => {
                "Crea un punto de restauración antes de cambios importantes del sistema"
            }
            "create_restore_point" => "Crear punto de restauración",
            "performance_title" => "Rendimiento",
            "performance_subtitle" => "Ajustes para juegos y rendimiento",
            "performance_search" => "Buscar ajustes de rendimiento...",
            "performance_quick" => "Acciones rápidas",
            "performance_apply_recommended" => "Aplicar ajustes recomendados",
            "performance_reset_defaults" => "Restaurar valores predeterminados de Windows",
            "performance_loading" => "Cargando ajustes de rendimiento...",
            "performance_empty" => "No hay ajustes de rendimiento para mostrar.",
            "performance_current_default" => "Predeterminado: ",
            "performance_current_recommended" => "Recomendado: ",
            "gaming-game-mode" => {
                "Optimiza tu PC para jugar desactivando procesos en segundo plano"
            }
            "gaming-performance-explorer-mouse-precision" => {
                "Ajusta la velocidad del cursor según la velocidad de movimiento (aceleración del ratón). La mayoría de los jugadores competitivos la desactivan para apuntar con más consistencia en juegos FPS"
            }
            "gaming-performance-mouse-hover-time" => {
                "Controla cuánto tiempo debes mantener el cursor sobre un elemento antes de que se active (en milisegundos). Los valores más bajos hacen que tooltips, menús y efectos al pasar el ratón aparezcan más rápido. El valor predeterminado es 400 ms"
            }
            "gaming-performance-autostart-delay" => {
                "Retrasa el inicio de las aplicaciones 10 segundos después del arranque para mejorar la respuesta inicial del sistema. Windows se vuelve más rápido de usar, pero las apps de inicio tardan más en cargar"
            }
            "gaming-background-apps" => {
                "Usa la Directiva de grupo para controlar si las aplicaciones pueden ejecutarse en segundo plano. Force Deny elimina los ajustes por aplicación de Windows. Usa User in Control si necesitas apps como Teams, Zoom o WhatsApp"
            }
            "gaming-storage-sense" => {
                "Libera espacio automáticamente eliminando archivos temporales, vaciando la papelera y gestionando descargas"
            }
            "gaming-performance-explorer-search" => {
                "Busca en todo el sistema de archivos en lugar de limitarse a ubicaciones indexadas. Ofrece resultados más completos, pero es considerablemente más lento que la búsqueda indexada y aumenta la actividad del disco"
            }
            "gaming-performance-search-webview2" => {
                "Permite que la Búsqueda de Windows use WebView2 (Edge) para renderizar los resultados. Desactivarlo eliminará los procesos de Edge creados por SearchHost.exe y reducirá el uso de recursos. Usa una anulación no documentada de Windows Feature Management (ID 37926450), que puede cambiar en futuras actualizaciones"
            }
            "gaming-performance-wallpaper-compression" => {
                "Permite a Windows comprimir los fondos de pantalla para ahorrar espacio en disco y mejorar el rendimiento. Solo afecta a las imágenes en formato JPEG."
            }
            "gaming-performance-explorer-menu-show-delay" => {
                "Añade un pequeño retraso antes de que aparezcan los menús (400 ms, valor predeterminado de Windows) o muéstralos al instante (0 ms) para acelerar la navegación"
            }
            "gaming-explorer-alt-tab-filter" => {
                "Alt+Tab solo muestra ventanas abiertas tradicionales en lugar de incluir pestañas de Microsoft Edge y otras sugerencias de Windows"
            }
            "gaming-win32-priority" => {
                "Configura cómo Windows distribuye el tiempo de CPU entre aplicaciones en primer plano y servicios en segundo plano"
            }
            "gaming-system-responsiveness" => {
                "Minimiza la interrupción de las tareas en segundo plano asignando más tiempo de CPU a tu juego o aplicación multimedia activa"
            }
            "gaming-cpu-priority" => {
                "Da a los juegos una prioridad de planificación de CPU más alta para dedicarles más tiempo de procesador"
            }
            "gaming-scheduling-category" => {
                "Asigna una categoría de planificación de alta prioridad para asegurar que los juegos reciban una asignación preferente de recursos del sistema"
            }
            "gaming-performance-svchost-split-threshold" => {
                "Define el umbral de memoria que determina cuándo Windows separa los servicios en procesos svchost.exe distintos. Los valores más altos agrupan más servicios, reduciendo el número de procesos. Selecciona el valor que corresponda a la RAM de tu sistema"
            }
            "gaming-gpu-priority" => {
                "Da a los juegos una prioridad más alta de planificación de GPU para mejorar el rendimiento gráfico y los FPS"
            }
            "gaming-gpu-scheduling" => {
                "Permite que tu GPU gestione su propia memoria y planificación para reducir la latencia y mejorar el rendimiento"
            }
            "gaming-directx-flip-model" => {
                "Reduce la latencia y aprovecha funciones avanzadas en juegos compatibles usando el modelo de presentación DirectX Flip"
            }
            "gaming-directx-vrr-optimizations" => {
                "Activa optimizaciones VRR (G-Sync/FreeSync) para una experiencia más fluida. Requiere un monitor compatible con VRR; este ajuste no tiene efecto si tu monitor no lo soporta"
            }
            "gaming-directx-auto-hdr" => {
                "Convierte automáticamente contenido SDR a HDR para mejorar colores y brillo. Requiere una pantalla compatible con HDR y HDR activado; este ajuste no tiene efecto si tu pantalla no lo soporta"
            }
            "gaming-nvidia-sharpening" => {
                "Activa el filtro heredado de nitidez de imagen de NVIDIA para mejorar la claridad visual. Solo funciona con controladores NVIDIA antiguos; los controladores nuevos deberían usar la nitidez del Panel de control de NVIDIA"
            }
            "gaming-fullscreen-optimizations" => {
                "Permite a Windows optimizar juegos que se ejecutan en pantalla completa. Desactivarlo puede resolver problemas de rendimiento o tartamudeo en algunos juegos antiguos que no funcionan bien con la optimización de pantalla completa sin bordes"
            }
            "gaming-performance-desktop-composition" => {
                "Activa los efectos visuales gestionados por Desktop Window Manager. Desactivarlo puede aportar pequeñas mejoras de rendimiento en hardware antiguo, pero afectará a los efectos Aero"
            }
            "gaming-auto-color-management" => {
                "Permite a Windows gestionar automáticamente los perfiles de color de todas las pantallas conectadas que lo admitan"
            }
            "gaming-disable-mpo" => {
                "Compone varias capas de pantalla por hardware usando la GPU. Desactivarlo puede resolver parpadeos, pantallas negras y tartamudeo en configuraciones con varios monitores"
            }
            "gaming-disable-mpo-min-fps" => {
                "Permite a Desktop Window Manager cambiar dinámicamente las apps entre modos de superposición según la tasa de fotogramas. Desactivarlo puede corregir tartamudeos en navegadores y Discord sin desactivar completamente MPO"
            }
            "gaming-network-throttling" => {
                "Controla el limitador de tasa de paquetes de red para aplicaciones multimedia. Se recomienda dejarlo activado (valor predeterminado: 10 paquetes/ms) porque ofrece mejor latencia DPC para jugar que desactivarlo por completo"
            }
            "gaming-nagle-algorithm" => {
                "Agrupa pequeños paquetes de red antes de enviarlos para reducir la sobrecarga. Desactívalo para reducir la latencia en juegos en línea o déjalo activado para una mayor eficiencia de red"
            }
            "gaming-dns-server" => {
                "Selecciona un servidor DNS para todos los adaptadores de red. Los cambios se aplican a cada adaptador de tu sistema (Wi‑Fi y Ethernet). Usa “Automático” para restaurar el DNS predeterminado de tu ISP/router"
            }
            "gaming-virtualization-based-security" => {
                "Aísla partes de la memoria para proteger el sistema de vulnerabilidades. Desactivarlo puede mejorar el rendimiento en juegos, pero reduce la seguridad del sistema"
            }
            "gaming-memory-integrity" => {
                "Impide la introducción de código malicioso en procesos de alta seguridad. Desactivarlo puede mejorar el rendimiento en juegos, pero reduce la seguridad del sistema"
            }
            "gaming-xbox-game-dvr" => {
                "Graba clips de juego y toma capturas de pantalla con la superposición de Xbox Game Bar. Desactivarlo reduce el uso de CPU/GPU y puede mejorar los FPS"
            }
            "gaming-game-bar-controller" => {
                "Permite que tu mando Xbox o compatible abra Game Bar al pulsar el botón Xbox. Desactiva esta opción para evitar activaciones accidentales de Game Bar mientras juegas"
            }
            "gaming-game-bar-tips" => {
                "Muestra consejos y sugerencias sobre las funciones de Game Bar cuando se abre la superposición. Desactivarlo reducirá las distracciones durante el juego"
            }
            "gaming-performance-background-services" => {
                "Reduce el tiempo de espera de inicio de los servicios de Windows de 60 a 30 segundos. Esto puede reducir ligeramente el tiempo de arranque"
            }
            "gaming-sysmain-service" => {
                "Precarga aplicaciones de uso frecuente en la RAM para acelerar el arranque. Se recomienda Automático para discos duros o sistemas mixtos; “Manual” o “Desactivado” solo es adecuado para sistemas SSD puros"
            }
            "gaming-performance-prefetch" => {
                "Precarga aplicaciones de uso frecuente y archivos de arranque en memoria para acelerar el inicio. Generalmente recomendado para HDD y no para SSD"
            }
            "gaming-windows-search-service" => {
                "Indexa archivos y carpetas para obtener resultados de búsqueda más rápidos. Desactivarlo reducirá la actividad de CPU y disco en segundo plano, pero romperá la búsqueda de Outlook y hará que el menú Inicio y el Explorador de archivos sean lentos o poco fiables"
            }
            "gaming-print-spooler-service" => {
                "Gestiona los trabajos de impresión enviados a impresoras. Si no usas impresora, ponlo en Manual o Desactivado para liberar recursos del sistema"
            }
            "gaming-telemetry-service" => {
                "Envía datos de uso y diagnósticos a Microsoft. Ponerlo en Manual o Desactivado reduce el uso de red y CPU en segundo plano"
            }
            "gaming-connected-devices-platform-service" => {
                "Activa experiencias entre dispositivos como el enlace del teléfono y el uso compartido cercano. Desactivarlo reduce la actividad en segundo plano y el registro de interacciones con dispositivos"
            }
            "gaming-compatibility-assistant-service" => {
                "Supervisa programas en busca de problemas de compatibilidad y sugiere soluciones. Desactivarlo evitará avisos de compatibilidad y ahorrará pequeños recursos del sistema"
            }
            "gaming-error-reporting-service" => {
                "Recopila y envía datos de fallos a Microsoft. Desactivarlo evita informes de errores, reduce el tráfico de red y mejora la protección de datos con un impacto mínimo en el sistema"
            }
            "gaming-geolocation-service" => {
                "Rastrea tu ubicación física para apps y servicios. Desactivarlo mejora la privacidad y evita el seguimiento de ubicación, pero las apps no podrán usar funciones de localización"
            }
            "gaming-retail-demo-service" => {
                "Controla la actividad del dispositivo en modo demostración comercial. En PC, desactivarlo puede ser seguro, ya que solo sirve para exhibición en tiendas"
            }
            "gaming-insider-service" => {
                "Gestiona las funciones del programa Windows Insider y las compilaciones preliminares. Desactivarlo es seguro si no participas en el programa Windows Insider"
            }
            "gaming-phone-service" => {
                "Gestiona el estado de telefonía del dispositivo. Puedes desactivar fácilmente esta función si no usas funciones de conexión telefónica ni realizas llamadas desde el PC"
            }
            "gaming-wallet-service" => {
                "Proporciona funciones de monedero para escenarios de pago y NFC. Es seguro desactivarlo si no usas las funciones de Microsoft Wallet"
            }
            "gaming-smart-card-services" => {
                "Activa la función de lector de tarjeta inteligente para autenticación de seguridad. Es seguro desactivarlo si no usas tarjetas inteligentes físicas ni lectores de tarjetas"
            }
            "gaming-maps-broker-service" => {
                "Proporciona acceso a mapas descargados para las aplicaciones. Déjalo en Manual para permitir el acceso a los mapas cuando haga falta y evitar actividad innecesaria en segundo plano"
            }
            "gaming-fax-service" => {
                "Permite enviar y recibir faxes. Para la mayoría de usuarios, desactivarlo es seguro porque la función de fax apenas se usa en sistemas modernos"
            }
            "gaming-wmp-network-service" => {
                "Comparte bibliotecas de Windows Media Player con otros reproductores y dispositivos multimedia de la red. Puedes desactivar esta función con seguridad si no compartes contenido multimedia en tu red"
            }
            "gaming-mixed-reality-service" => {
                "Ejecuta aplicaciones OpenXR en dispositivos Windows Mixed Reality. Es seguro desactivarlo si no usas cascos VR o AR"
            }
            "gaming-mobile-hotspot-service" => {
                "Permite compartir la conexión a Internet con otros dispositivos. Ponlo en Manual para mantener la función disponible y evitar actividad innecesaria en segundo plano"
            }
            "gaming-sms-router-service" => {
                "Reenvía mensajes SMS según reglas. Puedes desactivar esta función con seguridad si no usas SMS en tu PC"
            }
            "gaming-parental-controls-service" => {
                "Activa controles parentales y funciones de seguridad familiar. Puedes desactivarlo con seguridad si no usas controles parentales"
            }
            "gaming-payments-nfc-service" => {
                "Gestiona pagos y elementos seguros de la comunicación de campo cercano. Es seguro desactivarlo si no usas funciones de pago NFC"
            }
            "gaming-spot-verifier-service" => {
                "Comprueba posibles corrupciones del sistema de archivos. Ponlo en Manual para permitir comprobaciones cuando haga falta y reducir la actividad en segundo plano"
            }
            "gaming-remote-access-manager" => {
                "Gestiona conexiones VPN y de acceso telefónico. Ponlo en Manual para reducir la actividad en segundo plano y mantener la VPN disponible cuando sea necesario."
            }
            "gaming-remote-access-auto" => {
                "Se conecta automáticamente a redes remotas cuando los programas referencian recursos remotos. Puedes desactivar esta función con seguridad si no usas la conexión automática de la VPN"
            }
            "gaming-remote-desktop-services" => {
                "Permite a los usuarios conectarse interactivamente a un ordenador remoto. Ponlo en Manual para reducir la actividad en segundo plano y mantener disponible el escritorio remoto."
            }
            "gaming-remote-desktop-configuration" => {
                "Gestiona los Servicios de Escritorio remoto y las configuraciones relacionadas. Ponlo en Manual para reducir la actividad en segundo plano y mantener disponible el escritorio remoto"
            }
            "gaming-remote-desktop-port-redirector" => {
                "Activa la redirección de dispositivos locales para conexiones de escritorio remoto. Es seguro desactivarlo si no necesitas compartir dispositivos locales durante sesiones de escritorio remoto"
            }
            "gaming-xbox-auth-manager" => {
                "Proporciona servicios de autenticación y autorización para Xbox Live. Es seguro desactivarlo si no usas Xbox Game Pass, juegos de Microsoft Store o funciones de Xbox"
            }
            "gaming-xbox-game-save" => {
                "Sincroniza partidas guardadas con la nube de Xbox Live. Solo es necesario para Xbox Game Pass y juegos de Microsoft Store con guardado en la nube"
            }
            "gaming-xbox-networking" => {
                "Da soporte a las redes multijugador de Xbox Live. Es necesario para juegos multijugador de Xbox, pero no para Steam/Epic u otras plataformas"
            }
            "gaming-biometric-service" => {
                "Activa el inicio de sesión por huella y reconocimiento facial mediante Windows Hello. Seguro de desactivar en sistemas de escritorio sin hardware biométrico"
            }
            "gaming-touch-keyboard-service" => {
                "Gestiona la experiencia de entrada de Windows, incluido el teclado táctil, la entrada con lápiz/stylus, el panel de escritura a mano, el panel de emoji (Win+.), y el teclado del mando Xbox. Desactivarlo romperá toda entrada de teclado virtual/software, pero es seguro en equipos de escritorio sin pantalla táctil, stylus ni mando"
            }
            "gaming-sensor-monitoring-service" => {
                "Supervisa varios sensores como la luz ambiental y la orientación. Seguro de desactivar en sistemas de escritorio sin hardware de sensores"
            }
            "gaming-sensor-data-service" => {
                "Entrega a las aplicaciones datos de diversos sensores. Seguro de desactivar en sistemas de escritorio sin hardware de sensores"
            }
            "gaming-ai-fabric-service" => {
                "El servicio Windows AI Fabric (WSAIFabricSvc) gestiona cargas de trabajo de IA. Desactiva esta opción si no usas funciones de IA de Windows"
            }
            "CompatibilityAppraiserTask" => {
                "Recopila datos de telemetría de compatibilidad de programas para actualizaciones de Windows. Funciona junto con el servicio de Experiencias de usuario conectado y telemetría. Desactívalo para reducir la telemetría y la actividad del sistema en segundo plano"
            }
            "ProgramDataUpdaterTask" => {
                "Actualiza la base de datos de compatibilidad de programas con información sobre las aplicaciones instaladas. Desactiva esta opción para reducir la recopilación de telemetría"
            }
            "CEIPConsolidatorTask" => {
                "Consolida y sube datos de uso como parte del programa de mejora de la experiencia del cliente. Funciona con el servicio de Experiencias de usuario conectado y telemetría. Desactívalo para mejorar la privacidad"
            }
            "UsbCeipTask" => {
                "Recopila datos de telemetría relacionados con dispositivos USB para el programa de mejora de la satisfacción del cliente. Desactívalo para reducir la telemetría"
            }
            "DiskDiagnosticTask" => {
                "Recopila información de diagnóstico del disco duro y datos S.M.A.R.T. para Microsoft. Desactiva esta opción para reducir la actividad del disco en segundo plano y la telemetría"
            }
            "FeedbackDmClientTask" => {
                "Recopila comentarios y datos de diagnóstico para Microsoft. Desactiva esta opción para mejorar la privacidad y reducir la telemetría"
            }
            "FeedbackDmClientDownloadTask" => {
                "Descarga escenarios de comentarios y datos de configuración desde Microsoft. Desactiva esta opción para reducir la telemetría y la actividad de red"
            }
            "ErrorReportingQueueTask" => {
                "Encola informes de fallos y datos de error para enviarlos a Microsoft. Funciona junto con el Servicio de informes de errores de Windows. Desactiva ambos para impedir la recopilación de datos de fallos"
            }
            "SqmTask" => {
                "Recopila métricas de calidad y datos de fiabilidad del software para la telemetría de Microsoft. Desactívalo para mejorar la privacidad"
            }
            "MareBackupTask" => {
                "Realiza copias de seguridad de los datos de Microsoft Assisted Recovery. Desactiva esta opción para reducir la actividad del sistema en segundo plano"
            }
            "StartupAppTask" => {
                "Rastrea y supervisa las aplicaciones de inicio para telemetría y diagnósticos. Desactívalo para reducir la telemetría"
            }
            "MapsUpdateTask" => {
                "Actualiza los datos de mapas sin conexión para la aplicación Mapas de Windows. Desactiva esta opción cuando no uses la aplicación Mapas para ahorrar ancho de banda y espacio de almacenamiento"
            }
            "AutochkProxyTask" => {
                "Realiza comprobaciones del disco duro y recopila datos de diagnóstico. Considera dejar activada la supervisión del estado del disco"
            }
            "FamilySafetyTask" => {
                "Supervisa la configuración y el uso de la seguridad familiar. Desactiva esta opción si no usas las funciones de seguridad familiar"
            }
            "PowerEfficiencyTask" => {
                "Analiza el consumo de energía del sistema y recopila datos de eficiencia energética. Desactiva esta opción para reducir la telemetría y el análisis en segundo plano"
            }
            "WindowsAIRecallConfig" => {
                "Tareas programadas de Windows AI, incluida la configuración de Recall. Desactiva esta opción para evitar que las funciones de IA se ejecuten en segundo plano"
            }
            "WindowsAIRecallPipeline" => {
                "Tarea de canalización de Windows AI Recall. Desactiva esta opción para evitar que la canalización de Recall procese en segundo plano"
            }
            "OfficeActionsServer" => {
                "Tarea programada del servidor de acciones de Office AI. Desactiva esta opción para evitar que Office AI se ejecute en segundo plano"
            }
            "visual-effects-mode" => "Elige cómo muestra Windows los efectos visuales",
            "ui-effects" => "Activa efectos de animación para controles y elementos de la interfaz",
            "window-animation" => {
                "Muestra una animación fluida cuando las ventanas se minimizan o maximizan"
            }
            "taskbar-animations" => {
                "Controla los efectos de animación de la barra de tareas al abrir, cerrar y cambiar entre ventanas"
            }
            "enable-peek" => {
                "Permite ver el escritorio al pasar el cursor sobre el botón Mostrar escritorio"
            }
            "menu-animation" => {
                "Anima los menús al mostrarse con efectos de desvanecimiento o deslizamiento"
            }
            "fade-tooltip" => {
                "Anima los tooltips al mostrarse con efectos de desvanecimiento o deslizamiento"
            }
            "fade-menu-items" => {
                "Oculta los elementos del menú después de seleccionarlos antes de cerrarlo"
            }
            "taskbar-thumbnails" => {
                "Guarda vistas previas en miniatura de las ventanas de la barra de tareas para una visualización más rápida"
            }
            "mouse-shadow" => "Muestra un efecto de sombra debajo del puntero del ratón",
            "window-shadows" => "Muestra efectos de sombra debajo de las ventanas",
            "show-thumbnails" => {
                "Muestra vistas previas de imágenes y documentos en lugar de iconos de archivo genéricos"
            }
            "translucent-selection" => {
                "Muestra un cuadro de selección semitransparente mientras arrastras para seleccionar varios archivos u objetos"
            }
            "drag-full-windows" => {
                "Al arrastrar, muestra el contenido de la ventana en lugar de solo un contorno"
            }
            "combo-box-animation" => {
                "Anima los cuadros combinados al abrirse con un efecto deslizante"
            }
            "font-smoothing" => {
                "Aplica antialiasing al texto para conseguir fuentes más suaves y legibles en pantalla"
            }
            "smooth-scroll-listboxes" => {
                "Permite un desplazamiento fluido en las listas en lugar de saltos bruscos"
            }
            "drop-shadows" => {
                "Añade efectos de sombra detrás del texto de los iconos del escritorio para mejorar la legibilidad sobre el fondo"
            }
            "gaming-narrator-hotkey" => {
                "Activa el atajo Win+Ctrl+Enter para iniciar rápidamente el Narrador de Windows"
            }
            "accessibility-stickykeys-hotkey" => {
                "Activa el atajo de teclado para habilitar Teclas especiales pulsando la tecla Mayús cinco veces"
            }
            "accessibility-filterkeys-hotkey" => {
                "Activa el atajo de teclado para habilitar Teclas de filtro manteniendo pulsada la tecla Mayús derecha durante 8 segundos"
            }
            "accessibility-togglekeys-hotkey" => {
                "Para activar Teclas de alternancia, usa el atajo de teclado manteniendo pulsada la tecla Bloq Num durante 5 segundos. Esto reproducirá sonidos cuando se pulse Bloq Mayús/Bloq Num/Bloq Despl"
            }
            "accessibility-mousekeys-hotkey" => {
                "Activa el atajo de teclado para habilitar Teclas del mouse, que permite usar el teclado numérico para controlar el puntero"
            }
            "accessibility-highcontrast-hotkey" => {
                "Para activar el modo de alto contraste, usa el atajo de teclado pulsando Alt izquierda + Mayús izquierda + Impr Pant"
            }
            "performance_group_0" => "Juegos",
            "performance_group_1" => "Procesador",
            "performance_group_2" => "Gráficos",
            "performance_group_3" => "Red",
            "performance_group_4" => "Seguridad",
            "performance_group_5" => "Xbox",
            "performance_group_6" => "Servicios del sistema",
            "performance_group_7" => "Tareas programadas",
            "performance_group_8" => "Efectos visuales",
            "performance_group_9" => "Accesibilidad",
            "home_system" => "Sistema",
            "home_processor" => "Procesador",
            "home_graphics" => "Gráficos",
            "home_memory" => "Memoria",
            "home_storage" => "Almacenamiento",
            "home_windows" => "Windows",
            "home_uptime" => "Tiempo de actividad",
            "home_performance" => "Rendimiento",
            "home_product_name" => "Nombre del producto del sistema",
            "home_cpu" => "CPU",
            "home_cpu_model" => "Modelo de CPU",
            "home_cores" => "Núcleos",
            "home_gpu" => "GPU",
            "home_memory_total" => "Memoria total",
            "home_memory_used" => "Memoria usada",
            "home_version" => "Versión",
            "home_kernel" => "Kernel",
            "home_name" => "Nombre",
            "home_bios_version" => "Versión de BIOS",
            "home_bios_date" => "Fecha de BIOS",
            "home_unknown_cpu" => "CPU desconocida",
            "home_unknown_pc" => "PC desconocido",
            "home_unknown_os" => "SO desconocido",
            "home_unknown_kernel" => "Kernel desconocido",
            "home_unknown_model" => "Modelo desconocido",
            "home_unknown_vendor" => "Fabricante desconocido",
            "home_unknown_bios" => "BIOS desconocida",
            "home_unknown_date" => "Fecha desconocida",
            "home_unknown_gpu" => "GPU desconocida",
            "home_unknown_vram" => "VRAM desconocida",
            "home_unknown_value" => "Desconocido",
            "home_cores_suffix" => "{} núcleos",
            "home_gb_total" => "{:.1} GB en total",
            "home_gb_used" => "{:.1} GB usados",
            "home_tb_total" => "{:.2} TB en total",
            "home_tb_used" => "{:.2} TB usados",
            "home_uptime_fmt" => "{}d {:02}h {:02}m",
            "home_update_status" => "Estado de las actualizaciones",
            "update_available_prefix" => "Actualización disponible:",
            "update_failed_prefix" => "Error al comprobar actualizaciones:",
            "update_newer_version_prefix" => "Hay una versión más reciente disponible:",
            "update_ready" => "Listo",
            "update_checked" => "Ya estás en la versión más reciente.",
            "settings_saved" => "Ajustes guardados",
            "update_no_found_title" => "No se encontró ninguna actualización",
            "update_available_title" => "Actualización disponible",
            "update_failed_title" => "Error al comprobar actualizaciones",
            "update_up_to_date" => "Ya estás en la versión más reciente.",
            "update_error_check_updates" => "Error al comprobar actualizaciones",
            "update_error_read_response" => "No se pudo leer la respuesta",
            "update_error_parse_json" => "No se pudo analizar JSON",
            "update_error_no_tag_name" => "No hay tag_name en la respuesta",
            "update_error_download" => "No se pudo descargar la actualización",
            "update_error_create_temp_file" => "No se pudo crear el archivo temporal",
            "update_error_write_update_file" => "No se pudo escribir el archivo de actualización",
            "update_error_download_too_small" => "El archivo descargado es demasiado pequeño",
            "update_error_resolve_current_exe" => "No se pudo resolver el ejecutable actual",
            "update_error_write_update_script" => "No se pudo escribir el script de actualización",
            "update_error_launch_updater" => "No se pudo iniciar el actualizador",
            "update_download_restart" => "Descarga y reinicia la app para instalarla.",
            "update_failed" => "Error al comprobar actualizaciones.",
            "update_close" => "Cerrar",
            "update_check_again" => "Comprobar de nuevo",
            "update_download_restart_btn" => "Descargar y reiniciar",
            "restore_point_window" => "Crear punto de restauración",
            "restore_point_creating" => {
                "Creando punto de restauración. Esto puede tardar un momento..."
            }
            "restore_point_success" => "Punto de restauración creado correctamente.",
            "restore_point_failed" => "Falló el punto de restauración",
            "restore_point_failed_run" => "No se pudo ejecutar PowerShell.",
            "restore_point_failed_windows" => {
                "Los puntos de restauración solo están soportados en Windows."
            }
            "download_title" => "Aplicaciones y descargas",
            "download_subtitle" => "Instala aplicaciones útiles desde listas seleccionadas.",
            "download_search" => "Buscar aplicaciones...",
            "download_website" => "Sitio web",
            "download_installed" => "Instalado",
            "download_not_installed" => "No instalado",
            "download_nothing_selected" => "Nada seleccionado",
            "download_installing" => "Instalando las descargas seleccionadas...",
            "download_install_job_failed" => "Falló la tarea de instalación",
            "download_meta" => "IDs de Winget: {}\nCategoría: {}\nSitio web: {}",
            "download_confirm_install" => "Confirmar instalación",
            "download_cancel" => "Cancelar",
            "download_confirm_title" => "Confirmar instalación de la app",
            "download_confirm_desc" => "Estas aplicaciones y descargas se instalarán:",
            "download_result" => "Instalado: {}  Fallido: {}",
            "download_refresh" => "Actualizar",
            "download_install_selected" => "Instalar seleccionadas",
            "download_loading" => "Cargando aplicaciones y descargas...",
            "download_none" => "No hay aplicaciones ni descargas para mostrar.",
            "download_category_0" => "Navegadores",
            "download_category_1" => "Visores de documentos",
            "download_category_2" => "Mensajería, correo y calendario",
            "download_category_3" => "Almacenamiento en línea y copia de seguridad",
            "download_category_4" => "Multimedia",
            "download_category_5" => "Imágenes",
            "download_category_6" => "Utilidades de personalización",
            "download_category_7" => "Juegos",
            "download_category_8" => "Compresión",
            "download_category_9" => "Gestión de archivos y discos",
            "download_category_10" => "Acceso remoto",
            "download_category_11" => "Herramientas para discos ópticos",
            "download_category_12" => "Otras utilidades",
            "download_category_13" => "Privacidad y seguridad",
            "download_category_14" => "Aplicaciones de desarrollo",
            "download_category_15" => "Runtimes y dependencias",
            "Microsoft EdgeWebView" => "Runtime WebView2 para aplicaciones de Windows",
            "Thorium" => "Navegador basado en Chromium con funciones de privacidad mejoradas",
            "Mercury" => "Fork de Firefox optimizado por compilador y centrado en la privacidad",
            "Mozilla Firefox" => {
                "Navegador web popular conocido por su privacidad y personalización"
            }
            "Google Chrome" => {
                "Navegador web de Google con sincronización y soporte de extensiones"
            }
            "ungoogled-chromium" => "Navegador basado en Chromium con mejoras de privacidad",
            "Brave" => "Navegador centrado en la privacidad con bloqueo de anuncios integrado",
            "Opera" => {
                "Navegador web con muchas funciones, VPN y bloqueador de anuncios integrados"
            }
            "Opera GX" => "Versión de Opera orientada a videojuegos con funciones únicas",
            "Arc Browser" => {
                "Navegador innovador centrado en el diseño y la experiencia de usuario"
            }
            "Tor Browser" => {
                "Navegador centrado en la privacidad que enruta el tráfico a través de la red Tor"
            }
            "Vivaldi" => "Navegador altamente personalizable con enfoque en el control del usuario",
            "Waterfox" => {
                "Navegador basado en Firefox centrado en la privacidad y la personalización"
            }
            "Zen Browser" => {
                "Navegador centrado en la privacidad con bloqueo de anuncios integrado"
            }
            "Mullvad Browser" => {
                "Navegador centrado en la privacidad diseñado para minimizar el rastreo y la huella digital"
            }
            "Pale Moon Browser" => {
                "Navegador de código abierto basado en Goanna centrado en la eficiencia y la personalización"
            }
            "Maxthon" => {
                "Navegador centrado en la privacidad con bloqueo de anuncios y VPN integrados"
            }
            "Ablaze Floorp" => {
                "Navegador centrado en la privacidad con una fuerte protección contra el rastreo"
            }
            "DuckDuckGo" => {
                "Motor de búsqueda centrado en la privacidad con extensión para navegador"
            }
            "LibreOffice" => "Suite ofimática gratuita y de código abierto",
            "ONLYOFFICE Desktop Editors" => {
                "Alternativa gratuita y 100 % de código abierto a Microsoft Office"
            }
            "PDFgear" => {
                "Lee, edita, convierte, fusiona y firma archivos PDF en varios dispositivos, completamente gratis y sin registrarte."
            }
            "Foxit PDF Reader" => "Lector de PDF ligero con funciones avanzadas",
            "SumatraPDF" => {
                "Visor para Windows de PDF, eBooks (epub, mobi), cómics (cbz/cbr), DjVu, XPS, CHM e imágenes"
            }
            "OpenOffice" => {
                "Suite ofimática de código abierto descontinuada. Su sucesor activo es LibreOffice"
            }
            "Adobe Acrobat Reader DC" => "Lector y editor de PDF",
            "Evernote" => "Aplicación para tomar notas",
            "CherryTree" => {
                "Aplicación jerárquica para tomar notas con texto enriquecido y resaltado de sintaxis"
            }
            "Okular" => "Visor universal de documentos con soporte para PDF, eBooks y más",
            "PDF24 Creator" => "Creador y convertidor de PDF gratuito",
            "Telegram Desktop" => "Aplicación de mensajería instantánea y llamadas de voz",
            "WhatsApp" => "Aplicación de mensajería instantánea y llamadas de voz",
            "Zoom Workplace" => "Plataforma de videoconferencias y mensajería",
            "Discord" => "Servicio de comunicación por voz, vídeo y texto",
            "Pidgin" => "Cliente de mensajería instantánea multiprotocolo",
            "Mozilla Thunderbird" => "Aplicación de correo gratuita",
            "eM Client" => "Cliente de correo con calendario, tareas y chat",
            "Proton Mail" => "Servicio de correo seguro con cifrado de extremo a extremo",
            "Trillian" => "Aplicación de mensajería instantánea",
            "Google Drive" => "Servicio de almacenamiento en la nube y sincronización de archivos",
            "Dropbox" => {
                "Servicio de alojamiento de archivos que ofrece almacenamiento en la nube, sincronización de archivos y nube personal"
            }
            "SugarSync" => {
                "Accede y comparte automáticamente tus fotos, vídeos y archivos en cualquier carpeta"
            }
            "Nextcloud" => {
                "Accede, comparte y protege tus archivos, calendarios, contactos, comunicación y más en casa y en tu organización"
            }
            "Proton Drive" => "Almacenamiento seguro en la nube con cifrado de extremo a extremo",
            "FreeFileSync" => {
                "Herramienta de comparación y sincronización de carpetas de código abierto"
            }
            "Hekasoft Backup & Restore" => {
                "La solución gratuita completa para la copia de seguridad y gestión de navegadores"
            }
            "VLC media player" => "Reproductor multimedia y framework de código abierto",
            "iTunes" => "Reproductor multimedia y biblioteca",
            "AIMP" => "Reproductor de audio con soporte para varios formatos",
            "foobar2000" => "Reproductor de audio avanzado para Windows",
            "MusicBee" => "Gestor y reproductor de música",
            "Audacity" => "Editor y grabador de audio",
            "GOM Player" => "Reproductor multimedia para Windows",
            "Spotify" => "Servicio de streaming de música",
            "MediaMonkey" => "Gestor y reproductor multimedia",
            "HandBrake" => "Transcodificador de vídeo de código abierto",
            "OBS Studio" => {
                "Software gratuito y de código abierto para grabación de vídeo y transmisión en directo"
            }
            "Streamlabs OBS" => {
                "Software de streaming basado en OBS con funciones adicionales para streamers"
            }
            "MPC-BE" => "Media Player Classic - Edición Black",
            "K-Lite Codec Pack (Mega)" => "Colección de códecs y herramientas relacionadas",
            "CapCut" => "Editor de vídeo",
            "PotPlayer64" => "Reproductor multimedia completo para Windows",
            "kdenlive" => "Software de edición de vídeo gratuito y de código abierto",
            "MediaInfo" => "Herramienta para mostrar información técnica de archivos multimedia",
            "fre:ac - free audio converter" => "Convertidor de audio gratuito y extractor de CD",
            "SMPlayer" => {
                "Reproductor multimedia con códecs integrados que puede reproducir prácticamente todos los formatos de vídeo y audio"
            }
            "Shotcut" => "Editor de vídeo multiplataforma, gratuito y de código abierto",
            "LosslessCut" => {
                "Interfaz FFmpeg multiplataforma para recortar vídeo/audio rápido y sin pérdida"
            }
            "FxSound" => "Mejorador de audio para aumentar la calidad del sonido en Windows",
            "IrfanView64" => "Visor y convertidor de imágenes rápido y compacto",
            "Krita" => "Software de pintura digital e ilustración",
            "Blender" => "Suite de creación 3D",
            "Paint.NET" => "Software de edición de imágenes y fotos",
            "GIMP" => "Programa de manipulación de imágenes GNU",
            "XnViewMP" => "Visor, navegador y convertidor de imágenes",
            "XnView" => "Visor, navegador y convertidor de imágenes (versión clásica)",
            "Inkscape" => "Editor de gráficos vectoriales",
            "Greenshot" => "Herramienta de capturas de pantalla con funciones de anotación",
            "ShareX" => {
                "Herramienta de captura de pantalla, intercambio de archivos y productividad"
            }
            "Flameshot" => "Software de capturas de pantalla potente y fácil de usar",
            "FastStone Image Viewer" => "Navegador, convertidor y editor de imágenes",
            "Nilesoft Shell" => "Herramienta de personalización del menú contextual de Windows",
            "StartAllBack (Win 11)" => {
                "Personalización del menú Inicio y la barra de tareas de Windows 11"
            }
            "StartIsBack++ (Win 10)" => {
                "Personalización del menú Inicio y la barra de tareas de Windows 10"
            }
            "Open-Shell" => "Menú Inicio de estilo clásico para Windows",
            "Windhawk" => "Plataforma de personalización para Windows",
            "Lively Wallpaper" => {
                "Aplicación de fondos de pantalla animados gratuita y de código abierto"
            }
            "Sucrose Wallpaper Engine" => {
                "Aplicación de fondos de pantalla animados gratuita y de código abierto"
            }
            "Rainmeter" => "Herramienta de personalización del escritorio para Windows",
            "ExplorerPatcher" => "Utilidad que mejora la experiencia del Explorador de Windows",
            "John's Background Switcher" => {
                "Cambia automáticamente el fondo de pantalla a intervalos regulares"
            }
            "Microsoft PowerToys" => {
                "Conjunto de utilidades para usuarios avanzados que permite ajustar y agilizar la experiencia de Windows"
            }
            "Nexus" => "El sistema de acoplamiento avanzado para Windows",
            "AutoHotkey v2" => {
                "Lenguaje de scripting gratuito para crear macros y automatizar tareas (v2, actual)"
            }
            "Steam" => "Plataforma de distribución digital para juegos de PC",
            "Epic Games Launcher" => "Plataforma de distribución digital para juegos de PC",
            "7-Zip" => "Archivador de archivos de código abierto con una alta tasa de compresión",
            "WinRAR archiver" => "Archivador de archivos con una alta tasa de compresión",
            "PeaZip" => {
                "Utilidad gratuita para archivar archivos. Abre y extrae archivos RAR, TAR, ZIP y más"
            }
            "WinDirStat" => "Visor de estadísticas de uso del disco y herramienta de limpieza",
            "WizTree" => "Analizador de espacio en disco con escaneo extremadamente rápido",
            "TreeSize Free" => "Gestor de espacio en disco",
            "Everything" => "Localiza archivos y carpetas por nombre al instante",
            "TeraCopy" => "Copia archivos más rápido y de forma más segura",
            "File Converter" => "Convertidor de archivos por lotes para Windows",
            "Crystal Disk Info" => "Utilidad de supervisión del estado de discos duros",
            "Bulk Rename Utility" => "Software para renombrar archivos en Windows",
            "IObit Unlocker" => {
                "Herramienta para desbloquear archivos que están siendo usados por otros procesos"
            }
            "HiBit Uninstaller" => {
                "Desinstala por completo software difícil de quitar, aplicaciones de Windows y extensiones del navegador"
            }
            "SanDisk Dashboard" => {
                "Herramienta de gestión de unidades para SSD y memorias USB SanDisk"
            }
            "Rufus" => "Utilidad para crear memorias USB de arranque",
            "Advanced Renamer" => "Utilidad avanzada para renombrar archivos por lotes",
            "RustDesk" => "Software de acceso remoto y soporte rápido de código abierto",
            "AnyDesk" => "Software de escritorio remoto para acceso y soporte",
            "TeamViewer" => {
                "Control remoto, compartición de escritorio, reuniones en línea, videoconferencias y transferencia de archivos"
            }
            "UltraViewer" => {
                "Te ayuda a controlar el ordenador de tu pareja para ayudarle como si estuvieras sentado frente a su pantalla"
            }
            "RealVNC Server" => "Software de acceso remoto",
            "RealVNC Viewer" => "Software de acceso remoto",
            "Chrome Remote Desktop" => "Acceso remoto a tu ordenador a través del navegador Chrome",
            "Parsec" => {
                "El escritorio remoto reinventado. Acceso seguro, flexible y sin esfuerzo a todo lo que haces, en cualquier momento y desde cualquier lugar"
            }
            "Parsec Virtual Display Driver" => {
                "Controlador de pantalla virtual para Parsec Remote Desktop"
            }
            "Parsec Virtual USB Driver" => "Controlador USB virtual para Parsec Remote Desktop",
            "InputLeap" => {
                "Software KVM de código abierto para compartir ratón y teclado entre ordenadores"
            }
            "ImgBurn" => "Aplicación ligera para grabar CD / DVD / HD DVD / Blu-ray",
            "AnyBurn" => "Software ligero para grabar CD/DVD/Blu-ray",
            "CDBurnerXP" => "Software gratuito para grabar CD/DVD/Blu-ray",
            "CCleaner" => "Herramienta de optimización y limpieza del sistema",
            "Snappy Driver Installer Origin" => "Instalador y actualizador de controladores",
            "Wise Disk Cleaner" => "Herramienta gratuita de limpieza y desfragmentación de disco",
            "Wise Registry Cleaner" => "Herramienta de limpieza y optimización del registro",
            "UniGetUI" => {
                "Interfaz universal de gestor de paquetes compatible con WinGet, Chocolatey y más"
            }
            "OpenRGB" => "Software de código abierto para controlar la iluminación RGB",
            "OpenAudible" => "Gestor y convertidor de audiolibros para archivos Audible",
            "NAPS2" => "Aplicación de escaneo de documentos con soporte OCR",
            "IObit Uninstaller" => {
                "Desinstala por completo software no deseado, aplicaciones de Windows y complementos del navegador"
            }
            "Revo Uninstaller" => {
                "Revo Uninstaller te ayuda a desinstalar software y eliminar programas no deseados fácilmente."
            }
            "Malwarebytes" => "Software antimalware para Windows",
            "Malwarebytes AdwCleaner" => "Herramienta de eliminación de adware para Windows",
            "Windows Firewall Control" => "Aplicación Malwarebytes Windows Firewall Control",
            "OnionShare" => {
                "Comparte archivos, aloja sitios web y chatea de forma segura y anónima a través de la red Tor"
            }
            "Sniffnet" => "Herramienta de supervisión de red para analizar tu tráfico de Internet",
            "TeleGuard" => "Aplicación de mensajería segura con cifrado de extremo a extremo",
            "Python 3.13" => "Lenguaje de programación Python",
            "Notepad++" => "Editor de código fuente gratuito y sustituto de Bloc de notas",
            "WinSCP" => "Cliente gratuito de SFTP, SCP, Amazon S3, WebDAV y FTP",
            "PuTTY" => "Cliente gratuito de SSH y telnet",
            "WinMerge" => "Herramienta de comparación y fusión de código abierto",
            "Eclipse IDE for Java" => "IDE de Java y plataforma de desarrollo",
            "Microsoft Visual Studio Code" => {
                "Editor de código con soporte para operaciones de desarrollo"
            }
            "Git" => "Sistema de control de versiones distribuido",
            "GitHub Desktop" => "Cliente de escritorio de GitHub",
            "Microsoft .NET Runtime 3.1" => "Runtime .NET 3.1 para ejecutar aplicaciones",
            "Microsoft .NET Runtime 5.0" => "Runtime .NET 5.0 para ejecutar aplicaciones",
            "Microsoft .NET Runtime 6.0" => "Runtime .NET 6.0 LTS para ejecutar aplicaciones",
            "Microsoft .NET Runtime 7.0" => "Runtime .NET 7.0 para ejecutar aplicaciones",
            "Microsoft .NET Runtime 8.0" => "Runtime .NET 8.0 LTS para ejecutar aplicaciones",
            ".NET Framework 4.8.1" => "Paquete de desarrollo de .NET Framework",
            "DirectX Runtime" => {
                "Componentes de runtime de DirectX para ejecutar juegos y aplicaciones multimedia"
            }
            "Java Runtime Environment" => {
                "Entorno de ejecución de Java para ejecutar aplicaciones Java"
            }
            "Visual C++ 2005 (x86)" => "Componentes de runtime de Visual C++ 2005",
            "Visual C++ 2005 (x64)" => "Componentes de runtime de Visual C++ 2005",
            "Visual C++ 2008 (x86)" => "Componentes de runtime de Visual C++ 2008",
            "Visual C++ 2008 (x64)" => "Componentes de runtime de Visual C++ 2008",
            "Visual C++ 2010 (x86)" => "Componentes de runtime de Visual C++ 2010",
            "Visual C++ 2010 (x64)" => "Componentes de runtime de Visual C++ 2010",
            "Visual C++ 2012 (x86)" => "Componentes de runtime de Visual C++ 2012",
            "Visual C++ 2012 (x64)" => "Componentes de runtime de Visual C++ 2012",
            "Visual C++ 2013 (x86)" => "Componentes de runtime de Visual C++ 2013",
            "Visual C++ 2013 (x64)" => "Componentes de runtime de Visual C++ 2013",
            "Visual C++ 2015-2022 (x86)" => "Componentes de runtime de Visual C++ 2015-2022",
            "Visual C++ 2015-2022 (x64)" => "Componentes de runtime de Visual C++ 2015-2022",
            "Tabby" => "Terminal SSH y gestor de conexiones",
            "Riot Games Launcher" => {
                "Lanzador del cliente de Riot (puede requerir inicio de sesión manual)"
            }
            "Helium Browser" => "Navegador Chromium centrado en la privacidad",
            "DataGrip" => "IDE SQL de JetBrains",
            "Zed" => "Editor de código de alto rendimiento",
            "debloater_title" => "Desinstalador",
            "debloater_subtitle" => "Elimina componentes de Windows y funciones opcionales.",
            "debloater_search" => "Buscar paquetes",
            "debloater_nothing_selected" => "Nada seleccionado",
            "debloater_installed_status" => "Instalado",
            "debloater_not_installed_status" => "No instalado",
            "debloater_installing" => "Instalando los elementos seleccionados...",
            "debloater_removing" => "Eliminando los elementos seleccionados...",
            "debloater_all_items" => "Todos los elementos",
            "debloater_installed_only" => "Solo instalados",
            "debloater_not_installed_only" => "Solo no instalados",
            "debloater_refresh" => "Actualizar",
            "debloater_install_selected" => "Instalar seleccionados",
            "debloater_remove_selected" => "Eliminar seleccionados",
            "debloater_loading" => "Cargando paquetes...",
            "debloater_scanning" => {
                "Analizando aplicaciones instaladas, capacidades y funciones opcionales."
            }
            "debloater_none" => "No hay paquetes para mostrar.",
            "debloater_cannot_reinstall" => "No se puede reinstalar",
            "debloater_meta" => "Paquete: {}\nCategoría: {}\nGrupo: {}",
            "debloater_confirm_install_title" => "Confirmar instalación",
            "debloater_confirm_remove_title" => "Confirmar eliminación",
            "debloater_confirm_install_desc" => "Estas aplicaciones se instalarán:",
            "debloater_confirm_remove_desc" => "Estas aplicaciones se eliminarán:",
            "debloater_confirm_install_btn" => "Confirmar instalación",
            "debloater_confirm_remove_btn" => "Confirmar eliminación",
            "debloater_cancel" => "Cancelar",
            "debloater_installed" => "Instalado",
            "debloater_not_installed" => "No instalado",
            "debloater_result_install" => "Instalados: {}  Fallidos: {}",
            "debloater_result_remove" => "Eliminados: {}  Fallidos: {}",
            "debloater_tab_0" => "Aplicaciones de Windows",
            "debloater_tab_1" => "Capacidades",
            "debloater_tab_2" => "Funciones opcionales",
            "processes_title" => "Procesos",
            "processes_subtitle" => "Inspecciona procesos, afinidad y ajustes de prioridad.",
            "processes_refresh" => "Actualizar",
            "processes_active_only" => "Solo activos",
            "processes_visible" => "Visibles:",
            "processes_total_cpu" => "CPU total:",
            "processes_refreshing" => "Actualizando la lista de procesos...",
            "processes_waiting_first" => "Esperando la primera actualización...",
            "processes_reload_queued" => "Recarga en cola",
            "processes_pid" => "PID",
            "processes_name" => "Nombre",
            "processes_cpu" => "CPU %",
            "processes_priority" => "Prioridad",
            "processes_affinity" => "Afinidad",
            "processes_status" => "Estado",
            "processes_collapse_tree" => "Contraer árbol",
            "processes_expand_tree" => "Expandir árbol",
            "processes_cpu_priority" => "Prioridad de CPU",
            "processes_current" => "Actual",
            "processes_always" => "Siempre",
            "processes_io_priority" => "Prioridad de E/S",
            "processes_affinity_menu" => "Afinidad",
            "processes_open_editor" => "Abrir editor",
            "processes_all_cores" => "Todos los núcleos",
            "processes_affinity_prefix" => "CPU",
            "processes_selected" => "Proceso seleccionado",
            "processes_realtime_title" => "¿Establecer prioridad en tiempo real?",
            "processes_realtime_warn" => {
                "El tiempo real puede congelar la respuesta de Windows. Continúa solo si entiendes el riesgo."
            }
            "processes_cancel" => "Cancelar",
            "processes_confirm" => "Confirmar",
            "processes_affinity_title" => "Afinidad de CPU - {} (PID {})",
            "processes_affinity_mask" => "Máscara de afinidad (hex): {}",
            "processes_invert" => "Invertir",
            "processes_clear" => "Borrar",
            "processes_close" => "Cerrar",
            "processes_apply" => "Aplicar",
            "processes_last_refresh" => "Última actualización: hace {} s",
            "processes_priority_unknown" => "Desconocida",
            "processes_priority_idle" => "Inactiva",
            "processes_priority_below_normal" => "Inferior a normal",
            "processes_priority_normal" => "Normal",
            "processes_priority_above_normal" => "Superior a normal",
            "processes_priority_high" => "Alta",
            "processes_priority_realtime" => "Tiempo real",
            "processes_priority_background" => "Segundo plano: 4 (I/O y CPU bajas)",
            "processes_priority_low" => "Baja",
            "processes_priority_always_below" => "Por debajo",
            "processes_priority_always_above" => "Por encima",
            "processes_status_running" => "En ejecución",
            "processes_status_sleeping" => "En espera",
            "processes_status_idle" => "Inactiva",
            "processes_status_zombie" => "Zombi",
            "processes_status_stopped" => "Detenida",
            "processes_status_tracing" => "Rastreando",
            "processes_status_dead" => "Muerta",
            "processes_status_wakekill" => "Wakekill",
            "processes_status_waking" => "Despertando",
            "processes_status_lockblocked" => "Bloqueo",
            "processes_status_parked" => "Estacionado",
            "processes_status_unknown" => "Desconocido",
            "processes_process_scan_failed" => "Falló el análisis de procesos",
            "processes_openprocess_failed" => "OpenProcess falló para el PID {}: {}",
            "processes_get_affinity_failed" => "GetProcessAffinityMask falló para el PID {}: {}",
            "processes_invalid_system_mask" => {
                "Máscara de afinidad del sistema inválida para el PID {}"
            }
            "processes_invalid_affinity_mode" => "Modo de afinidad inválido",
            "processes_set_priority_failed" => "SetPriorityClass falló para el PID {}",
            "processes_set_io_failed" => "SetProcessInformation(I/O) falló para el PID {}",
            "processes_set_affinity_failed" => "SetProcessAffinityMask falló para el PID {}",
            "processes_perfoptions_open" => "No se pudo crear/abrir la clave PerfOptions",
            "processes_perfoptions_write_cpu" => "No se pudo escribir CpuPriorityClass",
            "processes_perfoptions_write_io" => "No se pudo escribir IoPriority",
            "processes_invalid_priority_level" => "Nivel de prioridad inválido",
            "latency_title" => "Latencia",
            "latency_subtitle" => "Analiza la latencia USB y la topología de dispositivos.",
            "latency_button" => "Analizar latencia USB",
            "latency_analyzing" => "Analizando...",
            "latency_starting" => "Iniciando análisis...",
            "latency_topology" => "Analizando la topología USB...",
            "latency_scanning" => {
                "Analizando dispositivos PnP, cadena de controladores, MSI y ajustes de energía."
            }
            "latency_begin" => "Haz clic en 'Analizar latencia USB' para comenzar.",
            "latency_loading_fail" => "Falló el análisis de latencia USB",
            "latency_admin" => "Asegúrate de ejecutar como Administrador.",
            "latency_error_title" => "ERROR - FALLÓ EL ANÁLISIS DE LATENCIA USB",
            "latency_failed" => "Falló el análisis de latencia USB",
            "latency_progress_power" => "Comprobando ajustes de energía...",
            "latency_progress_controllers" => "Analizando controladores USB...",
            "latency_progress_usb_registry_tree" => "Leyendo el árbol del registro USB...",
            "latency_progress_inputs" => "Buscando dispositivos de entrada...",
            "latency_progress_hubs" => "Siguiendo dispositivos hasta los concentradores raíz...",
            "latency_progress_verify" => "Verificando la topología y las sugerencias de energía...",
            "latency_progress_report" => "Generando informe...",
            //_ => "",
            _ => t(Language::English, key),
        },
        Language::Turkish => match key {
            "nav_title" => "Gezinme",
            "nav_subtitle" => "Winchisel kontrol merkezi",
            "home" => "Ana Sayfa",
            "debloater" => "Debloater",
            "downloads" => "Uygulamalar ve İndirmeler",
            "performance" => "Performans",
            "processes" => "İşlemler",
            "latency" => "Gecikme",
            "settings" => "Ayarlar",
            "check_updates" => "Güncellemeleri kontrol et",
            "donate" => "Bağış yap",
            "bug_report" => "Hata bildir",
            "status_admin" => "Yönetici",
            "status_standard" => "Standart",
            "settings_title" => "Ayarlar",
            "settings_subtitle" => "Uygulamanın davranışını iş akışınıza göre ayarlayın.",
            "settings_application" => "Uygulama",
            "settings_saved_auto" => "Bu ayarlar otomatik olarak kaydedilir.",
            "language" => "Dil",
            "sidebar_languages" => "Diller",
            "archive-restore" => "Geri yükle",
            "settings-2" => "Ayarlar",
            "shield-check" => "Güvenlik durumu",
            "refresh-cw" => "Yenile",
            "monitor" => "Konsol",
            "Winchisel-Updater" => "Winchisel Güncelleyici",
            "Disabled (Recommended)" => "Devre dışı (önerilen)",
            "0" => "0",
            "1" => "1",
            "app_name_m365_copilot_office_hub" => "MS 365 Copilot (Office Hub)",
            "app_name_remote_assistance" => "Uzaktan yardım uygulaması",
            "app_name_classic_paint" => "Klasik Paint uygulaması",
            "windows-app-office-hub" => "Microsoft 365 Copilot (eski adıyla Office hub)",
            "windows-app-remote-assistance" => "Uzaktan yardım uygulaması",
            "capability-paint-legacy" => "Klasik Paint uygulaması",
            "open_logs" => "Kayıtları aç",
            "check_updates_startup" => "Başlangıçta güncellemeleri kontrol et",
            "show_console" => "Konsolu göster",
            "system_protection" => "Sistem koruması",
            "restore_point" => "Geri yükleme noktası",
            "restore_point_desc" => {
                "Önemli sistem değişikliklerinden önce bir geri yükleme noktası oluşturur"
            }
            "create_restore_point" => "Geri yükleme noktası oluştur",
            _ => t(Language::English, key),
        },
        Language::Greek => match key {
            "nav_title" => "Πλοήγηση",
            "nav_subtitle" => "Κέντρο ελέγχου Winchisel",
            "home" => "Αρχική",
            "debloater" => "Debloater",
            "downloads" => "Εφαρμογές & Λήψεις",
            "performance" => "Επιδόσεις",
            "processes" => "Διαδικασίες",
            "latency" => "Καθυστέρηση",
            "settings" => "Ρυθμίσεις",
            "check_updates" => "Έλεγχος ενημερώσεων",
            "donate" => "Δωρεά",
            "bug_report" => "Αναφορά σφάλματος",
            "status_admin" => "Διαχειριστής",
            "status_standard" => "Τυπικό",
            "settings_title" => "Ρυθμίσεις",
            "settings_subtitle" => "Προσαρμόστε τη συμπεριφορά της εφαρμογής στη ροή εργασίας σας.",
            "settings_application" => "Εφαρμογή",
            "settings_saved_auto" => "Αυτές οι ρυθμίσεις αποθηκεύονται αυτόματα.",
            "language" => "Γλώσσα",
            "sidebar_languages" => "Γλώσσες",
            "check_updates_startup" => "Έλεγχος ενημερώσεων κατά την εκκίνηση",
            "show_console" => "Εμφάνιση κονσόλας",
            "restore_point" => "Σημείο επαναφοράς",
            "restore_point_desc" => {
                "Δημιουργεί σημείο επαναφοράς πριν από σημαντικές αλλαγές συστήματος"
            }
            "create_restore_point" => "Δημιουργία σημείου επαναφοράς",
            "performance_title" => "Επιδόσεις",
            "performance_subtitle" => "Βελτιστοποιήσεις για παιχνίδια και επιδόσεις",
            "performance_search" => "Αναζήτηση ρυθμίσεων επιδόσεων...",
            "performance_quick" => "Γρήγορες ενέργειες",
            "performance_apply_recommended" => "Εφαρμογή προτεινόμενων ρυθμίσεων",
            "performance_reset_defaults" => "Επαναφορά στις προεπιλογές των Windows",
            "performance_loading" => "Φόρτωση ρυθμίσεων επιδόσεων...",
            "performance_empty" => "Δεν υπάρχουν διαθέσιμες ρυθμίσεις επιδόσεων.",
            "processes_title" => "Διαδικασίες",
            "processes_subtitle" => "Επιθεώρηση διαδικασιών, συσχέτισης και προτεραιότητας.",
            "processes_refresh" => "Ανανέωση",
            "processes_active_only" => "Μόνο ενεργές",
            "processes_visible" => "Ορατές:",
            "processes_total_cpu" => "Συνολικό CPU:",
            "processes_refreshing" => "Ανανέωση λίστας διαδικασιών...",
            "processes_waiting_first" => "Αναμονή για την πρώτη ανανέωση...",
            "processes_reload_queued" => "Η επαναφόρτωση βρίσκεται σε ουρά",
            "processes_pid" => "PID",
            "processes_name" => "Όνομα",
            "processes_cpu" => "CPU %",
            "processes_priority" => "Προτεραιότητα",
            "processes_affinity" => "Συσχέτιση",
            "processes_status" => "Κατάσταση",
            "processes_collapse_tree" => "Σύμπτυξη δέντρου",
            "processes_expand_tree" => "Ανάπτυξη δέντρου",
            "processes_selected" => "Επιλεγμένη διεργασία",
            "update_available_prefix" => "Διαθέσιμη ενημέρωση:",
            "update_failed_prefix" => "Ο έλεγχος ενημέρωσης απέτυχε:",
            "update_newer_version_prefix" => "Υπάρχει νεότερη έκδοση:",
            "update_ready" => "Έτοιμο",
            "update_checked" => "Χρησιμοποιείτε ήδη την πιο πρόσφατη έκδοση.",
            "settings_saved" => "Οι ρυθμίσεις αποθηκεύτηκαν",
            "update_no_found_title" => "Δεν βρέθηκε ενημέρωση",
            "update_available_title" => "Διαθέσιμη ενημέρωση",
            "update_failed_title" => "Ο έλεγχος ενημέρωσης απέτυχε",
            "update_up_to_date" => "Χρησιμοποιείτε ήδη την πιο πρόσφατη έκδοση.",
            "update_error_check_updates" => "Ο έλεγχος ενημέρωσης απέτυχε",
            "update_error_read_response" => "Η απάντηση δεν μπόρεσε να διαβαστεί",
            "update_error_parse_json" => "Αποτυχία ανάλυσης JSON",
            "update_error_no_tag_name" => "Δεν ελήφθη tag_name",
            "update_error_download" => "Η ενημέρωση δεν μπόρεσε να ληφθεί",
            "update_error_create_temp_file" => "Δεν μπόρεσε να δημιουργηθεί προσωρινό αρχείο",
            "update_error_write_update_file" => "Δεν μπόρεσε να γραφτεί το αρχείο ενημέρωσης",
            "update_error_download_too_small" => "Η λήψη της ενημέρωσης είναι πολύ μικρή",
            "update_error_resolve_current_exe" => "Δεν μπόρεσε να επιλυθεί το τρέχον εκτελέσιμο",
            "update_error_write_update_script" => "Δεν μπόρεσε να γραφτεί το σενάριο ενημέρωσης",
            "update_error_launch_updater" => "Δεν μπόρεσε να εκκινηθεί ο ενημερωτής",
            _ => t(Language::English, key),
        },
        Language::Dutch => match key {
            "nav_title" => "Navigatie",
            "nav_subtitle" => "Winchisel-bedieningscentrum",
            "home" => "Start",
            "debloater" => "Debloater",
            "downloads" => "Apps & downloads",
            "performance" => "Prestaties",
            "processes" => "Processen",
            "latency" => "Latentie",
            "settings" => "Instellingen",
            "check_updates" => "Controleren op updates",
            "donate" => "Doneren",
            "bug_report" => "Bug melden",
            "status_admin" => "Beheerder",
            "status_standard" => "Standaard",
            "settings_title" => "Instellingen",
            "settings_subtitle" => "Pas het gedrag van de app aan op je workflow.",
            "settings_application" => "Applicatie",
            "settings_saved_auto" => "Deze instellingen worden automatisch opgeslagen.",
            "language" => "Taal",
            "sidebar_languages" => "Talen",
            "check_updates_startup" => "Bij opstarten op updates controleren",
            "show_console" => "Console weergeven",
            "restore_point" => "Herstelpunt",
            "restore_point_desc" => "Maakt een herstelpunt vóór belangrijke systeemwijzigingen",
            "create_restore_point" => "Herstelpunt maken",
            "performance_title" => "Prestaties",
            "performance_subtitle" => "Optimalisaties voor gaming en prestaties",
            "performance_search" => "Zoek prestatie-instellingen...",
            "performance_quick" => "Snelle acties",
            "performance_apply_recommended" => "Aanbevolen instellingen toepassen",
            "performance_reset_defaults" => "Windows-standaard herstellen",
            "performance_loading" => "Prestatie-instellingen laden...",
            "performance_empty" => "Geen prestatie-instellingen beschikbaar.",
            "processes_title" => "Processen",
            "processes_subtitle" => "Processen, affiniteit en prioriteit controleren.",
            "processes_refresh" => "Vernieuwen",
            "processes_active_only" => "Alleen actief",
            "processes_visible" => "Zichtbaar:",
            "processes_total_cpu" => "Totale CPU:",
            "processes_refreshing" => "Proceslijst wordt vernieuwd...",
            "processes_waiting_first" => "Wachten op eerste verversing...",
            "processes_reload_queued" => "Opnieuw laden in wachtrij",
            "processes_pid" => "PID",
            "processes_name" => "Naam",
            "processes_cpu" => "CPU %",
            "processes_priority" => "Prioriteit",
            "processes_affinity" => "Affiniteit",
            "processes_status" => "Status",
            "processes_collapse_tree" => "Boom samenvouwen",
            "processes_expand_tree" => "Boom uitvouwen",
            "processes_selected" => "Geselecteerd proces",
            "update_available_prefix" => "Update beschikbaar:",
            "update_failed_prefix" => "Updatecontrole mislukt:",
            "update_newer_version_prefix" => "Er is een nieuwere versie beschikbaar:",
            "update_ready" => "Klaar",
            "update_checked" => "Je gebruikt al de nieuwste versie.",
            "settings_saved" => "Instellingen opgeslagen",
            "update_no_found_title" => "Geen update gevonden",
            "update_available_title" => "Update beschikbaar",
            "update_failed_title" => "Updatecontrole mislukt",
            "update_up_to_date" => "Je gebruikt al de nieuwste versie.",
            "update_error_check_updates" => "Updatecontrole mislukt",
            "update_error_read_response" => "Antwoord kon niet worden gelezen",
            "update_error_parse_json" => "JSON kon niet worden geparseerd",
            "update_error_no_tag_name" => "Geen tag_name ontvangen",
            "update_error_download" => "Update kon niet worden gedownload",
            "update_error_create_temp_file" => "Tijdelijk bestand kon niet worden aangemaakt",
            "update_error_write_update_file" => "Updatebestand kon niet worden geschreven",
            "update_error_download_too_small" => "De update-download is te klein",
            "update_error_resolve_current_exe" => {
                "Huidige uitvoerbare bestand kon niet worden opgelost"
            }
            "update_error_write_update_script" => "Updatescript kon niet worden geschreven",
            "update_error_launch_updater" => "Updater kon niet worden gestart",
            _ => t(Language::English, key),
        },
        Language::Portuguese => match key {
            "nav_title" => "Navegação",
            "nav_subtitle" => "Central de controle do Winchisel",
            "home" => "Início",
            "downloads" => "Apps e Downloads",
            "performance" => "Desempenho",
            "processes" => "Processos",
            "settings" => "Configurações",
            "check_updates" => "Verificar atualizações",
            "donate" => "Doar",
            "bug_report" => "Relatar erro",
            "language" => "Idioma",
            "processes_refresh" => "Atualizar",
            "processes_active_only" => "Somente ativos",
            "update_available_prefix" => "Atualização disponível:",
            "update_checked" => "Você já está na versão mais recente.",
            "update_available_title" => "Atualização disponível",
            "update_failed_title" => "Falha na verificação de atualização",
            _ => t(Language::English, key),
        },
        Language::Italian => match key {
            "nav_title" => "Navigazione",
            "nav_subtitle" => "Centro di controllo Winchisel",
            "home" => "Home",
            "downloads" => "App e download",
            "performance" => "Prestazioni",
            "processes" => "Processi",
            "settings" => "Impostazioni",
            "check_updates" => "Controlla aggiornamenti",
            "donate" => "Dona",
            "bug_report" => "Segnala un bug",
            "language" => "Lingua",
            "processes_refresh" => "Aggiorna",
            "processes_active_only" => "Solo attivi",
            "update_available_prefix" => "Aggiornamento disponibile:",
            "update_checked" => "Hai già la versione più recente.",
            "update_available_title" => "Aggiornamento disponibile",
            "update_failed_title" => "Controllo aggiornamenti non riuscito",
            _ => t(Language::English, key),
        },
        Language::Polish => match key {
            "nav_title" => "Nawigacja",
            "nav_subtitle" => "Centrum sterowania Winchisel",
            "home" => "Start",
            "downloads" => "Aplikacje i pobieranie",
            "performance" => "Wydajność",
            "processes" => "Procesy",
            "settings" => "Ustawienia",
            "check_updates" => "Sprawdź aktualizacje",
            "donate" => "Wesprzyj",
            "bug_report" => "Zgłoś błąd",
            "language" => "Język",
            "processes_refresh" => "Odśwież",
            "processes_active_only" => "Tylko aktywne",
            "update_available_prefix" => "Dostępna aktualizacja:",
            "update_checked" => "Masz już najnowszą wersję.",
            "update_available_title" => "Dostępna aktualizacja",
            "update_failed_title" => "Nie udało się sprawdzić aktualizacji",
            _ => t(Language::English, key),
        },
        Language::Russian => match key {
            "nav_title" => "Навигация",
            "nav_subtitle" => "Панель управления Winchisel",
            "home" => "Главная",
            "downloads" => "Приложения и загрузки",
            "performance" => "Производительность",
            "processes" => "Процессы",
            "settings" => "Настройки",
            "check_updates" => "Проверить обновления",
            "donate" => "Пожертвовать",
            "bug_report" => "Сообщить об ошибке",
            "language" => "Язык",
            "processes_refresh" => "Обновить",
            "processes_active_only" => "Только активные",
            "update_available_prefix" => "Доступно обновление:",
            "update_checked" => "У вас уже установлена последняя версия.",
            "update_available_title" => "Доступно обновление",
            "update_failed_title" => "Не удалось проверить обновления",
            _ => t(Language::English, key),
        },
        Language::Japanese => match key {
            "nav_title" => "ナビゲーション",
            "nav_subtitle" => "Winchisel コントロールセンター",
            "home" => "ホーム",
            "downloads" => "アプリとダウンロード",
            "performance" => "パフォーマンス",
            "processes" => "プロセス",
            "settings" => "設定",
            "check_updates" => "更新を確認",
            "donate" => "寄付",
            "bug_report" => "バグ報告",
            "language" => "言語",
            "processes_refresh" => "更新",
            "processes_active_only" => "アクティブのみ",
            "update_available_prefix" => "更新があります:",
            "update_checked" => "すでに最新バージョンです。",
            "update_available_title" => "更新があります",
            "update_failed_title" => "更新の確認に失敗しました",
            _ => t(Language::English, key),
        },
        Language::ChineseSimplified => match key {
            "nav_title" => "导航",
            "nav_subtitle" => "Winchisel 控制中心",
            "home" => "主页",
            "downloads" => "应用和下载",
            "performance" => "性能",
            "processes" => "进程",
            "settings" => "设置",
            "check_updates" => "检查更新",
            "donate" => "捐赠",
            "bug_report" => "报告错误",
            "language" => "语言",
            "processes_refresh" => "刷新",
            "processes_active_only" => "仅活动",
            "update_available_prefix" => "有可用更新：",
            "update_checked" => "你已经是最新版本。",
            "update_available_title" => "有可用更新",
            "update_failed_title" => "检查更新失败",
            _ => t(Language::English, key),
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
