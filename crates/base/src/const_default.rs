
pub trait ConstDefault: Sized
{
	const DEFAULT: Self;
}

impl<T> ConstDefault for Option<T>
{
	const DEFAULT: Self = None;
}

impl<T: ConstDefault> ConstDefault for core::mem::MaybeUninit<T>
{
	const DEFAULT: Self = Self::new(T::DEFAULT);
}

impl ConstDefault for std::string::String
{
	const DEFAULT: Self = Self::new();
}

impl<K: Ord, V> ConstDefault for std::collections::BTreeMap<K, V>
{
	const DEFAULT: Self = Self::new();
}

impl<T: Ord> ConstDefault for std::collections::BTreeSet<T>
{
	const DEFAULT: Self = Self::new();
}

impl<'a, T: 'a> ConstDefault for &'a [T]
{
	const DEFAULT: Self = &[];
}

impl<T> ConstDefault for *const T
{
	const DEFAULT: Self = core::ptr::null();
}

impl<T> ConstDefault for *mut T
{
	const DEFAULT: Self = core::ptr::null_mut();
}

impl<T: ConstDefault> ConstDefault for core::mem::ManuallyDrop<T>
{
	const DEFAULT: Self = Self::new(T::DEFAULT);
}

impl<T: ?Sized> ConstDefault for core::marker::PhantomData<T>
{
	const DEFAULT: Self = Self;
}

impl ConstDefault for core::marker::PhantomPinned
{
	const DEFAULT: Self = Self;
}

impl ConstDefault for core::time::Duration
{
	const DEFAULT: Self = core::time::Duration::from_secs(0);
}

impl<T: ConstDefault, const N: usize> ConstDefault for [T; N]
{
	const DEFAULT: Self = [T::DEFAULT; N];
}

macro_rules! impl_primitive
{
	($($typ:ty=$d:expr),*) =>
	{
		$(
			impl ConstDefault for $typ
			{
				const DEFAULT: Self = $d;
			}

			impl ConstDefault for &$typ
			{
				const DEFAULT: Self = &<$typ as ConstDefault>::DEFAULT;
			}
		)*
	};
}

impl_primitive!
{
	()=(),
	bool=false,
	f32=0.0,
	f64=0.0,
	char='\x00',
	&str="",
	u8=0,
	u16=0,
	u32=0,
	u64=0,
	usize=0,
	u128=0,
	i8=0,
	i16=0,
	i32=0,
	i64=0,
	isize=0,
	i128=0
}

macro_rules! impl_tuple
{
	(@rec $t:ident) => { };
	(@rec $_:ident $($t:ident)+) =>
	{
		impl_tuple! { @impl $($t)* }
		impl_tuple! { @rec $($t)* }
	};
	(@impl $($t:ident)*) =>
	{
		impl<$($t: ConstDefault,)*> ConstDefault for ($($t,)*)
		{
			const DEFAULT: Self = ($($t::DEFAULT,)*);
		}
	};
	($($t:ident)*) =>
	{
		impl_tuple! { @rec _t $($t)* }
	};
}

impl_tuple!
{
	A B C D E F G H I J K L
}
