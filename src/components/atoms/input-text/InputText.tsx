import { type FC } from 'react'
import styles from './InputText.module.scss'

interface Props {
  value?: string
  length: number
  onChange: (value: string) => void
}

export const InputText: FC<Props> = (props) => {
  return (
    <input
      className={styles.component}
      type="text"
      value={props.value}
      maxLength={props.length}
      style={{ width: `${props.length * 1.3}rem` }}
      onChange={(e) => {
        props.onChange(e.target.value)
      }}
    />
  )
}
