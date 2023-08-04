import React, { type FC, Fragment, type MouseEventHandler, useEffect, useRef, useState } from 'react'
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
      {props.n === 1 && (
        <td rowSpan={2} align={'left'}>
          {props.primaryValue}
        </td>
      )}
      {props.colNames.map((colName, i) =>
        props.rowDiff !== undefined ? (
          colName in props.rowDiff ? (
            <td key={i} className={colors[props.rowDiff[colName].status]} align={'left'}>
              {props.rowDiff[colName].value}
            </td>
          ) : (
            <td key={i} className={colors.none} align={'left'}></td>
          )
        ) : (
          <td key={i} className={colors.none} align={'left'}></td>
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

const Resizer: FC<{ cellId: string; resizerId: string; height: number }> = (props) => {
  let x = 0
  let w = 0

  const downHandler: MouseEventHandler = (e: React.MouseEvent<HTMLDivElement>) => {
    x = e.clientX

    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    w = parseInt(window.getComputedStyle(document.getElementById(props.cellId)!).width, 10)

    document.addEventListener('mousemove', moveHandler)
    document.addEventListener('mouseup', upHandler)

    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    document.getElementById(props.resizerId)!.classList.add('resizing')
  }

  const moveHandler: (event: MouseEvent) => void = (e: MouseEvent) => {
    const dx = e.clientX - x
    console.log(x, w, dx, w + dx)

    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    document.getElementById(props.cellId)!.style.width = `${w + dx}px`
  }

  const upHandler: (event: MouseEvent) => void = () => {
    document.removeEventListener('mousemove', moveHandler)
    document.removeEventListener('mouseup', upHandler)

    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    document.getElementById(props.resizerId)!.classList.remove('resizing')
  }

  return <div id={props.resizerId} className={styles.resizer} style={{ height: `${props.height}px` }} onMouseDown={downHandler} />
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

  const rows = [...Object.values(props.tableDiff.rowDiffs1), ...Object.values(props.tableDiff.rowDiffs2)]
  const expandLens = props.tableDiff.colNames.map((colName) => {
    return [colName.length, ...rows.map((row) => row[colName].value.length)].reduce((a, b) => (a > b ? a : b))
  })
  const ellipsisLens = props.tableDiff.colNames.map((colName) => {
    const fullLen = rows.map((row) => row[colName].value.length).reduce((a, b) => (a > b ? a : b))
    return fullLen < colName.length ? colName.length : Math.min(22, fullLen)
  })
  const primaryLen = [...props.tableDiff.primaryValues.map((v) => v.length), props.tableDiff.primaryColName.length].reduce((a, b) => (a > b ? a : b))
  const cols = [primaryLen, ...ellipsisLens].length
  const sumPx =
    primaryLen * 10 + // primary
    expandLens.reduce((a, b) => a + b, 0) * 10 + // cols
    cols * 16 * 2 + // paddings
    (cols - 1) + // cell borders
    2 // table border

  return (
    <div id={props.tableDiff.tableName} className={styles.component} style={{ width: `${sumPx}px` }}>
      <span className={styles.label}>{props.tableDiff.tableName}</span>
      <button
        onClick={() => {
          setExpand(!expand)
        }}
      >
        f
      </button>
      <table ref={table}>
        <thead>
          <tr>
            <th align={'left'} style={{ width: `${primaryLen * 10}px` }}>
              {props.tableDiff.primaryColName}
            </th>
            {props.tableDiff.colNames.map((colName, i) => {
              const cellId = `${props.tableDiff.tableName}-${i}`
              return (
                <th
                  key={i}
                  id={cellId}
                  align={'left'}
                  style={
                    expand
                      ? { width: `${expandLens[i] * 10}px`, maxWidth: `${expandLens[i] * 10}px` }
                      : { width: `${ellipsisLens[i] * 10}px`, maxWidth: `${expandLens[i] * 10}px` }
                  }
                >
                  {colName}
                  <Resizer key={i} cellId={cellId} resizerId={`resizer-${i}`} height={height} />
                </th>
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
