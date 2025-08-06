use std::fmt;

/// 10진수 값과 그에 해당하는 2진수 문자열 표현을 함께 저장하는 구조체입니다.
#[derive(Debug)] // `{:?}` 포맷으로 쉽게 출력하기 위해 Debug 트레이트를 추가합니다.
pub struct Number {
    /// 10진수 값 (unsigned 32-bit integer)
    pub decimal: u32,
    /// 4자리마다 밑줄로 구분된 2진수 문자열 표현
    pub binary: String,
}

impl Number {
    /// 10진수 값을 받아 새로운 Number 인스턴스를 생성합니다.
    ///
    /// 이 함수는 인스턴스를 만드는 과정에서 4자리마다 밑줄이 들어간
    /// 2진수 문자열을 자동으로 생성하여 저장합니다.
    ///
    /// # Arguments
    ///
    /// * `decimal_value` - 변환할 u32 타입의 10진수 정수
    pub fn new(decimal_value: u32) -> Self {
        Number {
            decimal: decimal_value,
            binary: Self::format_binary_with_separator(decimal_value),
        }
    }

    /// 숫자를 4자리마다 '_'로 구분된 2진수 문자열로 변환하는 private 헬퍼 함수입니다.
    fn format_binary_with_separator(n: u32) -> String {
        let binary_string = format!("{:b}", n);
        // 문자열 길이가 4 이하이면 구분 기호를 넣을 필요가 없습니다.
        if binary_string.len() <= 4 {
            return binary_string;
        }

        let mut result = String::new();
        // 문자열을 뒤집어서 4자리씩 끊기 쉽게 만듭니다.
        let chars: Vec<char> = binary_string.chars().rev().collect();

        for (i, c) in chars.iter().enumerate() {
            // 4의 배수 인덱스이고, 첫 문자가 아닐 때 '_'를 추가합니다.
            if i > 0 && i % 4 == 0 {
                result.push('_');
            }
            result.push(*c);
        }

        // 다시 뒤집어서 최종 결과를 반환합니다.
        result.chars().rev().collect()
    }
}

// 사용자가 보기 편한 형태로 출력하기 위해 Display 트레이트를 구현합니다.
impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "10진수로: {}, 2진수로: {}", self.decimal, self.binary)
    }
}
