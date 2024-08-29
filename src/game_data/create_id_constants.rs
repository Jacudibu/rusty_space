/// Creates _ID and _NAME fields for the given constant names.
///
/// # Example
/// ```
/// create_id_constants!(ItemId, ITEM_A);
///
/// assert_eq!(ITEM_A_NAME, "item_a");
/// assert_eq!(ITEM_A_ID, ItemId::from_name(ITEM_A_NAME);
/// ```
#[macro_export]
macro_rules! create_id_constants {
    ($type_name:ident $(, $name:ident )+) => {
        paste::paste! {
            $(
                const [<$name _NAME>]: &str = stringify!([<$name:lower>]);
                pub const [<$name _ID>]: $type_name = $type_name::from_name([<$name _NAME>]);
            )+
        }
    };
}
