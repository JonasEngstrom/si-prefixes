pub enum Prefix {
    Deca,
    Hecto,
    Kilo,
    Mega,
    Giga,
    Tera,
    Peta,
    Exa,
    Zetta,
    Yotta,
    Ronna,
    Quetta,
    Deci,
    Centi,
    Milli,
    Micro,
    Nano,
    Pico,
    Femto,
    Atto,
    Zepto,
    Yocto,
    Ronto,
    Quecto,
    Kibi,
    Mebi,
    Gibi,
    Tebi,
    Pebi,
    Exbi,
    Zebi,
    Yobi,
    Robi,
    Quebi,
}

impl Prefix {
    pub fn factor(&self) -> f64 {
        match self {
            Prefix::Deca => 1e1f64,
            Prefix::Hecto => 1e2f64,
            Prefix::Kilo => 1e3f64,
            Prefix::Mega => 1e6f64,
            Prefix::Giga => 1e9f64,
            Prefix::Tera => 1e12f64,
            Prefix::Peta => 1e15f64,
            Prefix::Exa => 1e18f64,
            Prefix::Zetta => 1e21f64,
            Prefix::Yotta => 1e24f64,
            Prefix::Ronna => 1e27f64,
            Prefix::Quetta => 1e30f64,
            Prefix::Deci => 1e-1f64,
            Prefix::Centi => 1e-2f64,
            Prefix::Milli => 1e-3f64,
            Prefix::Micro => 1e-6f64,
            Prefix::Nano => 1e-9f64,
            Prefix::Pico => 1e-12f64,
            Prefix::Femto => 1e-15f64,
            Prefix::Atto => 1e-18f64,
            Prefix::Zepto => 1e-21f64,
            Prefix::Yocto => 1e-24f64,
            Prefix::Ronto => 1e-27f64,
            Prefix::Quecto => 1e-30f64,
            Prefix::Kibi => 10f64.exp2(),
            Prefix::Mebi => 20f64.exp2(),
            Prefix::Gibi => 30f64.exp2(),
            Prefix::Tebi => 40f64.exp2(),
            Prefix::Pebi => 50f64.exp2(),
            Prefix::Exbi => 60f64.exp2(),
            Prefix::Zebi => 70f64.exp2(),
            Prefix::Yobi => 80f64.exp2(),
            Prefix::Robi => 90f64.exp2(),
            Prefix::Quebi => 100f64.exp2(),
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Prefix::Deca => "deca",
            Prefix::Hecto => "hecto",
            Prefix::Kilo => "kilo",
            Prefix::Mega => "mega",
            Prefix::Giga => "giga",
            Prefix::Tera => "tera",
            Prefix::Peta => "peta",
            Prefix::Exa => "exa",
            Prefix::Zetta => "zetta",
            Prefix::Yotta => "yotta",
            Prefix::Ronna => "ronna",
            Prefix::Quetta => "quetta",
            Prefix::Deci => "deci",
            Prefix::Centi => "centi",
            Prefix::Milli => "milli",
            Prefix::Micro => "micro",
            Prefix::Nano => "nano",
            Prefix::Pico => "pico",
            Prefix::Femto => "femto",
            Prefix::Atto => "atto",
            Prefix::Zepto => "zepto",
            Prefix::Yocto => "yocto",
            Prefix::Ronto => "ronto",
            Prefix::Quecto => "quecto",
            Prefix::Kibi => "kibi",
            Prefix::Mebi => "mebi",
            Prefix::Gibi => "gibi",
            Prefix::Tebi => "tebi",
            Prefix::Pebi => "pebi",
            Prefix::Exbi => "exbi",
            Prefix::Zebi => "zebi",
            Prefix::Yobi => "yobi",
            Prefix::Robi => "robi",
            Prefix::Quebi => "quebi",
        }
    }

    pub fn symbol(&self) -> &'static str {
        match self {
            Prefix::Deca => "da",
            Prefix::Hecto => "h",
            Prefix::Kilo => "k",
            Prefix::Mega => "M",
            Prefix::Giga => "G",
            Prefix::Tera => "T",
            Prefix::Peta => "P",
            Prefix::Exa => "E",
            Prefix::Zetta => "Z",
            Prefix::Yotta => "Y",
            Prefix::Ronna => "R",
            Prefix::Quetta => "Q",
            Prefix::Deci => "d",
            Prefix::Centi => "c",
            Prefix::Milli => "m",
            Prefix::Micro => "µ",
            Prefix::Nano => "n",
            Prefix::Pico => "p",
            Prefix::Femto => "f",
            Prefix::Atto => "a",
            Prefix::Zepto => "z",
            Prefix::Yocto => "y",
            Prefix::Ronto => "r",
            Prefix::Quecto => "q",
            Prefix::Kibi => "Ki",
            Prefix::Mebi => "Mi",
            Prefix::Gibi => "Gi",
            Prefix::Tebi => "Ti",
            Prefix::Pebi => "Pi",
            Prefix::Exbi => "Ei",
            Prefix::Zebi => "Zi",
            Prefix::Yobi => "Yi",
            Prefix::Robi => "Ri",
            Prefix::Quebi => "Qi",
        }
    }

    pub fn conversion_constant(from: Self, to: Self) -> f64 {
        from.factor() / to.factor()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factors_are_correct() {
        assert_eq!(Prefix::Deca.factor(), 10f64);
        assert_eq!(Prefix::Hecto.factor(), 100f64);
        assert_eq!(Prefix::Kilo.factor(), 1_000f64);
        assert_eq!(Prefix::Mega.factor(), 1_000_000f64);
        assert_eq!(Prefix::Giga.factor(), 1_000_000_000f64);
        assert_eq!(Prefix::Tera.factor(), 1_000_000_000_000f64);
        assert_eq!(Prefix::Peta.factor(), 1_000_000_000_000_000f64);
        assert_eq!(Prefix::Exa.factor(), 1_000_000_000_000_000_000f64);
        assert_eq!(Prefix::Zetta.factor(), 1_000_000_000_000_000_000_000f64);
        assert_eq!(Prefix::Yotta.factor(), 1_000_000_000_000_000_000_000_000f64);
        assert_eq!(Prefix::Ronna.factor(), 1_000_000_000_000_000_000_000_000_000f64);
        assert_eq!(Prefix::Quetta.factor(), 1_000_000_000_000_000_000_000_000_000_000f64);

        assert_eq!(Prefix::Deci.factor(), 0.1f64);
        assert_eq!(Prefix::Centi.factor(), 0.01f64);
        assert_eq!(Prefix::Milli.factor(), 0.001f64);
        assert_eq!(Prefix::Micro.factor(), 0.000_001f64);
        assert_eq!(Prefix::Nano.factor(), 0.000_000_001f64);
        assert_eq!(Prefix::Pico.factor(), 0.000_000_000_001f64);
        assert_eq!(Prefix::Femto.factor(), 0.000_000_000_000_001f64);
        assert_eq!(Prefix::Atto.factor(), 0.000_000_000_000_000_001f64);
        assert_eq!(Prefix::Zepto.factor(), 0.000_000_000_000_000_000_001f64);
        assert_eq!(Prefix::Yocto.factor(), 0.000_000_000_000_000_000_000_001f64);
        assert_eq!(Prefix::Ronto.factor(), 0.000_000_000_000_000_000_000_000_001f64);
        assert_eq!(Prefix::Quecto.factor(), 0.000_000_000_000_000_000_000_000_000_001f64);

        assert_eq!(Prefix::Kibi.factor(), 1_024f64);
        assert_eq!(Prefix::Mebi.factor(), 1_048_576f64);
        assert_eq!(Prefix::Gibi.factor(), 1_073_741_824f64);
        assert_eq!(Prefix::Tebi.factor(), 1_099_511_627_776f64);
        assert_eq!(Prefix::Pebi.factor(), 1_125_899_906_842_624f64);
        assert_eq!(Prefix::Exbi.factor(), 1_152_921_504_606_846_976f64);
        assert_eq!(Prefix::Zebi.factor(), 1_180_591_620_717_411_303_424f64);
        assert_eq!(Prefix::Yobi.factor(), 1_208_925_819_614_629_174_706_176f64);
        assert_eq!(Prefix::Robi.factor(), 1_237_940_039_285_380_274_899_124_224f64);
        assert_eq!(Prefix::Quebi.factor(), 1_267_650_600_228_229_401_496_703_205_376f64);
    }

    #[test]
    fn names_are_correct() {
        assert_eq!(Prefix::Deca.name(), "deca");
        assert_eq!(Prefix::Hecto.name(), "hecto");
        assert_eq!(Prefix::Kilo.name(), "kilo");
        assert_eq!(Prefix::Mega.name(), "mega");
        assert_eq!(Prefix::Giga.name(), "giga");
        assert_eq!(Prefix::Tera.name(), "tera");
        assert_eq!(Prefix::Peta.name(), "peta");
        assert_eq!(Prefix::Exa.name(), "exa");
        assert_eq!(Prefix::Zetta.name(), "zetta");
        assert_eq!(Prefix::Yotta.name(), "yotta");
        assert_eq!(Prefix::Ronna.name(), "ronna");
        assert_eq!(Prefix::Quetta.name(), "quetta");

        assert_eq!(Prefix::Deci.name(), "deci");
        assert_eq!(Prefix::Centi.name(), "centi");
        assert_eq!(Prefix::Milli.name(), "milli");
        assert_eq!(Prefix::Micro.name(), "micro");
        assert_eq!(Prefix::Nano.name(), "nano");
        assert_eq!(Prefix::Pico.name(), "pico");
        assert_eq!(Prefix::Femto.name(), "femto");
        assert_eq!(Prefix::Atto.name(), "atto");
        assert_eq!(Prefix::Zepto.name(), "zepto");
        assert_eq!(Prefix::Yocto.name(), "yocto");
        assert_eq!(Prefix::Ronto.name(), "ronto");
        assert_eq!(Prefix::Quecto.name(), "quecto");

        assert_eq!(Prefix::Kibi.name(), "kibi");
        assert_eq!(Prefix::Mebi.name(), "mebi");
        assert_eq!(Prefix::Gibi.name(), "gibi");
        assert_eq!(Prefix::Tebi.name(), "tebi");
        assert_eq!(Prefix::Pebi.name(), "pebi");
        assert_eq!(Prefix::Exbi.name(), "exbi");
        assert_eq!(Prefix::Zebi.name(), "zebi");
        assert_eq!(Prefix::Yobi.name(), "yobi");
        assert_eq!(Prefix::Robi.name(), "robi");
        assert_eq!(Prefix::Quebi.name(), "quebi");
    }

    #[test]
    fn symbols_are_correct() {
        assert_eq!(Prefix::Deca.symbol(), "da");
        assert_eq!(Prefix::Hecto.symbol(), "h");
        assert_eq!(Prefix::Kilo.symbol(), "k");
        assert_eq!(Prefix::Mega.symbol(), "M");
        assert_eq!(Prefix::Giga.symbol(), "G");
        assert_eq!(Prefix::Tera.symbol(), "T");
        assert_eq!(Prefix::Peta.symbol(), "P");
        assert_eq!(Prefix::Exa.symbol(), "E");
        assert_eq!(Prefix::Zetta.symbol(), "Z");
        assert_eq!(Prefix::Yotta.symbol(), "Y");
        assert_eq!(Prefix::Ronna.symbol(), "R");
        assert_eq!(Prefix::Quetta.symbol(), "Q");

        assert_eq!(Prefix::Deci.symbol(), "d");
        assert_eq!(Prefix::Centi.symbol(), "c");
        assert_eq!(Prefix::Milli.symbol(), "m");
        assert_eq!(Prefix::Micro.symbol(), "µ");
        assert_eq!(Prefix::Nano.symbol(), "n");
        assert_eq!(Prefix::Pico.symbol(), "p");
        assert_eq!(Prefix::Femto.symbol(), "f");
        assert_eq!(Prefix::Atto.symbol(), "a");
        assert_eq!(Prefix::Zepto.symbol(), "z");
        assert_eq!(Prefix::Yocto.symbol(), "y");
        assert_eq!(Prefix::Ronto.symbol(), "r");
        assert_eq!(Prefix::Quecto.symbol(), "q");

        assert_eq!(Prefix::Kibi.symbol(), "Ki");
        assert_eq!(Prefix::Mebi.symbol(), "Mi");
        assert_eq!(Prefix::Gibi.symbol(), "Gi");
        assert_eq!(Prefix::Tebi.symbol(), "Ti");
        assert_eq!(Prefix::Pebi.symbol(), "Pi");
        assert_eq!(Prefix::Exbi.symbol(), "Ei");
        assert_eq!(Prefix::Zebi.symbol(), "Zi");
        assert_eq!(Prefix::Yobi.symbol(), "Yi");
        assert_eq!(Prefix::Robi.symbol(), "Ri");
        assert_eq!(Prefix::Quebi.symbol(), "Qi");
    }

    #[test]
    fn conversion_constant_correct() {
        let decimeters = 5f64;
        let centimeters = 50f64;
        assert_eq!(decimeters * Prefix::conversion_constant(Prefix::Deci, Prefix::Centi), centimeters);
        assert_eq!(centimeters * Prefix::conversion_constant(Prefix::Centi, Prefix::Deci), decimeters);
    }
}
