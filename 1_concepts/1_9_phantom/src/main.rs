use std::marker::PhantomData;
use rand::{thread_rng, Rng};

struct Fact<T> {
    _phantom: PhantomData<T>
}

impl<T> Fact<T> {
    fn new() -> Self {
        Self { _phantom: PhantomData }
    }
}

impl Fact<Vec<String>> {
    fn fact(&self) -> &'static str {
        let facts = [
            "Vec is heap-allocated.",
            "Vec may re-allocate on growing.",
        ];
        let mut rng = thread_rng();
        facts[rng.gen_range(0..facts.len())]
    }
}

impl Fact<String> {
    fn fact(&self) -> &'static str {
        let facts = [
            "String is a growable UTF-8 encoded string.",
            "String is heap-allocated.",
        ];
        let mut rng = thread_rng();
        facts[rng.gen_range(0..facts.len())]
    }
}

fn main() {
    let f: Fact<Vec<String>> = Fact::new();
    println!("Fact about Vec: {}", f.fact());
    println!("Fact about Vec: {}", f.fact());

    let f: Fact<String> = Fact::new();
    println!("Fact about String: {}", f.fact());
    println!("Fact about String: {}", f.fact());
}
