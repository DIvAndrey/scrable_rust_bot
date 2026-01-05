pub const MIN_LETTER_CODE: u32 = 'а' as u32;

const fn make_lookup_table() -> [i8; 34] {
    let mut table = [127; 34];

    let mappings = [
        ('а', 1),
        ('б', 3),
        ('в', 1),
        ('г', 3),
        ('д', 2),
        ('е', 1),
        ('ж', 5),
        ('з', 5),
        ('и', 1),
        ('й', 4),
        ('к', 2),
        ('л', 2),
        ('м', 2),
        ('н', 1),
        ('о', 1),
        ('п', 2),
        ('р', 1),
        ('с', 1),
        ('т', 1),
        ('у', 2),
        ('ф', 10),
        ('х', 5),
        ('ц', 5),
        ('ч', 5),
        ('ш', 8),
        ('щ', 10),
        ('ъ', 10),
        ('ы', 4),
        ('ь', 3),
        ('э', 8),
        ('ю', 8),
        ('я', 3),
        ('ё', 3),
    ];

    let mut i = 0;
    while i < mappings.len() {
        let (c, value) = mappings[i];
        assert!('а' <= c && c <= 'я' || c == 'ё');
        table[(c as u32 - MIN_LETTER_CODE) as usize] = value;
        i += 1;
    }

    table
}

pub const LOOKUP_TABLE: [i8; 34] = make_lookup_table();

pub fn get_char_score(c: char) -> i32 {
    assert!('а' <= c && c <= 'я' || c == 'ё', "{c}");
    let c_u32 = c as u32;
    LOOKUP_TABLE[(c_u32 - MIN_LETTER_CODE) as usize] as i32
}
