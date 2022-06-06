SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;


create table message
(
    id                uuid primary key            not null,
    user_id           uuid                        not null,
    is_reply          boolean                     not null,
    reply_to          uuid,
    tags              array  
    created_datetime  timestamp without time zone not null,
    modified_datetime timestamp without time zone not null
);  

create table content
(
    id                uuid primary key                               not null,
    message_id        uuid references message (id) on delete cascade not null,
    data              text                                           not null,
    created_datetime  timestamp without time zone                    not null,
    modified_datetime timestamp without time zone                    not null                                          
);

CREATE INDEX idx_message_created_datetime ON message(created_datetime);
CREATE INDEX idx_content_message_id ON content(message_id);

