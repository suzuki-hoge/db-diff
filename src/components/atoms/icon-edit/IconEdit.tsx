import { type FC } from 'react'
import styles from './IconEdit.module.scss'
import { BsPencil } from 'react-icons/bs'

interface Props {
  variant: 'small' | 'medium' | 'large'
  onClick: () => void
}

export const IconEdit: FC<Props> = (props) => {
  return <BsPencil className={['icon_edit', styles[props.variant]].join(' ')} onClick={props.onClick} />
}
