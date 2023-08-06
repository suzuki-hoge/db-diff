import { type FC } from 'react'
import styles from './IconHide.module.scss'
import { RiMergeCellsHorizontal, RiSplitCellsHorizontal } from 'react-icons/ri'

interface Props {
  variant: 'small' | 'medium' | 'large'
  hide: boolean
  onClick: () => void
}

export const IconHide: FC<Props> = (props) => {
  return (
    <>
      {props.hide ? (
        <RiSplitCellsHorizontal className={['icon_hide', styles[props.variant]].join(' ')} onClick={props.onClick} />
      ) : (
        <RiMergeCellsHorizontal className={['icon_hide', styles[props.variant]].join(' ')} onClick={props.onClick} />
      )}
    </>
  )
}
