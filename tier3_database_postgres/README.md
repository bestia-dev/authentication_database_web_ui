# tier3_database_postgres

First rule of SQL: keep it as simple as possible. Use a limited set of data-types, objects and rules. Keep the syntax as uniform as possible.
Sql has a terrifyingly incoherent syntax and the same thing can be written in so many ways, that is impossible to analyze after that.
Things can be written in so complicated ways that is impossible to maintain later. This is a big no-no. 
Foreign-keys must be one column only to keep our sanity. All modification of data must be done with sql functions. Because one day a different module will need the same functionality and the only way is to have it on the database level.


TODO: I need a migration utility.  
After I update database objects in development, I need to be able to repeat this changes as a migration in the production database.  
There are different objects that need different approach.

It is short-sighted to treat all the database as a monolith. It is an easy way out, because we can have a strict sort order of changes.
But usually real life is more complicated and we have different modules in the database that are developed by different teams and are loosely
coupled together. Inside one module things are very interconnected, but between different modules as little as possible. It is wise to have a strict hierarchy where higher level modules can call lower level, but not contrary.

Table schema: Check if the schema is equal and modify only the minimum.
    Adding columns or widening columns must be non-breaking for all other code. Other code must be written that way, that these changes will not break it.

View: postgres does not store the original source code.
Function: a part of the definition is stored, but not whole.

Data: inserting, updating and deleting data is very much dependent on the state of other objects.

I would like to code this migration tool entirely inside the database. 
Outside the database I need just a tool to import/export data from files that are Version-controlled with Git, to the database.

One table can have foreign keys to another table. The later must be installed first.


-- create table but column by column, so I can extend it later with new columns and constraints