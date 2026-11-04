// trois types: OK ERR EVT

enum Type {
	OK, // OK ARG(si besoin)[LF]
	ERR, // ERR NUMBER NAME[LF]
	EVT, // EVT EVENT_TYPE EVENT_DATA[LF]
}

enum Id_Err {

	None,
}

// et creer un autre type pour evt??

struct Answer {
	nature: Type,
	number: Id_Err,


}