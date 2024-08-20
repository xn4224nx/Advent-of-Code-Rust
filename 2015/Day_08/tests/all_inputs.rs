#[allow(dead_code)]
#[allow(unused_variables)]
#[path = "../src/main.rs"]
mod day_08;
use day_08::{encoded_str_len, parse_data, raw_str_len, str_len};

#[test]
fn str_len_ex_0() {
    assert_eq!(raw_str_len(&r#""azlgxdbljwygyttzkfwuxv""#.to_string()), 24);
    assert_eq!(str_len(&r#""azlgxdbljwygyttzkfwuxv""#.to_string()), 22);
    assert_eq!(
        encoded_str_len(&r#""azlgxdbljwygyttzkfwuxv""#.to_string()),
        28
    );
}

#[test]
fn str_len_ex_1() {
    assert_eq!(
        raw_str_len(&r#""v\xfb\"lgs\"kvjfywmut\x9cr""#.to_string()),
        28
    );
    assert_eq!(str_len(&r#""v\xfb\"lgs\"kvjfywmut\x9cr""#.to_string()), 18);
    assert_eq!(
        encoded_str_len(&r#""v\xfb\"lgs\"kvjfywmut\x9cr""#.to_string()),
        38
    );
}

#[test]
fn str_len_ex_2() {
    assert_eq!(raw_str_len(&r#""merxdhj""#.to_string()), 9);
    assert_eq!(str_len(&r#""merxdhj""#.to_string()), 7);
    assert_eq!(encoded_str_len(&r#""merxdhj""#.to_string()), 13);
}

#[test]
fn str_len_ex_3() {
    assert_eq!(raw_str_len(&r#""dwz""#.to_string()), 5);
    assert_eq!(str_len(&r#""dwz""#.to_string()), 3);
    assert_eq!(encoded_str_len(&r#""dwz""#.to_string()), 9);
}

#[test]
fn str_len_ex_4() {
    assert_eq!(raw_str_len(&r#""d\\gkbqo\\fwukyxab\"u""#.to_string()), 23);
    assert_eq!(str_len(&r#""d\\gkbqo\\fwukyxab\"u""#.to_string()), 18);
    assert_eq!(
        encoded_str_len(&r#""d\\gkbqo\\fwukyxab\"u""#.to_string()),
        33
    );
}

#[test]
fn str_len_ex_5() {
    assert_eq!(
        raw_str_len(&r#""k\xd4cfixejvkicryipucwurq\x7eq""#.to_string()),
        32
    );
    assert_eq!(
        str_len(&r#""k\xd4cfixejvkicryipucwurq\x7eq""#.to_string()),
        24
    );
    assert_eq!(
        encoded_str_len(&r#""k\xd4cfixejvkicryipucwurq\x7eq""#.to_string()),
        38
    );
}

#[test]
fn str_len_ex_6() {
    assert_eq!(raw_str_len(&r#""nvtidemacj\"hppfopvpr""#.to_string()), 23);
    assert_eq!(str_len(&r#""nvtidemacj\"hppfopvpr""#.to_string()), 20);
    assert_eq!(
        encoded_str_len(&r#""nvtidemacj\"hppfopvpr""#.to_string()),
        29
    );
}

#[test]
fn str_len_ex_7() {
    assert_eq!(
        raw_str_len(&r#""kbngyfvvsdismznhar\\p\"\"gpryt\"jaeh""#.to_string()),
        38
    );
    assert_eq!(
        str_len(&r#""kbngyfvvsdismznhar\\p\"\"gpryt\"jaeh""#.to_string()),
        32
    );
    assert_eq!(
        encoded_str_len(&r#""kbngyfvvsdismznhar\\p\"\"gpryt\"jaeh""#.to_string()),
        50
    );
}

#[test]
fn str_len_ex_8() {
    assert_eq!(raw_str_len(&r#""khre\"o\x0elqfrbktzn""#.to_string()), 22);
    assert_eq!(str_len(&r#""khre\"o\x0elqfrbktzn""#.to_string()), 16);
    assert_eq!(
        encoded_str_len(&r#""khre\"o\x0elqfrbktzn""#.to_string()),
        29
    );
}

#[test]
fn str_len_ex_9() {
    assert_eq!(
        raw_str_len(&r#""nugkdmqwdq\x50amallrskmrxoyo""#.to_string()),
        30
    );
    assert_eq!(
        str_len(&r#""nugkdmqwdq\x50amallrskmrxoyo""#.to_string()),
        25
    );
    assert_eq!(
        encoded_str_len(&r#""nugkdmqwdq\x50amallrskmrxoyo""#.to_string()),
        35
    );
}

#[test]
fn str_len_ex_10() {
    assert_eq!(
        raw_str_len(&r#""jcrkptrsasjp\\\"cwigzynjgspxxv\\vyb""#.to_string()),
        37
    );
    assert_eq!(
        str_len(&r#""jcrkptrsasjp\\\"cwigzynjgspxxv\\vyb""#.to_string()),
        32
    );
    assert_eq!(
        encoded_str_len(&r#""jcrkptrsasjp\\\"cwigzynjgspxxv\\vyb""#.to_string()),
        47
    );
}

#[test]
fn str_len_ex_11() {
    assert_eq!(
        raw_str_len(&r#""ramf\"skhcmenhbpujbqwkltmplxygfcy""#.to_string()),
        35
    );
    assert_eq!(
        str_len(&r#""ramf\"skhcmenhbpujbqwkltmplxygfcy""#.to_string()),
        32
    );
    assert_eq!(
        encoded_str_len(&r#""ramf\"skhcmenhbpujbqwkltmplxygfcy""#.to_string()),
        41
    );
}

#[test]
fn str_len_ex_12() {
    assert_eq!(
        raw_str_len(&r#""aqjqgbfqaxga\\fkdcahlfi\"pvods""#.to_string()),
        32
    );
    assert_eq!(
        str_len(&r#""aqjqgbfqaxga\\fkdcahlfi\"pvods""#.to_string()),
        28
    );
    assert_eq!(
        encoded_str_len(&r#""aqjqgbfqaxga\\fkdcahlfi\"pvods""#.to_string()),
        40
    );
}

#[test]
fn str_len_ex_13() {
    assert_eq!(raw_str_len(&r#""pcrtfb""#.to_string()), 8);
    assert_eq!(str_len(&r#""pcrtfb""#.to_string()), 6);
    assert_eq!(encoded_str_len(&r#""pcrtfb""#.to_string()), 12);
}

#[test]
fn str_len_ex_14() {
    assert_eq!(
        raw_str_len(&r#""\x83qg\"nwgugfmfpzlrvty\"ryoxm""#.to_string()),
        32
    );
    assert_eq!(
        str_len(&r#""\x83qg\"nwgugfmfpzlrvty\"ryoxm""#.to_string()),
        25
    );
    assert_eq!(
        encoded_str_len(&r#""\x83qg\"nwgugfmfpzlrvty\"ryoxm""#.to_string()),
        41
    );
}

#[test]
fn str_len_ex_15() {
    assert_eq!(raw_str_len(&r#""fvhvvokdnl\\eap""#.to_string()), 17);
    assert_eq!(str_len(&r#""fvhvvokdnl\\eap""#.to_string()), 14);
    assert_eq!(encoded_str_len(&r#""fvhvvokdnl\\eap""#.to_string()), 23);
}

#[test]
fn str_len_ex_16() {
    assert_eq!(raw_str_len(&r#""kugdkrat""#.to_string()), 10);
    assert_eq!(str_len(&r#""kugdkrat""#.to_string()), 8);
    assert_eq!(encoded_str_len(&r#""kugdkrat""#.to_string()), 14);
}

#[test]
fn str_len_ex_17() {
    assert_eq!(raw_str_len(&r#""seuxwc""#.to_string()), 8);
    assert_eq!(str_len(&r#""seuxwc""#.to_string()), 6);
    assert_eq!(encoded_str_len(&r#""seuxwc""#.to_string()), 12);
}

#[test]
fn str_len_ex_18() {
    assert_eq!(raw_str_len(&r#""vhioftcosshaqtnz""#.to_string()), 18);
    assert_eq!(str_len(&r#""vhioftcosshaqtnz""#.to_string()), 16);
    assert_eq!(encoded_str_len(&r#""vhioftcosshaqtnz""#.to_string()), 22);
}

#[test]
fn str_len_ex_19() {
    assert_eq!(raw_str_len(&r#""gzkxqrdq\\uko\"mrtst""#.to_string()), 22);
    assert_eq!(str_len(&r#""gzkxqrdq\\uko\"mrtst""#.to_string()), 18);
    assert_eq!(
        encoded_str_len(&r#""gzkxqrdq\\uko\"mrtst""#.to_string()),
        30
    );
}

#[test]
fn str_len_ex_20() {
    assert_eq!(raw_str_len(&r#""znjcomvy\x16hhsenmroswr""#.to_string()), 25);
    assert_eq!(str_len(&r#""znjcomvy\x16hhsenmroswr""#.to_string()), 20);
    assert_eq!(
        encoded_str_len(&r#""znjcomvy\x16hhsenmroswr""#.to_string()),
        30
    );
}

#[test]
fn str_len_ex_21() {
    assert_eq!(raw_str_len(&r#""clowmtra""#.to_string()), 10);
    assert_eq!(str_len(&r#""clowmtra""#.to_string()), 8);
    assert_eq!(encoded_str_len(&r#""clowmtra""#.to_string()), 14);
}

#[test]
fn str_len_ex_22() {
    assert_eq!(raw_str_len(&r#""\xc4""#.to_string()), 6);
    assert_eq!(str_len(&r#""\xc4""#.to_string()), 1);
    assert_eq!(encoded_str_len(&r#""\xc4""#.to_string()), 11);
}

#[test]
fn str_len_ex_23() {
    assert_eq!(raw_str_len(&r#""jpavsevmziklydtqqm""#.to_string()), 20);
    assert_eq!(str_len(&r#""jpavsevmziklydtqqm""#.to_string()), 18);
    assert_eq!(encoded_str_len(&r#""jpavsevmziklydtqqm""#.to_string()), 24);
}

#[test]
fn str_len_ex_24() {
    assert_eq!(
        raw_str_len(&r#""egxjqytcttr\\ecfedmmovkyn\"m""#.to_string()),
        30
    );
    assert_eq!(
        str_len(&r#""egxjqytcttr\\ecfedmmovkyn\"m""#.to_string()),
        26
    );
    assert_eq!(
        encoded_str_len(&r#""egxjqytcttr\\ecfedmmovkyn\"m""#.to_string()),
        38
    );
}

#[test]
fn str_len_ex_25() {
    assert_eq!(raw_str_len(&r#""mjulrvqgmsvmwf""#.to_string()), 16);
    assert_eq!(str_len(&r#""mjulrvqgmsvmwf""#.to_string()), 14);
    assert_eq!(encoded_str_len(&r#""mjulrvqgmsvmwf""#.to_string()), 20);
}

#[test]
fn str_len_ex_26() {
    assert_eq!(
        raw_str_len(&r#""o\\prxtlfbatxerhev\xf9hcl\x44rzmvklviv""#.to_string()),
        40
    );
    assert_eq!(
        str_len(&r#""o\\prxtlfbatxerhev\xf9hcl\x44rzmvklviv""#.to_string()),
        31
    );
    assert_eq!(
        encoded_str_len(&r#""o\\prxtlfbatxerhev\xf9hcl\x44rzmvklviv""#.to_string()),
        48
    );
}

#[test]
fn str_len_ex_27() {
    assert_eq!(
        raw_str_len(&r#""lregjexqaqgwloydxdsc\\o\"dnjfmjcu""#.to_string()),
        35
    );
    assert_eq!(
        str_len(&r#""lregjexqaqgwloydxdsc\\o\"dnjfmjcu""#.to_string()),
        31
    );
    assert_eq!(
        encoded_str_len(&r#""lregjexqaqgwloydxdsc\\o\"dnjfmjcu""#.to_string()),
        43
    );
}

#[test]
fn str_len_ex_28() {
    assert_eq!(
        raw_str_len(&r#""lnxluajtk\x8desue\\k\x7abhwokfhh""#.to_string()),
        34
    );
    assert_eq!(
        str_len(&r#""lnxluajtk\x8desue\\k\x7abhwokfhh""#.to_string()),
        25
    );
    assert_eq!(
        encoded_str_len(&r#""lnxluajtk\x8desue\\k\x7abhwokfhh""#.to_string()),
        42
    );
}

#[test]
fn str_len_ex_29() {
    assert_eq!(
        raw_str_len(&r#""wrssfvzzn\"llrysjgiu\"npjtdli""#.to_string()),
        31
    );
    assert_eq!(
        str_len(&r#""wrssfvzzn\"llrysjgiu\"npjtdli""#.to_string()),
        27
    );
    assert_eq!(
        encoded_str_len(&r#""wrssfvzzn\"llrysjgiu\"npjtdli""#.to_string()),
        39
    );
}

#[test]
fn str_len_ex_30() {
    assert_eq!(raw_str_len(&r#""\x67lwkks""#.to_string()), 11);
    assert_eq!(str_len(&r#""\x67lwkks""#.to_string()), 6);
    assert_eq!(encoded_str_len(&r#""\x67lwkks""#.to_string()), 16);
}

#[test]
fn str_len_ex_31() {
    assert_eq!(
        raw_str_len(&r#""bifw\"ybvmwiyi\"vhol\"vol\xd4""#.to_string()),
        31
    );
    assert_eq!(
        str_len(&r#""bifw\"ybvmwiyi\"vhol\"vol\xd4""#.to_string()),
        23
    );
    assert_eq!(
        encoded_str_len(&r#""bifw\"ybvmwiyi\"vhol\"vol\xd4""#.to_string()),
        42
    );
}

#[test]
fn str_len_ex_32() {
    assert_eq!(raw_str_len(&r#""aywdqhvtvcpvbewtwuyxrix""#.to_string()), 25);
    assert_eq!(str_len(&r#""aywdqhvtvcpvbewtwuyxrix""#.to_string()), 23);
    assert_eq!(
        encoded_str_len(&r#""aywdqhvtvcpvbewtwuyxrix""#.to_string()),
        29
    );
}

#[test]
fn str_len_ex_33() {
    assert_eq!(raw_str_len(&r#""gc\xd3\"caukdgfdywj""#.to_string()), 21);
    assert_eq!(str_len(&r#""gc\xd3\"caukdgfdywj""#.to_string()), 15);
    assert_eq!(encoded_str_len(&r#""gc\xd3\"caukdgfdywj""#.to_string()), 28);
}

#[test]
fn str_len_ex_34() {
    assert_eq!(raw_str_len(&r#""uczy\\fk""#.to_string()), 10);
    assert_eq!(str_len(&r#""uczy\\fk""#.to_string()), 7);
    assert_eq!(encoded_str_len(&r#""uczy\\fk""#.to_string()), 16);
}

#[test]
fn str_len_ex_35() {
    assert_eq!(
        raw_str_len(&r#""bnlxkjvl\x7docehufkj\\\"qoyhag""#.to_string()),
        32
    );
    assert_eq!(
        str_len(&r#""bnlxkjvl\x7docehufkj\\\"qoyhag""#.to_string()),
        25
    );
    assert_eq!(
        encoded_str_len(&r#""bnlxkjvl\x7docehufkj\\\"qoyhag""#.to_string()),
        41
    );
}

#[test]
fn str_len_ex_36() {
    assert_eq!(raw_str_len(&r#""bidsptalmoicyorbv\\""#.to_string()), 21);
    assert_eq!(str_len(&r#""bidsptalmoicyorbv\\""#.to_string()), 18);
    assert_eq!(encoded_str_len(&r#""bidsptalmoicyorbv\\""#.to_string()), 27);
}

#[test]
fn str_len_ex_37() {
    assert_eq!(raw_str_len(&r#""jorscv\"mufcvvfmcv\"ga""#.to_string()), 24);
    assert_eq!(str_len(&r#""jorscv\"mufcvvfmcv\"ga""#.to_string()), 20);
    assert_eq!(
        encoded_str_len(&r#""jorscv\"mufcvvfmcv\"ga""#.to_string()),
        32
    );
}

#[test]
fn str_len_ex_38() {
    assert_eq!(raw_str_len(&r#""sofpwfal\\a""#.to_string()), 13);
    assert_eq!(str_len(&r#""sofpwfal\\a""#.to_string()), 10);
    assert_eq!(encoded_str_len(&r#""sofpwfal\\a""#.to_string()), 19);
}

#[test]
fn str_len_ex_39() {
    assert_eq!(raw_str_len(&r#""kcuqtbboaly\"uj\"k""#.to_string()), 20);
    assert_eq!(str_len(&r#""kcuqtbboaly\"uj\"k""#.to_string()), 16);
    assert_eq!(encoded_str_len(&r#""kcuqtbboaly\"uj\"k""#.to_string()), 28);
}

#[test]
fn str_len_ex_40() {
    assert_eq!(raw_str_len(&r#""n\\c""#.to_string()), 6);
    assert_eq!(str_len(&r#""n\\c""#.to_string()), 3);
    assert_eq!(encoded_str_len(&r#""n\\c""#.to_string()), 12);
}

#[test]
fn str_len_ex_41() {
    assert_eq!(raw_str_len(&r#""x\"\xcaj\\xwwvpdldz""#.to_string()), 21);
    assert_eq!(str_len(&r#""x\"\xcaj\\xwwvpdldz""#.to_string()), 14);
    assert_eq!(encoded_str_len(&r#""x\"\xcaj\\xwwvpdldz""#.to_string()), 30);
}

#[test]
fn str_len_ex_42() {
    assert_eq!(raw_str_len(&r#""eyukphh""#.to_string()), 9);
    assert_eq!(str_len(&r#""eyukphh""#.to_string()), 7);
    assert_eq!(encoded_str_len(&r#""eyukphh""#.to_string()), 13);
}

#[test]
fn str_len_ex_43() {
    assert_eq!(raw_str_len(&r#""wcyjq""#.to_string()), 7);
    assert_eq!(str_len(&r#""wcyjq""#.to_string()), 5);
    assert_eq!(encoded_str_len(&r#""wcyjq""#.to_string()), 11);
}

#[test]
fn str_len_ex_44() {
    assert_eq!(raw_str_len(&r#""vjx\"\"hjroj\"l\x4cjwbr""#.to_string()), 25);
    assert_eq!(str_len(&r#""vjx\"\"hjroj\"l\x4cjwbr""#.to_string()), 17);
    assert_eq!(
        encoded_str_len(&r#""vjx\"\"hjroj\"l\x4cjwbr""#.to_string()),
        36
    );
}

#[test]
fn str_len_ex_45() {
    assert_eq!(
        raw_str_len(&r#""xcodsxzfqw\\rowqtuwvjnxupjnrh""#.to_string()),
        31
    );
    assert_eq!(
        str_len(&r#""xcodsxzfqw\\rowqtuwvjnxupjnrh""#.to_string()),
        28
    );
    assert_eq!(
        encoded_str_len(&r#""xcodsxzfqw\\rowqtuwvjnxupjnrh""#.to_string()),
        37
    );
}

#[test]
fn str_len_ex_46() {
    assert_eq!(raw_str_len(&r#""yc""#.to_string()), 4);
    assert_eq!(str_len(&r#""yc""#.to_string()), 2);
    assert_eq!(encoded_str_len(&r#""yc""#.to_string()), 8);
}

#[test]
fn str_len_ex_47() {
    assert_eq!(raw_str_len(&r#""fpvzldgbdtca\"hqwa""#.to_string()), 20);
    assert_eq!(str_len(&r#""fpvzldgbdtca\"hqwa""#.to_string()), 17);
    assert_eq!(encoded_str_len(&r#""fpvzldgbdtca\"hqwa""#.to_string()), 26);
}

#[test]
fn str_len_ex_48() {
    assert_eq!(
        raw_str_len(&r#""ymjq\x8ahohvafubra\"hgqoknkuyph""#.to_string()),
        33
    );
    assert_eq!(
        str_len(&r#""ymjq\x8ahohvafubra\"hgqoknkuyph""#.to_string()),
        27
    );
    assert_eq!(
        encoded_str_len(&r#""ymjq\x8ahohvafubra\"hgqoknkuyph""#.to_string()),
        40
    );
}

#[test]
fn str_len_ex_49() {
    assert_eq!(raw_str_len(&r#""kx\\mkaaklvcup""#.to_string()), 16);
    assert_eq!(str_len(&r#""kx\\mkaaklvcup""#.to_string()), 13);
    assert_eq!(encoded_str_len(&r#""kx\\mkaaklvcup""#.to_string()), 22);
}

#[test]
fn str_len_ex_50() {
    assert_eq!(raw_str_len(&r#""belddrzegcsxsyfhzyz""#.to_string()), 21);
    assert_eq!(str_len(&r#""belddrzegcsxsyfhzyz""#.to_string()), 19);
    assert_eq!(encoded_str_len(&r#""belddrzegcsxsyfhzyz""#.to_string()), 25);
}

#[test]
fn str_len_ex_51() {
    assert_eq!(raw_str_len(&r#""fuyswi""#.to_string()), 8);
    assert_eq!(str_len(&r#""fuyswi""#.to_string()), 6);
    assert_eq!(encoded_str_len(&r#""fuyswi""#.to_string()), 12);
}

#[test]
fn str_len_ex_52() {
    assert_eq!(
        raw_str_len(&r#""\\hubzebo\"ha\\qyr\"dv\\""#.to_string()),
        26
    );
    assert_eq!(str_len(&r#""\\hubzebo\"ha\\qyr\"dv\\""#.to_string()), 19);
    assert_eq!(
        encoded_str_len(&r#""\\hubzebo\"ha\\qyr\"dv\\""#.to_string()),
        40
    );
}

#[test]
fn str_len_ex_53() {
    assert_eq!(raw_str_len(&r#""mxvlz\"fwuvx\"cyk\"""#.to_string()), 21);
    assert_eq!(str_len(&r#""mxvlz\"fwuvx\"cyk\"""#.to_string()), 16);
    assert_eq!(encoded_str_len(&r#""mxvlz\"fwuvx\"cyk\"""#.to_string()), 31);
}

#[test]
fn str_len_ex_54() {
    assert_eq!(raw_str_len(&r#""ftbh\"ro\\tmcpnpvh\"xx""#.to_string()), 24);
    assert_eq!(str_len(&r#""ftbh\"ro\\tmcpnpvh\"xx""#.to_string()), 19);
    assert_eq!(
        encoded_str_len(&r#""ftbh\"ro\\tmcpnpvh\"xx""#.to_string()),
        34
    );
}

#[test]
fn str_len_ex_55() {
    assert_eq!(raw_str_len(&r#""ygi""#.to_string()), 5);
    assert_eq!(str_len(&r#""ygi""#.to_string()), 3);
    assert_eq!(encoded_str_len(&r#""ygi""#.to_string()), 9);
}

#[test]
fn str_len_ex_56() {
    assert_eq!(
        raw_str_len(&r#""rw\"\"wwn\\fgbjumq\"vgvoh\xd0\"mm""#.to_string()),
        35
    );
    assert_eq!(
        str_len(&r#""rw\"\"wwn\\fgbjumq\"vgvoh\xd0\"mm""#.to_string()),
        25
    );
    assert_eq!(
        encoded_str_len(&r#""rw\"\"wwn\\fgbjumq\"vgvoh\xd0\"mm""#.to_string()),
        50
    );
}

#[test]
fn str_len_ex_57() {
    assert_eq!(
        raw_str_len(&r#""\"pat\"\x63kpfc\"\x2ckhfvxk\"uwqzlx""#.to_string()),
        37
    );
    assert_eq!(
        str_len(&r#""\"pat\"\x63kpfc\"\x2ckhfvxk\"uwqzlx""#.to_string()),
        25
    );
    assert_eq!(
        encoded_str_len(&r#""\"pat\"\x63kpfc\"\x2ckhfvxk\"uwqzlx""#.to_string()),
        51
    );
}

#[test]
fn str_len_ex_58() {
    assert_eq!(raw_str_len(&r#""o""#.to_string()), 3);
    assert_eq!(str_len(&r#""o""#.to_string()), 1);
    assert_eq!(encoded_str_len(&r#""o""#.to_string()), 7);
}

#[test]
fn str_len_ex_59() {
    assert_eq!(
        raw_str_len(&r#""d\"hqtsfp\xceaswe\"\xc0lw""#.to_string()),
        27
    );
    assert_eq!(str_len(&r#""d\"hqtsfp\xceaswe\"\xc0lw""#.to_string()), 17);
    assert_eq!(
        encoded_str_len(&r#""d\"hqtsfp\xceaswe\"\xc0lw""#.to_string()),
        37
    );
}

#[test]
fn str_len_ex_60() {
    assert_eq!(
        raw_str_len(&r#""zajpvfawqntvoveal\"\"trcdarjua""#.to_string()),
        32
    );
    assert_eq!(
        str_len(&r#""zajpvfawqntvoveal\"\"trcdarjua""#.to_string()),
        28
    );
    assert_eq!(
        encoded_str_len(&r#""zajpvfawqntvoveal\"\"trcdarjua""#.to_string()),
        40
    );
}

#[test]
fn str_len_ex_61() {
    assert_eq!(raw_str_len(&r#""xzapq""#.to_string()), 7);
    assert_eq!(str_len(&r#""xzapq""#.to_string()), 5);
    assert_eq!(encoded_str_len(&r#""xzapq""#.to_string()), 11);
}

#[test]
fn str_len_ex_62() {
    assert_eq!(raw_str_len(&r#""rkmhm""#.to_string()), 7);
    assert_eq!(str_len(&r#""rkmhm""#.to_string()), 5);
    assert_eq!(encoded_str_len(&r#""rkmhm""#.to_string()), 11);
}

#[test]
fn str_len_ex_63() {
    assert_eq!(raw_str_len(&r#""byuq""#.to_string()), 6);
    assert_eq!(str_len(&r#""byuq""#.to_string()), 4);
    assert_eq!(encoded_str_len(&r#""byuq""#.to_string()), 10);
}

#[test]
fn str_len_ex_64() {
    assert_eq!(raw_str_len(&r#""rwwmt\xe8jg\xc2\"omt""#.to_string()), 22);
    assert_eq!(str_len(&r#""rwwmt\xe8jg\xc2\"omt""#.to_string()), 13);
    assert_eq!(
        encoded_str_len(&r#""rwwmt\xe8jg\xc2\"omt""#.to_string()),
        30
    );
}

#[test]
fn str_len_ex_65() {
    assert_eq!(raw_str_len(&r#""nfljgdmgefvlh\"x""#.to_string()), 18);
    assert_eq!(str_len(&r#""nfljgdmgefvlh\"x""#.to_string()), 15);
    assert_eq!(encoded_str_len(&r#""nfljgdmgefvlh\"x""#.to_string()), 24);
}

#[test]
fn str_len_ex_66() {
    assert_eq!(raw_str_len(&r#""rpjxcexisualz""#.to_string()), 15);
    assert_eq!(str_len(&r#""rpjxcexisualz""#.to_string()), 13);
    assert_eq!(encoded_str_len(&r#""rpjxcexisualz""#.to_string()), 19);
}

#[test]
fn str_len_ex_67() {
    assert_eq!(raw_str_len(&r#""doxcycmgaiptvd""#.to_string()), 16);
    assert_eq!(str_len(&r#""doxcycmgaiptvd""#.to_string()), 14);
    assert_eq!(encoded_str_len(&r#""doxcycmgaiptvd""#.to_string()), 20);
}

#[test]
fn str_len_ex_68() {
    assert_eq!(
        raw_str_len(&r#""rq\\\"mohnjdf\\xv\\hrnosdtmvxot""#.to_string()),
        33
    );
    assert_eq!(
        str_len(&r#""rq\\\"mohnjdf\\xv\\hrnosdtmvxot""#.to_string()),
        27
    );
    assert_eq!(
        encoded_str_len(&r#""rq\\\"mohnjdf\\xv\\hrnosdtmvxot""#.to_string()),
        45
    );
}

#[test]
fn str_len_ex_69() {
    assert_eq!(raw_str_len(&r#""oqvbcenib\"uhy\\npjxg""#.to_string()), 23);
    assert_eq!(str_len(&r#""oqvbcenib\"uhy\\npjxg""#.to_string()), 19);
    assert_eq!(
        encoded_str_len(&r#""oqvbcenib\"uhy\\npjxg""#.to_string()),
        31
    );
}

#[test]
fn str_len_ex_70() {
    assert_eq!(raw_str_len(&r#""pkvgnm\\ruayuvpbpd""#.to_string()), 20);
    assert_eq!(str_len(&r#""pkvgnm\\ruayuvpbpd""#.to_string()), 17);
    assert_eq!(encoded_str_len(&r#""pkvgnm\\ruayuvpbpd""#.to_string()), 26);
}

#[test]
fn str_len_ex_71() {
    assert_eq!(raw_str_len(&r#""kknmzpxqfbcdgng""#.to_string()), 17);
    assert_eq!(str_len(&r#""kknmzpxqfbcdgng""#.to_string()), 15);
    assert_eq!(encoded_str_len(&r#""kknmzpxqfbcdgng""#.to_string()), 21);
}

#[test]
fn str_len_ex_72() {
    assert_eq!(raw_str_len(&r#""piduhbmaympxdexz""#.to_string()), 18);
    assert_eq!(str_len(&r#""piduhbmaympxdexz""#.to_string()), 16);
    assert_eq!(encoded_str_len(&r#""piduhbmaympxdexz""#.to_string()), 22);
}

#[test]
fn str_len_ex_73() {
    assert_eq!(raw_str_len(&r#""vapczawekhoa\\or""#.to_string()), 18);
    assert_eq!(str_len(&r#""vapczawekhoa\\or""#.to_string()), 15);
    assert_eq!(encoded_str_len(&r#""vapczawekhoa\\or""#.to_string()), 24);
}

#[test]
fn str_len_ex_74() {
    assert_eq!(
        raw_str_len(&r#""tlwn\"avc\"bycg\"\"xuxea""#.to_string()),
        26
    );
    assert_eq!(str_len(&r#""tlwn\"avc\"bycg\"\"xuxea""#.to_string()), 20);
    assert_eq!(
        encoded_str_len(&r#""tlwn\"avc\"bycg\"\"xuxea""#.to_string()),
        38
    );
}

#[test]
fn str_len_ex_75() {
    assert_eq!(
        raw_str_len(&r#""\xcdvryveteqzxrgopmdmihkcgsuozips""#.to_string()),
        35
    );
    assert_eq!(
        str_len(&r#""\xcdvryveteqzxrgopmdmihkcgsuozips""#.to_string()),
        30
    );
    assert_eq!(
        encoded_str_len(&r#""\xcdvryveteqzxrgopmdmihkcgsuozips""#.to_string()),
        40
    );
}

#[test]
fn str_len_ex_76() {
    assert_eq!(raw_str_len(&r#""kpzziqt""#.to_string()), 9);
    assert_eq!(str_len(&r#""kpzziqt""#.to_string()), 7);
    assert_eq!(encoded_str_len(&r#""kpzziqt""#.to_string()), 13);
}

#[test]
fn str_len_ex_77() {
    assert_eq!(raw_str_len(&r#""sdy\\s\"cjq""#.to_string()), 13);
    assert_eq!(str_len(&r#""sdy\\s\"cjq""#.to_string()), 9);
    assert_eq!(encoded_str_len(&r#""sdy\\s\"cjq""#.to_string()), 21);
}

#[test]
fn str_len_ex_78() {
    assert_eq!(raw_str_len(&r#""yujs""#.to_string()), 6);
    assert_eq!(str_len(&r#""yujs""#.to_string()), 4);
    assert_eq!(encoded_str_len(&r#""yujs""#.to_string()), 10);
}

#[test]
fn str_len_ex_79() {
    assert_eq!(raw_str_len(&r#""qte\"q""#.to_string()), 8);
    assert_eq!(str_len(&r#""qte\"q""#.to_string()), 5);
    assert_eq!(encoded_str_len(&r#""qte\"q""#.to_string()), 14);
}

#[test]
fn str_len_ex_80() {
    assert_eq!(
        raw_str_len(&r#""qyvpnkhjcqjv\"cclvv\"pclgtg\xeak\"tno""#.to_string()),
        39
    );
    assert_eq!(
        str_len(&r#""qyvpnkhjcqjv\"cclvv\"pclgtg\xeak\"tno""#.to_string()),
        31
    );
    assert_eq!(
        encoded_str_len(&r#""qyvpnkhjcqjv\"cclvv\"pclgtg\xeak\"tno""#.to_string()),
        50
    );
}

#[test]
fn str_len_ex_81() {
    assert_eq!(raw_str_len(&r#""xwx""#.to_string()), 5);
    assert_eq!(str_len(&r#""xwx""#.to_string()), 3);
    assert_eq!(encoded_str_len(&r#""xwx""#.to_string()), 9);
}

#[test]
fn str_len_ex_82() {
    assert_eq!(raw_str_len(&r#""vibuvv""#.to_string()), 8);
    assert_eq!(str_len(&r#""vibuvv""#.to_string()), 6);
    assert_eq!(encoded_str_len(&r#""vibuvv""#.to_string()), 12);
}

#[test]
fn str_len_ex_83() {
    assert_eq!(raw_str_len(&r#""qq\"""#.to_string()), 6);
    assert_eq!(str_len(&r#""qq\"""#.to_string()), 3);
    assert_eq!(encoded_str_len(&r#""qq\"""#.to_string()), 12);
}

#[test]
fn str_len_ex_84() {
    assert_eq!(
        raw_str_len(&r#""wwjduomtbkbdtorhpyalxswisq\"r""#.to_string()),
        31
    );
    assert_eq!(
        str_len(&r#""wwjduomtbkbdtorhpyalxswisq\"r""#.to_string()),
        28
    );
    assert_eq!(
        encoded_str_len(&r#""wwjduomtbkbdtorhpyalxswisq\"r""#.to_string()),
        37
    );
}

#[test]
fn str_len_ex_85() {
    assert_eq!(
        raw_str_len(&r#""afuw\\mfjzctcivwesutxbk\"lk""#.to_string()),
        29
    );
    assert_eq!(str_len(&r#""afuw\\mfjzctcivwesutxbk\"lk""#.to_string()), 25);
    assert_eq!(
        encoded_str_len(&r#""afuw\\mfjzctcivwesutxbk\"lk""#.to_string()),
        37
    );
}

#[test]
fn str_len_ex_86() {
    assert_eq!(raw_str_len(&r#""e\xcef\\hkiu""#.to_string()), 14);
    assert_eq!(str_len(&r#""e\xcef\\hkiu""#.to_string()), 8);
    assert_eq!(encoded_str_len(&r#""e\xcef\\hkiu""#.to_string()), 21);
}

#[test]
fn str_len_ex_87() {
    assert_eq!(raw_str_len(&r#""ftdrgzvygcw\"jwsrcmgxj""#.to_string()), 24);
    assert_eq!(str_len(&r#""ftdrgzvygcw\"jwsrcmgxj""#.to_string()), 21);
    assert_eq!(
        encoded_str_len(&r#""ftdrgzvygcw\"jwsrcmgxj""#.to_string()),
        30
    );
}

#[test]
fn str_len_ex_88() {
    assert_eq!(
        raw_str_len(&r#""zrddqfkx\x21dr\"ju\"elybk\"powj\"\"kpryz""#.to_string()),
        42
    );
    assert_eq!(
        str_len(&r#""zrddqfkx\x21dr\"ju\"elybk\"powj\"\"kpryz""#.to_string()),
        32
    );
    assert_eq!(
        encoded_str_len(&r#""zrddqfkx\x21dr\"ju\"elybk\"powj\"\"kpryz""#.to_string()),
        57
    );
}

#[test]
fn str_len_ex_89() {
    assert_eq!(raw_str_len(&r#""dttdkfvbodkma\"""#.to_string()), 17);
    assert_eq!(str_len(&r#""dttdkfvbodkma\"""#.to_string()), 14);
    assert_eq!(encoded_str_len(&r#""dttdkfvbodkma\"""#.to_string()), 23);
}

#[test]
fn str_len_ex_90() {
    assert_eq!(raw_str_len(&r#""lzygktugpqw""#.to_string()), 13);
    assert_eq!(str_len(&r#""lzygktugpqw""#.to_string()), 11);
    assert_eq!(encoded_str_len(&r#""lzygktugpqw""#.to_string()), 17);
}

#[test]
fn str_len_ex_91() {
    assert_eq!(
        raw_str_len(&r#""qu\x83tes\\u\"tnid\"ryuz""#.to_string()),
        26
    );
    assert_eq!(str_len(&r#""qu\x83tes\\u\"tnid\"ryuz""#.to_string()), 18);
    assert_eq!(
        encoded_str_len(&r#""qu\x83tes\\u\"tnid\"ryuz""#.to_string()),
        37
    );
}

#[test]
fn str_len_ex_92() {
    assert_eq!(
        raw_str_len(&r#""\\o\"pe\\vqwlsizjklwrjofg\xe2oau\\rd""#.to_string()),
        38
    );
    assert_eq!(
        str_len(&r#""\\o\"pe\\vqwlsizjklwrjofg\xe2oau\\rd""#.to_string()),
        29
    );
    assert_eq!(
        encoded_str_len(&r#""\\o\"pe\\vqwlsizjklwrjofg\xe2oau\\rd""#.to_string()),
        51
    );
}

#[test]
fn str_len_ex_93() {
    assert_eq!(
        raw_str_len(&r#""mikevjzhnwgx\"fozrj\"h\"""#.to_string()),
        26
    );
    assert_eq!(str_len(&r#""mikevjzhnwgx\"fozrj\"h\"""#.to_string()), 21);
    assert_eq!(
        encoded_str_len(&r#""mikevjzhnwgx\"fozrj\"h\"""#.to_string()),
        36
    );
}

#[test]
fn str_len_ex_94() {
    assert_eq!(raw_str_len(&r#""ligxmxznzvtachvvbahnff""#.to_string()), 24);
    assert_eq!(str_len(&r#""ligxmxznzvtachvvbahnff""#.to_string()), 22);
    assert_eq!(
        encoded_str_len(&r#""ligxmxznzvtachvvbahnff""#.to_string()),
        28
    );
}

#[test]
fn str_len_ex_95() {
    assert_eq!(raw_str_len(&r#""d\\kq""#.to_string()), 7);
    assert_eq!(str_len(&r#""d\\kq""#.to_string()), 4);
    assert_eq!(encoded_str_len(&r#""d\\kq""#.to_string()), 13);
}

#[test]
fn str_len_ex_96() {
    assert_eq!(raw_str_len(&r#""tnbkxpzmcakqhaa""#.to_string()), 17);
    assert_eq!(str_len(&r#""tnbkxpzmcakqhaa""#.to_string()), 15);
    assert_eq!(encoded_str_len(&r#""tnbkxpzmcakqhaa""#.to_string()), 21);
}

#[test]
fn str_len_ex_97() {
    assert_eq!(raw_str_len(&r#""g\\yeakebeyv""#.to_string()), 14);
    assert_eq!(str_len(&r#""g\\yeakebeyv""#.to_string()), 11);
    assert_eq!(encoded_str_len(&r#""g\\yeakebeyv""#.to_string()), 20);
}

#[test]
fn str_len_ex_98() {
    assert_eq!(
        raw_str_len(&r#""cqkcnd\"sxjxfnawy\x31zax\x6ceha""#.to_string()),
        33
    );
    assert_eq!(
        str_len(&r#""cqkcnd\"sxjxfnawy\x31zax\x6ceha""#.to_string()),
        24
    );
    assert_eq!(
        encoded_str_len(&r#""cqkcnd\"sxjxfnawy\x31zax\x6ceha""#.to_string()),
        41
    );
}

#[test]
fn str_len_ex_99() {
    assert_eq!(
        raw_str_len(&r#""m\x0dtqotffzdnetujtsgjqgwddc""#.to_string()),
        30
    );
    assert_eq!(
        str_len(&r#""m\x0dtqotffzdnetujtsgjqgwddc""#.to_string()),
        25
    );
    assert_eq!(
        encoded_str_len(&r#""m\x0dtqotffzdnetujtsgjqgwddc""#.to_string()),
        35
    );
}

#[test]
fn str_len_ex_100() {
    assert_eq!(
        raw_str_len(&r#""masnugb\"etgmxul\x3bqd\\tmtddnvcy""#.to_string()),
        35
    );
    assert_eq!(
        str_len(&r#""masnugb\"etgmxul\x3bqd\\tmtddnvcy""#.to_string()),
        28
    );
    assert_eq!(
        encoded_str_len(&r#""masnugb\"etgmxul\x3bqd\\tmtddnvcy""#.to_string()),
        44
    );
}

#[test]
fn str_len_ex_101() {
    assert_eq!(
        raw_str_len(&r#""floediikodfgre\x23wyoxlswxflwecdjpt""#.to_string()),
        37
    );
    assert_eq!(
        str_len(&r#""floediikodfgre\x23wyoxlswxflwecdjpt""#.to_string()),
        32
    );
    assert_eq!(
        encoded_str_len(&r#""floediikodfgre\x23wyoxlswxflwecdjpt""#.to_string()),
        42
    );
}

#[test]
fn str_len_ex_102() {
    assert_eq!(raw_str_len(&r#""zu""#.to_string()), 4);
    assert_eq!(str_len(&r#""zu""#.to_string()), 2);
    assert_eq!(encoded_str_len(&r#""zu""#.to_string()), 8);
}

#[test]
fn str_len_ex_103() {
    assert_eq!(raw_str_len(&r#""r""#.to_string()), 3);
    assert_eq!(str_len(&r#""r""#.to_string()), 1);
    assert_eq!(encoded_str_len(&r#""r""#.to_string()), 7);
}

#[test]
fn str_len_ex_104() {
    assert_eq!(
        raw_str_len(&r#""\"ashzdbd\"pdvba\xeeumkr\\amnj""#.to_string()),
        32
    );
    assert_eq!(
        str_len(&r#""\"ashzdbd\"pdvba\xeeumkr\\amnj""#.to_string()),
        24
    );
    assert_eq!(
        encoded_str_len(&r#""\"ashzdbd\"pdvba\xeeumkr\\amnj""#.to_string()),
        43
    );
}

#[test]
fn str_len_ex_105() {
    assert_eq!(
        raw_str_len(&r#""ckslmuwbtfouwpfwtuiqmeozgspwnhx""#.to_string()),
        33
    );
    assert_eq!(
        str_len(&r#""ckslmuwbtfouwpfwtuiqmeozgspwnhx""#.to_string()),
        31
    );
    assert_eq!(
        encoded_str_len(&r#""ckslmuwbtfouwpfwtuiqmeozgspwnhx""#.to_string()),
        37
    );
}

#[test]
fn str_len_ex_106() {
    assert_eq!(
        raw_str_len(&r#""t\\qjsjek\xf9gjcxsyco\"r""#.to_string()),
        26
    );
    assert_eq!(str_len(&r#""t\\qjsjek\xf9gjcxsyco\"r""#.to_string()), 19);
    assert_eq!(
        encoded_str_len(&r#""t\\qjsjek\xf9gjcxsyco\"r""#.to_string()),
        35
    );
}

#[test]
fn str_len_ex_107() {
    assert_eq!(raw_str_len(&r#""hoed\x1b\\tcmaqch\"epdy""#.to_string()), 25);
    assert_eq!(str_len(&r#""hoed\x1b\\tcmaqch\"epdy""#.to_string()), 18);
    assert_eq!(
        encoded_str_len(&r#""hoed\x1b\\tcmaqch\"epdy""#.to_string()),
        34
    );
}

#[test]
fn str_len_ex_108() {
    assert_eq!(
        raw_str_len(&r#""mgjiojwzc\\ypqcn\xb1njmp\"aeeblxt""#.to_string()),
        35
    );
    assert_eq!(
        str_len(&r#""mgjiojwzc\\ypqcn\xb1njmp\"aeeblxt""#.to_string()),
        28
    );
    assert_eq!(
        encoded_str_len(&r#""mgjiojwzc\\ypqcn\xb1njmp\"aeeblxt""#.to_string()),
        44
    );
}

#[test]
fn str_len_ex_109() {
    assert_eq!(raw_str_len(&r#""\xdf\"h\x5enfracj""#.to_string()), 19);
    assert_eq!(str_len(&r#""\xdf\"h\x5enfracj""#.to_string()), 10);
    assert_eq!(encoded_str_len(&r#""\xdf\"h\x5enfracj""#.to_string()), 27);
}

#[test]
fn str_len_ex_110() {
    assert_eq!(raw_str_len(&r#""\x6fpbpocrb""#.to_string()), 13);
    assert_eq!(str_len(&r#""\x6fpbpocrb""#.to_string()), 8);
    assert_eq!(encoded_str_len(&r#""\x6fpbpocrb""#.to_string()), 18);
}

#[test]
fn str_len_ex_111() {
    assert_eq!(raw_str_len(&r#""jbmhrswyyq\\""#.to_string()), 14);
    assert_eq!(str_len(&r#""jbmhrswyyq\\""#.to_string()), 11);
    assert_eq!(encoded_str_len(&r#""jbmhrswyyq\\""#.to_string()), 20);
}

#[test]
fn str_len_ex_112() {
    assert_eq!(raw_str_len(&r#""wtyqtenfwatji\"ls\\""#.to_string()), 21);
    assert_eq!(str_len(&r#""wtyqtenfwatji\"ls\\""#.to_string()), 17);
    assert_eq!(encoded_str_len(&r#""wtyqtenfwatji\"ls\\""#.to_string()), 29);
}

#[test]
fn str_len_ex_113() {
    assert_eq!(raw_str_len(&r#""voy""#.to_string()), 5);
    assert_eq!(str_len(&r#""voy""#.to_string()), 3);
    assert_eq!(encoded_str_len(&r#""voy""#.to_string()), 9);
}

#[test]
fn str_len_ex_114() {
    assert_eq!(raw_str_len(&r#""awj""#.to_string()), 5);
    assert_eq!(str_len(&r#""awj""#.to_string()), 3);
    assert_eq!(encoded_str_len(&r#""awj""#.to_string()), 9);
}

#[test]
fn str_len_ex_115() {
    assert_eq!(raw_str_len(&r#""rtbj\"j""#.to_string()), 9);
    assert_eq!(str_len(&r#""rtbj\"j""#.to_string()), 6);
    assert_eq!(encoded_str_len(&r#""rtbj\"j""#.to_string()), 15);
}

#[test]
fn str_len_ex_116() {
    assert_eq!(raw_str_len(&r#""hynl""#.to_string()), 6);
    assert_eq!(str_len(&r#""hynl""#.to_string()), 4);
    assert_eq!(encoded_str_len(&r#""hynl""#.to_string()), 10);
}

#[test]
fn str_len_ex_117() {
    assert_eq!(
        raw_str_len(&r#""orqqeuaat\\xu\\havsgr\xc5qdk""#.to_string()),
        30
    );
    assert_eq!(
        str_len(&r#""orqqeuaat\\xu\\havsgr\xc5qdk""#.to_string()),
        23
    );
    assert_eq!(
        encoded_str_len(&r#""orqqeuaat\\xu\\havsgr\xc5qdk""#.to_string()),
        39
    );
}

#[test]
fn str_len_ex_118() {
    assert_eq!(raw_str_len(&r#""g\"npyzjfq\"rjefwsk""#.to_string()), 21);
    assert_eq!(str_len(&r#""g\"npyzjfq\"rjefwsk""#.to_string()), 17);
    assert_eq!(encoded_str_len(&r#""g\"npyzjfq\"rjefwsk""#.to_string()), 29);
}

#[test]
fn str_len_ex_119() {
    assert_eq!(
        raw_str_len(&r#""rk\\kkcirjbixr\\zelndx\"bsnqvqj\"""#.to_string()),
        35
    );
    assert_eq!(
        str_len(&r#""rk\\kkcirjbixr\\zelndx\"bsnqvqj\"""#.to_string()),
        29
    );
    assert_eq!(
        encoded_str_len(&r#""rk\\kkcirjbixr\\zelndx\"bsnqvqj\"""#.to_string()),
        47
    );
}

#[test]
fn str_len_ex_120() {
    assert_eq!(raw_str_len(&r#""tecoz""#.to_string()), 7);
    assert_eq!(str_len(&r#""tecoz""#.to_string()), 5);
    assert_eq!(encoded_str_len(&r#""tecoz""#.to_string()), 11);
}

#[test]
fn str_len_ex_121() {
    assert_eq!(raw_str_len(&r#""dn\"uswngbdk\"""#.to_string()), 16);
    assert_eq!(str_len(&r#""dn\"uswngbdk\"""#.to_string()), 12);
    assert_eq!(encoded_str_len(&r#""dn\"uswngbdk\"""#.to_string()), 24);
}

#[test]
fn str_len_ex_122() {
    assert_eq!(raw_str_len(&r#""qb\\""#.to_string()), 6);
    assert_eq!(str_len(&r#""qb\\""#.to_string()), 3);
    assert_eq!(encoded_str_len(&r#""qb\\""#.to_string()), 12);
}

#[test]
fn str_len_ex_123() {
    assert_eq!(raw_str_len(&r#""wpyis\\ebq""#.to_string()), 12);
    assert_eq!(str_len(&r#""wpyis\\ebq""#.to_string()), 9);
    assert_eq!(encoded_str_len(&r#""wpyis\\ebq""#.to_string()), 18);
}

#[test]
fn str_len_ex_124() {
    assert_eq!(
        raw_str_len(&r#""ppwue\\airoxzjjdqbvyurhaabetv""#.to_string()),
        31
    );
    assert_eq!(
        str_len(&r#""ppwue\\airoxzjjdqbvyurhaabetv""#.to_string()),
        28
    );
    assert_eq!(
        encoded_str_len(&r#""ppwue\\airoxzjjdqbvyurhaabetv""#.to_string()),
        37
    );
}

#[test]
fn str_len_ex_125() {
    assert_eq!(raw_str_len(&r#""fxlvt""#.to_string()), 7);
    assert_eq!(str_len(&r#""fxlvt""#.to_string()), 5);
    assert_eq!(encoded_str_len(&r#""fxlvt""#.to_string()), 11);
}

#[test]
fn str_len_ex_126() {
    assert_eq!(raw_str_len(&r#""ql\"oqsmsvpxcg\"k""#.to_string()), 19);
    assert_eq!(str_len(&r#""ql\"oqsmsvpxcg\"k""#.to_string()), 15);
    assert_eq!(encoded_str_len(&r#""ql\"oqsmsvpxcg\"k""#.to_string()), 27);
}

#[test]
fn str_len_ex_127() {
    assert_eq!(raw_str_len(&r#""vqlhuec\\adw""#.to_string()), 14);
    assert_eq!(str_len(&r#""vqlhuec\\adw""#.to_string()), 11);
    assert_eq!(encoded_str_len(&r#""vqlhuec\\adw""#.to_string()), 20);
}

#[test]
fn str_len_ex_128() {
    assert_eq!(raw_str_len(&r#""qzmi\xffberakqqkk""#.to_string()), 19);
    assert_eq!(str_len(&r#""qzmi\xffberakqqkk""#.to_string()), 14);
    assert_eq!(encoded_str_len(&r#""qzmi\xffberakqqkk""#.to_string()), 24);
}

#[test]
fn str_len_ex_129() {
    assert_eq!(raw_str_len(&r#""tisjqff\"wf""#.to_string()), 13);
    assert_eq!(str_len(&r#""tisjqff\"wf""#.to_string()), 10);
    assert_eq!(encoded_str_len(&r#""tisjqff\"wf""#.to_string()), 19);
}

#[test]
fn str_len_ex_130() {
    assert_eq!(raw_str_len(&r#""yhnpudoaybwucvppj""#.to_string()), 19);
    assert_eq!(str_len(&r#""yhnpudoaybwucvppj""#.to_string()), 17);
    assert_eq!(encoded_str_len(&r#""yhnpudoaybwucvppj""#.to_string()), 23);
}

#[test]
fn str_len_ex_131() {
    assert_eq!(
        raw_str_len(&r#""xhfuf\\ehsrhsnfxcwtibd\"ubfpz""#.to_string()),
        31
    );
    assert_eq!(
        str_len(&r#""xhfuf\\ehsrhsnfxcwtibd\"ubfpz""#.to_string()),
        27
    );
    assert_eq!(
        encoded_str_len(&r#""xhfuf\\ehsrhsnfxcwtibd\"ubfpz""#.to_string()),
        39
    );
}

#[test]
fn str_len_ex_132() {
    assert_eq!(raw_str_len(&r#""ihgjquzhf\"""#.to_string()), 13);
    assert_eq!(str_len(&r#""ihgjquzhf\"""#.to_string()), 10);
    assert_eq!(encoded_str_len(&r#""ihgjquzhf\"""#.to_string()), 19);
}

#[test]
fn str_len_ex_133() {
    assert_eq!(
        raw_str_len(&r#""ff\x66dsupesrnusrtqnywoqcn\\""#.to_string()),
        30
    );
    assert_eq!(
        str_len(&r#""ff\x66dsupesrnusrtqnywoqcn\\""#.to_string()),
        24
    );
    assert_eq!(
        encoded_str_len(&r#""ff\x66dsupesrnusrtqnywoqcn\\""#.to_string()),
        37
    );
}

#[test]
fn str_len_ex_134() {
    assert_eq!(raw_str_len(&r#""z\x77zpubbjmd""#.to_string()), 15);
    assert_eq!(str_len(&r#""z\x77zpubbjmd""#.to_string()), 10);
    assert_eq!(encoded_str_len(&r#""z\x77zpubbjmd""#.to_string()), 20);
}

#[test]
fn str_len_ex_135() {
    assert_eq!(
        raw_str_len(&r#""\"vhzlbwq\"xeimjt\\xe\x85umho\"m\"\"bmy""#.to_string()),
        41
    );
    assert_eq!(
        str_len(&r#""\"vhzlbwq\"xeimjt\\xe\x85umho\"m\"\"bmy""#.to_string()),
        30
    );
    assert_eq!(
        encoded_str_len(&r#""\"vhzlbwq\"xeimjt\\xe\x85umho\"m\"\"bmy""#.to_string()),
        58
    );
}

#[test]
fn str_len_ex_136() {
    assert_eq!(
        raw_str_len(&r#""mmuvkioocmzjjysi\"mkfbec\"""#.to_string()),
        28
    );
    assert_eq!(str_len(&r#""mmuvkioocmzjjysi\"mkfbec\"""#.to_string()), 24);
    assert_eq!(
        encoded_str_len(&r#""mmuvkioocmzjjysi\"mkfbec\"""#.to_string()),
        36
    );
}

#[test]
fn str_len_ex_137() {
    assert_eq!(
        raw_str_len(&r#""rpgghowbduw\x2fayslubajinoik\xd0hcfy""#.to_string()),
        38
    );
    assert_eq!(
        str_len(&r#""rpgghowbduw\x2fayslubajinoik\xd0hcfy""#.to_string()),
        30
    );
    assert_eq!(
        encoded_str_len(&r#""rpgghowbduw\x2fayslubajinoik\xd0hcfy""#.to_string()),
        44
    );
}

#[test]
fn str_len_ex_138() {
    assert_eq!(
        raw_str_len(&r#""xrkyjqul\xdexlojgdphczp\"jfk""#.to_string()),
        30
    );
    assert_eq!(
        str_len(&r#""xrkyjqul\xdexlojgdphczp\"jfk""#.to_string()),
        24
    );
    assert_eq!(
        encoded_str_len(&r#""xrkyjqul\xdexlojgdphczp\"jfk""#.to_string()),
        37
    );
}

#[test]
fn str_len_ex_139() {
    assert_eq!(
        raw_str_len(&r#""mg\x07cnr\x8b\x67xdgszmgiktpjhawho""#.to_string()),
        36
    );
    assert_eq!(
        str_len(&r#""mg\x07cnr\x8b\x67xdgszmgiktpjhawho""#.to_string()),
        25
    );
    assert_eq!(
        encoded_str_len(&r#""mg\x07cnr\x8b\x67xdgszmgiktpjhawho""#.to_string()),
        43
    );
}

#[test]
fn str_len_ex_140() {
    assert_eq!(raw_str_len(&r#""kdgufhaoab""#.to_string()), 12);
    assert_eq!(str_len(&r#""kdgufhaoab""#.to_string()), 10);
    assert_eq!(encoded_str_len(&r#""kdgufhaoab""#.to_string()), 16);
}

#[test]
fn str_len_ex_141() {
    assert_eq!(raw_str_len(&r#""rlhela\"nldr""#.to_string()), 14);
    assert_eq!(str_len(&r#""rlhela\"nldr""#.to_string()), 11);
    assert_eq!(encoded_str_len(&r#""rlhela\"nldr""#.to_string()), 20);
}

#[test]
fn str_len_ex_142() {
    assert_eq!(raw_str_len(&r#""wzye\x87u""#.to_string()), 11);
    assert_eq!(str_len(&r#""wzye\x87u""#.to_string()), 6);
    assert_eq!(encoded_str_len(&r#""wzye\x87u""#.to_string()), 16);
}

#[test]
fn str_len_ex_143() {
    assert_eq!(
        raw_str_len(&r#""yif\x75bjhnitgoarmfgqwpmopu""#.to_string()),
        29
    );
    assert_eq!(str_len(&r#""yif\x75bjhnitgoarmfgqwpmopu""#.to_string()), 24);
    assert_eq!(
        encoded_str_len(&r#""yif\x75bjhnitgoarmfgqwpmopu""#.to_string()),
        34
    );
}

#[test]
fn str_len_ex_144() {
    assert_eq!(raw_str_len(&r#""pvlbyez\"wyy\x3dpgr""#.to_string()), 21);
    assert_eq!(str_len(&r#""pvlbyez\"wyy\x3dpgr""#.to_string()), 15);
    assert_eq!(encoded_str_len(&r#""pvlbyez\"wyy\x3dpgr""#.to_string()), 28);
}

#[test]
fn str_len_ex_145() {
    assert_eq!(
        raw_str_len(&r#""ezdm\"ovkruthkvdwtqwr\"ibdoawzgu""#.to_string()),
        34
    );
    assert_eq!(
        str_len(&r#""ezdm\"ovkruthkvdwtqwr\"ibdoawzgu""#.to_string()),
        30
    );
    assert_eq!(
        encoded_str_len(&r#""ezdm\"ovkruthkvdwtqwr\"ibdoawzgu""#.to_string()),
        42
    );
}

#[test]
fn str_len_ex_146() {
    assert_eq!(raw_str_len(&r#""qubp""#.to_string()), 6);
    assert_eq!(str_len(&r#""qubp""#.to_string()), 4);
    assert_eq!(encoded_str_len(&r#""qubp""#.to_string()), 10);
}

#[test]
fn str_len_ex_147() {
    assert_eq!(raw_str_len(&r#""b\\kcpegcn\\zgdemgorjnk""#.to_string()), 25);
    assert_eq!(str_len(&r#""b\\kcpegcn\\zgdemgorjnk""#.to_string()), 21);
    assert_eq!(
        encoded_str_len(&r#""b\\kcpegcn\\zgdemgorjnk""#.to_string()),
        33
    );
}

#[test]
fn str_len_ex_148() {
    assert_eq!(raw_str_len(&r#""gjsva\\kzaor\"\"gtpd""#.to_string()), 22);
    assert_eq!(str_len(&r#""gjsva\\kzaor\"\"gtpd""#.to_string()), 17);
    assert_eq!(
        encoded_str_len(&r#""gjsva\\kzaor\"\"gtpd""#.to_string()),
        32
    );
}

#[test]
fn str_len_ex_149() {
    assert_eq!(raw_str_len(&r#""\"kt""#.to_string()), 6);
    assert_eq!(str_len(&r#""\"kt""#.to_string()), 3);
    assert_eq!(encoded_str_len(&r#""\"kt""#.to_string()), 12);
}

#[test]
fn str_len_ex_150() {
    assert_eq!(raw_str_len(&r#""rlymwlcodix""#.to_string()), 13);
    assert_eq!(str_len(&r#""rlymwlcodix""#.to_string()), 11);
    assert_eq!(encoded_str_len(&r#""rlymwlcodix""#.to_string()), 17);
}

#[test]
fn str_len_ex_151() {
    assert_eq!(raw_str_len(&r#""qqtmswowxca\"jvv""#.to_string()), 18);
    assert_eq!(str_len(&r#""qqtmswowxca\"jvv""#.to_string()), 15);
    assert_eq!(encoded_str_len(&r#""qqtmswowxca\"jvv""#.to_string()), 24);
}

#[test]
fn str_len_ex_152() {
    assert_eq!(raw_str_len(&r#""jni\xebwhozb""#.to_string()), 14);
    assert_eq!(str_len(&r#""jni\xebwhozb""#.to_string()), 9);
    assert_eq!(encoded_str_len(&r#""jni\xebwhozb""#.to_string()), 19);
}

#[test]
fn str_len_ex_153() {
    assert_eq!(
        raw_str_len(&r#""zhino\"kzjtmgxpi\"zzexijg""#.to_string()),
        27
    );
    assert_eq!(str_len(&r#""zhino\"kzjtmgxpi\"zzexijg""#.to_string()), 23);
    assert_eq!(
        encoded_str_len(&r#""zhino\"kzjtmgxpi\"zzexijg""#.to_string()),
        35
    );
}

#[test]
fn str_len_ex_154() {
    assert_eq!(raw_str_len(&r#""tyrbat\\mejgzplufxixkyg""#.to_string()), 25);
    assert_eq!(str_len(&r#""tyrbat\\mejgzplufxixkyg""#.to_string()), 22);
    assert_eq!(
        encoded_str_len(&r#""tyrbat\\mejgzplufxixkyg""#.to_string()),
        31
    );
}

#[test]
fn str_len_ex_155() {
    assert_eq!(raw_str_len(&r#""lhmopxiao\x09\"p\xebl""#.to_string()), 23);
    assert_eq!(str_len(&r#""lhmopxiao\x09\"p\xebl""#.to_string()), 14);
    assert_eq!(
        encoded_str_len(&r#""lhmopxiao\x09\"p\xebl""#.to_string()),
        31
    );
}

#[test]
fn str_len_ex_156() {
    assert_eq!(raw_str_len(&r#""xefioorxvate""#.to_string()), 14);
    assert_eq!(str_len(&r#""xefioorxvate""#.to_string()), 12);
    assert_eq!(encoded_str_len(&r#""xefioorxvate""#.to_string()), 18);
}

#[test]
fn str_len_ex_157() {
    assert_eq!(raw_str_len(&r#""nmcgd\x46xfujt\"w""#.to_string()), 19);
    assert_eq!(str_len(&r#""nmcgd\x46xfujt\"w""#.to_string()), 13);
    assert_eq!(encoded_str_len(&r#""nmcgd\x46xfujt\"w""#.to_string()), 26);
}

#[test]
fn str_len_ex_158() {
    assert_eq!(raw_str_len(&r#""\xe3wnwpat\"gtimrb""#.to_string()), 20);
    assert_eq!(str_len(&r#""\xe3wnwpat\"gtimrb""#.to_string()), 14);
    assert_eq!(encoded_str_len(&r#""\xe3wnwpat\"gtimrb""#.to_string()), 27);
}

#[test]
fn str_len_ex_159() {
    assert_eq!(
        raw_str_len(&r#""wpq\"xkjuw\xebbohgcagppb""#.to_string()),
        26
    );
    assert_eq!(str_len(&r#""wpq\"xkjuw\xebbohgcagppb""#.to_string()), 20);
    assert_eq!(
        encoded_str_len(&r#""wpq\"xkjuw\xebbohgcagppb""#.to_string()),
        33
    );
}

#[test]
fn str_len_ex_160() {
    assert_eq!(raw_str_len(&r#""fmvpwaca""#.to_string()), 10);
    assert_eq!(str_len(&r#""fmvpwaca""#.to_string()), 8);
    assert_eq!(encoded_str_len(&r#""fmvpwaca""#.to_string()), 14);
}

#[test]
fn str_len_ex_161() {
    assert_eq!(raw_str_len(&r#""mlsw""#.to_string()), 6);
    assert_eq!(str_len(&r#""mlsw""#.to_string()), 4);
    assert_eq!(encoded_str_len(&r#""mlsw""#.to_string()), 10);
}

#[test]
fn str_len_ex_162() {
    assert_eq!(raw_str_len(&r#""fdan\\\x9e""#.to_string()), 12);
    assert_eq!(str_len(&r#""fdan\\\x9e""#.to_string()), 6);
    assert_eq!(encoded_str_len(&r#""fdan\\\x9e""#.to_string()), 19);
}

#[test]
fn str_len_ex_163() {
    assert_eq!(raw_str_len(&r#""\"f\"fmdlzc""#.to_string()), 13);
    assert_eq!(str_len(&r#""\"f\"fmdlzc""#.to_string()), 9);
    assert_eq!(encoded_str_len(&r#""\"f\"fmdlzc""#.to_string()), 21);
}

#[test]
fn str_len_ex_164() {
    assert_eq!(
        raw_str_len(&r#""nyuj\\jnnfzdnrqmhvjrahlvzl""#.to_string()),
        28
    );
    assert_eq!(str_len(&r#""nyuj\\jnnfzdnrqmhvjrahlvzl""#.to_string()), 25);
    assert_eq!(
        encoded_str_len(&r#""nyuj\\jnnfzdnrqmhvjrahlvzl""#.to_string()),
        34
    );
}

#[test]
fn str_len_ex_165() {
    assert_eq!(raw_str_len(&r#""zn\"f\xcfsshcdaukkimfwk""#.to_string()), 25);
    assert_eq!(str_len(&r#""zn\"f\xcfsshcdaukkimfwk""#.to_string()), 19);
    assert_eq!(
        encoded_str_len(&r#""zn\"f\xcfsshcdaukkimfwk""#.to_string()),
        32
    );
}

#[test]
fn str_len_ex_166() {
    assert_eq!(
        raw_str_len(&r#""uayugezzo\\\"e\"blnrgjaupqhik""#.to_string()),
        31
    );
    assert_eq!(
        str_len(&r#""uayugezzo\\\"e\"blnrgjaupqhik""#.to_string()),
        26
    );
    assert_eq!(
        encoded_str_len(&r#""uayugezzo\\\"e\"blnrgjaupqhik""#.to_string()),
        41
    );
}

#[test]
fn str_len_ex_167() {
    assert_eq!(
        raw_str_len(&r#""efd\"apkndelkuvfvwyyatyttkehc""#.to_string()),
        31
    );
    assert_eq!(
        str_len(&r#""efd\"apkndelkuvfvwyyatyttkehc""#.to_string()),
        28
    );
    assert_eq!(
        encoded_str_len(&r#""efd\"apkndelkuvfvwyyatyttkehc""#.to_string()),
        37
    );
}

#[test]
fn str_len_ex_168() {
    assert_eq!(
        raw_str_len(&r#""ufxq\\\"m\"bwkh\x93kapbqrvxxzbzp\\""#.to_string()),
        36
    );
    assert_eq!(
        str_len(&r#""ufxq\\\"m\"bwkh\x93kapbqrvxxzbzp\\""#.to_string()),
        27
    );
    assert_eq!(
        encoded_str_len(&r#""ufxq\\\"m\"bwkh\x93kapbqrvxxzbzp\\""#.to_string()),
        49
    );
}

#[test]
fn str_len_ex_169() {
    assert_eq!(
        raw_str_len(&r#""fgypsbgjak\x79qblbeidavqtddfacq\\i\"h""#.to_string()),
        39
    );
    assert_eq!(
        str_len(&r#""fgypsbgjak\x79qblbeidavqtddfacq\\i\"h""#.to_string()),
        32
    );
    assert_eq!(
        encoded_str_len(&r#""fgypsbgjak\x79qblbeidavqtddfacq\\i\"h""#.to_string()),
        48
    );
}

#[test]
fn str_len_ex_170() {
    assert_eq!(
        raw_str_len(&r#""kcfgpiysdxlgejjvgndb\\dovfpqodw""#.to_string()),
        33
    );
    assert_eq!(
        str_len(&r#""kcfgpiysdxlgejjvgndb\\dovfpqodw""#.to_string()),
        30
    );
    assert_eq!(
        encoded_str_len(&r#""kcfgpiysdxlgejjvgndb\\dovfpqodw""#.to_string()),
        39
    );
}

#[test]
fn str_len_ex_171() {
    assert_eq!(
        raw_str_len(&r#""\"onpqnssmighipuqgwx\"nrokzgvg""#.to_string()),
        32
    );
    assert_eq!(
        str_len(&r#""\"onpqnssmighipuqgwx\"nrokzgvg""#.to_string()),
        28
    );
    assert_eq!(
        encoded_str_len(&r#""\"onpqnssmighipuqgwx\"nrokzgvg""#.to_string()),
        40
    );
}

#[test]
fn str_len_ex_172() {
    assert_eq!(
        raw_str_len(&r#""vhjrrhfrba\"jebdanzsrdusut\\wbs""#.to_string()),
        33
    );
    assert_eq!(
        str_len(&r#""vhjrrhfrba\"jebdanzsrdusut\\wbs""#.to_string()),
        29
    );
    assert_eq!(
        encoded_str_len(&r#""vhjrrhfrba\"jebdanzsrdusut\\wbs""#.to_string()),
        41
    );
}

#[test]
fn str_len_ex_173() {
    assert_eq!(raw_str_len(&r#""o\xdakymbaxakys""#.to_string()), 17);
    assert_eq!(str_len(&r#""o\xdakymbaxakys""#.to_string()), 12);
    assert_eq!(encoded_str_len(&r#""o\xdakymbaxakys""#.to_string()), 22);
}

#[test]
fn str_len_ex_174() {
    assert_eq!(
        raw_str_len(&r#""uwxhhzz\\mtmhghjn\\\\tnhzbejj""#.to_string()),
        31
    );
    assert_eq!(
        str_len(&r#""uwxhhzz\\mtmhghjn\\\\tnhzbejj""#.to_string()),
        26
    );
    assert_eq!(
        encoded_str_len(&r#""uwxhhzz\\mtmhghjn\\\\tnhzbejj""#.to_string()),
        41
    );
}

#[test]
fn str_len_ex_175() {
    assert_eq!(raw_str_len(&r#""yd\\""#.to_string()), 6);
    assert_eq!(str_len(&r#""yd\\""#.to_string()), 3);
    assert_eq!(encoded_str_len(&r#""yd\\""#.to_string()), 12);
}

#[test]
fn str_len_ex_176() {
    assert_eq!(
        raw_str_len(&r#""bpgztp\\lzwpdqju\"it\x35qjhihjv""#.to_string()),
        33
    );
    assert_eq!(
        str_len(&r#""bpgztp\\lzwpdqju\"it\x35qjhihjv""#.to_string()),
        26
    );
    assert_eq!(
        encoded_str_len(&r#""bpgztp\\lzwpdqju\"it\x35qjhihjv""#.to_string()),
        42
    );
}

#[test]
fn str_len_ex_177() {
    assert_eq!(
        raw_str_len(&r#""\\my\\b\"klnnto\\\xb3mbtsh""#.to_string()),
        28
    );
    assert_eq!(str_len(&r#""\\my\\b\"klnnto\\\xb3mbtsh""#.to_string()), 19);
    assert_eq!(
        encoded_str_len(&r#""\\my\\b\"klnnto\\\xb3mbtsh""#.to_string()),
        41
    );
}

#[test]
fn str_len_ex_178() {
    assert_eq!(
        raw_str_len(&r#""ezyvknv\"l\x2bdhhfjcvwzhjgmhwbqd\"\\""#.to_string()),
        38
    );
    assert_eq!(
        str_len(&r#""ezyvknv\"l\x2bdhhfjcvwzhjgmhwbqd\"\\""#.to_string()),
        30
    );
    assert_eq!(
        encoded_str_len(&r#""ezyvknv\"l\x2bdhhfjcvwzhjgmhwbqd\"\\""#.to_string()),
        49
    );
}

#[test]
fn str_len_ex_179() {
    assert_eq!(
        raw_str_len(&r#""ftkz\"amoncbsohtaumhl\"wsodemopodq""#.to_string()),
        36
    );
    assert_eq!(
        str_len(&r#""ftkz\"amoncbsohtaumhl\"wsodemopodq""#.to_string()),
        32
    );
    assert_eq!(
        encoded_str_len(&r#""ftkz\"amoncbsohtaumhl\"wsodemopodq""#.to_string()),
        44
    );
}

#[test]
fn str_len_ex_180() {
    assert_eq!(raw_str_len(&r#""ifv""#.to_string()), 5);
    assert_eq!(str_len(&r#""ifv""#.to_string()), 3);
    assert_eq!(encoded_str_len(&r#""ifv""#.to_string()), 9);
}

#[test]
fn str_len_ex_181() {
    assert_eq!(raw_str_len(&r#""dmzfxvzq""#.to_string()), 10);
    assert_eq!(str_len(&r#""dmzfxvzq""#.to_string()), 8);
    assert_eq!(encoded_str_len(&r#""dmzfxvzq""#.to_string()), 14);
}

#[test]
fn str_len_ex_182() {
    assert_eq!(
        raw_str_len(&r#""sped\"bvmf\"mmevl\"zydannpfny""#.to_string()),
        31
    );
    assert_eq!(
        str_len(&r#""sped\"bvmf\"mmevl\"zydannpfny""#.to_string()),
        26
    );
    assert_eq!(
        encoded_str_len(&r#""sped\"bvmf\"mmevl\"zydannpfny""#.to_string()),
        41
    );
}

#[test]
fn str_len_ex_183() {
    assert_eq!(
        raw_str_len(&r#""fjxcjwlv\"pnqyrzatsjwsqfidb""#.to_string()),
        29
    );
    assert_eq!(str_len(&r#""fjxcjwlv\"pnqyrzatsjwsqfidb""#.to_string()), 26);
    assert_eq!(
        encoded_str_len(&r#""fjxcjwlv\"pnqyrzatsjwsqfidb""#.to_string()),
        35
    );
}

#[test]
fn str_len_ex_184() {
    assert_eq!(
        raw_str_len(&r#""muc\xfdqouwwnmuixru\\zlhjintplvtee""#.to_string()),
        36
    );
    assert_eq!(
        str_len(&r#""muc\xfdqouwwnmuixru\\zlhjintplvtee""#.to_string()),
        30
    );
    assert_eq!(
        encoded_str_len(&r#""muc\xfdqouwwnmuixru\\zlhjintplvtee""#.to_string()),
        43
    );
}

#[test]
fn str_len_ex_185() {
    assert_eq!(raw_str_len(&r#""mraqgvmj""#.to_string()), 10);
    assert_eq!(str_len(&r#""mraqgvmj""#.to_string()), 8);
    assert_eq!(encoded_str_len(&r#""mraqgvmj""#.to_string()), 14);
}

#[test]
fn str_len_ex_186() {
    assert_eq!(raw_str_len(&r#""njopq\"ftcsryo""#.to_string()), 16);
    assert_eq!(str_len(&r#""njopq\"ftcsryo""#.to_string()), 13);
    assert_eq!(encoded_str_len(&r#""njopq\"ftcsryo""#.to_string()), 22);
}

#[test]
fn str_len_ex_187() {
    assert_eq!(raw_str_len(&r#""enoh\"n""#.to_string()), 9);
    assert_eq!(str_len(&r#""enoh\"n""#.to_string()), 6);
    assert_eq!(encoded_str_len(&r#""enoh\"n""#.to_string()), 15);
}

#[test]
fn str_len_ex_188() {
    assert_eq!(
        raw_str_len(&r#""t\"ntjhjc\"nzqh\xf7dcohhlsja\x7dtr""#.to_string()),
        36
    );
    assert_eq!(
        str_len(&r#""t\"ntjhjc\"nzqh\xf7dcohhlsja\x7dtr""#.to_string()),
        26
    );
    assert_eq!(
        encoded_str_len(&r#""t\"ntjhjc\"nzqh\xf7dcohhlsja\x7dtr""#.to_string()),
        46
    );
}

#[test]
fn str_len_ex_189() {
    assert_eq!(raw_str_len(&r#""flbqcmcoun""#.to_string()), 12);
    assert_eq!(str_len(&r#""flbqcmcoun""#.to_string()), 10);
    assert_eq!(encoded_str_len(&r#""flbqcmcoun""#.to_string()), 16);
}

#[test]
fn str_len_ex_190() {
    assert_eq!(raw_str_len(&r#""dxkiysrn\\dyuqoaig""#.to_string()), 20);
    assert_eq!(str_len(&r#""dxkiysrn\\dyuqoaig""#.to_string()), 17);
    assert_eq!(encoded_str_len(&r#""dxkiysrn\\dyuqoaig""#.to_string()), 26);
}

#[test]
fn str_len_ex_191() {
    assert_eq!(
        raw_str_len(&r#""nehkzi\"h\"syktzfufotng\xdafqo""#.to_string()),
        32
    );
    assert_eq!(
        str_len(&r#""nehkzi\"h\"syktzfufotng\xdafqo""#.to_string()),
        25
    );
    assert_eq!(
        encoded_str_len(&r#""nehkzi\"h\"syktzfufotng\xdafqo""#.to_string()),
        41
    );
}

#[test]
fn str_len_ex_192() {
    assert_eq!(
        raw_str_len(&r#""dzkjg\\hqjk\\\"zfegssjhn""#.to_string()),
        26
    );
    assert_eq!(str_len(&r#""dzkjg\\hqjk\\\"zfegssjhn""#.to_string()), 21);
    assert_eq!(
        encoded_str_len(&r#""dzkjg\\hqjk\\\"zfegssjhn""#.to_string()),
        36
    );
}

#[test]
fn str_len_ex_193() {
    assert_eq!(raw_str_len(&r#""sadlsjv""#.to_string()), 9);
    assert_eq!(str_len(&r#""sadlsjv""#.to_string()), 7);
    assert_eq!(encoded_str_len(&r#""sadlsjv""#.to_string()), 13);
}

#[test]
fn str_len_ex_194() {
    assert_eq!(raw_str_len(&r#""vmfnrdb\"""#.to_string()), 11);
    assert_eq!(str_len(&r#""vmfnrdb\"""#.to_string()), 8);
    assert_eq!(encoded_str_len(&r#""vmfnrdb\"""#.to_string()), 17);
}

#[test]
fn str_len_ex_195() {
    assert_eq!(raw_str_len(&r#""ac\\bdp\"n""#.to_string()), 12);
    assert_eq!(str_len(&r#""ac\\bdp\"n""#.to_string()), 8);
    assert_eq!(encoded_str_len(&r#""ac\\bdp\"n""#.to_string()), 20);
}

#[test]
fn str_len_ex_196() {
    assert_eq!(raw_str_len(&r#""qt\x89h""#.to_string()), 9);
    assert_eq!(str_len(&r#""qt\x89h""#.to_string()), 4);
    assert_eq!(encoded_str_len(&r#""qt\x89h""#.to_string()), 14);
}

#[test]
fn str_len_ex_197() {
    assert_eq!(
        raw_str_len(&r#""lsndeugwvijwde\\vjapbm\\k\\nljuva""#.to_string()),
        35
    );
    assert_eq!(
        str_len(&r#""lsndeugwvijwde\\vjapbm\\k\\nljuva""#.to_string()),
        30
    );
    assert_eq!(
        encoded_str_len(&r#""lsndeugwvijwde\\vjapbm\\k\\nljuva""#.to_string()),
        45
    );
}

#[test]
fn str_len_ex_198() {
    assert_eq!(
        raw_str_len(&r#""twpmltdzyynqt\\z\\tnund\x64hm""#.to_string()),
        31
    );
    assert_eq!(
        str_len(&r#""twpmltdzyynqt\\z\\tnund\x64hm""#.to_string()),
        24
    );
    assert_eq!(
        encoded_str_len(&r#""twpmltdzyynqt\\z\\tnund\x64hm""#.to_string()),
        40
    );
}

#[test]
fn str_len_ex_199() {
    assert_eq!(raw_str_len(&r#""hpcyata\"ocylbkzdnhujh""#.to_string()), 24);
    assert_eq!(str_len(&r#""hpcyata\"ocylbkzdnhujh""#.to_string()), 21);
    assert_eq!(
        encoded_str_len(&r#""hpcyata\"ocylbkzdnhujh""#.to_string()),
        30
    );
}

#[test]
fn str_len_ex_200() {
    assert_eq!(
        raw_str_len(&r#""hskzq\"knntuhscex\"q\\y\\vqj\x3an""#.to_string()),
        35
    );
    assert_eq!(
        str_len(&r#""hskzq\"knntuhscex\"q\\y\\vqj\x3an""#.to_string()),
        26
    );
    assert_eq!(
        encoded_str_len(&r#""hskzq\"knntuhscex\"q\\y\\vqj\x3an""#.to_string()),
        48
    );
}

#[test]
fn str_len_ex_201() {
    assert_eq!(
        raw_str_len(&r#""eekwyufvji\\mqgeroekxeyrmymq""#.to_string()),
        30
    );
    assert_eq!(
        str_len(&r#""eekwyufvji\\mqgeroekxeyrmymq""#.to_string()),
        27
    );
    assert_eq!(
        encoded_str_len(&r#""eekwyufvji\\mqgeroekxeyrmymq""#.to_string()),
        36
    );
}

#[test]
fn str_len_ex_202() {
    assert_eq!(
        raw_str_len(&r#""hl\"durthetvri\xebw\\jxu\"rcmiuy""#.to_string()),
        34
    );
    assert_eq!(
        str_len(&r#""hl\"durthetvri\xebw\\jxu\"rcmiuy""#.to_string()),
        26
    );
    assert_eq!(
        encoded_str_len(&r#""hl\"durthetvri\xebw\\jxu\"rcmiuy""#.to_string()),
        45
    );
}

#[test]
fn str_len_ex_203() {
    assert_eq!(
        raw_str_len(&r#""\"fxdnmvnftxwesmvvq\"sjnf\xaabpg\"iary""#.to_string()),
        40
    );
    assert_eq!(
        str_len(&r#""\"fxdnmvnftxwesmvvq\"sjnf\xaabpg\"iary""#.to_string()),
        32
    );
    assert_eq!(
        encoded_str_len(&r#""\"fxdnmvnftxwesmvvq\"sjnf\xaabpg\"iary""#.to_string()),
        51
    );
}

#[test]
fn str_len_ex_204() {
    assert_eq!(raw_str_len(&r#""\"\"nksqso""#.to_string()), 12);
    assert_eq!(str_len(&r#""\"\"nksqso""#.to_string()), 8);
    assert_eq!(encoded_str_len(&r#""\"\"nksqso""#.to_string()), 20);
}

#[test]
fn str_len_ex_205() {
    assert_eq!(
        raw_str_len(&r#""ruq\xbezugge\"d\"hwvoxmy\"iawikddxn\"x""#.to_string()),
        40
    );
    assert_eq!(
        str_len(&r#""ruq\xbezugge\"d\"hwvoxmy\"iawikddxn\"x""#.to_string()),
        31
    );
    assert_eq!(
        encoded_str_len(&r#""ruq\xbezugge\"d\"hwvoxmy\"iawikddxn\"x""#.to_string()),
        53
    );
}

#[test]
fn str_len_ex_206() {
    assert_eq!(raw_str_len(&r#""rxxnlfay""#.to_string()), 10);
    assert_eq!(str_len(&r#""rxxnlfay""#.to_string()), 8);
    assert_eq!(encoded_str_len(&r#""rxxnlfay""#.to_string()), 14);
}

#[test]
fn str_len_ex_207() {
    assert_eq!(raw_str_len(&r#""stcu\"mv\xabcqts\\fasff""#.to_string()), 25);
    assert_eq!(str_len(&r#""stcu\"mv\xabcqts\\fasff""#.to_string()), 18);
    assert_eq!(
        encoded_str_len(&r#""stcu\"mv\xabcqts\\fasff""#.to_string()),
        34
    );
}

#[test]
fn str_len_ex_208() {
    assert_eq!(
        raw_str_len(&r#""yrnvwfkfuzuoysfdzl\x02bk""#.to_string()),
        26
    );
    assert_eq!(str_len(&r#""yrnvwfkfuzuoysfdzl\x02bk""#.to_string()), 21);
    assert_eq!(
        encoded_str_len(&r#""yrnvwfkfuzuoysfdzl\x02bk""#.to_string()),
        31
    );
}

#[test]
fn str_len_ex_209() {
    assert_eq!(
        raw_str_len(&r#""qbdsmlwdbfknivtwijbwtatqfe""#.to_string()),
        28
    );
    assert_eq!(str_len(&r#""qbdsmlwdbfknivtwijbwtatqfe""#.to_string()), 26);
    assert_eq!(
        encoded_str_len(&r#""qbdsmlwdbfknivtwijbwtatqfe""#.to_string()),
        32
    );
}

#[test]
fn str_len_ex_210() {
    assert_eq!(raw_str_len(&r#""\"erqh\\csjph""#.to_string()), 15);
    assert_eq!(str_len(&r#""\"erqh\\csjph""#.to_string()), 11);
    assert_eq!(encoded_str_len(&r#""\"erqh\\csjph""#.to_string()), 23);
}

#[test]
fn str_len_ex_211() {
    assert_eq!(raw_str_len(&r#""ikfv""#.to_string()), 6);
    assert_eq!(str_len(&r#""ikfv""#.to_string()), 4);
    assert_eq!(encoded_str_len(&r#""ikfv""#.to_string()), 10);
}

#[test]
fn str_len_ex_212() {
    assert_eq!(
        raw_str_len(&r#""\xd2cuhowmtsxepzsivsvnvsb""#.to_string()),
        27
    );
    assert_eq!(str_len(&r#""\xd2cuhowmtsxepzsivsvnvsb""#.to_string()), 22);
    assert_eq!(
        encoded_str_len(&r#""\xd2cuhowmtsxepzsivsvnvsb""#.to_string()),
        32
    );
}

#[test]
fn str_len_ex_213() {
    assert_eq!(raw_str_len(&r#""vj""#.to_string()), 4);
    assert_eq!(str_len(&r#""vj""#.to_string()), 2);
    assert_eq!(encoded_str_len(&r#""vj""#.to_string()), 8);
}

#[test]
fn str_len_ex_214() {
    assert_eq!(raw_str_len(&r#""d""#.to_string()), 3);
    assert_eq!(str_len(&r#""d""#.to_string()), 1);
    assert_eq!(encoded_str_len(&r#""d""#.to_string()), 7);
}

#[test]
fn str_len_ex_215() {
    assert_eq!(raw_str_len(&r#""\\g""#.to_string()), 5);
    assert_eq!(str_len(&r#""\\g""#.to_string()), 2);
    assert_eq!(encoded_str_len(&r#""\\g""#.to_string()), 11);
}

#[test]
fn str_len_ex_216() {
    assert_eq!(raw_str_len(&r#""porvg\x62qghorthnc\"\\""#.to_string()), 24);
    assert_eq!(str_len(&r#""porvg\x62qghorthnc\"\\""#.to_string()), 17);
    assert_eq!(
        encoded_str_len(&r#""porvg\x62qghorthnc\"\\""#.to_string()),
        33
    );
}

#[test]
fn str_len_ex_217() {
    assert_eq!(
        raw_str_len(&r#""tiks\\kr\"\x0fuejvuxzswnwdjscrk""#.to_string()),
        33
    );
    assert_eq!(
        str_len(&r#""tiks\\kr\"\x0fuejvuxzswnwdjscrk""#.to_string()),
        26
    );
    assert_eq!(
        encoded_str_len(&r#""tiks\\kr\"\x0fuejvuxzswnwdjscrk""#.to_string()),
        42
    );
}

#[test]
fn str_len_ex_218() {
    assert_eq!(
        raw_str_len(&r#""xmgfel\"atma\\zaxmlgfjx\"ajmqf""#.to_string()),
        32
    );
    assert_eq!(
        str_len(&r#""xmgfel\"atma\\zaxmlgfjx\"ajmqf""#.to_string()),
        27
    );
    assert_eq!(
        encoded_str_len(&r#""xmgfel\"atma\\zaxmlgfjx\"ajmqf""#.to_string()),
        42
    );
}

#[test]
fn str_len_ex_219() {
    assert_eq!(raw_str_len(&r#""oz\\rnxwljc\\\"umhymtwh""#.to_string()), 25);
    assert_eq!(str_len(&r#""oz\\rnxwljc\\\"umhymtwh""#.to_string()), 20);
    assert_eq!(
        encoded_str_len(&r#""oz\\rnxwljc\\\"umhymtwh""#.to_string()),
        35
    );
}

#[test]
fn str_len_ex_220() {
    assert_eq!(
        raw_str_len(&r#""wlsxxhm\x7fqx\\gjoyrvccfiner\\qloluqv""#.to_string()),
        39
    );
    assert_eq!(
        str_len(&r#""wlsxxhm\x7fqx\\gjoyrvccfiner\\qloluqv""#.to_string()),
        32
    );
    assert_eq!(
        encoded_str_len(&r#""wlsxxhm\x7fqx\\gjoyrvccfiner\\qloluqv""#.to_string()),
        48
    );
}

#[test]
fn str_len_ex_221() {
    assert_eq!(raw_str_len(&r#""k\\ieq""#.to_string()), 8);
    assert_eq!(str_len(&r#""k\\ieq""#.to_string()), 5);
    assert_eq!(encoded_str_len(&r#""k\\ieq""#.to_string()), 14);
}

#[test]
fn str_len_ex_222() {
    assert_eq!(
        raw_str_len(&r#""xidjj\"ksnlgnwxlddf\\s\\kuuleb""#.to_string()),
        32
    );
    assert_eq!(
        str_len(&r#""xidjj\"ksnlgnwxlddf\\s\\kuuleb""#.to_string()),
        27
    );
    assert_eq!(
        encoded_str_len(&r#""xidjj\"ksnlgnwxlddf\\s\\kuuleb""#.to_string()),
        42
    );
}

#[test]
fn str_len_ex_223() {
    assert_eq!(raw_str_len(&r#""wjpnzgprzv\\maub\x0cj""#.to_string()), 23);
    assert_eq!(str_len(&r#""wjpnzgprzv\\maub\x0cj""#.to_string()), 17);
    assert_eq!(
        encoded_str_len(&r#""wjpnzgprzv\\maub\x0cj""#.to_string()),
        30
    );
}

#[test]
fn str_len_ex_224() {
    assert_eq!(raw_str_len(&r#""r""#.to_string()), 3);
    assert_eq!(str_len(&r#""r""#.to_string()), 1);
    assert_eq!(encoded_str_len(&r#""r""#.to_string()), 7);
}

#[test]
fn str_len_ex_225() {
    assert_eq!(raw_str_len(&r#""y""#.to_string()), 3);
    assert_eq!(str_len(&r#""y""#.to_string()), 1);
    assert_eq!(encoded_str_len(&r#""y""#.to_string()), 7);
}

#[test]
fn str_len_ex_226() {
    assert_eq!(
        raw_str_len(&r#""\"yecqiei\"ire\\jdhlnnlde\xc5u""#.to_string()),
        32
    );
    assert_eq!(
        str_len(&r#""\"yecqiei\"ire\\jdhlnnlde\xc5u""#.to_string()),
        24
    );
    assert_eq!(
        encoded_str_len(&r#""\"yecqiei\"ire\\jdhlnnlde\xc5u""#.to_string()),
        43
    );
}

#[test]
fn str_len_ex_227() {
    assert_eq!(raw_str_len(&r#""drvdiycqib""#.to_string()), 12);
    assert_eq!(str_len(&r#""drvdiycqib""#.to_string()), 10);
    assert_eq!(encoded_str_len(&r#""drvdiycqib""#.to_string()), 16);
}

#[test]
fn str_len_ex_228() {
    assert_eq!(raw_str_len(&r#""egnrbefezcrhgldrtb""#.to_string()), 20);
    assert_eq!(str_len(&r#""egnrbefezcrhgldrtb""#.to_string()), 18);
    assert_eq!(encoded_str_len(&r#""egnrbefezcrhgldrtb""#.to_string()), 24);
}

#[test]
fn str_len_ex_229() {
    assert_eq!(
        raw_str_len(&r#""plqodxv\\zm\"uodwjdocri\x55ucaezutm""#.to_string()),
        37
    );
    assert_eq!(
        str_len(&r#""plqodxv\\zm\"uodwjdocri\x55ucaezutm""#.to_string()),
        30
    );
    assert_eq!(
        encoded_str_len(&r#""plqodxv\\zm\"uodwjdocri\x55ucaezutm""#.to_string()),
        46
    );
}

#[test]
fn str_len_ex_230() {
    assert_eq!(
        raw_str_len(&r#""f\"wexcw\x02ekewx\"alyzn""#.to_string()),
        26
    );
    assert_eq!(str_len(&r#""f\"wexcw\x02ekewx\"alyzn""#.to_string()), 19);
    assert_eq!(
        encoded_str_len(&r#""f\"wexcw\x02ekewx\"alyzn""#.to_string()),
        35
    );
}

#[test]
fn str_len_ex_231() {
    assert_eq!(
        raw_str_len(&r#""pqajwuk\\\\oatkfqdyspnrupo""#.to_string()),
        28
    );
    assert_eq!(str_len(&r#""pqajwuk\\\\oatkfqdyspnrupo""#.to_string()), 24);
    assert_eq!(
        encoded_str_len(&r#""pqajwuk\\\\oatkfqdyspnrupo""#.to_string()),
        36
    );
}

#[test]
fn str_len_ex_232() {
    assert_eq!(
        raw_str_len(&r#""rkczj\"fzntabpnygrhamk\\km\x68xfkmr""#.to_string()),
        37
    );
    assert_eq!(
        str_len(&r#""rkczj\"fzntabpnygrhamk\\km\x68xfkmr""#.to_string()),
        30
    );
    assert_eq!(
        encoded_str_len(&r#""rkczj\"fzntabpnygrhamk\\km\x68xfkmr""#.to_string()),
        46
    );
}

#[test]
fn str_len_ex_233() {
    assert_eq!(raw_str_len(&r#""wejam\xbac\x37kns""#.to_string()), 19);
    assert_eq!(str_len(&r#""wejam\xbac\x37kns""#.to_string()), 11);
    assert_eq!(encoded_str_len(&r#""wejam\xbac\x37kns""#.to_string()), 25);
}

#[test]
fn str_len_ex_234() {
    assert_eq!(raw_str_len(&r#""qqmlwjk\"gh""#.to_string()), 13);
    assert_eq!(str_len(&r#""qqmlwjk\"gh""#.to_string()), 10);
    assert_eq!(encoded_str_len(&r#""qqmlwjk\"gh""#.to_string()), 19);
}

#[test]
fn str_len_ex_235() {
    assert_eq!(raw_str_len(&r#""fdcjsxlgx""#.to_string()), 11);
    assert_eq!(str_len(&r#""fdcjsxlgx""#.to_string()), 9);
    assert_eq!(encoded_str_len(&r#""fdcjsxlgx""#.to_string()), 15);
}

#[test]
fn str_len_ex_236() {
    assert_eq!(
        raw_str_len(&r#""\\cxvxy\"kb\"\"unubvrsq\\y\\awfhbmarj\\""#.to_string()),
        41
    );
    assert_eq!(
        str_len(&r#""\\cxvxy\"kb\"\"unubvrsq\\y\\awfhbmarj\\""#.to_string()),
        32
    );
    assert_eq!(
        encoded_str_len(&r#""\\cxvxy\"kb\"\"unubvrsq\\y\\awfhbmarj\\""#.to_string()),
        59
    );
}

#[test]
fn str_len_ex_237() {
    assert_eq!(raw_str_len(&r#""geunceaqr""#.to_string()), 11);
    assert_eq!(str_len(&r#""geunceaqr""#.to_string()), 9);
    assert_eq!(encoded_str_len(&r#""geunceaqr""#.to_string()), 15);
}

#[test]
fn str_len_ex_238() {
    assert_eq!(
        raw_str_len(&r#""tpkg\"svvngk\\sizlsyaqwf""#.to_string()),
        26
    );
    assert_eq!(str_len(&r#""tpkg\"svvngk\\sizlsyaqwf""#.to_string()), 22);
    assert_eq!(
        encoded_str_len(&r#""tpkg\"svvngk\\sizlsyaqwf""#.to_string()),
        34
    );
}

#[test]
fn str_len_ex_239() {
    assert_eq!(raw_str_len(&r#""\"pa\\x\x18od\\emgje\\""#.to_string()), 24);
    assert_eq!(str_len(&r#""\"pa\\x\x18od\\emgje\\""#.to_string()), 15);
    assert_eq!(
        encoded_str_len(&r#""\"pa\\x\x18od\\emgje\\""#.to_string()),
        37
    );
}

#[test]
fn str_len_ex_240() {
    assert_eq!(
        raw_str_len(&r#""ffiizogjjptubzqfuh\"cctieqcdh""#.to_string()),
        31
    );
    assert_eq!(
        str_len(&r#""ffiizogjjptubzqfuh\"cctieqcdh""#.to_string()),
        28
    );
    assert_eq!(
        encoded_str_len(&r#""ffiizogjjptubzqfuh\"cctieqcdh""#.to_string()),
        37
    );
}

#[test]
fn str_len_ex_241() {
    assert_eq!(raw_str_len(&r#""yikhiyyrpgglpos""#.to_string()), 17);
    assert_eq!(str_len(&r#""yikhiyyrpgglpos""#.to_string()), 15);
    assert_eq!(encoded_str_len(&r#""yikhiyyrpgglpos""#.to_string()), 21);
}

#[test]
fn str_len_ex_242() {
    assert_eq!(raw_str_len(&r#""h\\""#.to_string()), 5);
    assert_eq!(str_len(&r#""h\\""#.to_string()), 2);
    assert_eq!(encoded_str_len(&r#""h\\""#.to_string()), 11);
}

#[test]
fn str_len_ex_243() {
    assert_eq!(raw_str_len(&r#""jotqojodcv""#.to_string()), 12);
    assert_eq!(str_len(&r#""jotqojodcv""#.to_string()), 10);
    assert_eq!(encoded_str_len(&r#""jotqojodcv""#.to_string()), 16);
}

#[test]
fn str_len_ex_244() {
    assert_eq!(
        raw_str_len(&r#""ervsz\x87ade\"fevq\\tcqowt""#.to_string()),
        28
    );
    assert_eq!(str_len(&r#""ervsz\x87ade\"fevq\\tcqowt""#.to_string()), 21);
    assert_eq!(
        encoded_str_len(&r#""ervsz\x87ade\"fevq\\tcqowt""#.to_string()),
        37
    );
}

#[test]
fn str_len_ex_245() {
    assert_eq!(
        raw_str_len(&r#""\\y\"fgrxtppkcseeg\\onxjarx\\hyhfn\x5fi""#.to_string()),
        41
    );
    assert_eq!(
        str_len(&r#""\\y\"fgrxtppkcseeg\\onxjarx\\hyhfn\x5fi""#.to_string()),
        32
    );
    assert_eq!(
        encoded_str_len(&r#""\\y\"fgrxtppkcseeg\\onxjarx\\hyhfn\x5fi""#.to_string()),
        54
    );
}

#[test]
fn str_len_ex_246() {
    assert_eq!(
        raw_str_len(&r#""kxndlabn\\wwumctuzdcfiitrbnn""#.to_string()),
        30
    );
    assert_eq!(
        str_len(&r#""kxndlabn\\wwumctuzdcfiitrbnn""#.to_string()),
        27
    );
    assert_eq!(
        encoded_str_len(&r#""kxndlabn\\wwumctuzdcfiitrbnn""#.to_string()),
        36
    );
}

#[test]
fn str_len_ex_247() {
    assert_eq!(raw_str_len(&r#""eoosynwhwm""#.to_string()), 12);
    assert_eq!(str_len(&r#""eoosynwhwm""#.to_string()), 10);
    assert_eq!(encoded_str_len(&r#""eoosynwhwm""#.to_string()), 16);
}

#[test]
fn str_len_ex_248() {
    assert_eq!(raw_str_len(&r#""\"c\x04""#.to_string()), 9);
    assert_eq!(str_len(&r#""\"c\x04""#.to_string()), 3);
    assert_eq!(encoded_str_len(&r#""\"c\x04""#.to_string()), 16);
}

#[test]
fn str_len_ex_249() {
    assert_eq!(raw_str_len(&r#""ny\xf6vuwlec""#.to_string()), 14);
    assert_eq!(str_len(&r#""ny\xf6vuwlec""#.to_string()), 9);
    assert_eq!(encoded_str_len(&r#""ny\xf6vuwlec""#.to_string()), 19);
}

#[test]
fn str_len_ex_250() {
    assert_eq!(raw_str_len(&r#""ubgxxcvnltzaucrzg\\xcez""#.to_string()), 25);
    assert_eq!(str_len(&r#""ubgxxcvnltzaucrzg\\xcez""#.to_string()), 22);
    assert_eq!(
        encoded_str_len(&r#""ubgxxcvnltzaucrzg\\xcez""#.to_string()),
        31
    );
}

#[test]
fn str_len_ex_251() {
    assert_eq!(raw_str_len(&r#""pnocjvo\\yt""#.to_string()), 13);
    assert_eq!(str_len(&r#""pnocjvo\\yt""#.to_string()), 10);
    assert_eq!(encoded_str_len(&r#""pnocjvo\\yt""#.to_string()), 19);
}

#[test]
fn str_len_ex_252() {
    assert_eq!(raw_str_len(&r#""fcabrtqog\"a\"zj""#.to_string()), 18);
    assert_eq!(str_len(&r#""fcabrtqog\"a\"zj""#.to_string()), 14);
    assert_eq!(encoded_str_len(&r#""fcabrtqog\"a\"zj""#.to_string()), 26);
}

#[test]
fn str_len_ex_253() {
    assert_eq!(
        raw_str_len(&r#""o\\bha\\mzxmrfltnflv\xea""#.to_string()),
        26
    );
    assert_eq!(str_len(&r#""o\\bha\\mzxmrfltnflv\xea""#.to_string()), 19);
    assert_eq!(
        encoded_str_len(&r#""o\\bha\\mzxmrfltnflv\xea""#.to_string()),
        35
    );
}

#[test]
fn str_len_ex_254() {
    assert_eq!(
        raw_str_len(&r#""tbfvzwhexsdxjmxejwqqngzixcx""#.to_string()),
        29
    );
    assert_eq!(str_len(&r#""tbfvzwhexsdxjmxejwqqngzixcx""#.to_string()), 27);
    assert_eq!(
        encoded_str_len(&r#""tbfvzwhexsdxjmxejwqqngzixcx""#.to_string()),
        33
    );
}

#[test]
fn str_len_ex_255() {
    assert_eq!(
        raw_str_len(&r#""wdptrakok\"rgymturdmwfiwu""#.to_string()),
        27
    );
    assert_eq!(str_len(&r#""wdptrakok\"rgymturdmwfiwu""#.to_string()), 24);
    assert_eq!(
        encoded_str_len(&r#""wdptrakok\"rgymturdmwfiwu""#.to_string()),
        33
    );
}

#[test]
fn str_len_ex_256() {
    assert_eq!(raw_str_len(&r#""reffmj""#.to_string()), 8);
    assert_eq!(str_len(&r#""reffmj""#.to_string()), 6);
    assert_eq!(encoded_str_len(&r#""reffmj""#.to_string()), 12);
}

#[test]
fn str_len_ex_257() {
    assert_eq!(raw_str_len(&r#""lqm""#.to_string()), 5);
    assert_eq!(str_len(&r#""lqm""#.to_string()), 3);
    assert_eq!(encoded_str_len(&r#""lqm""#.to_string()), 9);
}

#[test]
fn str_len_ex_258() {
    assert_eq!(raw_str_len(&r#""\\oc""#.to_string()), 6);
    assert_eq!(str_len(&r#""\\oc""#.to_string()), 3);
    assert_eq!(encoded_str_len(&r#""\\oc""#.to_string()), 12);
}

#[test]
fn str_len_ex_259() {
    assert_eq!(raw_str_len(&r#""p\"""#.to_string()), 5);
    assert_eq!(str_len(&r#""p\"""#.to_string()), 2);
    assert_eq!(encoded_str_len(&r#""p\"""#.to_string()), 11);
}

#[test]
fn str_len_ex_260() {
    assert_eq!(raw_str_len(&r#""ygkdnhcuehlx""#.to_string()), 14);
    assert_eq!(str_len(&r#""ygkdnhcuehlx""#.to_string()), 12);
    assert_eq!(encoded_str_len(&r#""ygkdnhcuehlx""#.to_string()), 18);
}

#[test]
fn str_len_ex_261() {
    assert_eq!(
        raw_str_len(&r#""vsqmv\"bqay\"olimtkewedzm""#.to_string()),
        27
    );
    assert_eq!(str_len(&r#""vsqmv\"bqay\"olimtkewedzm""#.to_string()), 23);
    assert_eq!(
        encoded_str_len(&r#""vsqmv\"bqay\"olimtkewedzm""#.to_string()),
        35
    );
}

#[test]
fn str_len_ex_262() {
    assert_eq!(
        raw_str_len(&r#""isos\x6azbnkojhxoopzetbj\xe1yd""#.to_string()),
        32
    );
    assert_eq!(
        str_len(&r#""isos\x6azbnkojhxoopzetbj\xe1yd""#.to_string()),
        24
    );
    assert_eq!(
        encoded_str_len(&r#""isos\x6azbnkojhxoopzetbj\xe1yd""#.to_string()),
        38
    );
}

#[test]
fn str_len_ex_263() {
    assert_eq!(raw_str_len(&r#""yo\\pgayjcyhshztnbdv""#.to_string()), 22);
    assert_eq!(str_len(&r#""yo\\pgayjcyhshztnbdv""#.to_string()), 19);
    assert_eq!(
        encoded_str_len(&r#""yo\\pgayjcyhshztnbdv""#.to_string()),
        28
    );
}

#[test]
fn str_len_ex_264() {
    assert_eq!(raw_str_len(&r#""fg\"h""#.to_string()), 7);
    assert_eq!(str_len(&r#""fg\"h""#.to_string()), 4);
    assert_eq!(encoded_str_len(&r#""fg\"h""#.to_string()), 13);
}

#[test]
fn str_len_ex_265() {
    assert_eq!(raw_str_len(&r#""vcmcojolfcf\\\\oxveua""#.to_string()), 23);
    assert_eq!(str_len(&r#""vcmcojolfcf\\\\oxveua""#.to_string()), 19);
    assert_eq!(
        encoded_str_len(&r#""vcmcojolfcf\\\\oxveua""#.to_string()),
        31
    );
}

#[test]
fn str_len_ex_266() {
    assert_eq!(
        raw_str_len(&r#""w\"vyszhbrr\"jpeddpnrjlca\x69bdbopd\\z""#.to_string()),
        40
    );
    assert_eq!(
        str_len(&r#""w\"vyszhbrr\"jpeddpnrjlca\x69bdbopd\\z""#.to_string()),
        32
    );
    assert_eq!(
        encoded_str_len(&r#""w\"vyszhbrr\"jpeddpnrjlca\x69bdbopd\\z""#.to_string()),
        51
    );
}

#[test]
fn str_len_ex_267() {
    assert_eq!(raw_str_len(&r#""jikeqv""#.to_string()), 8);
    assert_eq!(str_len(&r#""jikeqv""#.to_string()), 6);
    assert_eq!(encoded_str_len(&r#""jikeqv""#.to_string()), 12);
}

#[test]
fn str_len_ex_268() {
    assert_eq!(raw_str_len(&r#""\"dkjdfrtj""#.to_string()), 12);
    assert_eq!(str_len(&r#""\"dkjdfrtj""#.to_string()), 9);
    assert_eq!(encoded_str_len(&r#""\"dkjdfrtj""#.to_string()), 18);
}

#[test]
fn str_len_ex_269() {
    assert_eq!(raw_str_len(&r#""is""#.to_string()), 4);
    assert_eq!(str_len(&r#""is""#.to_string()), 2);
    assert_eq!(encoded_str_len(&r#""is""#.to_string()), 8);
}

#[test]
fn str_len_ex_270() {
    assert_eq!(raw_str_len(&r#""hgzx""#.to_string()), 6);
    assert_eq!(str_len(&r#""hgzx""#.to_string()), 4);
    assert_eq!(encoded_str_len(&r#""hgzx""#.to_string()), 10);
}

#[test]
fn str_len_ex_271() {
    assert_eq!(raw_str_len(&r#""z\"""#.to_string()), 5);
    assert_eq!(str_len(&r#""z\"""#.to_string()), 2);
    assert_eq!(encoded_str_len(&r#""z\"""#.to_string()), 11);
}

#[test]
fn str_len_ex_272() {
    assert_eq!(raw_str_len(&r#""woubquq\\ag\"""#.to_string()), 15);
    assert_eq!(str_len(&r#""woubquq\\ag\"""#.to_string()), 11);
    assert_eq!(encoded_str_len(&r#""woubquq\\ag\"""#.to_string()), 23);
}

#[test]
fn str_len_ex_273() {
    assert_eq!(raw_str_len(&r#""xvclriqa\xe6ltt""#.to_string()), 17);
    assert_eq!(str_len(&r#""xvclriqa\xe6ltt""#.to_string()), 12);
    assert_eq!(encoded_str_len(&r#""xvclriqa\xe6ltt""#.to_string()), 22);
}

#[test]
fn str_len_ex_274() {
    assert_eq!(raw_str_len(&r#""tfxinifmd""#.to_string()), 11);
    assert_eq!(str_len(&r#""tfxinifmd""#.to_string()), 9);
    assert_eq!(encoded_str_len(&r#""tfxinifmd""#.to_string()), 15);
}

#[test]
fn str_len_ex_275() {
    assert_eq!(raw_str_len(&r#""mvywzf\"jz""#.to_string()), 12);
    assert_eq!(str_len(&r#""mvywzf\"jz""#.to_string()), 9);
    assert_eq!(encoded_str_len(&r#""mvywzf\"jz""#.to_string()), 18);
}

#[test]
fn str_len_ex_276() {
    assert_eq!(raw_str_len(&r#""vlle""#.to_string()), 6);
    assert_eq!(str_len(&r#""vlle""#.to_string()), 4);
    assert_eq!(encoded_str_len(&r#""vlle""#.to_string()), 10);
}

#[test]
fn str_len_ex_277() {
    assert_eq!(
        raw_str_len(&r#""c\"rf\"wynhye\x25vccvb\"""#.to_string()),
        26
    );
    assert_eq!(str_len(&r#""c\"rf\"wynhye\x25vccvb\"""#.to_string()), 18);
    assert_eq!(
        encoded_str_len(&r#""c\"rf\"wynhye\x25vccvb\"""#.to_string()),
        37
    );
}

#[test]
fn str_len_ex_278() {
    assert_eq!(raw_str_len(&r#""zvuxm""#.to_string()), 7);
    assert_eq!(str_len(&r#""zvuxm""#.to_string()), 5);
    assert_eq!(encoded_str_len(&r#""zvuxm""#.to_string()), 11);
}

#[test]
fn str_len_ex_279() {
    assert_eq!(raw_str_len(&r#""\xf2\"jdstiwqer\"h""#.to_string()), 20);
    assert_eq!(str_len(&r#""\xf2\"jdstiwqer\"h""#.to_string()), 13);
    assert_eq!(encoded_str_len(&r#""\xf2\"jdstiwqer\"h""#.to_string()), 29);
}

#[test]
fn str_len_ex_280() {
    assert_eq!(raw_str_len(&r#""kyogyogcknbzv\x9f\\\\e""#.to_string()), 24);
    assert_eq!(str_len(&r#""kyogyogcknbzv\x9f\\\\e""#.to_string()), 17);
    assert_eq!(
        encoded_str_len(&r#""kyogyogcknbzv\x9f\\\\e""#.to_string()),
        33
    );
}

#[test]
fn str_len_ex_281() {
    assert_eq!(raw_str_len(&r#""kspodj\"edpeqgypc""#.to_string()), 19);
    assert_eq!(str_len(&r#""kspodj\"edpeqgypc""#.to_string()), 16);
    assert_eq!(encoded_str_len(&r#""kspodj\"edpeqgypc""#.to_string()), 25);
}

#[test]
fn str_len_ex_282() {
    assert_eq!(raw_str_len(&r#""oh\\x\\h""#.to_string()), 10);
    assert_eq!(str_len(&r#""oh\\x\\h""#.to_string()), 6);
    assert_eq!(encoded_str_len(&r#""oh\\x\\h""#.to_string()), 18);
}

#[test]
fn str_len_ex_283() {
    assert_eq!(raw_str_len(&r#""julb""#.to_string()), 6);
    assert_eq!(str_len(&r#""julb""#.to_string()), 4);
    assert_eq!(encoded_str_len(&r#""julb""#.to_string()), 10);
}

#[test]
fn str_len_ex_284() {
    assert_eq!(
        raw_str_len(&r#""bmcfkidxyilgoy\\xmu\"ig\\qg""#.to_string()),
        29
    );
    assert_eq!(str_len(&r#""bmcfkidxyilgoy\\xmu\"ig\\qg""#.to_string()), 24);
    assert_eq!(
        encoded_str_len(&r#""bmcfkidxyilgoy\\xmu\"ig\\qg""#.to_string()),
        39
    );
}

#[test]
fn str_len_ex_285() {
    assert_eq!(raw_str_len(&r#""veqww\"ea""#.to_string()), 11);
    assert_eq!(str_len(&r#""veqww\"ea""#.to_string()), 8);
    assert_eq!(encoded_str_len(&r#""veqww\"ea""#.to_string()), 17);
}

#[test]
fn str_len_ex_286() {
    assert_eq!(
        raw_str_len(&r#""fkdbemtgtkpqisrwlxutllxc\"mbelhs""#.to_string()),
        34
    );
    assert_eq!(
        str_len(&r#""fkdbemtgtkpqisrwlxutllxc\"mbelhs""#.to_string()),
        31
    );
    assert_eq!(
        encoded_str_len(&r#""fkdbemtgtkpqisrwlxutllxc\"mbelhs""#.to_string()),
        40
    );
}

#[test]
fn str_len_ex_287() {
    assert_eq!(raw_str_len(&r#""e""#.to_string()), 3);
    assert_eq!(str_len(&r#""e""#.to_string()), 1);
    assert_eq!(encoded_str_len(&r#""e""#.to_string()), 7);
}

#[test]
fn str_len_ex_288() {
    assert_eq!(raw_str_len(&r#""ecn\x50ooprbstnq""#.to_string()), 18);
    assert_eq!(str_len(&r#""ecn\x50ooprbstnq""#.to_string()), 13);
    assert_eq!(encoded_str_len(&r#""ecn\x50ooprbstnq""#.to_string()), 23);
}

#[test]
fn str_len_ex_289() {
    assert_eq!(
        raw_str_len(&r#""\"\xe8\"ec\xeah\"qo\\g\"iuqxy\"e\"y\xe7xk\xc6d""#.to_string()),
        48
    );
    assert_eq!(
        str_len(&r#""\"\xe8\"ec\xeah\"qo\\g\"iuqxy\"e\"y\xe7xk\xc6d""#.to_string()),
        27
    );
    assert_eq!(
        encoded_str_len(&r#""\"\xe8\"ec\xeah\"qo\\g\"iuqxy\"e\"y\xe7xk\xc6d""#.to_string()),
        70
    );
}

#[test]
fn str_len_ex_290() {
    assert_eq!(raw_str_len(&r#""lwj\"aftrcqj""#.to_string()), 14);
    assert_eq!(str_len(&r#""lwj\"aftrcqj""#.to_string()), 11);
    assert_eq!(encoded_str_len(&r#""lwj\"aftrcqj""#.to_string()), 20);
}

#[test]
fn str_len_ex_291() {
    assert_eq!(
        raw_str_len(&r#""jduij\x97zk\"rftjrixzgscxxllpqx\"bwwb""#.to_string()),
        39
    );
    assert_eq!(
        str_len(&r#""jduij\x97zk\"rftjrixzgscxxllpqx\"bwwb""#.to_string()),
        32
    );
    assert_eq!(
        encoded_str_len(&r#""jduij\x97zk\"rftjrixzgscxxllpqx\"bwwb""#.to_string()),
        48
    );
}

#[test]
fn str_len_ex_292() {
    assert_eq!(raw_str_len(&r#""fqcditz""#.to_string()), 9);
    assert_eq!(str_len(&r#""fqcditz""#.to_string()), 7);
    assert_eq!(encoded_str_len(&r#""fqcditz""#.to_string()), 13);
}

#[test]
fn str_len_ex_293() {
    assert_eq!(
        raw_str_len(&r#""f\x19azclj\"rsvaokgvty\"aeq""#.to_string()),
        29
    );
    assert_eq!(str_len(&r#""f\x19azclj\"rsvaokgvty\"aeq""#.to_string()), 22);
    assert_eq!(
        encoded_str_len(&r#""f\x19azclj\"rsvaokgvty\"aeq""#.to_string()),
        38
    );
}

#[test]
fn str_len_ex_294() {
    assert_eq!(
        raw_str_len(&r#""erse\x9etmzhlmhy\x67yftoti""#.to_string()),
        28
    );
    assert_eq!(str_len(&r#""erse\x9etmzhlmhy\x67yftoti""#.to_string()), 20);
    assert_eq!(
        encoded_str_len(&r#""erse\x9etmzhlmhy\x67yftoti""#.to_string()),
        34
    );
}

#[test]
fn str_len_ex_295() {
    assert_eq!(raw_str_len(&r#""lsdw\xb3dmiy\\od""#.to_string()), 18);
    assert_eq!(str_len(&r#""lsdw\xb3dmiy\\od""#.to_string()), 12);
    assert_eq!(encoded_str_len(&r#""lsdw\xb3dmiy\\od""#.to_string()), 25);
}

#[test]
fn str_len_ex_296() {
    assert_eq!(raw_str_len(&r#""x\x6fxbljsjdgd\xaau""#.to_string()), 21);
    assert_eq!(str_len(&r#""x\x6fxbljsjdgd\xaau""#.to_string()), 13);
    assert_eq!(encoded_str_len(&r#""x\x6fxbljsjdgd\xaau""#.to_string()), 27);
}

#[test]
fn str_len_ex_297() {
    assert_eq!(
        raw_str_len(&r#""hjg\\w\"\x78uoqbsdikbjxpip\"w\"jnhzec""#.to_string()),
        39
    );
    assert_eq!(
        str_len(&r#""hjg\\w\"\x78uoqbsdikbjxpip\"w\"jnhzec""#.to_string()),
        30
    );
    assert_eq!(
        encoded_str_len(&r#""hjg\\w\"\x78uoqbsdikbjxpip\"w\"jnhzec""#.to_string()),
        52
    );
}

#[test]
fn str_len_ex_298() {
    assert_eq!(raw_str_len(&r#""gk""#.to_string()), 4);
    assert_eq!(str_len(&r#""gk""#.to_string()), 2);
    assert_eq!(encoded_str_len(&r#""gk""#.to_string()), 8);
}

#[test]
fn str_len_ex_299() {
    assert_eq!(raw_str_len(&r#""\\zrs\\syur""#.to_string()), 13);
    assert_eq!(str_len(&r#""\\zrs\\syur""#.to_string()), 9);
    assert_eq!(encoded_str_len(&r#""\\zrs\\syur""#.to_string()), 21);
}
