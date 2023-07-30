import { type FC } from 'react'
import styles from './LabeledInputText.module.scss'
import { InputText } from '../../atoms/input-text/InputText'

interface Props {
  label: string
  value?: string
  maxLength: number
  onChange: (value: string) => void
  chars: 'all' | 'half' | 'number'
}

export const LabeledInputText: FC<Props> = (props) => {
  return (
    <div className={styles.component}>
      <span>{props.label}</span>
      <InputText value={props.value} maxLength={props.maxLength} onChange={props.onChange} chars={props.chars} />
    </div>
  )
}
