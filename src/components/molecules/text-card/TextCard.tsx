import { type FC } from 'react'
import styles from './TextCard.module.scss'

interface Props {
  label: string
  text: string
  selected: boolean
  onClick: () => void
}

export const TextCard: FC<Props> = (props) => {
  return (
    <div className={['text_card', styles.component, props.selected ? styles.selected : ''].join(' ')} onClick={props.onClick}>
      <span className={styles.label}>{props.label}</span>
      <span className={styles.text}>{props.text}</span>
    </div>
  )
}
