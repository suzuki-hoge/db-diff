import { type FC } from 'react'
import styles from './LabeledRadioText.module.scss'
import { RadioText } from '../../atoms/radio-text/RadioText'

interface Props {
  label: string
  value: string
  values: string[]
  name: string
  onChange: (value: string) => void
}

export const LabeledRadioText: FC<Props> = (props) => {
  return (
    <div className={styles.component}>
      <span>{props.label}</span>
      <div>
        {props.values.map((value) => (
          <RadioText key={value} name={props.name} value={value} currentValue={props.value} onChange={props.onChange} />
        ))}
      </div>
    </div>
  )
}
