-- tier3_database_postgres/test_data_and functions/fn_test2_multi_column_foreign_key.sql

-- For this simple database application I limit the foreign key only to 1 column. For simplicity.

create table test2_a(
  id_a serial,
  ref_a1 varchar(3),
  ref_a2 varchar(3),
  constraint test2_a_pkey primary key (id_a)
)
-- add unique constraint

alter table test2_a add constraint test2_a_uniq unique (ref_a1,ref_a2)

----------------------------------------------------------------------------
create table test2_b(
  id_b serial,
  ref_b1 varchar(3),
  ref_b2 varchar(3),
  constraint test2_b_pkey primary key (id_b)
)
-- add foreign key multi column
alter table test2_b add constraint test2_b_foreign foreign key (ref_b1,ref_b2) references test2_a (ref_a1,ref_a2)

----------------------------------------------------------------------------
-- check to avoid this type of multiple columns for my simple database example

select * from a3_check_multi_column_unique ;

select * from a3_check_multi_column_foreign ;

----------------------------------------------------------------------------

drop table test2_b;
drop table test2_a;