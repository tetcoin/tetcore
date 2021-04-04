trait Config {
	type C;
}

#[derive(fabric_support::CloneNoBound)]
struct Foo<T: Config> {
	c: T::C,
}

fn main() {}
