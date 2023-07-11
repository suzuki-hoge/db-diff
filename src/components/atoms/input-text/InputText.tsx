import { type ChangeEvent, type FC } from 'react'
import styles from './InputText.module.scss'

interface Props {
  value?: string
  length: number
  onInput: (e: ChangeEvent<HTMLInputElement>) => void
}

export const InputText: FC<Props> = (props) => {
  return (
    <input className={styles.component} type="text" value={props.value} size={props.length * 2} maxLength={props.length} onChange={props.onInput} />
  )
}
