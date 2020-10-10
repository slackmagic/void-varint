pub const VARINT_32_MAX_BYTES: usize = 5;
pub const VARINT_64_MAX_BYTES: usize = 10;

pub mod varint32;
pub mod varint64;

trait Varint {
    fn read_varint(bytes: &[u8]) -> Option<(Self, usize)>
    where
        Self: Sized;

    fn write_varint(bytes: &mut [u8], value: u64) -> usize;

    fn msb_detect(bytes: &[u8]) -> usize {
        match &bytes[0] | 127 & 255 == 255 {
            true => 1 + Self::msb_detect(&bytes[1..]),
            false => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct DummyStruct;
    impl Varint for DummyStruct {
        fn read_varint(bytes: &[u8]) -> Option<(Self, usize)> {
            None
        }
        fn write_varint(bytes: &mut [u8], value: u64) -> usize {
            0
        }
    }

    #[test]
    fn should_detect_msb() {
        let mut data_input: Vec<u8> = Vec::new();
        data_input.push(0b0010_1100);
        data_input.push(0b0000_0010);
        data_input.push(0b1111_1111);
        assert_eq!(DummyStruct::msb_detect(&data_input), 1);

        let mut data_input: Vec<u8> = Vec::new();
        data_input.push(0b1010_1100);
        data_input.push(0b0000_0010);
        data_input.push(0b1111_1111);
        assert_eq!(DummyStruct::msb_detect(&data_input), 2);

        let mut data_input: Vec<u8> = Vec::new();
        data_input.push(0b1010_1100);
        data_input.push(0b1000_0010);
        data_input.push(0b0111_1111);
        data_input.push(0b1111_1111);
        assert_eq!(DummyStruct::msb_detect(&data_input), 3);

        let mut data_input: Vec<u8> = Vec::new();
        data_input.push(0b1010_1100);
        data_input.push(0b1000_0010);
        data_input.push(0b0111_1111);
        assert_eq!(DummyStruct::msb_detect(&data_input), 3);
    }
}
