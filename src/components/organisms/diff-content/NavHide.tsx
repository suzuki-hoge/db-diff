import React, { type FC } from 'react'
import styles from './NavHide.module.scss'
import { IconHide } from '../../atoms/icon-hide/IconHide'

interface Props {
  isShowNoDiffCol: boolean
  setIsShowNoDiffCol: (isShowNoDiffCol: boolean) => void
}

export const NavHide: FC<Props> = (props) => {
  return (
    <div className={styles.component}>
      <span>hide white cols</span>
      <IconHide
        variant={'medium'}
        hide={!props.isShowNoDiffCol}
        onClick={() => {
          props.setIsShowNoDiffCol(!props.isShowNoDiffCol)
        }}
      />
    </div>
  )
}
