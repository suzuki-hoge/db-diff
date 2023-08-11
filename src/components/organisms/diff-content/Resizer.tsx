import { type FC, type MouseEventHandler } from 'react'
import styles from './Resizer.module.scss'

interface Props {
  cellId: string
  resizerId: string
  height: number
}

export const Resizer: FC<Props> = (props) => {
  let x = 0
  let w = 0
  let max = 0

  const downHandler: MouseEventHandler = (e: React.MouseEvent<HTMLDivElement>) => {
    x = e.clientX

    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    w = parseInt(window.getComputedStyle(document.getElementById(props.cellId)!).width, 10)

    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    max = parseInt(window.getComputedStyle(document.getElementById(props.cellId)!).maxWidth, 10)

    document.addEventListener('mousemove', moveHandler)
    document.addEventListener('mouseup', upHandler)

    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    document.getElementById(props.resizerId)!.classList.add('resizing')
  }

  const moveHandler: (event: MouseEvent) => void = (e: MouseEvent) => {
    const dx = e.clientX - x

    if (w + dx <= max) {
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      document.getElementById(props.cellId)!.style.width = `${w + dx}px`
    } else {
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      document.getElementById(props.cellId)!.style.width = `${max}px`
    }
  }

  const upHandler: (event: MouseEvent) => void = () => {
    document.removeEventListener('mousemove', moveHandler)
    document.removeEventListener('mouseup', upHandler)

    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    document.getElementById(props.resizerId)!.classList.remove('resizing')
  }

  return <div id={props.resizerId} className={styles.resizer} style={{ height: `${props.height}px` }} onMouseDown={downHandler} />
}
