# conv_bit

`conv_bit`는 컴퓨터 구조 및 데이터 표현을 학습하는 분들을 돕기 위해 만든 Rust 라이브러리입니다. 숫자, 비트 배열, bool 배열 간의 변환을 쉽게 수행할 수 있는 유틸리티 함수들을 제공합니다.

## 주요 기능

- `bool` 배열과 `u8` 비트 배열 간의 상호 변환
- 8비트 `u8` 배열과 `u8` 정수 간의 상호 변환
- 10진수를 받아 보기 쉽게 포맷팅된 2진수 문자열을 함께 관리하는 `Number` 구조체 제공

## 설치

`Cargo.toml` 파일의 `[dependencies]` 섹션에 다음 라인을 추가하세요:

```toml
[dependencies]
conv_bit = "0.1.0"
```

## 사용법

### 비트/Bool 변환

#### `from_eight_bool_to_eight_bit`

8개의 `bool` 값으로 이루어진 배열을 `u8` 배열(0과 1)로 변환합니다.

```rust
use conv_bit::from_eight_bool_to_eight_bit;

let bool_array = [false, true, true, false, true, false, false, true];
let bit_array = from_eight_bool_to_eight_bit(bool_array);

assert_eq!(bit_array, [0, 1, 1, 0, 1, 0, 0, 1]);
```

#### `from_eight_bit_to_eight_bool`

8개의 `u8` 값(0과 1)으로 이루어진 배열을 `bool` 배열로 변환합니다.

```rust
use conv_bit::from_eight_bit_to_eight_bool;

let bit_array = [0, 1, 1, 0, 1, 0, 0, 1];
let bool_array = from_eight_bit_to_eight_bool(bit_array);

assert_eq!(bool_array, [false, true, true, false, true, false, false, true]);
```

### 비트/정수 변환

#### `from_eight_bit_to_one_u8_int`

8개의 `u8` 값(0과 1)으로 이루어진 배열을 하나의 `u8` 정수로 변환합니다.

```rust
use conv_bit::from_eight_bit_to_one_u8_int;

let bit_array = [0, 1, 1, 0, 1, 0, 0, 1]; // 105
let integer = from_eight_bit_to_one_u8_int(bit_array);

assert_eq!(integer, 105);
```

#### `from_one_u8_int_to_eight_bit`

`u8` 정수를 8개의 `u8` 값(0과 1)으로 이루어진 배열로 변환합니다.

```rust
use conv_bit::from_one_u8_int_to_eight_bit;

let integer = 5;
let bit_array = from_one_u8_int_to_eight_bit(integer);

assert_eq!(bit_array, [0, 0, 0, 0, 0, 1, 0, 1]);
```

### `Number` 구조체 사용하기

`Number` 구조체는 10진수 값을 받아, **8비트 단위(1바이트)로 패딩된** 2진수 문자열을 생성합니다. 가독성을 위해 4자리마다 밑줄(`_`)이 자동으로 추가됩니다.

- 8비트보다 작은 수는 8비트로 패딩됩니다. (`10` -> `0000_1010`)
- 16비트보다 작은 수는 16비트로 패딩됩니다.
- 이와 같은 방식으로 24비트, 32비트로 확장됩니다.

```rust
use conv_bit::number::Number;

// 10진수 값으로 Number 인스턴스 생성
let num = Number::new(255);

// 10진수와 2진수 값 확인
assert_eq!(num.decimal, 255);
assert_eq!(num.binary, "1111_1111");

// Display 트레이트 구현을 통해 쉽게 출력 가능
println!("{}", num); // 출력: 10진수로: 255, 2진수로: 1111_1111

// 작은 숫자는 8비트에 맞춰 왼쪽에 0이 패딩됩니다.
let num2 = Number::new(60);
println!("{}", num2); // 출력: 10진수로: 60, 2진수로: 0011_1100
```

---

## 참고: 파이썬 사례

`conv_bit`는 Python의 `bin()`, `oct()`, `hex()`, `int()`와 같은 내장 함수에서 영감을 얻었습니다. 다음은 Python에서 비슷한 작업을 수행하는 방법입니다.

### 10진수를 다른 진법으로 변환

```python
# 10진수 60을 2진수, 8진수, 16진수로 변환
print(bin(60))  # 출력: '0b111100'
print(oct(60))  # 출력: '0o74'
print(hex(60))  # 출력: '0x3c'
```

### 다른 진법을 10진수로 변환

```python
# 2진수, 8진수, 16진수 문자열을 10진수 정수로 변환
print(int('111100', 2)) # 출력: 60
print(int('74', 8))     # 출력: 60
print(int('3c', 16))    # 출력: 60
```
