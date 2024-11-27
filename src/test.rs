use std::{
    fmt::{Display, Write},
    ops::{Deref, DerefMut},
};

mod mandatory;

#[cfg(feature = "bonus")]
mod bonus;

///
/// An object that will write to stderr the C function call that led to the
/// error
///
struct PrintFunctionOnDrop<'a> {
    should_print: bool,
    function_name: &'a str,
    parameters: Vec<Box<dyn Display + 'a>>,
}

impl<'a> PrintFunctionOnDrop<'a> {
    pub fn interrupt(mut self) {
        self.should_print = false;
    }
    pub fn new(function_name: &'a str, parameters: Vec<Box<dyn Display + 'a>>) -> Self {
        Self {
            should_print: true,
            function_name,
            parameters,
        }
    }
}

impl Drop for PrintFunctionOnDrop<'_> {
    fn drop(&mut self) {
        if !self.should_print {
            return;
        }
        eprintln!("{}(", self.function_name);
        for param in &self.parameters {
            let mut string = String::new();
            if write!(string, "{param}").is_ok() {
                let string: String = string
                    .chars()
                    .flat_map(|x| {
                        if x.is_ascii() && !x.is_ascii_control() {
                            // forced to do this kind of thing so that the return value is the same
                            std::iter::repeat_with(Box::new(move || x) as Box<dyn FnMut() -> char>)
                                .take(1)
                                .chain(std::iter::repeat_n('x', 0))
                                .chain(x.escape_unicode().skip(1).take(0))
                        } else {
                            // skips \u{
                            let skipped_begin = x.escape_unicode().skip(3);
                            let skipped_len = skipped_begin.len();
                            // take up to }
                            let without_end = skipped_begin.take(skipped_len - 1);

                            let identifier_len = without_end.len();

                            let mut i: i8 = -1;

                            std::iter::repeat_with(Box::new(move || {
                                i += 1;
                                match i {
                                    0 => '\\',
                                    1 => 'U',
                                    _ => panic!(),
                                }
                            })
                                as Box<dyn FnMut() -> char>)
                            .take(2)
                            .chain(std::iter::repeat_n('0', 8 - identifier_len))
                            .chain(without_end)
                        }
                    })
                    .collect();
                eprintln!("\t\"{string}\",");
            } else {
                eprintln!("couldn't print parameter")
            }
        }
        eprintln!(")");
    }
}

macro_rules! test {
    (
        $(#![test $test_name: literal => $($test_param: expr),+])*
        $function_name: ident ($($param: ident: $param_type: ty),*) $body: block
    ) => {
        // crate::fork_test! {
            $(#[test_case::test_case($($test_param),+ ; $test_name)])*
            fn test($($param: $param_type),*) {
                use $crate::test::PartialClone;
                let print_function_on_drop = $crate::test::PrintFunctionOnDrop::new(stringify!($function_name), vec![$(Box::new($param.partial_clone())),*]);
                {$body}
                print_function_on_drop.interrupt();
            }
        // }
    };
}
use test;

trait PartialClone {
    fn partial_clone(&self) -> Self;
}

impl<T: Clone> PartialClone for T {
    fn partial_clone(&self) -> Self {
        self.clone()
    }
}

///
/// This structure is a kind of Option interface, excepts it implements Display (printing "unpritable")
/// and clone (excepts it just returns a None)
///
struct Unprintable<T>(pub Option<T>);
impl<T> Deref for Unprintable<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.0.as_ref().unwrap()
    }
}
impl<T> DerefMut for Unprintable<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut().unwrap()
    }
}
impl<T> Display for Unprintable<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unprintable")
    }
}
impl<T> Unprintable<T> {
    pub fn unwrap(self) -> T {
        self.0.unwrap()
    }
}
impl<T> PartialClone for Unprintable<T> {
    fn partial_clone(&self) -> Self {
        Self(None)
    }
}

#[derive(Clone)]
struct DisplayableStringSlice<'a, T: Clone + Display>(&'a [T]);
impl<T: Clone + Display> Display for DisplayableStringSlice<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        if let Some((first, rest)) = self.0.split_first() {
            write!(f, "\"{first}\"")?;
            for x in rest {
                write!(f, ", \"{x}\"")?;
            }
        }
        write!(f, "]")
    }
}

#[derive(Clone)]
struct DisplayableSlice<'a, T: Clone + Display>(&'a [T]);
impl<T: Clone + Display> Display for DisplayableSlice<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        if let Some((first, rest)) = self.0.split_first() {
            write!(f, "{first}")?;
            for x in rest {
                write!(f, ", {x}")?;
            }
        }
        write!(f, "]")
    }
}
