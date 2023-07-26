import { type FC } from 'react'
import { ColorTag } from '../../atoms/color-tag/ColorTag'
import styles from './ColorTagCard.module.scss'

interface Props {
  label: string
  color: string
  onClick: () => void
}

export const ColorTagCard: FC<Props> = (props) => {
  return (
    <div className={['color_tag_card', styles.component].join(' ')} onClick={props.onClick}>
      <ColorTag color={props.color} />
      <span>{props.label}</span>
    </div>
  )
}
