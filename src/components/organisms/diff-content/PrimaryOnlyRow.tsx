import React, { type FC } from 'react'
import styles from './Color.module.scss'

interface Props {
  primaryValue: string
  rowDiff1: boolean
  rowDiff2: boolean
}

export const PrimaryOnlyRow: FC<Props> = (props) => {
  return (
    <>
      {props.rowDiff1 && (
        <tr>
          <td className={styles.deleted} align={'left'}>
            {props.primaryValue}
          </td>
        </tr>
      )}
      {props.rowDiff2 && (
        <tr>
          <td className={styles.added} align={'left'}>
            {props.primaryValue}
          </td>
        </tr>
      )}
    </>
  )
}
