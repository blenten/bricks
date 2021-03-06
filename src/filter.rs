pub fn filter_keystring(keystr: String) -> Vec<String> {
	let mut ommbuf = String::with_capacity(50);
	let mut result = vec![String::with_capacity(20)];
	let mut word_id = 0;
	for c in keystr.to_lowercase().chars().filter(|x| {x.is_alphabetic() || *x == ' '}) {
		if c != ' ' {
			result[word_id].push(c);
		} else {
			if result[word_id].as_str() == "не" {
				continue;
			}

			if !ommbuf.is_empty() {
				ommbuf.push(' ');
			}
			ommbuf.push_str(&result[word_id]);
			if !OMMITS.contains(&ommbuf.as_str()) {
				result[word_id].shrink_to_fit();
				result.push(String::with_capacity(20));
				word_id += 1;
				ommbuf.clear();
			}
			result[word_id].clear();
		}
	}
	result
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn filter_keystr_tst() {
		let input = String::from("Под прудовое&6 хозя1йство Для размещения объектов в виде рыбного### хозяйства");
		let res = vec!["прудовое", "хозяйство", "размещения", "объектов", "рыбного", "хозяйства"];
		assert_eq!(filter_keystring(input), res);
	}
}

const OMMITS: [&'static str; 204] = [
    "а-ля",
    "без ведома",
    "безо",
    "благодаря",
    "близ",
    "близко от",
    "в",
    "в виде",
    "в зависимости от",
    "в интересах",
    "в качестве",
    "в лице",
    "в отличие от",
    "в отношении",
    "в пандан",
    "в пользу",
    "в преддверии",
    "в продолжение",
    "в результате",
    "в роли",
    "в связи с",
    "в силу",
    "в случае",
    "в соответствии с",
    "в течение",
    "в целях",
    "в честь",
    "вблизи",
    "ввиду",
    "вглубь",
    "вдогон",
    "вдоль",
    "вдоль по",
    "взамен",
    "включая",
    "вкруг",
    "вместо",
    "вне",
    "внизу",
    "внутри",
    "внутрь",
    "во",
    "во имя",
    "во славу",
    "вовнутрь",
    "возле",
    "вокруг",
    "вопреки",
    "вослед",
    "впереди",
    "вплоть до",
    "впредь до",
    "вразрез",
    "вроде",
    "вслед",
    "вслед за",
    "вследствие",
    "встречу",
    "выключая",
    "для",
    "для-ради",
    "до",
    "за",
    "за вычетом",
    "за исключением",
    "за счёт",
    "замест",
    "заместо",
    "из",
    "из-за",
    "из-под",
    "из-подо",
    "изнутри",
    "изо",
    "исключая",
    "исходя из",
    "к",
    "касаемо",
    "касательно",
    "ко",
    "кончая",
    "кроме",
    "кругом",
    "лицом к лицу с",
    "меж",
    "между",
    "мимо",
    "на",
    "на благо",
    "на виду у",
    "на глазах у",
    "на предмет",
    "наверху",
    "навроде",
    "навстречу",
    "над",
    "надо",
    "назад",
    "назади",
    "накануне",
    "наместо",
    "наперекор",
    "наперерез",
    "наперехват",
    "наподобие",
    "наподобье",
    "напротив",
    "наряду с",
    "насупротив",
    "насчёт",
    "начиная с",
    "не без",
    "не считая",
    "невзирая на",
    "недалеко от",
    "независимо от",
    "несмотря",
    "несмотря на",
    "ниже",
    "о",
    "об",
    "обо",
    "обок",
    "обочь",
    "около",
    "окрест",
    "окроме",
    "окромя",
    "округ",
    "опосля",
    "опричь",
    "от",
    "от имени",
    "от лица",
    "относительно",
    "ото",
    "перед",
    "передо",
    "по",
    "по линии",
    "по мере",
    "по направлению к",
    "по поводу",
    "по причине",
    "по случаю",
    "по сравнению с",
    "по-за",
    "по-над",
    "по-под",
    "поблизости от",
    "повдоль",
    "поверх",
    "под",
    "под видом",
    "под эгидой",
    "подле",
    "подо",
    "подобно",
    "позади",
    "позадь",
    "позднее",
    "помимо",
    "поперёд",
    "поперёк",
    "порядка",
    "посверху",
    "посереди",
    "посередине",
    "посерёдке",
    "посередь",
    "после",
    "посреди",
    "посредине",
    "посредством",
    "пред",
    "предо",
    "преж",
    "прежде",
    "при",
    "при помощи",
    "применительно к",
    "про",
    "промеж",
    "промежду",
    "против",
    "противно",
    "противу",
    "путём",
    "ради",
    "рядом с",
    "с",
    "с ведома",
    "с помощью",
    "с прицелом на",
    "с точки зрения",
    "с целью",
    "сверх",
    "сверху",
    "свыше",
    "и",
    "неболее",
    "неменее",
    "их",
    "ул"
];