use libc::{c_double, c_long};
use rayon::prelude::*;
use bfrs::bf_onesample_default_t;
use bfrs::Prior;

#[no_mangle]
pub extern "C" fn bfwrapper(
    d_double: *mut c_double,
    n_double: *mut c_double,
    location_double: *mut c_double,
    scale_double: *mut c_double,
    ll_double: *mut c_double,
    ul_double: *mut c_double,
    bf_double: *mut c_double,
    bf_comp_double: *mut c_double,
    elements: c_long,
) {
    let size: usize = elements as usize;
    let mut d: Vec<f64> = vec![0f64; size];
    let mut n: Vec<f64> = vec![0f64; size];
    let mut location: Vec<f64> = vec![0f64; size];
    let mut scale: Vec<f64> = vec![0f64; size];
    let mut ll: Vec<f64> = vec![0f64; size];
    let mut ul: Vec<f64> = vec![0f64; size];

    for i in 0..size {
        unsafe {
            d[i] = *(d_double.offset(i as isize)) as f64;
            n[i] = *(n_double.offset(i as isize)) as f64;
            location[i] = *(location_double.offset(i as isize)) as f64;
            scale[i] = *(scale_double.offset(i as isize)) as f64;
            ll[i] = *(ll_double.offset(i as isize)) as f64;
            ul[i] = *(ul_double.offset(i as isize)) as f64;

        }
    }

    let (bf, bf_comp) = bfwrapper_safe(d, n, location, scale, ll, ul);

    for i in 0..size {
        unsafe {
            *bf_double.offset(i as isize) = bf[i];
            *bf_comp_double.offset(i as isize) = bf_comp[i];
        }
    }
}

fn bfwrapper_safe(
    d: Vec<f64>,
    n: Vec<f64>,
    location: Vec<f64>,
    scale: Vec<f64>,
    ll: Vec<f64>,
    ul: Vec<f64>,
) -> (Vec<f64>, Vec<f64>) {
    d.into_par_iter().enumerate().map(|(i, this_d)| {
        let this_n = n[i];
        let this_location = location[i];
        let this_scale = scale[i];
        let this_ll = ll[i];
        let this_ul = ul[i];
        let prior = Prior{family: "Cauchy".into(), params: vec![this_location, this_scale, this_ll, this_ul]};

        let (bf, bf_comp) = bf_onesample_default_t(this_d, this_n as i32, &prior);
        (bf, bf_comp)

    }
    ).collect()
}


