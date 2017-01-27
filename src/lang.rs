#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
pub enum Lang {
    Amh,
    Arb,
    Ben,
    Cmn,
    Deu,
    Eng,
    Fra,
    Heb,
    Hin,
    Jpn,
    Kat,
    Pol,
    Por,
    Rus,
    Spa,
    Tir,
    Ukr,
    Ydd,
}


impl Lang {
    pub fn from_code<S: Into<String>>(code: S) -> Option<Lang> {
        match code.into().to_lowercase().as_ref() {
                        "amh" => Some(Lang::Amh),
                        "arb" => Some(Lang::Arb),
                        "ben" => Some(Lang::Ben),
                        "cmn" => Some(Lang::Cmn),
                        "deu" => Some(Lang::Deu),
                        "eng" => Some(Lang::Eng),
                        "fra" => Some(Lang::Fra),
                        "heb" => Some(Lang::Heb),
                        "hin" => Some(Lang::Hin),
                        "jpn" => Some(Lang::Jpn),
                        "kat" => Some(Lang::Kat),
                        "pol" => Some(Lang::Pol),
                        "por" => Some(Lang::Por),
                        "rus" => Some(Lang::Rus),
                        "spa" => Some(Lang::Spa),
                        "tir" => Some(Lang::Tir),
                        "ukr" => Some(Lang::Ukr),
                        "ydd" => Some(Lang::Ydd),
                        _ => None
        }
    }

    pub fn to_code(&self) -> &str {
        match *self {
                        Lang::Amh => "amh",
                        Lang::Arb => "arb",
                        Lang::Ben => "ben",
                        Lang::Cmn => "cmn",
                        Lang::Deu => "deu",
                        Lang::Eng => "eng",
                        Lang::Fra => "fra",
                        Lang::Heb => "heb",
                        Lang::Hin => "hin",
                        Lang::Jpn => "jpn",
                        Lang::Kat => "kat",
                        Lang::Pol => "pol",
                        Lang::Por => "por",
                        Lang::Rus => "rus",
                        Lang::Spa => "spa",
                        Lang::Tir => "tir",
                        Lang::Ukr => "ukr",
                        Lang::Ydd => "ydd",
                    }
    }
}

pub type LangProfile = &'static [&'static str];
pub type LangProfileList = &'static [(Lang, LangProfile)];

pub const LATIN_LANGS : LangProfileList = &[
    (Lang::Spa, &[" de", "os ", "de ", " la", "la ", " y ", " a ", "es ", "ón ", "ión", "rec", "ere", "der", " co", "e l", "el ", "en ", "ien", "cho", "ent", "ech", "ció", "aci", "o a", "a p", " el", "a l", "al ", "as ", "e d", " en", "na ", "ona", "s d", "da ", "nte", " to", "ad ", "ene", "con", " pr", " su", "tod", " se", "ho ", "los", " pe", "per", "ers", " lo", "o d", " ti", "cia", "n d", "cio", " es", "ida", "res", "a t", "tie", "ion", "rso", "te ", "do ", " in", "son", " re", " li", "to ", "dad", "tad", "e s", "est", "pro", "que", "men", " po", "a e", "oda", "nci", " qu", " un", "ue ", "ne ", "n e", "s y", "lib", "su ", " na", "s e", "nac", "ia ", "e e", "tra", " pa", "or ", "ado", "a d", "nes", "ra ", "se ", "ual", "a c", "er ", "por", "com", "nal", "rta", "a s", "ber", " o ", "one", "s p", "dos", "rá ", "sta", "les", "des", "ibe", "ser", "era", "ar ", "ert", "ter", " di", "ale", "l d", "nto", "hos", "del", "ica", "a a", "s n", "n c", "oci", "imi", "io ", "o e", "re ", "y l", "e c", "ant", "cci", " as", "las", "par", "ame", " cu", "ici", "ara", "enc", "s t", "ndi", " so", "o s", "mie", "tos", "una", "bre", "dic", "cla", "s l", "e a", "l p", "pre", "ntr", "o t", "ial", "y a", "nid", "n p", "a y", "man", "omo", "so ", "n l", " al", "ali", "s a", "no ", " ig", "s s", "e p", "nta", "uma", "ten", "gua", "ade", "y e", "soc", "mo ", " fu", "igu", "o p", "n t", "hum", "d d", "ran", "ria", "y d", "ada", "tiv", "l e", "cas", " ca", "vid", "l t", "s c", "ido", "das", "dis", "s i", " hu", "s o", "nad", "fun", " ma", "rac", "nda", "eli", "sar", "und", " ac", "uni", "mbr", "a u", "die", "e i", "qui", "a i", " ha", "lar", " tr", "odo", "ca ", "tic", "o y", "cti", "lid", "ori", "ndo", "ari", " me", "ta ", "ind", "esa", "cua", "un ", "ier", "tal", "esp", "seg", "ele", "ons", "ito", "ont", "iva", "s h", "d y", "nos", "ist", "rse", " le", "cie", "ide", "edi", "ecc", "ios", "l m", "r e", "med", "tor", "sti", "n a", "rim", "uie", "ple", "tri", "ibr", "sus", "lo ", "ect", "pen", "y c", "an ", "e h", "n s", "ern", "tar", "l y", "egu", "gur", "ura", "int", "ond", "mat", "l r", "r a", "isf", "ote"]),
    (Lang::Eng, &[" th", "the", " an", "he ", "nd ", "and", "ion", " of", "of ", "tio", " to", "to ", "on ", " in", "al ", "ati", "igh", "ght", "rig", " ri", "or ", "ent", "as ", "ed ", "is ", "ll ", "in ", " be", "e r", "ne ", "one", "ver", "all", "s t", "eve", "t t", " fr", "s a", " ha", " re", "ty ", "ery", " or", "d t", " pr", "ht ", " co", " ev", "e h", "e a", "ng ", "ts ", "his", "ing", "be ", "yon", " sh", "ce ", "ree", "fre", "ryo", "n t", "her", "men", "nat", "sha", "pro", "nal", "y a", "has", "es ", "for", " hi", "hal", "f t", "n a", "n o", "nt ", " pe", "s o", " fo", "d i", "nce", "er ", "ons", "res", "e s", "ect", "ity", "ly ", "l b", "ry ", "e e", "ers", "e i", "an ", "e o", " de", "cti", "dom", "edo", "eed", "hts", "ter", "ona", "re ", " no", " wh", " a ", " un", "d f", " as", "ny ", "l a", "e p", "ere", " en", " na", " wi", "nit", "nte", "d a", "any", "ted", " di", "ns ", "sta", "th ", "per", "ith", "e t", "st ", "e c", "y t", "om ", "soc", " ar", "ch ", "t o", "d o", "nti", "s e", "equ", "ve ", "oci", "man", " fu", "ote", "oth", "ess", " al", " ac", "wit", "ial", " ma", "uni", " se", "rea", " so", " on", "lit", "int", "r t", "y o", "enc", "thi", "ual", "t a", " eq", "tat", "qua", "ive", " st", "ali", "e w", "l o", "are", "f h", "con", "te ", "led", " is", "und", "cia", "e f", "le ", " la", "y i", "uma", "by ", " by", "hum", "f a", "ic ", " hu", "ave", "ge ", "r a", " wo", "o a", "ms ", "com", " me", "eas", "s d", "tec", " li", "n e", "en ", "rat", "tit", "ple", "whe", "ate", "o t", "s r", "t f", "rot", " ch", "cie", "dis", "age", "ary", "o o", "anc", "eli", "no ", " fa", " su", "son", "inc", "at ", "nda", "hou", "wor", "t i", "nde", "rom", "oms", " ot", "g t", "eme", "tle", "iti", "gni", "s w", "itl", "duc", "d w", "whi", "act", "hic", "aw ", "law", " he", "ich", "min", "imi", "ort", "o s", "se ", "e b", "ntr", "tra", "edu", "oun", "tan", "e d", "nst", "l p", "d n", "ld ", "nta", "s i", "ble", "n p", " pu", "n s", " at", "ily", "rth", "tho", "ful", "ssi", "der", "o e", "cat", "uca", "unt", "ien", " ed", "o p", "h a", "era", "ind", "pen", "sec", "n w", "omm", "r s"]),
    (Lang::Por, &["os ", "de ", " de", " a ", " e ", "o d", "to ", "ão ", " di", "ent", "da ", "ito", "em ", " co", "eit", "as ", "dir", "es ", "ire", "rei", " se", "ção", "ade", "a p", "dad", "e d", "s d", "men", "nte", "do ", "s e", " pr", " pe", "dos", " to", " da", "a a", "o e", " o ", "o a", "ess", "con", "tod", "que", " qu", "te ", "e a", " do", "al ", "res", "ida", "m d", " in", " ou", "er ", "sso", " na", " re", " po", "a s", " li", "uma", "cia", "ar ", "pro", "e e", "a d", " te", "açã", "a t", " es", " su", "ou ", "ue ", "s p", "tos", "a e", "des", "ra ", "com", "no ", "ame", "ia ", "e p", "tem", "nto", " pa", "is ", "est", "tra", "ões", "na ", "s o", "oda", "das", "ser", "soa", "s n", "pes", "o p", "s a", "o s", "e o", " em", " as", " à ", "o o", "ais", "ber", "ado", "oa ", "o t", "e s", "man", "sua", "ua ", " no", " os", "a c", "ter", "çõe", "erd", "lib", "rda", "s s", "nci", "ibe", "e n", "ica", "odo", "so ", "nal", "ntr", "s t", "hum", "ura", " ao", "ona", "ual", " so", "or ", "ma ", "sta", "o c", "a n", "pre", "ara", "era", "ons", "e t", "r a", "par", "o à", " hu", "ind", "por", "cio", "ria", "m a", "s c", " um", "a l", "gua", "ran", " en", "ndi", "o i", "e c", "raç", "ion", "nid", "aci", "ano", "soc", "e r", "oci", " ac", "und", "sen", "nos", "nsi", "rec", "ime", "ali", "int", "um ", "per", "nac", " al", "m o", "r p", " fu", "ndo", "ont", "açõ", " ig", "igu", "fun", "nta", " ma", "uni", "cçã", "ere", " ex", "a i", " me", "ese", "rio", "l d", "a o", "s h", "pel", "ada", "pri", "ide", "am ", "m p", "pod", "s f", "ém ", "a f", "io ", "ode", "ca ", "ita", "lid", "tiv", "e f", "vid", "r e", "esp", "nda", "omo", "e l", "naç", "o r", "ant", "a q", "tad", "lic", "iva", " fa", "ver", "s l", "ial", "cla", "ngu", "ing", " ca", "mo ", "der", " vi", "eli", "ist", "ta ", "se ", "ati", "ios", "ido", "r o", "eci", "dis", " un", "e i", "r d", "ecç", "o q", "s i", "qua", "ênc", "a m", "seu", "sti", "nin", "uer", "rar", "cas", "aos", "ens", "gué", "ias", "sid", "uém", "tur", "dam", "sse", "ao ", "ela", "l e", "for", "tec", "ote", " pl", "ena", " tr", "m c", "tro", " ni", "ico", "rot"]),
    (Lang::Fra, &[" de", "es ", "de ", "ion", "nt ", "et ", "tio", " et", "ent", " la", "la ", "e d", "on ", "ne ", "oit", "e l", "le ", " le", "s d", "e p", "t d", "ati", "roi", " dr", "dro", "it ", " à ", " co", "té ", "ns ", "te ", "e s", "men", "re ", " to", "con", " l’", "tou", "que", " qu", "les", " so", "des", "son", " pe", "ons", " un", "s l", "s e", " pr", "ue ", " pa", "e c", "t l", "ts ", "onn", " au", "e a", "eme", "e e", " li", "ont", "ant", "out", "ute", "t à", "res", "ers", " sa", "ce ", " a ", "tre", "per", "a d", "cti", "er ", "lib", "ité", " en", "ux ", " re", "en ", "rso", "à l", " ou", " in", "lle", "un ", "nat", "ou ", "nne", "n d", "une", " d’", " se", "par", "nte", "us ", "ur ", "s s", "ans", "dan", "a p", "r l", "pro", "its", "és ", "t p", "ire", "e t", "s p", "sa ", " dé", "ond", "é d", "a l", "nce", "ert", "aux", "omm", "nal", "me ", " na", " fo", "iqu", " ce", "rté", "ect", "ale", "ber", "t a", "s a", " da", "mme", "ibe", "san", "e r", " po", "com", "al ", "s c", "qui", "our", "t e", " ne", "e n", "ous", "r d", "ali", "ter", " di", "fon", "e o", "au ", " ch", "air", "ui ", "ell", " es", "lit", "s n", "iss", "éra", "tes", "soc", "aut", "oci", "êtr", "ien", "int", "du ", "est", "été", "tra", "pou", " pl", "rat", "ar ", "ran", "rai", "s o", "ona", "ain", "cla", "éga", "anc", "rs ", "eur", "pri", "n c", "e m", "s t", "à u", " do", "ure", "bre", "ut ", " êt", "age", " ét", "nsi", "sur", "ein", "sen", "ser", "ndi", "ens", "ess", "ntr", "ir ", " ma", "cia", "n p", "st ", "a c", " du", "l e", " su", "bli", "ge ", "rés", " ré", "e q", "ass", "nda", "peu", "ée ", "l’a", " te", "a s", "tat", "il ", "tés", "ais", "u d", "ine", "ind", "é e", "qu’", " ac", "s i", "n t", "t c", "n a", "l’h", "t q", "soi", "t s", "cun", "rit", " ég", "oir", "’en", "nta", "hom", " on", "n e", " mo", "ie ", "ign", "rel", "nna", "t i", "l n", " tr", "ill", "ple", "s é", "l’e", "rec", "a r", "ote", "sse", "uni", "idé", "ive", "s u", "t ê", "ins", "act", " fa", "n s", " vi", "gal", " as", "lig", "ssa", "pré", "leu", "e f", "lic", "dis", "ver", " nu", "ten", "ssi", "rot", "tec", "s m", "abl"]),
    (Lang::Deu, &["en ", "er ", "der", " un", "nd ", "und", "ein", "ung", "cht", " de", "ich", "sch", "ng ", " ge", "ie ", "che", "ech", " di", "die", "rec", "gen", "ine", "eit", " re", "ch ", " da", "n d", "ver", "hen", " zu", "t d", " au", "ht ", " ha", "lic", "it ", "ten", "rei", " be", "in ", " ve", " in", " ei", "nde", "auf", "den", "ede", "zu ", "n s", "uf ", "fre", "ne ", "ter", "es ", " je", "jed", "n u", " an", "sei", "and", " fr", "run", "at ", " se", "e u", "das", "hei", "s r", "hte", "hat", "nsc", "nge", "r h", "as ", "ens", " al", "ere", "lle", "t a", " we", "n g", "rde", "nte", "ese", "men", " od", "ode", "ner", "g d", "all", "t u", "ers", "te ", "nen", " so", "d d", "n a", "ben", "lei", " gr", " vo", "wer", "e a", "ege", "ion", " st", "ige", "le ", "cha", " me", "haf", "aft", "n j", "ren", " er", "erk", "ent", "bei", " si", "eih", "ihe", "kei", "erd", "tig", "n i", "on ", "lun", "r d", "len", "gem", "ies", "gru", "tli", "unt", "chu", "ern", "ges", "end", "e s", "ft ", "st ", "ist", "tio", "ati", " gl", "sta", "gun", "mit", "sen", "n n", " na", "n z", "ite", " wi", "r g", "eic", "e e", "ei ", "lie", "r s", "n w", "gle", "mei", "de ", "uch", "em ", "chl", "nat", "rch", "t w", "des", "n e", "hre", "ale", "spr", "d f", "ach", "sse", "r e", " sc", "urc", "r m", "nie", "e f", "fen", "e g", "e d", " ni", "dur", "dar", "int", " du", "geh", "ied", "t s", " mi", "alt", "her", "hab", "f g", "sic", "ste", "taa", "aat", "he ", "ang", "ruc", "hli", "tz ", "eme", "abe", "h a", "n v", "nun", "geg", "arf", "rf ", "ehe", "pru", " is", "erf", "e m", "ans", "ndl", "e b", "tun", "n o", "d g", "n r", "r v", "wie", "ber", "r a", "arb", "bes", "t i", "h d", "r w", "r b", " ih", "d s", "igk", "gke", "nsp", "dig", "ema", "ell", "eru", "n f", "ins", "rbe", "ffe", "esc", "igu", "ger", "str", "ken", "e v", "gew", "han", "ind", "rt ", " ar", "ieß", "n h", "rn ", "man", "r i", "hut", "utz", "d a", "ls ", "ebe", "von", "lte", "r o", "rli", "etz", "tra", "aus", "det", "hul", "e i", "one", "nne", "isc", "son", "sel", "et ", "ohn", "t g", "sam", " fa", "rst", "rkl", "ser", "iem", "g v", "t z", "err"]),
    (Lang::Pol, &[" pr", "nie", " i ", "ie ", "pra", " po", "ani", "raw", "ia ", "nia", "wie", "go ", " do", "ch ", "ego", "iek", "owi", " ni", "ści", "ci ", "a p", "do ", "awo", " cz", "ośc", "ych", " ma", "ek ", "rze", " na", "prz", " w ", "wo ", "ej ", " za", "noś", "czł", "zło", "eni", "wa ", " je", "łow", "i p", "wol", "oln", " lu", "rod", " ka", " wo", "lno", "wsz", "y c", "ma ", "ny ", "każ", "ażd", "o d", "stw", "owa", "dy ", "żdy", " wy", "rzy", "sta", "ecz", " sw", "dzi", "i w", "e p", "czn", "twa", "na ", "zys", "ów ", "szy", "ub ", "lub", "a w", "est", "kie", "k m", "wan", " sp", "ają", " ws", "e w", "pow", "pos", "nyc", "rac", "spo", "ać ", "a i", "cze", "sze", "neg", "yst", "jak", " ja", "o p", "pod", "acj", "ne ", "ńst", "aro", "mi ", " z ", "i i", "nar", " ko", "obo", "awa", " ro", "i n", "jąc", "zec", "zne", "zan", "dow", " ró", "iej", "zy ", "zen", "nic", "ony", "aw ", "i z", "czy", "no ", "nej", "o s", "rów", "odn", "cy ", "ówn", "odz", "o w", "o z", "jeg", "edn", "o o", "aki", "mie", "ien", "kol", " in", "zie", "bez", "ami", "eńs", "owo", "dno", " ob", " or", " st", "a s", "ni ", "orz", "o u", "ym ", "stę", "tęp", "łec", "jed", "i k", " os", "w c", "lwi", "ez ", "olw", "ołe", "poł", "cji", "y w", "o n", "wia", " be", "któ", "a j", "zna", "zyn", "owe", "wob", "ka ", "wyc", "owy", "ji ", " od", "aln", "inn", "jes", "icz", "h p", "i s", "się", "a o", "ją ", "ost", "kra", "st ", "sza", "swo", "war", "cza", "roz", "y s", "raz", "nik", "ara", "ora", "lud", "i o", "a z", "zes", " kr", "ran", "ows", "ech", "w p", "dów", "ą p", "pop", "a n", "tki", "stk", "gan", "zon", "raj", "e o", "iec", "i l", " si", "że ", "eka", " kt", " de", "em ", "tór", "ię ", "wni", "lni", "ejs", "ini", "odo", "dni", "ełn", "kow", "peł", "a d", "ron", "dek", "pie", "udz", "bod", "nan", "h i", "dst", "ieg", "taw", "z p", "z w", "zeń", "god", "iu ", "ano", "lar", " to", "y z", "a k", "ale", "kla", "trz", "zaw", "ich", "e i", "ier", "iko", "dzy", "chn", "w z", "by ", "ków", "adz", "ekl", "ywa", "ju ", "och", "kor", "sob", "ocz", "oso", "u p", "du ", "tyc", "tan", "ędz", " mi", "e s", " ta", "ki "]),
];
pub const CYRILLIC_LANGS : LangProfileList = &[
    (Lang::Rus, &[" пр", " и ", "рав", "ств", " на", "пра", "го ", "ени", "ове", "во ", " ка", "ани", "ть ", " в ", " по", " об", "ия ", "сво", " св", "лов", "на ", " че", "ело", "о н", " со", "ост", "чел", "ие ", "ого", "ет ", "ния", "ест", "аво", "ый ", "ажд", " им", "ние", "век", " не", "льн", "ли ", "ова", "име", "ать", "при", "т п", "и п", "каж", "или", "обо", " ра", "ых ", "жды", " до", "дый", "воб", "ек ", "бод", "ва ", "й ч", "его", "ся ", "и с", "ии ", "аци", "еет", "но ", "мее", "и и", "лен", "ой ", "тва", "ных", "то ", " ил", "к и", "енн", " бы", "ию ", " за", "ми ", "тво", "и н", "о п", "ван", "о с", "сто", "аль", " вс", "ом ", "о в", "ьно", "их ", "ног", "и в", "нов", "ако", "про", "ий ", "сти", "и о", "пол", "олж", "дол", "ое ", "бра", "я в", " ос", "ным", "жен", "раз", "ти ", "нос", "я и", " во", "тор", "все", " ег", "ей ", "тел", "не ", "и р", "ред", "ель", "тве", "оди", " ко", "общ", "о и", " де", "има", "а и", "чес", "ним", "сно", "как", " ли", "щес", "вле", "ься", "нны", "аст", "тьс", "нно", "осу", "е д", " от", "пре", "шен", "а с", "бще", "осн", "одн", "быт", "сов", "ыть", "лжн", "ран", "нию", "иче", "ак ", "ым ", "ват", "что", "сту", "чен", "е в", " ст", "рес", "оль", " ни", "ном", "род", "ля ", "нар", "вен", "ду ", "оже", "ны ", "е и", " то", "вер", "а о", "зов", "м и", "нац", "ден", "рин", "туп", "ежд", "стр", " чт", "я п", "она", "дос", "х и", "й и", "тоя", "есп", "лич", "бес", "обр", "ото", "о б", "ьны", "ь в", "нии", "е м", "ую ", " мо", "ем ", " ме", "аро", " ре", "ава", "кот", "ав ", " вы", "ам ", "жно", "ста", "ая ", "под", "и к", "ное", " к ", " та", " го", "гос", "суд", "еоб", "я н", "ен ", "и д", "мож", "еск", "ели", "авн", "ве ", "ече", "уще", "печ", "дно", "о д", "ход", "ка ", " дл", "для", "ово", "ате", "льс", "ю и", "в к", "нен", "ции", "ной", "уда", "вов", " бе", "оро", "нст", "ами", "циа", "кон", "сем", "е о", "вно", " эт", "азо", "х п", "ни ", "жде", "м п", "ког", "от ", "дст", "вны", "сть", "ые ", "о о", "пос", "сре", "тра", "ейс", "так", "и б", "дов", "му ", "я к", "нал", "дру", " др", "кой", "тер", "ь п", "арс", "изн", "соц", "еди", "олн"]),
    (Lang::Ukr, &["на ", " пр", " і ", "пра", "рав", " на", "ня ", "ння", " за", "ого", " по", "ти ", "го ", "люд", " лю", "во ", " ко", " ма", "льн", "юди", "их ", "о н", " не", "аво", "анн", "дин", " св", "сво", "ожн", "кож", "енн", "пов", "жна", " до", "ати", "ина", "ає ", "а л", " бу", "аці", "не ", "ува", "обо", " ос", " як", "має", " ви", "них", "аль", "або", "є п", " та", "ні ", "ть ", "ови", "бо ", " ві", " аб", "ере", "і п", "а м", "вин", "без", "при", "іль", "ног", "о п", "ми ", "та ", "ом ", "ою ", "бод", "ста", "воб", " бе", "до ", "ва ", "ті ", " об", "о в", "ост", " в ", " що", "ий ", "ся ", "і с", " сп", "инн", "від", "ств", "и п", "ван", "нов", "нан", "кон", " у ", "ват", "она", "ії ", "но ", "дно", "ій ", "езп", "пер", " де", "ути", "ьно", "ист", "під", "сті", "бут", " мо", "и і", "ідн", "ако", "нні", "ід ", "тис", "що ", "род", "і в", "а з", "ава", " пе", "му ", "і н", "а п", "соб", "ої ", "а в", "спр", "ів ", "ний", "яко", "ду ", "вно", "і д", "ну ", "аро", "и с", " ін", "ля ", "рів", "у в", " рі", "и д", "нар", "нен", "ова", "ому", "лен", "нац", "ним", "ися", "чи ", "ав ", "і р", "ном", " ро", "нос", "ві ", "вни", "овн", " її", "ові", "мож", "віл", "у п", " пі", " су", "її ", "одн", " вс", "ово", "ють", "іст", "сть", "і з", " ст", "буд", " ра", "чен", "про", "роз", "івн", "оду", "а о", "ьни", "ни ", "о с", "сно", "зна", "рац", "им ", "о д", "ими", "я і", "ції", "х п", "дер", "чин", " со", "а с", "ерж", "и з", "и в", "е п", "ди ", "заб", "осо", "у с", "е б", "сі ", "тер", "ніх", "я н", "і б", "кла", "спі", "в і", " ні", "о з", "ржа", "сту", "їх ", "а н", "нна", "так", "я п", "зпе", " од", "абе", "для", "ту ", "і м", "печ", " дл", "же ", "ки ", "віт", "ніс", "гал", "ага", "е м", "ами", "зах", "рим", "ї о", "тан", "ког", "рес", "удь", " ре", "то ", "ков", "тор", "ара", "сві", "тва", "а б", "оже", "соц", "оці", "ціа", "осн", "роб", "дь‐", "ь‐я", "‐як", "і і", "заг", "ахи", "хис", "піл", "цій", "х в", "лив", "осв", "іал", "руч", "ь п", "інш", "в я", "ги ", "аги", " ді", "ком", "ини", "а і", "оди", "нал", "тво", "кої", "всі", "я в", "ною", "об ", "о у", "о о", "і о"]),
];
pub const ARABIC_LANGS : LangProfileList = &[
    (Lang::Arb, &[" ال", "ية ", " في", "الح", "في ", " وا", "وال", " أو", "ة ا", "أو ", "الم", "الت", "لحق", "حق ", "لى ", "كل ", "ان ", "ة و", "الأ", " لك", "لكل", "ن ا", "ها ", "ق ف", "ات ", "مة ", "ون ", "أن ", "ما ", "اء ", "ته ", "و ا", "الع", "ي ا", "شخص", "ي أ", " أن", "الإ", "م ا", "حري", " عل", "ة ل", "من ", "الا", "حقو", "على", "قوق", "ت ا", "أي ", "رد ", " شخ", " لل", " أي", "ق ا", "لا ", "فرد", "رية", " ول", " من", "د ا", " كا", " إل", "خص ", "وق ", "ا ا", "ة أ", "ا ي", "ل ف", "ه ا", "نسا", "جتم", "ن ي", "امة", "كان", "دة ", " حق", "ام ", "الق", "ة م", " فر", "اية", "سان", "ل ش", "ين ", "ن ت", "إنس", "ا ل", " لا", "ذا ", "هذا", "ن أ", "لة ", "ي ح", " دو", "ه ل", "لك ", "ترا", "لتع", "اً ", "له ", "إلى", " عن", "ى ا", "ه و", "ع ا", "ماع", "د أ", "اسي", " حر", "ة ع", "مع ", "الد", "نون", " با", "لحر", "لعا", "ن و", "، و", "يات", "ي ت", "الج", " هذ", "ير ", "بال", "دول", "لإن", "عية", "الف", "ص ا", " وي", "الو", "لأس", " إن", "أسا", "ساس", "ماي", "حما", "رام", "سية", "انو", "مل ", "ي و", "عام", "ا و", "تما", " مت", "ة ت", "علي", "ع ب", "ك ا", " له", "ة ف", "قان", "ى أ", "ول ", "هم ", "الب", "ة ب", "ساو", "لقا", "الر", "لجم", "ا ك", "تمت", "ليه", "لتم", "لمت", "انت", " قد", "اد ", "ه أ", " يج", "ريا", "ق و", "ل ا", "ا ب", "ال ", "يه ", "اعي", "لدو", "ل و", "لإع", "لمي", "لمج", "لأم", "تع ", "دم ", "تسا", "عمل", "اته", "لاد", "رة ", "اة ", "غير", "قدم", "وز ", "جوز", "يجو", "عال", "لان", "متع", "مان", "فيه", "اجت", "م و", "يد ", "تعل", "ن ل", "ر ا", " يع", " كل", "مم ", "مجت", "تمع", "دون", " مع", "تمي", "ذلك", "كرا", "يها", " مس", "ميع", "إعل", "علا", " تم", " عا", "ملا", "اعا", "لاج", "ني ", "ليم", "متس", "ييز", "يم ", "اعت", "الش", " تع", "ميي", "عن ", "تنا", " بح", "لما", "ي ي", "يز ", "ود ", "أمم", "لات", "أسر", "شتر", "تي ", " جم", "ه ع", "ر و", "ي إ", "تحد", "حدة", " أس", "عة ", "ي م", "ة، ", "معي", "ن م", "لمس", "م ب", "اق ", "جمي", "لي ", "مية", "الض", "الس", "لضم", "ضما", "لفر", " وس", "لحم", "امل", "ق م", "را ", "ا ح", "نت ", " تن", "يته", " أم", "إلي", "واج", "د و", "لتي", " مر", "مرا", "متح", " ذل", " وأ", " تح", "ا ف", " به", " وم", " بم", "وية", "ولي", "لزو"]),
];
pub const DEVANAGARI_LANGS : LangProfileList = &[
    (Lang::Hin, &["के ", "प्र", "और ", " और", " के", "ों ", " का", "कार", " प्", "का ", " को", "या ", "ं क", "ति ", "ार ", "को ", " है", "िका", "ने ", "है ", "्रत", "धिक", " अध", "अधि", "की ", "ा क", " कि", " की", " सम", "ें ", "व्य", "्ति", "क्त", "से ", " व्", "ा अ", "्यक", "में", "मान", "ि क", " स्", " मे", "सी ", "न्त", " हो", "े क", "ता ", "यक्", "क्ष", "ै ।", "िक ", "त्य", " कर", "्य ", " या", "भी ", " वि", "रत्", "र स", "ी स", " जा", "स्व", "रों", "्ये", "ेक ", "येक", "त्र", "िया", "ा ज", "क व", "र ह", "ित ", "्रा", "किस", " अन", "ा स", "िसी", "ा ह", "ना ", " से", " पर", "र क", " सा", "देश", "गा ", " । ", " अप", "्त्", "े स", "समा", "ान ", "ी क", "्त ", "वार", " ।प", "ा प", " रा", "षा ", "न क", "।प्", "ष्ट", "था ", "अन्", " मा", "्षा", "्वा", "ारो", "तन्", "वतन", "ट्र", "्वत", "प्त", "ाप्", "्ट्", "राष", "ाष्", " इस", "े अ", " उस", " सं", "राप", "कि ", "त ह", "हो ", "ं औ", "ार्", "ा ।", "किय", "े प", " दे", " भी", "करन", "री ", "जाए", "ी प", " न ", "र अ", "क स", "अपन", "े व", "ाओं", "्तर", "ओं ", " नि", "सभी", "रा ", " तथ", "तथा", "िवा", "यों", "पर ", " ऐस", "रता", "ारा", "्री", "सम्", " द्", "ीय ", "िए ", "व क", "सके", "द्व", "होग", " सभ", "ं म", "माज", "रने", "िक्", "्या", "ा व", "र प", " जि", "ो स", "र उ", "रक्", "े म", "पूर", " लि", "ाएग", " भा", "इस ", "त क", "ाव ", "स्थ", "पने", "ा औ", "द्ध", "श्य", "र्व", " घो", "घोष", "रूप", "भाव", "ाने", "कृत", "ो प", "े ल", "लिए", "शिक", "ूर्", " उन", "। इ", "ं स", "य क", "्ध ", "दी ", "ी र", "र्य", "णा ", "एगा", "न्य", "रीय", "ेश ", "रति", "े ब", " रू", "ूप ", "परा", "्र ", "तर्", " पा", " सु", "जिस", "तिक", "सार", "जो ", "ेशो", " शि", "ानव", "ी अ", "चित", "े औ", " पू", "ियो", "ा उ", "म क", "ी भ", "शों", " बु", "म्म", "स्त", "िश्", "्रो", "्म ", "ो क", " यह", "र द", "नव ", "चार", "दिय", "े य", "र्ण", "राध", "ोगा", "ले ", "नून", "ानू", "ोषण", "षणा", "विश", " जन", "ारी", "परि", "गी ", "वाह", "साम", "ाना", "रका", " जो", "ाज ", "ी ज", "ध क", "बन्", "ताओ", "ंकि", "ूंक", "ास ", "कर ", "चूं", "ी व", "य ह", "ा ग", "य स", "न स", "त र", "कोई", "ुक्", "ोई ", " ।क", "ं न", "हित", "निय", "याद", "ादी", "्मा", "्था", "ामा", "ाह ", "ी म", "े ज"]),
];
pub const ETHIOPIC_LANGS : LangProfileList = &[
    (Lang::Amh, &["፡መብ", "ሰው፡", "ት፡አ", "ብት፡", "መብት", "፡ሰው", "፡አለ", "፡ወይ", "ወይም", "ይም፡", "ነት፡", "ንዱ፡", "አለው", "ለው።", "ዳንዱ", "ያንዳ", "ንዳን", "እያን", "ዱ፡ሰ", "ት፡መ", "፡እን", "፡የመ", "።እያ", "እንዲ", "፡ነጻ", "፡የተ", "ም፡በ", "ው፡የ", "ም፡የ", "፡የሚ", "ና፡በ", "ን፡የ", "፡የማ", "፡አይ", "ነጻነ", "ና፡የ", "ው፡በ", "ቶች፡", "ው።፡", "ሆነ፡", "ት፡የ", "፡በሚ", "፡መን", "ው።እ", "ትና፡", "ኀብረ", "ትን፡", "ውም፡", "ንኛው", "እኩል", "ብቻ፡", "ኛውም", "ንም፡", "፡ለመ", "፡ያለ", "ም፡ሰ", "ማንኛ", "መብቶ", "፡አገ", "ት፡በ", "ራዊ፡", "፡እኩ", "፡ለማ", "ለት፡", "በት፡", "ሆን፡", "መንግ", "፡በተ", "ረት፡", "ብቶች", "ጋብቻ", "ዎች፡", "ህንነ", "ጻነት", "ም፡እ", "ወንጀ", "፡ልዩ", "ሰብ፡", "ማንም", "ጠበቅ", "ኩል፡", "ደህን", "።ማን", "ነጻ፡", "ግኘት", "ማግኘ", "።፡እ", "፡የሆ", "፡ሁሉ", "ች፡በ", "፡በመ", "ሥራ፡", "፡ደህ", "ፈጸም", "ል፡መ", "ተግባ", "፡ድር", "ት፡ወ", "ው።ማ", "ፍርድ", "ርድ፡", "፡በሆ", "ር፡ወ", "በትም", "ትም፡", "ይነት", "ቸው፡", "ብ፡የ", "ነትና", "ቱን፡", "ሕግ፡", "ንና፡", "፡ሥራ", "የማግ", "፡መሠ", "ኘት፡", "፡ጊዜ", "ጻነቶ", "ነቶች", "በር፡", "በኀብ", "ዩነት", "ልዩነ", "፡በኀ", "፡ዓይ", "ዓይነ", "ችና፡", "ግባር", "ባር፡", "፡ደረ", "ነው።", "፡ነው", "ደረጃ", "ም።እ", "ም፡መ", "፡ወን", "ይማኖ", "ማኀበ", "ሃይማ", "፡ኑሮ", "መሠረ", "ሁሉ፡", "ነቱ፡", "ሌሎች", "ንግሥ", "በቅ፡", "የሆነ", "፡ይህ", "ንዲጠ", "ገር፡", "ተባበ", "ትክክ", "ጸም፡", "ር፡የ", "ዲጠበ", "ትም።", "ው፡ከ", "፡እያ", "ሩት፡", "ድርጅ", "፡ብቻ", "ና፡ለ", "ይገባ", "የመኖ", "፡ማን", "ንነት", "ቤተሰ", "ርጅት", "ት፡ድ", "፡መሰ", "እንደ", "፡አላ", "ብሔራ", "ት፡ለ", "ሔራዊ", "ርት፡", "ህርት", "ውን፡", "የሚያ", "ል።እ", "ሆኑ፡", "ምህር", "ትምህ", "በት።", "ለበት", "አለበ", "፡አስ", "ሎች፡", "ች፡የ", "፡በሕ", "ብረ፡", "፡ከሚ", "ን፡አ", "ት፡እ", "ን፡ወ", "ረግ፡", "በሆነ", "የኀብ", "፡የኀ", "መሆን", "፡መሆ", "ን፡መ", "፡ውሳ", "ንጀል", "ፈላጊ", "ህም፡", "ረታዊ", "ክለኛ", "ክክለ", "ታዊ፡", "ጀል፡", "ኑሮ፡", "።፡ይ", "ዓዊ፡", "ዜግነ", "ንዲሁ", "ዲሁም", "፡ማኀ", "ገሩ፡", "ር፡በ", "ብዓዊ", "አገሩ", "ሁም፡", "ና፡ነ", "ሰብዓ", "የተባ", "ጅት፡", "ማኖት", "ር፡አ", "ንግስ", "ኖት፡", "በሕግ", "መኖር", "ው፡ያ", "መጠበ", "ረጃ፡", "፡በማ", "ነትን", "ብነት", "ገብነ", "፡ገብ", "መፈጸ", "፡ሁኔ", "ሁኔታ", "ን፡ለ", "ው፡ለ", "፡ተግ", "፡የአ", "፡ይገ", "፡በአ", "ችን፡", "፡ትም", "ነቱን", "፡ቢሆ", "ቢሆን", "ጊዜ፡", "ረ፡ሰ", "ት፡ጊ", "ሰቡ፡", "ምበት", "ላቸው", "አላቸ", "በነጻ", "፡በነ", "አንድ", "ቅ፡መ", "፡መጠ", "ት፡ይ", "መሰረ", "ጥ፡የ", "ስጥ፡", "ፈጸመ", "ውስጥ", "ንድ፡", "፡ውስ", "፡በግ", "፡ሆኖ", "ሉ፡በ", "፡ጋብ", "ንስ፡", "ንነቱ", "መው፡", "የሚፈ", "አይፈ", "ብረሰ", "ነ፡መ", "፡የሃ", "ም፡ከ", "ች፡እ", "ስት፡", "ሙሉ፡", "አገር", "ሆኖ፡", "ደረግ", "ኢንተ", "ንተር", "ተርና", "ርናሽ", "ናሽና", "ሽናል"]),
    (Lang::Tir, &[" መሰ", " ሰብ", "ሰብ ", " ኦለ", "ትን ", "ኦለዎ", "ናይ ", " ናይ", " ኦብ", "ዎ፡፡", "ለዎ፡", "ሕድሕ", "ኦብ ", "ድሕድ", "ሕድ ", "መሰል", "ውን ", "ሰል ", "ድ ሰ", "ይ ም", "ል ኦ", "ካብ ", "፡ሕድ", "፡፡ሕ", " ወይ", "ወይ ", " መን", " ነፃ", "ን መ", "ዝኾነ", "፡፡ ", "ታት ", "ብ ዝ", "ነት ", "ን ነ", " ካብ", "መሰላ", "ነፃነ", " እዚ", "ብ መ", "ኦዊ ", "ታትን", "መንግ", "ዊ መ", " እን", "ብ ብ", "ንግስ", "ት ኦ", "ሰላት", "ን ም", "ኾነ ", "እዚ ", "ብኦዊ", "ሰብኦ", "ን ኦ", "ን፡፡", " ንክ", " ዝኾ", "ን ን", " ምር", "ኹን ", "ይኹን", " ይኹ", "ምርካ", "ርካብ", " ኦይ", " ሃገ", "ሕጊ ", "ራት ", "ሎም ", " ብሕ", "ነ ይ", " ከም", "ማዕሪ", "ይ ብ", " ንም", " ዝተ", "ርን ", "ን ብ", "ራዊ ", " ፣ ", "ብ ሕ", "ላትን", "ብ ኦ", "ማሕበ", "ነታት", " ኦድ", "ዕሪ ", " ማዕ", "ስታት", "ግስታ", "’ውን", "ት መ", "ን ዝ", "ታዊ ", "፣ ብ", " ማሕ", "ነትን", "ንጋገ", "ድንጋ", " ስለ", " ድን", "ስራሕ", "ኩሎም", "ሕበራ", "ኦት ", "ን ሰ", "ዓለም", "ፃነታ", " ብም", "ት ወ", "መሰሪ", " ስራ", "ፃነት", "ተሰብ", "ካልኦ", "ልኦት", "ን ሓ", "ዓት ", "ዋን ", "ቡራት", "ሕቡራ", " ሕቡ", "ብሕጊ", "ድብ ", "ውድብ", " ውድ", "ብን ", "ትምህ", "ነቱ ", "ዚ ድ", "፣ ኦ", "ሃገራ", " ኩሎ", "ለዎም", "ምህር", "ም፡፡", "ም መ", " ብዝ", "ምኡ’", "ኡ’ው", "እንት", " ዓለ", " ብዘ", "በራዊ", " ሓለ", "ሓለዋ", "ዎም፡", "ቱ ን", "ት ብ", "ጋገ ", "ነፃ ", " ምዃ", "ን ዘ", " ገበ", "ት፣ ", " ትም", "ኸውን", "ራሕ ", " ዘይ", "ህርቲ", "ርቲ ", "ከምኡ", "ሃይማ", " ምስ", "ነ፣ ", "እንተ", " ስር", "ስርዓ", "ርዓት", "ባት ", "ይማኖ", "ሰሪታ", "ን ና", " ክብ", "ልን ", " ብማ", "ገሩ ", " ህዝ", "ላት ", "ት ና", "ይ ኦ", "ዕሊ ", "ለዝኾ", "ስለዝ", "ሪተሰ", "ብሪተ", "ሕብሪ", " ሕብ", "ን ተ", "ኾነ፣", "በን ", "ሃገሩ", "ገ እ", "ኻዊ ", " ሃይ", "እን ", "ሪጋገ", " ምሕ", "ን እ", "ለኻዊ", "ር፣ ", " ብሓ", " ብሃ", " ክኸ", "ክኸው", "ብ ዘ", "ዃኑ ", "ዊ ክ", "ምን ", "ሓደ ", "ምዃኑ", "ም ን", "ት እ", "ዊ ወ", "ታውን", " ሕድ", "ብዘይ", " ሕጊ", "ት ን", " ልዕ", " ካል", "ን ካ", "ሰባት", "ን ስ", "ናን ", "ቤተሰ", "ሕን ", "ለምለ", "ት ስ", "ምለኻ", "፣ ከ", "ተደን", "ባል ", "ኦድላ", "እዋን", " እዋ", "ደቂ ", " ደቂ", " ሰባ", "ፃን ", "ነፃን", "ግስቲ", "፣ ን", "ዚ ብ", "ስቲ ", " ቤተ", "ምጥሓ", " ክሳ", " ነዚ", "ን ክ", "ነቲ ", " ነቲ", "ነዚ ", " ምእ", "ብነፃ", " ምዕ", "ምዕባ", "ዕባለ", "ክሳብ", " ብነ", "ል እ", "ዚ መ", "ልዕሊ", "ክብሩ", "ብማዕ", "ሳብ ", "ህይወ", "ኦቶም", "ምስ ", "ንገገ", "እምነ", " እም", "ድ ኦ", "ቶም ", "ቲ ክ", "ፍትሓ", "ለም ", " ፍት", "ብ ን", "ን ዓ", "ራውን", "ሓፈሻ", "ደንገ", "ም ብ", "ትዮን", " ዝሰ", "ዝተደ", "ሉ መ", "ብ ና", "ጊ ካ", "ልዎ ", "ኦባል", " ኦባ", "ድልዎ", "ን ድ", "ኦድል", "ዜግነ", "ላውን", " ድሕ"]),
];
pub const HEBREW_LANGS : LangProfileList = &[
    (Lang::Heb, &["ות ", "ים ", "כל ", "ת ה", " כל", "דם ", "אדם", "יות", " של", " זכ", "ל א", " אד", "של ", "ל ה", "אי ", "ויו", "כאי", "ת ו", "י ל", "זכא", " ול", "לא ", " וה", "רות", "זכו", "ית ", "ירו", "ין ", " או", "ם ז", " לא", " הח", "או ", " הא", " וב", " המ", "חיר", "ת ל", "יים", "ם ל", "את ", "ת ב", "ת ש", "רה ", "ון ", " לה", "נה ", "כוי", "ותי", "ה ש", "ו ל", "ו ב", " הו", "ת א", "ם ב", "ם ו", "תו ", " את", "לה ", "ני ", "אומ", " במ", "דה ", "א י", "ה ה", "ה ב", "על ", "ם ה", " על", "הוא", "וך ", "ה א", "בוד", "וד ", "ואי", "נות", "ה ו", "ת כ", "י ה", "יה ", "ם ש", "ו ו", " שה", "ם א", "ו כ", "ינו", "ן ה", " שו", "שוו", "החי", "כות", "לאו", "בות", "דות", "ה ל", "לית", "ה מ", " בי", "וה ", "וא ", " הי", " לפ", "ור ", " לב", "ל ב", "בחי", "הכר", "לו ", "ת מ", "ן ש", "החו", "ה כ", " בכ", "ומי", "בין", "ן ו", "ן ל", "רוי", "פלי", "ולה", "ליה", " הז", "חינ", " לע", " בנ", "יבו", "חוק", " אח", "חבר", " יה", " חי", "מי ", "ירה", " חו", "האד", "ווה", "חופ", "ופש", "וק ", "נו ", "יו ", "ל מ", "מדי", "כבו", " הע", "נוך", " הד", "י א", "י ו", " הכ", "בני", "עה ", "ו א", "רצו", "דינ", "בזכ", "מות", "יפו", " אל", "סוד", "לם ", "איש", "רך ", " אי", "הגנ", "הם ", "פי ", "ם כ", "חות", "ל ו", "איל", "ילי", "תיה", "כלל", "אלי", "יסו", "האו", "זש ", " בא", "ר א", "ו ה", "זו ", "אחר", " הפ", " בע", " בז", "משפ", " בה", " לח", "דרך", "ומו", " בח", " דר", " מע", "ל י", "תוך", "מנו", " בש", "לל ", "רבו", " למ", "פני", " לק", "תם ", "שה ", "שית", "ללא", "לפי", "היה", "מעש", "דו ", "שות", "להג", "וצי", "שוא", "אין", "וי ", "תי ", "ונו", "ליל", " לו", "חיי", "ל ז", " זו", "היא", "יא ", "נתו", "ה פ", "לת ", "ובי", " לכ", "ך ה", "יל ", "י ש", "שיו", "ן ב", "עול", "המד", "ודה", "ולם", " ומ", "א ה", "ולא", " בת", "הכל", " סו", " מש", " עב", "סוצ", "ארצ", " אר", "ציא", "ד א", "לחי", "הן ", "יחס", " יח", "יאל", "הזכ", "ם נ", " שר", "בו ", "עבו", "היס", " לי", "ת ז", "פול", "יהי", "גבל", "תיו", "המא", "שהי", "א ל", "מאו", " יו", "ותו", "ישי", "גנה", "פשי", "וחד", "יהם", "חרו", "לכל", "ידה", "עות", "ונה", "ום ", "חה ", "עם ", "שרי", "ם י", "שר ", "והח", " אש", " הג", "ק ב", "הפל", "נשו", "הגב", "ד ו"]),
    (Lang::Ydd, &[" פֿ", "ון ", "ער ", "ן א", " אַ", "דער", "ט א", " או", "און", "אַר", "ען ", "פֿו", " אױ", " אי", "ן פ", "ֿון", "רעכ", " דע", " רע", "עכט", "פֿא", "ן ד", "כט ", " די", "די ", "אַ ", "אױף", "ױף ", "ֿאַ", " זײ", " גע", "אַל", "אָס", " אָ", "ונג", " הא", "האָ", "זײַ", " מע", "אָל", "נג ", "װאָ", "ַן ", "אַנ", "רײַ", " װא", "ָס ", "באַ", " יע", "יעד", "ניט", "ן ז", "ר א", "יט ", "אָט", "אָר", "עדע", "מען", "זאָ", "ָט ", "פֿר", "ײַן", " בא", "טן ", "אין", "ן ג", "ין ", "ן װ", "נאַ", "ֿרײ", "ר ה", " זא", "לעכ", "ע א", "אָד", "ַ ר", "ענט", "אַצ", "ַצי", "אָנ", " צו", " װע", "יז ", "מענ", "ָדע", "איז", "ן מ", "ַלע", "בן ", "ר מ", "טער", " מי", " פּ", "מיט", "טלע", "ָל ", "עכע", "ײט ", "ַנד", "ע פ", "לע ", "געז", "לאַ", "אַפ", "עזע", "ראַ", " ני", "ַפֿ", "רן ", "ײַנ", "נען", "טיק", "כע ", "פֿע", "יע ", "הײט", "ַהײ", "נטש", "ײַה", "ט ד", "ן ב", "לן ", "ן נ", "פֿט", "שאַ", "רונ", " זי", " װי", "ט פ", " דא", "טאָ", "דיק", "קן ", "ר פ", "ר ג", "יקן", "אָב", "ף א", "אַק", "קער", "ערע", "כער", "י פ", "ות ", "ַרב", "פּר", "קט ", "עם ", "יאָ", "ציע", "ציא", "יט־", "צו ", "ישע", " קײ", "ן ק", "סער", " גל", "דאָ", "ונט", "גן ", "ַרא", "יקע", " טא", "ענע", "לײַ", "שן ", "ַנע", "יק ", "טאַ", "ס א", "עט ", "נגע", "ט־א", "ָנא", "־אי", "יקט", "נטע", "ײנע", "־ני", "ָר ", "װער", "י א", "ן י", "יך ", "זיך", "ער־", "ערן", "אױס", "ָבן", "נדע", "ָסע", "װי ", "ֿעל", "ר־נ", "ן ה", " גר", "גלײ", " צי", "ראָ", "זעל", "עלק", "נד ", "לקע", "אָפ", " כּ", "ט װ", "ג א", " נא", "ט צ", "ר ד", "עס ", "דור", "גען", "קע ", "ג פ", "ֿט ", "ן ל", "שע ", "ר ז", "רע ", "ײטן", "פּע", "קלא", "קײט", "יטע", "ים ", "ס ז", "ײַ ", " דו", "אַט", " לא", "ר װ", "קײנ", "עלש", "י ד", "לשא", "יות", "נט ", "ַרז", "ע ר", "ל ז", "אַמ", "ן ש", " שו", "אינ", "נטל", " הי", "בעט", "ָפּ", "ף פ", "ײַכ", "בער", "ן צ", "מאָ", " שט", " לע", "גער", "ורך", "רך ", "נעם", "גרו", "פֿן", "לער", "װעל", "ע מ", "ום ", "שפּ", "ך א", "יונ", "רבע", "עפֿ", "טעט", "ן כ", "רעס", "ערצ", "ז א", "עמע", "ם א", "שטע", "כן ", "רט ", "י ג", "סן ", "נער", "ליט", "ט ז", "נעמ", "ּרא", "היו", "אַש", "ת װ", "אומ", "ק א", "יבע", "ֿן ", "ץ א", "פֿי", "ײן ", "ם ט"]),
];



#[cfg(test)]
mod tests {
    use super::Lang;

    #[test]
    fn test_from_code() {
        assert_eq!(Lang::from_code("rus".to_string()), Some(Lang::Rus));
        assert_eq!(Lang::from_code("ukr"), Some(Lang::Ukr));
        assert_eq!(Lang::from_code("ENG"), Some(Lang::Eng));
        assert_eq!(Lang::from_code("oops"), None);
    }

    #[test]
    fn test_to_code() {
        assert_eq!(Lang::Spa.to_code(), "spa");
    }
}
