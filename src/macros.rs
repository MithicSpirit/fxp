#[macro_export]
macro_rules! ifxp {
	($name:ident, $base:ty) => {

		#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Default, Debug)]
		pub struct $name<const EXPONENT: usize>($base);

		impl<const EXP: usize> $name<EXP>
		{
			fn precision<const EXPONENT: usize>(
				self,
			) -> $name<EXPONENT>
			{
				let diff = EXPONENT - EXP;
				$name::<EXPONENT>(self.0 >> diff)
			}
		}

		impl<const EXP1: usize, const EXP2: usize>
			ops::Add<$name<EXP2>> for $name<EXP1>
		where
			$name<{ max(EXP1, EXP2) }>: Sized,
		{
			type Output = $name<{ max(EXP1, EXP2) }>;

			fn add(self, rhs: $name<EXP2>) -> Self::Output
			{
				$name::<{ max(EXP1, EXP2) }>(
					self.precision::<{ max(EXP1, EXP2) }>().0
						+ rhs.precision::<{ max(EXP1, EXP2) }>().0,
				)
			}
		}

		impl<const EXP: usize> ops::AddAssign<$name<EXP>>
			for $name<EXP>
		{
			fn add_assign(&mut self, rhs: Self)
			{
				self.0 += rhs.0
			}
		}

		impl<const EXP1: usize, const EXP2: usize>
			ops::Div<$name<EXP2>> for $name<EXP1>
		where
			$name<{ EXP1 - EXP2 }>: Sized,
		{
			type Output = $name<{ EXP1 - EXP2 }>;

			fn div(self, rhs: $name<EXP2>) -> Self::Output
			{
				$name::<{ EXP1 - EXP2 }>(self.0 / rhs.0)
			}
		}

		impl<const EXP1: usize, const EXP2: usize>
			ops::Mul<$name<EXP2>> for $name<EXP1>
		where
			$name<{ EXP1 + EXP2 }>: Sized,
		{
			type Output = $name<{ EXP1 + EXP2 }>;

			fn mul(self, rhs: $name<EXP2>) -> Self::Output
			{
				$name::<{ EXP1 + EXP2 }>(self.0 * rhs.0)
			}
		}

		impl<const EXP: usize> ops::Neg for $name<EXP>
		{
			type Output = Self;

			fn neg(self) -> Self::Output
			{
				Self(-self.0)
			}
		}

		impl<const EXP1: usize, const EXP2: usize>
			ops::Sub<$name<EXP2>> for $name<EXP1>
		where
			$name<{ max(EXP1, EXP2) }>: Sized,
		{
			type Output = $name<{ max(EXP1, EXP2) }>;

			fn sub(self, rhs: $name<EXP2>) -> Self::Output
			{
				$name::<{ max(EXP1, EXP2) }>(
					self.precision::<{ max(EXP1, EXP2) }>().0
						- rhs.precision::<{ max(EXP1, EXP2) }>().0,
				)
			}
		}

		impl<const EXP: usize> ops::SubAssign<$name<EXP>>
			for $name<EXP>
		{
			fn sub_assign(&mut self, rhs: Self)
			{
				self.0 -= rhs.0
			}
		}
	};
}

#[macro_export]
macro_rules! ufxp {
	($name:ident, $base:ty) => {
		#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Default, Debug)]
		pub struct $name<const EXPONENT: usize>($base);

		impl<const EXP: usize> $name<EXP>
		{
			fn precision<const EXPONENT: usize>(
				self,
			) -> $name<EXPONENT>
			{
				let diff = EXPONENT - EXP;
				$name::<EXPONENT>(self.0 >> diff)
			}
		}

		impl<const EXP1: usize, const EXP2: usize>
			ops::Add<$name<EXP2>> for $name<EXP1>
		where
			$name<{ max(EXP1, EXP2) }>: Sized,
		{
			type Output = $name<{ max(EXP1, EXP2) }>;

			fn add(self, rhs: $name<EXP2>) -> Self::Output
			{
				$name::<{ max(EXP1, EXP2) }>(
					self.precision::<{ max(EXP1, EXP2) }>().0
						+ rhs.precision::<{ max(EXP1, EXP2) }>().0,
				)
			}
		}

		impl<const EXP: usize> ops::AddAssign<$name<EXP>>
			for $name<EXP>
		{
			fn add_assign(&mut self, rhs: Self)
			{
				self.0 += rhs.0
			}
		}

		impl<const EXP1: usize, const EXP2: usize>
			ops::Div<$name<EXP2>> for $name<EXP1>
		where
			$name<{ EXP1 - EXP2 }>: Sized,
		{
			type Output = $name<{ EXP1 - EXP2 }>;

			fn div(self, rhs: $name<EXP2>) -> Self::Output
			{
				$name::<{ EXP1 - EXP2 }>(self.0 / rhs.0)
			}
		}

		impl<const EXP1: usize, const EXP2: usize>
			ops::Mul<$name<EXP2>> for $name<EXP1>
		where
			$name<{ EXP1 + EXP2 }>: Sized,
		{
			type Output = $name<{ EXP1 + EXP2 }>;

			fn mul(self, rhs: $name<EXP2>) -> Self::Output
			{
				$name::<{ EXP1 + EXP2 }>(self.0 * rhs.0)
			}
		}

		impl<const EXP1: usize, const EXP2: usize>
			ops::Sub<$name<EXP2>> for $name<EXP1>
		where
			$name<{ max(EXP1, EXP2) }>: Sized,
		{
			type Output = $name<{ max(EXP1, EXP2) }>;

			fn sub(self, rhs: $name<EXP2>) -> Self::Output
			{
				$name::<{ max(EXP1, EXP2) }>(
					self.precision::<{ max(EXP1, EXP2) }>().0
						- rhs.precision::<{ max(EXP1, EXP2) }>().0,
				)
			}
		}

		impl<const EXP: usize> ops::SubAssign<$name<EXP>>
			for $name<EXP>
		{
			fn sub_assign(&mut self, rhs: Self)
			{
				self.0 -= rhs.0
			}
		}
	};
}
