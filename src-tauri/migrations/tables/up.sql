PRAGMA foreign_keys = ON;

create table projects
(
    project_id text not null,
    name       text not null,
    color      text not null,
    rdbms      text not null,
    user       text not null,
    password   text not null,
    host       text not null,
    port       text not null,
    `schema`   text not null,
    primary key (project_id)
);

create table snapshot_summaries
(
    snapshot_id   text not null,
    project_id    text not null,
    snapshot_name text not null,
    create_at     text not null,
    primary key (snapshot_id),
    foreign key (project_id) references projects (project_id) on delete cascade
);

create table table_snapshots
(
    snapshot_id text not null,
    table_name  text not null,
    data        text not null,
    primary key (snapshot_id, table_name),
    foreign key (snapshot_id) references snapshot_summaries (snapshot_id) on delete cascade
);

create table snapshot_diffs
(
    diff_id      text not null,
    snapshot_id1 text not null,
    snapshot_id2 text not null,
    data         text not null,
    primary key (diff_id),
    unique (snapshot_id1, snapshot_id2),
    foreign key (snapshot_id1) references snapshot_summaries (snapshot_id) on delete cascade,
    foreign key (snapshot_id2) references snapshot_summaries (snapshot_id) on delete cascade
);
