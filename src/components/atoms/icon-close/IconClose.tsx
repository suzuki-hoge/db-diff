import React, { type FC } from 'react'
import styles from './IconClose.module.scss'
import { IoCloseCircleOutline } from 'react-icons/io5'

interface Props {
  variant: 'small' | 'medium' | 'large'
  onClick: () => void
}

export const IconClose: FC<Props> = (props) => {
  return <IoCloseCircleOutline className={styles[props.variant]} onClick={props.onClick} />
}
