use rand::{
    Rng, SeedableRng,
    distributions::uniform::{SampleRange, SampleUniform},
};

type RngCore = rand::prelude::StdRng;

pub struct RandomNumberGenerator {
    rng: RngCore,
}

impl RandomNumberGenerator {
    pub fn new() -> Self {
        Self {
            rng: RngCore::from_entropy(),
        }
    }

    pub fn seeded(seed: u64) -> Self {
        Self {
            rng: RngCore::seed_from_u64(seed),
        }
    }

    pub fn range<T>(&mut self, range: impl SampleRange<T>) -> T
    where
        T: SampleUniform + PartialOrd,
    {
        self.rng.gen_range(range)
    }

    #[allow(clippy::should_implement_trait)] // for coherence with the book
    pub fn next<T>(&mut self) -> T
    where
        rand::distributions::Standard: rand::prelude::Distribution<T>,
    {
        self.rng.r#gen()
    }
}

impl Default for RandomNumberGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_range_bounds() {
        let mut rng = RandomNumberGenerator::new();
        for _ in 0..1000 {
            let n = rng.range(1..10);
            assert!(1 <= n);
            assert!(n < 10);
        }
    }

    #[test]
    fn test_reproducibility() {
        let mut rng1 = RandomNumberGenerator::seeded(1);
        let mut rng2 = RandomNumberGenerator::seeded(1);
        (0..1000).for_each(|_| {
            assert_eq!(
                rng1.range(u32::MIN..u32::MAX),
                rng2.range(u32::MIN..u32::MAX)
            )
        });
    }

    #[test]
    fn test_next_types() {
        let mut rng = RandomNumberGenerator::new();
        let _: i32 = rng.next();
        let _ = rng.next::<f32>();
    }

    #[test]
    fn test_float() {
        let mut rng = RandomNumberGenerator::new();
        for _ in 0..1000 {
            let n = rng.range(-5000.0f32..5000.0f32);
            assert!(n.is_finite());
            assert!(n > -5000.);
            assert!(n < 5000.);
        }
    }
}
