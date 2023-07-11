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

interface TRProps {
  primaryValue: string
  colNames: string[]
  rowDiff?: Record<string, { status: 'stay' | 'added' | 'deleted' | 'none'; value: string }>
  n: number
}

const TR: FC<TRProps> = (props) => {
  return (
    <tr>
      {props.n === 1 && <td rowSpan={2}>{props.primaryValue}</td>}
      {props.colNames.map((colName, i) =>
        props.rowDiff != null ? (
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

export const DiffContent: FC<Props> = (props) => {
  return (
    <div className={styles.component}>
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
          {props.tableDiff.primaryValues.map((primaryValue, i) => (
            <Fragment key={i}>
              <TR
                key={`${i}-1`}
                primaryValue={primaryValue}
                colNames={props.tableDiff.colNames}
                rowDiff={props.tableDiff.rowDiffs1[primaryValue]}
                n={1}
              />
              <TR
                key={`${i}-2`}
                primaryValue={primaryValue}
                colNames={props.tableDiff.colNames}
                rowDiff={props.tableDiff.rowDiffs2[primaryValue]}
                n={2}
              />
            </Fragment>
          ))}
        </tbody>
      </table>
    </div>
  )
}
