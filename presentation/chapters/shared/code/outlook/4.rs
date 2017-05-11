pub mod foo {
    // `semisecret` will be used "many" places within `foo`, but
    // is not meant to be exposed outside of `foo`.
    // (`pub use` would be *rejected*)
    use self::bar::semisecret;

    pub fn bar(z: i32) -> i32 { semisecret(z) }

    mod bar {
        pub(foo) fn semisecret(x: i32) -> i32  { x }
    }
}