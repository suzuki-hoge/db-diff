import { type FC } from 'react'
import styles from './IconBack.module.scss'
import { IoIosArrowBack } from 'react-icons/io'

interface Props {
  variant: 'small' | 'medium' | 'large'
  onClick: () => void
}

export const IconBack: FC<Props> = (props) => {
  return <IoIosArrowBack className={styles[props.variant]} onClick={props.onClick} />
}
