use wcl_proc_macros::Matcher;
#[derive(Eq, PartialEq, Hash, Matcher)]
// This enum lists all currently supported file formats.
pub enum Format {
    #[file_format("rs")]
    Rust,
    B,
    C,
    // Source code files for the D programming language have the same extension as make utility's
    // dependency files and there is no real way to distinguish them without actively parsing the
    // file which this program should not be bothered with.
    #[output("Standart ASCII Text")]
    Txt,
    Java,
    #[file_format("rb")]
    Ruby,
    Json,
    #[output("C++")]
    #[file_format("cpp")]
    CPlusPlus,
    #[file_format("cs")]
    #[output("C#")]
    CSharp,
    #[file_format("hs")]
    Haskell,
    #[output("JavaScript")]
    Js,
    #[file_format("adb")]
    Ada,
    Svelte,
    #[file_format("s")]
    Assembly,
    Dart,
    #[file_format("py")]
    Python,
    #[output("TypeScript")]
    Ts,
    Other,
}
