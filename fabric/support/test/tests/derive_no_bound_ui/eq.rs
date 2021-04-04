trait Config {
	type C;
}

#[derive(fabric_support::EqNoBound)]
struct Foo<T: Config> {
	c: T::C,
}

fn main() {}
