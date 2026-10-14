//! Bundled fallback fonts for standalone operation.
//!
//! The default `full-fonts` feature embeds all 27 font files below.
//! Without it, macOS and Windows use system or caller-supplied fonts.
//! Other targets embed only the four Liberation Sans styles.
//!
//! - **Carlito** — metric-compatible with Calibri (Word's default font),
//!   licensed under the SIL Open Font License 1.1
//! - **Caladea** — metric-compatible with Cambria, licensed under the Apache
//!   License 2.0
//! - **Liberation Sans** — metric-compatible with Arial, licensed under the
//!   SIL Open Font License 1.1
//! - **Liberation Serif** — metric-compatible with Times New Roman, licensed
//!   under the SIL Open Font License 1.1
//! - **Liberation Mono** — metric-compatible with Courier New, licensed under
//!   the SIL Open Font License 1.1
//! - **Noto Sans Arabic**, **Noto Sans Devanagari**, **Noto Sans Thai**,
//!   **Noto Sans SC**, **Noto Sans Hebrew**, **Noto Sans KR**, and **Noto Sans
//!   JP** — deterministic complex-script fallbacks licensed under the SIL Open
//!   Font License 1.1
//!
//! The Noto Sans SC, Hebrew, KR and JP faces are deterministic subsets, not
//! the full families. Each ships only its approved fixture repertoire, which
//! its `fonts/SUBSET-*.md` record names and reproduces.

/// Returns bundled font data: `(family_name, font_bytes)` pairs.
///
/// Returns Carlito, Caladea, Liberation Sans, Liberation Serif, and Liberation
/// Mono in four styles, plus the complex-script fallback fonts.
#[cfg(feature = "full-fonts")]
pub fn bundled_font_data() -> Vec<(&'static str, &'static [u8])> {
    vec![
        // Carlito — metric-compatible replacement for Calibri
        (
            "Carlito",
            include_bytes!("../fonts/Carlito-Regular.ttf").as_slice(),
        ),
        (
            "Carlito",
            include_bytes!("../fonts/Carlito-Bold.ttf").as_slice(),
        ),
        (
            "Carlito",
            include_bytes!("../fonts/Carlito-Italic.ttf").as_slice(),
        ),
        (
            "Carlito",
            include_bytes!("../fonts/Carlito-BoldItalic.ttf").as_slice(),
        ),
        // Caladea — metric-compatible replacement for Cambria
        (
            "Caladea",
            include_bytes!("../fonts/Caladea-Regular.ttf").as_slice(),
        ),
        (
            "Caladea",
            include_bytes!("../fonts/Caladea-Bold.ttf").as_slice(),
        ),
        (
            "Caladea",
            include_bytes!("../fonts/Caladea-Italic.ttf").as_slice(),
        ),
        (
            "Caladea",
            include_bytes!("../fonts/Caladea-BoldItalic.ttf").as_slice(),
        ),
        // Liberation Sans — metric-compatible replacement for Arial
        (
            "Liberation Sans",
            include_bytes!("../fonts/LiberationSans-Regular.ttf").as_slice(),
        ),
        (
            "Liberation Sans",
            include_bytes!("../fonts/LiberationSans-Bold.ttf").as_slice(),
        ),
        (
            "Liberation Sans",
            include_bytes!("../fonts/LiberationSans-Italic.ttf").as_slice(),
        ),
        (
            "Liberation Sans",
            include_bytes!("../fonts/LiberationSans-BoldItalic.ttf").as_slice(),
        ),
        // Liberation Serif — metric-compatible replacement for Times New Roman
        (
            "Liberation Serif",
            include_bytes!("../fonts/LiberationSerif-Regular.ttf").as_slice(),
        ),
        (
            "Liberation Serif",
            include_bytes!("../fonts/LiberationSerif-Bold.ttf").as_slice(),
        ),
        (
            "Liberation Serif",
            include_bytes!("../fonts/LiberationSerif-Italic.ttf").as_slice(),
        ),
        (
            "Liberation Serif",
            include_bytes!("../fonts/LiberationSerif-BoldItalic.ttf").as_slice(),
        ),
        // Liberation Mono — metric-compatible replacement for Courier New
        (
            "Liberation Mono",
            include_bytes!("../fonts/LiberationMono-Regular.ttf").as_slice(),
        ),
        (
            "Liberation Mono",
            include_bytes!("../fonts/LiberationMono-Bold.ttf").as_slice(),
        ),
        (
            "Liberation Mono",
            include_bytes!("../fonts/LiberationMono-Italic.ttf").as_slice(),
        ),
        (
            "Liberation Mono",
            include_bytes!("../fonts/LiberationMono-BoldItalic.ttf").as_slice(),
        ),
        (
            "Noto Sans Arabic",
            include_bytes!("../fonts/NotoSansArabic.ttf").as_slice(),
        ),
        (
            "Noto Sans Devanagari",
            include_bytes!("../fonts/NotoSansDevanagari.ttf").as_slice(),
        ),
        (
            "Noto Sans Thai",
            include_bytes!("../fonts/NotoSansThai.ttf").as_slice(),
        ),
        (
            "Noto Sans SC",
            include_bytes!("../fonts/NotoSansSC-FX058-subset.ttf").as_slice(),
        ),
        (
            "Noto Sans Hebrew",
            include_bytes!("../fonts/NotoSansHebrew-F266a-subset.ttf").as_slice(),
        ),
        (
            "Noto Sans KR",
            include_bytes!("../fonts/NotoSansKR-F266a-subset.ttf").as_slice(),
        ),
        (
            "Noto Sans JP",
            include_bytes!("../fonts/NotoSansJP-F266a-subset.ttf").as_slice(),
        ),
    ]
}

/// Returns no bundled fonts on desktop targets without `full-fonts`.
#[cfg(all(
    not(feature = "full-fonts"),
    any(target_os = "macos", target_os = "windows")
))]
pub fn bundled_font_data() -> Vec<(&'static str, &'static [u8])> {
    Vec::new()
}

/// Returns the four Liberation Sans styles on other targets without `full-fonts`.
#[cfg(all(
    not(feature = "full-fonts"),
    not(any(target_os = "macos", target_os = "windows"))
))]
pub fn bundled_font_data() -> Vec<(&'static str, &'static [u8])> {
    vec![
        (
            "Liberation Sans",
            include_bytes!("../fonts/LiberationSans-Regular.ttf").as_slice(),
        ),
        (
            "Liberation Sans",
            include_bytes!("../fonts/LiberationSans-Bold.ttf").as_slice(),
        ),
        (
            "Liberation Sans",
            include_bytes!("../fonts/LiberationSans-Italic.ttf").as_slice(),
        ),
        (
            "Liberation Sans",
            include_bytes!("../fonts/LiberationSans-BoldItalic.ttf").as_slice(),
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::bundled_font_data;
    use std::collections::BTreeSet;
    use std::path::Path;

    #[test]
    fn every_bundled_font_family_has_a_licence_file() {
        let family_licences = [
            ("Caladea", "LICENSE-Caladea"),
            ("Carlito", "LICENSE-Carlito"),
            ("Liberation Mono", "LICENSE-Liberation"),
            ("Liberation Sans", "LICENSE-Liberation"),
            ("Liberation Serif", "LICENSE-Liberation"),
            ("Noto Sans Arabic", "LICENSE-Noto"),
            ("Noto Sans Devanagari", "LICENSE-Noto"),
            ("Noto Sans Hebrew", "LICENSE-Noto"),
            ("Noto Sans JP", "LICENSE-Noto"),
            ("Noto Sans KR", "LICENSE-Noto"),
            ("Noto Sans SC", "LICENSE-Noto"),
            ("Noto Sans Thai", "LICENSE-Noto"),
        ];
        let bundled_families = bundled_font_data()
            .into_iter()
            .map(|(family, _)| family)
            .collect::<BTreeSet<_>>();
        let licensed_families = family_licences
            .iter()
            .map(|(family, _)| *family)
            .collect::<BTreeSet<_>>();

        assert!(bundled_families.is_subset(&licensed_families));
        #[cfg(feature = "full-fonts")]
        {
            assert_eq!(bundled_families, licensed_families);
            assert_eq!(bundled_font_data().len(), 27);
        }

        let fonts_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("fonts");
        for (family, licence_file) in family_licences {
            assert!(
                fonts_dir.join(licence_file).is_file(),
                "{family} is missing {licence_file}"
            );
        }

        assert!(fonts_dir.join("NOTICE-Caladea").is_file());
        assert!(fonts_dir.join("NOTICE-Noto").is_file());
        for subset_record in [
            "SUBSET-NotoSansSC.md",
            "SUBSET-NotoSansHebrew.md",
            "SUBSET-NotoSansKR.md",
            "SUBSET-NotoSansJP.md",
        ] {
            assert!(
                fonts_dir.join(subset_record).is_file(),
                "{subset_record} is missing"
            );
        }
    }

    #[cfg(feature = "full-fonts")]
    #[test]
    fn deterministic_complex_script_fonts_cover_the_approved_fixture_repertoire() {
        let fixtures = [
            ("Noto Sans Arabic", "العربية"),
            ("Noto Sans Devanagari", "कि"),
            ("Noto Sans Thai", "ภาษาไทยยินดีต้อนรับ"),
            ("Noto Sans SC", "〈中〉、你好世界"),
            ("Noto Sans Hebrew", "שלום עולם"),
            ("Noto Sans KR", "안녕하세요 세계"),
            ("Noto Sans JP", "こんにちは、カタカナ世界"),
        ];
        for (family, text) in fixtures {
            let bytes = bundled_font_data()
                .into_iter()
                .find_map(|(candidate, bytes)| (candidate == family).then_some(bytes))
                .expect("approved family is bundled");
            let face = ttf_parser::Face::parse(bytes, 0).expect("bundled font parses");
            assert!(
                text.chars()
                    .filter(|character| !character.is_whitespace())
                    .all(|character| face.glyph_index(character).is_some()),
                "{family} misses its approved fixture repertoire"
            );
        }
    }

    #[cfg(not(feature = "full-fonts"))]
    #[test]
    fn small_builds_embed_only_the_platform_fallback_fonts() {
        let fonts = bundled_font_data();
        if cfg!(any(target_os = "macos", target_os = "windows")) {
            assert!(fonts.is_empty());
        } else {
            assert_eq!(fonts.len(), 4);
            for ((family, bytes), (bold, italic)) in
                fonts
                    .into_iter()
                    .zip([(false, false), (true, false), (false, true), (true, true)])
            {
                assert_eq!(family, "Liberation Sans");
                let face = ttf_parser::Face::parse(bytes, 0).expect("fallback font parses");
                assert_eq!(face.is_bold(), bold);
                assert_eq!(face.is_italic(), italic);
                assert!(
                    "HỆ QUẢN TRỊ"
                        .chars()
                        .all(|ch| face.glyph_index(ch).is_some())
                );
            }
        }
    }

    #[cfg(not(feature = "full-fonts"))]
    #[test]
    fn caller_fonts_render_vietnamese_without_the_full_bundle() {
        let mut fonts = crate::FontManager::new_deterministic().expect("font database loads");
        if cfg!(any(target_os = "macos", target_os = "windows")) {
            assert!(fonts.resolve_font(None, false, false).is_err());
        }
        fonts.load_additional_fonts(&[crate::FontFile {
            family: "Liberation Sans".to_owned(),
            data: include_bytes!("../fonts/LiberationSans-Regular.ttf").to_vec(),
        }]);
        let font_id = fonts
            .resolve_font_for_text(Some("Liberation Sans"), false, false, "HỆ QUẢN TRỊ")
            .expect("caller font covers Vietnamese");
        let shaped = fonts.shape_text(font_id, "HỆ QUẢN TRỊ", 12.0).unwrap();
        assert!(shaped.width > 0.0);
        assert!(!shaped.glyph_ids.is_empty());
        assert!(shaped.glyph_ids.iter().all(|id| *id != 0));
    }
}
