create table llm_context (
    uid bigint unsigned primary key,
    last_updated timestamp default current_timestamp not null,
    context json not null
);
