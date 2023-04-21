use ark_ff::{biginteger::BigInteger256 as BigInteger, Fp256Parameters, fields::FftParameters, FpParameters};

// #[derive(MontConfig)]
// #[modulus = "7237005577332262213973186563042994240857116359379907606001950938285454250989"]
// #[generator = "2"]
// #[small_subgroup_base = "3"]
// #[small_subgroup_power = "1"]
// pub struct FrConfig;
// pub type Fr = Fp256<MontBackend<FrConfig, 4>>;

pub struct FrParameters;
impl Fp256Parameters for FrParameters {}

impl FftParameters for FrParameters {
    type BigInt = BigInteger;

    const TWO_ADICITY: u32 = 2;

    // TWO_ADIC_ROOT_OF_UNITY = GENERATOR^T
    // Encoded in Montgomery form, so the value here is (5^T)R mod q.
    const TWO_ADIC_ROOT_OF_UNITY: BigInteger = BigInteger([
        0xbe8775dfebbe07d4,
        0xef0565342ce83fe,
        0x7d3d6d60abc1c27a,
        0x94a7310e07981e7,
    ]);
}

impl FpParameters for FrParameters {
        // 28948022309329048855892746252171976963363056481941647379679742748393362948097
        const MODULUS: BigInteger = BigInteger([
            0x5812631A5CF5D3ED,
            0x14DEF9DEA2F79CD6,
            0x0000000000000000,
            0x1000000000000000,
        ]);
    
        // R = 2^256 mod q
        const R: BigInteger = BigInteger([
            0xd6ec31748d98951d,
            0xc6ef5bf4737dcf70,
            0xfffffffffffffffe,
            0xfffffffffffffff,
        ]);
    
        // R2 = (2^256)^2 mod q
        const R2: BigInteger = BigInteger([
            0xa40611e3449c0f01,
            0xd00e1ba768859347,
            0xceec73d217f5be65,
            0x399411b7c309a3d,
        ]);
    
        const MODULUS_MINUS_ONE_DIV_TWO: BigInteger = BigInteger([
            0x2c09318d2e7ae9f6,
            0xa6f7cef517bce6b,
            0x0000000000000000,
            0x800000000000000,
        ]);
    
        // T and T_MINUS_ONE_DIV_TWO, where MODULUS - 1 = 2^S * T
    
        const T: BigInteger = BigInteger([
            0x960498c6973d74fb,
            0x537be77a8bde735,
            0x0000000000000000,
            0x400000000000000,
        ]);
    
        const T_MINUS_ONE_DIV_TWO: BigInteger = BigInteger([
            0xcb024c634b9eba7d,
            0x29bdf3bd45ef39a,
            0x0000000000000000,
            0x200000000000000,
        ]);
    
        // Encoded in Montgomery form, so the value here is 5R mod q.
        const GENERATOR: BigInteger = BigInteger([
            0x2,
            0x0,
            0x0,
            0x0,
        ]);
    
        const MODULUS_BITS: u32 = 253;
    
        const CAPACITY: u32 = Self::MODULUS_BITS - 1;
    
        const REPR_SHAVE_BITS: u32 = 1;
    
        // INV = -q^{-1} (mod 2^64)
        const INV: u64 = 15183074304973897243;
}
