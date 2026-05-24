//! # SI Prefixes
//! 
//! SI Prefixes is a crate for converting between units using different SI prefixes. It is developed as a dependency to be used in other crates requiring unit conversion. The prefixes are based on [*The International System of Units*](https://doi.org/10.59161/AUEZ1291)<sup>1</sup>.
//! 
//! ## Included Prefixes
//!
//! Apart from strict SI system prefixes, that refer to powers of 10, the crate also includes prefixes referring to powers of 2. The included prefixes are listed in the tables below.
//! 
//! ### SI Prefixes (Referring to Powers of 10)
//! 
//! |Name|Symbol|Factor|
//! |-|-|-|
//! |deca|da|10<sup>1</sup>|
//! |hecto|h|10<sup>2</sup>|
//! |kilo|k|10<sup>3</sup>|
//! |mega|M|10<sup>6</sup>|
//! |giga|G|10<sup>9</sup>|
//! |tera|T|10<sup>12</sup>|
//! |peta|P|10<sup>15</sup>|
//! |exa|E|10<sup>18</sup>|
//! |zetta|Z|10<sup>21</sup>|
//! |yotta|Y|10<sup>24</sup>|
//! |ronna|R|10<sup>27</sup>|
//! |quetta|Q|10<sup>30</sup>|
//! |deci|d|10<sup>-1</sup>|
//! |centi|c|10<sup>-2</sup>|
//! |milli|m|10<sup>-3</sup>|
//! |micro|µ|10<sup>-6</sup>|
//! |nano|n|10<sup>-9</sup>|
//! |pico|p|10<sup>-12</sup>|
//! |femto|f|10<sup>-15</sup>|
//! |atto|a|10<sup>-18</sup>|
//! |zepto|z|10<sup>-21</sup>|
//! |yocto|y|10<sup>-24</sup>|
//! |ronto|r|10<sup>-27</sup>|
//! |quecto|q|10<sup>-30</sup>|
//! 
//! ### Prefixes Referring to Powers of 2
//! 
//! |Name|Symbol|Factor|
//! |-|-|-|
//! |kibi|Ki|2<sup>10</sup>|
//! |mebi|Mi|2<sup>20</sup>|
//! |gibi|Gi|2<sup>30</sup>|
//! |tebi|Ti|2<sup>40</sup>|
//! |pebi|Pi|2<sup>50</sup>|
//! |exbi|Ei|2<sup>60</sup>|
//! |zebi|Zi|2<sup>70</sup>|
//! |yobi|Yi|2<sup>80</sup>|
//! |robi|Ri|2<sup>90</sup>|
//! |quebi|Qi|2<sup>100</sup>|
//! 
//! ## Usage
//! 
//! The crate includes the factors, names, and symbols listed above for use in calculations and output formatting. It also includes a method to calculate conversion constants that can be multiplied with a value in order to change its prefix.
//! 
//! Usage revolves around the `Prefix` enum. For example the prefix kilo is represented as `Prefix::Kilo`.
//! 
//! Note that, even though *Bureau International des Poids et Mesures* uses lower-case letters for the entire prefixes, the enum variants used in the source code have initial capital letters, as to conform to [*The Rust Style Guide*](https://doc.rust-lang.org/style-guide/advice.html).
//! 
//! ### Getting a Prefix Name
//! 
//! ```
//! use si_prefixes::Prefix;
//! 
//! let prefix_name = Prefix::Micro.name();
//! 
//! assert_eq!(prefix_name, "micro");
//! ```
//! 
//! ### Getting a Prefix Symbol
//! 
//! ```
//! use si_prefixes::Prefix;
//! 
//! let prefix_symbol = Prefix::Mega.symbol();
//! 
//! assert_eq!(prefix_symbol, "M");
//! ```
//! 
//! ### Getting a Prefix Factor
//! 
//! ```
//! use si_prefixes::Prefix;
//! 
//! let prefix_factor = Prefix::Kilo.factor();
//! 
//! assert_eq!(prefix_factor, 1_000f64);
//! ```
//!
//! ### Getting a Prefix Conversion Constant
//! 
//! #### Converting from One Prefix to Another
//! 
//! ```
//! use si_prefixes::Prefix;
//! 
//! let centimeters = 50f64;
//! let decimeters = centimeters * Prefix::conversion_constant(Prefix::Centi, Prefix::Deci);
//! 
//! assert_eq!(decimeters, 5f64);
//! ```
//! 
//! #### Adding a Prefix
//! 
//! ```
//! use si_prefixes::Prefix;
//! 
//! let meters = 0.5f64;
//! let decimeters = meters * Prefix::conversion_constant(Prefix::None, Prefix::Deci);
//! 
//! assert_eq!(decimeters, 5f64);
//! ```
//! 
//! #### Removing a Prefix
//! 
//! ```
//! use si_prefixes::Prefix;
//! 
//! let decimeters = 5f64;
//! let meters = decimeters * Prefix::conversion_constant(Prefix::Deci, Prefix::None);
//! 
//! assert_eq!(meters, 0.5f64);
//! ```
//! 
//! ## References
//!
//! 1. Bureau International des Poids et Mesures. (2025). *Le Système international d'unités/The International System of Units* [Brochure]. 9th edition. [https://doi.org/10.59161/AUEZ1291](https://doi.org/10.59161/AUEZ1291)

pub enum Prefix {
    /// Name: deca, symbol: da, factor: 10<sup>1</sup>
    Deca,
    /// Name: hecto, symbol: h, factor: 10<sup>2</sup>
    Hecto,
    /// Name: kilo, symbol: k, factor: 10<sup>3</sup>
    Kilo,
    /// Name: mega, symbol: M, factor: 10<sup>6</sup>
    Mega,
    /// Name: giga, symbol: G, factor: 10<sup>9</sup>
    Giga,
    /// Name: tera, symbol: T, factor: 10<sup>12</sup>
    Tera,
    /// Name: peta, symbol: P, factor: 10<sup>15</sup>
    Peta,
    /// Name: exa, symbol: E, factor: 10<sup>18</sup>
    Exa,
    /// Name: zetta, symbol: Z, factor: 10<sup>21</sup>
    Zetta,
    /// Name: yotta, symbol: Y, factor: 10<sup>24</sup>
    Yotta,
    /// Name: ronna, symbol: R, factor: 10<sup>27</sup>
    Ronna,
    /// Name: quetta, symbol: Q, factor: 10<sup>30</sup>
    Quetta,
    /// Name: deci, symbol: d, factor: 10<sup>-1</sup>
    Deci,
    /// Name: centi, symbol: c, factor: 10<sup>-2</sup>
    Centi,
    /// Name: milli, symbol: m, factor: 10<sup>-3</sup>
    Milli,
    /// Name: micro, symbol: µ, factor: 10<sup>-6</sup>
    Micro,
    /// Name: nano, symbol: n, factor: 10<sup>-9</sup>
    Nano,
    /// Name: pico, symbol: p, factor: 10<sup>-12</sup>
    Pico,
    /// Name: femto, symbol: f, factor: 10<sup>-15</sup>
    Femto,
    /// Name: atto, symbol: a, factor: 10<sup>-18</sup>
    Atto,
    /// Name: zepto, symbol: z, factor: 10<sup>-21</sup>
    Zepto,
    /// Name: yocto, symbol: y, factor: 10<sup>-24</sup>
    Yocto,
    /// Name: ronto, symbol: r, factor: 10<sup>-27</sup>
    Ronto,
    /// Name: quecto, symbol: q, factor: 10<sup>-30</sup>
    Quecto,
    /// Name: kibi, symbol: Ki, factor: 2<sup>10</sup>
    Kibi,
    /// Name: mebi, symbol: Mi, factor: 2<sup>20</sup>
    Mebi,
    /// Name: gibi, symbol: Gi, factor: 2<sup>30</sup>
    Gibi,
    /// Name: tebi, symbol: Ti, factor: 2<sup>40</sup>
    Tebi,
    /// Name: pebi, symbol: Pi, factor: 2<sup>50</sup>
    Pebi,
    /// Name: exbi, symbol: Ei, factor: 2<sup>60</sup>
    Exbi,
    /// Name: zebi, symbol: Zi, factor: 2<sup>70</sup>
    Zebi,
    /// Name: yobi, symbol: Yi, factor: 2<sup>80</sup>
    Yobi,
    /// Name: robi, symbol: Ri, factor: 2<sup>90</sup>
    Robi,
    /// Name: quebi, symbol: Qi, factor: 2<sup>100</sup>
    Quebi,
    /// Name: no name, symbol: no symbol, factor: 1
    None,
}

impl Prefix {
    /// Returns the factor associated with a prefix.
    /// 
    /// ```
    /// use si_prefixes::Prefix;
    /// 
    /// let unit_factor = Prefix::Milli.factor();
    /// 
    /// assert_eq!(unit_factor, 0.001f64);
    /// ```
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
            Prefix::None => 1f64,
        }
    }

    /// Returns the name of a prefix.
    /// 
    /// ```
    /// use si_prefixes::Prefix;
    /// 
    /// let unit_name = Prefix::Kilo.name();
    /// 
    /// assert_eq!(unit_name, "kilo");
    /// ```
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
            Prefix::None => "",
        }
    }

    /// Returns the symbol of a prefix.
    /// 
    /// ```
    /// use si_prefixes::Prefix;
    /// 
    /// let unit_symbol = Prefix::Kibi.symbol();
    /// 
    /// assert_eq!(unit_symbol, "Ki");
    /// ```
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
            Prefix::None => "",
        }
    }

    /// Calculates and returns a conversion constant that can be multiplied with a value to change its prefix.
    /// ## Converting from One Prefix to Another
    /// 
    /// ```
    /// use si_prefixes::Prefix;
    /// 
    /// let centimeters = 50f64;
    /// let decimeters = centimeters * Prefix::conversion_constant(Prefix::Centi, Prefix::Deci);
    /// 
    /// assert_eq!(decimeters, 5f64);
    /// ```
    /// 
    /// ## Adding a Prefix
    /// 
    /// ```
    /// use si_prefixes::Prefix;
    /// 
    /// let meters = 0.5f64;
    /// let decimeters = meters * Prefix::conversion_constant(Prefix::None, Prefix::Deci);
    /// 
    /// assert_eq!(decimeters, 5f64);
    /// ```
    /// 
    /// ## Removing a Prefix
    /// 
    /// ```
    /// use si_prefixes::Prefix;
    /// 
    /// let decimeters = 5f64;
    /// let meters = decimeters * Prefix::conversion_constant(Prefix::Deci, Prefix::None);
    /// 
    /// assert_eq!(meters, 0.5f64);
    /// ```
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

        assert_eq!(Prefix::None.factor(), 1f64);
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

        assert_eq!(Prefix::None.name(), "");
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
        
        assert_eq!(Prefix::None.symbol(), "")
    }

    #[test]
    fn conversion_constant_is_correct() {
        let meters = 0.5f64;
        let decimeters = 5f64;
        let centimeters = 50f64;

        assert_eq!(meters * Prefix::conversion_constant(Prefix::None, Prefix::None), meters);
        assert_eq!(meters * Prefix::conversion_constant(Prefix::None, Prefix::Deci), decimeters);
        assert_eq!(meters * Prefix::conversion_constant(Prefix::None, Prefix::Centi), centimeters);

        assert_eq!(decimeters * Prefix::conversion_constant(Prefix::Deci, Prefix::None), meters);
        assert_eq!(decimeters * Prefix::conversion_constant(Prefix::Deci, Prefix::Deci), decimeters);
        assert_eq!(decimeters * Prefix::conversion_constant(Prefix::Deci, Prefix::Centi), centimeters);

        assert_eq!(centimeters * Prefix::conversion_constant(Prefix::Centi, Prefix::Deci), decimeters);
        assert_eq!(centimeters * Prefix::conversion_constant(Prefix::Centi, Prefix::None), meters);
        assert_eq!(centimeters * Prefix::conversion_constant(Prefix::Centi, Prefix::Centi), centimeters);
    }
}
