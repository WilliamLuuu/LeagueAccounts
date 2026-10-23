//! Game product locales.
//!
//! A product locale is independent of the account's region: Riot lets a TW2
//! account run the game in `zh_MY`, `en_US`, `ja_JP` and so on. The tool keeps
//! the choice per account and applies it through the Riot Client's local API
//! before the game launches, which is exactly what the client's own language
//! picker does.
//!
//! `zh_MY` (Malaysian Chinese) is the only locale Riot ships that renders
//! Simplified Chinese with Traditional Chinese fallbacks for the SEA/TW
//! clusters, so it is also the value used by "汉化" tools.

/// Locales Riot's product registry reports for `league_of_legends` on the
/// `live` patchline, in the order the registry returns them.
pub const GAME_LOCALES: &[&str] = &[
    "ar_AE", "cs_CZ", "de_DE", "el_GR", "en_AU", "en_GB", "en_PH", "en_SG", "en_US", "es_AR",
    "es_ES", "es_MX", "fr_FR", "hu_HU", "it_IT", "ja_JP", "ko_KR", "pl_PL", "pt_BR", "ro_RO",
    "ru_RU", "th_TH", "tr_TR", "vi_VN", "zh_CN", "zh_MY", "zh_TW",
];

/// Display names, so the UI does not have to ship its own table.
pub fn locale_name(locale: &str) -> &'static str {
    match locale {
        "ar_AE" => "العربية",
        "cs_CZ" => "Čeština",
        "de_DE" => "Deutsch",
        "el_GR" => "Ελληνικά",
        "en_AU" => "English (Australia)",
        "en_GB" => "English (United Kingdom)",
        "en_PH" => "English (Philippines)",
        "en_SG" => "English (Singapore)",
        "en_US" => "English (United States)",
        "es_AR" => "Español (Argentina)",
        "es_ES" => "Español (España)",
        "es_MX" => "Español (Latinoamérica)",
        "fr_FR" => "Français",
        "hu_HU" => "Magyar",
        "it_IT" => "Italiano",
        "ja_JP" => "日本語",
        "ko_KR" => "한국어",
        "pl_PL" => "Polski",
        "pt_BR" => "Português (Brasil)",
        "ro_RO" => "Română",
        "ru_RU" => "Русский",
        "th_TH" => "ไทย",
        "tr_TR" => "Türkçe",
        "vi_VN" => "Tiếng Việt",
        // Named by script first: `zh_CN` and `zh_MY` are both Simplified, so
        // "Simplified/Traditional" is what tells them apart in the picker.
        "zh_CN" => "简体中文（中国大陆）",
        "zh_MY" => "简体中文（马来西亚）",
        "zh_TW" => "繁體中文（中國台灣）",
        _ => "English (United States)",
    }
}

/// `zh_CN` is installable, but Riot never published every home-page
/// (activity-center) page for it. `rcp-be-lol-activity-center` requests
/// `.../channel/league_of_legends_client/page/<page>?locale=zh_CN`, gets a
/// 404 for pages such as `jade-home`, and the home page then fails to
/// render. `zh_TW` has every page, which is why the fallback works.
///
/// The only working fix lives inside the game client's renderer: ABC's
/// `zh-cn-home-locale` Pengu Loader plugin wraps `fetch` and
/// `XMLHttpRequest` there and retries a failed page with `locale=zh_TW`.
/// Reaching that renderer from this app would mean injecting code into
/// `LeagueClientUx` — Pengu's `core.dll` does exactly that with
/// `VirtualAllocEx` + `WriteProcessMemory` + `CreateRemoteThread`. The
/// client does not offer a supported way in: `LeagueClientUx.exe`'s own
/// switch allowlist omits `--remote-debugging-port`, so even the
/// `customLaunchArgs` channel that the Riot Client's own Play button uses
/// cannot open a debugging port on the game client. This tool therefore
/// warns and lets the user choose, instead of pretending it can patch.
pub fn needs_homepage_patch(locale: &str) -> bool {
    locale == "zh_CN"
}

/// Locales whose home page renders correctly as-is.
pub fn home_page_works(locale: &str) -> bool {
    !needs_homepage_patch(locale)
}

/// Accept only a locale the client can actually install, so a typo in a config
/// file cannot make the Riot Client reject the launch.
pub fn is_known(locale: &str) -> bool {
    GAME_LOCALES.contains(&locale)
}

/// `zh-my` / `ZH_MY` / ` zh_my ` all normalize to `zh_MY`.
pub fn normalize(value: &str) -> Option<&'static str> {
    let trimmed = value.trim().replace('-', "_");
    GAME_LOCALES
        .iter()
        .find(|locale| locale.eq_ignore_ascii_case(&trimmed))
        .copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_accepts_case_and_dashes() {
        assert_eq!(normalize("zh-my"), Some("zh_MY"));
        assert_eq!(normalize(" ZH_MY "), Some("zh_MY"));
        assert_eq!(normalize("en_us"), Some("en_US"));
        assert_eq!(normalize("xx_YY"), None);
    }

    #[test]
    fn every_locale_has_a_name() {
        for locale in GAME_LOCALES {
            assert!(!locale_name(locale).is_empty(), "{locale}");
            assert!(is_known(locale));
        }
        // Only the locale that *is* the fallback may carry its name.
        assert_eq!(locale_name("en_US"), "English (United States)");
        assert_ne!(locale_name("en_GB"), locale_name("en_US"));
        assert_ne!(locale_name("zh_MY"), locale_name("zh_TW"));
    }

    #[test]
    fn only_zh_cn_needs_the_homepage_patch() {
        for locale in GAME_LOCALES {
            assert_eq!(needs_homepage_patch(locale), *locale == "zh_CN", "{locale}");
            assert_eq!(home_page_works(locale), !needs_homepage_patch(locale), "{locale}");
        }
        assert!(needs_homepage_patch("zh_CN"));
        assert!(!home_page_works("zh_CN"));
        assert!(home_page_works("zh_TW"));
        assert!(home_page_works("zh_MY"));
    }
}
