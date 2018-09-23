
macro_rules! assert_approx_eq {
    ($expected: expr, $actual: expr, $tolerance: expr) => {
        let ex = $expected;
        let act = $actual;
        if act < ex - $tolerance {
            panic!(format!(
                "Actual value {:?} is lower than expected value {:?} (at tolerance {:?})",
                act, ex, $tolerance
            ));
        }
        if act > ex + $tolerance {
            panic!(format!(
                "Actual value {:?} is higher than expected value {:?} (at tolerance {:?})",
                act, ex, $tolerance
            ));
        }
    };
}
