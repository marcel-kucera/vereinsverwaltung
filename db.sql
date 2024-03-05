drop table if exists member;
drop table if exists payment;
drop table if exists memberfile;

create table member(
  id INTEGER PRIMARY KEY NOT NULL,
  firstname text NOT NULL,
  lastname text NOT NULL,
  email text not null,
  plz text not null,
  city text not null,
  street text not null,
  housenumber text not null,
  iban text not null,
  bic text not null,
  sepa boolean not null,
  note text,
  joindate int not null
);

create table payment(
  id INTEGER PRIMARY KEY NOT NULL,
  year int not null,
  memberid int not null,
  FOREIGN KEY (memberid) REFERENCES member(id) ON DELETE cascade,
  UNIQUE(memberid,year)
);

create table memberfile(
  id INTEGER PRIMARY KEY NOT NULL,
  memberid int not null,
  name text not null,
  file BLOB not null,
  FOREIGN KEY (memberid) REFERENCES member(id) ON DELETE cascade
);

create table user(
  id INTEGER PRIMARY KEY NOT NULL,
  name text NOT NULL,
  password text NOT NULL,
  UNIQUE(name)
);
