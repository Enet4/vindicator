
macro_rules! test_fusion {
    ($name: ident) => {
        mod $name {
            use approx::assert_abs_diff_eq;
            use vindicator::{
                SearchEntry,
                fuser::{comb_max, comb_mnz, comb_sum, fuse_scored},
                trec::parse_from_trec,
            };

            #[test]
            fn test_comb_max() {
                let raw_data = include_str!(concat!("resources/", stringify!($name), ".top.txt"));
                let data = parse_from_trec(raw_data).expect("could not parse test input file");

                let out = fuse_scored(&data, comb_max);
                let raw_gt = include_str!(concat!("resources/", stringify!($name), ".out.max.txt"));
                let gt =
                    parse_from_trec(raw_gt).expect("could not parse comMAX test ground truth file");
                let gt: Vec<_> = gt.into_iter().map(|e| e.to_entry()).collect();
                assert_abs_diff_eq!(&*out, &*gt);
            }

            #[test]
            fn test_comb_sum() {
                let raw_data = include_str!(concat!("resources/", stringify!($name), ".top.txt"));
                let data = parse_from_trec(raw_data).expect("could not parse test input file");

                let out = fuse_scored(&data, comb_sum);
                let raw_gt = include_str!(concat!("resources/", stringify!($name), ".out.sum.txt"));
                let gt =
                    parse_from_trec(raw_gt).expect("could not parse combSUM test ground truth file");
                let gt: Vec<_> = gt.into_iter().map(|e| e.to_entry()).collect();
                assert_abs_diff_eq!(&*out, &*gt);
            }

            #[test]
            fn test_comb_mnz() {
                let raw_data = include_str!(concat!("resources/", stringify!($name), ".top.txt"));
                let data = parse_from_trec(raw_data).expect("could not parse test input file");

                let out = fuse_scored(&data, comb_mnz);
                let raw_gt = include_str!(concat!("resources/", stringify!($name), ".out.mnz.txt"));
                let gt = parse_from_trec(raw_gt).expect("could not parse test ground truth file");
                let gt: Vec<_> = gt.into_iter().map(|e| e.to_entry()).collect();
                assert_abs_diff_eq!(&*out, &*gt);
            }
        }
    };
}

test_fusion!(test1);
