import { type FC } from 'react'
import styles from './IconVExpand.module.scss'
import { BsArrowsCollapse, BsArrowsExpand } from 'react-icons/bs'

interface Props {
  variant: 'small' | 'medium' | 'large'
  expanded: boolean
  onClick: () => void
}

export const IconVExpand: FC<Props> = (props) => {
  return (
    <>
      {props.expanded ? (
        <BsArrowsCollapse className={['icon_v_expand', styles[props.variant]].join(' ')} onClick={props.onClick} />
      ) : (
        <BsArrowsExpand className={['icon_v_expand', styles[props.variant]].join(' ')} onClick={props.onClick} />
      )}
    </>
  )
}
