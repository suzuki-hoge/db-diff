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

create table snapshot_results
(
    snapshot_id text not null,
    percent     integer,
    done        integer,
    total       integer,
    status      text not null,
    primary key (snapshot_id),
    foreign key (snapshot_id) references snapshot_summaries (snapshot_id) on delete cascade
);

create table dump_configs
(
    snapshot_id text not null,
    project_id  text not null,
    data        text not null,
    create_at   text not null,
    primary key (snapshot_id),
    foreign key (snapshot_id) references snapshot_summaries (snapshot_id) on delete cascade,
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

insert into projects (project_id, name, color, rdbms, user, password, host, port, `schema`)
values ('D07231B4-D5CC-4E25-AF01-2D5F9DB59980', 'Sample 1', '#c2e0c6', 'MySQL', 'user', 'password', '127.0.0.1',
        '13306',
        'sample1'),
       ('0BB817C5-5045-4668-8385-0E0B6625FE0D', 'Sample 2', '#c5def5', 'MySQL', 'user', 'password', '127.0.0.1',
        '23306',
        'sample2'),
       ('1AA6531E-F31C-46B8-98E4-19DA51DA23F0', 'Sample 3', '#f9d0c4', 'MySQL', 'user', 'password', '127.0.0.1',
        '33306',
        'sample3');
