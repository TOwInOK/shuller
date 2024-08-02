/// This is `Rule34`'s rule to create `Link`s
pub mod rule34;

/// Concat 2 arraes into 1 [String] with adding "-" for itch item in second array
///
/// **inputs**: array - [Vec<&str>]||[`Vec<String>`] narray - [Vec<&str>]||[`Vec<String>`]
///
/// **out** - [String]
#[macro_export]
macro_rules! tag_suppress {
    ($array:expr, $narray:expr) => {{
        let nt: Vec<String> = $narray.iter().map(|x| format!("-{}", x)).collect();
        let nt = nt.join(" ");
        let pt = $array.join(" ");
        format!("{} {}", pt, nt)
    }};
}

/// Bool converter to [str]
///
/// **input** - [bool]
///
/// **out** - [str]
#[macro_export]
macro_rules! toggler {
    ($toggle:expr) => {
        if $toggle {
            "1"
        } else {
            "0"
        }
    };
}
