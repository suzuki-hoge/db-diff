import React, { type FC, Fragment } from 'react'
import { type TableDiff } from '../../../types'
import styles from './DiffContent.module.scss'

interface Props {
  tableDiff: TableDiff
}

const colors = {
  stay: styles.stay,
  added: styles.added,
  deleted: styles.deleted,
  none: styles.none,
}

interface ColsRowProps {
  primaryValue: string
  colNames: string[]
  rowDiff?: Record<string, { status: 'stay' | 'added' | 'deleted' | 'none'; value: string }>
  n: number
}

const ColsRow: FC<ColsRowProps> = (props) => {
  return (
    <tr>
      {props.n === 1 && <td rowSpan={2}>{props.primaryValue}</td>}
      {props.colNames.map((colName, i) =>
        props.rowDiff !== undefined ? (
          colName in props.rowDiff ? (
            <td key={i} className={colors[props.rowDiff[colName].status]}>
              {props.rowDiff[colName].value}
            </td>
          ) : (
            <td key={i} className={colors.none}></td>
          )
        ) : (
          <td key={i} className={colors.none}></td>
        )
      )}
    </tr>
  )
}

interface PrimaryOnlyRowProps {
  primaryValue: string
  rowDiff1: boolean
  rowDiff2: boolean
}

const PrimaryOnlyRow: FC<PrimaryOnlyRowProps> = (props) => {
  return (
    <>
      {props.rowDiff1 && (
        <tr>
          <td className={[styles.primaryOnly, colors.deleted].join(' ')}>{props.primaryValue}</td>
        </tr>
      )}
      {props.rowDiff2 && (
        <tr>
          <td className={[styles.primaryOnly, colors.added].join(' ')}>{props.primaryValue}</td>
        </tr>
      )}
    </>
  )
}

export const DiffContent: FC<Props> = (props) => {
  return (
    <div id={props.tableDiff.tableName} className={styles.component}>
      <span className={styles.label}>{props.tableDiff.tableName}</span>
      <table>
        <thead>
          <tr>
            <th>{props.tableDiff.primaryColName}</th>
            {props.tableDiff.colNames.map((colName, i) => (
              <th key={i}>{colName}</th>
            ))}
          </tr>
        </thead>
        <tbody>
          {props.tableDiff.primaryValues.map((primaryValue, i) =>
            props.tableDiff.colNames.length !== 0 ? (
              <Fragment key={i}>
                <ColsRow
                  key={`${i}-1`}
                  primaryValue={primaryValue}
                  colNames={props.tableDiff.colNames}
                  rowDiff={props.tableDiff.rowDiffs1[primaryValue]}
                  n={1}
                />
                <ColsRow
                  key={`${i}-2`}
                  primaryValue={primaryValue}
                  colNames={props.tableDiff.colNames}
                  rowDiff={props.tableDiff.rowDiffs2[primaryValue]}
                  n={2}
                />
              </Fragment>
            ) : (
              <Fragment key={i}>
                <PrimaryOnlyRow
                  primaryValue={primaryValue}
                  rowDiff1={props.tableDiff.rowDiffs1[primaryValue] !== undefined}
                  rowDiff2={props.tableDiff.rowDiffs2[primaryValue] !== undefined}
                />
              </Fragment>
            )
          )}
        </tbody>
      </table>
    </div>
  )
}
