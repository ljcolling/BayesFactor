#![allow(non_camel_case_types, clippy::float_cmp)]

use crate::error::{IntegrationResult, RuntimeError::*};
use crate::single::algorithm::Algorithm;
use crate::single::common::{Integrand, IntegrationConfig, Interval};
use crate::single::qk::{qk15, qk21};
use crate::single::util::{bisect, subinterval_too_small};
use crate::single::workspace::{SubIntervalInfo, WorkSpaceProvider};

/// QAG algorithm over finite interval
#[derive(Clone)]
pub struct QAG_FINITE {
    provider: WorkSpaceProvider,
}

impl QAG_FINITE {
    #[inline]
    pub fn new() -> Self {
        Self {
            provider: WorkSpaceProvider::new(),
        }
    }

    // initial integral
    fn initial_integral<F: Integrand>(
        &self,
        f: &mut F,
        interval: &Interval,
        config: &IntegrationConfig,
    ) -> (IntegrationResult, bool) {
        for i in 0..2 {
            let result0 = if i == 0 {
                qk15(f, &interval)
            } else if i == 1 {
                qk21(f, &interval)
            } else {
                unreachable!();
            };

            if result0.estimate.is_nan() {
                return (
                    IntegrationResult::new(
                        result0.estimate,
                        result0.delta,
                        Some(NanValueEncountered),
                    ),
                    true,
                );
            }

            let tolerance = config.tolerance.to_abs(result0.estimate.abs());
            if result0.delta <= tolerance && result0.delta != result0.asc || result0.delta == 0.0 {
                return (
                    IntegrationResult::new(result0.estimate, result0.delta, None),
                    true,
                );
            } else if config.limit == 1 {
                return (
                    IntegrationResult::new(
                        result0.estimate,
                        result0.delta,
                        Some(InsufficientIteration),
                    ),
                    true,
                );
            }

            let round_off = 50. * std::f64::EPSILON * result0.absvalue;
            if result0.delta <= round_off && result0.delta > tolerance {
                // ????????????????????????????????????????????????????????????????????????
                return (
                    IntegrationResult::new(result0.estimate, result0.delta, Some(RoundoffError)),
                    true,
                );
            }

            if i == 1 || result0.delta > tolerance * 1024. {
                return (
                    IntegrationResult::new(result0.estimate, result0.delta, None),
                    false,
                );
            }
        }

        unreachable!();
    }
}

impl<F: Integrand> Algorithm<F> for QAG_FINITE {
    fn integrate(
        &self,
        f: &mut F,
        interval: &Interval,
        config: &IntegrationConfig,
    ) -> IntegrationResult {
        let (mut roundoff_type1, mut roundoff_type2) = (0_i32, 0_i32);
        let mut error = None;

        // initial integral
        let (result0, finished) = self.initial_integral(f, interval, config);
        if finished {
            return result0;
        }

        // sum of the integral estimates for each interval
        let mut area = result0.estimate;

        // sum of the errors for each interval
        let mut deltasum = result0.delta;

        let ws = unsafe { self.provider.get_mut() };
        ws.clear();
        ws.reserve(config.limit);

        ws.push(SubIntervalInfo::new(
            interval.clone(),
            result0.estimate,
            result0.delta,
            0,
        ));

        for _ in 2..config.limit {
            // ???????????????????????????????????????????????????
            let info = ws.get();
            let current_level = info.level + 1;

            // ????????????????????????
            let (il1, il2) = bisect(&info.interval);

            // ??????????????????Gauss-Kronrod??????
            let result1 = qk21(f, &il1);
            let result2 = qk21(f, &il2);

            if result1.estimate.is_nan() || result2.estimate.is_nan() {
                error = Some(NanValueEncountered);
                break;
            }

            // ????????????????????????????????????????????????????????????????????????????????????
            let area12 = result1.estimate + result2.estimate;
            let delta12 = result1.delta + result2.delta;
            deltasum += delta12 - info.delta;
            area += area12 - info.estimate;

            // resasc?????????error????????????????????????????????????
            // => ??????????????????????????????????????????????????????????????????
            if result1.asc != result1.delta && result2.asc != result2.delta {
                if (info.estimate - area12).abs() <= 1e-5 * area12.abs()
                    && delta12 >= 0.99 * info.delta
                {
                    roundoff_type1 += 1;
                } else {
                    roundoff_type2 += 1;
                }
            }

            let tolerance = config.tolerance.to_abs(area.abs());

            // ???????????????????????????????????????????????????????????????????????????????????????????????????
            if deltasum > tolerance {
                if roundoff_type1 >= 6 || roundoff_type2 >= 20 {
                    error = Some(RoundoffError);
                } else if subinterval_too_small(il1.begin, il1.end, il2.end) {
                    error = Some(SubintervalTooSmall);
                }

                if error.is_some() {
                    return IntegrationResult::new(
                        ws.sum_results() + result1.estimate + result2.estimate,
                        deltasum,
                        error,
                    );
                }
            }

            // ???????????????????????????????????????????????????
            ws.update(
                SubIntervalInfo::new(il1, result1.estimate, result1.delta, current_level),
                SubIntervalInfo::new(il2, result2.estimate, result2.delta, current_level),
            );

            if deltasum <= tolerance {
                break;
            }
        }

        // ????????????????????????????????????????????????????????????
        IntegrationResult::new(ws.sum_results(), deltasum, error)
    }
}

extra_traits!(QAG_FINITE);
