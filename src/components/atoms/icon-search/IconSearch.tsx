import { type FC } from 'react'
import styles from './IconSearch.module.scss'
import { BsSearch } from 'react-icons/bs'

interface Props {
  variant: 'small' | 'medium' | 'large'
  onClick: () => void
}

export const IconSearch: FC<Props> = (props) => {
  return <BsSearch className={['icon_search', styles[props.variant]].join(' ')} onClick={props.onClick} />
}
