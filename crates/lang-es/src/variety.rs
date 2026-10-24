/// Which Spanish (docs/decisions.md, decision 36).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Variety {
    /// Spain's: θ in cielo and caza, ʎ in calle (eSpeak NG's `es` voice).
    #[default]
    Castilian,
    /// Latin America's: s in cielo and caza (seseo), ʝ in calle (yeísmo; eSpeak NG's `es-419`).
    LatinAmerican,
}
