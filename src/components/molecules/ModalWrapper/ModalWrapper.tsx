import { type FC, type ReactNode } from 'react'
import styles from './ModalWrapper.module.scss'
import Modal from 'react-modal'
import { IconClose } from '../../atoms/icon-close/IconClose'

interface Props {
  isOpen: boolean
  setIsOpen: (isOpen: boolean) => void
  button?: ReactNode
  children: ReactNode
}

export const ModalWrapper: FC<Props> = (props) => {
  return (
    <Modal
      className={styles.component}
      overlayClassName={styles.overlay}
      isOpen={props.isOpen}
      ariaHideApp={false}
      onRequestClose={() => {
        props.setIsOpen(false)
      }}
    >
      <div className={styles.content}>
        <div className={styles.navigation}>
          {props.button}
          <IconClose
            variant={'large'}
            onClick={() => {
              props.setIsOpen(false)
            }}
          />
        </div>
        {props.children}
      </div>
    </Modal>
  )
}
