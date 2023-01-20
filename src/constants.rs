// about const, see more: 
//   https://github.com/rust-lang/rfcs/blob/master/text/0246-const-vs-static.md
//   https://stackoverflow.com/questions/52751597/what-is-the-difference-between-a-constant-and-a-static-variable-and-which-should
// about env! marco, see more: 
//   https://stackoverflow.com/questions/27840394/how-can-a-rust-program-access-metadata-from-its-cargo-package
pub const MANIFEST_DIR: &'static str = env!("CARGO_MANIFEST_DIR");