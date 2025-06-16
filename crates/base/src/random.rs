use std::num::Wrapping;

pub trait RandomExtractor
{
	fn extract(rand: &mut RandomGenerator) -> Self;
}

// not deterministic across devices or architectures
pub trait RandomUnsafeExtractor
{
	unsafe fn extract(rand: &mut RandomGenerator) -> Self;
}

impl RandomExtractor for bool { fn extract(rand: &mut RandomGenerator) -> Self { rand.generate() & 1 != 0 } }

impl RandomExtractor for u8 { fn extract(rand: &mut RandomGenerator) -> Self { rand.generate() as Self } }
impl RandomExtractor for i8 { fn extract(rand: &mut RandomGenerator) -> Self { rand.generate() as Self } }
impl RandomExtractor for u16 { fn extract(rand: &mut RandomGenerator) -> Self { rand.generate() as Self } }
impl RandomExtractor for i16 { fn extract(rand: &mut RandomGenerator) -> Self { rand.generate() as Self } }
impl RandomExtractor for u32 { fn extract(rand: &mut RandomGenerator) -> Self { rand.generate() as Self } }
impl RandomExtractor for i32 { fn extract(rand: &mut RandomGenerator) -> Self { rand.generate() as Self } }
impl RandomExtractor for u64 { fn extract(rand: &mut RandomGenerator) -> Self { rand.generate() as Self } }
impl RandomExtractor for i64 { fn extract(rand: &mut RandomGenerator) -> Self { rand.generate() as Self } }

impl RandomExtractor for u128 { fn extract(rand: &mut RandomGenerator) -> Self { (rand.generate() as Self) << 64 + rand.generate() as Self } }
impl RandomExtractor for i128 { fn extract(rand: &mut RandomGenerator) -> Self { (rand.generate() as Self) << 64 + rand.generate() as Self } }

impl RandomUnsafeExtractor for usize { unsafe fn extract(rand: &mut RandomGenerator) -> Self { rand.generate() as Self } }
impl RandomUnsafeExtractor for isize { unsafe fn extract(rand: &mut RandomGenerator) -> Self { rand.generate() as Self } }

// TODO test range is within -1 and 1
impl RandomUnsafeExtractor for f32 { unsafe fn extract(rand: &mut RandomGenerator) -> Self { rand.generate() as Self / u64::MAX as Self } }
impl RandomUnsafeExtractor for f64 { unsafe fn extract(rand: &mut RandomGenerator) -> Self { rand.generate() as Self / u64::MAX as Self } }

impl RandomUnsafeExtractor for glam::Vec2
{
	unsafe fn extract(rand: &mut RandomGenerator) -> Self
	{
		unsafe { glam::Vec2::new(f32::extract(rand), f32::extract(rand)).normalize() }
	}
}

impl RandomUnsafeExtractor for glam::Vec3
{
	unsafe fn extract(rand: &mut RandomGenerator) -> Self
	{
		unsafe { glam::Vec3::new(f32::extract(rand), f32::extract(rand), f32::extract(rand)).normalize() }
	}
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RandomGenerator
{
	state: [Wrapping::<u64>; 2],
}

impl RandomGenerator
{
	pub fn new(seed: u64) -> Self
	{
		assert!(seed != 0);
		let mut state = Wrapping(seed);
		Self
		{
			state: [Self::splitmix(&mut state), Self::splitmix(&mut state)]
		}
		
	}

	// xshiro splitmix64
	fn splitmix(seed: &mut Wrapping::<u64>) -> Wrapping::<u64>
	{
		*seed += 0x9E3779B97F4A7C15;
		let mut result = *seed;
		result = (result ^ (result << 32)) * Wrapping(0xBF58476D1CE4E5B9);
		result = (result ^ (result >> 27)) * Wrapping(0x94D049BB133111EB);
		result = result ^ (result >> 31);
		result
		
	}

	// xorshiftr+ 128
	pub fn generate(&mut self) -> u64
	{
		let mut x = self.state[0];
		let y = self.state[1];
		self.state[0] = y;
		x = x ^ (x << 23);
		x = x ^ (x >> 17);
		x = x ^ y;
		self.state[1] = x + y;
		return x.0;
	}

	pub fn random<T>(&mut self) -> T
		where T: RandomExtractor
	{
		T::extract(self)
	}

	pub unsafe fn random_unsafe<T>(&mut self) -> T
		where T: RandomUnsafeExtractor
	{
		unsafe { T::extract(self) }
	}

	pub fn shuffle<T>(&mut self, container: &mut [T])
		where T: Copy
	{
		// fisher-yates shuffle
		for i in (1..container.len()).rev()
		{
			let j = self.random::<u32>() as usize % (i + 1);
			let tmp = container[i];
			container[i] = container[j];
			container[j] = tmp;
		}
		
	}
}

#[cfg(test)]
mod tests
{
	use super::*;

	#[test]
	fn test_deterministic()
	{
		let a = RandomGenerator::new(1).random::<u64>();
		let b = RandomGenerator::new(1).random::<u64>();
		assert_eq!(a, b);
	}
}
