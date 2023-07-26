import { type FC } from 'react'
import Tour, { type ReactourStep } from 'reactour'

interface Props {
  steps: ReactourStep[]
  isTouring: boolean
  onClose: () => void
}

export const TourWrapper: FC<Props> = (props) => {
  return (
    <Tour
      steps={props.steps}
      isOpen={props.isTouring}
      onRequestClose={props.onClose}
      startAt={0}
      accentColor={'#4488ff'}
      rounded={8}
      showNumber={false}
      showCloseButton={false}
    />
  )
}
