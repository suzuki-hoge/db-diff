import { type FC } from 'react'
import styles from './LabeledRadioText.module.scss'
import { RadioText } from '../../atoms/radio-text/RadioText'

interface Props {
  label: string
  value: string
  values: string[]
  displayValues?: string[]
  name: string
  disabled?: boolean
  onChange: (value: string) => void
}

export const LabeledRadioText: FC<Props> = (props) => {
  const displayValues = props.displayValues ?? props.values

  return (
    <div className={styles.component}>
      <span>{props.label}</span>
      <div>
        {props.values.map((value, i) => (
          <RadioText
            key={`${props.name}-${value}`}
            name={props.name}
            value={value}
            displayValue={displayValues[i]}
            currentValue={props.value}
            disabled={props.disabled}
            onChange={props.onChange}
          />
        ))}
      </div>
    </div>
  )
}
