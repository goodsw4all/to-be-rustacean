#[macro_export]
macro_rules! hashmap {
    () => {
        {
//let _empty: ::std::collections::HashMap<(), ()> = hashmap!();
// |                                                       ^^^^^^^^^^ expected struct
//`std::collections::HashMap`, found struct `test_type_override::std::collections::HashMap`
            ::std::collections::HashMap::new()
        }
    };
    ($($k:expr => $v:expr),*) => {
        {
            let mut _map = ::std::collections::HashMap::new();
            $(_map.insert($k, $v);)*
            _map
        }
    };
    ($($k:expr => $v:expr),+ $(,)?) => {
        {
            let mut _map = ::std::collections::HashMap::new();
            $(_map.insert($k, $v);)*
            _map
        }
    };
}

#[test]
fn test_macro_hashmap() {
    let mut expected = ::std::collections::HashMap::new();
    expected.insert(1, "one");
    expected.insert(2, "two");
    assert_eq!(hashmap!(1 => "one", 2 => "two"), expected);
}
