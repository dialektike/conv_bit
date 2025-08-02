// 라이브러리의 모든 public 함수를 가져옵니다.
// 'conv_bit'은 Cargo.toml에 정의된 라이브러리 이름입니다.
use conv_bit::*;

#[test]
fn _from_eight_bool_to_eight_bit() {
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
fn _from_eight_bit_to_one_u8_int() {
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
fn _from_u8_int_to_eight_bit() {
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
