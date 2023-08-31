use super::rules::{next::NextStep, traits::IntoSpecificCaptureType};
use super::{convert::convert_and_filter, *};
use log::info;
use std::{
    fmt::{Debug, Display},
    ops::RangeInclusive,
    str::FromStr,
};

fn single_range_str_check<
    'a,
    T: FromStr + Copy + Debug + Display + PartialOrd,
    C: IntoSpecificCaptureType<'a>,
>(
    captures: &mut CaptureData<'a, C>,
    range: &RangeInclusive<T>,
    range_mode: RangeMode,
) -> bool {
    match range_mode {
        RangeMode::Any => {
            info!(
                "(range mode {}) for data {:?}",
                "`any`".yellow(),
                captures.text_for_capture
            );
            captures
                .text_for_capture
                .iter()
                .filter(|&num| {
                    convert_and_filter(num.as_str().unwrap())
                        .map(|num| {
                            if range.contains(&num) {
                                info!("range contains {:?}", num);
                                true
                            } else {
                                info!("range does not contain {:?}", num);
                                false
                            }
                        })
                        .unwrap_or(false)
                })
                .count()
                > 0
        }
        RangeMode::All => {
            info!(
                "(range mode {}) for data {:?}",
                "`all`".yellow(),
                captures.text_for_capture
            );
            captures
                .text_for_capture
                .iter()
                .filter(|&num| {
                    convert_and_filter(num.as_str().unwrap())
                        .map(|num| {
                            if range.contains(&num) {
                                info!("range contains {:?}", num);
                                true
                            } else {
                                info!("range does not contain {:?}", num);
                                false
                            }
                        })
                        .unwrap_or(false)
                })
                .count()
                == captures.text_for_capture.len()
        }
        RangeMode::Exactly(target_count) => {
            info!(
                "(range mode {}) for data {:?}",
                "`exactly`".yellow(),
                captures.text_for_capture
            );
            let required_count = target_count.min(captures.text_for_capture.len());
            captures
                .text_for_capture
                .iter()
                .filter(|&num| {
                    convert_and_filter(num.as_str().unwrap())
                        .map(|num| {
                            if range.contains(&num) {
                                info!("range contains {:?}", num);
                                true
                            } else {
                                info!("range does not contain {:?}", num);
                                false
                            }
                        })
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
    C: IntoSpecificCaptureType<'a>,
>(
    range_str: &RangeStr,
    captures: &mut CaptureData<'a, C>,
    value: &RangeInclusive<T>,
) -> NextStep {
    if single_range_str_check(captures, value, range_str.range_mode) {
        NextStep::Finish
    } else {
        NextStep::Error(Some(std::mem::take(&mut captures.hashmap_for_error)))
    }
}
