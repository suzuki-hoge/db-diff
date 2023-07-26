import { type FC } from 'react'
import styles from './ColorTag.module.scss'

interface Props {
  color: string
  onClick?: () => void
}

export const ColorTag: FC<Props> = (props) => {
  return <span className={styles.component} style={{ backgroundColor: props.color }} onClick={props.onClick}></span>
}
