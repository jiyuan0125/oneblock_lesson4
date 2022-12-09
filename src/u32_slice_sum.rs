pub fn u32_slice_sum(slice: &[u32]) -> Option<u32> {
    let mut sum = 0;
    for i in slice {
        if sum > u32::MAX - i {
            return None;
        }
        sum += i;
    }
    Some(sum)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_u32_slice_sum() {
        let slice = [1, 2, 3, 4, 5];
        assert_eq!(u32_slice_sum(&slice), Some(15));

        let slice = [u32::MAX, 1];
        assert_eq!(u32_slice_sum(&slice), None);
    }

}
