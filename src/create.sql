begin;

create table if not exists langs (
	name text primary key
) without rowid;

create table if not exists classes (
	lang text,
	name text,
	primary key (lang, name),
	foreign key (lang) references langs (name)
) without rowid;

create table if not exists lemmas (
	lang text,
	name text,
	primary key (lang, name),
	foreign key (lang) references langs (name)
) without rowid;

create table if not exists defs (
	lang text,
	lemma text,
	rank integer,
	class text not null,
	translit text not null,
	def text not null,
	primary key (lang, lemma, rank),
	foreign key (lang) references langs (name),
	foreign key (lang, lemma) references lemmas (lang, name),
	foreign key (lang, class) references classes (lang, class)
) without rowid;

create table if not exists trans (
	lang1 text,
	lemma1 text,
	lang2 text,
	lemma2 text,
	primary key (lang1, lemma1, lang2, lemma2),
	foreign key (lang1) references langs (name),
	foreign key (lang2) references langs (name),
	foreign key (lang1, lemma1) references lemmas (lang, name),
	foreign key (lang2, lemma2) references lemmas (lang, name)
) without rowid;

create table if not exists trans_note (
	lang1 text,
	lemma1 text,
	lang2 text,
	lemma2 text,
	rank integer,
	note text not null,
	primary key (lang1, lemma1, lang2, lemma2, rank),
	foreign key (lang1) references langs (name),
	foreign key (lang2) references langs (name),
	foreign key (lang1, lemma1) references lemmas (lang, name),
	foreign key (lang2, lemma2) references lemmas (lang, name),
	foreign key (lang1, lemma1, lang2, lemma2) references trans (lang1, lemma1, lang2, lemma2)
) without rowid;

create table if not exists cfs (
	lang text,
	lemma1 text,
	lemma2 text,
	primary key (lang, lemma1, lemma2),
	foreign key (lang) references langs (name),
	foreign key (lang, lemma1) references lemmas (lang, name),
	foreign key (lang, lemma2) references lemmas (lang, name)
) without rowid;

commit;
