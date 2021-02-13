#![allow(
    non_snake_case,
    unused_variables,
    dead_code,
    unused_imports,
    unused_macros,
    incomplete_features,
    non_camel_case_types,
    non_upper_case_globals,
    unreachable_code
)]
#![feature(generic_associated_types, box_syntax, never_type, const_generics)]

#[macro_use]
mod ADT;
mod CoData;
mod GADT;
mod HKT;
// mod HKTMore;
#[macro_use]
mod Monad;
mod Algeff;
mod CHIso;
mod ChurchE;
mod Continuation;
mod Lifting;
mod Monoid;
mod StateMonad;
mod TableDriven;
mod playground;
