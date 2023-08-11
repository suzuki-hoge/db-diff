import React, { type FC, Fragment, useEffect, useRef, useState } from 'react'
import { type TableDiff } from '../../../types'
import styles from './DiffContent.module.scss'
import { Resizer } from './Resizer'
import { IconExpand } from '../../atoms/icon-expand/IconExpand'
import { IconHide } from '../../atoms/icon-hide/IconHide'

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
  noDiffColNames: string[]
  isShowNoDiffCol: boolean
}

const ColsRow: FC<ColsRowProps> = (props) => {
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
          <td className={[styles.primaryOnly, colors.deleted].join(' ')} align={'left'}>
            {props.primaryValue}
          </td>
        </tr>
      )}
      {props.rowDiff2 && (
        <tr>
          <td className={[styles.primaryOnly, colors.added].join(' ')} align={'left'}>
            {props.primaryValue}
          </td>
        </tr>
      )}
    </>
  )
}

export const DiffContent: FC<Props> = (props) => {
  const table = useRef<HTMLTableElement>(null)
  const [height, setHeight] = useState(0)

  useEffect(() => {
    if (table.current !== null) {
      setHeight(table.current.offsetHeight - 2) // table border
    } else {
      setHeight(0)
    }
  }, [table])

  const [expand, setExpand] = useState(false)

  const pxs = calcPxs(props.tableDiff)

  const [isShowNoDiffCol, setIsShowNoDiffCol] = useState(true)

  return (
    <div id={props.tableDiff.tableName} className={styles.component}>
      <div className={styles.header}>
        <span className={styles.label}>{props.tableDiff.tableName}</span>
        <div>
          {pxs.ellipsized && (
            <IconExpand
              variant={'medium'}
              expanded={expand}
              onClick={() => {
                setExpand(!expand)
              }}
            />
          )}
          {props.tableDiff.noDiffColNames.length !== 0 && (
            <IconHide
              variant={'medium'}
              hide={!isShowNoDiffCol}
              onClick={() => {
                setIsShowNoDiffCol(!isShowNoDiffCol)
              }}
            />
          )}
        </div>
      </div>
      <div className={styles.body}>
        <div style={{ width: `${pxs.sum}px` }}>
          <table ref={table}>
            <thead>
              <tr>
                <th align={'left'} style={{ width: `${pxs.primary}px` }}>
                  {props.tableDiff.primaryColName}
                </th>
                {props.tableDiff.colNames.map((colName, i) => {
                  const cellId = `${props.tableDiff.tableName}-${i}`
                  return (
                    (isShowNoDiffCol || !props.tableDiff.noDiffColNames.includes(colName)) && (
                      <th
                        key={i}
                        id={cellId}
                        align={'left'}
                        style={
                          expand
                            ? { width: `${pxs.expand[i]}px`, maxWidth: `${pxs.expand[i]}px` }
                            : { width: `${pxs.ellipsis[i]}px`, maxWidth: `${pxs.expand[i]}px` }
                        }
                      >
                        {colName}
                        <Resizer key={i} cellId={cellId} resizerId={`resizer-${i}`} height={height} />
                      </th>
                    )
                  )
                })}
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
                      noDiffColNames={props.tableDiff.noDiffColNames}
                      isShowNoDiffCol={isShowNoDiffCol}
                    />
                    <ColsRow
                      key={`${i}-2`}
                      primaryValue={primaryValue}
                      colNames={props.tableDiff.colNames}
                      rowDiff={props.tableDiff.rowDiffs2[primaryValue]}
                      n={2}
                      noDiffColNames={props.tableDiff.noDiffColNames}
                      isShowNoDiffCol={isShowNoDiffCol}
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
      </div>
    </div>
  )
}

const calcPxs: (tableDiff: TableDiff) => { primary: number; expand: number[]; ellipsis: number[]; sum: number; ellipsized: boolean } = (
  tableDiff: TableDiff
) => {
  const rate = 9.61
  const upper = 21

  const primary = [...tableDiff.primaryValues.map((v) => v.length), tableDiff.primaryColName.length].reduce((a, b) => (a > b ? a : b)) * rate
  const rows = [...Object.values(tableDiff.rowDiffs1), ...Object.values(tableDiff.rowDiffs2)]
  const expand = tableDiff.colNames.map((colName) => {
    return [colName.length, ...rows.map((row) => row[colName]?.value.length ?? 0)].reduce((a, b) => (a > b ? a : b)) * rate
  })
  const ellipsis = tableDiff.colNames.map((colName) => {
    const fullLen = rows.map((row) => row[colName]?.value.length ?? 0).reduce((a, b) => (a > b ? a : b))
    return (fullLen < colName.length ? colName.length : Math.min(upper, fullLen)) * rate
  })
  const cols = [primary, ...ellipsis].length
  const sum =
    primary + // primary
    expand.reduce((a, b) => a + b, 0) + // cols
    cols * 16 * 2 + // paddings
    (cols - 1) + // cell borders
    2 // table border

  return { primary, expand, ellipsis, sum, ellipsized: ellipsis.toString() !== expand.toString() }
}
