// 'my_number_lib'는 Cargo.toml에 정의된 라이브러리 이름입니다.
// 라이브러리의 number 모듈에서 Number 구조체를 가져옵니다.
use conv_bit::number::Number;

#[test]
fn _number_creation_and_formatting() {
    // 1. 8비트 미만 숫자 테스트 (0으로 패딩)
    let num1 = Number::new(10);
    assert_eq!(num1.decimal, 10);
    assert_eq!(num1.binary, "0000_1010");

    // 2. 8비트 미만 다른 숫자 테스트 (0으로 패딩)
    let num2 = Number::new(42);
    assert_eq!(num2.decimal, 42);
    assert_eq!(num2.binary, "0010_1010");

    // 3. 16비트 숫자 테스트 (패딩 필요 없음)
    let num3 = Number::new(48879);
    assert_eq!(num3.decimal, 48879);
    assert_eq!(num3.binary, "1011_1110_1110_1111");

    // 4. 8비트 경계값 테스트 (패딩 필요 없음)
    let num4 = Number::new(255);
    assert_eq!(num4.decimal, 255);
    assert_eq!(num4.binary, "1111_1111");

    // 5. 0 테스트 (0으로 패딩)
    let num5 = Number::new(0);
    assert_eq!(num5.decimal, 0);
    assert_eq!(num5.binary, "0000_0000");
}

#[test]
fn _display_trait() {
    // 105는 2진수로 '1101001'이므로, '0110_1001'로 패딩 및 포맷팅되어야 합니다.
    let num = Number::new(105);
    let formatted_string = format!("{}", num);
    assert_eq!(formatted_string, "10진수로: 105, 2진수로: 0110_1001");
}
