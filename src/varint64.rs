use crate::Varint;

impl Varint for u64 {
    fn read_varint(bytes: &[u8]) -> Option<(u64, usize)> {
        type ReturnType = u64;
        let mut ret = 0 as ReturnType;
        let mut shift = 0;
        let nb_bytes_to_read = Self::msb_detect(&bytes);

        for byte in bytes[0..nb_bytes_to_read].iter() {
            ret |= ((byte & 127) as ReturnType) << shift;
            shift += 7;
        }

        Some((ret, nb_bytes_to_read))
    }

    //TODO: Implement
    fn write_varint(bytes: &mut [u8], value: u64) -> usize {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_write() {}

    #[test]
    fn should_read() {
        let mut data_input: Vec<u8> = Vec::new();
        data_input.push(0b1010_1100);
        data_input.push(0b0000_0010);
        data_input.push(0b0111_1111);
        data_input.push(0b1111_1111);
        let mut data_expected: Vec<Option<u64>> = Vec::new();
        data_expected.push(Some(0u64));
        let result = u64::read_varint(&data_input);
        assert_eq!(result.unwrap().0, 300);
        assert_eq!(result.unwrap().1, 2);
    }
}
