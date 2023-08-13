import { type FC } from 'react'
import styles from './IconPage.module.scss'
import { BsChevronDoubleLeft, BsChevronDoubleRight, BsChevronLeft, BsChevronRight } from 'react-icons/bs'

interface Props {
  variant: 'small' | 'medium' | 'large'
  vector: 'left' | 'right'
  end: boolean
  enabled: boolean
  onClick: () => void
}

export const IconPage: FC<Props> = (props) => {
  const color = props.enabled ? styles.black : styles.gray
  if (props.vector === 'left' && !props.end) {
    return <BsChevronLeft className={['icon_page', styles[props.variant], color].join(' ')} onClick={props.onClick} />
  } else if (props.vector === 'left' && props.end) {
    return <BsChevronDoubleLeft className={['icon_page', styles[props.variant], color].join(' ')} onClick={props.onClick} />
  } else if (props.vector === 'right' && !props.end) {
    return <BsChevronRight className={['icon_page', styles[props.variant], color].join(' ')} onClick={props.onClick} />
  } else {
    return <BsChevronDoubleRight className={['icon_page', styles[props.variant], color].join(' ')} onClick={props.onClick} />
  }
}
