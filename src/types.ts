import { v4 } from 'uuid'

export const createProjectId: () => string = () => v4()

export interface Project {
  projectId: string
  name: string
  rdbms: string
  user: string
  password: string
  host: string
  port: string
  schema: string
}

export interface SnapshotSummary {
  snapshotId: string
  snapshotName: string
  createAt: string
}

type PrimaryValue = string
type ColName = string
type RowDiff = Record<
  PrimaryValue,
  Record<
    ColName,
    { status: 'stay' | 'added' | 'deleted' | 'none'; value: string }
  >
>

export interface TableDiff {
  tableName: string
  primaryValues: PrimaryValue[]
  primaryColName: ColName
  colNames: ColName[]
  rowDiffs1: RowDiff
  rowDiffs2: RowDiff
}
