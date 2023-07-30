import { type FC } from 'react'
import styles from './InputText.module.scss'

interface Props {
  value?: string
  maxLength: number
  onChange: (value: string) => void
  chars: 'all' | 'half' | 'number'
}

const regex = { all: /.*/g, half: /^[!-~]*$/g, number: /^[0-9]*$/g }

export const InputText: FC<Props> = (props) => {
  return (
    <input
      className={styles.component}
      type="text"
      value={props.value}
      maxLength={props.maxLength}
      onChange={(e) => {
        const value = e.target.value
        if (value.match(regex[props.chars]) != null) {
          props.onChange(value)
        }
      }}
    />
  )
}
