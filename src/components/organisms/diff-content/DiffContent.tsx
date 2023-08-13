import React, { type FC, Fragment, useEffect, useRef, useState } from 'react'
import { type PrimaryValue, type RowDiff, type TableDiff } from '../../../types'
import styles from './DiffContent.module.scss'
import { Resizer } from './Resizer'
import { ColsRow } from './ColsRow'
import { PrimaryOnlyRow } from './PrimaryOnlyRow'
import { NavPage } from './NavPage'
import { NavExpand } from './NavExpand'
import { NavHide } from './NavHide'

interface Props {
  tableDiff: TableDiff
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

  const [isExpanded, setIsExpanded] = useState(false)
  const [isShowNoDiffCol, setIsShowNoDiffCol] = useState(true)

  const len = props.tableDiff.primaryValues.length
  const perpage = 30

  const [s, setS] = useState(0)

  const tableDiff = pick(props.tableDiff, s, s + perpage)

  const pxs = calcPxs(tableDiff)

  const noDiffColNames = tableDiff.colNames.filter((colName) =>
    [...Object.values(tableDiff.rowDiffs1), ...Object.values(tableDiff.rowDiffs2)].every((row) => row[colName]?.status === 'stay')
  )

  return (
    <div id={tableDiff.tableName} className={styles.component}>
      <span className={styles.label}>{tableDiff.tableName}</span>
      <div className={styles.header}>
        <NavPage s={s} perpage={perpage} len={len} setS={setS} />
        {pxs.ellipsized && <NavExpand isExpanded={isExpanded} setIsExpanded={setIsExpanded} />}
        {noDiffColNames.length !== 0 && <NavHide isShowNoDiffCol={isShowNoDiffCol} setIsShowNoDiffCol={setIsShowNoDiffCol} />}
      </div>
      <div className={styles.body}>
        <div style={{ width: `${pxs.sum}px` }}>
          <table ref={table}>
            <thead>
              <tr>
                <th align={'left'} style={{ width: `${pxs.primary}px` }}>
                  {tableDiff.primaryColName}
                </th>
                {tableDiff.colNames.map((colName, i) => {
                  const cellId = `${tableDiff.tableName}-${i}`
                  return (
                    (isShowNoDiffCol || !noDiffColNames.includes(colName)) && (
                      <th
                        key={i}
                        id={cellId}
                        align={'left'}
                        style={
                          isExpanded
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
              {tableDiff.primaryValues.map((primaryValue, i) =>
                tableDiff.colNames.length !== 0 ? (
                  <Fragment key={i}>
                    <ColsRow
                      key={`${i}-1`}
                      primaryValue={primaryValue}
                      colNames={tableDiff.colNames}
                      rowDiff={tableDiff.rowDiffs1[primaryValue]}
                      n={1}
                      noDiffColNames={noDiffColNames}
                      isShowNoDiffCol={isShowNoDiffCol}
                    />
                    <ColsRow
                      key={`${i}-2`}
                      primaryValue={primaryValue}
                      colNames={tableDiff.colNames}
                      rowDiff={tableDiff.rowDiffs2[primaryValue]}
                      n={2}
                      noDiffColNames={noDiffColNames}
                      isShowNoDiffCol={isShowNoDiffCol}
                    />
                  </Fragment>
                ) : (
                  <Fragment key={i}>
                    <PrimaryOnlyRow
                      primaryValue={primaryValue}
                      rowDiff1={tableDiff.rowDiffs1[primaryValue] !== undefined}
                      rowDiff2={tableDiff.rowDiffs2[primaryValue] !== undefined}
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

const pick: (tableDiff: TableDiff, s: number, e: number) => TableDiff = (tableDiff: TableDiff, s: number, e: number) => {
  const primaryValues = tableDiff.primaryValues.slice(s, e)

  const record: (primaryValues: PrimaryValue[], rowDiff: RowDiff) => RowDiff = (primaryValues: PrimaryValue[], rowDiff: RowDiff) => {
    const filtered = primaryValues.map((primaryValue) => [primaryValue, rowDiff[primaryValue]])
    return Object.fromEntries(filtered.filter((x) => x[1]))
  }

  return {
    tableName: tableDiff.tableName,
    primaryValues,
    primaryColName: tableDiff.primaryColName,
    colNames: tableDiff.colNames,
    rowDiffs1: record(primaryValues, tableDiff.rowDiffs1),
    rowDiffs2: record(primaryValues, tableDiff.rowDiffs2),
  }
}

const calcPxs: (tableDiff: TableDiff) => { primary: number; expand: number[]; ellipsis: number[]; sum: number; ellipsized: boolean } = (
  tableDiff: TableDiff
) => {
  const rate = 8
  const upper = 21

  const isMultibyte = /[ -~]/
  const len: (s: string) => number = (s) =>
    s
      .split('')
      .map((c) => (c.match(isMultibyte) != null ? 1 : 2))
      .reduce((a, b) => a + b, 0)

  const primary = [...tableDiff.primaryValues.map((v) => len(v)), tableDiff.primaryColName.length].reduce((a, b) => (a > b ? a : b)) * rate
  const rows = [...Object.values(tableDiff.rowDiffs1), ...Object.values(tableDiff.rowDiffs2)]
  const expand = tableDiff.colNames.map((colName) => {
    return [len(colName), ...rows.map((row) => len(row[colName]?.value ?? ''))].reduce((a, b) => (a > b ? a : b)) * rate
  })
  const ellipsis = tableDiff.colNames.map((colName) => {
    const fullLen = rows.map((row) => len(row[colName]?.value ?? '')).reduce((a, b) => (a > b ? a : b))
    return (fullLen < len(colName) ? len(colName) : Math.min(upper, fullLen)) * rate
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
