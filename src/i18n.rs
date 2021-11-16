use crate::config::Language;

// Internationalization and localization
pub struct I18n;
impl I18n {
    pub fn day_labels(lang: &Language) -> Vec<&'static str> {
        match lang {
            Language::English => vec![
                "Monday",
                "Tuesday",
                "Wednesday",
                "Thursday",
                "Friday",
                "Saturday",
                "Sunday",
            ],
            Language::Norsk => vec![
                "Mandag", "Tirsdag", "Onsdag", "Torsdag", "Fredag", "Lørdag", "Søndag",
            ],
        }
    }
    pub fn comment_label(lang: &Language) -> &'static str {
        match lang {
            Language::English => "Comment",
            Language::Norsk => "Kommentar",
        }
    }
    pub fn week_label(lang: &Language) -> &'static str {
        match lang {
            Language::English => "Week",
            Language::Norsk => "Uke",
        }
    }
    pub fn key_labels(lang: &Language) -> Vec<&'static str> {
        match lang {
            _ => vec![
                "?",
                "hjkl / ←↓↑→",
                "K / Shift + ↑",
                "J / Shift + ↓",
                "Enter",
                "N",
                "S",
                "U",
                "q / Esc",
                "Ctrl + C",
                "w",
            ],
        }
    }
    pub fn action_labels(lang: &Language) -> Vec<&'static str> {
        match lang {
            Language::English => vec![
                "Show this menu",
                "Movement",
                "Increment hours",
                "Decrement hours",
                "Write comment",
                "New timecode",
                "Star timecode",
                "Unstar timecode",
                "Quit (Saves on exit)",
                "Force quit",
                "Save",
            ],
            Language::Norsk => vec![
                "Vis denne menyen",
                "Bevegelse",
                "Inkrementer timer",
                "Dekrementer timer",
                "Skriv kommentar",
                "Ny timekode",
                "Legg til timekode som favoritt",
                "Fjern timekode som favoritt",
                "Avslutt (Lagrer ved avslutning)",
                "Tvangsavslutt",
                "Lagre",
            ],
        }
    }
    pub fn info_screen(lang: &Language) -> &'static str {
        match lang {
            Language::English => "Press ? to show controls",
            Language::Norsk => "Trykk på ? for å vise kontrollene",
        }
    }
}
