import { type FC } from 'react'
import styles from './IconHelp.module.scss'
import { BsQuestionCircle } from 'react-icons/bs'

interface Props {
  variant: 'small' | 'medium' | 'large'
  onClick: () => void
}

export const IconHelp: FC<Props> = (props) => {
  return <BsQuestionCircle className={styles[props.variant]} onClick={props.onClick} />
}
