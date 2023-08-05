import { type FC, useState } from 'react'
import styles from './LabeledColorTagInput.module.scss'
import { ColorTag } from '../../atoms/color-tag/ColorTag'
import { CirclePicker } from 'react-color'

interface Props {
  label: string
  value: string
  maxLength: number
  onChange: (value: string) => void
  color: string
  setColor: (color: string) => void
  autoFocus?: true
  errors?: string[]
}

export const LabeledColorTagInput: FC<Props> = (props) => {
  const [isPicking, setIsPicking] = useState(false)
  const [active, setActive] = useState<string>()

  return (
    <div className={styles.component}>
      <span>{props.label}</span>
      <div className={[styles.input, active, (props.errors ?? []).length !== 0 ? styles.error_div : ''].join(' ')}>
        <ColorTag
          color={props.color}
          onClick={() => {
            setIsPicking(true)
          }}
        />
        <input
          className={styles.text}
          type="text"
          value={props.value}
          maxLength={props.maxLength}
          onChange={(e) => {
            props.onChange(e.target.value)
          }}
          onFocus={() => {
            setActive(styles.active)
          }}
          onBlur={() => {
            setActive('')
          }}
          autoFocus={props.autoFocus}
        />
      </div>
      {isPicking && (
        <CirclePicker
          className={styles.picker}
          colors={colors}
          width={'184px'}
          circleSize={16}
          circleSpacing={0}
          onChange={(color) => {
            props.setColor(color.hex)
            setIsPicking(false)
          }}
        />
      )}
      {(props.errors ?? []).map((error) => (
        <p key={error} className={styles.error}>
          {error}
        </p>
      ))}
    </div>
  )
}

const colors: string[] = [
  '#b60205',
  '#d93f0b',
  '#fbca04',
  '#0e8a16',
  '#006b75',
  '#1d76db',
  '#0052cc',
  '#5319e7',
  '#e99695',
  '#f9d0c4',
  '#fef2c0',
  '#c2e0c6',
  '#bfdadc',
  '#c5def5',
  '#bfd4f2',
  '#d4c5f9',
]
