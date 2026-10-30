#![no_main]

use cpp_demangle::{BorrowedSymbol, DemangleOptions, OwnedSymbol, ParseOptions};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let parse_options = ParseOptions::default();
    let borrowed = BorrowedSymbol::new_with_options(data, &parse_options);
    let owned = OwnedSymbol::new_with_options(data.to_vec(), &parse_options);
    match (borrowed, owned) {
        (Ok(borrowed), Ok(owned)) => {
            for options in [
                DemangleOptions::default(),
                DemangleOptions::new().no_params(),
                DemangleOptions::new().no_return_type(),
                DemangleOptions::new().hide_expression_literal_types(),
                DemangleOptions::new().recursion_limit(8),
            ] {
                // Parsing can succeed while rendering hits its recursion limit
                // or an unresolved reference. Compare errors as well as output.
                assert_eq!(
                    borrowed.demangle_with_options(&options),
                    owned.demangle_with_options(&options),
                );
            }
        }
        (Err(borrowed), Err(owned)) => assert_eq!(borrowed, owned),
        _ => panic!("owned and borrowed parsing disagree"),
    }
});
