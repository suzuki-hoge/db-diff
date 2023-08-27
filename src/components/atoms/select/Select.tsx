import { type FC } from 'react'
import styles from './Select.module.scss'

interface Props {
  defaultValue: string
  values: string[]
  disabled?: boolean
  onChange: (v: string) => void
}

export const Select: FC<Props> = (props) => {
  const len = Math.max(...props.values.map((value) => value.length))

  return (
    <div className={styles.component} style={{ width: `${len * 0.7}rem` }}>
      <select
        disabled={props.disabled}
        defaultValue={props.defaultValue}
        style={{ width: `${len * 0.7}rem` }}
        onChange={(e) => {
          props.onChange(e.target.value)
        }}
      >
        {props.values.map((value) => (
          <option key={value}>{value}</option>
        ))}
      </select>
    </div>
  )
}
