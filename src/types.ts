import { v4 } from 'uuid'

export const createProjectId: () => string = () => v4()

export interface Project {
  projectId: string
  name: string
  color: string
  rdbms: string
  user: string
  password: string
  host: string
  port: string
  schema: string
}

export const createSnapshotId: () => string = () => v4()

export interface SnapshotSummary {
  snapshotId: string
  snapshotName: string
  createAt: string
}

export interface DumpConfig {
  tableName: string
  colNames: string[]
  value: DumpConfigValue
}
export type DumpConfigValue = 'limited' | 'ignore' | string

export interface SnapshotDiff {
  diffId: string
  snapshotId1: string
  snapshotId2: string
  tableDiffs: TableDiff[]
}

export type PrimaryValue = string
export type ColName = string

export interface ColDiff {
  status: 'stay' | 'added' | 'deleted' | 'none'
  value: string
}

export type RowDiff = Record<PrimaryValue, Record<ColName, ColDiff>>

export interface TableDiff {
  tableName: string
  primaryValues: PrimaryValue[]
  primaryColName: ColName
  colNames: ColName[]
  rowDiffs1: RowDiff
  rowDiffs2: RowDiff
}
