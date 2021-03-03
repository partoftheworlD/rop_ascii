#[cfg(test)]
use crate::io;

macro_rules! test_multiplicity {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let mut flag: bool = false;
            io::IO.check(&$value, &mut flag);
            assert_eq!(flag, false);
        }
    )*
    }
}

test_multiplicity! {
    test_0: "0",
    test_1: "12",
    test_2: "123",
    test_3: "1111",
    test_4: "11111",
    test_5: "111111",
    test_6: "1111111",
    test_7: "",
    test_8: "0xAABBCCDD",
    test_9: "0x0",
    test_10: "\n",
    test_11: "\t",
    test_12: "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
    test_13: "01010203",
    test_14: "414141414141",
}