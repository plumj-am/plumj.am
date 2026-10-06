//! Iconify icon sets rendered by the site.
//!
//! The sets are staged from `build.rs` into `normal/icons` and committed, so
//! builds never download them.

topcoat::icon::iconify::include!(pub "lucide");
topcoat::icon::iconify::include!(pub "simple-icons");
