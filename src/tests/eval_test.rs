#[cfg(test)]
mod eval_tests {
    use crate::ast::ast_tree::AstTree;
    use crate::eval::environment::{Env, EnvWrapper};
    use crate::eval::eval::{EvalRes, Evaluate};
    use crate::lexer::lexer::Lexer;
    use crate::lexer::line_reader_lexer::LineReaderLexer;
    use crate::parser::basic_parser_macros::stone_parser;
    use crate::token::token_identifier::TokenIdentifier;
    use crate::token::TokenValue;
    use std::any::TypeId;
    use TokenValue::IDENTIFIER;
    use crate::ast::leaf::identifier_literal::IdentifierLiteral;
    use crate::ast::list::binary_expr::BinaryExpr;
    use crate::ast::list::null_stmt::NullStmt;

    #[test]
    fn eval_test() {
        // let literal = IdentifierLiteral::new(TokenIdentifier::new(0,"+"));
        //
        // let x = literal.eval();
        // println!("a identifier literal eval while return : {:?}",x.do_eval());
        // let x1 = literal.eval();
        // println!("a identifier literal eval while return : {:?}",x1.do_eval());
        let value = IDENTIFIER("=".to_string());
        let eq = &value == "=";
        println!("{}", eq);
        let mut wrapper = EnvWrapper::new();
        let literal = IdentifierLiteral::new(TokenIdentifier::new(0, "i"));

        let res = literal.do_eval(&mut wrapper).unwrap();
        println!("{:?}", res);

        let expr = BinaryExpr::new(vec![]);
        expr.eval();
    }

    #[test]
    fn env_test() {
        let mut wrapper = EnvWrapper::new();
        let code = r#"
i = 2 ;
i = i+ " love u";
j = -6
j = j + 10 % 3
k = j * 3 +1-j/2*(j+1)
if j < -1 {
    l = 10
} else {
    l = 5
}
"#.to_string();
        _eval(code, &mut wrapper);
        println!(" i = {:?}", wrapper.get_ref("i").unwrap());
        println!(" j = {:?}", wrapper.get_ref("j").unwrap());
        println!(" k = {:?}", wrapper.get_ref("k").unwrap());
        // println!(" l = {:?}", wrapper.get_ref("l").unwrap());

    }


    #[test]
    fn binary_test() {
        let binary_vec = vec![
            "795*867+795*723==795*(867+723)",
            "246*133+246*118==246*(133+118)",
            "515*511+515*468==515*(511+468)",
            "784*927+784*229==784*(927+229)",
            "707*915+707*222==707*(915+222)",
            "734*922+734*123==734*(922+123)",
            "520*877+520*51==520*(877+51)",
            "305*979+305*385==305*(979+385)",
            "249*782+249*163==249*(782+163)",
            "683*614+683*316==683*(614+316)",
            "85*659+85*749==85*(659+749)",
            "647*481+647*287==647*(481+287)",
            "186*131+186*879==186*(131+879)",
            "513*10+513*844==513*(10+844)",
            "436*889+436*887==436*(889+887)",
            "866*986+866*552==866*(986+552)",
            "604*923+604*591==604*(923+591)",
            "230*630+230*419==230*(630+419)",
            "782*918+782*96==782*(918+96)",
            "867*768+867*654==867*(768+654)",
            "106*726+106*495==106*(726+495)",
            "777*260+777*232==777*(260+232)",
            "134*200+134*728==134*(200+728)",
            "509*264+509*16==509*(264+16)",
            "12*11+12*520==12*(11+520)",
            "995*431+995*336==995*(431+336)",
            "160*318+160*590==160*(318+590)",
            "621*89+621*481==621*(89+481)",
            "673*214+673*212==673*(214+212)",
            "310*730+310*175==310*(730+175)",
            "109*635+109*855==109*(635+855)",
            "421*657+421*418==421*(657+418)",
            "168*972+168*31==168*(972+31)",
            "443*114+443*940==443*(114+940)",
            "426*685+426*913==426*(685+913)",
            "388*470+388*426==388*(470+426)",
            "682*688+682*999==682*(688+999)",
            "611*158+611*912==611*(158+912)",
            "270*502+270*827==270*(502+827)",
            "312*712+312*185==312*(712+185)",
            "586*105+586*432==586*(105+432)",
            "534*881+534*158==534*(881+158)",
            "527*826+527*962==527*(826+962)",
            "685*638+685*586==685*(638+586)",
            "192*331+192*744==192*(331+744)",
            "441*163+441*62==441*(163+62)",
            "710*138+710*421==710*(138+421)",
            "376*392+376*990==376*(392+990)",
            "320*978+320*509==320*(978+509)",
            "243*263+243*803==243*(263+803)",
            "497*604+497*669==497*(604+669)",
            "239*700+239*623==239*(700+623)",
            "442*915+442*964==442*(915+964)",
            "575*411+575*41==575*(411+41)",
            "31*533+31*661==31*(533+661)",
            "737*461+737*609==737*(461+609)",
            "396*99+396*185==396*(99+185)",
            "291*538+291*963==291*(538+963)",
            "161*710+161*953==161*(710+953)",
            "707*437+707*811==707*(437+811)",
            "904*477+904*722==904*(477+722)",
            "272*225+272*675==272*(225+675)",
            "256*601+256*216==256*(601+216)",
            "375*759+375*271==375*(759+271)",
            "97*210+97*161==97*(210+161)",
            "526*590+526*191==526*(590+191)",
            "761*515+761*631==761*(515+631)",
            "141*126+141*576==141*(126+576)",
            "460*76+460*836==460*(76+836)",
            "791*399+791*775==791*(399+775)",
            "974*197+974*88==974*(197+88)",
            "996*542+996*985==996*(542+985)",
            "439*155+439*807==439*(155+807)",
            "147*651+147*720==147*(651+720)",
            "192*984+192*654==192*(984+654)",
            "90*277+90*923==90*(277+923)",
            "209*452+209*799==209*(452+799)",
            "502*750+502*620==502*(750+620)",
            "358*909+358*872==358*(909+872)",
            "931*884+931*826==931*(884+826)",
            "838*302+838*633==838*(302+633)",
            "81*40+81*33==81*(40+33)",
            "117*723+117*900==117*(723+900)",
            "365*336+365*123==365*(336+123)",
            "619*85+619*478==619*(85+478)",
            "121*709+121*616==121*(709+616)",
            "874*965+874*516==874*(965+516)",
            "206*622+206*940==206*(622+940)",
            "148*184+148*269==148*(184+269)",
            "182*282+182*332==182*(282+332)",
            "920*489+920*983==920*(489+983)",
            "780*407+780*869==780*(407+869)",
            "479*466+479*74==479*(466+74)",
            "58*521+58*557==58*(521+557)",
            "510*909+510*136==510*(909+136)",
            "321*75+321*110==321*(75+110)",
            "632*969+632*438==632*(969+438)",
            "169*795+169*440==169*(795+440)",
            "832*943+832*678==832*(943+678)",
            "626*318+626*987==626*(318+987)",
            "124*538+124*902==124*(538+902)",
            "525*959+525*657==525*(959+657)",
            "473*687+473*305==473*(687+305)",
            "766*632+766*987==766*(632+987)",
            "868*633+868*678==868*(633+678)",
            "116*117+116*367==116*(117+367)",
            "788*239+788*343==788*(239+343)",
            "537*415+537*159==537*(415+159)",
            "53*549+53*929==53*(549+929)",
            "712*16+712*932==712*(16+932)",
            "870*907+870*706==870*(907+706)",
            "746*464+746*77==746*(464+77)",
            "125*324+125*755==125*(324+755)",
            "54*122+54*433==54*(122+433)",
            "756*400+756*198==756*(400+198)",
            "514*757+514*64==514*(757+64)",
            "813*333+813*127==813*(333+127)",
            "570*941+570*439==570*(941+439)",
            "681*134+681*967==681*(134+967)",
            "738*188+738*321==738*(188+321)",
            "852*752+852*364==852*(752+364)",
            "704*851+704*248==704*(851+248)",
            "197*520+197*370==197*(520+370)",
            "112*655+112*286==112*(655+286)",
            "437*689+437*285==437*(689+285)",
            "926*786+926*710==926*(786+710)",
            "115*101+115*184==115*(101+184)",
            "337*784+337*425==337*(784+425)",
            "929*608+929*270==929*(608+270)",
            "801*739+801*418==801*(739+418)",
            "846*883+846*296==846*(883+296)",
            "917*546+917*253==917*(546+253)",
            "561*181+561*956==561*(181+956)",
            "84*637+84*477==84*(637+477)",
            "695*729+695*585==695*(729+585)",
            "813*330+813*279==813*(330+279)",
            "887*363+887*828==887*(363+828)",
            "554*851+554*383==554*(851+383)",
            "142*353+142*371==142*(353+371)",
            "531*471+531*382==531*(471+382)",
            "428*518+428*247==428*(518+247)",
            "190*985+190*471==190*(985+471)",
            "825*865+825*623==825*(865+623)",
            "975*922+975*365==975*(922+365)",
            "298*217+298*363==298*(217+363)",
            "248*249+248*376==248*(249+376)",
            "285*926+285*32==285*(926+32)",
            "784*82+784*362==784*(82+362)",
            "1000*506+1000*549==1000*(506+549)",
            "496*459+496*842==496*(459+842)",
            "222*366+222*488==222*(366+488)",
            "277*823+277*776==277*(823+776)",
            "991*275+991*471==991*(275+471)",
            "19*81+19*955==19*(81+955)",
            "884*272+884*702==884*(272+702)",
            "536*168+536*746==536*(168+746)",
            "355*684+355*952==355*(684+952)",
            "788*949+788*375==788*(949+375)",
            "442*444+442*904==442*(444+904)",
            "15*34+15*39==15*(34+39)",
            "603*143+603*292==603*(143+292)",
            "816*870+816*437==816*(870+437)",
            "402*144+402*919==402*(144+919)",
            "255*367+255*930==255*(367+930)",
            "893*523+893*462==893*(523+462)",
            "780*605+780*643==780*(605+643)",
            "260*149+260*487==260*(149+487)",
            "703*966+703*461==703*(966+461)",
            "509*390+509*115==509*(390+115)",
            "220*968+220*722==220*(968+722)",
            "627*321+627*594==627*(321+594)",
            "398*391+398*488==398*(391+488)",
            "96*476+96*368==96*(476+368)",
            "364*54+364*692==364*(54+692)",
            "114*714+114*341==114*(714+341)",
            "479*937+479*566==479*(937+566)",
            "745*84+745*638==745*(84+638)",
            "717*398+717*549==717*(398+549)",
            "966*122+966*554==966*(122+554)",
            "530*333+530*668==530*(333+668)",
            "254*563+254*840==254*(563+840)",
            "450*427+450*594==450*(427+594)",
            "377*96+377*416==377*(96+416)",
            "742*65+742*930==742*(65+930)",
            "39*314+39*909==39*(314+909)",
            "276*219+276*523==276*(219+523)",
            "181*85+181*690==181*(85+690)",
            "94*650+94*318==94*(650+318)",
            "968*804+968*724==968*(804+724)",
            "354*999+354*932==354*(999+932)",
            "630*110+630*910==630*(110+910)",
            "925*965+925*598==925*(965+598)",
            "597*580+597*688==597*(580+688)",
            "107*79+107*265==107*(79+265)",
            "634*888+634*45==634*(888+45)",
            "368*315+368*539==368*(315+539)",
            "696*504+696*639==696*(504+639)",
            "655*433+655*360==655*(433+360)",
            "664*469+664*89==664*(469+89)",
            "(898*753+898*389)%3+77777==898*(753+389)%3+77777",
        ];

        let mut wrapper = EnvWrapper::new();
        for binary_str in binary_vec {
            let res = _eval(binary_str.to_string(), &mut wrapper);
            assert_eq!(res == EvalRes::BOOLEAN(true), true, "结果应当为true");
        };
    }


    #[test]
    fn env_test_2() {
        let mut wrapper = EnvWrapper::new();
        let code = r#"
even = 0
odd = 0
i = 1
while i < 10 {
	if i % 2 == 0 {
		even = even + i
	}else {
		odd = odd + i
	}
	i = i + 1
}
even + odd
"#.to_string();
        let res = _eval(code, &mut wrapper);
        println!(" even = {:?}", wrapper.get_ref("even").unwrap());
        println!(" odd = {:?}", wrapper.get_ref("odd").unwrap());
        println!(" res = {:?}", res);
        // println!(" k = {:?}", wrapper.get_ref("k").unwrap());
        // println!(" l = {:?}", wrapper.get_ref("l").unwrap());

    }

    fn _eval(code: String, env: &mut EnvWrapper) -> EvalRes {
        let mut lexer = LineReaderLexer::new(code);
        let parser = stone_parser();
        let mut res = EvalRes::VOID;
        while let Some(_) = lexer.peek(0) {
            let tree_res = parser.parse(&mut lexer);
            let tree = match tree_res {
                Ok(tree) => {
                    tree
                }
                Err(msg) => {
                    panic!("{}", msg);
                }
            };
            let is_null_sata = tree.actual_type_id() == TypeId::of::<NullStmt>();
            if is_null_sata {
                continue;
            }
            let eval = tree.eval();
            let eval_res_res = eval.do_eval(env);
            res = match eval_res_res {
                Ok(eval_res) => {
                    println!("{:?}", eval_res);
                    eval_res
                }
                Err(err) => {
                    panic!("Eval error: {:?}", err);
                }
            }
        }
        res
    }
}