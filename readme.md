# Intro

Rust는 매크로 `print!`와 `println!`를 사용하여 콘솔에 텍스트를 출력

- [Hello World](https://doc.rust-lang.org/rust-by-example/hello.html)
* [Formatted print](https://doc.rust-lang.org/rust-by-example/hello/print.html)


> **함수 매크로 차이 :**

함수와 매크로는 처리되는 시간과 작동방식에 차이가 있음

함수는 런타임에 아규먼트를 해석하고 이미 존재하는 컴파일된 바이너리 함수를 호출하는것임

매크로는 컴파일 타임에 해석되며 컴파일러가 `매크로가 작성된 자리에 맞는 코드`를 작성함 (이 작성된 코드안에 이미 존재하는 컴파일된 바이너리 함수가 포함되어있음) 

매크로 쓰는 이유 :

1. 가변 인자 처리
2. 포맷 스트링 어택 방지(컴파일 타임에 포맷 스트링을 분석해서 코드로 박제해서 런타임에 이상한짓 못하게) 

# Variables

Rust에서는 변수가 기본적으로 불변이라 값을 변경할 수 없음

`let mut`으로 변수 선언시 값 변경가능

- [Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)


# Functions

```rust
const fn add_bytes(a: u8, b: u8) -> u8 {
    a + b + 2
}

fn main() {
    const ANSWER: u8 = add_bytes(b'~', b'\x7f');
    println!("{}", ANSWER);     // 255
}
```

- [How Functions Work](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)