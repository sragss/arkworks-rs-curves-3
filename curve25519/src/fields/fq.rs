use ark_ff::{biginteger::BigInteger256 as BigInteger, Fp256Parameters, fields::FftParameters, fields::Fp256, FpParameters};

// use ark_ff::fields::{Fp256, MontBackend, MontConfig};

// #[derive(MontConfig)]
// #[modulus = "57896044618658097711785492504343953926634992332820282019728792003956564819949"]
// #[generator = "2"]
// #[small_subgroup_base = "3"]
// #[small_subgroup_power = "1"]
// pub struct FqConfig;
// pub type Fq = Fp256<MontBackend<FqConfig, 4>>;

pub type Fq = Fp256<FqParameters>;

pub struct FqParameters;
impl Fp256Parameters for FqParameters {}

impl FftParameters for FqParameters {
    type BigInt = BigInteger;

    const TWO_ADICITY: u32 = 2;

    // TWO_ADIC_ROOT_OF_UNITY = GENERATOR^T
    // Encoded in Montgomery form, so the value here is (5^T)R mod q.
    const TWO_ADIC_ROOT_OF_UNITY: BigInteger = BigInteger([
        0xc4ee1b274a0ea0b0,
        0x2f431806ad2fe478,
        0x2b4d00993dfbd7a7,
        0x2b8324804fc1df0b,
    ]);
}

impl FpParameters for FqParameters {
        // 28948022309329048855892746252171976963363056481941647379679742748393362948097
        const MODULUS: BigInteger = BigInteger([
            0xffffffffffffffed,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0x7fffffffffffffff,
        ]);
    
        // R = 2^256 mod q
        const R: BigInteger = BigInteger([
            0x26,
            0x0,
            0x0,
            0x0,
        ]);
    
        // R2 = (2^256)^2 mod q
        const R2: BigInteger = BigInteger([
            0x5a4,
            0x0,
            0x0,
            0x0,
        ]);
    
        const MODULUS_MINUS_ONE_DIV_TWO: BigInteger = BigInteger([
            0xfffffffffffffff6,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0x3fffffffffffffff,
        ]);
    
        // T and T_MINUS_ONE_DIV_TWO, where MODULUS - 1 = 2^S * T
    
        const T: BigInteger = BigInteger([
            0xfffffffffffffffb,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0x1fffffffffffffff,
        ]);
    
        const T_MINUS_ONE_DIV_TWO: BigInteger = BigInteger([
            0xfffffffffffffffd,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xfffffffffffffff,
        ]);
    
        // Encoded in Montgomery form, so the value here is 5R mod q.
        const GENERATOR: BigInteger = BigInteger([
            0x2,
            0x0,
            0x0,
            0x0,
        ]);
    
        const MODULUS_BITS: u32 = 255;
    
        const CAPACITY: u32 = Self::MODULUS_BITS - 1;
    
        const REPR_SHAVE_BITS: u32 = 1;
    
        // INV = -q^{-1} (mod 2^64)
        const INV: u64 = 9708812670373448219;
}
