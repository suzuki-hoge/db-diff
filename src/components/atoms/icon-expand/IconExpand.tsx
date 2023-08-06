import { type FC } from 'react'
import styles from './IconExpand.module.scss'
import { RiExpandLeftLine, RiExpandRightLine } from 'react-icons/ri'

interface Props {
  variant: 'small' | 'medium' | 'large'
  expanded: boolean
  onClick: () => void
}

export const IconExpand: FC<Props> = (props) => {
  return (
    <>
      {props.expanded ? (
        <RiExpandLeftLine className={['icon_expand', styles[props.variant]].join(' ')} onClick={props.onClick} />
      ) : (
        <RiExpandRightLine className={['icon_expand', styles[props.variant]].join(' ')} onClick={props.onClick} />
      )}
    </>
  )
}
