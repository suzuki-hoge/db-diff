import React, { type FC } from 'react'
import styles from './NavExpand.module.scss'
import { IconExpand } from '../../atoms/icon-expand/IconExpand'

interface Props {
  isExpanded: boolean
  setIsExpanded: (isExpanded: boolean) => void
}

export const NavExpand: FC<Props> = (props) => {
  return (
    <div className={styles.component}>
      <span>expand</span>
      <IconExpand
        variant={'medium'}
        expanded={props.isExpanded}
        onClick={() => {
          props.setIsExpanded(!props.isExpanded)
        }}
      />
    </div>
  )
}
