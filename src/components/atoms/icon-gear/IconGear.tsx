import { type FC } from 'react'
import styles from './IconGear.module.scss'
import { BsGear } from 'react-icons/bs'

interface Props {
  variant: 'small' | 'medium' | 'large'
  onClick: () => void
}

export const IconGear: FC<Props> = (props) => {
  return <BsGear className={['icon_gear', styles[props.variant]].join(' ')} onClick={props.onClick} />
}
