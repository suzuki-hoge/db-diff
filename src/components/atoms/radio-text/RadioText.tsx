import { type FC } from 'react'
import styles from './RadioText.module.scss'

interface Props {
  name: string
  currentValue: string
  value: string
  onChange: (value: string) => void
}

export const RadioText: FC<Props> = (props) => {
  return (
    <div>
      <input
        className={styles.radio}
        type="radio"
        id={props.value}
        name={props.name}
        value={props.value}
        checked={props.currentValue === props.value}
        onChange={() => {
          props.onChange(props.value)
        }}
      />
      <label htmlFor={props.value}>{props.value}</label>
    </div>
  )
}
