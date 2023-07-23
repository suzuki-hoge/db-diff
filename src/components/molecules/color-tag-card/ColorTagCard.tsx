import { type FC } from 'react'
import { ColorTag } from '../../atoms/color-tag/ColorTag'
import styles from './ColorTagCard.module.scss'

interface Props {
  label: string
  variant: 'red' | 'yellow' | 'green' | 'blue' | 'purple'
  onClick: () => void
}

export const ColorTagCard: FC<Props> = (props) => {
  return (
    <div className={['color_tag_card', styles.component].join(' ')} onClick={props.onClick}>
      <ColorTag variant={props.variant} />
      <span>{props.label}</span>
    </div>
  )
}
