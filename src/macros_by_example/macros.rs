
#[macro_export]
macro_rules! with_element {
    ($variant:ident, $error:ident) => {
        struct $variant;

        impl<T> WithElement<T, $error<T>> for $variant {
            fn from_element(element: T) -> $error<T> {
                $error::$variant(element)
            }
        }
    }
}


#[macro_export]
macro_rules! error_only {
    ($variant:ident, $error:ident) => {
        struct $variant;

        impl<T> ErrorOnly<T, $error<T>> for $variant {
            fn new() -> $error<T> {
                $error::$variant
            }
        }
    }
}


#[macro_export]
macro_rules! validerr {
    ($error:ident {
        $(WithElement{$($we_variant:ident),+})?
        $(ErrorOnly{$($eo_variant:ident),+})?
    }) => {
        enum $error<T> {
            $($($we_variant(T),)+)?
            $($($eo_variant,)+)?
        }

        impl<T> ValidErr<T> for $error<T> {}

        $($(with_element!{$we_variant, $error})+)?
        $($(error_only!{$eo_variant, $error})+)?
    };

    ($error:ident {
        $(ErrorOnly{$($eo_variant:ident),+})?
        $(WithElement{$($we_variant:ident),+})?
    }) => {
        enum_gen!{
            $error {
                $(WithElement{$($we_variant),+})?
                $(ErrorOnly{$($eo_variant),+})?
            }
        }
    };
}


