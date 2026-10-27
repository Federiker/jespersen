# TODO

The shared roadmap (see [AGENTS.md](AGENTS.md#shared-roadmap)). Order, from
[the decisions](docs/decisions.md): English phonemes in manuzio, then the
other languages manuzio needs. jespersen does phonetics only (decision 50).

## Foundation

- [ ] Repository (the owner creates `Federiker/jespersen` and pushes by hand):
  - [ ] Checks green on Windows, macOS and Linux (a manual run with all
    platforms), once it is on GitHub.

## English phonemes

**Done for book narration (decision 30), 23 October 2026.** Measured against
eSpeak NG 1.52.0 as a black box: 88.1% of words on Tom Sawyer (American),
89.6% on "A Scandal in Bohemia" (British); through a local copy of manuzio's
pipeline, 76.5% and 75.8% on its golden lines (years, money and dates read
naturally on purpose); unknown words 51.7% exactly right on held-out CMUdict
words. The open items below are maintenance.

- [ ] Sources for English, checked from their licence texts and recorded in
  [docs/data-sources.md](docs/data-sources.md):
  - [x] American English lexicon candidates: CMUdict (BSD-style) and Moby
    Pronunciator II (public domain) are usable.
  - [x] British English lexicon candidates: none usable (all are
    research-only, paid, or built from Wiktionary or commercial
    dictionaries).
  - [x] Decide how jespersen gets British English: own rules from the
    American forms plus a CC0 exceptions list (decision 20).
  - [x] Commit CMUdict and Moby, pinned, under `data/upstream/`
    (decision 21).
  - [x] A check that the committed upstream files still match their pinned
    SHA-256 (a test in `jespersen_lang_en`).
  - [ ] Word frequencies and part-of-speech data for homographs.
  - [ ] CC0 or public-domain texts for test sets (for example Project
    Gutenberg).
- [ ] The local comparison with eSpeak NG (black box,
  [docs/clean-room.md](docs/clean-room.md)):
  - [x] Text lists per language, jespersen's own (CC0): English in
    `crates/parity/texts/en.txt`.
  - [x] `mise run parity:capture`: runs the oracle named by the environment
    and writes its answers to `tmp/`.
  - [x] `mise run parity`: agreement by line and word, and the most
    frequent differences; skips when there is no capture.
  - [ ] More texts (public-domain prose), and the other languages as they
    come.
- [ ] Phoneme model (`jespersen_phonemes`):
  - [x] Phonemes with a symbol per notation, stress, syllables, and
    syllabification by maximal onset with the language's phonotactics.
  - [x] Writers: plain IPA (stress before the syllable) and eSpeak NG's
    notation (stress before the vowel).
  - [ ] Check secondary stress, long vowels and syllabic consonants in
    eSpeak NG's notation against its output.
- [ ] Text front end (`jespersen_text`), language-independent parts first:
  - [x] Sentences and tokens (UAX #29), with spans, kinds and scripts;
    punctuation kept as tokens; languages supply their abbreviations.
  - [ ] Clauses, for voices that want shorter chunks.
  - [x] Numbers: cardinals, ordinals, years, decades, decimals (English).
  - [x] Fractions, signed numbers (-5), ranges, digit groups and Roman
    numerals (English).
  - [x] Dates (12/05/2024 by locale, January 5, 5 January), times, units
    (5 km, 20°C, km/h) (English).
  - [x] Abbreviations by context (Dr. Jones, Main Dr.; Jan. 5), initialisms
    and acronyms (English).
  - [x] Web and e-mail addresses as one token, read as words (English).
- [ ] Lexicon (`jespersen_lexicon`):
  - [x] Compiled form searched in place, built by each language's build
    script.
  - [ ] Text source format, reviewable in diffs.
- [ ] English (`jespersen_lang_en`):
  - [x] American lookup in CMUdict, with English syllabification and the
    allophones eSpeak NG's notation writes (flapped t, initial ɐ, short i).
  - [ ] Moby's part-of-speech entries and missing words, after review.
  - [x] British English (decision 20): jespersen's own rules from the American
    forms (non-rhotic r with linking r, LOT/PALM and cloth by spelling, GOAT,
    yod, -ary/-ory, no flapping), BATH as an option (decision 28), a CC0
    list of exceptions (data/en-gb/lexicon.tsv), zed, British number reading
    and both spellings recognised (colour, realise, centre).
  - [ ] More British exceptions as reading shows them (stress shifts such as
    controversy, garage; -ile words).
  - [x] Unknown words read from known ones: inflections and common suffixes
    (-s, -ed, -ing, -ly, -ness…) with their spelling changes and sounds, and
    two-word compounds with a syllable boundary between the parts.
  - [x] Letter-to-sound rules learned from CMUdict for the words that still
    fail (decision 29): 51.7% of held-out words exactly right, 58.2% but
    for stress (`mise run en:letters:evaluate`, 23 October 2026).
  - [x] Compound stress: a secondary stress on a closed last syllable after
    the main one is dropped (lighthouse, someone; not Mexico).
  - [x] Better unknown words: contexts of up to five letters each side
    (51.3% to 51.7%). A learned table of stress by word ending was tried
    and gave nothing over the letter contexts (23 October 2026).
  - [ ] Unknown words, later:
    acronyms and initialisms spelled out (BBC), names (Moby).
  - [x] Homographs told apart by the words around them: about fifty
    stress-shift nouns and verbs (record), -ate words (separate), read, lead,
    live, lives, close, use, excuse, abuse, used to, house, tear, wind,
    wound, minute.
  - [ ] British forms per homograph sense (a record: RP ˈrɛkɔːd, not ˈrɛkəd).
  - [x] Glottal t and syllabic n in eSpeak NG's American notation
    (button).
  - [x] CC0 test sets with hand-checked expectations (crates/lang-en/tests:
    American, British, homographs, sentences, numbers).
- [ ] Library API on the `jespersen` facade and the `jespersen phonemes` command:
  - [x] `jespersen::phonemize` (sentences of tokens with pronunciations) and
    `jespersen::write` (plain or eSpeak NG notation, punctuation kept or
    dropped; what cannot be pronounced is reported, not skipped).
  - [x] `jespersen phonemes [-v TAG] [--ipa | --espeak] [--no-punctuation]`,
    from arguments or line by line from standard input.
  - [ ] Script switching for mixed text (decision 25), once a second
    language exists.
  - [x] A Cargo feature per language on the `jespersen` facade (`en`, on by
    default); it also builds with none (`mise run check`).
- [ ] manuzio (the owner integrates it; agents work on a local copy, see the
  local notes):
  - [x] jespersen behind a copy of manuzio's `Phonemizer` seam and pipeline,
    scored on its golden lines: American 75.6% of words, British 73.1%, no line
    failing (23 October 2026).
  - [ ] The rest of the gap: years and money read as eSpeak NG reads them
    (jespersen reads them naturally, by choice), homographs, and lexical
    differences.

## manuzio's other languages

Mandarin first (decision 22), then the languages Kokoro has voices for (Portuguese, then
French), then regular spelling, then the harder ones (decision 39).
manuzio's narration languages:
it, es, fi, pl, cs, sk, hr, sl, tr, id, eu, ro, ca, pt, de, nl, hu, et, lv, lt,
af, sq, sv, no (Bokmål), da, is, cy, ga, fr, ru, and Mandarin (cmn), which
manuzio reads with its own front end today.

- [x] Mandarin, done for book narration (decisions 31–34; see the README and
  docs/data-sources.md). Maintenance from here: more everyday words and
  context rules as real text shows them (`mise run parity` lists the
  characters most often read one by one in the Lu Xun text).
  - [ ] Replace manuzio's own Chinese front end with jespersen (in manuzio:
    `Notation::Misaki` with `ToneSandhi::Written`, and the credit for 開放詞典's
    data).
- Later, if ever: Taiwan's readings (Guoyu) for zh-TW.
- [ ] Spanish, Castilian and Latin American (decisions 35, 36 and 38):
  - [x] Reading from the spelling: stress, diphthongs and hiatus, -mente
    adverbs, function words, jespersen's respellings (méxico → méjico).
  - [x] Front end: numbers with agreement (veintiún años, doscientas casas),
    decimals, ordinals, times, dates, money, percentages, Roman numerals,
    abbreviations, initialisms, addresses.
  - [x] eSpeak NG's notation: β ð ɣ, ŋ and m, ɛ, rhythmic secondary stress,
    jj and ʝ in Latin American Spanish. Galdós' Marianela (chapters I–II):
    99.5% of words as eSpeak NG writes them; Quiroga's La gallina degollada
    and El almohadón de pluma in Latin American Spanish 98.8%; jespersen's own
    list 93.0% (the rest mostly front-end choices jespersen makes differently).
  - [ ] Clause-level stress words eSpeak NG treats specially (ser, qué
    without its accent), if real text shows they matter.
- [ ] Italian (decision 37):
  - [x] Reading from the spelling: stress (penultimate, written accents,
    -issimo, -ico, -avano, -arono, gerunds with pronouns), open and closed
    e and o by rule (glides, -ente, -ello, -io), long consonants, elisions
    (l'amico, dell'acqua, po'); jespersen's own word list for the rest
    (data/it/words.tsv, about 400 words).
  - [x] Front end: numbers as one word (milleduecento, ventitré), ordinals,
    times, dates, money, percentages, Roman numerals as ordinals,
    abbreviations, initialisms, addresses.
  - [x] eSpeak NG's notation: kː and tsː, rɾ, z between vowels, ʊ and ɪ,
    long ɲ, its glides as vowels, function words and lighter stresses.
    Collodi's Pinocchio (chapters I–III): 90.5% of words as eSpeak NG
    writes them; jespersen's own list 90.1%.
  - [x] Heard through Kokoro by the owner (27 October 2026, kempelen's
    listening checks): ʎ after the stressed vowel is long, written ʎʎ as ɲɲ
    is (meglio ˈmɛʎʎo, famiglia, where eSpeak NG writes one; short before
    the stress: sbagliavo, migliore), and an s that begins a compound's
    second word stays voiceless (stasera, girasole, ventisei; `|` in
    data/it/words.tsv marks the join).
  - [ ] Number words longer than tens and units keep a voiced s where a
    sei or sette joins them (trecentoventisei, milleseicento): mark the
    join in numbers.rs rather than listing each one.
  - [ ] Other compounds of the kind (risalire, risuonare): judged from
    Italian dictionaries, added to the word list as they turn up.
  - [ ] The conditional's first person in -rei is stressed on its last
    syllable (farei faˈrei̯, potrei, vorrei, sarei; read ˈfarei̯ today), found
    with the -ai fix of Jerome's verse report (27 October 2026). Unlike
    -ai, -ei has words stressed earlier (spontanei, aerei, from -eo):
    - [ ] -rei after a verb stem, and a list of the exceptions.
  - [ ] Grow the word list from real text: the rest is mostly words whose
    stress or open vowels the rules miss (judged from Italian dictionaries,
    never copied from eSpeak NG, decision 27), and enclitic verb forms
    (difendersi, rimettiamoci).
  - [ ] No clean lexicon found yet (PhonItalia is non-commercial).
- [x] Finnish: words from the spelling (long sounds and diphthongs as one
  nucleus, stress on the first syllable, rhythmic secondary stresses),
  numbers, dates, money and abbreviations. Aho's Rautatie (chapter I):
  93.1% of words as eSpeak NG writes them; jespersen's own list 86.2%. Left:
  eSpeak NG's boundary gemination (mitä kːuːluː), set by individual words.
- [x] Turkish: words from the spelling (ğ, vowel harmony in suffixes),
  stress on the last syllable unless an unstressed suffix (-yor, -sınız,
  -dır) or jespersen's list moves it, a name keeping its stress before its
  suffix (Ankara'da); eSpeak NG's lax vowels, ɟ, ɫ and ɾ; numbers, dates,
  money (%50 yüzde elli) and abbreviations. jespersen's own list: 68.8% of
  words as eSpeak NG writes them, much of the rest its own stresses
  (dˈeniz for deniz), where jespersen keeps the standard. No public-domain
  Turkish prose found on Project Gutenberg yet.
- [x] Indonesian: words from the spelling (ng, ny, sy, kh, ai, au, oi),
  stress on the last syllable but one, e as ɛ under the stress and ə
  elsewhere, o before r as ɔ, prepositions unstressed in running text;
  numbers (se- forms), dates, money (Rp 1.250 read after its number) and
  abbreviations. jespersen's own list: 89.7% of words as eSpeak NG writes them;
  most of the rest are initialisms eSpeak NG spells in English. The e that
  is e rather than ə (meja, enak) needs a word list.
- [x] Polish: words from the spelling (cz, sz, rz, ch, dz, dż, dź, the i that
  only softens, old é), voicing inside and across words, ą and ę, stress on
  the last syllable but one (third from the end in -liśmy, -łaby and -yka
  words); numbers with their three forms (dwa tysiące, pięć tysięcy), dates
  and centuries as ordinals, money (22 zł dwadzieścia dwa złote) and
  abbreviations. Prus's O odkryciach i wynalazkach: 93.3% of words as eSpeak
  NG writes them, jespersen's own list 93.6%; jespersen keeps dz where eSpeak NG
  writes dʑ (bardzo) and reads dates and zł in words.
- [x] Portuguese, European by default and Brazilian as an option (decision
  40): words from the spelling (digraphs, the readings of c, g, s, r and x,
  nasal vowels and diphthongs, glides), stress from the accents and endings,
  each variety's vowels by their place from the stress (Portugal's ɐ ɨ u,
  Brazil's tʃ dʒ and vocalised l), eSpeak NG's rhythmic stresses, s and final
  vowels across words, function words; jespersen's own list of open e and o and
  of x read ks (data/pt/words.tsv); numbers with gender, dates, times, money,
  ordinals and abbreviations. Eça de Queirós's Contos (1911 spelling): 90.3%
  of words as eSpeak NG writes them for Portugal, 93.4% for Brazil; jespersen's
  own list 89.1% and 94.7%. Left: the open e and o of more words and verb
  forms (the list grows with the texts), and a Brazilian public-domain text in
  a modern enough spelling (Gutenberg's are pre-1911).
- [x] French: words from the spelling once their silent endings are settled
  (-er é, -ent of verbs silent, final s, x, z, t, d and p dropped), vowel
  groups, nasal vowels, the glides of bien, nuit and fille, e as é, è, ə or
  silent and the ə dropped between single consonants (samedi samdi);
  liaison (z, t, n, p) before a vowel but not before an aspirated h or after
  et, elision (l'école), eSpeak NG's dash after le, la, de and the like;
  jespersen's own lists of respellings and aspirated h (data/fr); numbers
  (soixante et onze, quatre-vingts), dates, times, money, ordinals and
  abbreviations. Daudet's Lettres de mon moulin: 92.8% of words as eSpeak NG
  writes them, jespersen's own list 93.8%. Left: liaison by grammar (eSpeak NG
  links some plural nouns and not others), homographs (les poules couvent,
  le couvent), centuries written XXe, the ə kept before two consonants
  across words (quelque chose).
- [x] Czech: words from the spelling (ch, dž, ou, the soft d, t and n before
  ě and i, ě as jɛ and mɲɛ, syllabic r and l, long vowels), voicing inside
  and across words (eSpeak NG's notation keeps a voiced letter voiced before
  a vowel: než odejdeš), stress on the first syllable with eSpeak NG's
  secondary on the third; prepositions said with the next word (na stole
  ˈnastolɛ), the ones of one consonant voiced by it in plain IPA; jespersen's
  own list of loanwords with a hard d, t and n (data/cs); numbers with
  their gender and forms, dates and centuries as ordinals, money and
  abbreviations. Čapek's R.U.R.: 95.9% of words as eSpeak NG writes them,
  jespersen's own list 93.1%.
- [x] Slovak: as Czech (jespersen_lang_sk started from it), with ä, ô, the
  diphthongs ia, ie and iu, ľ, ĺ and ŕ, v said u̯ after a vowel (domov), d, t
  and n soft before e and i but hard before é and in -ej, and a final
  obstruent voiced before a vowel or a sonorant too (otec ide); na, o, pri
  and bez join the next word. jespersen's own list: 91.1% of words as eSpeak NG
  writes them. No public-domain Slovak text on Project Gutenberg, nor a
  Wikisource one from a known edition (see data-sources.md).
- [x] Croatian: words from the spelling (dž, lj, nj, syllabic r), voicing
  across words and none lost at the end, the stress on the first syllable
  as eSpeak NG reads it (it passes over a syllabic r: trgu trɡˈu; Croatian's
  own pitch accent is not written and not read), and eSpeak NG's
  unstressed vowels (æ, ɪ, ʊ); numbers with their gender and forms, dates,
  times, money and abbreviations. jespersen's own list: 93.4% of words as
  eSpeak NG writes them; no Croatian text on Project Gutenberg.
- [x] Slovenian: words from the spelling (dž, lj and nj soft before i and at
  the end, syllabic r said ər, l and v said u̯ after a vowel, v said u
  before an obstruent at the start, e as ə in a last syllable before r or
  n), voicing that follows the next word, the stress on the last syllable
  but one as eSpeak NG reads it (the last in -aj, -al, -il, -ač, -oj), the
  stressed vowel long; v, z, s and k joined to the next word; jespersen's own
  list of stresses and open vowels written as Slovenian dictionaries write
  them (data/sl); numbers (enaindvajset), dates, money with its dual and
  abbreviations. Jurčič's Deseti brat: 96.2% of words as eSpeak NG writes
  them, jespersen's own list 94.4%.
- [x] Basque: words from the spelling (tz, ts, tx, tt, dd, ll, rr, z and s
  laminal and apical, h silent, l and n palatal after i, the diphthongs),
  the stress on the second syllable as eSpeak NG reads it (the first of a
  word of two ending in a vowel) with its secondary on the last; b, d and g
  soft in eSpeak NG's notation but at the start and after a nasal; numbers
  counted by twenties, dates, % before its number, money and abbreviations.
  jespersen's own list: 95.4% of words as eSpeak NG writes them; no Basque text
  on Project Gutenberg.
- [x] Romanian: words from the spelling (ă, â and î, ș and ț, c and g soft
  before e and i, che, chi, ghe and ghi, the diphthongs ea and oa, i and u
  as glides beside a vowel, a final i only softening the consonant before
  it, r a tap between vowels), the stress as eSpeak NG reads it (the last
  syllable but one after a vowel, the last after a consonant, in -ai, -oi,
  -ou and the past in -u; the articles -ul and -ului passed over) with its
  rhythmic stresses; the older spelling's grave accents; jespersen's own list
  of words said with j (este, el) and of stresses the rules miss (așa);
  numbers with their genders (două, douăzeci de mii), dates, money and
  abbreviations. Slavici's Povești: 88.3% of words as eSpeak NG writes
  them, jespersen's own list 85.3%.
- [x] Catalan (central): words from the spelling (the accents, unstressed a
  and e as ə and o as u, ll, ny, l·l, ix, tx, tg, tj, a final ig, the
  glides, a final r, t and p unsaid where Catalan drops them), the stress
  from the accent or the ending, a stressed e or o open or closed by the
  letters after it as eSpeak NG reads them; across words b, d and g soft,
  s and the consonants ending a word voiced, n and l assimilated, i and u
  as glides beside a vowel, que before a vowel, elisions (l'home), verbs
  with their pronouns (menjar-se) and adverbs in -ment with both stresses;
  jespersen's own list of open and closed vowels and final r from standard
  Catalan; numbers with their genders (dues, dos-cents), dates, money and
  abbreviations. Bertrana's El meu amic Pellini: 92.6% of words as eSpeak
  NG writes them, jespersen's own list 90.3%.
- [ ] German.
  - [x] Words from the spelling (long vowels before one consonant and a
    vowel, after h, doubled or in ie, short before two consonants; ch as ç or
    x, sp and st as ʃp and ʃt at a stem's start, s as z before a vowel, final
    devoicing, -ig, an unstressed e as ə and -er as ɐ), the stress on the stem
    after an unstressed prefix, on loanword endings and on the second part of
    da-, wo- and her- adverbs, a secondary one on a stem after a stressed
    prefix; the function words unstressed; jespersen's own list of vowel lengths
    and stresses the rules miss, and of the old spelling's ß (daß); numbers
    grouped as eSpeak NG says them, dates, money and abbreviations. Kafka's
    Die Verwandlung: 88.1% of words as eSpeak NG writes them, jespersen's own
    list 89.9%.
  - [ ] Compounds: the second part's long vowel and secondary stress
    (Augenblick, Flughafen, Bahnhofstraße) need a list of stems or a
    permissive pronunciation dictionary (see One sweep).
- [x] Dutch: words from the spelling (a vowel long in an open syllable and
  doubled, short in a closed one; oe, eu, ui, ei and ij, ou and au; g and ch
  as ɣ and x, sch as sx at a stem's start and s elsewhere, final devoicing,
  an unstressed e as ə, -ig as əx and -lijk as lək), the stress on the stem
  after an unstressed prefix and on loanword endings (-teit, -eren, -tie,
  -ij), eSpeak NG's secondary stresses before and after it; the function
  words unstressed, het and de with ə, t over a t or d after it; jespersen's
  own list of stresses the rules miss; numbers grouped as eSpeak NG says
  them, dates, money and abbreviations. Couperus's De stille kracht: 84.8%
  of words as eSpeak NG writes them, jespersen's own list 87.3%.
- [x] Hungarian: words from the spelling (the vowels long with an accent, the
  digraphs, doubled consonants long, ː after a stop and the consonant twice
  in eSpeak NG's notation, short before another consonant; t, d, n and l
  joined to a j after them, a voiced obstruent unvoiced before a voiceless
  one, sz before s, t before c), the stress on the first syllable, the
  articles and commonest conjunctions lighter; numbers, dates in the
  Hungarian order, money and abbreviations. Gárdonyi's Egri csillagok:
  86.9% of words as eSpeak NG writes them, jespersen's own list 85.3%; most
  of the rest is eSpeak NG's own secondary stress on single words.
- [x] Afrikaans: words from the spelling (a vowel long in an open syllable and
  doubled, short in a closed one; ee as iə, oo as uə, a short i as ə, e as æ
  before k, l, r and g; oe, eu, ui, y, ou, ê, ô and û; ij in the older
  spelling; g as x, v as f, w as v but after a t, k, s or d starting a
  word; a, e and o nasal before ns; the d of -onder unsaid, a ə in final rm
  and lm), the stress on the stem after an unstressed prefix and on
  loanword endings (-eer, -teit, -ade), the function words unstressed with
  ə, 'n as ə; jespersen's own list of stresses the rules miss; numbers grouped
  as eSpeak NG says them, dates, money and abbreviations. Von Wielligh's
  Boesman-Stories: 87.9% of words as eSpeak NG writes them, jespersen's own
  list 91.3%.
- [x] Latvian: words from the spelling (the vowels long with a macron, o as
  uo in native words and o in loanwords, long after their first syllable;
  e and ē open before a back vowel or l, r or n and a dental, as eSpeak NG
  reads native words; ie, ai, ei, au and ui; the palatals ķ, ģ, ļ and ņ, č,
  š, ž, dz and dž; l written ll before a voiced consonant in eSpeak NG's
  notation), the stress on the first syllable, the function words
  unstressed; jespersen's own list of loanwords the rules miss; numbers with
  their genders, dates, money and abbreviations. jespersen's own list: 88.3%
  of words as eSpeak NG writes them; no Latvian text on Project Gutenberg.
- [x] Estonian: words from the spelling (each vowel as written, doubled long
  and overlong in a word of one syllable, two vowels a diphthong in the
  first syllable and before i; a doubled consonant long, and a single p, t
  or k after a long vowel or a diphthong; l, n, s, d and t palatalised
  before i or j after the first vowel; the vowel after a short first
  syllable as eSpeak NG writes it; the older spelling's w as v), the stress
  on the first syllable with a secondary one on the third of a longer word,
  and on the long vowel of a loanword whose first vowel is short; the short
  pronouns and conjunctions unstressed, the long ones lighter; numbers,
  dates, money and abbreviations. Kitzberg's Libahunt (a play, the only
  Estonian text on Project Gutenberg): 91.2% of words as eSpeak NG writes
  them, jespersen's own list 85.5%. Palatalisation the spelling does not show
  (hunt huɲt) is left: it needs a word list.
- [x] Swedish: words from the spelling (a stressed vowel long before one
  consonant and short before two, o as uː or ɔ, å as oː or ɔ, u as ʉː or ɵ;
  the sj and tj sounds, soft k and g at the start, j for g after r and l;
  rs and rt as ʂ and ʈ, ng as ŋ, a doubled t or k long; an unstressed e as
  ə, unstressed a, -ig and closed o, u and å short), the stress on the first
  syllable as eSpeak NG reads it, on be- stems and loanword endings (-tion,
  -ent), eSpeak NG's secondary stress after it; the function words
  unstressed; jespersen's own list of short vowels and stresses (han, miljon,
  och); NST's own entries for nouns, adjectives, verbs, numbers and names
  (decision 42), before the rules: its retroflex consonants (barn ˈbɑːɳ,
  bˈɑːrn in eSpeak NG's notation), stress and vowels, with eSpeak NG's
  ə, long final o and i (varit vˈɑːriːt) and doubled t and k in its notation;
  numbers grouped as eSpeak NG says them, dates, money and abbreviations.
  Lagerlöf's Valda berättelser: 85.8% of words as eSpeak NG writes them
  (85.7% from the rules alone), jespersen's own list 81.2% (81.6%); much of the
  rest is eSpeak NG's own word-by-word choices (hade and inte with ɛ),
  loanword stress and NST's real Swedish where eSpeak NG reads the spelling
  (förstod fœʂˈtuːd, stöt).

- [x] Norwegian Bokmål: words from the spelling (a vowel long before one
  consonant and short before two, o as uː or ɔ, å as oː or ɔ, u as ʉ, e as
  æ before r; ei, øy, au and a final eg; the sj and kj sounds, tj, j for g
  before a front vowel and for gj, hj and lj; rs as ʃ, hv as v, d unsaid
  after r and lengthening l and n (in the older spelling's vilde, kunde);
  a doubled consonant long, ː after a stop and the consonant twice in
  eSpeak NG's notation; an unstressed e in a last syllable as ə, written a
  at the end), the stress on the first syllable as eSpeak NG reads it, but
  after be-, its secondary stress after it; the function words unstressed;
  jespersen's own list of function words from standard Norwegian (og → å);
  numbers grouped as eSpeak NG says them, dates, money and abbreviations.
  Hamsun's Sult: 81.1% of words as eSpeak NG writes them, jespersen's own list
  81.1%; much of the rest is eSpeak NG's spelling pronunciation of og and
  om.
- [x] Albanian: words from the spelling (the digraphs dh, gj, ll, nj, rr, sh,
  th, xh and zh; ç, q, x and y; ë as ə, ʌ under a stress in eSpeak NG's
  notation; e close after j, q, nj, sh and l as eSpeak NG reads it; u before
  a and j after a vowel as glides; a ə before an rr starting a word), the
  stress as eSpeak NG reads it (the last syllable of a word ending in a
  consonant, else the one before; before a final ë and the endings -et,
  -in, -it, -ave and -ur), the function words unstressed; numbers grouped as
  eSpeak NG says them (njëzet-e-një, pesë-mbëdhjetë), dates, money and
  abbreviations. jespersen's own list: 92.9% of words as eSpeak NG writes them;
  no Albanian text on Project Gutenberg. eSpeak NG's own stress on some
  words (qytet, mëngjes, zotëri) is left; standard Albanian stresses qytet
  and mëngjes on the last syllable.
- [x] Lithuanian: words from the spelling (the long vowels ą, ę, ė, į, y, ū
  and ų; the diphthongs ie, uo, ai, au and ui, ei and iau as two vowels; an
  i between a consonant and a back vowel only softening it, -iai one
  syllable; the consonants soft before a front vowel, t, n, s, z, l, š and ž
  before a soft consonant too, written as eSpeak NG writes them (ʂ, ɕ, ɭ,
  l̩); voicing assimilation, final devoicing, a doubled consonant said
  once), the stress on the last syllable but one as eSpeak NG places it
  without its dictionary, ir and the conjunctions unstressed; jespersen's own
  list of the numbers' standard stresses (penkì, dvýlika); numbers with
  their genders and the forms their nouns take (du tūkstančiai, dešimt
  eurų), dates, money and abbreviations. jespersen's own list: 88.0% of words
  as eSpeak NG writes them; no Lithuanian text on Project Gutenberg. The
  lexical stress and tones (vaĩkas) need a stress dictionary (One sweep).
- [x] Welsh: words from the spelling (the stress on the last syllable but one;
  y as ɨ in a last syllable and ʌ before it, u as ɨ, w as a vowel or a
  glide, i as j before a vowel; the diphthongs; ch, dd, ff, ll, ng, ph, rh,
  th, si; f as v; a stressed vowel long before b, ch, d, dd, f, ff, g, s and
  th or at the end, long with a circumflex), the northern vowels as eSpeak
  NG reads them; the function words and their elided forms (a'r, mae'r)
  unstressed, with ə for a y that starts them; jespersen's own list of
  stresses the rules miss (Cymraeg); numbers in the decimal system grouped
  as eSpeak NG says them, dates, money and abbreviations. O. M. Edwards's
  Cartrefi Cymru: 88.9% of words as eSpeak NG writes them, jespersen's own
  list 89.7%.
- [x] Icelandic: words from the spelling, stressed on the first syllable
  (the vowels long before one consonant and before p, t, k or s with j, r or
  v; á, ó, æ, au, ei and ey as diphthongs, a, e and o before ng, nk and gi
  too; é as jɛ; p, t and k aspirated at the start and plain elsewhere,
  preaspirated when doubled and before l and n; ll and nn as tl and tn; f,
  g and the voiceless l, m, n and r where the spelling places them; hv as
  kv; the ə eSpeak NG says between a stop and r), the function words
  unstressed or lighter; numbers with the genders of one to four and joined
  as eSpeak NG says them (tuttugu-og-einn, fjöru-tíu), dates, money and
  abbreviations. jespersen's own sentences: 83.9% of words as eSpeak NG writes
  them; Jón Ólafsson's Alaska (1875): 78.2%. Most of the rest is lexical:
  the second stress of compounds (skólalóðinni, regnbogi), eSpeak NG's own
  number words (þrír θr#ˈiːrr#, tveir), plús and Alaska. 18 of Alaska's
  sentences are not read yet: abbreviations with several full stops (t.d.,
  o.s.frv., þ.e.) and the fractions ¼, ½ and ¾ need the shared reader.
- [x] Danish: words from the spelling (a vowel long before one consonant and
  a vowel, short before two, long in a word of one syllable before d, s, r
  and some l and n; a as ɑ before r, m, k, v and ng; short i as e before n,
  m, k, d, t, l and s with a consonant; e as ɛ before r; o as ʌ, u as ɔ
  before n, m, f and g; ej, øj, ov, og and av as diphthongs; the soft d, d
  unsaid after n and l, g after a vowel as j, w or unsaid, -igt as it, v as
  w; r after a vowel as ɐ̯ or in the vowel, the unstressed -er, -erne and -re
  as ɐ, -et as əð, t after a consonant before ə as d; the older spelling's
  aa as å), the stress on the first syllable, after be-, ge- and for- and on
  loanword endings (-ion, -tet, -ere), eSpeak NG's rhythmic secondary
  stresses and its glottal-stop marks on full vowels; the function words
  unstressed; jespersen's own list of function words from standard Danish (og
  → o', det → de, hun → hu'n, er → ær); numbers grouped as eSpeak NG says
  them (fire-og-tyve), dates, money and abbreviations. NST's lexicon (CC0,
  decisions 41 and 43) before the rules and after jespersen's list: its SAMPA
  with the stød in plain IPA (hund ˈhunˀ), and in eSpeak NG's notation its
  vowels, rhythmic stresses and spelt p, t and k (efter ˈεftʔʌ), without the
  stød; a homograph read as its function word (man, den, sig). jespersen's own
  list: 61.9% of words as eSpeak NG writes them; J. P. Jacobsen's Fru Marie
  Grubbe: 67.0% (64.2% from the rules alone). Much of the rest is eSpeak NG's
  own word-by-word notation (den dɛn, sig saɪ, de dʔi), and NST's real
  Danish where eSpeak NG reads the spelling (dag dˈaj).
- [x] Irish: words from the spelling (broad and slender consonants from the
  vowel next to them, slender c, t and d as eSpeak NG writes them; the vowel
  groups ao, aoi, ea, ei, ia, io, oi, ua, ui, iai, uai, eó, with the síneadh
  fada long and á, ó before a slender consonant gliding; a and o before bh,
  mh, dh and gh as aʊ and aɪ; lenition, eclipsis, the t-, n- and h- that a
  word before adds, and d'; final -adh, -igh and -idh, the f of the future as
  h, the ə in orm and gnó), the stress on the first syllable, on a long second
  one after a short first and on a final -ís, as eSpeak NG's southern reading
  places it; the article, particles and possessives unstressed and reduced;
  jespersen's own list, checked against Sjoestedt's descriptions of Kerry Irish
  (decision 45): second-syllable adverbs (amach, isteach, anois), acu, aige
  and aici stressed on their second syllable, air, is and raibh, ag with its g
  only before a vowel (ag teacht ɛ ˈtʲaxt) and ar with a broad r, féin never
  lenited; numbers grouped as eSpeak NG says them (fiche-a-haon,
  dhá-chéad), dates, money and abbreviations. An tAthair Peadar Ua
  Laoghaire's Niamh (old spelling): 85.8% of words as eSpeak NG writes them,
  jespersen's own list 87.7%. Most of the rest is eSpeak NG's own word forms
  (féin hˈeːn, which Kerry does not lenite) and the old spelling's quirks.
- [ ] Danish's NST lexicon:
  - [ ] Trim jespersen's own list (data/da/words.tsv) to what NST lacks, entry
    by entry, without lowering the scores.
  - [ ] Match names and all-capital entries as Swedish does: Danish's build
    lower-cases every NST word, which in Swedish turned vi into the numeral
    VI and oss into OSS.
- [x] Russian (decision 44): words from the spelling (hard and soft
  consonants, softened too by a soft one after them; е, ё, ю and я as j and a
  vowel; the unstressed vowels reduced by their distance from the stress; ж,
  ш and ц hard, ч and щ soft; voicing inside words and across them; сч, тс,
  -тся, стн, здн, вств, -ого and the silent л of солнце), the stress from an
  acute or ё in the text, then from jespersen's own list of about 2,150 words,
  word forms and stems whose endings keep or take the stress (data/ru,
  written from the standard language, ё put back where the text leaves it
  out), with up to two prefixes before them, then from the endings whose
  stress is predictable (-а́ние, -и́ровать, the vowel before -ость, -тель and
  -ция, an adjective's stem, вы- before a perfective) and otherwise the last
  vowel but one; the prepositions and particles unstressed as eSpeak NG reads
  them (на nə, для dɭʲɑ); eSpeak NG's notation (ɭ, ɑ, ɵ, u", ʑ, ja and ju as
  one vowel, ь after н, л, р and в); numbers with their gender, ordinals for
  dates, years and centuries, money and abbreviations. Женское
  международное движение (1920): 89.3% of words as eSpeak NG writes them,
  jespersen's own list 91.2%. Much of the rest is eSpeak NG's own reading of
  digits (одна тысяча, двадцатьпять as one word) and of ё left unwritten
  (еще, ее), and its errors (ле́су, уже́ as у́же), where jespersen follows the
  language.
- [ ] Russian, later:
  - [ ] Grow the word list from real text (the stress of verbs' present
    forms and of nouns whose stress moves), judged from the standard
    language, never from eSpeak NG's answers (decision 27).
  - [ ] Homographs by context (ру́ки and руки́, до́ма and дома́): the list
    keeps one reading of each.
  - [ ] A longer public-domain prose text: Project Gutenberg has little
    Russian in modern spelling (the 1920 collection gives 163 sentences).
- [ ] Questions only the owner can ask upstream (decision 46); each would
  open a source the sweep could not clear:
  - [ ] Dansk Sprognævn: when COR.TALE (Danish pronunciations by syntactic
    role, said to be CC0) will be released.
  - [ ] The LIEPA-3 team (Vilnius University): how the stress and tone in
    its phoneme layer were made, for a Lithuanian lexicon.
  - [ ] Silero: what its Russian stress model was trained on.
  - [ ] Grammatek: where ice-g2p's Icelandic compound maps come from.
  - [ ] manuzio (the owner): ship eSpeak NG's dictionaries for ms, sr, mk,
    sw, el and om in its helper, so those languages can be compared too.
    Their own sentences (crates/parity/texts/<code>.txt) are ready; the
    capture and report lines go into mise.toml once a capture has answers.
- [ ] The parity tool: refuse a capture whose answers are all empty (today a
  voice without its dictionary writes 33 empty answers and exits 0).
- [ ] Single letters on their own (`x y z`, `a b c`, a list's `b)`, an initial
  without its full stop), found through manuzio's golden lines on 24 October
  2026: Italian says `ks ˈi dz` where eSpeak NG says the letter names (ics,
  ipsilon, zeta). The same probe (`b c x`) across the languages:
  - Letter names already: cs, da, de, en, eu, hr, mk, pl, ro, ru, sk, sl, sr.
  - Sounds instead of names: af, ca, cy, es, et, fi, fr, ga, hu, is, it, lt,
    lv, nb, nl, pt, sq, tr.
  - Refused (cannot pronounce): ak, ee, el (Latin letters), ha, id, ig, lg,
    ln, ms, om, rn, rw, sn, so, sw, yo. manuzio retries a refused Latin word
    as American English, so its readers hear English names there.
  - [ ] Swedish reads a lone c as the Roman numeral (`hˈɵndra`); check the
    other letters that are numerals (i, v, x, l, d, m) in every language.
  - [ ] Decide when a lone letter is a name: not where it is a word (Italian
    a, e, o; Spanish y, o, u; Swedish i, å; French à, y), and not a Roman
    numeral in a heading or after a king's name.
  - [ ] Each language's letter names, as Serbian and Malay have them, from
    the language's own alphabet (judged from its grammars, never copied from
    eSpeak NG, decision 27).

manuzio's book languages added on 24 October 2026, typeset only there until
jespersen reads them (decisions 47 and 48):

- [x] Malay: words from the spelling as Malaysia's standard (Johor-Riau)
  speech says them (ng, ny, sy, kh, gh; the diphthongs ai, au and oi only at
  the end of a word, main ma.in; a final k as ʔ; a final open a as ə, saya
  ˈsajə), the stress on the last syllable but one, or on the last when that
  one has ə (besar bəˈsar); e as ə, and jespersen's own list of words whose e is
  e (meja, kereta, merdeka); numbers (lapan, bilion), dates, ringgit and
  abbreviations; letters named as in English (bi, si, dabelyu). No oracle
  yet: jespersen's own sentences (crates/parity/texts/ms.txt) wait for eSpeak
  NG's dictionary.
  - [ ] Times written with a point (pukul 10.30) are read as decimals by the
    shared front end.
- [x] Serbian, in Cyrillic or Latin letters (sr, sr-Cyrl, sr-Latn):
  Cyrillic turned into Latin letter by letter (љ lj, њ nj, џ dž, ђ đ, ћ ć),
  then read as Croatian is (jespersen_lang_sr started from it), with the
  ekavian standard's numbers (dve, dvesta), hiljada for a thousand, the months
  in the genitive (trećeg maja), dinars, Serbian letter names (lje, nje, dže)
  and abbreviations in both scripts. No oracle yet: jespersen's own sentences
  (crates/parity/texts/sr.txt) wait for eSpeak NG's dictionary, and eSpeak
  NG's notation follows what its Croatian voice writes until then.
  - [ ] A day written as an ordinal with a full stop (3. maja) is read as a
    cardinal, as in Croatian.
- [x] Swahili: words from the spelling (ch, sh, ny, ng', dh, th, gh, kh, ng
  as ŋɡ, every vowel its own syllable), m and n syllabic before a consonant
  they do not join (mtu, nchi, mbwa, mwanamke) and the stress on the last
  syllable but one, counting them; the connectives and copulas unstressed;
  numbers (kumi na moja, mia moja ishirini na tano, elfu), dates, shillings
  and percentages (asilimia before the number), abbreviations. jespersen's own
  sentences only, no oracle yet; time of day is still said as numbers, not
  in Swahili hours (saa nne for 10 o'clock).
- [x] Greek: words from the spelling, monotonic or polytonic (accents,
  breathings and the iota subscript folded, the diaeresis kept): the vowel
  digraphs, αυ and ευ as av/af and ev/ef by the next sound, μπ ντ γκ as b d
  ɡ at the start and mb nd ŋɡ inside, γγ, τσ, τζ, ξ, ψ; κ γ χ fronted to c ʝ
  ç before e and i; σ as z before a voiced consonant; an unstressed i-sound
  between a consonant and a vowel as a glide (παιδιά, ελιά, μια, πιάνο), but
  a vowel after ρ before o or u (αύριο, κύριος); doubled consonants said
  once; the stress where the accent is written, else on the last syllable
  but one; articles, weak pronouns and particles unstressed; numbers agreeing
  with the noun after them (μία, τρεις, διακόσιες, χιλιάδες), dates in the
  genitive, money, abbreviations, Greek and Latin letter names. jespersen's own
  sentences only, no oracle yet. Open: learned words with hiatus elsewhere
  (a short list, judged from Greek dictionaries), sandhi across words (τον
  πατέρα), polytonic function words (ὁ, τὸ) not yet unstressed.
- [x] Macedonian: words from the Cyrillic spelling (ѓ ɟ, ќ c, ѕ dz, љ ʎ, њ ɲ,
  џ dʒ; л dark except before е, и and ј; р as a syllable with no vowel
  beside it, прст, 'рѓа), voicing in clusters and a final obstruent
  voiceless (град ɡrat), and the stress on the third syllable from the end
  of the accentual word: the clitics (не, ќе, the short pronouns) join the
  stressed word after them (не знам ˈnɛ znam, не разбирам nɛ ˈrazbiram);
  numbers with и before the last part (илјада деветстотини седумдесет и
  пет), dates with the day as an ordinal (трети мај), denars and
  abbreviations (д-р). No oracle yet: jespersen's own sentences
  (crates/parity/texts/mk.txt) wait for eSpeak NG's dictionary.
  - [ ] Loanwords stressed elsewhere than the third syllable from the end
    (литература, -ист, -изам) need a list of their own; enclitics after an
    imperative (дај ми го) keep the verb's stress, as the rule gives.
- [x] Somali: words from the official spelling (c ʕ, x ħ, dh ɖ, kh x, sh ʃ,
  j dʒ, q, the apostrophe ʔ), doubled vowels long and doubled consonants
  geminate, b, d, g and dh softened between vowels (β ð ɣ ɽ); the stress on
  the second-last mora (the last syllable when its vowel is long), since the
  spelling does not show the tone; iyo and the particles unstressed; numbers
  with iyo (shan iyo labaatan, kun iyo sagaal boqol), ordinals in -aad, dates,
  shillings, percentages (boqolkiiba before the number), abbreviations.
  eSpeak NG has no Somali voice: jespersen's own sentences only, and its notation
  writes ħ as h and ʕ as ʔ (decision 48).
- [x] Oromo: words from the Qubee spelling (the ejectives c tʃʼ, ph pʼ, q kʼ,
  x tʼ and ts tsʼ, the implosive dh ɗ, ch, ny, sh, zh and the apostrophe ʔ),
  doubled vowels long, doubled consonants geminate and a digraph made
  geminate by doubling its first letter (qopphaa'uu); the stress on the last
  syllable but one, since the spelling does not show the pitch accent; fi and
  the particles unstressed; numbers joined with fi (dhibba tokko fi digdamii
  shan), ordinals in -ffaa, dates, birr, percentages (dhibbantaa before the
  number), abbreviations. No oracle (eSpeak NG's om dictionary is not in
  manuzio's helper): jespersen's own sentences only, and its notation writes
  the ejectives as plain sounds and ɗ as d (decision 48).
- [x] Bantu languages, read with a stress on the last syllable but one as a
  convention (their tones are not written, and Kokoro cannot say them);
  jespersen's own sentences only, no oracle:
  - [x] Kinyarwanda: words from the spelling (c tʃ, cy c, j ʒ, jy ɟ, sh ʃ, shy
    ʃc, ny ɲ, pf, ts; b as β but after m; r as ɾ), a consonant before w said
    with a velar (Rwanda ɾɡwaːnda, umwana umŋwana, twese tkwese), a doubled
    vowel long and a vowel long before a nasal and a consonant (inka iːŋka);
    elided words said with the next (n'umwana); the connectives, ku, mu and i
    unstressed; numbers with their agreement (magana abiri, ibihumbi bibiri),
    dates, francs, percentages (ku ijana) and abbreviations.
  - [x] Kirundi: as Kinyarwanda (jespersen_lang_rn started from it), without
    cy, jy and shy; ca and vya among the connectives; its own numbers (indwi,
    icenda, mirongo ibiri, amajana), months (Nzero…Kigarama) and francs
    (Fbu).
  - [x] Luganda: words from the spelling (c and ky c, j and gy ɟ, ny ɲ, ŋ
    and ng' ŋ, r ɾ), doubled vowels long and doubled consonants long, split
    between syllables (ekkubo ekˈkubo, omusajja omuˈsaɟɟa), a vowel long
    before a nasal and a consonant (Luganda luˈɡaːnda); elided words said
    with the next; the connectives, ne, nga, e, ku and mu unstressed; numbers
    (amakumi abiri mu emu, kikumi, lukaaga, enkumi bbiri), dates, shillings,
    percentages (ku kikumi) and abbreviations.
  - [x] Lingala: words from the spelling, its seven vowels (ɛ and ɔ, and è
    and ò taken for them) and its tone marks set aside (mbóka ˈmboka), ny ɲ,
    sh ʃ, n as ŋ before k and g, a doubled vowel long; na, ya, mpe, mpo, o
    and te unstressed; numbers (zomi na mɔkɔ, ntuku mibale, nkama, nkoto),
    dates with the months counted (sanza ya yambo), francs and
    abbreviations.
  - [x] Shona: words from the spelling, b and d implosive but plain after a
    nasal and in bv and dz, bh and dh breathy, mh and nh breathy nasals, v
    as ʋ and vh as v, h as ɦ, the whistled sv, zv, tsv and dzv (s͎ and z͎ in
    plain IPA, sw and zw in eSpeak NG's notation), ch, sh, zh, ny, ng'; the
    stress on the last syllable but one, its most prominent; the
    connectives and locatives unstressed; numbers joined with ne (gumi
    nerimwe, zana nemakumi maviri neshanu), dates, dollars, percentages
    (muzana) and abbreviations.
- [ ] Cushitic: Somali, Oromo.
- [x] West African, from their spelling (decision 48: no eSpeak NG voice,
  no tone said, each word marked on its last syllable but one for Kokoro):
  - [x] Yoruba: ẹ, ọ and ṣ, gb ɡ͡b and p k͡p, j ɟ, a vowel before an n that no
    vowel follows nasal (wọn, ẹran), n and m syllabic beside no vowel (ń lọ,
    m̀bá); tone marks and dots read precomposed or combining
    (jespersen_spoken::marks) and not said; the pronouns and particles
    unstressed whatever marks they carry; numbers counted in twenties up to
    199 (ọ̀kanlélógún, àrùndínlógún), the hundreds by name (igba, irinwó) and
    thousands with their count after them (ẹgbẹ̀rún méjì), dates with the
    Yoruba months, naira and abbreviations. jespersen's own sentences only
    (crates/parity/texts/yo.txt).
  - [x] Igbo: ị, ọ and ụ, ṅ as ŋ, the digraphs ch, gb, gh, gw, kp, kw, nw, ny
    and sh, h as ɦ, m, n and ṅ syllabic before no vowel (ndị, mmiri, nke,
    n'ụlọ); tone marks read and not said; na, nke, ka, bụ and the short
    pronouns unstressed; numbers in tens as the standard language counts
    them (iri abụọ na ise, narị, puku, nde; not the older twenties, ọgụ),
    ordinals with nke (nke mbụ), dates with the numbered months (n'ọnwa
    ise), naira and abbreviations. jespersen's own sentences only
    (crates/parity/texts/ig.txt).
  - [x] Hausa: ɓ and ɗ, ƙ and ts as ejectives, ʼy or ƴ as ʔʲ, c tʃ, f ɸ, sh,
    the diphthongs ai and au, gw, kw, ƙw, gy, ky, ƙy and fy, an apostrophe
    between vowels as ʔ (saʼa) and a doubled consonant long (dalla); tone and
    length marks, when a text writes them, read and not said; the linkers,
    copulas and short pronouns unstressed; numbers with da (dubu biyu da
    ashirin da huɗu), ordinals with na (na farko), dates with watan, naira
    and CFA francs, abbreviations (Alh., Mal.). jespersen's own sentences only
    (crates/parity/texts/ha.txt).
    - [ ] A word written with a plain apostrophe at its start ('ya'ya) loses
      it to the tokeniser, which keeps ʼ (U+02BC) and ƴ in the word.
  - [x] Ewe: ɛ and ɔ, nasal vowels with a tilde or before an n that no vowel
    follows, ɖ, ƒ ɸ, ɣ, h ɦ, ŋ, ʋ β, x, dz, gb, kp, ny and ts, a consonant
    before l or r in one syllable (kple), m, n and ŋ syllabic beside no vowel
    (ŋkeke); tone marks read and not said; le, na, kple, la, ƒe and the
    short pronouns unstressed; numbers with wui- and bla- (wuieve, blaeve vɔ
    ɖeka), alafa and akpe, ordinals with -lia (gbãtɔ, evelia), dates with
    the Ewe months, cedis, CFA francs and abbreviations; zero said as Ghana
    says it. jespersen's own sentences only (crates/parity/texts/ee.txt).
  - [x] Akan, in its Twi spelling: ɛ and ɔ, ky tɕ, gy dʑ, hy ɕ, tw, dw and hw
    rounded, kw, nw and ny, r ɾ, n as ŋ before k and g, and m and n syllabic
    at the start of a word or after a consonant when no vowel follows (mpɔ,
    nnipa, nkɔ) but closing the syllable after a vowel (ɔman); the
    conjunctions, no, wɔ and the short pronouns unstressed; numbers (dubaako,
    aduonu baako, ɔha ne, mpem abien), ordinals with a ɛtɔ so, dates with the
    Akan months, cedis and abbreviations. jespersen's own sentences only
    (crates/parity/texts/ak.txt).
- Not now: Ukrainian and Bulgarian (stress, as Russian), Japanese (a kanji
  dictionary).
