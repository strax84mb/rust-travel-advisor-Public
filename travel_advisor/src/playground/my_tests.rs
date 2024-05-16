#[cfg(test)]
pub mod some_test_mod {

    pub fn concat_strings(first: &String, second: String) -> String {
        format!("{first} {second}")
    }

    #[test]
    fn test_ownership() {
        let rust = "Rust".to_string();
        let language = String::from("language");
        assert_eq!("Rust language", concat_strings(&rust, language));
        // let my_ref = &rust;
        // let second_ref = &rust;
        // rust = my_ref.chars().skip(2).collect();
        // print!("{rust}");
        // println!(" {language}");
    }

    enum Either<L, R> {
        Left(L),
        Right(R),
    }

    impl <L, R> Either<L, R> {
        fn is_left(&self) -> bool {
            match self {
                Left(_) => true,
                Right(_) => false,
            }
        }

        fn is_right(&self) -> bool {
            !self.is_left()
        }
    }

    use Either::{
        Left as Left,
        Right as Right,
    };

    fn get_some(get_left: bool) -> Either<i32, &'static str> {
        match get_left {
            true => Left(-12),
            false => Right("qwerty"),
        }
    }

    #[test]
    fn test_enums() {
        let some_value = get_some(true);
        match some_value {
            Left(num) => println!("Number {num}"),
            Right(text) => println!("Text \"{text}\""),
        };
        if some_value.is_left() {
            println!("This is a left sided value");
        }
        if some_value.is_right() {
            println!("This is a right sided value");
        }
    }

}

use bencher::{
    Bencher,
    benchmark_group,
    benchmark_main,
};

#[allow(dead_code)]
fn bench_fn(bench: &mut Bencher) {
    bench.iter(|| {
        println!("Strale benched here");
    });
}

benchmark_group!(benches, bench_fn);

benchmark_main!(benches);
