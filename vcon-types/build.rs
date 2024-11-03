fn main() {
    cfg_aliases::cfg_aliases! {
        ser: { any(feature = "json", feature = "cbor") },
        json: { feature = "json" },
        cbor: { feature = "cbor" },
    }
}
