trait Config {
	type C;
}

#[derive(fabric_support::PartialEqNoBound)]
struct Foo<T: Config> {
	c: T::C,
}

fn main() {}
