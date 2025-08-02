//! # 변환(conv) 모듈
//!
//! 이 모듈에는 비트와 숫자를 변환하는 함수가 들어 있습니다. 이 모듈 안에 들어 있는 코드는
//! 기본적으로 편의상 최적화를 실행하지 못했습니다. 기능을 구현하는데 급급한 면이 있습니다. 
//! 참고하세요.
//!
//! ## 목록
//! 
//! - `from_eight_bool_to_eight_bit`: 8 자리 `bool` Array 을 8 자리 bit Array 으로 변환
//! - `from_eight_bit_to_eight_bool`: 8 자리 bit Array 를 8 자리 `bool` Array 으로 변환
//! - `from_eight_bit_to_one_u8_int`: 8 자리 bit Array를 1개의 `u8` int로 변환


/// `bool` 형식으로 된 여덟 자리 Array를 `u8` 형식으로 된 여덟 자리 Array로 변경하는 함수
///
/// 이 함수는 `bool` 값을 `u8` 값으로 변환합니다. `true`는 `1`, `false`는 `0`으로 변환됩니다.
/// 다음과 같이 작동합니다.
///  ```rust
///     let temp_a = [false, true, true, false, true, false, false, true];
///     let test = from_eight_bool_to_eight_bit(temp_a);
///     assert_eq!(test, [0, 1, 1, 0, 1, 0, 0, 1]);
/// ```
pub fn from_eight_bool_to_eight_bit(input: [bool; 8]) -> [u8; 8] {
    input.map(|b| b as u8)
}
///  `u8` 형식으로 된 여덟 자리 Array 를 bool` 형식으로 된 여덟 자리 Array로 변경하는 함수
///
/// 다음과 같이 작동합니다.
///  ```rust
///     let temp_a = [0, 1, 1, 0, 1, 0, 0, 1];
///     let test = from_eight_bit_to_eight_bool(temp_a);
///      assert_eq!(test, [false, true, true, false, true, false, false, true]);
/// ```
pub fn from_eight_bit_to_eight_bool(input: [u8; 8]) -> [bool; 8] {
    input.map(|b| b == 1)
}

///  `u8` 형식으로 된 이진수 여덟 자리 리스트를 `u8` 형식으로 된 `int`로 변경하는 함수
///
/// 다음과 같이 작동합니다.
///  ```rust
///     let temp_a = [0, 1, 1, 0, 1, 0, 0, 1];
///     let test = from_eight_bit_to_one_u8_int(temp_a);
/// ```
pub fn from_eight_bit_to_one_u8_int(bits: [u8; 8]) -> u8 {
    let mut result: u8 = 0;
    for bit in bits {
        // 1. 현재 결과를 왼쪽으로 1비트 시프트합니다. (결과 * 2 와 동일)
        // 예: 101(5) -> 1010(10)
        result <<= 1;

        // 2. 가장 오른쪽 비트에 현재 비트(0 또는 1)를 추가합니다.
        // 예: 1010(10) | 1 -> 1011(11)
        result |= bit;
    }
    result
}
///  `u8` 형식인 `int`를 `bool`로 된 여덟 자리 리스트로 변경하는 함수
///
/// 다음과 같이 작동합니다.
///  ```rust
///     let result = from_u8_int_to_eight_bit(5);
///     assert_eq!(result, [0, 0, 0, 0, 0, 1, 0, 1]);
/// ```
pub fn from_u8_int_to_eight_bit(n: u8) -> [u8; 8] {
    let mut result = [0u8; 8];
    for i in 0..8 {
        // i번째 비트를 확인하기 위해 오른쪽으로 시프트하고 1과 AND 연산합니다.
        // 예: i=0일 때, 7비트 시프트 -> 가장 왼쪽 비트(MSB)
        if (n >> (7 - i)) & 1 == 1 {
            result[i] = 1;
        } else {
            result[i] = 0;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn temp_ones_c_in_eight_bit() {
        let temp_a: [bool; 8] = [false, false, false, false, false, false, false, false];
        let result = from_eight_bool_to_eight_bit(temp_a);
        assert_eq!(result, [0, 0, 0, 0, 0, 0, 0, 0]);
        let temp_a: [bool; 8] = [false, false, false, false, false, false, false, true];
        let result = from_eight_bool_to_eight_bit(temp_a);
        assert_eq!(result, [0, 0, 0, 0, 0, 0, 0, 1]);
        let temp_a: [bool; 8] = [false, false, false, false, false, false, true, true];
        let result = from_eight_bool_to_eight_bit(temp_a);
        assert_eq!(result, [0, 0, 0, 0, 0, 0, 1, 1]);
        let temp_a: [bool; 8] = [false, false, false, false, false, true, true, true];
        let result = from_eight_bool_to_eight_bit(temp_a);
        assert_eq!(result, [0, 0, 0, 0, 0, 1, 1, 1]);
        let temp_a: [bool; 8] = [true, true, true, false, false, true, true, true];
        let result = from_eight_bool_to_eight_bit(temp_a);
        assert_eq!(result, [1, 1, 1, 0, 0, 1, 1, 1]);
        let temp_a: [bool; 8] = [true, true, true, true, true, true, true, true];
        let result = from_eight_bool_to_eight_bit(temp_a);
        assert_eq!(result, [1, 1, 1, 1, 1, 1, 1, 1]);
    }

    #[test]
    fn temp_from_eight_bit_to_one_u8_int() {
        let temp_a: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
        let result = from_eight_bit_to_one_u8_int(temp_a);
        assert_eq!(result, 0);
        let temp_a: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 1];
        let result = from_eight_bit_to_one_u8_int(temp_a);
        assert_eq!(result, 1);
        let temp_a: [u8; 8] = [0, 0, 0, 0, 0, 0, 1, 0];
        let result = from_eight_bit_to_one_u8_int(temp_a);
        assert_eq!(result, 2);
        let temp_a: [u8; 8] = [0, 0, 0, 0, 0, 0, 1, 1];
        let result = from_eight_bit_to_one_u8_int(temp_a);
        assert_eq!(result, 3);
        let temp_a: [u8; 8] = [0, 0, 0, 0, 0, 1, 0, 0];
        let result = from_eight_bit_to_one_u8_int(temp_a);
        assert_eq!(result, 4);
        let temp_a: [u8; 8] = [0, 0, 0, 0, 0, 1, 0, 1];
        let result = from_eight_bit_to_one_u8_int(temp_a);
        assert_eq!(result, 5);
    }

    #[test]
    fn temp_u8_int_to_eight_bit() {
        let result = from_u8_int_to_eight_bit(0);
        let temp_0 = [false, false, false, false, false, false, false, false];
        assert_eq!(result, from_eight_bool_to_eight_bit(temp_0));
        let result = from_u8_int_to_eight_bit(1);
        let temp_1 = [false, false, false, false, false, false, false, true];
        assert_eq!(result, from_eight_bool_to_eight_bit(temp_1));
        let result = from_u8_int_to_eight_bit(2);
        let temp_1 = [false, false, false, false, false, false, true, false];
        assert_eq!(result, from_eight_bool_to_eight_bit(temp_1));
        let result = from_u8_int_to_eight_bit(3);
        let temp_1 = [false, false, false, false, false, false, true, true];
        assert_eq!(result, from_eight_bool_to_eight_bit(temp_1));
        let result = from_u8_int_to_eight_bit(4);
        let temp_1 = [false, false, false, false, false, true, false, false];
        assert_eq!(result, from_eight_bool_to_eight_bit(temp_1));
        let result = from_u8_int_to_eight_bit(79);
        let temp_1 = [false, true, false, false, true, true, true, true];
        assert_eq!(result, from_eight_bool_to_eight_bit(temp_1));
        let result = from_u8_int_to_eight_bit(100);
        let temp_1 = [false, true, true, false, false, true, false, false];
        assert_eq!(result, from_eight_bool_to_eight_bit(temp_1));
        let result = from_u8_int_to_eight_bit(255);
        let temp_1 = [true, true, true, true, true, true, true, true];
        assert_eq!(result, from_eight_bool_to_eight_bit(temp_1));
    }
}
