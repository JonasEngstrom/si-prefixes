[![Rust](https://github.com/JonasEngstrom/si-prefixes/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/JonasEngstrom/si-prefixes/actions/workflows/rust.yml)

# SI Prefixes

SI Prefixes is a crate for converting between units using different SI prefixes. It is developed as a dependency to be used in other crates requiring unit conversion. The prefixes are based on [*The International System of Units*](https://doi.org/10.59161/AUEZ1291)<sup>1</sup>.

## Usage

The crate includes the factors, names, and symbols listed above for use in calculations and output formatting. It also includes a method to calculate conversion constants that can be multiplied with a value in order to change its prefix.

Usage revolves around the `Prefix` enum. For example the prefix kilo is represented as `Prefix::Kilo`.

Note that, even though *Bureau International des Poids et Mesures* uses lower-case letters for the entire prefixes, the enum variants used in the source code have initial capital letters, as to conform to [*The Rust Style Guide*](https://doc.rust-lang.org/style-guide/advice.html).

### Getting a Prefix Name

```rust
use si_prefixes::Prefix;

let prefix_name = Prefix::Micro.name();

assert_eq!(prefix_name, "micro");
```

### Getting a Prefix Symbol

```rust
use si_prefixes::Prefix;

let prefix_symbol = Prefix::Mega.symbol();

assert_eq!(prefix_symbol, "M");
```

### Getting a Prefix Factor

```rust
use si_prefixes::Prefix;

let prefix_factor = Prefix::Kilo.factor();

assert_eq!(prefix_factor, 1_000f64);
```

### Getting a Prefix Conversion Constant

#### Converting from One Prefix to Another

```rust
use si_prefixes::Prefix;

let centimeters = 50f64;
let decimeters = centimeters * Prefix::conversion_constant(Prefix::Centi, Prefix::Deci);

assert_eq!(decimeters, 5f64);
```

#### Adding a Prefix

```rust
use si_prefixes::Prefix;

let meters = 0.5f64;
let decimeters = meters * Prefix::conversion_constant(Prefix::None, Prefix::Deci);

assert_eq!(decimeters, 5f64);
```

#### Removing a Prefix

```rust
use si_prefixes::Prefix;

let decimeters = 5f64;
let meters = decimeters * Prefix::conversion_constant(Prefix::Deci, Prefix::None);

assert_eq!(meters, 0.5f64);
```

## Included Prefixes

Apart from strict SI system prefixes, that refer to powers of 10, the crate also includes prefixes referring to powers of 2. The included prefixes are listed in the tables below.

### SI Prefixes (Referring to Powers of 10)

|Name|Symbol|Factor|
|-|-|-|
|deca|da|10<sup>1</sup>|
|hecto|h|10<sup>2</sup>|
|kilo|k|10<sup>3</sup>|
|mega|M|10<sup>6</sup>|
|giga|G|10<sup>9</sup>|
|tera|T|10<sup>12</sup>|
|peta|P|10<sup>15</sup>|
|exa|E|10<sup>18</sup>|
|zetta|Z|10<sup>21</sup>|
|yotta|Y|10<sup>24</sup>|
|ronna|R|10<sup>27</sup>|
|quetta|Q|10<sup>30</sup>|
|deci|d|10<sup>-1</sup>|
|centi|c|10<sup>-2</sup>|
|milli|m|10<sup>-3</sup>|
|micro|µ|10<sup>-6</sup>|
|nano|n|10<sup>-9</sup>|
|pico|p|10<sup>-12</sup>|
|femto|f|10<sup>-15</sup>|
|atto|a|10<sup>-18</sup>|
|zepto|z|10<sup>-21</sup>|
|yocto|y|10<sup>-24</sup>|
|ronto|r|10<sup>-27</sup>|
|quecto|q|10<sup>-30</sup>|

### Prefixes Referring to Powers of 2

|Name|Symbol|Factor|
|-|-|-|
|kibi|Ki|2<sup>10</sup>|
|mebi|Mi|2<sup>20</sup>|
|gibi|Gi|2<sup>30</sup>|
|tebi|Ti|2<sup>40</sup>|
|pebi|Pi|2<sup>50</sup>|
|exbi|Ei|2<sup>60</sup>|
|zebi|Zi|2<sup>70</sup>|
|yobi|Yi|2<sup>80</sup>|
|robi|Ri|2<sup>90</sup>|
|quebi|Qi|2<sup>100</sup>|

## References

1. Bureau International des Poids et Mesures. (2025). *Le Système international d’unités/The International System of Units*. 9th edition. [https://doi.org/10.59161/AUEZ1291](https://doi.org/10.59161/AUEZ1291)