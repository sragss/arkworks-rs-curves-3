Derived from V4:
```
use ark_curve25519::{Fr, FrConfig};
use ark_ff::{FpConfig, MontBackend, MontConfig, PrimeField, BigInt};

type Mont25519 = MontBackend<FrConfig, 4>;

fn main() {
    println!("MODULUS {:?}", hexify(FrConfig::MODULUS));
    println!("R {:?}", hexify(FrConfig::R));
    println!("R2 {:?}", hexify(FrConfig::R2));
    println!("MODULUS_MINUS_ONE_DIV_TWO {:?}", hexify(<Fr as PrimeField>::MODULUS_MINUS_ONE_DIV_TWO));
    println!("T {:?}", hexify(<Fr as PrimeField>::TRACE));
    println!("T_MINUS_ONE_DIV_TWO {:?}", hexify(<Fr as PrimeField>::TRACE_MINUS_ONE_DIV_TWO));
    println!("GENERATOR {:?}", hexify(FrConfig::GENERATOR.into()));
    println!("GENERATOR {:?}", FrConfig::GENERATOR);
    println!("MOD BITS {:?}", <Fr as PrimeField>::MODULUS_BIT_SIZE);
    println!("INVERSE {:?}", hexify(BigInt::<4>::from(FrConfig::INV)));
    println!("INVERSE {:?}", FrConfig::INV);
    println!("THATS ALL OF THEM");

    println!("TWO_ADACITY {:?}", Mont25519::TWO_ADICITY);
    println!("TWO_ADIC_ROOT_OF_UNITY {:?}", hexify(FrConfig::TWO_ADIC_ROOT_OF_UNITY.into()));
    println!("{:?}", FrConfig::SMALL_SUBGROUP_BASE);
    println!("{:?}", FrConfig::SMALL_SUBGROUP_BASE_ADICITY);
    println!("{:?}", FrConfig::LARGE_SUBGROUP_ROOT_OF_UNITY);
}

fn hexify(big: BigInt<4>) -> Vec<String> {
    big.0.iter().map(|num| format!("0x{:x}", num)).collect()
}
```

And 
```
fn main() {
    println!("MODULUS {:?}", hexify(FqConfig::MODULUS));
    println!("R {:?}", hexify(FqConfig::R));
    println!("R2 {:?}", hexify(FqConfig::R2));
    println!("MODULUS_MINUS_ONE_DIV_TWO {:?}", hexify(<Fq as PrimeField>::MODULUS_MINUS_ONE_DIV_TWO));
    println!("T {:?}", hexify(<Fq as PrimeField>::TRACE));
    println!("T_MINUS_ONE_DIV_TWO {:?}", hexify(<Fq as PrimeField>::TRACE_MINUS_ONE_DIV_TWO));
    println!("GENERATOR {:?}", hexify(FqConfig::GENERATOR.into()));
    println!("GENERATOR {:?}", FqConfig::GENERATOR);
    println!("MOD BITS {:?}", <Fq as PrimeField>::MODULUS_BIT_SIZE);
    println!("INVERSE {:?}", hexify(BigInt::<4>::from(FqConfig::INV)));
    println!("INVERSE {:?}", FqConfig::INV);
    println!("THATS ALL OF THEM");

    println!("TWO_ADACITY {:?}", MontFq25519::TWO_ADICITY);
    println!("TWO_ADIC_ROOT_OF_UNITY {:?}", hexify(FqConfig::TWO_ADIC_ROOT_OF_UNITY.into()));
}
```