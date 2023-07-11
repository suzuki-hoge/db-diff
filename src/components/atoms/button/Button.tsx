import { type FC } from 'react'
import styles from './Button.module.scss'

interface Props {
  variant: 'primary' | 'secondary' | 'warn'
  label: string
  onClick: () => void
}

const classes = {
  primary: styles.primary,
  secondary: styles.secondary,
  warn: styles.warn,
}

export const Button: FC<Props> = (props) => {
  return (
    <button type="button" className={classes[props.variant]} onClick={props.onClick}>
      {props.label}
    </button>
  )
}
