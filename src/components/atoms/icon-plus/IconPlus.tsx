import { type FC } from 'react'
import styles from './IconPlus.module.scss'
import { BsPlusCircle } from 'react-icons/bs'

interface Props {
  variant: 'small' | 'medium' | 'large'
  onClick: () => void
}

export const IconPlus: FC<Props> = (props) => {
  return <BsPlusCircle className={styles[props.variant]} onClick={props.onClick} />
}
