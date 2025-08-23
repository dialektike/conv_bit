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

    /// 숫자를 8비트 단위로 패딩하고, 4자리마다 '_'로 구분된 2진수 문자열로 변환합니다.
    fn format_binary_with_separator(n: u32) -> String {
        let binary_string = format!("{:b}", n);
        let len = binary_string.len();

        // 비트 길이에 따라 패딩할 길이를 결정합니다 (8, 16, 24, 32).
        let padded_len = match len {
            0..=8 => 8,
            9..=16 => 16,
            17..=24 => 24,
            _ => 32,
        };

        // 왼쪽을 '0'으로 채워서 최종 길이에 맞춥니다.
        let padded_binary = format!("{:0>width$}", binary_string, width = padded_len);

        // 4자리마다 '_' 구분 기호를 삽입합니다.
        let mut result = String::with_capacity(padded_len + (padded_len / 4) - 1);
        for (i, c) in padded_binary.chars().enumerate() {
            if i > 0 && i % 4 == 0 {
                result.push('_');
            }
            result.push(c);
        }
        result
    }
}

// 사용자가 보기 편한 형태로 출력하기 위해 Display 트레이트를 구현합니다.
impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "10진수로: {}, 2진수로: {}", self.decimal, self.binary)
    }
}
