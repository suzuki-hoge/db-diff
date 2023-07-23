import { type FC } from 'react'
import styles from './IconDelete.module.scss'
import { AiOutlineDelete } from 'react-icons/ai'

interface Props {
  variant: 'small' | 'medium' | 'large'
  onClick: () => void
}

export const IconDelete: FC<Props> = (props) => {
  return <AiOutlineDelete className={['icon_delete', styles[props.variant]].join(' ')} onClick={props.onClick} />
}
