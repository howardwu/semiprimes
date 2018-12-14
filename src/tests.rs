#[cfg(test)]
mod test {
    use crate::biginteger::BigInteger;
    use crate::field::Fp;
    use rand;
    use rand::Rand;

    fn biginteger_arithmetic_test(a: BigInteger, b: BigInteger, zero: BigInteger) {
        // zero == zero
        assert_eq!(zero, zero);

        // zero.is_zero() == true
        assert_eq!(zero.is_zero(), true);

        // a == a
        assert_eq!(a, a);

        // a + 0 = a
        let mut a0_add = a.clone();
        a0_add.add_nocarry(&zero);
        assert_eq!(a0_add, a);

        // a - 0 = a
        let mut a0_sub = a.clone();
        a0_sub.sub_noborrow(&zero);
        assert_eq!(a0_sub, a);

        // a - a = 0
        let mut aa_sub = a.clone();
        aa_sub.sub_noborrow(&a);
        assert_eq!(aa_sub, zero);

        // a + b = b + a
        let mut ab_add = a.clone();
        ab_add.add_nocarry(&b);
        let mut ba_add = b.clone();
        ba_add.add_nocarry(&a);
        assert_eq!(ab_add, ba_add);
    }

    pub fn field_test(a: Fp, b: Fp) {
        let zero = Fp::zero();
        assert_eq!(zero, zero);
        assert_eq!(zero.is_zero(), true);
        assert_eq!(zero.is_one(), false);

        let one = Fp::one();
        assert_eq!(one, one);
        assert_eq!(one.is_zero(), false);
        assert_eq!(one.is_one(), true);
        assert_eq!(zero + one, one);

        let two = one + one;
        assert_eq!(two, two);
        assert_ne!(zero, two);
        assert_ne!(one, two);

        // a == a
        assert_eq!(a, a);
        // a + 0 = a
        assert_eq!(a + zero, a);
        // a - 0 = a
        assert_eq!(a - zero, a);
        // a - a = 0
        assert_eq!(a - a, zero);
        // 0 - a = -a
        assert_eq!(zero - a, -a);
        // a.double() = a + a
        assert_eq!(a.double(), a + a);
        // b.double() = b + b
        assert_eq!(b.double(), b + b);
        // a + b = b + a
        assert_eq!(a + b, b + a);
        // a - b = -(b - a)
        assert_eq!(a - b, -(b - a));
        // (a + b) + a = a + (b + a)
        assert_eq!((a + b) + a, a + (b + a));
        // (a + b).double() = (a + b) + (b + a)
        assert_eq!((a + b).double(), (a + b) + (b + a));

        // a * 0 = 0
        assert_eq!(a * zero, zero);
        // a * 1 = a
        assert_eq!(a * one, a);
        // a * 2 = a.double()
        assert_eq!(a * two, a.double());
        // a * a^-1 = 1
        assert_eq!(a * a.inverse().unwrap(), one);
        // a * a = a^2
        assert_eq!(a * a, a.square());
        // a * a * a = a^3
        assert_eq!(a * (a * a), a.pow([0x3, 0x0, 0x0, 0x0]));
        // a * b = b * a
        assert_eq!(a * b, b * a);
        // (a * b) * a = a * (b * a)
        assert_eq!((a * b) * a, a * (b * a));
        // (a + b)^2 = a^2 + 2ab + b^2
        assert_eq!(
            (a + b).square(),
            a.square() + ((a * b) + (a * b)) + b.square()
        );
        // (a - b)^2 = (-(b - a))^2
        assert_eq!((a - b).square(), (-(b - a)).square());
    }

    #[test]
    fn test_biginteger() {
        let a: BigInteger = BigInteger::rand(&mut rand::thread_rng());
        let b: BigInteger = BigInteger::rand(&mut rand::thread_rng());
        let zero = BigInteger::new([0u64; 33]);
        biginteger_arithmetic_test(a, b, zero);
    }

    #[test]
    fn test_fp() {
        let a: Fp = Fp::rand(&mut rand::thread_rng());
        let b: Fp = Fp::rand(&mut rand::thread_rng());
        field_test(a, b);
    }
}
