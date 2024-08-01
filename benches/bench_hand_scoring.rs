use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};

use fivesou::scoring::Payment;
    
fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("jpml2022 scoring #1", |b| b.iter(|| {
        let _ = fivesou::score_hand_from_str("p1,p2,p3,p4,p4,p4,p5,p6,p7,p8,s2,s3,s4", "", "p9", 'e', 'e', 't', "", "", 0, "JPML2022");
        let _ = fivesou::score_hand_from_str("p1,p2,p3,p4,p4,p4,p5,p6,p7,p8,s2,s3,s4", "", "p9", 'e', 'e', 'r', "", "", 0, "JPML2022");
        let _ = fivesou::score_hand_from_str("p1,p2,p3,p4,p4,p4,p5,p6,p7,p8,s2,s3,s4", "", "p9", 's', 'e', 't', "", "", 0, "JPML2022");
        let _ = fivesou::score_hand_from_str("p1,p2,p3,p4,p4,p4,p5,p6,p7,p8,s2,s3,s4", "", "p9", 's', 'e', 'r', "", "", 0, "JPML2022");
    }));

    c.bench_function("jpml2022 scoring #2", |b| b.iter(|| {
        let _ = fivesou::score_hand_from_str("m2,m2,m3,m3,p3,p3,p5,p5,s6,s6,s7,s8,s8", "", "s7", 'e', 'e', 't', "", "", 0, "JPML2022");
        let _ = fivesou::score_hand_from_str("m2,m2,m3,m3,p3,p3,p5,p5,s6,s6,s7,s8,s8", "", "s7", 'e', 'e', 'r', "", "", 0, "JPML2022");
        let _ = fivesou::score_hand_from_str("m2,m2,m3,m3,p3,p3,p5,p5,s6,s6,s7,s8,s8", "", "s7", 's', 'e', 't', "", "", 0, "JPML2022");
        let _ = fivesou::score_hand_from_str("m2,m2,m3,m3,p3,p3,p5,p5,s6,s6,s7,s8,s8", "", "s7", 's', 'e', 'r', "", "", 0, "JPML2022");
    }));

    c.bench_function("jpml2022 scoring #3", |b| b.iter(|| {
        let _ = fivesou::score_hand_from_str("m3,m5,m6,m7,m8,m8,m8", "p8,p8,p8|m2,m2,m2", "m3", 'e', 'e', 't', "", "", 0, "JPML2022");
        let _ = fivesou::score_hand_from_str("m3,m5,m6,m7,m8,m8,m8", "p8,p8,p8|m2,m2,m2", "m3", 'e', 'e', 'r', "", "", 0, "JPML2022");
        let _ = fivesou::score_hand_from_str("m3,m5,m6,m7,m8,m8,m8", "p8,p8,p8|m2,m2,m2", "m3", 's', 'e', 't', "", "", 0, "JPML2022");
        let _ = fivesou::score_hand_from_str("m3,m5,m6,m7,m8,m8,m8", "p8,p8,p8|m2,m2,m2", "m3", 's', 'e', 'r', "", "", 0, "JPML2022");
    }));

    c.bench_function("jpml2022 scoring #4", |b| b.iter(|| {
        let _ = fivesou::score_hand_from_str("p2,p2,we,we", "m8,m8,m8|p3,p3,p3|s8,s8,s8", "p2", 'e', 'e', 't', "", "", 0, "JPML2022");
        let _ = fivesou::score_hand_from_str("p2,p2,we,we", "m8,m8,m8|p3,p3,p3|s8,s8,s8", "p2", 'e', 'e', 'r', "", "", 0, "JPML2022");
        let _ = fivesou::score_hand_from_str("p2,p2,we,we", "m8,m8,m8|p3,p3,p3|s8,s8,s8", "p2", 's', 'e', 't', "", "", 0, "JPML2022");
        let _ = fivesou::score_hand_from_str("p2,p2,we,we", "m8,m8,m8|p3,p3,p3|s8,s8,s8", "p2", 's', 'e', 'r', "", "", 0, "JPML2022");
    }));

    c.bench_function("jpml2022 scoring #5", |b| b.iter(|| {
        let _ = fivesou::score_hand_from_str("p2,p3,p4,p5,p6,p7,p7,p7,we,we", "ws,ws,ws", "p1", 'e', 'e', 't', "", "", 0, "JPML2022");
        let _ = fivesou::score_hand_from_str("p2,p3,p4,p5,p6,p7,p7,p7,we,we", "ws,ws,ws", "p1", 'e', 'e', 'r', "", "", 0, "JPML2022");
        let _ = fivesou::score_hand_from_str("p2,p3,p4,p5,p6,p7,p7,p7,we,we", "ws,ws,ws", "p1", 's', 'e', 't', "", "", 0, "JPML2022");
        let _ = fivesou::score_hand_from_str("p2,p3,p4,p5,p6,p7,p7,p7,we,we", "ws,ws,ws", "p1", 's', 'e', 'r', "", "", 0, "JPML2022");
    }));

    c.bench_function("jpml2022 scoring #6", |b| b.iter(|| {
        let _ = fivesou::score_hand_from_str("p3,p3,p4,p4,p5,p5,p2", "s8,s8,s8|!s7,s7,s7,s7", "p2", 'e', 'e', 't', "", "", 0, "JPML2022");
        let _ = fivesou::score_hand_from_str("p3,p3,p4,p4,p5,p5,p2", "s8,s8,s8|!s7,s7,s7,s7", "p2", 'e', 'e', 'r', "", "", 0, "JPML2022");
        let _ = fivesou::score_hand_from_str("p3,p3,p4,p4,p5,p5,p2", "s8,s8,s8|!s7,s7,s7,s7", "p2", 's', 'e', 't', "", "", 0, "JPML2022");
        let _ = fivesou::score_hand_from_str("p3,p3,p4,p4,p5,p5,p2", "s8,s8,s8|!s7,s7,s7,s7", "p2", 's', 'e', 'r', "", "", 0, "JPML2022");
    }));

    c.bench_function("jpml2022 scoring #7", |b| b.iter(|| {
        let _ = fivesou::score_hand_from_str("m2,m2,m4,m4,m3,s7,s7,s7,ws,ws", "!wn,wn,wn,wn", "m3", 'e', 'e', 't', "", "", 0, "JPML2022");
        let _ = fivesou::score_hand_from_str("m2,m2,m4,m4,m3,s7,s7,s7,ws,ws", "!wn,wn,wn,wn", "m3", 'e', 'e', 'r', "", "", 0, "JPML2022");
        let _ = fivesou::score_hand_from_str("m2,m2,m4,m4,m3,s7,s7,s7,ws,ws", "!wn,wn,wn,wn", "m3", 's', 'e', 't', "", "", 0, "JPML2022");
        let _ = fivesou::score_hand_from_str("m2,m2,m4,m4,m3,s7,s7,s7,ws,ws", "!wn,wn,wn,wn", "m3", 's', 'e', 'r', "", "", 0, "JPML2022");
    }));

    c.bench_function("jpml2022 scoring #8", |b| b.iter(|| {
        let _ = fivesou::score_hand_from_str("s1,s1,s1,s2,s4,we,we", "m9,m9,m9|!dr,dr,dr,dr", "s3", 'e', 'e', 't', "", "", 0, "JPML2022");
        let _ = fivesou::score_hand_from_str("s1,s1,s1,s2,s4,we,we", "m9,m9,m9|!dr,dr,dr,dr", "s3", 'e', 'e', 'r', "", "", 0, "JPML2022");
        let _ = fivesou::score_hand_from_str("s1,s1,s1,s2,s4,we,we", "m9,m9,m9|!dr,dr,dr,dr", "s3", 's', 'e', 't', "", "", 0, "JPML2022");
        let _ = fivesou::score_hand_from_str("s1,s1,s1,s2,s4,we,we", "m9,m9,m9|!dr,dr,dr,dr", "s3", 's', 'e', 'r', "", "", 0, "JPML2022");
    }));

    c.bench_function("jpml2022 scoring #9", |b| b.iter(|| {
        let _ = fivesou::score_hand_from_str("m7,m8,m9,p7,p8,p8,p8", "!ws,ws,ws,ws|!dg,dg,dg,dg", "p9", 'e', 'e', 't', "", "", 0, "JPML2022");
        let _ = fivesou::score_hand_from_str("m7,m8,m9,p7,p8,p8,p8", "!ws,ws,ws,ws|!dg,dg,dg,dg", "p9", 'e', 'e', 'r', "", "", 0, "JPML2022");
        let _ = fivesou::score_hand_from_str("m7,m8,m9,p7,p8,p8,p8", "!ws,ws,ws,ws|!dg,dg,dg,dg", "p9", 's', 'e', 't', "", "", 0, "JPML2022");
        let _ = fivesou::score_hand_from_str("m7,m8,m9,p7,p8,p8,p8", "!ws,ws,ws,ws|!dg,dg,dg,dg", "p9", 's', 'e', 'r', "", "", 0, "JPML2022");
    }));

    c.bench_function("jpml2022 scoring #10", |b| b.iter(|| {
        let _ = fivesou::score_hand_from_str("m2,m3,m4,m4,m5,m6,m7,s8,s8,s8", "we,we,we,we", "m1", 'e', 'e', 't', "", "rinshan", 0, "JPML2022");
        let _ = fivesou::score_hand_from_str("m2,m3,m4,m4,m5,m6,m7,s8,s8,s8", "we,we,we,we", "m1", 'e', 'e', 'r', "", "", 0, "JPML2022");
        let _ = fivesou::score_hand_from_str("m2,m3,m4,m4,m5,m6,m7,s8,s8,s8", "we,we,we,we", "m1", 's', 'e', 't', "", "rinshan", 0, "JPML2022");
        let _ = fivesou::score_hand_from_str("m2,m3,m4,m4,m5,m6,m7,s8,s8,s8", "we,we,we,we", "m1", 's', 'e', 'r', "", "", 0, "JPML2022");
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);