/// Common handling used by all the assertion-related macros
#[cfg(not(feature="no-antithesis-sdk"))]
#[doc(hidden)]
#[macro_export]
macro_rules! assert_helper {
    // The handling of this pattern-arm of assert_helper
    // is wrapped in a block {} to avoid name collisions
    (condition = $condition:expr, $message:literal, $details:expr, $assert_type:path, $display_type:literal, must_hit = $must_hit:literal) => {{
        // Force evaluation of expressions.
        let condition = $condition;
        let details = $details;

        // Define a do-nothing function 'f()' within the context of 
        // the function invoking an assertion.  Then the type_name of 
        // this do-nothing will be something like:
        //
        //     bincrate::binmod::do_stuff::f
        //
        // After trimming off the last three chars `::f` what remains is 
        // the full path to the name of the function invoking the assertion
        //
        // Both the untrimmed `NAME` and trimmed `FUN_NAME` are lazily 
        // initialized statics so that `FUN_NAME` can be available at 
        // assertion catalog registration time.
        use $crate::once_cell::sync::Lazy;
        fn f(){}
        fn type_name_of<T>(_: T) -> &'static str {
            ::std::any::type_name::<T>()
        }
        static NAME: Lazy<&'static str> = Lazy::new(|| type_name_of(f));
        static FUN_NAME: Lazy<&'static str> = Lazy::new(|| &NAME[..NAME.len() - 3]);

        #[$crate::linkme::distributed_slice($crate::assert::ANTITHESIS_CATALOG)]
        #[linkme(crate = $crate::linkme)] // Refer to our re-exported linkme.
        static ALWAYS_CATALOG_ITEM: $crate::assert::CatalogInfo = $crate::assert::CatalogInfo{
            assert_type: $assert_type,
            display_type: $display_type,
            condition: false,
            message: $message,
            class: ::std::module_path!(),
            function: &FUN_NAME, /* function: &Lazy<&str> */
            file: ::std::file!(),
            begin_line: ::std::line!(),
            begin_column: ::std::column!(),
            must_hit: $must_hit,
            id: $message
        };

        let ptr_function = Lazy::force(&FUN_NAME);

        $crate::assert::assert_impl(
            $assert_type, /* assert_type */ 
            $display_type.to_owned(), /* display_type */ 
            condition, /* condition */
            $message.to_owned(), /* message */
            ::std::module_path!().to_owned(), /* class */
            String::from(*ptr_function), /* function */
            ::std::file!().to_owned(), /* file */ 
            ::std::line!(), /* line */
            ::std::column!(), /* column */
            true,/* hit */ 
            $must_hit, /* must-hit */ 
            $message.to_owned(), /* id */ 
            details /* details */
        )
    }} // end pattern-arm block
}
#[cfg(feature="no-antithesis-sdk")]
#[doc(hidden)]
#[macro_export]
macro_rules! assert_helper {
    (condition = $condition:expr, $message:literal, $details:expr, $assert_type:path, $display_type:literal, must_hit = $must_hit:literal) => {{
        // Force evaluation of expressions, ensuring that
        // any side effects of these expressions will always be
        // evaluated at runtime - even if the assertion itself 
        // is supressed by the `no-antithesis-sdk` feature
        let condition = $condition;
        let details = $details;
    }}
}

/// Assert that condition is true every time this function is called, AND that it is 
/// called at least once. This test property will be viewable in the "Antithesis SDK: Always" 
/// group of your triage report.
#[macro_export]
macro_rules! assert_always {
    ($condition:expr, $message:literal, $details:expr) => {
        $crate::assert_helper!(condition = $condition, $message, $details, AssertType::Always, "Always", must_hit = true)
    }
}

/// Assert that condition is true every time this function is called. Unlike the Always 
/// function, the test property spawned by AlwaysOrUnreachable will not be marked as 
/// failing if the function is never invoked. This test property will be viewable in 
/// the "Antithesis SDK: Always" group of your triage report.
#[macro_export]
macro_rules! assert_always_or_unreachable {
    ($condition:expr, $message:literal, $details:expr) => {
        $crate::assert_helper!(condition = $condition, $message, $details, AssertType::Always, "AlwaysOrUnreachable", must_hit = false)
    }
}

/// Assert that condition is true at least one time that this function was called. 
/// The test property spawned by Sometimes will be marked as failing if this function 
/// is never called, or if condition is false every time that it is called. This 
/// test property will be viewable in the "Antithesis SDK: Sometimes" group.
#[macro_export]
macro_rules! assert_sometimes {
    ($condition:expr, $message:literal, $details:expr) => {
        $crate::assert_helper!(condition = $condition, $message, $details, AssertType::Sometimes, "Sometimes", must_hit = true)
    }
}

/// Assert that a line of code is reached at least once. The test property spawned by 
/// Reachable will be marked as failing if this function is never called. This test 
/// property will be viewable in the "Antithesis SDK: Reachablity assertions" group.
#[macro_export]
macro_rules! assert_reachable {
    ($message:literal, $details:expr) => {
        $crate::assert_helper!(condition = true, $message, $details, AssertType::Reachability, "Reachable", must_hit = true)
    }
}

/// Assert that a line of code is never reached. The test property spawned by Unreachable 
/// will be marked as failing if this function is ever called. This test property will 
/// be viewable in the "Antithesis SDK: Reachablity assertions" group.
#[macro_export]
macro_rules! assert_unreachable {
    ($message:literal, $details:expr) => {
        $crate::assert_helper!(condition = false, $message, $details, AssertType::Reachability, "Unreachable", must_hit = false)
    }
}

