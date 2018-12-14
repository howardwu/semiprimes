/// Calculate a + b + carry, returning the sum and modifying the carry value.
#[inline(always)]
pub(crate) fn adc(a: u64, b: u64, carry: &mut u64) -> u64 {
    let tmp = u128::from(a) + u128::from(b) + u128::from(*carry);

    *carry = (tmp >> 64) as u64;

    tmp as u64
}

/// Calculate a - b - borrow, returning the result and modifying the borrow value.
#[inline(always)]
pub(crate) fn sbb(a: u64, b: u64, borrow: &mut u64) -> u64 {
    let tmp = (1u128 << 64) + u128::from(a) - u128::from(b) - u128::from(*borrow);

    *borrow = if tmp >> 64 == 0 { 1 } else { 0 };

    tmp as u64
}

/// Calculate a + (b * c) + carry, returning the least significant digit
/// and setting carry to the most significant digit.
#[inline(always)]
pub(crate) fn mac_with_carry(a: u64, b: u64, c: u64, carry: &mut u64) -> u64 {
    let tmp = (u128::from(a)) + u128::from(b) * u128::from(c) + u128::from(*carry);

    *carry = (tmp >> 64) as u64;

    tmp as u64
}


#[derive(Debug)]
pub struct BitIterator<E> {
    t: E,
    n: usize,
}

impl<E: AsRef<[u64]>> BitIterator<E> {
    pub fn new(t: E) -> Self {
        let n = t.as_ref().len() * 64;

        BitIterator { t, n }
    }
}

impl<E: AsRef<[u64]>> Iterator for BitIterator<E> {
    type Item = bool;

    fn next(&mut self) -> Option<bool> {
        if self.n == 0 {
            None
        } else {
            self.n -= 1;
            let part = self.n / 64;
            let bit = self.n - (64 * part);

            Some(self.t.as_ref()[part] & (1 << bit) > 0)
        }
    }
}
