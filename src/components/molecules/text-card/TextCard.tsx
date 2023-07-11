import { type FC } from 'react'
import styles from './TextCard.module.scss'

interface Props {
  label: string // todo: validation
  text: string // todo: validation
  selected: boolean
  onClick: () => void
}

export const TextCard: FC<Props> = (props) => {
  return (
    <div className={[styles.component, props.selected ? styles.selected : ''].join(' ')} onClick={props.onClick}>
      <span className={styles.label}>{props.label}</span>
      <span className={styles.text}>{props.text}</span>
    </div>
  )
}
