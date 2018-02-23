

pub fn greekshort_i(v: u64) -> String {
    const GREEK_SUFFIXES: &'static [&'static str] = &["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];

    let mut number = v;
    let mut multi = 0;

    while number >= 1000 && multi < GREEK_SUFFIXES.len()-1 {
        multi += 1;
        number /= 1024u64;
    }

    let mut s = format!("{}", number);
    s.truncate(4);
    if s.ends_with(".") {
        s.pop();
    }
    if s.len() < 4 { s.push(' ' ); }

    return format!("{}{}", s, GREEK_SUFFIXES[multi]);
}

pub fn greek(v: f64) -> String {
    const GREEK_SUFFIXES: &'static [&'static str] = &["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];

    let mut number = v;
    let mut multi = 0;

    while number >= 1000.0 && multi < GREEK_SUFFIXES.len()-1 {
        multi += 1;
        number /= 1024.0;
    }

    let mut s = format!("{}", number);
    s.truncate(4);
    if s.ends_with(".") {
        s.pop();
    }
    if s.len() < 4 { s.push(' ' ); }

    return format!("{}{}", s, GREEK_SUFFIXES[multi]);
}

/*

pub fn greek(v: f64) -> String {
	const GR_BACKOFF: f64 = 24.0;
	const GROWTH: f64 = 1024.0;
	const KK : f64 = GROWTH;
	const MM : f64 = KK*GROWTH;
	const GG: f64 = MM*GROWTH;
	const TT: f64 = GG*GROWTH;
	const PP: f64 = TT*GROWTH;

	let a = v.abs();
		// println!("hereZ {}  {}  {}", v, MM-(GR_BACKOFF*KK), GG-(GR_BACKOFF*MM));
	let t = if a > 0.0 && a < KK - GR_BACKOFF {
		(v, "B")
	} else if a >= KK - GR_BACKOFF && a < MM-(GR_BACKOFF*KK) {
		// println!("here {}", v);
		(v/KK, "K")
	} else if a >= MM-(GR_BACKOFF*KK) && a < GG-(GR_BACKOFF*MM) {
		// println!("here2 {}  {}  {}", v, MM-(GR_BACKOFF*KK), GG-(GR_BACKOFF*MM));
		(v/MM, "M")
	} else if a >= GG-(GR_BACKOFF*MM) && a < TT-(GR_BACKOFF*GG) {
		// println!("here3 {}", v);
		(v/GG, "G")
	} else if a >= TT-(GR_BACKOFF*GG) && a < PP-(GR_BACKOFF*TT) {
		// println!("here4 {}", v);
		(v/TT, "T")
	} else {
		// println!("here5 {}", v);
		(v/PP, "P")
	};

	let mut s = format!("{}", t.0);
    s.truncate(4);
	if s.ends_with(".") {
		s.pop();
	}

	format!("{}{}", s, t.1)
}

*/
