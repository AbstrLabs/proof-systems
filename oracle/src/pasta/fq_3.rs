use crate::poseidon::ArithmeticSpongeParams;
use mina_curves::pasta::Fq;

/* Generated by params.sage (arguments: name=3, rounds=55, width=3) */

use std::str::FromStr;

pub fn params() -> ArithmeticSpongeParams<Fq> {
    ArithmeticSpongeParams {
        mds: vec![
            vec![
                Fq::from_str(
                    "14212619405464931950324959059321425378927712125468729936419676295285526059287",
                )
                .unwrap(),
                Fq::from_str(
                    "953411757309523470508801209918493160026697872719996640925911583424528890148",
                )
                .unwrap(),
                Fq::from_str(
                    "6313453445432773088195374907442899358187378500424466808411010712726560510509",
                )
                .unwrap(),
            ],
            vec![
                Fq::from_str(
                    "11465332517882075520500781387292574119333162974773870222660636846007579035194",
                )
                .unwrap(),
                Fq::from_str(
                    "21850192249239148640986255127986873383270029512243019535124147025635008450342",
                )
                .unwrap(),
                Fq::from_str(
                    "11741912571094137206132343459207099891895332820389101665509583322734640388474",
                )
                .unwrap(),
            ],
            vec![
                Fq::from_str(
                    "18248178871277756986962078109321694982204432316644279006613703624727543015263",
                )
                .unwrap(),
                Fq::from_str(
                    "18631809286574460725958868876904331958158242244798242988647226926132968135973",
                )
                .unwrap(),
                Fq::from_str(
                    "21478205704100287674624301356876101848396342204065165498852579847359369739990",
                )
                .unwrap(),
            ],
        ],
        round_constants: vec![
            vec![
                Fq::from_str(
                    "929363847231671740736701723544075955102406645674983925079996084088176565325",
                )
                .unwrap(),
                Fq::from_str(
                    "13704806796153848590952000472207045057021666098628213343856684515567732821129",
                )
                .unwrap(),
                Fq::from_str(
                    "6291703371391882288377807553825265597844556928460654372535909150977210404692",
                )
                .unwrap(),
            ],
            vec![
                Fq::from_str(
                    "9463423228356472157010273600032607498818952551012406359386271861374144218572",
                )
                .unwrap(),
                Fq::from_str(
                    "20439642311667370328599847717391835060529258266969840991319958485234361317178",
                )
                .unwrap(),
                Fq::from_str(
                    "10926621557667333972762727170489779082295959111204840705759315900852925292382",
                )
                .unwrap(),
            ],
            vec![
                Fq::from_str(
                    "19616690819131348059506417484904115712118503471771485115718560118193740933804",
                )
                .unwrap(),
                Fq::from_str(
                    "14243025526129180206741778332311989552557217237659803577529256145922982785392",
                )
                .unwrap(),
                Fq::from_str(
                    "1024299346165564473351004546255405305019072247119919257702869849220544705877",
                )
                .unwrap(),
            ],
            vec![
                Fq::from_str(
                    "26830713994462238852292689968951557919899538144186444892250184633115121787921",
                )
                .unwrap(),
                Fq::from_str(
                    "21623457644394506096135831921771682461687307948052485223224509378397743878201",
                )
                .unwrap(),
                Fq::from_str(
                    "24576191320642894585479968742305171537676837854365803572235918073732882966789",
                )
                .unwrap(),
            ],
            vec![
                Fq::from_str(
                    "3539856943234597184163004234973319306550601319322777367299902108673698657705",
                )
                .unwrap(),
                Fq::from_str(
                    "13595971941168444735386401703724077261913748480115708185843857068828284859043",
                )
                .unwrap(),
                Fq::from_str(
                    "17003994860693470289482535978842637576122595578422843069848357577940974230078",
                )
                .unwrap(),
            ],
            vec![
                Fq::from_str(
                    "5850981784416613728037144287926060072607630457631116419558980760711700082351",
                )
                .unwrap(),
                Fq::from_str(
                    "19993350498872477343940557534106299313098798931226727619865971826173893148686",
                )
                .unwrap(),
                Fq::from_str(
                    "8946709113899526541263245022108418908387284498198809026917094161035060530667",
                )
                .unwrap(),
            ],
            vec![
                Fq::from_str(
                    "8442444772594186912791656015386419028945918027972498384503885382753211813452",
                )
                .unwrap(),
                Fq::from_str(
                    "6112256257408444446379679484155390428476516078351983309520702326013894637751",
                )
                .unwrap(),
                Fq::from_str(
                    "1342259615233514310384526406740955923501884329606960464801102610773986362465",
                )
                .unwrap(),
            ],
            vec![
                Fq::from_str(
                    "10376374175718291920668902615549667764500433802452161831244158236728909181797",
                )
                .unwrap(),
                Fq::from_str(
                    "15815151147892744676274509060581509487126718623777289215374219640416075733787",
                )
                .unwrap(),
                Fq::from_str(
                    "26110682571717227832823308688592633458040704205086884575494355145341454637359",
                )
                .unwrap(),
            ],
            vec![
                Fq::from_str(
                    "3103879239744375021553733170117970864717169259559696054530863137578193280769",
                )
                .unwrap(),
                Fq::from_str(
                    "21445614591383571568254054382899673413791766259136624439149696609541897847392",
                )
                .unwrap(),
                Fq::from_str(
                    "18733705848535115251331921374829091084490869232693896887393635421797726069994",
                )
                .unwrap(),
            ],
            vec![
                Fq::from_str(
                    "23174471134284788655169405525606166182146204756871014477826806484757344350986",
                )
                .unwrap(),
                Fq::from_str(
                    "22614502692907223297060113532901182162435626994532028851461731138691515001810",
                )
                .unwrap(),
                Fq::from_str(
                    "21954071577595853967239115480212632994635873354461903773404160989312769432302",
                )
                .unwrap(),
            ],
            vec![
                Fq::from_str(
                    "23424915376769792702616605805145632245607123769723433579191449495432845057850",
                )
                .unwrap(),
                Fq::from_str(
                    "2149759203800310762974124909329518592343761641255797003325579489076431119481",
                )
                .unwrap(),
                Fq::from_str(
                    "3664278234216067673933340198105353910396757541815744373609058673609700597407",
                )
                .unwrap(),
            ],
            vec![
                Fq::from_str(
                    "4616472457191291572420501095088666383378217723641824770669911243521142361396",
                )
                .unwrap(),
                Fq::from_str(
                    "19218287657506037022559665794016348734873323217577291144656168442295135505434",
                )
                .unwrap(),
                Fq::from_str(
                    "16656844535789274541191456637851043513034481593413626170253757737808040315328",
                )
                .unwrap(),
            ],
            vec![
                Fq::from_str(
                    "21252926193616660302872797701692620845709517002500474401218387448880467508085",
                )
                .unwrap(),
                Fq::from_str(
                    "12935287723113135191699173390872960024679081021077323538214929419992808205652",
                )
                .unwrap(),
                Fq::from_str(
                    "2780603379504942181593882582631274532513737164989538724642431149458911791371",
                )
                .unwrap(),
            ],
            vec![
                Fq::from_str(
                    "8052505685507867068009616595225318702768361579027777766927880675877151271999",
                )
                .unwrap(),
                Fq::from_str(
                    "17632607931991090805007430100447570437597366031829748928884127160791459303756",
                )
                .unwrap(),
                Fq::from_str(
                    "1194153812876468869960071221859585433511315473985076949620776648007930129801",
                )
                .unwrap(),
            ],
            vec![
                Fq::from_str(
                    "13448183286559938377241313710699646820173375112108683404361345713269468267496",
                )
                .unwrap(),
                Fq::from_str(
                    "15308106694760858530800665798200727785155205781893551933644365465566842236599",
                )
                .unwrap(),
                Fq::from_str(
                    "23611789814624476146042113317607513551270700657748390331982220709590134205326",
                )
                .unwrap(),
            ],
            vec![
                Fq::from_str(
                    "16517445624433838934538652663580386127343900191874067606649908037495307365969",
                )
                .unwrap(),
                Fq::from_str(
                    "13134171610508913684409298768858076345814850147514311341019949189060817750044",
                )
                .unwrap(),
                Fq::from_str(
                    "5182854352369834733875921462096401299931859553025019117167415754768111668613",
                )
                .unwrap(),
            ],
            vec![
                Fq::from_str(
                    "21247561414561460737981698259508444623902765687990685100620351822298412112089",
                )
                .unwrap(),
                Fq::from_str(
                    "17226609388893157560191416618652406124786814245233719367680678453320573957562",
                )
                .unwrap(),
                Fq::from_str(
                    "13625777423349440484959721011584627645153949459405728623910152069195891025923",
                )
                .unwrap(),
            ],
            vec![
                Fq::from_str(
                    "5723906914758977359764806729642236685655099635595868353757706049485091864238",
                )
                .unwrap(),
                Fq::from_str(
                    "25290936419598330015660581098582181708701972295437836668223057635410675869428",
                )
                .unwrap(),
                Fq::from_str(
                    "18127806376758626438766553012556276970103214686895497352585739824785059237206",
                )
                .unwrap(),
            ],
            vec![
                Fq::from_str(
                    "9442220240706706663253136622188632650753403508268257574929070724996445591152",
                )
                .unwrap(),
                Fq::from_str(
                    "10381979691432890600915794335445978368960175366325117749406839729956038121541",
                )
                .unwrap(),
                Fq::from_str(
                    "5712659774869806688984985514447412274690841894673694596090441530164268757441",
                )
                .unwrap(),
            ],
            vec![
                Fq::from_str(
                    "5805071792924939313454176064608092391602956628876107591998526929195586041702",
                )
                .unwrap(),
                Fq::from_str(
                    "9471286169551903086534951434986691382098570780222105627413303665751318070265",
                )
                .unwrap(),
                Fq::from_str(
                    "27353183133002568986178027428087525913138583443869555462581918776246130620400",
                )
                .unwrap(),
            ],
            vec![
                Fq::from_str(
                    "13760685734562426115073156820251126336756673613493257256543478766078101080953",
                )
                .unwrap(),
                Fq::from_str(
                    "21696698128929487617536924152287608232818390292653651262211067759630837719025",
                )
                .unwrap(),
                Fq::from_str(
                    "9282226091094467937509782923316418598072333021707192775341535300285708981137",
                )
                .unwrap(),
            ],
            vec![
                Fq::from_str(
                    "26078569707550210412407086256051136004809504185951123006181407189786548632862",
                )
                .unwrap(),
                Fq::from_str(
                    "20265066380793053082187575214968540042559519218599924648618019201572992729346",
                )
                .unwrap(),
                Fq::from_str(
                    "25098787034223100626476398122687248514017535233564372345060833107183571850388",
                )
                .unwrap(),
            ],
            vec![
                Fq::from_str(
                    "27386685215476693351635920296215886133366073270879149435313993741723095351276",
                )
                .unwrap(),
                Fq::from_str(
                    "15689696420217846229746120220242612971040530082441344593873732471468129646004",
                )
                .unwrap(),
                Fq::from_str(
                    "10761226001405622900273617767054920741836549450679050735850997875531907964527",
                )
                .unwrap(),
            ],
            vec![
                Fq::from_str(
                    "4259161286834250688169427633296974761860501491422282732753616933275339584241",
                )
                .unwrap(),
                Fq::from_str(
                    "3771754600391740040625180863462908007028979692293079703377975274815811092073",
                )
                .unwrap(),
                Fq::from_str(
                    "5328145768724703636995244004774053009871569719172015425060633638866369356372",
                )
                .unwrap(),
            ],
            vec![
                Fq::from_str(
                    "18294121241526082558895505175555407984263268443323100667607832935518026776154",
                )
                .unwrap(),
                Fq::from_str(
                    "25253072480545214839827113382661310964282698828181921384795035068285685747900",
                )
                .unwrap(),
                Fq::from_str(
                    "8306714538359254324833451857956091575587433311208671517293242594201037457045",
                )
                .unwrap(),
            ],
            vec![
                Fq::from_str(
                    "11884672756673522181941710482488254172352714316851169924910625842044323177648",
                )
                .unwrap(),
                Fq::from_str(
                    "4772561199895112486510156188215311004476007698177799453201929109262075040549",
                )
                .unwrap(),
                Fq::from_str(
                    "18190915332134215662070380438135847480837392646499706188163323045458834403672",
                )
                .unwrap(),
            ],
            vec![
                Fq::from_str(
                    "8322178416085190493051519391239446639453789905254703190899740335288881569625",
                )
                .unwrap(),
                Fq::from_str(
                    "23146906919418574668663297637662012025034904140713019455584575348880008247455",
                )
                .unwrap(),
                Fq::from_str(
                    "5489505613504687684724158357199886344115797136766787390437852683299303349293",
                )
                .unwrap(),
            ],
            vec![
                Fq::from_str(
                    "10414193415882357027844463149262799888927179560013467195342670510571433188971",
                )
                .unwrap(),
                Fq::from_str(
                    "26181309209671605912855181125743348747799846644378113923991234218718512855656",
                )
                .unwrap(),
                Fq::from_str(
                    "16202661606238215131180337531150039072699766012221645893402863657631978103555",
                )
                .unwrap(),
            ],
            vec![
                Fq::from_str(
                    "28706815269070578233350963867295933356458396447057866671465244308978642104352",
                )
                .unwrap(),
                Fq::from_str(
                    "27569780992420954783450507226926471188365779555066914883575169449667878687019",
                )
                .unwrap(),
                Fq::from_str(
                    "10053328832739162799795355895542536050121751440340836619387180841123154894028",
                )
                .unwrap(),
            ],
            vec![
                Fq::from_str(
                    "12231179863878300048907197486165228689403722823970628685002785017928975252392",
                )
                .unwrap(),
                Fq::from_str(
                    "26648821648039331351604901996877167971550448948558098796136409744514037651794",
                )
                .unwrap(),
                Fq::from_str(
                    "17927507842869166172758114380693835059348752882877194330940633115037863279181",
                )
                .unwrap(),
            ],
            vec![
                Fq::from_str(
                    "12469607453647557296747600314216152361846778873479887646842774609762738034811",
                )
                .unwrap(),
                Fq::from_str(
                    "23663168843078892926169399612222175008556818317154362840301392886655522142670",
                )
                .unwrap(),
                Fq::from_str(
                    "3272846654039139240807219901096512240894847142167150168712833396011260322683",
                )
                .unwrap(),
            ],
            vec![
                Fq::from_str(
                    "27864773311024613223416502198174261930281972690527458482559815226893575273910",
                )
                .unwrap(),
                Fq::from_str(
                    "6161059230877687308127312345251304039628767404381396906491402262700285250461",
                )
                .unwrap(),
                Fq::from_str(
                    "4301511218881318427538705305002567616171273271022293267676596670361617013486",
                )
                .unwrap(),
            ],
            vec![
                Fq::from_str(
                    "13237319687957860562112338252905232187094393573052123949328868998124675208481",
                )
                .unwrap(),
                Fq::from_str(
                    "15315464291608184963207152903977711152361399510760996406588747786930890986578",
                )
                .unwrap(),
                Fq::from_str(
                    "4993134594712700770030417566521589239252733984496287100119103648300326264276",
                )
                .unwrap(),
            ],
            vec![
                Fq::from_str(
                    "21418944801520772058961300439108491806832309467391853486874677675959070453563",
                )
                .unwrap(),
                Fq::from_str(
                    "4261783828602307896245439874016279627998131280415499031683687995301321043156",
                )
                .unwrap(),
                Fq::from_str(
                    "19028382530966264216780505180080215996068062047807412369052001516374552192389",
                )
                .unwrap(),
            ],
            vec![
                Fq::from_str(
                    "19570579515140211345838998867774148843297104318982916951474616358829124051361",
                )
                .unwrap(),
                Fq::from_str(
                    "25642748647129789234613446054783856286429464235190229647822169897671843473820",
                )
                .unwrap(),
                Fq::from_str(
                    "11962339111988941919936857195056618081485540477171146318505867597481697349356",
                )
                .unwrap(),
            ],
            vec![
                Fq::from_str(
                    "6131301643144162198755171831650082146439803315648815117670323669264624411978",
                )
                .unwrap(),
                Fq::from_str(
                    "7261848047869572794432242065952295357328375955768356961412295343381903585641",
                )
                .unwrap(),
                Fq::from_str(
                    "23652462333285041763388306655941922200265735301179770404960208269574996173449",
                )
                .unwrap(),
            ],
            vec![
                Fq::from_str(
                    "25983647882592563824841419382336047971928599664806986333486988124041275824229",
                )
                .unwrap(),
                Fq::from_str(
                    "2772404318729055600861447512907983839027442232077501263187573533897673577856",
                )
                .unwrap(),
                Fq::from_str(
                    "15922417437928534658481268293377661299957391772865819916806719934488736534916",
                )
                .unwrap(),
            ],
            vec![
                Fq::from_str(
                    "8240225035522959489626744030210749272528095723043828636315642888692553588536",
                )
                .unwrap(),
                Fq::from_str(
                    "24876786483443661642700546588412983709795139355233675947368115206293315041945",
                )
                .unwrap(),
                Fq::from_str(
                    "1725845953640074717576948947076083306004484511752183493003850911961348330721",
                )
                .unwrap(),
            ],
            vec![
                Fq::from_str(
                    "17573510371569506394592747304506008730968039063949679977222071775990268827822",
                )
                .unwrap(),
                Fq::from_str(
                    "6177836994395667116676929773107071292272534501018539907018682132992965261240",
                )
                .unwrap(),
                Fq::from_str(
                    "11954632015740074488648765461659244892865966650883175459899237998125534296876",
                )
                .unwrap(),
            ],
            vec![
                Fq::from_str(
                    "9592730059817158052697299095929309993263976124241216013817347010643002262489",
                )
                .unwrap(),
                Fq::from_str(
                    "21341574810624432361706538791511563783231423366017501322122842855933378861091",
                )
                .unwrap(),
                Fq::from_str(
                    "1328900350982110214708829489095696429417220071592166299522851016728455250964",
                )
                .unwrap(),
            ],
            vec![
                Fq::from_str(
                    "10244885559629916311872774214836245019159256894920831718449184377020757134534",
                )
                .unwrap(),
                Fq::from_str(
                    "5794675428192273311656680488696507981094159263992533223443300757245189879739",
                )
                .unwrap(),
                Fq::from_str(
                    "7289175911788050936348145536470893651901241555109060123860620121331940173445",
                )
                .unwrap(),
            ],
            vec![
                Fq::from_str(
                    "18116244709588593774763824935747828731426714234382756146348336755306451470121",
                )
                .unwrap(),
                Fq::from_str(
                    "24187182995494018176125915973293388695224922709684442125263593501084887559506",
                )
                .unwrap(),
                Fq::from_str(
                    "16352915866872745651742638049875459737241772202017670888822427766398500506431",
                )
                .unwrap(),
            ],
            vec![
                Fq::from_str(
                    "23118109010598240081441752343309616425546753051715015409684745244378186055790",
                )
                .unwrap(),
                Fq::from_str(
                    "12727545204774054214881400964563470540875404076096999744800273166956920935116",
                )
                .unwrap(),
                Fq::from_str(
                    "13284902511018362172294552288238901547817472168460461563785897499441079714201",
                )
                .unwrap(),
            ],
            vec![
                Fq::from_str(
                    "6450399346441268573957234898260734806044644385788296307214066611824293622398",
                )
                .unwrap(),
                Fq::from_str(
                    "25888930784432762862427064425708919865607715780478331086046531491683002302440",
                )
                .unwrap(),
                Fq::from_str(
                    "26890687999443547202450631007305064033665584727162042358333796090346370196457",
                )
                .unwrap(),
            ],
            vec![
                Fq::from_str(
                    "24923061170613270333124999616152465845282912598256565245615245184298750857960",
                )
                .unwrap(),
                Fq::from_str(
                    "5077330431951176338709386781422318157913372028793294064636115912546122076035",
                )
                .unwrap(),
                Fq::from_str(
                    "8621205718628710966463563954480139676117060534495388868319495632546529454120",
                )
                .unwrap(),
            ],
            vec![
                Fq::from_str(
                    "12064185271502604727201739390513771175545341535221090031857854019756088515216",
                )
                .unwrap(),
                Fq::from_str(
                    "8815500385625655834643179783503826420221815597470850553100832657309384496466",
                )
                .unwrap(),
                Fq::from_str(
                    "5491811040146508982321766696477294588259183521609413623463979490217866552678",
                )
                .unwrap(),
            ],
            vec![
                Fq::from_str(
                    "2247228422266238985920252957130297073349283023245974437819113087053147620468",
                )
                .unwrap(),
                Fq::from_str(
                    "28109090560869204868293038453987596860012656930230251447135258517533633674175",
                )
                .unwrap(),
                Fq::from_str(
                    "9251200736661147546767292518958053448191166684852159550258310877390676830801",
                )
                .unwrap(),
            ],
            vec![
                Fq::from_str(
                    "11390563482965240595872521781436723464640801054074887967489711366075490527478",
                )
                .unwrap(),
                Fq::from_str(
                    "440084237790735753066569582193274143998668490550052806099930626883032132391",
                )
                .unwrap(),
                Fq::from_str(
                    "28009166729201388175062294193752027500444685987794214445209460984687384205473",
                )
                .unwrap(),
            ],
            vec![
                Fq::from_str(
                    "26281292941389307987920569139560021121297915376007991932326260328324383325527",
                )
                .unwrap(),
                Fq::from_str(
                    "8147699842710318855854720227344588255344328926043596563171049742567162906422",
                )
                .unwrap(),
                Fq::from_str(
                    "12466194843678305453264632363328097267082446500719501233933257327058517379610",
                )
                .unwrap(),
            ],
            vec![
                Fq::from_str(
                    "3433125996381104175673408765896124365585067538541880995183405151083532274301",
                )
                .unwrap(),
                Fq::from_str(
                    "21072405799551683689662425952064209419792699042498892091797968372908825129238",
                )
                .unwrap(),
                Fq::from_str(
                    "12645809397344900224501993138621436296796117507356830855434839290883346968101",
                )
                .unwrap(),
            ],
            vec![
                Fq::from_str(
                    "16222802629689604666788475876773803723453083129829764557212775061441128900432",
                )
                .unwrap(),
                Fq::from_str(
                    "11958465892100742235447603617712277087488789284027270107820384066066942004065",
                )
                .unwrap(),
                Fq::from_str(
                    "14379153894401487873276384881785966620778292442775174057705282548293605036073",
                )
                .unwrap(),
            ],
            vec![
                Fq::from_str(
                    "23810427634631504324113280324001992515052347372178248816574152472472565987501",
                )
                .unwrap(),
                Fq::from_str(
                    "4867166398566695823004805386650778072576352241676975015866774413147528595746",
                )
                .unwrap(),
                Fq::from_str(
                    "26994730362951509435021580547997832503428733367142484915712733552356906460683",
                )
                .unwrap(),
            ],
            vec![
                Fq::from_str(
                    "12693326416167738364744809134851998850704454391901939057013275466715154426666",
                )
                .unwrap(),
                Fq::from_str(
                    "14387281493958170127417665779670798819647264401471428543377248953294586029780",
                )
                .unwrap(),
                Fq::from_str(
                    "9970124256187473713166485842022769844325248499023912181256653136263085889401",
                )
                .unwrap(),
            ],
            vec![
                Fq::from_str(
                    "19757879240520485366079227876090623526882072623905208015744228881887218581736",
                )
                .unwrap(),
                Fq::from_str(
                    "10035928898442041474127812128680980305343129345367335459329282174553765053845",
                )
                .unwrap(),
                Fq::from_str(
                    "22027161512406874925372179917328503293635564556395704794724355438667885971677",
                )
                .unwrap(),
            ],
            vec![
                Fq::from_str(
                    "2004037393806009100662697392068074613799686654771062691305639735463112288473",
                )
                .unwrap(),
                Fq::from_str(
                    "9435429181170964194952963481300592796958632335208233032570453110290784759077",
                )
                .unwrap(),
                Fq::from_str(
                    "13776188437108678337596564733403817038019457370417467845707079307383178898036",
                )
                .unwrap(),
            ],
        ],
    }
}
