# 主キー判定について

`Null` にできない重複のないカラムを主キーとして扱います

複合主キーも扱えます

主キーがないテーブルは抽出対象外となります

## MySQL ( 8.0 )

### パターン: 1 カラム

| col1     | constraint        | column_key | is_nullable | 主キー  |
|:---------|:------------------|:-----------|:------------|:-----|
|          | primary key(col1) | PRI        | NO          | col1 |
|          | unique(col1)      | UNI        | YES         | x    |
| not null | unique(col1)      | PRI        | NO          | col1 |

### パターン: 2 カラム ( 複合主キー )

| col1 | col2 | constraint              | column_key | is_nullable | 主キー       |
|:-----|:-----|:------------------------|:-----------|:------------|:----------|
|      |      | primary key(col1, col2) | PRI<br>PRI | NO<br>NO    | col1-col2 |

### パターン: 2 カラム ( 主キーと一意制約の混在 )

| col1 | col2     | constraint                        | column_key | is_nullable | 主キー  |
|:-----|:---------|:----------------------------------|:-----------|:------------|:-----|
|      |          | primary key(col1)<br>unique(col2) | PRI<br>UNI | NO<br>YES   | col1 |
|      | not null | primary key(col1)<br>unique(col2) | PRI<br>UNI | NO<br>NO    | col1 |

### パターン: 2 カラム ( 一意制約の混在 )

| col1     | col2     | constraint                   | column_key | is_nullable | 主キー  |
|:---------|:---------|:-----------------------------|:-----------|:------------|:-----|
|          |          | unique(col1)<br>unique(col2) | UNI<br>UNI | YES<br>YES  | x    |
| not null |          | unique(col1)<br>unique(col2) | PRI<br>UNI | NO<br>YES   | col1 |
| not null | not null | unique(col1)<br>unique(col2) | PRI<br>UNI | NO<br>NO    | col1 |

### パターン: 2 カラム ( 複合一意制約 )

| col1     | col2     | constraint         | column_key | is_nullable | 主キー       |
|:---------|:---------|:-------------------|:-----------|:------------|:----------|
|          |          | unique(col1, col2) | MUL        | YES<br>YES  | x         |
| not null |          | unique(col1, col2) | MUL        | NO<br>YES   | x         |
| not null | not null | unique(col1, col2) | PRI<br>PRI | NO<br>NO    | col1-col2 |

## MySQL ( 5.7 )

TBD

## MySQL ( 5.6 )

TBD

## PostgreSQL ( 15 )

TBD

## PostgreSQL ( 14 )

TBD

## PostgreSQL ( 13 )

TBD

## PostgreSQL ( 12 )

TBD
