import { type FC } from 'react'
import styles from './RadioText.module.scss'

interface Props {
  name: string
  currentValue: string
  value: string
  displayValue: string
  disabled?: boolean
  onChange: (value: string) => void
}

export const RadioText: FC<Props> = (props) => {
  return (
    <div>
      <input
        className={styles.radio}
        type="radio"
        id={`${props.name}-${props.value}`}
        name={props.name}
        value={props.value}
        checked={props.currentValue === props.value}
        disabled={props.disabled}
        onChange={() => {
          props.onChange(props.value)
        }}
      />
      <label className={props.disabled ?? false ? styles.disabled : ''} htmlFor={`${props.name}-${props.value}`}>
        {props.displayValue}
      </label>
    </div>
  )
}
