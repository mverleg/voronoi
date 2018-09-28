#![feature(prelude_import)]
#![no_std]
#![feature(duration_as_u128)]

#[prelude_import]
use ::std::prelude::v1::*;

#[macro_use]
extern crate std;

extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::time::Instant;
use std::fmt::Debug;

pub fn main() {
    let mut rng = rand::thread_rng();
    let mut data_4_deriv: Vec<Deriv> =
        (0..25000).map(|_| Deriv { x: rng.gen_range(0, 5000) }).collect();
    let mut data_4_manual: Vec<Manual> =
        data_4_deriv.iter().map(|d| Manual { x: d.x }).collect();
    let start = Instant::now();
    bubble_sort(&mut data_4_deriv);


    // https://rosettacode.org/wiki/Sorting_algorithms/Bubble_sort#Rust


    {
        ::io::_print(::std::fmt::Arguments::new_v1_formatted(&["derive: ",
            "ms\n"],
                                                             &match (&Instant::now().duration_since(start).as_millis(), )
                                                                 {
                                                                     (arg0, ) =>
                                                                         [::std::fmt::ArgumentV1::new(arg0,
                                                                                                      ::std::fmt::Display::fmt)],
                                                                 },
                                                             &[::std::fmt::rt::v1::Argument {
                                                                 position:
                                                                 ::std::fmt::rt::v1::Position::At(0usize),
                                                                 format:
                                                                 ::std::fmt::rt::v1::FormatSpec {
                                                                     fill:
                                                                     ' ',
                                                                     align:
                                                                     ::std::fmt::rt::v1::Alignment::Unknown,
                                                                     flags:
                                                                     0u32,
                                                                     precision:
                                                                     ::std::fmt::rt::v1::Count::Is(3usize),
                                                                     width:
                                                                     ::std::fmt::rt::v1::Count::Implied,
                                                                 },
                                                             }]));
    };
    let start = Instant::now();
    bubble_sort(&mut data_4_manual);
    {
        ::io::_print(::std::fmt::Arguments::new_v1_formatted(&["derive: ",
            "ms\n"],
                                                             &match (&Instant::now().duration_since(start).as_millis(), )
                                                                 {
                                                                     (arg0, ) =>
                                                                         [::std::fmt::ArgumentV1::new(arg0,
                                                                                                      ::std::fmt::Display::fmt)],
                                                                 },
                                                             &[::std::fmt::rt::v1::Argument {
                                                                 position:
                                                                 ::std::fmt::rt::v1::Position::At(0usize),
                                                                 format:
                                                                 ::std::fmt::rt::v1::FormatSpec {
                                                                     fill:
                                                                     ' ',
                                                                     align:
                                                                     ::std::fmt::rt::v1::Alignment::Unknown,
                                                                     flags:
                                                                     0u32,
                                                                     precision:
                                                                     ::std::fmt::rt::v1::Count::Is(3usize),
                                                                     width:
                                                                     ::std::fmt::rt::v1::Count::Implied,
                                                                 },
                                                             }]));
    };
}

#[structural_match]
struct Deriv {
    x: usize,
}

#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for Deriv {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Deriv { x: ref __self_0_0 } => {
                let mut debug_trait_builder = f.debug_struct("Deriv");
                let _ = debug_trait_builder.field("x", &&(*__self_0_0));
                debug_trait_builder.finish()
            }
        }
    }
}

#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::Eq for Deriv {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        { let _: ::std::cmp::AssertParamIsEq<usize>; }
    }
}

#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for Deriv {
    #[inline]
    fn eq(&self, other: &Deriv) -> bool {
        match *other {
            Deriv { x: ref __self_1_0 } =>
                match *self {
                    Deriv { x: ref __self_0_0 } => (*__self_0_0) == (*__self_1_0),
                },
        }
    }
    #[inline]
    fn ne(&self, other: &Deriv) -> bool {
        match *other {
            Deriv { x: ref __self_1_0 } =>
                match *self {
                    Deriv { x: ref __self_0_0 } => (*__self_0_0) != (*__self_1_0),
                },
        }
    }
}

#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::Ord for Deriv {
    #[inline]
    fn cmp(&self, other: &Deriv) -> ::std::cmp::Ordering {
        match *other {
            Deriv { x: ref __self_1_0 } =>
                match *self {
                    Deriv { x: ref __self_0_0 } =>
                        match ::std::cmp::Ord::cmp(&(*__self_0_0), &(*__self_1_0)) {
                            ::std::cmp::Ordering::Equal =>
                                ::std::cmp::Ordering::Equal,
                            cmp => cmp,
                        },
                },
        }
    }
}

#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialOrd for Deriv {
    #[inline]
    fn partial_cmp(&self, other: &Deriv)
                   -> ::std::option::Option<::std::cmp::Ordering> {
        match *other {
            Deriv { x: ref __self_1_0 } =>
                match *self {
                    Deriv { x: ref __self_0_0 } =>
                        match ::std::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                  &(*__self_1_0)) {
                            ::std::option::Option::Some(::std::cmp::Ordering::Equal)
                            =>
                                ::std::option::Option::Some(::std::cmp::Ordering::Equal),
                            cmp => cmp,
                        },
                },
        }
    }
    #[inline]
    fn lt(&self, other: &Deriv) -> bool {
        match *other {
            Deriv { x: ref __self_1_0 } =>
                match *self {
                    Deriv { x: ref __self_0_0 } =>
                        ::std::option::Option::unwrap_or(::std::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                             &(*__self_1_0)),
                                                         ::std::cmp::Ordering::Greater)
                            == ::std::cmp::Ordering::Less,
                },
        }
    }
    #[inline]
    fn le(&self, other: &Deriv) -> bool {
        match *other {
            Deriv { x: ref __self_1_0 } =>
                match *self {
                    Deriv { x: ref __self_0_0 } =>
                        ::std::option::Option::unwrap_or(::std::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                             &(*__self_1_0)),
                                                         ::std::cmp::Ordering::Greater)
                            != ::std::cmp::Ordering::Greater,
                },
        }
    }
    #[inline]
    fn gt(&self, other: &Deriv) -> bool {
        match *other {
            Deriv { x: ref __self_1_0 } =>
                match *self {
                    Deriv { x: ref __self_0_0 } =>
                        ::std::option::Option::unwrap_or(::std::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                             &(*__self_1_0)),
                                                         ::std::cmp::Ordering::Less)
                            == ::std::cmp::Ordering::Greater,
                },
        }
    }
    #[inline]
    fn ge(&self, other: &Deriv) -> bool {
        match *other {
            Deriv { x: ref __self_1_0 } =>
                match *self {
                    Deriv { x: ref __self_0_0 } =>
                        ::std::option::Option::unwrap_or(::std::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                             &(*__self_1_0)),
                                                         ::std::cmp::Ordering::Less)
                            != ::std::cmp::Ordering::Less,
                },
        }
    }
}

#[structural_match]
struct Manual {
    x: usize,
}

#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for Manual {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Manual { x: ref __self_0_0 } => {
                let mut debug_trait_builder = f.debug_struct("Manual");
                let _ = debug_trait_builder.field("x", &&(*__self_0_0));
                debug_trait_builder.finish()
            }
        }
    }
}

#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::Eq for Manual {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        { let _: ::std::cmp::AssertParamIsEq<usize>; }
    }
}

#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for Manual {
    #[inline]
    fn eq(&self, other: &Manual) -> bool {
        match *other {
            Manual { x: ref __self_1_0 } =>
                match *self {
                    Manual { x: ref __self_0_0 } =>
                        (*__self_0_0) == (*__self_1_0),
                },
        }
    }
    #[inline]
    fn ne(&self, other: &Manual) -> bool {
        match *other {
            Manual { x: ref __self_1_0 } =>
                match *self {
                    Manual { x: ref __self_0_0 } =>
                        (*__self_0_0) != (*__self_1_0),
                },
        }
    }
}

impl PartialOrd for Manual {
    #[inline]
    fn partial_cmp(&self, other: &Manual) -> Option<Ordering> {
        self.x.partial_cmp(&other.x)
    }
}

impl Ord for Manual {
    #[inline]
    fn cmp(&self, other: &Manual) -> Ordering { self.x.cmp(&other.x) }
}

fn bubble_sort<T: Ord + Debug>(values: &mut [T]) {
    {
        ::io::_print(::std::fmt::Arguments::new_v1_formatted(&["", "\n"],
                                                             &match (&values[0], )
                                                                 {
                                                                     (arg0, ) =>
                                                                         [::std::fmt::ArgumentV1::new(arg0,
                                                                                                      ::std::fmt::Debug::fmt)],
                                                                 },
                                                             &[::std::fmt::rt::v1::Argument {
                                                                 position:
                                                                 ::std::fmt::rt::v1::Position::At(0usize),
                                                                 format:
                                                                 ::std::fmt::rt::v1::FormatSpec {
                                                                     fill:
                                                                     ' ',
                                                                     align:
                                                                     ::std::fmt::rt::v1::Alignment::Unknown,
                                                                     flags:
                                                                     0u32,
                                                                     precision:
                                                                     ::std::fmt::rt::v1::Count::Implied,
                                                                     width:
                                                                     ::std::fmt::rt::v1::Count::Implied,
                                                                 },
                                                             }]));
    };
    let mut n = values.len();
    let mut swapped = true;
    while swapped {
        swapped = false;
        for i in 1..n {
            if values[i - 1] > values[i] {
                values.swap(i - 1, i);
                swapped = true;
            }
        }
        n = n - 1;
    }
}
