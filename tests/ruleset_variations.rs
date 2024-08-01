use fivesou::scoring::Payment;

#[test]
fn jpml_pro_test_2022(){
    // test cases sourced from the 2022 JPML pro test:
    // https://cloudymahjong.com/wp-content/uploads/2023/12/JPML-Pro-Test-Questions-2022.pdf
    // https://cloudymahjong.com/wp-content/uploads/2023/12/JPML-Pro-Test-Answers-2022.pdf
    // while it doesn't hit everything that needs to be tested, it's full of tricky edge cases

    let ruleset = "JPML2022";

    // #1
    assert_eq!(fivesou::score_hand_from_str("p1,p2,p3,p4,p4,p4,p5,p6,p7,p8,s2,s3,s4", "", "p9", 'e', 'e', 't', "", "", 0, ruleset), Ok(Payment::DealerTsumo(2600)));
    assert_eq!(fivesou::score_hand_from_str("p1,p2,p3,p4,p4,p4,p5,p6,p7,p8,s2,s3,s4", "", "p9", 'e', 'e', 'r', "", "", 0, ruleset), Ok(Payment::Ron(5800)));
    assert_eq!(fivesou::score_hand_from_str("p1,p2,p3,p4,p4,p4,p5,p6,p7,p8,s2,s3,s4", "", "p9", 's', 'e', 't', "", "", 0, ruleset), Ok(Payment::Tsumo{dealer: 2600, non_dealer: 1300}));
    assert_eq!(fivesou::score_hand_from_str("p1,p2,p3,p4,p4,p4,p5,p6,p7,p8,s2,s3,s4", "", "p9", 's', 'e', 'r', "", "", 0, ruleset), Ok(Payment::Ron(3900)));

    // #2
    assert_eq!(fivesou::score_hand_from_str("m2,m2,m3,m3,p3,p3,p5,p5,s6,s6,s7,s8,s8", "", "s7", 'e', 'e', 't', "", "", 0, ruleset), Ok(Payment::DealerTsumo(3200)));
    assert_eq!(fivesou::score_hand_from_str("m2,m2,m3,m3,p3,p3,p5,p5,s6,s6,s7,s8,s8", "", "s7", 'e', 'e', 'r', "", "", 0, ruleset), Ok(Payment::Ron(4800)));
    assert_eq!(fivesou::score_hand_from_str("m2,m2,m3,m3,p3,p3,p5,p5,s6,s6,s7,s8,s8", "", "s7", 's', 'e', 't', "", "", 0, ruleset), Ok(Payment::Tsumo{dealer: 3200, non_dealer: 1600}));
    assert_eq!(fivesou::score_hand_from_str("m2,m2,m3,m3,p3,p3,p5,p5,s6,s6,s7,s8,s8", "", "s7", 's', 'e', 'r', "", "", 0, ruleset), Ok(Payment::Ron(3200)));

    // #3
    assert_eq!(fivesou::score_hand_from_str("m3,m5,m6,m7,m8,m8,m8", "p8,p8,p8|m2,m2,m2", "m3", 'e', 'e', 't', "", "", 0, ruleset), Ok(Payment::DealerTsumo(700)));
    assert_eq!(fivesou::score_hand_from_str("m3,m5,m6,m7,m8,m8,m8", "p8,p8,p8|m2,m2,m2", "m3", 'e', 'e', 'r', "", "", 0, ruleset), Ok(Payment::Ron(1500)));
    assert_eq!(fivesou::score_hand_from_str("m3,m5,m6,m7,m8,m8,m8", "p8,p8,p8|m2,m2,m2", "m3", 's', 'e', 't', "", "", 0, ruleset), Ok(Payment::Tsumo{dealer: 700, non_dealer: 400}));
    assert_eq!(fivesou::score_hand_from_str("m3,m5,m6,m7,m8,m8,m8", "p8,p8,p8|m2,m2,m2", "m3", 's', 'e', 'r', "", "", 0, ruleset), Ok(Payment::Ron(1000)));

    // #4
    assert_eq!(fivesou::score_hand_from_str("p2,p2,we,we", "m8,m8,m8|p3,p3,p3|s8,s8,s8", "p2", 'e', 'e', 't', "", "", 0, ruleset), Ok(Payment::DealerTsumo(1300)));
    assert_eq!(fivesou::score_hand_from_str("p2,p2,we,we", "m8,m8,m8|p3,p3,p3|s8,s8,s8", "p2", 'e', 'e', 'r', "", "", 0, ruleset), Ok(Payment::Ron(3900)));
    assert_eq!(fivesou::score_hand_from_str("p2,p2,we,we", "m8,m8,m8|p3,p3,p3|s8,s8,s8", "p2", 's', 'e', 't', "", "", 0, ruleset), Ok(Payment::Tsumo{dealer: 1300, non_dealer: 700}));
    assert_eq!(fivesou::score_hand_from_str("p2,p2,we,we", "m8,m8,m8|p3,p3,p3|s8,s8,s8", "p2", 's', 'e', 'r', "", "", 0, ruleset), Ok(Payment::Ron(2000)));

    // #5
    assert_eq!(fivesou::score_hand_from_str("p2,p3,p4,p5,p6,p7,p7,p7,we,we", "ws,ws,ws", "p1", 'e', 'e', 't', "", "", 0, ruleset), Ok(Payment::DealerTsumo(1300)));
    assert_eq!(fivesou::score_hand_from_str("p2,p3,p4,p5,p6,p7,p7,p7,we,we", "ws,ws,ws", "p1", 'e', 'e', 'r', "", "", 0, ruleset), Ok(Payment::Ron(3900)));
    assert_eq!(fivesou::score_hand_from_str("p2,p3,p4,p5,p6,p7,p7,p7,we,we", "ws,ws,ws", "p1", 's', 'e', 't', "", "", 0, ruleset), Ok(Payment::Tsumo{dealer: 2600, non_dealer: 1300}));
    assert_eq!(fivesou::score_hand_from_str("p2,p3,p4,p5,p6,p7,p7,p7,we,we", "ws,ws,ws", "p1", 's', 'e', 'r', "", "", 0, ruleset), Ok(Payment::Ron(3900)));

    // #6
    assert_eq!(fivesou::score_hand_from_str("p3,p3,p4,p4,p5,p5,p2", "s8,s8,s8|!s7,s7,s7,s7", "p2", 'e', 'e', 't', "", "", 0, ruleset), Ok(Payment::DealerTsumo(800)));
    assert_eq!(fivesou::score_hand_from_str("p3,p3,p4,p4,p5,p5,p2", "s8,s8,s8|!s7,s7,s7,s7", "p2", 'e', 'e', 'r', "", "", 0, ruleset), Ok(Payment::Ron(2000)));
    assert_eq!(fivesou::score_hand_from_str("p3,p3,p4,p4,p5,p5,p2", "s8,s8,s8|!s7,s7,s7,s7", "p2", 's', 'e', 't', "", "", 0, ruleset), Ok(Payment::Tsumo{dealer: 800, non_dealer: 400}));
    assert_eq!(fivesou::score_hand_from_str("p3,p3,p4,p4,p5,p5,p2", "s8,s8,s8|!s7,s7,s7,s7", "p2", 's', 'e', 'r', "", "", 0, ruleset), Ok(Payment::Ron(1300)));

    // #7
    assert_eq!(fivesou::score_hand_from_str("m2,m2,m4,m4,m3,s7,s7,s7,ws,ws", "!wn,wn,wn,wn", "m3", 'e', 'e', 't', "", "", 0, ruleset), Ok(Payment::DealerTsumo(2000)));
    assert_eq!(fivesou::score_hand_from_str("m2,m2,m4,m4,m3,s7,s7,s7,ws,ws", "!wn,wn,wn,wn", "m3", 'e', 'e', 'r', "", "", 0, ruleset), Ok(Payment::Ron(3400)));
    assert_eq!(fivesou::score_hand_from_str("m2,m2,m4,m4,m3,s7,s7,s7,ws,ws", "!wn,wn,wn,wn", "m3", 's', 'e', 't', "", "", 0, ruleset), Ok(Payment::Tsumo{dealer: 2300, non_dealer: 1200}));
    assert_eq!(fivesou::score_hand_from_str("m2,m2,m4,m4,m3,s7,s7,s7,ws,ws", "!wn,wn,wn,wn", "m3", 's', 'e', 'r', "", "", 0, ruleset), Ok(Payment::Ron(2300)));

    // #8
    assert_eq!(fivesou::score_hand_from_str("s1,s1,s1,s2,s4,we,we", "m9,m9,m9|!dr,dr,dr,dr", "s3", 'e', 'e', 't', "", "", 0, ruleset), Ok(Payment::DealerTsumo(1300)));
    assert_eq!(fivesou::score_hand_from_str("s1,s1,s1,s2,s4,we,we", "m9,m9,m9|!dr,dr,dr,dr", "s3", 'e', 'e', 'r', "", "", 0, ruleset), Ok(Payment::Ron(3400)));
    assert_eq!(fivesou::score_hand_from_str("s1,s1,s1,s2,s4,we,we", "m9,m9,m9|!dr,dr,dr,dr", "s3", 's', 'e', 't', "", "", 0, ruleset), Ok(Payment::Tsumo{dealer: 1200, non_dealer: 600}));
    assert_eq!(fivesou::score_hand_from_str("s1,s1,s1,s2,s4,we,we", "m9,m9,m9|!dr,dr,dr,dr", "s3", 's', 'e', 'r', "", "", 0, ruleset), Ok(Payment::Ron(2300)));

    // #9
    assert_eq!(fivesou::score_hand_from_str("m7,m8,m9,p7,p8,p8,p8", "!ws,ws,ws,ws|!dg,dg,dg,dg", "p9", 'e', 'e', 't', "", "", 0, ruleset), Ok(Payment::DealerTsumo(2900)));
    assert_eq!(fivesou::score_hand_from_str("m7,m8,m9,p7,p8,p8,p8", "!ws,ws,ws,ws|!dg,dg,dg,dg", "p9", 'e', 'e', 'r', "", "", 0, ruleset), Ok(Payment::Ron(4800)));
    assert_eq!(fivesou::score_hand_from_str("m7,m8,m9,p7,p8,p8,p8", "!ws,ws,ws,ws|!dg,dg,dg,dg", "p9", 's', 'e', 't', "", "", 0, ruleset), Ok(Payment::Tsumo{dealer: 4000, non_dealer: 2000}));
    assert_eq!(fivesou::score_hand_from_str("m7,m8,m9,p7,p8,p8,p8", "!ws,ws,ws,ws|!dg,dg,dg,dg", "p9", 's', 'e', 'r', "", "", 0, ruleset), Ok(Payment::Ron(6400)));

    // #10
    assert_eq!(fivesou::score_hand_from_str("m2,m3,m4,m4,m5,m6,m7,s8,s8,s8", "we,we,we,we", "m1", 'e', 'e', 't', "", "rinshan", 0, ruleset), Ok(Payment::DealerTsumo(2600)));
    assert_eq!(fivesou::score_hand_from_str("m2,m3,m4,m4,m5,m6,m7,s8,s8,s8", "we,we,we,we", "m1", 'e', 'e', 'r', "", "", 0, ruleset), Ok(Payment::Ron(3900)));
    assert_eq!(fivesou::score_hand_from_str("m2,m3,m4,m4,m5,m6,m7,s8,s8,s8", "we,we,we,we", "m1", 's', 'e', 't', "", "rinshan", 0, ruleset), Ok(Payment::Tsumo{dealer: 1300, non_dealer: 700}));
    assert_eq!(fivesou::score_hand_from_str("m2,m3,m4,m4,m5,m6,m7,s8,s8,s8", "we,we,we,we", "m1", 's', 'e', 'r', "", "", 0, ruleset), Ok(Payment::Ron(1300)));
}

#[test]
fn jpml_pro_test_2023(){
    // test cases sourced from the 2023 JPML pro test:
    // https://cloudymahjong.com/wp-content/uploads/2024/06/JPML-Pro-Test-2023-40th-Season-Questions.pdf
    // https://cloudymahjong.com/wp-content/uploads/2024/06/JPML-Pro-Test-2023-40th-Season-Answers.pdf

    let ruleset = "JPML2023";

    // #1
    assert_eq!(fivesou::score_hand_from_str("p6,p7,p8,s1,s1,s2,s2,s2,s3,s3,s3,we,we", "", "s1", 'e', 'e', 't', "", "", 0, ruleset), Ok(Payment::DealerTsumo(2600)));
    assert_eq!(fivesou::score_hand_from_str("p6,p7,p8,s1,s1,s2,s2,s2,s3,s3,s3,we,we", "", "s1", 'e', 'e', 'r', "", "", 0, ruleset), Ok(Payment::Ron(2000)));
    assert_eq!(fivesou::score_hand_from_str("p6,p7,p8,s1,s1,s2,s2,s2,s3,s3,s3,we,we", "", "s1", 's', 'e', 't', "", "", 0, ruleset), Ok(Payment::Tsumo{dealer: 2600, non_dealer: 1300}));
    assert_eq!(fivesou::score_hand_from_str("p6,p7,p8,s1,s1,s2,s2,s2,s3,s3,s3,we,we", "", "s1", 's', 'e', 'r', "", "", 0, ruleset), Ok(Payment::Ron(1300)));

    assert_eq!(fivesou::score_hand_from_str("p6,p7,p8,s1,s1,s2,s2,s2,s3,s3,s3,we,we", "", "s4", 'e', 'e', 't', "", "", 0, ruleset), Ok(Payment::DealerTsumo(1000)));
    assert_eq!(fivesou::score_hand_from_str("p6,p7,p8,s1,s1,s2,s2,s2,s3,s3,s3,we,we", "", "s4", 'e', 'e', 'r', "", "", 0, ruleset), Ok(Payment::Ron(2000)));
    assert_eq!(fivesou::score_hand_from_str("p6,p7,p8,s1,s1,s2,s2,s2,s3,s3,s3,we,we", "", "s4", 's', 'e', 't', "", "", 0, ruleset), Ok(Payment::Tsumo{dealer: 1000, non_dealer: 500}));
    assert_eq!(fivesou::score_hand_from_str("p6,p7,p8,s1,s1,s2,s2,s2,s3,s3,s3,we,we", "", "s4", 's', 'e', 'r', "", "", 0, ruleset), Ok(Payment::Ron(1300)));

    assert_eq!(fivesou::score_hand_from_str("p6,p7,p8,s1,s1,s2,s2,s2,s3,s3,s3,we,we", "", "we", 'e', 'e', 't', "", "", 0, ruleset), Ok(Payment::DealerTsumo(4000)));
    assert_eq!(fivesou::score_hand_from_str("p6,p7,p8,s1,s1,s2,s2,s2,s3,s3,s3,we,we", "", "we", 'e', 'e', 'r', "", "", 0, ruleset), Ok(Payment::Ron(4800)));
    assert_eq!(fivesou::score_hand_from_str("p6,p7,p8,s1,s1,s2,s2,s2,s3,s3,s3,we,we", "", "we", 's', 'e', 't', "", "", 0, ruleset), Ok(Payment::Tsumo{dealer: 4000, non_dealer: 2000}));
    assert_eq!(fivesou::score_hand_from_str("p6,p7,p8,s1,s1,s2,s2,s2,s3,s3,s3,we,we", "", "we", 's', 'e', 'r', "", "", 0, ruleset), Ok(Payment::Ron(1600)));

    // #2
    assert_eq!(fivesou::score_hand_from_str("m7,m7,p5,p6,p7,p7,p8,p8,p9,p9,dg,dg,dg", "", "p7", 'e', 'e', 't', "", "", 0, ruleset), Ok(Payment::DealerTsumo(2600)));
    assert_eq!(fivesou::score_hand_from_str("m7,m7,p5,p6,p7,p7,p8,p8,p9,p9,dg,dg,dg", "", "p7", 'e', 'e', 'r', "", "", 0, ruleset), Ok(Payment::Ron(3900)));
    assert_eq!(fivesou::score_hand_from_str("m7,m7,p5,p6,p7,p7,p8,p8,p9,p9,dg,dg,dg", "", "p7", 's', 'e', 't', "", "", 0, ruleset), Ok(Payment::Tsumo{dealer: 2600, non_dealer: 1300}));
    assert_eq!(fivesou::score_hand_from_str("m7,m7,p5,p6,p7,p7,p8,p8,p9,p9,dg,dg,dg", "", "p7", 's', 'e', 'r', "", "", 0, ruleset), Ok(Payment::Ron(2600)));

    // #3
    assert_eq!(fivesou::score_hand_from_str("s1,s2,s4,s5,s6,s7,s8,s9,ws,ws", "wn,wn,wn,wn", "s3", 'e', 'e', 't', "", "", 0, ruleset), Ok(Payment::DealerTsumo(2600)));
    assert_eq!(fivesou::score_hand_from_str("s1,s2,s4,s5,s6,s7,s8,s9,ws,ws", "wn,wn,wn,wn", "s3", 'e', 'e', 'r', "", "", 0, ruleset), Ok(Payment::Ron(7700)));
    assert_eq!(fivesou::score_hand_from_str("s1,s2,s4,s5,s6,s7,s8,s9,ws,ws", "wn,wn,wn,wn", "s3", 's', 'e', 't', "", "", 0, ruleset), Ok(Payment::Tsumo{dealer: 3200, non_dealer: 1600}));
    assert_eq!(fivesou::score_hand_from_str("s1,s2,s4,s5,s6,s7,s8,s9,ws,ws", "wn,wn,wn,wn", "s3", 's', 'e', 'r', "", "", 0, ruleset), Ok(Payment::Ron(5200)));

    // #4
    assert_eq!(fivesou::score_hand_from_str("m5,m5,m5,s3,s3,s3,s5,s6,s7,s8", "!p2,p2,p2,p2", "s4", 'e', 'e', 't', "", "", 0, ruleset), Ok(Payment::DealerTsumo(1600)));
    assert_eq!(fivesou::score_hand_from_str("m5,m5,m5,s3,s3,s3,s5,s6,s7,s8", "!p2,p2,p2,p2", "s4", 'e', 'e', 'r', "", "", 0, ruleset), Ok(Payment::Ron(2900)));
    assert_eq!(fivesou::score_hand_from_str("m5,m5,m5,s3,s3,s3,s5,s6,s7,s8", "!p2,p2,p2,p2", "s4", 's', 'e', 't', "", "", 0, ruleset), Ok(Payment::Tsumo{dealer: 1600, non_dealer: 800}));
    assert_eq!(fivesou::score_hand_from_str("m5,m5,m5,s3,s3,s3,s5,s6,s7,s8", "!p2,p2,p2,p2", "s4", 's', 'e', 'r', "", "", 0, ruleset), Ok(Payment::Ron(2000)));

    // #5
    assert_eq!(fivesou::score_hand_from_str("m2,m3,p5,p5", "ws,ws,ws|dg,dg,dg|!s1,s1,s1,s1", "m4", 'e', 'e', 't', "", "", 0, ruleset), Ok(Payment::DealerTsumo(1200)));
    assert_eq!(fivesou::score_hand_from_str("m2,m3,p5,p5", "ws,ws,ws|dg,dg,dg|!s1,s1,s1,s1", "m4", 'e', 'e', 'r', "", "", 0, ruleset), Ok(Payment::Ron(2900)));
    assert_eq!(fivesou::score_hand_from_str("m2,m3,p5,p5", "ws,ws,ws|dg,dg,dg|!s1,s1,s1,s1", "m4", 's', 'e', 't', "", "", 0, ruleset), Ok(Payment::Tsumo{dealer: 2300, non_dealer: 1200}));
    assert_eq!(fivesou::score_hand_from_str("m2,m3,p5,p5", "ws,ws,ws|dg,dg,dg|!s1,s1,s1,s1", "m4", 's', 'e', 'r', "", "", 0, ruleset), Ok(Payment::Ron(3900)));

    // #6
    assert_eq!(fivesou::score_hand_from_str("p5,p7,p7,p8,p9,we,we", "!m1,m1,m1,m1|!dr,dr,dr,dr", "p6", 'e', 'e', 't', "", "", 0, ruleset), Ok(Payment::DealerTsumo(2900)));
    assert_eq!(fivesou::score_hand_from_str("p5,p7,p7,p8,p9,we,we", "!m1,m1,m1,m1|!dr,dr,dr,dr", "p6", 'e', 'e', 'r', "", "", 0, ruleset), Ok(Payment::Ron(4800)));
    assert_eq!(fivesou::score_hand_from_str("p5,p7,p7,p8,p9,we,we", "!m1,m1,m1,m1|!dr,dr,dr,dr", "p6", 's', 'e', 't', "", "", 0, ruleset), Ok(Payment::Tsumo{dealer: 2900, non_dealer: 1500}));
    assert_eq!(fivesou::score_hand_from_str("p5,p7,p7,p8,p9,we,we", "!m1,m1,m1,m1|!dr,dr,dr,dr", "p6", 's', 'e', 'r', "", "", 0, ruleset), Ok(Payment::Ron(3200)));

    // #7
    assert_eq!(fivesou::score_hand_from_str("s2,s2,s3,s3,s4,s4,s4,s6,s6,s6,s8,s8,s8", "", "s2", 'e', 'e', 't', "", "", 0, ruleset), Ok(Payment::DealerTsumo(32000)));
    assert_eq!(fivesou::score_hand_from_str("s2,s2,s3,s3,s4,s4,s4,s6,s6,s6,s8,s8,s8", "", "s2", 'e', 'e', 'r', "", "", 0, ruleset), Ok(Payment::Ron(48000)));
    assert_eq!(fivesou::score_hand_from_str("s2,s2,s3,s3,s4,s4,s4,s6,s6,s6,s8,s8,s8", "", "s2", 's', 'e', 't', "", "", 0, ruleset), Ok(Payment::Tsumo{dealer: 32000, non_dealer: 16000}));
    assert_eq!(fivesou::score_hand_from_str("s2,s2,s3,s3,s4,s4,s4,s6,s6,s6,s8,s8,s8", "", "s2", 's', 'e', 'r', "", "", 0, ruleset), Ok(Payment::Ron(32000)));

    // #8
    assert_eq!(fivesou::score_hand_from_str("m7,m9,m9,m9,s9,s9,s9", "ws,ws,ws,ws|s9,s9,s9", "m8", 'e', 'e', 't', "", "rinshan", 0, ruleset), Ok(Payment::DealerTsumo(2000)));
    assert_eq!(fivesou::score_hand_from_str("m7,m9,m9,m9,s9,s9,s9", "ws,ws,ws,ws|s9,s9,s9", "m8", 'e', 'e', 'r', "", "", 0, ruleset), Ok(Payment::Ron(2400)));
    assert_eq!(fivesou::score_hand_from_str("m7,m9,m9,m9,s9,s9,s9", "ws,ws,ws,ws|s9,s9,s9", "m8", 's', 'e', 't', "", "rinshan", 0, ruleset), Ok(Payment::Tsumo{dealer: 3900, non_dealer: 2000}));
    assert_eq!(fivesou::score_hand_from_str("m7,m9,m9,m9,s9,s9,s9", "ws,ws,ws,ws|s9,s9,s9", "m8", 's', 'e', 'r', "", "", 0, ruleset), Ok(Payment::Ron(3200)));
}

#[test]
fn wrc2022(){
    
}

#[test]
fn ema2016(){
    
}

#[test]
fn majsoul(){
    
}