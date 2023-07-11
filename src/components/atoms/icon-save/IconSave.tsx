import React, { type FC } from 'react'
import styles from './IconSave.module.scss'
import { IoSaveOutline } from 'react-icons/io5'

interface Props {
  variant: 'small' | 'medium' | 'large'
  onClick: () => void
}

export const IconSave: FC<Props> = (props) => {
  return <IoSaveOutline className={styles[props.variant]} onClick={props.onClick} />
}
