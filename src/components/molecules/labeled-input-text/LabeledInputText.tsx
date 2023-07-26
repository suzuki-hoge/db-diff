import { type FC } from 'react'
import styles from './LabeledInputText.module.scss'
import { InputText } from '../../atoms/input-text/InputText'

interface Props {
  label: string
  value?: string
  length: number
  onChange: (value: string) => void
}

export const LabeledInputText: FC<Props> = (props) => {
  return (
    <div className={styles.component}>
      <span>{props.label}</span>
      <InputText value={props.value} length={props.length} onChange={props.onChange} />
    </div>
  )
}
