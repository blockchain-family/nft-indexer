use bigdecimal::num_bigint::{BigInt, Sign};
use bigdecimal::BigDecimal;
use indexer_lib::{AnyExtractable, AnyExtractableOutput, ExtractInput, ParsedOutput};
use num_traits::ToPrimitive;
use ton_block::Transaction;
use ton_types::UInt256;

pub fn extract_events(
    data: &Transaction,
    hash: UInt256,
    events: &[AnyExtractable],
) -> Option<ParsedOutput<AnyExtractableOutput>> {
    ExtractInput {
        transaction: data,
        what_to_extract: events,
        hash,
    }
    .process()
    .ok()
    .flatten()
}

pub trait SafeDecimals {
    fn round_safe(&self, round_digits: i64) -> BigDecimal;
}

impl SafeDecimals for BigDecimal {
    fn round_safe(&self, round_digits: i64) -> BigDecimal {
        let (bigint, decimal_part_digits) = self.as_bigint_and_exponent();
        let need_to_round_digits = decimal_part_digits - round_digits;
        if round_digits >= 0 && need_to_round_digits <= 0 {
            return self.clone();
        }
        match bigint.to_i128() {
            None => {
                let mut number = bigint.clone();
                if number.sign() == Sign::Minus {
                    number = -number;
                }
                for _ in 0..(need_to_round_digits - 1) {
                    number /= 10;
                }
                let digit = number % 10;

                if digit <= BigInt::from(4) {
                    return self.with_scale(round_digits);
                }
            }
            Some(mut number) => {
                if number < 0 {
                    number = -number;
                }
                for _ in 0..(need_to_round_digits - 1) {
                    number /= 10;
                }
                let digit = number % 10;

                if digit <= 4 {
                    return self.with_scale(round_digits);
                }
            }
        }

        if bigint.sign() == Sign::Minus {
            self.with_scale(round_digits) - BigDecimal::new(BigInt::from(1), round_digits)
        } else {
            self.with_scale(round_digits) + BigDecimal::new(BigInt::from(1), round_digits)
        }
    }
}
