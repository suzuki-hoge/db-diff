import React, { type FC, Fragment } from 'react'
import styles from './Color.module.scss'

const colors = {
  stay: styles.stay,
  added: styles.added,
  deleted: styles.deleted,
  none: styles.none,
}

interface Props {
  primaryValue: string
  colNames: string[]
  rowDiff?: Record<string, { status: 'stay' | 'added' | 'deleted' | 'none'; value: string }>
  n: number
  noDiffColNames: string[]
  isShowNoDiffCol: boolean
}

export const ColsRow: FC<Props> = (props) => {
  return (
    <tr>
      {props.n === 1 && (
        <td rowSpan={2} align={'left'}>
          {props.primaryValue}
        </td>
      )}
      {props.colNames.map((colName, i) => {
        if (props.rowDiff !== undefined) {
          if (colName in props.rowDiff) {
            if (props.isShowNoDiffCol || !props.noDiffColNames.includes(colName)) {
              const nullStyle = props.rowDiff[colName].value === '<null>' ? styles.null : ''
              return (
                <td key={i} className={[colors[props.rowDiff[colName].status], nullStyle].join(' ')} align={'left'}>
                  {props.rowDiff[colName].value}
                </td>
              )
            } else {
              return <Fragment key={i} />
            }
          } else {
            return <td key={i} className={colors.none} align={'left'}></td>
          }
        } else {
          return <td key={i} className={colors.none} align={'left'}></td>
        }
      })}
    </tr>
  )
}
