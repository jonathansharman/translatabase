begin;

create table if not exists lang (
	id integer primary key,
	name text not null unique
);

create table if not exists class (
	id integer primary key,
	lang_id integer not null,
	name text not null,
	unique (lang_id, name),
	foreign key (lang_id) references lang (id)
);

create table if not exists lemma (
	id integer primary key,
	lang_id integer not null,
	name text not null,
	unique (lang_id, name),
	foreign key (lang_id) references lang (id)
);

create table if not exists def (
	id integer primary key,
	lemma_id integer not null,
	sort_order integer not null,
	class_id integer not null,
	translit text not null,
	content text not null,
	unique (lemma_id, sort_order),
	foreign key (lemma_id) references lemma (id),
	foreign key (class_id) references class (id)
);

create table if not exists trans (
	id integer primary key,
	lemma_id_1 integer not null,
	lemma_id_2 integer not null,
	unique (lemma_id_1, lemma_id_2),
	foreign key (lemma_id_1) references lemma (id),
	foreign key (lemma_id_2) references lemma (id)
);

create table if not exists trans_note (
	id integer primary key,
	trans_id integer not null,
	sort_order integer not null,
	content text not null,
	unique (trans_id, sort_order),
	foreign key (trans_id) references trans (id)
);

create table if not exists cf (
	id integer primary key,
	lemma_id_1 integer not null,
	lemma_id_2 integer not null,
	unique (lemma_id_1, lemma_id_2),
	foreign key (lemma_id_1) references lemma (id),
	foreign key (lemma_id_2) references lemma (id)
);

commit;
