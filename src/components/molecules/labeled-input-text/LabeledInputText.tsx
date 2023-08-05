import { type FC } from 'react'
import styles from './LabeledInputText.module.scss'
import { InputText } from '../../atoms/input-text/InputText'

interface Props {
  label: string
  value?: string
  maxLength: number
  onChange: (value: string) => void
  chars: 'all' | 'half' | 'number'
  autoFocus?: boolean
  errors?: string[]
}

export const LabeledInputText: FC<Props> = (props) => {
  return (
    <div className={styles.component}>
      <span>{props.label}</span>
      <InputText hasError={props.errors !== undefined} {...props} />
      {(props.errors ?? []).map((error) => (
        <p key={error} className={styles.error}>
          {error}
        </p>
      ))}
    </div>
  )
}
