use super::rules::{next::NextStep, traits::IntoConcreteType};
use super::{convert::convert_and_filter, *};
use std::{
    fmt::{Debug, Display},
    ops::RangeInclusive,
    str::FromStr,
};

fn single_range_str_check<
    'a,
    T: FromStr + Copy + Debug + Display + PartialOrd,
    C: IntoConcreteType<'a>,
>(
    captures: &mut CaptureData<'a, C>,
    range: &RangeInclusive<T>,
    range_mode: RangeMode,
) -> bool {
    match range_mode {
        RangeMode::Any => {
            captures
                .text_for_capture
                .iter()
                .filter(|&num| {
                    convert_and_filter(num.into_str().unwrap())
                        .map(|num| range.contains(&num))
                        .unwrap_or(false)
                })
                .count()
                > 0
        }
        RangeMode::All => {
            captures
                .text_for_capture
                .iter()
                .filter(|&num| {
                    convert_and_filter(num.into_str().unwrap())
                        .map(|num| range.contains(&num))
                        .unwrap_or(false)
                })
                .count()
                == captures.text_for_capture.len()
        }
        RangeMode::Exactly(target_count) => {
            let required_count = target_count.min(captures.text_for_capture.len());
            captures
                .text_for_capture
                .iter()
                .filter(|&num| {
                    convert_and_filter(num.into_str().unwrap())
                        .map(|num| range.contains(&num))
                        .unwrap_or(false)
                })
                .take(required_count)
                .count()
                == required_count
        }
    }
}

pub fn single_str_result<
    'a,
    T: Debug + FromStr + Copy + Display + PartialOrd,
    C: IntoConcreteType<'a>,
>(
    range_str: &RangeStr,
    captures: &mut CaptureData<'a, C>,
    value: &RangeInclusive<T>,
) -> NextStep {
    if single_range_str_check(captures, value, range_str.range_mode) {
        return NextStep::Finish;
    } else {
        return NextStep::Error(Some(std::mem::take(&mut captures.hashmap_for_error)));
    }
}
